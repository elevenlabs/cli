pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DubbingListRequestDubbingModelsItem {
    DubbingV1,
    DubbingV2,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DubbingListRequestDubbingModelsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DubbingV1 => serializer.serialize_str("dubbing_v1"),
            Self::DubbingV2 => serializer.serialize_str("dubbing_v2"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DubbingListRequestDubbingModelsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "dubbing_v1" => Ok(Self::DubbingV1),
            "dubbing_v2" => Ok(Self::DubbingV2),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DubbingListRequestDubbingModelsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DubbingV1 => write!(f, "dubbing_v1"),
            Self::DubbingV2 => write!(f, "dubbing_v2"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
