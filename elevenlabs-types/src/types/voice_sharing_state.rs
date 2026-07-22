pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The status of the voice sharing.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoiceSharingState {
    Enabled,
    Disabled,
    Copied,
    CopiedDisabled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoiceSharingState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Enabled => serializer.serialize_str("enabled"),
            Self::Disabled => serializer.serialize_str("disabled"),
            Self::Copied => serializer.serialize_str("copied"),
            Self::CopiedDisabled => serializer.serialize_str("copied_disabled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoiceSharingState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "enabled" => Ok(Self::Enabled),
            "disabled" => Ok(Self::Disabled),
            "copied" => Ok(Self::Copied),
            "copied_disabled" => Ok(Self::CopiedDisabled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoiceSharingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Enabled => write!(f, "enabled"),
            Self::Disabled => write!(f, "disabled"),
            Self::Copied => write!(f, "copied"),
            Self::CopiedDisabled => write!(f, "copied_disabled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
