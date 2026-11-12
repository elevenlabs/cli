pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LeadsListRequestStatus {
    InProgress,
    Finished,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LeadsListRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InProgress => serializer.serialize_str("in_progress"),
            Self::Finished => serializer.serialize_str("finished"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LeadsListRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "in_progress" => Ok(Self::InProgress),
            "finished" => Ok(Self::Finished),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LeadsListRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InProgress => write!(f, "in_progress"),
            Self::Finished => write!(f, "finished"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
