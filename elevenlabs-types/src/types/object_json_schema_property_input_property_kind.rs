pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectJsonSchemaPropertyInputPropertyKind {
    Array,
    Object,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ObjectJsonSchemaPropertyInputPropertyKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Array => serializer.serialize_str("array"),
            Self::Object => serializer.serialize_str("object"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ObjectJsonSchemaPropertyInputPropertyKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "array" => Ok(Self::Array),
            "object" => Ok(Self::Object),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ObjectJsonSchemaPropertyInputPropertyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Array => write!(f, "array"),
            Self::Object => write!(f, "object"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
