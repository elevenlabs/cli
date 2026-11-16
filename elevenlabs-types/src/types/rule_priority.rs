pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How strictly the receptionist follows a rule.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RulePriority {
    Strict,
    Suggestion,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RulePriority {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Strict => serializer.serialize_str("strict"),
            Self::Suggestion => serializer.serialize_str("suggestion"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RulePriority {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "strict" => Ok(Self::Strict),
            "suggestion" => Ok(Self::Suggestion),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RulePriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Strict => write!(f, "strict"),
            Self::Suggestion => write!(f, "suggestion"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
