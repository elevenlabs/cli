pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CampaignSource {
    Ui,
    Api,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CampaignSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ui => serializer.serialize_str("ui"),
            Self::Api => serializer.serialize_str("api"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CampaignSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ui" => Ok(Self::Ui),
            "api" => Ok(Self::Api),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CampaignSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ui => write!(f, "ui"),
            Self::Api => write!(f, "api"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
