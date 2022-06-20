use std::fs::File;
use std::io::{Read, Error};
use serde_derive::{Serialize, Deserialize};
use serde_json::{self, json};
#[derive(Serialize, Deserialize)]
pub struct Config {    
    pub level_name: String,    
    pub path: String,
    pub backup_path: String,
    pub simulations: Vec<LayerConfig>    
}
#[derive(Serialize, Deserialize)]
pub struct LayerConfig {
    pub layer: u16,
    #[serde(default = "default_height")]
    pub height: f32,
    #[serde(default = "default_fps")]
    pub fps: f32,
    #[serde(default = "default_interval")]
    pub keyframe_interval: u16,
    #[serde(default = "default_gravity")]
    pub gravity: [f32; 2],
    #[serde(default = "default_objects")]    
    pub objects: Vec<ObjectConfig>,
    #[serde(default = "default_rotation_modifier")]    
    pub rotation_duration_modifier: f32,
    #[serde(default = "default_time")]
    pub sim_time: f32,
    #[serde(default = "default_anchor")]
    pub anchor_id: i32,
    #[serde(default = "default_ground")]
    pub ground: bool
}
#[derive(Serialize, Deserialize)]
pub struct ObjectConfig {
    pub group: u16,
    #[serde(default = "default_vel")]
    pub velocity: [f32; 2],
    #[serde(default = "default_angvel")]
    pub angular_velocity: f32,
    #[serde(default = "default_density")]
    pub density: f32,
    #[serde(default = "default_dynamic")]
    pub dynamic: bool,
    #[serde(default = "default_restitution")]
    pub restitution: f32,
    #[serde(default = "default_linear_damping")]
    pub linear_damping: f32,
    #[serde(default = "default_angular_damping")]
    pub angular_damping: f32,
    #[serde(default = "default_friction")]
    pub friction: f32,
    #[serde(default = "default_pos_fixed")]
    pub position_fixed: bool,
    #[serde(default = "default_ang_fixed")]
    pub rotation_fixed: bool,    
}
impl Config {
    pub fn new(path: String) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let mut json_str = String::new();
        file.read_to_string(&mut json_str)?;
        let config: Config = serde_json::from_str(&json_str)?;
        Ok(config)        
    }
}
impl ObjectConfig {
    pub fn new(group: u16) -> Self {
        serde_json::from_value(json!({
            "group" : group
        })).unwrap()        
    }
}
// Todo: move defaults to another config file
fn default_fps() -> f32 {60.0}
fn default_interval() -> u16 {6}
fn default_vel() -> [f32; 2] {[0.0, 0.0]}
fn default_gravity() -> [f32; 2] {[0.0, -9.81 * 30.0]}
fn default_objects() -> Vec<ObjectConfig> {Vec::new()}
fn default_density() -> f32 {1.0}
fn default_dynamic() -> bool {true}
fn default_height() -> f32 {2100.0}
fn default_restitution() -> f32 {0.0}
fn default_linear_damping() -> f32 {0.1}
fn default_angular_damping() -> f32 {0.1}
fn default_friction() -> f32 {1.0}
fn default_angvel() -> f32 {0.0}
fn default_pos_fixed() -> bool {false}
fn default_ang_fixed() -> bool {false}
fn default_rotation_modifier() -> f32 {0.9}
fn default_time() -> f32 {5.0}
fn default_anchor() -> i32 {41}
fn default_ground() -> bool {true}