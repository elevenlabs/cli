pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FineTuningResponseStateValue {
    NotStarted,
    Queued,
    FineTuning,
    FineTuned,
    Failed,
    Delayed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FineTuningResponseStateValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotStarted => serializer.serialize_str("not_started"),
            Self::Queued => serializer.serialize_str("queued"),
            Self::FineTuning => serializer.serialize_str("fine_tuning"),
            Self::FineTuned => serializer.serialize_str("fine_tuned"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Delayed => serializer.serialize_str("delayed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FineTuningResponseStateValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_started" => Ok(Self::NotStarted),
            "queued" => Ok(Self::Queued),
            "fine_tuning" => Ok(Self::FineTuning),
            "fine_tuned" => Ok(Self::FineTuned),
            "failed" => Ok(Self::Failed),
            "delayed" => Ok(Self::Delayed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FineTuningResponseStateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotStarted => write!(f, "not_started"),
            Self::Queued => write!(f, "queued"),
            Self::FineTuning => write!(f, "fine_tuning"),
            Self::FineTuned => write!(f, "fine_tuned"),
            Self::Failed => write!(f, "failed"),
            Self::Delayed => write!(f, "delayed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
