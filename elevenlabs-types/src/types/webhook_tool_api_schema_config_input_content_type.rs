pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Content type for the request body. Only applies to POST/PUT/PATCH requests.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookToolApiSchemaConfigInputContentType {
    ApplicationJson,
    ApplicationXWwwFormUrlencoded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookToolApiSchemaConfigInputContentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ApplicationJson => serializer.serialize_str("application/json"),
            Self::ApplicationXWwwFormUrlencoded => serializer.serialize_str("application/x-www-form-urlencoded"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookToolApiSchemaConfigInputContentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "application/json" => Ok(Self::ApplicationJson),
            "application/x-www-form-urlencoded" => Ok(Self::ApplicationXWwwFormUrlencoded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookToolApiSchemaConfigInputContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApplicationJson => write!(f, "application/json"),
            Self::ApplicationXWwwFormUrlencoded => write!(f, "application/x-www-form-urlencoded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
