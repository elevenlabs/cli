pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SecretDependencyResourceType {
    Tools,
    Agents,
    PhoneNumbers,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SecretDependencyResourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Tools => serializer.serialize_str("tools"),
            Self::Agents => serializer.serialize_str("agents"),
            Self::PhoneNumbers => serializer.serialize_str("phone_numbers"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SecretDependencyResourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "tools" => Ok(Self::Tools),
            "agents" => Ok(Self::Agents),
            "phone_numbers" => Ok(Self::PhoneNumbers),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SecretDependencyResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tools => write!(f, "tools"),
            Self::Agents => write!(f, "agents"),
            Self::PhoneNumbers => write!(f, "phone_numbers"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
