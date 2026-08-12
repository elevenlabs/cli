pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Separator for scopes
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApiIntegrationOAuth2AuthCodeResponseScopeSeparator {
    Space,
    Comma,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApiIntegrationOAuth2AuthCodeResponseScopeSeparator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Space => serializer.serialize_str(" "),
            Self::Comma => serializer.serialize_str(","),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApiIntegrationOAuth2AuthCodeResponseScopeSeparator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            " " => Ok(Self::Space),
            "," => Ok(Self::Comma),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApiIntegrationOAuth2AuthCodeResponseScopeSeparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Space => write!(f, " "),
            Self::Comma => write!(f, ","),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
