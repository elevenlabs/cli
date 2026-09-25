pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The MIME type of the encoded media.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InlineBase64ReferenceMimeType {
    ImageJpeg,
    ImagePng,
    ImageWebp,
    ImageHeic,
    ImageHeif,
    AudioMpeg,
    AudioWav,
    VideoMp4,
    VideoQuicktime,
    VideoWebm,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InlineBase64ReferenceMimeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ImageJpeg => serializer.serialize_str("image/jpeg"),
            Self::ImagePng => serializer.serialize_str("image/png"),
            Self::ImageWebp => serializer.serialize_str("image/webp"),
            Self::ImageHeic => serializer.serialize_str("image/heic"),
            Self::ImageHeif => serializer.serialize_str("image/heif"),
            Self::AudioMpeg => serializer.serialize_str("audio/mpeg"),
            Self::AudioWav => serializer.serialize_str("audio/wav"),
            Self::VideoMp4 => serializer.serialize_str("video/mp4"),
            Self::VideoQuicktime => serializer.serialize_str("video/quicktime"),
            Self::VideoWebm => serializer.serialize_str("video/webm"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InlineBase64ReferenceMimeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "image/jpeg" => Ok(Self::ImageJpeg),
            "image/png" => Ok(Self::ImagePng),
            "image/webp" => Ok(Self::ImageWebp),
            "image/heic" => Ok(Self::ImageHeic),
            "image/heif" => Ok(Self::ImageHeif),
            "audio/mpeg" => Ok(Self::AudioMpeg),
            "audio/wav" => Ok(Self::AudioWav),
            "video/mp4" => Ok(Self::VideoMp4),
            "video/quicktime" => Ok(Self::VideoQuicktime),
            "video/webm" => Ok(Self::VideoWebm),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InlineBase64ReferenceMimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImageJpeg => write!(f, "image/jpeg"),
            Self::ImagePng => write!(f, "image/png"),
            Self::ImageWebp => write!(f, "image/webp"),
            Self::ImageHeic => write!(f, "image/heic"),
            Self::ImageHeif => write!(f, "image/heif"),
            Self::AudioMpeg => write!(f, "audio/mpeg"),
            Self::AudioWav => write!(f, "audio/wav"),
            Self::VideoMp4 => write!(f, "video/mp4"),
            Self::VideoQuicktime => write!(f, "video/quicktime"),
            Self::VideoWebm => write!(f, "video/webm"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
