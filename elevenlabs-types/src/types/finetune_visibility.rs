pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinetuneVisibility {
    Private,
    Workspace,
    Public,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinetuneVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Private => serializer.serialize_str("private"),
            Self::Workspace => serializer.serialize_str("workspace"),
            Self::Public => serializer.serialize_str("public"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinetuneVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "private" => Ok(Self::Private),
            "workspace" => Ok(Self::Workspace),
            "public" => Ok(Self::Public),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinetuneVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Private => write!(f, "private"),
            Self::Workspace => write!(f, "workspace"),
            Self::Public => write!(f, "public"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
