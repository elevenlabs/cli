pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RenderType {
    Mp4,
    Aac,
    Mp3,
    Wav,
    Aaf,
    TracksZip,
    ClipsZip,
    Zip,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RenderType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mp4 => serializer.serialize_str("mp4"),
            Self::Aac => serializer.serialize_str("aac"),
            Self::Mp3 => serializer.serialize_str("mp3"),
            Self::Wav => serializer.serialize_str("wav"),
            Self::Aaf => serializer.serialize_str("aaf"),
            Self::TracksZip => serializer.serialize_str("tracks_zip"),
            Self::ClipsZip => serializer.serialize_str("clips_zip"),
            Self::Zip => serializer.serialize_str("zip"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RenderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mp4" => Ok(Self::Mp4),
            "aac" => Ok(Self::Aac),
            "mp3" => Ok(Self::Mp3),
            "wav" => Ok(Self::Wav),
            "aaf" => Ok(Self::Aaf),
            "tracks_zip" => Ok(Self::TracksZip),
            "clips_zip" => Ok(Self::ClipsZip),
            "zip" => Ok(Self::Zip),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RenderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mp4 => write!(f, "mp4"),
            Self::Aac => write!(f, "aac"),
            Self::Mp3 => write!(f, "mp3"),
            Self::Wav => write!(f, "wav"),
            Self::Aaf => write!(f, "aaf"),
            Self::TracksZip => write!(f, "tracks_zip"),
            Self::ClipsZip => write!(f, "clips_zip"),
            Self::Zip => write!(f, "zip"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
