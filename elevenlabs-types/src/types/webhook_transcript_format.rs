pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookTranscriptFormat {
    Json,
    Opentelemetry,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookTranscriptFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Json => serializer.serialize_str("json"),
            Self::Opentelemetry => serializer.serialize_str("opentelemetry"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookTranscriptFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "json" => Ok(Self::Json),
            "opentelemetry" => Ok(Self::Opentelemetry),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookTranscriptFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Opentelemetry => write!(f, "opentelemetry"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
