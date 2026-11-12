pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CampaignVariant {
    Simple,
    Advanced,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CampaignVariant {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Simple => serializer.serialize_str("simple"),
            Self::Advanced => serializer.serialize_str("advanced"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CampaignVariant {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "simple" => Ok(Self::Simple),
            "advanced" => Ok(Self::Advanced),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CampaignVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simple => write!(f, "simple"),
            Self::Advanced => write!(f, "advanced"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
