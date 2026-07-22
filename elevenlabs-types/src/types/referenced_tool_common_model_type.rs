pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of the tool
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReferencedToolCommonModelType {
    System,
    Webhook,
    Client,
    Workflow,
    ApiIntegrationWebhook,
    Mcp,
    Code,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReferencedToolCommonModelType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::System => serializer.serialize_str("system"),
            Self::Webhook => serializer.serialize_str("webhook"),
            Self::Client => serializer.serialize_str("client"),
            Self::Workflow => serializer.serialize_str("workflow"),
            Self::ApiIntegrationWebhook => serializer.serialize_str("api_integration_webhook"),
            Self::Mcp => serializer.serialize_str("mcp"),
            Self::Code => serializer.serialize_str("code"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReferencedToolCommonModelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "system" => Ok(Self::System),
            "webhook" => Ok(Self::Webhook),
            "client" => Ok(Self::Client),
            "workflow" => Ok(Self::Workflow),
            "api_integration_webhook" => Ok(Self::ApiIntegrationWebhook),
            "mcp" => Ok(Self::Mcp),
            "code" => Ok(Self::Code),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReferencedToolCommonModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::System => write!(f, "system"),
            Self::Webhook => write!(f, "webhook"),
            Self::Client => write!(f, "client"),
            Self::Workflow => write!(f, "workflow"),
            Self::ApiIntegrationWebhook => write!(f, "api_integration_webhook"),
            Self::Mcp => write!(f, "mcp"),
            Self::Code => write!(f, "code"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
