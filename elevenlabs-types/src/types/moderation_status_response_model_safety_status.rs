pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModerationStatusResponseModelSafetyStatus {
    AppealApproved,
    AppealDenied,
    FalsePositive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ModerationStatusResponseModelSafetyStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AppealApproved => serializer.serialize_str("appeal_approved"),
            Self::AppealDenied => serializer.serialize_str("appeal_denied"),
            Self::FalsePositive => serializer.serialize_str("false_positive"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ModerationStatusResponseModelSafetyStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "appeal_approved" => Ok(Self::AppealApproved),
            "appeal_denied" => Ok(Self::AppealDenied),
            "false_positive" => Ok(Self::FalsePositive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ModerationStatusResponseModelSafetyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AppealApproved => write!(f, "appeal_approved"),
            Self::AppealDenied => write!(f, "appeal_denied"),
            Self::FalsePositive => write!(f, "false_positive"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
