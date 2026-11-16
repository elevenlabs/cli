pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Media (SRTP) encryption policy for a SIP trunk.
/// 
/// Values match the Agents platform ``SIPMediaEncryptionEnum`` so the SMB
/// adapter maps by value.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmbSipMediaEncryption {
    Disabled,
    Allowed,
    Required,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SmbSipMediaEncryption {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Disabled => serializer.serialize_str("disabled"),
            Self::Allowed => serializer.serialize_str("allowed"),
            Self::Required => serializer.serialize_str("required"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SmbSipMediaEncryption {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "disabled" => Ok(Self::Disabled),
            "allowed" => Ok(Self::Allowed),
            "required" => Ok(Self::Required),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SmbSipMediaEncryption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disabled => write!(f, "disabled"),
            Self::Allowed => write!(f, "allowed"),
            Self::Required => write!(f, "required"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
