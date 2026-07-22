pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CriteriaScoringMode {
    Binary,
    NumericUniform,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CriteriaScoringMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Binary => serializer.serialize_str("binary"),
            Self::NumericUniform => serializer.serialize_str("numeric_uniform"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CriteriaScoringMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "binary" => Ok(Self::Binary),
            "numeric_uniform" => Ok(Self::NumericUniform),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CriteriaScoringMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binary => write!(f, "binary"),
            Self::NumericUniform => write!(f, "numeric_uniform"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
