pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionStatusType {
    Trialing,
    Active,
    Incomplete,
    PastDue,
    Free,
    FreeDisabled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubscriptionStatusType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Trialing => serializer.serialize_str("trialing"),
            Self::Active => serializer.serialize_str("active"),
            Self::Incomplete => serializer.serialize_str("incomplete"),
            Self::PastDue => serializer.serialize_str("past_due"),
            Self::Free => serializer.serialize_str("free"),
            Self::FreeDisabled => serializer.serialize_str("free_disabled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubscriptionStatusType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "trialing" => Ok(Self::Trialing),
            "active" => Ok(Self::Active),
            "incomplete" => Ok(Self::Incomplete),
            "past_due" => Ok(Self::PastDue),
            "free" => Ok(Self::Free),
            "free_disabled" => Ok(Self::FreeDisabled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubscriptionStatusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trialing => write!(f, "trialing"),
            Self::Active => write!(f, "active"),
            Self::Incomplete => write!(f, "incomplete"),
            Self::PastDue => write!(f, "past_due"),
            Self::Free => write!(f, "free"),
            Self::FreeDisabled => write!(f, "free_disabled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
