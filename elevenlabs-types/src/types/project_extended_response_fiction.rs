pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectExtendedResponseFiction {
    Fiction,
    NonFiction,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProjectExtendedResponseFiction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fiction => serializer.serialize_str("fiction"),
            Self::NonFiction => serializer.serialize_str("non-fiction"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectExtendedResponseFiction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fiction" => Ok(Self::Fiction),
            "non-fiction" => Ok(Self::NonFiction),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProjectExtendedResponseFiction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fiction => write!(f, "fiction"),
            Self::NonFiction => write!(f, "non-fiction"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
