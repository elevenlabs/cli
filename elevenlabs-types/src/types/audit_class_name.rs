pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// OCSF event classes emitted in audit logs.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AuditClassName {
    AccountChange,
    Authentication,
    EntityManagement,
    UserAccessManagement,
    GroupManagement,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AuditClassName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AccountChange => serializer.serialize_str("Account Change"),
            Self::Authentication => serializer.serialize_str("Authentication"),
            Self::EntityManagement => serializer.serialize_str("Entity Management"),
            Self::UserAccessManagement => serializer.serialize_str("User Access Management"),
            Self::GroupManagement => serializer.serialize_str("Group Management"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AuditClassName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Account Change" => Ok(Self::AccountChange),
            "Authentication" => Ok(Self::Authentication),
            "Entity Management" => Ok(Self::EntityManagement),
            "User Access Management" => Ok(Self::UserAccessManagement),
            "Group Management" => Ok(Self::GroupManagement),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AuditClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountChange => write!(f, "Account Change"),
            Self::Authentication => write!(f, "Authentication"),
            Self::EntityManagement => write!(f, "Entity Management"),
            Self::UserAccessManagement => write!(f, "User Access Management"),
            Self::GroupManagement => write!(f, "Group Management"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
