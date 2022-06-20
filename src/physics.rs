use crate::{object::{*, AttribKey::*, AttribValue::*}, config::{LayerConfig, ObjectConfig}, speed::SpeedTracker};
use rapier2d::prelude::*;
use std::collections::*;

pub fn simulate(objects: &mut Vec<Obj>, config: LayerConfig, tracker: &SpeedTracker) {
    let mut shapes: HashMap<i32, Vec<Point<Real>>> = HashMap::new();
    let mut shape_groups: Vec<i32> = Vec::new();
    let mut centers: HashMap<i32, ObjCenter> = HashMap::new();
    let mut used_link_groups = Vec::new();
    let rotate_trigger = 1346;
    let move_trigger = 901;
    let mut i = 0;
    let mut anchor_obj_index = -1;
    while i < objects.len() {
        let object = &objects[i];
        let mut layer = 0;
        if let Some(Int(group)) = object.get(LinkedGroupID) {
            used_link_groups.push(*group);
        }
        if let Some(AttribValue::Int(l)) = object.get(EditorLayer1) {
            layer = *l;
        }
        if let Some(Int(id)) = object.get(ObjID) {
            let id = *id;
            if layer == config.layer as i32 && id == config.anchor_id {
                anchor_obj_index = i as i32
            }
            if layer == config.layer as i32 && (id == move_trigger || id == rotate_trigger) {
                objects.remove(i);                    
            }
            else {
                i += 1;
            }
            
        }            
    }
    let anchor_x = if anchor_obj_index != -1 {
        objects[anchor_obj_index as usize].get_pos().0
    }
    else {
        0.0
    };
    
    let link_group = used_link_groups.iter().max().unwrap_or(&0) + 1;
    for (i, object) in objects.iter().enumerate() {
        let mut layer = 0;      
        
        if let Some(AttribValue::Int(l)) = object.get(EditorLayer1) {
            layer = *l;
        }
        if layer == config.layer as i32 {          
            if let Some(Array(groups)) = object.get(GroupIDs) {
                if groups.is_empty() {continue}

                if let Some(points) = shapes.get_mut(&groups[0]) {
                    let p = object.get_pos();
                    points.push([p.0, p.1].into())
                }
                else {
                    let p = object.get_pos();
                    let points = vec![[p.0, p.1].into()];
                    shapes.insert(groups[0], points);
                    shape_groups.push(groups[0]);                    
                }
                if groups.len() > 1 {
                    centers.insert(groups[0], ObjCenter { center_group: groups[1], obj: i });
                }
            }
        }
    }
    
    let mut collider_set = ColliderSet::new();
    let mut rigid_body_set = RigidBodySet::new();
    let mut handles = Vec::new();
    if config.ground {
        let ground = ColliderBuilder::cuboid(100000.0, 0.1).build();
        collider_set.insert(ground);
    }
    for (group, points) in shapes.iter_mut() {
        let mut center = vector![0.0, 0.0];
        for point in points.iter() {
            center += vector![point.x, point.y];
        }
        center /= points.len() as f32;
        for point in points.iter_mut() {
            *point -= center;
        }
        let obj_config_option = config.objects.iter()
            .find(|c| c.group as i32 == *group);
        let default_config = ObjectConfig::new(*group as u16);
        let obj_config = if let Some(c) = obj_config_option {c} else {&default_config};
        let collider_builder = ColliderBuilder::convex_hull(points).unwrap();
        let rigid_body_builder = if obj_config.dynamic { RigidBodyBuilder::dynamic() } 
            else { RigidBodyBuilder::fixed() }
            .translation(center);
        let collider = collider_builder.restitution(obj_config.restitution)
            .density(obj_config.density)
            .friction(obj_config.friction).build();
        let mut rigid_body = rigid_body_builder.linear_damping(obj_config.linear_damping)
            .angular_damping(obj_config.angular_damping).build();
        rigid_body.set_linvel(vector![obj_config.velocity[0], obj_config.velocity[1]], true);
        rigid_body.set_angvel(-obj_config.angular_velocity.to_radians(), true);
        if obj_config.rotation_fixed {
            rigid_body.lock_rotations(true, true)
        }
        if obj_config.position_fixed {
            rigid_body.lock_translations(true, true)
        }
        let handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, handle, &mut rigid_body_set);

        handles.push((*group, handle));
    }
    
    let gravity = vector!(config.gravity[0], config.gravity[1]);
    let mut integration_parameters = IntegrationParameters::default();
    integration_parameters.set_inv_dt(config.fps as f32);
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();    

    let mut history = HashMap::new();
    for group in shape_groups {
        history.insert(group, Vec::new());
    }
    
    for _ in 0..(config.sim_time / (integration_parameters.dt * config.keyframe_interval as f32)) as i32 {        
        for (group, handle) in handles.iter() {
            let v = history.get_mut(group).unwrap();
            let object = &rigid_body_set[*handle];
            
            v.push(BodyState {
                x: object.translation().x,
                y: object.translation().y,
                rotation: (-object.rotation().angle().to_degrees())
            });
        }
        for _ in 0..config.keyframe_interval {
            physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut island_manager,
                &mut broad_phase,
                &mut narrow_phase,
                &mut rigid_body_set,
                &mut collider_set,
                &mut impulse_joint_set,
                &mut multibody_joint_set,
                &mut ccd_solver,
                &physics_hooks,
                &event_handler,
            );
        }
    }
    for (group, path) in history.iter() {
        if let Some(center) = centers.get(group) {
            let obj = &mut objects[center.obj];
            obj.set(XPos, Float(path[0].x));
            obj.set(YPos, Float(path[0].y));
        } 
    }
    
    let mut height = config.height;
    let duration = integration_parameters.dt * config.keyframe_interval as f32;    
    for (group, path) in history.iter() {
        let (start_x, start_y, start_rot) = (path[0].x, path[0].y, path[0].rotation);
        let center_group = if let Some(g) = centers.get(group) {g.center_group} else {continue};

        let mut trigger_x = anchor_x;
        let (mut total_dx, mut total_dy, mut total_rot) = (start_x, start_y, start_rot);
        
        for state in path.iter().skip(1) {
            let dx = state.x - total_dx;
            let dy = state.y - total_dy;
            let dr = (state.rotation - total_rot) % 360.0;       
            
            if dx.round().abs() >= 1.0 || dy.round().abs() >= 1.0 {
                let movement = Obj::new(move_trigger, trigger_x, height)
                    .with(MoveOffsetX, Int(dx.round() as i32))
                    .with(MoveOffsetY, Int(dy.round() as i32))
                    .with(TargetGroupID, Int(*group))
                    .with(Duration, Float(duration))
                    .with(EditorLayer1, Int(config.layer as i32))
                    .with(LinkedGroupID, Int(link_group));
                total_dx += dx.round();
                total_dy += dy.round();
                objects.push(movement);                
            }
            if dr.round().abs() >= 1.0 {                
                let degrees = if dr.round().abs() <= 180.0 {
                    dr.round()
                }
                else {
                    if dr > 0.0 {
                        dr.round() - 360.0
                    }
                    else {
                        dr.round() + 360.0
                    }
                }; 
                total_rot += degrees;               
                let rotation = Obj::new(rotate_trigger, trigger_x, height + 30.0)
                    .with(RotateDegrees, Int(degrees as i32))
                    .with(Duration, Float(duration * config.rotation_duration_modifier))
                    .with(TargetGroupID, Int(*group))
                    .with(SecondaryGroupID, Int(center_group))
                    .with(EditorLayer1, Int(config.layer as i32))
                    .with(LinkedGroupID, Int(link_group));
                if degrees as i32 != 0 {
                    objects.push(rotation);
                }
            }
            trigger_x += tracker.speed_at(trigger_x).bps() * duration * 30.0;
        }
        height -= 60.0;
    }
}
struct BodyState {
    x: f32,
    y: f32,
    rotation: f32
}
struct ObjCenter {
    center_group: i32,
    obj: usize
}