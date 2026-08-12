pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnvironmentVariablesListRequestType {
    String_,
    Secret,
    AuthConnection,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EnvironmentVariablesListRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::String_ => serializer.serialize_str("string"),
            Self::Secret => serializer.serialize_str("secret"),
            Self::AuthConnection => serializer.serialize_str("auth_connection"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EnvironmentVariablesListRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "string" => Ok(Self::String_),
            "secret" => Ok(Self::Secret),
            "auth_connection" => Ok(Self::AuthConnection),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EnvironmentVariablesListRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String_ => write!(f, "string"),
            Self::Secret => write!(f, "secret"),
            Self::AuthConnection => write!(f, "auth_connection"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
