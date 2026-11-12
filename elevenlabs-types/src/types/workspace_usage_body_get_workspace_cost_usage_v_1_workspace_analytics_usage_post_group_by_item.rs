pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostGroupByItem {
    Model,
    RequestSource,
    FiatChargeType,
    FiatCurrency,
    UserId,
    HashedXiApiKey,
    Region,
    ReportingWorkspaceId,
    BillingGroupId,
    ProductType,
    Surface,
    Actor,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostGroupByItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Model => serializer.serialize_str("model"),
            Self::RequestSource => serializer.serialize_str("request_source"),
            Self::FiatChargeType => serializer.serialize_str("fiat_charge_type"),
            Self::FiatCurrency => serializer.serialize_str("fiat_currency"),
            Self::UserId => serializer.serialize_str("user_id"),
            Self::HashedXiApiKey => serializer.serialize_str("hashed_xi_api_key"),
            Self::Region => serializer.serialize_str("region"),
            Self::ReportingWorkspaceId => serializer.serialize_str("reporting_workspace_id"),
            Self::BillingGroupId => serializer.serialize_str("billing_group_id"),
            Self::ProductType => serializer.serialize_str("product_type"),
            Self::Surface => serializer.serialize_str("surface"),
            Self::Actor => serializer.serialize_str("actor"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostGroupByItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "model" => Ok(Self::Model),
            "request_source" => Ok(Self::RequestSource),
            "fiat_charge_type" => Ok(Self::FiatChargeType),
            "fiat_currency" => Ok(Self::FiatCurrency),
            "user_id" => Ok(Self::UserId),
            "hashed_xi_api_key" => Ok(Self::HashedXiApiKey),
            "region" => Ok(Self::Region),
            "reporting_workspace_id" => Ok(Self::ReportingWorkspaceId),
            "billing_group_id" => Ok(Self::BillingGroupId),
            "product_type" => Ok(Self::ProductType),
            "surface" => Ok(Self::Surface),
            "actor" => Ok(Self::Actor),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostGroupByItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Model => write!(f, "model"),
            Self::RequestSource => write!(f, "request_source"),
            Self::FiatChargeType => write!(f, "fiat_charge_type"),
            Self::FiatCurrency => write!(f, "fiat_currency"),
            Self::UserId => write!(f, "user_id"),
            Self::HashedXiApiKey => write!(f, "hashed_xi_api_key"),
            Self::Region => write!(f, "region"),
            Self::ReportingWorkspaceId => write!(f, "reporting_workspace_id"),
            Self::BillingGroupId => write!(f, "billing_group_id"),
            Self::ProductType => write!(f, "product_type"),
            Self::Surface => write!(f, "surface"),
            Self::Actor => write!(f, "actor"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
