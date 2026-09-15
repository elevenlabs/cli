pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModerationStatusResponseModelWarningStatus {
    Warning,
    WarningCleared,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ModerationStatusResponseModelWarningStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Warning => serializer.serialize_str("warning"),
            Self::WarningCleared => serializer.serialize_str("warning_cleared"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ModerationStatusResponseModelWarningStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "warning" => Ok(Self::Warning),
            "warning_cleared" => Ok(Self::WarningCleared),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ModerationStatusResponseModelWarningStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Warning => write!(f, "warning"),
            Self::WarningCleared => write!(f, "warning_cleared"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
