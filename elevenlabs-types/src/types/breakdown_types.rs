pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How to break down the information. Cannot be "user" or "api_key" if include_workspace_metrics is False.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BreakdownTypes {
    None,
    Voice,
    VoiceMultiplier,
    User,
    Groups,
    ApiKeys,
    AllApiKeys,
    ProductType,
    Model,
    Resource,
    RequestQueue,
    Region,
    SubresourceId,
    ReportingWorkspaceId,
    HasApiKey,
    RequestSource,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BreakdownTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Voice => serializer.serialize_str("voice"),
            Self::VoiceMultiplier => serializer.serialize_str("voice_multiplier"),
            Self::User => serializer.serialize_str("user"),
            Self::Groups => serializer.serialize_str("groups"),
            Self::ApiKeys => serializer.serialize_str("api_keys"),
            Self::AllApiKeys => serializer.serialize_str("all_api_keys"),
            Self::ProductType => serializer.serialize_str("product_type"),
            Self::Model => serializer.serialize_str("model"),
            Self::Resource => serializer.serialize_str("resource"),
            Self::RequestQueue => serializer.serialize_str("request_queue"),
            Self::Region => serializer.serialize_str("region"),
            Self::SubresourceId => serializer.serialize_str("subresource_id"),
            Self::ReportingWorkspaceId => serializer.serialize_str("reporting_workspace_id"),
            Self::HasApiKey => serializer.serialize_str("has_api_key"),
            Self::RequestSource => serializer.serialize_str("request_source"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BreakdownTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "voice" => Ok(Self::Voice),
            "voice_multiplier" => Ok(Self::VoiceMultiplier),
            "user" => Ok(Self::User),
            "groups" => Ok(Self::Groups),
            "api_keys" => Ok(Self::ApiKeys),
            "all_api_keys" => Ok(Self::AllApiKeys),
            "product_type" => Ok(Self::ProductType),
            "model" => Ok(Self::Model),
            "resource" => Ok(Self::Resource),
            "request_queue" => Ok(Self::RequestQueue),
            "region" => Ok(Self::Region),
            "subresource_id" => Ok(Self::SubresourceId),
            "reporting_workspace_id" => Ok(Self::ReportingWorkspaceId),
            "has_api_key" => Ok(Self::HasApiKey),
            "request_source" => Ok(Self::RequestSource),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BreakdownTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Voice => write!(f, "voice"),
            Self::VoiceMultiplier => write!(f, "voice_multiplier"),
            Self::User => write!(f, "user"),
            Self::Groups => write!(f, "groups"),
            Self::ApiKeys => write!(f, "api_keys"),
            Self::AllApiKeys => write!(f, "all_api_keys"),
            Self::ProductType => write!(f, "product_type"),
            Self::Model => write!(f, "model"),
            Self::Resource => write!(f, "resource"),
            Self::RequestQueue => write!(f, "request_queue"),
            Self::Region => write!(f, "region"),
            Self::SubresourceId => write!(f, "subresource_id"),
            Self::ReportingWorkspaceId => write!(f, "reporting_workspace_id"),
            Self::HasApiKey => write!(f, "has_api_key"),
            Self::RequestSource => write!(f, "request_source"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
