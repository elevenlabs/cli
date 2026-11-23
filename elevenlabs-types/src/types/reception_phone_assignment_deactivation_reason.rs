pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Why a phone assignment is currently inactive.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReceptionPhoneAssignmentDeactivationReason {
    TierDowngrade,
    NoEntitlement,
    User,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReceptionPhoneAssignmentDeactivationReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TierDowngrade => serializer.serialize_str("tier_downgrade"),
            Self::NoEntitlement => serializer.serialize_str("no_entitlement"),
            Self::User => serializer.serialize_str("user"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReceptionPhoneAssignmentDeactivationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "tier_downgrade" => Ok(Self::TierDowngrade),
            "no_entitlement" => Ok(Self::NoEntitlement),
            "user" => Ok(Self::User),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReceptionPhoneAssignmentDeactivationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TierDowngrade => write!(f, "tier_downgrade"),
            Self::NoEntitlement => write!(f, "no_entitlement"),
            Self::User => write!(f, "user"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
