pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The HTTP method to use for the webhook
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookToolApiSchemaConfigOutputMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookToolApiSchemaConfigOutputMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Get => serializer.serialize_str("GET"),
            Self::Post => serializer.serialize_str("POST"),
            Self::Put => serializer.serialize_str("PUT"),
            Self::Patch => serializer.serialize_str("PATCH"),
            Self::Delete => serializer.serialize_str("DELETE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookToolApiSchemaConfigOutputMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookToolApiSchemaConfigOutputMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Patch => write!(f, "PATCH"),
            Self::Delete => write!(f, "DELETE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
