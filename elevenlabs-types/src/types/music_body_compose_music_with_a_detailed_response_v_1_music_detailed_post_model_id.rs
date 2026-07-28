pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The model to use for the generation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BodyComposeMusicWithADetailedResponseV1MusicDetailedPostModelId {
    MusicV1,
    MusicV2,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BodyComposeMusicWithADetailedResponseV1MusicDetailedPostModelId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MusicV1 => serializer.serialize_str("music_v1"),
            Self::MusicV2 => serializer.serialize_str("music_v2"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BodyComposeMusicWithADetailedResponseV1MusicDetailedPostModelId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "music_v1" => Ok(Self::MusicV1),
            "music_v2" => Ok(Self::MusicV2),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BodyComposeMusicWithADetailedResponseV1MusicDetailedPostModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MusicV1 => write!(f, "music_v1"),
            Self::MusicV2 => write!(f, "music_v2"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
