use std::collections::HashMap;
use strum::*;
use strum_macros::EnumIter;
use self::AttribKey::*;
use self::AttribValue::*;

#[derive(PartialEq, Eq)]
pub enum AttribType {
    Int, Float, Bool, Array, Hsv, Text 
}
#[derive(Clone)]
pub enum AttribValue {
    Int(i32), Float(f32), Bool(bool), Array(Vec<i32>), 
    Hsv {
        h: f32, s: f32, v: f32, s_checked: bool, v_checked: bool 
    },
    Text(String)
}
impl AttribValue {
    pub fn serialize(&self) -> String {
        match self {
            Int(i) => i.to_string(),
            Float(f) => f.to_string(),
            Bool(b) => if *b {String::from("1")} else {String::from("0")},
            Array(a) => {
                let mut str = a[0].to_string();
                for i in a.iter().skip(1) {
                    str = format!("{str}.{i}")
                }
                str
            },
            Hsv { h, s, v, s_checked, v_checked } => {
                let sc = if *s_checked {1} else {0};
                let vc = if *v_checked {1} else {0};

                format!("{h}a{s}a{v}a{sc}a{vc}")
            },
            AttribValue::Text(text) => text.clone(),
        }
    }   
    pub fn get_type(&self) -> AttribType {
        match self {
            Int(_) => AttribType::Int,
            Float(_) => AttribType::Float,
            Bool(_) => AttribType::Bool,
            Array(_) => AttribType::Array,
            Hsv { h: _, s: _, v: _, s_checked: _, v_checked: _ } => AttribType::Hsv,
            AttribValue::Text(_) => AttribType::Text,
        }
    }
}
#[derive(Hash, PartialEq, Eq, EnumIter, Copy, Clone)]
pub enum AttribKey {
    ObjID = 1, 
    XPos = 2,
    YPos = 3,
    FlipH = 4,
    FlipV = 5,
    Rotation = 6,
    Red = 7,
    Green = 8,
    Blue = 9,
    Duration = 10,
    TouchTriggered = 11,
    SecretCoinID = 12,
    SpecialCheck = 13,
    TintGround = 14,
    PlayerCol1 = 15,
    PlayerCol2 = 16,
    Blending = 17,
    EditorLayer1 = 20,
    MainColorChannelID = 21,
    SecondColorChannelID = 22,
    TargetColorID = 23,
    ZLayer = 24,
    ZOrder = 25,
    MoveOffsetX = 28,
    MoveOffsetY = 29,
    Easing = 30, 
    Text = 31,
    Scale = 32,
    IsGroupParent = 34,
    Opacity = 35,
    MainHSVEnabled = 41,
    SecondHSVEnabled = 42,
    MainColorHSV = 43,
    SecondColorHSV = 44,
    FadeInPulse = 45,
    HoldPulse = 46,
    FadeOutPulse = 47,
    PulseMode = 48,
    CopiedColorHSV = 49,
    CopiedColorID = 50,
    TargetGroupID = 51,
    PulseTargetType = 52,
    TeleportOffset = 54,
    TeleportEase = 55,
    ActivateGroup = 56,
    GroupIDs = 57,
    LockToPlayerX = 58,
    LockToPlayerY = 59,
    CopyOpacity = 60,
    EditorLayer2 = 61,
    SpawnTriggered = 62,
    SpawnDelay = 63,
    DontFade = 64,
    MainOnlyPulse = 65,
    DetailOnlyPulse = 66,
    DontEnter = 67,
    RotateDegrees = 68,
    RotateTimes360 = 69,
    LockObjectRotation = 70,
    SecondaryGroupID = 71,
    FollowXMod = 72,
    FollowYMod = 73,
    ShakeStrength = 75,
    AnimationID = 76,
    Count = 77,
    SubtractCount = 78,
    PickupMode = 79,
    ItemID = 80,
    TouchHoldMode = 81,
    TouchToggleMode = 82,
    ShakeInterval = 84,
    EasingRate = 85,
    ExclusivePulse = 86,
    MultiTrigger = 87,
    InstantCountComparison = 88,
    TouchDualMode = 89,
    FollowPlayerYSpeed = 90,
    FollowYDelay = 91,
    FollowYOffset = 92,
    TriggerOnExit = 93,
    IsDynamic = 94,
    BlockBID = 95,
    DisableGlow = 96,
    CustomRotationSpeed = 97,
    DisableRotation = 98,
    MultiActivateOrb = 99,
    EnableUseTarget = 100,
    TargetPosCoordinates = 101,
    EditorDisable = 102,
    HighDetail = 103,
    MultiActivateTrigger = 104,
    MaxSpeedFollowY = 105,
    RandomizeStartAnimation = 106,
    AnimationSpeed = 107,
    LinkedGroupID = 108
}
impl AttribKey {
    pub fn get_type(&self) -> AttribType {
        match self {
            AttribKey::ObjID => AttribType::Int,
            AttribKey::XPos => AttribType::Float,
            AttribKey::YPos => AttribType::Float,
            AttribKey::FlipH => AttribType::Bool,
            AttribKey::FlipV => AttribType::Bool,
            AttribKey::Rotation => AttribType::Float,
            AttribKey::Red => AttribType::Int,
            AttribKey::Green => AttribType::Int,
            AttribKey::Blue => AttribType::Int,
            AttribKey::Duration => AttribType::Float,
            AttribKey::TouchTriggered => AttribType::Bool,
            AttribKey::SecretCoinID => AttribType::Int,
            AttribKey::SpecialCheck => AttribType::Bool,
            AttribKey::TintGround => AttribType::Bool,
            AttribKey::PlayerCol1 => AttribType::Bool,
            AttribKey::PlayerCol2 => AttribType::Bool,
            AttribKey::Blending => AttribType::Bool,
            AttribKey::EditorLayer1 => AttribType::Int,
            AttribKey::MainColorChannelID => AttribType::Int,
            AttribKey::SecondColorChannelID => AttribType::Int,
            AttribKey::TargetColorID => AttribType::Int,
            AttribKey::ZLayer => AttribType::Int,
            AttribKey::ZOrder => AttribType::Int,
            AttribKey::MoveOffsetX => AttribType::Int,
            AttribKey::MoveOffsetY => AttribType::Int,
            AttribKey::Easing => AttribType::Int,
            AttribKey::Text => AttribType::Text,
            AttribKey::Scale => AttribType::Float,
            AttribKey::IsGroupParent => AttribType::Bool,
            AttribKey::Opacity => AttribType::Float,
            AttribKey::MainHSVEnabled => AttribType::Bool,
            AttribKey::SecondHSVEnabled => AttribType::Bool,
            AttribKey::MainColorHSV => AttribType::Hsv,
            AttribKey::SecondColorHSV => AttribType::Hsv,
            AttribKey::FadeInPulse => AttribType::Float,
            AttribKey::HoldPulse => AttribType::Float,
            AttribKey::FadeOutPulse => AttribType::Float,
            AttribKey::PulseMode => AttribType::Int,
            AttribKey::CopiedColorHSV => AttribType::Hsv,
            AttribKey::CopiedColorID => AttribType::Int,
            AttribKey::TargetGroupID => AttribType::Int,
            AttribKey::PulseTargetType => AttribType::Int,
            AttribKey::TeleportOffset => AttribType::Float,
            AttribKey::TeleportEase => AttribType::Bool,
            AttribKey::ActivateGroup => AttribType::Bool,
            AttribKey::GroupIDs => AttribType::Array,
            AttribKey::LockToPlayerX => AttribType::Bool,
            AttribKey::LockToPlayerY => AttribType::Bool,
            AttribKey::CopyOpacity => AttribType::Bool,
            AttribKey::EditorLayer2 => AttribType::Int,
            AttribKey::SpawnTriggered => AttribType::Bool,
            AttribKey::SpawnDelay => AttribType::Float,
            AttribKey::DontFade => AttribType::Bool,
            AttribKey::MainOnlyPulse => AttribType::Bool,
            AttribKey::DetailOnlyPulse => AttribType::Bool,
            AttribKey::DontEnter => AttribType::Bool,
            AttribKey::RotateDegrees => AttribType::Int,
            AttribKey::RotateTimes360 => AttribType::Int,
            AttribKey::LockObjectRotation => AttribType::Bool,
            AttribKey::SecondaryGroupID => AttribType::Int,
            AttribKey::FollowXMod => AttribType::Float,
            AttribKey::FollowYMod => AttribType::Float,
            AttribKey::ShakeStrength => AttribType::Float,
            AttribKey::AnimationID => AttribType::Int,
            AttribKey::Count => AttribType::Int,
            AttribKey::SubtractCount => AttribType::Bool,
            AttribKey::PickupMode => AttribType::Int,
            AttribKey::ItemID => AttribType::Int,
            AttribKey::TouchHoldMode => AttribType::Bool,
            AttribKey::TouchToggleMode => AttribType::Bool,
            AttribKey::ShakeInterval => AttribType::Float,
            AttribKey::EasingRate => AttribType::Float,
            AttribKey::ExclusivePulse => AttribType::Bool,
            AttribKey::MultiTrigger => AttribType::Bool,
            AttribKey::InstantCountComparison => AttribType::Int,
            AttribKey::TouchDualMode => AttribType::Bool,
            AttribKey::FollowPlayerYSpeed => AttribType::Float,
            AttribKey::FollowYDelay => AttribType::Float,
            AttribKey::FollowYOffset => AttribType::Float,
            AttribKey::TriggerOnExit => AttribType::Bool,
            AttribKey::IsDynamic => AttribType::Bool,
            AttribKey::BlockBID => AttribType::Int,
            AttribKey::DisableGlow => AttribType::Bool,
            AttribKey::CustomRotationSpeed => AttribType::Float,
            AttribKey::DisableRotation => AttribType::Bool,
            AttribKey::MultiActivateOrb => AttribType::Bool,
            AttribKey::EnableUseTarget => AttribType::Bool,
            AttribKey::TargetPosCoordinates => AttribType::Int,
            AttribKey::EditorDisable => AttribType::Bool,
            AttribKey::HighDetail => AttribType::Bool,
            AttribKey::MultiActivateTrigger => AttribType::Bool,
            AttribKey::MaxSpeedFollowY => AttribType::Float,
            AttribKey::RandomizeStartAnimation => AttribType::Bool,
            AttribKey::AnimationSpeed => AttribType::Float,
            AttribKey::LinkedGroupID => AttribType::Int,
        }
    }
    pub fn from_id(id: u16) -> Option<Self> {
        for key in AttribKey::iter() {
            if key as u16 == id {
                return Some(key);
            }
        }
        None
    }
}
/// A wrapper for a Geometry Dash object
pub struct Obj {
    attribs: HashMap<AttribKey, AttribValue>    
}
impl Obj {
    pub fn new(id: i32, x: f32, y: f32) -> Self {
        let obj = Obj {
            attribs: HashMap::new()
        };        
        obj.with(ObjID, AttribValue::Int(id))
            .with(XPos, Float(x))
            .with(YPos, Float(y))
    }
    pub fn get(&self, key: AttribKey) -> Option<&AttribValue> {
        self.attribs.get(&key)
    }
    pub fn get_pos(&self) -> (f32, f32) {
        let x = self.get(XPos);
        let y = self.get(YPos);
        if let Float(x) = x.unwrap() {
            if let Float(y) = y.unwrap() {
                return (*x, *y);
            }
        }
        panic!("No position properties")
    }
    pub fn _with_rot(self, rot: f32) -> Self {
        self.with(Rotation, Float(rot))
    }
    pub fn set(&mut self, key: AttribKey, val: AttribValue) {
        self.attribs.insert(key, val);
    }
    pub fn with(mut self, key: AttribKey, val: AttribValue) -> Self {
        assert!(key.get_type() == val.get_type());
        self.attribs.insert(key, val);
        self
    }
    pub fn as_str(&self) -> String {
        let mut str = String::new();
        for (key, value) in self.attribs.iter() {
            let key_id = *key as u16;
            let value_str = value.serialize();
            let pair = format!("{key_id}, {value_str},");
            str.push_str(&*pair);
        }
        str 
    }
}
impl From<String> for Obj {
    fn from(str: String) -> Self {
        let split: Vec<&str> = str.split(',').collect();
        let mut obj = Obj {
            attribs: HashMap::new()
        };
        for i in (0..split.len() - 1).step_by(2) {
            let key_str: String = split[i].chars().filter(|c| !c.is_whitespace()).collect();
            let val_str: String = split[i + 1].chars().filter(|c| !c.is_whitespace()).collect();
            if let Ok(key_id) = key_str.parse::<u16>() {
                if let Some(key) = AttribKey::from_id(key_id) {
                    let value = match key.get_type() {
                        AttribType::Int => {
                            let parsed = val_str.parse::<i32>().unwrap_or_else(|_| panic!("Failed to parse integer: {val_str}"));
                            AttribValue::Int(parsed)
                        },
                        AttribType::Float => {
                            let parsed = val_str.parse::<f32>().unwrap_or_else(|_| panic!("Failed to parse float: {val_str}"));
                            AttribValue::Float(parsed)
                        },
                        AttribType::Bool => {
                            let val = val_str.parse::<i32>().unwrap_or_else(|_| panic!("Failed to parse bo: {val_str}"));
                            AttribValue::Bool(val == 1)
                        },
                        AttribType::Array => {
                            let split = val_str.split('.');
                            let mut arr = Vec::new();
                            for i in split {
                                arr.push(i.parse().unwrap_or_else(|_| panic!("Failed to parse array element: {val_str}")))
                            }
                            AttribValue::Array(arr)
                        },
                        AttribType::Hsv => {
                            let split: Vec<&str> = val_str.split('a').collect();
                            assert!(split.len() == 5);
                            AttribValue::Hsv { 
                                h: split[0].parse().unwrap_or_else(|_| panic!("Failed to parse hue: {val_str}")), 
                                s: split[1].parse().unwrap_or_else(|_| panic!("Failed to parse saturation: {val_str}")), 
                                v: split[2].parse().unwrap_or_else(|_| panic!("Failed to parse value: {val_str}")), 
                                s_checked: split[3].parse::<u16>().unwrap_or_else(|_| panic!("Failed to parse s_checked: {val_str}")) == 1,
                                v_checked: split[4].parse::<u16>().unwrap_or_else(|_| panic!("Failed to parse v_checked: {val_str}")) == 1
                            }
                        },
                        AttribType::Text => {
                            AttribValue::Text(val_str.to_string())
                        }
                    };
                    obj.attribs.insert(key, value);
                }
            }
        }
        obj
    }
}