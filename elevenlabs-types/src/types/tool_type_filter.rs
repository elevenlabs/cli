pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolTypeFilter {
    Webhook,
    Client,
    ApiIntegrationWebhook,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolTypeFilter {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Webhook => serializer.serialize_str("webhook"),
            Self::Client => serializer.serialize_str("client"),
            Self::ApiIntegrationWebhook => serializer.serialize_str("api_integration_webhook"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolTypeFilter {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "webhook" => Ok(Self::Webhook),
            "client" => Ok(Self::Client),
            "api_integration_webhook" => Ok(Self::ApiIntegrationWebhook),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolTypeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Webhook => write!(f, "webhook"),
            Self::Client => write!(f, "client"),
            Self::ApiIntegrationWebhook => write!(f, "api_integration_webhook"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
