pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// This parameter controls text normalization with three modes: 'auto', 'on', and 'off'. When set to 'auto', the system will automatically decide whether to apply text normalization (e.g., spelling out numbers). With 'on', text normalization will always be applied, while with 'off', it will be skipped.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BodyTextToDialogueStreamWithTimestampsApplyTextNormalization {
    Auto,
    On,
    Off,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BodyTextToDialogueStreamWithTimestampsApplyTextNormalization {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::On => serializer.serialize_str("on"),
            Self::Off => serializer.serialize_str("off"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BodyTextToDialogueStreamWithTimestampsApplyTextNormalization {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BodyTextToDialogueStreamWithTimestampsApplyTextNormalization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
