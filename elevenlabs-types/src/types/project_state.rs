pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The state of the project.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectState {
    Creating,
    Default,
    Converting,
    InQueue,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProjectState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Creating => serializer.serialize_str("creating"),
            Self::Default => serializer.serialize_str("default"),
            Self::Converting => serializer.serialize_str("converting"),
            Self::InQueue => serializer.serialize_str("in_queue"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "creating" => Ok(Self::Creating),
            "default" => Ok(Self::Default),
            "converting" => Ok(Self::Converting),
            "in_queue" => Ok(Self::InQueue),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProjectState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Creating => write!(f, "creating"),
            Self::Default => write!(f, "default"),
            Self::Converting => write!(f, "converting"),
            Self::InQueue => write!(f, "in_queue"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
