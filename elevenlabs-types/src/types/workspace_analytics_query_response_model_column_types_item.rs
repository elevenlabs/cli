pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkspaceAnalyticsQueryResponseModelColumnTypesItem {
    String_,
    Float,
    DateTime,
    Int,
    Bool,
    Json,
    Map,
    Array,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkspaceAnalyticsQueryResponseModelColumnTypesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::String_ => serializer.serialize_str("String"),
            Self::Float => serializer.serialize_str("Float"),
            Self::DateTime => serializer.serialize_str("DateTime"),
            Self::Int => serializer.serialize_str("Int"),
            Self::Bool => serializer.serialize_str("Bool"),
            Self::Json => serializer.serialize_str("JSON"),
            Self::Map => serializer.serialize_str("Map"),
            Self::Array => serializer.serialize_str("Array"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkspaceAnalyticsQueryResponseModelColumnTypesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "String" => Ok(Self::String_),
            "Float" => Ok(Self::Float),
            "DateTime" => Ok(Self::DateTime),
            "Int" => Ok(Self::Int),
            "Bool" => Ok(Self::Bool),
            "JSON" => Ok(Self::Json),
            "Map" => Ok(Self::Map),
            "Array" => Ok(Self::Array),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkspaceAnalyticsQueryResponseModelColumnTypesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String_ => write!(f, "String"),
            Self::Float => write!(f, "Float"),
            Self::DateTime => write!(f, "DateTime"),
            Self::Int => write!(f, "Int"),
            Self::Bool => write!(f, "Bool"),
            Self::Json => write!(f, "JSON"),
            Self::Map => write!(f, "Map"),
            Self::Array => write!(f, "Array"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
