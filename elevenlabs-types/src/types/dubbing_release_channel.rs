pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DubbingReleaseChannel {
    Stable,
    Release,
    Experimental,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DubbingReleaseChannel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Stable => serializer.serialize_str("stable"),
            Self::Release => serializer.serialize_str("release"),
            Self::Experimental => serializer.serialize_str("experimental"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DubbingReleaseChannel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "stable" => Ok(Self::Stable),
            "release" => Ok(Self::Release),
            "experimental" => Ok(Self::Experimental),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DubbingReleaseChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stable => write!(f, "stable"),
            Self::Release => write!(f, "release"),
            Self::Experimental => write!(f, "experimental"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
