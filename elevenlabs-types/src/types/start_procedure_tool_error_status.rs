pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StartProcedureToolErrorStatus {
    NotFound,
    InvalidName,
    AlreadyActive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StartProcedureToolErrorStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotFound => serializer.serialize_str("not_found"),
            Self::InvalidName => serializer.serialize_str("invalid_name"),
            Self::AlreadyActive => serializer.serialize_str("already_active"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StartProcedureToolErrorStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_found" => Ok(Self::NotFound),
            "invalid_name" => Ok(Self::InvalidName),
            "already_active" => Ok(Self::AlreadyActive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StartProcedureToolErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "not_found"),
            Self::InvalidName => write!(f, "invalid_name"),
            Self::AlreadyActive => write!(f, "already_active"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
