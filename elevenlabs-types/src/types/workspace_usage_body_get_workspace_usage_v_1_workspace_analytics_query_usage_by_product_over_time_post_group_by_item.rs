pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostGroupByItem {
    ProductType,
    Model,
    VoiceId,
    UserId,
    FiatCurrency,
    FiatChargeType,
    Region,
    ReportingWorkspaceId,
    RequestSource,
    ResourceId,
    SubresourceId,
    RequestQueueType,
    VoiceMultiplier,
    HashedXiApiKey,
    BillingGroupId,
    Surface,
    Actor,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostGroupByItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ProductType => serializer.serialize_str("product_type"),
            Self::Model => serializer.serialize_str("model"),
            Self::VoiceId => serializer.serialize_str("voice_id"),
            Self::UserId => serializer.serialize_str("user_id"),
            Self::FiatCurrency => serializer.serialize_str("fiat_currency"),
            Self::FiatChargeType => serializer.serialize_str("fiat_charge_type"),
            Self::Region => serializer.serialize_str("region"),
            Self::ReportingWorkspaceId => serializer.serialize_str("reporting_workspace_id"),
            Self::RequestSource => serializer.serialize_str("request_source"),
            Self::ResourceId => serializer.serialize_str("resource_id"),
            Self::SubresourceId => serializer.serialize_str("subresource_id"),
            Self::RequestQueueType => serializer.serialize_str("request_queue_type"),
            Self::VoiceMultiplier => serializer.serialize_str("voice_multiplier"),
            Self::HashedXiApiKey => serializer.serialize_str("hashed_xi_api_key"),
            Self::BillingGroupId => serializer.serialize_str("billing_group_id"),
            Self::Surface => serializer.serialize_str("surface"),
            Self::Actor => serializer.serialize_str("actor"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostGroupByItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "product_type" => Ok(Self::ProductType),
            "model" => Ok(Self::Model),
            "voice_id" => Ok(Self::VoiceId),
            "user_id" => Ok(Self::UserId),
            "fiat_currency" => Ok(Self::FiatCurrency),
            "fiat_charge_type" => Ok(Self::FiatChargeType),
            "region" => Ok(Self::Region),
            "reporting_workspace_id" => Ok(Self::ReportingWorkspaceId),
            "request_source" => Ok(Self::RequestSource),
            "resource_id" => Ok(Self::ResourceId),
            "subresource_id" => Ok(Self::SubresourceId),
            "request_queue_type" => Ok(Self::RequestQueueType),
            "voice_multiplier" => Ok(Self::VoiceMultiplier),
            "hashed_xi_api_key" => Ok(Self::HashedXiApiKey),
            "billing_group_id" => Ok(Self::BillingGroupId),
            "surface" => Ok(Self::Surface),
            "actor" => Ok(Self::Actor),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostGroupByItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProductType => write!(f, "product_type"),
            Self::Model => write!(f, "model"),
            Self::VoiceId => write!(f, "voice_id"),
            Self::UserId => write!(f, "user_id"),
            Self::FiatCurrency => write!(f, "fiat_currency"),
            Self::FiatChargeType => write!(f, "fiat_charge_type"),
            Self::Region => write!(f, "region"),
            Self::ReportingWorkspaceId => write!(f, "reporting_workspace_id"),
            Self::RequestSource => write!(f, "request_source"),
            Self::ResourceId => write!(f, "resource_id"),
            Self::SubresourceId => write!(f, "subresource_id"),
            Self::RequestQueueType => write!(f, "request_queue_type"),
            Self::VoiceMultiplier => write!(f, "voice_multiplier"),
            Self::HashedXiApiKey => write!(f, "hashed_xi_api_key"),
            Self::BillingGroupId => write!(f, "billing_group_id"),
            Self::Surface => write!(f, "surface"),
            Self::Actor => write!(f, "actor"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
