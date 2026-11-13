pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetronomeUsageColumnFilterOperation {
    Eq,
    In,
    Neq,
    NotIn,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MetronomeUsageColumnFilterOperation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eq => serializer.serialize_str("eq"),
            Self::In => serializer.serialize_str("in"),
            Self::Neq => serializer.serialize_str("neq"),
            Self::NotIn => serializer.serialize_str("not_in"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MetronomeUsageColumnFilterOperation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eq" => Ok(Self::Eq),
            "in" => Ok(Self::In),
            "neq" => Ok(Self::Neq),
            "not_in" => Ok(Self::NotIn),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MetronomeUsageColumnFilterOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "eq"),
            Self::In => write!(f, "in"),
            Self::Neq => write!(f, "neq"),
            Self::NotIn => write!(f, "not_in"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
