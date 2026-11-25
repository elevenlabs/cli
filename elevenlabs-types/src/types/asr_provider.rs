pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsrProvider {
    /// Deprecated: Use scribe_realtime instead.
    Elevenlabs,
    ScribeRealtime,
    ScribeV2Turbo,
    ScribeV3Live,
    ScribeV3LiveMedical,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AsrProvider {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Elevenlabs => serializer.serialize_str("elevenlabs"),
            Self::ScribeRealtime => serializer.serialize_str("scribe_realtime"),
            Self::ScribeV2Turbo => serializer.serialize_str("scribe_v2_turbo"),
            Self::ScribeV3Live => serializer.serialize_str("scribe_v3_live"),
            Self::ScribeV3LiveMedical => serializer.serialize_str("scribe_v3_live_medical"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AsrProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "elevenlabs" => Ok(Self::Elevenlabs),
            "scribe_realtime" => Ok(Self::ScribeRealtime),
            "scribe_v2_turbo" => Ok(Self::ScribeV2Turbo),
            "scribe_v3_live" => Ok(Self::ScribeV3Live),
            "scribe_v3_live_medical" => Ok(Self::ScribeV3LiveMedical),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AsrProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Elevenlabs => write!(f, "elevenlabs"),
            Self::ScribeRealtime => write!(f, "scribe_realtime"),
            Self::ScribeV2Turbo => write!(f, "scribe_v2_turbo"),
            Self::ScribeV3Live => write!(f, "scribe_v3_live"),
            Self::ScribeV3LiveMedical => write!(f, "scribe_v3_live_medical"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
