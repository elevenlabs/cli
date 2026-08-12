pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceAccessInfoAccessSource {
    Creator,
    Explicit,
    WorkspaceAdmin,
    WorkspaceDefault,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResourceAccessInfoAccessSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Creator => serializer.serialize_str("creator"),
            Self::Explicit => serializer.serialize_str("explicit"),
            Self::WorkspaceAdmin => serializer.serialize_str("workspace_admin"),
            Self::WorkspaceDefault => serializer.serialize_str("workspace_default"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResourceAccessInfoAccessSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "creator" => Ok(Self::Creator),
            "explicit" => Ok(Self::Explicit),
            "workspace_admin" => Ok(Self::WorkspaceAdmin),
            "workspace_default" => Ok(Self::WorkspaceDefault),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResourceAccessInfoAccessSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Creator => write!(f, "creator"),
            Self::Explicit => write!(f, "explicit"),
            Self::WorkspaceAdmin => write!(f, "workspace_admin"),
            Self::WorkspaceDefault => write!(f, "workspace_default"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
