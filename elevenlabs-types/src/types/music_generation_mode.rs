pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MusicGenerationMode {
    Track,
    Loop,
    Ambience,
    VideoToMusic,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MusicGenerationMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Track => serializer.serialize_str("track"),
            Self::Loop => serializer.serialize_str("loop"),
            Self::Ambience => serializer.serialize_str("ambience"),
            Self::VideoToMusic => serializer.serialize_str("video_to_music"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MusicGenerationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "track" => Ok(Self::Track),
            "loop" => Ok(Self::Loop),
            "ambience" => Ok(Self::Ambience),
            "video_to_music" => Ok(Self::VideoToMusic),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MusicGenerationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Track => write!(f, "track"),
            Self::Loop => write!(f, "loop"),
            Self::Ambience => write!(f, "ambience"),
            Self::VideoToMusic => write!(f, "video_to_music"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
