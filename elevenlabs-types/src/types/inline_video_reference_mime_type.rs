pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The MIME type of the encoded video.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InlineVideoReferenceMimeType {
    VideoMp4,
    VideoQuicktime,
    VideoWebm,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InlineVideoReferenceMimeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::VideoMp4 => serializer.serialize_str("video/mp4"),
            Self::VideoQuicktime => serializer.serialize_str("video/quicktime"),
            Self::VideoWebm => serializer.serialize_str("video/webm"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InlineVideoReferenceMimeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "video/mp4" => Ok(Self::VideoMp4),
            "video/quicktime" => Ok(Self::VideoQuicktime),
            "video/webm" => Ok(Self::VideoWebm),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InlineVideoReferenceMimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VideoMp4 => write!(f, "video/mp4"),
            Self::VideoQuicktime => write!(f, "video/quicktime"),
            Self::VideoWebm => write!(f, "video/webm"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
