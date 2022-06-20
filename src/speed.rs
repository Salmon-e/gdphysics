use crate::object::{*, AttribKey::*, AttribValue::*};
use self::Speed::*;
pub struct SpeedTracker {
    pub speeds: Vec<(Speed, f32)>
}
impl SpeedTracker {
    pub fn new(objects: &Vec<Obj>) -> Self {
        let mut portals: Vec<(Speed, f32)> = vec![(Normal, 0.0)];
        for object in objects {
            if let Int(id) = object.get(ObjID).unwrap() {
                if let Some(speed) = Speed::from(*id) {
                    if let Some(Bool(b)) = object.get(SpecialCheck) {
                        if *b {
                            portals.push((speed, object.get_pos().0 - speed.width()/2.0))                            
                        }
                    }
                }
            }
        }
        portals.dedup();
        SpeedTracker {
            speeds: portals
        }
    }
    pub fn speed_at(&self, x: f32) -> Speed {
        for (speed, dist) in self.speeds.iter().rev() {        
            if x >= *dist {
                return *speed
            }    
        }
        return self.speeds[0].0
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Speed {
    Half, Normal, Double, Triple, Quad
}
impl Speed {
    pub fn bps(&self) -> f32 {
        match self {
            Speed::Half => 8.383,
            Speed::Normal => 10.387,
            Speed::Double => 12.915,
            Speed::Triple => 15.601,
            Speed::Quad => 19.201,
        }
    }
    pub fn from(id: i32) -> Option<Speed> {
        match id {
            200 => Some(Half),
            201 => Some(Normal),
            202 => Some(Double),
            203 => Some(Triple),
            1334 => Some(Quad),
            _ => None
        }
    }
    pub fn width(&self) -> f32 {
        match self {
            Half => 34.0,
            Normal => 32.0,
            Double => 50.0,
            Triple => 65.0,
            Quad => 69.0,
        }
    }
}