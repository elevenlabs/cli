pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientToolResultItemErrorType {
    UserRejected,
    ExternalServer,
    ExternalClient,
    CustomerAuth,
    ClientTimeout,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ClientToolResultItemErrorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UserRejected => serializer.serialize_str("user_rejected"),
            Self::ExternalServer => serializer.serialize_str("external_server"),
            Self::ExternalClient => serializer.serialize_str("external_client"),
            Self::CustomerAuth => serializer.serialize_str("customer_auth"),
            Self::ClientTimeout => serializer.serialize_str("client_timeout"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ClientToolResultItemErrorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user_rejected" => Ok(Self::UserRejected),
            "external_server" => Ok(Self::ExternalServer),
            "external_client" => Ok(Self::ExternalClient),
            "customer_auth" => Ok(Self::CustomerAuth),
            "client_timeout" => Ok(Self::ClientTimeout),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ClientToolResultItemErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserRejected => write!(f, "user_rejected"),
            Self::ExternalServer => write!(f, "external_server"),
            Self::ExternalClient => write!(f, "external_client"),
            Self::CustomerAuth => write!(f, "customer_auth"),
            Self::ClientTimeout => write!(f, "client_timeout"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
