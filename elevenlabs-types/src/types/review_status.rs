pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The review status of the voice.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReviewStatus {
    NotRequested,
    Pending,
    Declined,
    Allowed,
    AllowedWithChanges,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReviewStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotRequested => serializer.serialize_str("not_requested"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Declined => serializer.serialize_str("declined"),
            Self::Allowed => serializer.serialize_str("allowed"),
            Self::AllowedWithChanges => serializer.serialize_str("allowed_with_changes"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReviewStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_requested" => Ok(Self::NotRequested),
            "pending" => Ok(Self::Pending),
            "declined" => Ok(Self::Declined),
            "allowed" => Ok(Self::Allowed),
            "allowed_with_changes" => Ok(Self::AllowedWithChanges),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReviewStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRequested => write!(f, "not_requested"),
            Self::Pending => write!(f, "pending"),
            Self::Declined => write!(f, "declined"),
            Self::Allowed => write!(f, "allowed"),
            Self::AllowedWithChanges => write!(f, "allowed_with_changes"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
