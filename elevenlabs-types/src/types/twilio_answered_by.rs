pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Carrier-level AMD verdict reported by Twilio via `AnsweredBy`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TwilioAnsweredBy {
    Human,
    Fax,
    Unknown,
    MachineStart,
    MachineEndBeep,
    MachineEndSilence,
    MachineEndOther,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TwilioAnsweredBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Human => serializer.serialize_str("human"),
            Self::Fax => serializer.serialize_str("fax"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::MachineStart => serializer.serialize_str("machine_start"),
            Self::MachineEndBeep => serializer.serialize_str("machine_end_beep"),
            Self::MachineEndSilence => serializer.serialize_str("machine_end_silence"),
            Self::MachineEndOther => serializer.serialize_str("machine_end_other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TwilioAnsweredBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "human" => Ok(Self::Human),
            "fax" => Ok(Self::Fax),
            "unknown" => Ok(Self::Unknown),
            "machine_start" => Ok(Self::MachineStart),
            "machine_end_beep" => Ok(Self::MachineEndBeep),
            "machine_end_silence" => Ok(Self::MachineEndSilence),
            "machine_end_other" => Ok(Self::MachineEndOther),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TwilioAnsweredBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Human => write!(f, "human"),
            Self::Fax => write!(f, "fax"),
            Self::Unknown => write!(f, "unknown"),
            Self::MachineStart => write!(f, "machine_start"),
            Self::MachineEndBeep => write!(f, "machine_end_beep"),
            Self::MachineEndSilence => write!(f, "machine_end_silence"),
            Self::MachineEndOther => write!(f, "machine_end_other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
