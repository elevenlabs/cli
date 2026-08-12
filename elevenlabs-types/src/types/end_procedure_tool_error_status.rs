pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EndProcedureToolErrorStatus {
    NotFound,
    InvalidId,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EndProcedureToolErrorStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotFound => serializer.serialize_str("not_found"),
            Self::InvalidId => serializer.serialize_str("invalid_id"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EndProcedureToolErrorStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_found" => Ok(Self::NotFound),
            "invalid_id" => Ok(Self::InvalidId),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EndProcedureToolErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "not_found"),
            Self::InvalidId => write!(f, "invalid_id"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
