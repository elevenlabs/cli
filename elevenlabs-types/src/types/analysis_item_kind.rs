pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnalysisItemKind {
    Evaluation,
    DataCollection,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AnalysisItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Evaluation => serializer.serialize_str("evaluation"),
            Self::DataCollection => serializer.serialize_str("data_collection"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AnalysisItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "evaluation" => Ok(Self::Evaluation),
            "data_collection" => Ok(Self::DataCollection),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AnalysisItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Evaluation => write!(f, "evaluation"),
            Self::DataCollection => write!(f, "data_collection"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
