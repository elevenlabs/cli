pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DependentAvailableMcpServerIdentifierAccessLevel {
    Admin,
    Editor,
    Commenter,
    Viewer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DependentAvailableMcpServerIdentifierAccessLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Admin => serializer.serialize_str("admin"),
            Self::Editor => serializer.serialize_str("editor"),
            Self::Commenter => serializer.serialize_str("commenter"),
            Self::Viewer => serializer.serialize_str("viewer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DependentAvailableMcpServerIdentifierAccessLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "admin" => Ok(Self::Admin),
            "editor" => Ok(Self::Editor),
            "commenter" => Ok(Self::Commenter),
            "viewer" => Ok(Self::Viewer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DependentAvailableMcpServerIdentifierAccessLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admin => write!(f, "admin"),
            Self::Editor => write!(f, "editor"),
            Self::Commenter => write!(f, "commenter"),
            Self::Viewer => write!(f, "viewer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
