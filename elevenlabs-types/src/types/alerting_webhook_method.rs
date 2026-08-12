pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlertingWebhookMethod {
    Post,
    Put,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AlertingWebhookMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Post => serializer.serialize_str("POST"),
            Self::Put => serializer.serialize_str("PUT"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AlertingWebhookMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AlertingWebhookMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
