pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookAuthMethodType {
    Hmac,
    Oauth2,
    Mtls,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookAuthMethodType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Hmac => serializer.serialize_str("hmac"),
            Self::Oauth2 => serializer.serialize_str("oauth2"),
            Self::Mtls => serializer.serialize_str("mtls"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookAuthMethodType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "hmac" => Ok(Self::Hmac),
            "oauth2" => Ok(Self::Oauth2),
            "mtls" => Ok(Self::Mtls),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookAuthMethodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hmac => write!(f, "hmac"),
            Self::Oauth2 => write!(f, "oauth2"),
            Self::Mtls => write!(f, "mtls"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
