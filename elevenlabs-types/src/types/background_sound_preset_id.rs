pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Predefined background sound preset identifiers.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BackgroundSoundPresetId {
    Office2,
    Office1,
    Restaurant,
    City,
    Typing,
    Elevator1,
    Elevator2,
    Elevator3,
    Elevator4,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BackgroundSoundPresetId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Office2 => serializer.serialize_str("office2"),
            Self::Office1 => serializer.serialize_str("office1"),
            Self::Restaurant => serializer.serialize_str("restaurant"),
            Self::City => serializer.serialize_str("city"),
            Self::Typing => serializer.serialize_str("typing"),
            Self::Elevator1 => serializer.serialize_str("elevator1"),
            Self::Elevator2 => serializer.serialize_str("elevator2"),
            Self::Elevator3 => serializer.serialize_str("elevator3"),
            Self::Elevator4 => serializer.serialize_str("elevator4"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BackgroundSoundPresetId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "office2" => Ok(Self::Office2),
            "office1" => Ok(Self::Office1),
            "restaurant" => Ok(Self::Restaurant),
            "city" => Ok(Self::City),
            "typing" => Ok(Self::Typing),
            "elevator1" => Ok(Self::Elevator1),
            "elevator2" => Ok(Self::Elevator2),
            "elevator3" => Ok(Self::Elevator3),
            "elevator4" => Ok(Self::Elevator4),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BackgroundSoundPresetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Office2 => write!(f, "office2"),
            Self::Office1 => write!(f, "office1"),
            Self::Restaurant => write!(f, "restaurant"),
            Self::City => write!(f, "city"),
            Self::Typing => write!(f, "typing"),
            Self::Elevator1 => write!(f, "elevator1"),
            Self::Elevator2 => write!(f, "elevator2"),
            Self::Elevator3 => write!(f, "elevator3"),
            Self::Elevator4 => write!(f, "elevator4"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
