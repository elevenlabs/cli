pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DubbingListRequestCreationSourcesItem {
    FlowNode,
    DubbingUi,
    DubbingApi,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DubbingListRequestCreationSourcesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FlowNode => serializer.serialize_str("flow_node"),
            Self::DubbingUi => serializer.serialize_str("dubbing_ui"),
            Self::DubbingApi => serializer.serialize_str("dubbing_api"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DubbingListRequestCreationSourcesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "flow_node" => Ok(Self::FlowNode),
            "dubbing_ui" => Ok(Self::DubbingUi),
            "dubbing_api" => Ok(Self::DubbingApi),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DubbingListRequestCreationSourcesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FlowNode => write!(f, "flow_node"),
            Self::DubbingUi => write!(f, "dubbing_ui"),
            Self::DubbingApi => write!(f, "dubbing_api"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
