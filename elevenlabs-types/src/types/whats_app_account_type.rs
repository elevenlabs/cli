pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhatsAppAccountType {
    CloudApi,
    Coexistence,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WhatsAppAccountType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CloudApi => serializer.serialize_str("cloud_api"),
            Self::Coexistence => serializer.serialize_str("coexistence"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WhatsAppAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "cloud_api" => Ok(Self::CloudApi),
            "coexistence" => Ok(Self::Coexistence),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WhatsAppAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CloudApi => write!(f, "cloud_api"),
            Self::Coexistence => write!(f, "coexistence"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
