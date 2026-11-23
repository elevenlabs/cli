pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Which kind of regulated number a request or an issued number is for.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApplicationNumberType {
    Local,
    Mobile,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApplicationNumberType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Local => serializer.serialize_str("local"),
            Self::Mobile => serializer.serialize_str("mobile"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApplicationNumberType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "local" => Ok(Self::Local),
            "mobile" => Ok(Self::Mobile),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApplicationNumberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::Mobile => write!(f, "mobile"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
