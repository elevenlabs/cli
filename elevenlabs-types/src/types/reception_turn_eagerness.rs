pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How quickly the receptionist decides the caller has finished speaking.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReceptionTurnEagerness {
    Patient,
    Normal,
    Eager,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReceptionTurnEagerness {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Patient => serializer.serialize_str("patient"),
            Self::Normal => serializer.serialize_str("normal"),
            Self::Eager => serializer.serialize_str("eager"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReceptionTurnEagerness {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "patient" => Ok(Self::Patient),
            "normal" => Ok(Self::Normal),
            "eager" => Ok(Self::Eager),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReceptionTurnEagerness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Patient => write!(f, "patient"),
            Self::Normal => write!(f, "normal"),
            Self::Eager => write!(f, "eager"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
