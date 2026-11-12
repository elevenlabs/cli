pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CalComConnectionHealthStatus {
    Healthy,
    NeedsReauth,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CalComConnectionHealthStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Healthy => serializer.serialize_str("healthy"),
            Self::NeedsReauth => serializer.serialize_str("needs_reauth"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CalComConnectionHealthStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "healthy" => Ok(Self::Healthy),
            "needs_reauth" => Ok(Self::NeedsReauth),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CalComConnectionHealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Healthy => write!(f, "healthy"),
            Self::NeedsReauth => write!(f, "needs_reauth"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
