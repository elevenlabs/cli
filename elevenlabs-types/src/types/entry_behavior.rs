pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntryBehavior {
    GenerateImmediately,
    WaitForUser,
    Auto,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EntryBehavior {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::GenerateImmediately => serializer.serialize_str("generate_immediately"),
            Self::WaitForUser => serializer.serialize_str("wait_for_user"),
            Self::Auto => serializer.serialize_str("auto"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EntryBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "generate_immediately" => Ok(Self::GenerateImmediately),
            "wait_for_user" => Ok(Self::WaitForUser),
            "auto" => Ok(Self::Auto),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EntryBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenerateImmediately => write!(f, "generate_immediately"),
            Self::WaitForUser => write!(f, "wait_for_user"),
            Self::Auto => write!(f, "auto"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
