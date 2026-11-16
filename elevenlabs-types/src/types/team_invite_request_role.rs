pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The role the invitee joins with: 'workspace_admin' or 'workspace_member'.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TeamInviteRequestRole {
    WorkspaceAdmin,
    WorkspaceMember,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TeamInviteRequestRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WorkspaceAdmin => serializer.serialize_str("workspace_admin"),
            Self::WorkspaceMember => serializer.serialize_str("workspace_member"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TeamInviteRequestRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "workspace_admin" => Ok(Self::WorkspaceAdmin),
            "workspace_member" => Ok(Self::WorkspaceMember),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TeamInviteRequestRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkspaceAdmin => write!(f, "workspace_admin"),
            Self::WorkspaceMember => write!(f, "workspace_member"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
