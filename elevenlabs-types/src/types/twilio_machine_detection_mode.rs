pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Which Twilio answering-machine-detection (AMD) mode to request for a call.
/// 
/// `enable` returns a verdict as soon as Twilio can tell a human from a machine.
/// `detect_message_end` additionally waits for the greeting to finish, which is what
/// distinguishes the `machine_end_*` verdicts, at the cost of a later result.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TwilioMachineDetectionMode {
    Enable,
    DetectMessageEnd,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TwilioMachineDetectionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Enable => serializer.serialize_str("enable"),
            Self::DetectMessageEnd => serializer.serialize_str("detect_message_end"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TwilioMachineDetectionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "enable" => Ok(Self::Enable),
            "detect_message_end" => Ok(Self::DetectMessageEnd),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TwilioMachineDetectionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Enable => write!(f, "enable"),
            Self::DetectMessageEnd => write!(f, "detect_message_end"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
