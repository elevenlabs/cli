pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of the principal: user, group, or service account (under 'key').
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShareOptionResponseModelType {
    User,
    Group,
    Key,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ShareOptionResponseModelType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::User => serializer.serialize_str("user"),
            Self::Group => serializer.serialize_str("group"),
            Self::Key => serializer.serialize_str("key"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ShareOptionResponseModelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user" => Ok(Self::User),
            "group" => Ok(Self::Group),
            "key" => Ok(Self::Key),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ShareOptionResponseModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Group => write!(f, "group"),
            Self::Key => write!(f, "key"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
