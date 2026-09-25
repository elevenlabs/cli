pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneNumberSortBy {
    Label,
    PhoneNumber,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PhoneNumberSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Label => serializer.serialize_str("label"),
            Self::PhoneNumber => serializer.serialize_str("phone_number"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PhoneNumberSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "label" => Ok(Self::Label),
            "phone_number" => Ok(Self::PhoneNumber),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PhoneNumberSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Label => write!(f, "label"),
            Self::PhoneNumber => write!(f, "phone_number"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
