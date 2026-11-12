pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateOAuth2JwtRequestJwtUsage {
    AuthorizationGrant,
    ClientAuthentication,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateOAuth2JwtRequestJwtUsage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AuthorizationGrant => serializer.serialize_str("authorization_grant"),
            Self::ClientAuthentication => serializer.serialize_str("client_authentication"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateOAuth2JwtRequestJwtUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "authorization_grant" => Ok(Self::AuthorizationGrant),
            "client_authentication" => Ok(Self::ClientAuthentication),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateOAuth2JwtRequestJwtUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthorizationGrant => write!(f, "authorization_grant"),
            Self::ClientAuthentication => write!(f, "client_authentication"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
