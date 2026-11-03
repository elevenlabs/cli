pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnalysisItemAttachmentAction {
    Attach,
    Detach,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AnalysisItemAttachmentAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Attach => serializer.serialize_str("attach"),
            Self::Detach => serializer.serialize_str("detach"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AnalysisItemAttachmentAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "attach" => Ok(Self::Attach),
            "detach" => Ok(Self::Detach),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AnalysisItemAttachmentAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attach => write!(f, "attach"),
            Self::Detach => write!(f, "detach"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
