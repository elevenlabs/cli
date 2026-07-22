pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DirectPublishingReadResponseModelPayoutType {
    None,
    EngagementBased,
    FixedPayout,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DirectPublishingReadResponseModelPayoutType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::EngagementBased => serializer.serialize_str("engagement_based"),
            Self::FixedPayout => serializer.serialize_str("fixed_payout"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DirectPublishingReadResponseModelPayoutType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "engagement_based" => Ok(Self::EngagementBased),
            "fixed_payout" => Ok(Self::FixedPayout),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DirectPublishingReadResponseModelPayoutType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::EngagementBased => write!(f, "engagement_based"),
            Self::FixedPayout => write!(f, "fixed_payout"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
