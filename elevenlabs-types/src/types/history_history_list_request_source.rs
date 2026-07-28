pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HistoryListRequestSource {
    Tts,
    Sts,
    Flows,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for HistoryListRequestSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Tts => serializer.serialize_str("TTS"),
            Self::Sts => serializer.serialize_str("STS"),
            Self::Flows => serializer.serialize_str("Flows"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for HistoryListRequestSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TTS" => Ok(Self::Tts),
            "STS" => Ok(Self::Sts),
            "Flows" => Ok(Self::Flows),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for HistoryListRequestSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tts => write!(f, "TTS"),
            Self::Sts => write!(f, "STS"),
            Self::Flows => write!(f, "Flows"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
