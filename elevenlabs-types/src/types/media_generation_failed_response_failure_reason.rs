pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The category of failure.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MediaGenerationFailedResponseFailureReason {
    Timeout,
    ModelError,
    Moderated,
    InvalidParameters,
    DependencyFailed,
    ChargingFailed,
    InternalError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MediaGenerationFailedResponseFailureReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Timeout => serializer.serialize_str("timeout"),
            Self::ModelError => serializer.serialize_str("model_error"),
            Self::Moderated => serializer.serialize_str("moderated"),
            Self::InvalidParameters => serializer.serialize_str("invalid_parameters"),
            Self::DependencyFailed => serializer.serialize_str("dependency_failed"),
            Self::ChargingFailed => serializer.serialize_str("charging_failed"),
            Self::InternalError => serializer.serialize_str("internal_error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MediaGenerationFailedResponseFailureReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "timeout" => Ok(Self::Timeout),
            "model_error" => Ok(Self::ModelError),
            "moderated" => Ok(Self::Moderated),
            "invalid_parameters" => Ok(Self::InvalidParameters),
            "dependency_failed" => Ok(Self::DependencyFailed),
            "charging_failed" => Ok(Self::ChargingFailed),
            "internal_error" => Ok(Self::InternalError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MediaGenerationFailedResponseFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timeout => write!(f, "timeout"),
            Self::ModelError => write!(f, "model_error"),
            Self::Moderated => write!(f, "moderated"),
            Self::InvalidParameters => write!(f, "invalid_parameters"),
            Self::DependencyFailed => write!(f, "dependency_failed"),
            Self::ChargingFailed => write!(f, "charging_failed"),
            Self::InternalError => write!(f, "internal_error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
