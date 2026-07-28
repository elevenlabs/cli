pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Token field to extract from the token endpoint response.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateOAuth2JwtRequestTokenResponseField {
    AccessToken,
    IdToken,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateOAuth2JwtRequestTokenResponseField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AccessToken => serializer.serialize_str("access_token"),
            Self::IdToken => serializer.serialize_str("id_token"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateOAuth2JwtRequestTokenResponseField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "access_token" => Ok(Self::AccessToken),
            "id_token" => Ok(Self::IdToken),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateOAuth2JwtRequestTokenResponseField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccessToken => write!(f, "access_token"),
            Self::IdToken => write!(f, "id_token"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
