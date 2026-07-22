pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestType {
    Llm,
    Tool,
    Simulation,
    Folder,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Llm => serializer.serialize_str("llm"),
            Self::Tool => serializer.serialize_str("tool"),
            Self::Simulation => serializer.serialize_str("simulation"),
            Self::Folder => serializer.serialize_str("folder"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "llm" => Ok(Self::Llm),
            "tool" => Ok(Self::Tool),
            "simulation" => Ok(Self::Simulation),
            "folder" => Ok(Self::Folder),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Llm => write!(f, "llm"),
            Self::Tool => write!(f, "tool"),
            Self::Simulation => write!(f, "simulation"),
            Self::Folder => write!(f, "folder"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
