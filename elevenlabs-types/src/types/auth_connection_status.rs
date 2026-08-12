pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Single status field shared by every auth type's stored credential.
/// 
/// OAuth values (``REFRESH_FAILED``, ``REVOKED``) are written by the OAuth
/// token-manager refresh path. ``CREDENTIAL_INVALID`` is written by the
/// tool execution path when an upstream response matches a credential's
/// ``failure_signatures`` entry (Bearer, Basic auth, etc.).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AuthConnectionStatus {
    Active,
    RefreshFailed,
    Revoked,
    CredentialInvalid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AuthConnectionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::RefreshFailed => serializer.serialize_str("refresh_failed"),
            Self::Revoked => serializer.serialize_str("revoked"),
            Self::CredentialInvalid => serializer.serialize_str("credential_invalid"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AuthConnectionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "refresh_failed" => Ok(Self::RefreshFailed),
            "revoked" => Ok(Self::Revoked),
            "credential_invalid" => Ok(Self::CredentialInvalid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AuthConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::RefreshFailed => write!(f, "refresh_failed"),
            Self::Revoked => write!(f, "revoked"),
            Self::CredentialInvalid => write!(f, "credential_invalid"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
