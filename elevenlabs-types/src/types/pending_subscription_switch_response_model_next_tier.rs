pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The tier to change to.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PendingSubscriptionSwitchResponseModelNextTier {
    Free,
    Starter,
    Creator,
    Pro,
    GrowingBusiness,
    Scale20240810,
    GrantTier120250723,
    GrantTier220250723,
    Trial,
    Enterprise,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PendingSubscriptionSwitchResponseModelNextTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Free => serializer.serialize_str("free"),
            Self::Starter => serializer.serialize_str("starter"),
            Self::Creator => serializer.serialize_str("creator"),
            Self::Pro => serializer.serialize_str("pro"),
            Self::GrowingBusiness => serializer.serialize_str("growing_business"),
            Self::Scale20240810 => serializer.serialize_str("scale_2024_08_10"),
            Self::GrantTier120250723 => serializer.serialize_str("grant_tier_1_2025_07_23"),
            Self::GrantTier220250723 => serializer.serialize_str("grant_tier_2_2025_07_23"),
            Self::Trial => serializer.serialize_str("trial"),
            Self::Enterprise => serializer.serialize_str("enterprise"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PendingSubscriptionSwitchResponseModelNextTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "free" => Ok(Self::Free),
            "starter" => Ok(Self::Starter),
            "creator" => Ok(Self::Creator),
            "pro" => Ok(Self::Pro),
            "growing_business" => Ok(Self::GrowingBusiness),
            "scale_2024_08_10" => Ok(Self::Scale20240810),
            "grant_tier_1_2025_07_23" => Ok(Self::GrantTier120250723),
            "grant_tier_2_2025_07_23" => Ok(Self::GrantTier220250723),
            "trial" => Ok(Self::Trial),
            "enterprise" => Ok(Self::Enterprise),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PendingSubscriptionSwitchResponseModelNextTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Free => write!(f, "free"),
            Self::Starter => write!(f, "starter"),
            Self::Creator => write!(f, "creator"),
            Self::Pro => write!(f, "pro"),
            Self::GrowingBusiness => write!(f, "growing_business"),
            Self::Scale20240810 => write!(f, "scale_2024_08_10"),
            Self::GrantTier120250723 => write!(f, "grant_tier_1_2025_07_23"),
            Self::GrantTier220250723 => write!(f, "grant_tier_2_2025_07_23"),
            Self::Trial => write!(f, "trial"),
            Self::Enterprise => write!(f, "enterprise"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
