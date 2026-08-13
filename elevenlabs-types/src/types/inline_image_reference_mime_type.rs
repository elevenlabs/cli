pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The MIME type of the encoded image.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InlineImageReferenceMimeType {
    ImageJpeg,
    ImagePng,
    ImageWebp,
    ImageHeic,
    ImageHeif,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InlineImageReferenceMimeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ImageJpeg => serializer.serialize_str("image/jpeg"),
            Self::ImagePng => serializer.serialize_str("image/png"),
            Self::ImageWebp => serializer.serialize_str("image/webp"),
            Self::ImageHeic => serializer.serialize_str("image/heic"),
            Self::ImageHeif => serializer.serialize_str("image/heif"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InlineImageReferenceMimeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "image/jpeg" => Ok(Self::ImageJpeg),
            "image/png" => Ok(Self::ImagePng),
            "image/webp" => Ok(Self::ImageWebp),
            "image/heic" => Ok(Self::ImageHeic),
            "image/heif" => Ok(Self::ImageHeif),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InlineImageReferenceMimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImageJpeg => write!(f, "image/jpeg"),
            Self::ImagePng => write!(f, "image/png"),
            Self::ImageWebp => write!(f, "image/webp"),
            Self::ImageHeic => write!(f, "image/heic"),
            Self::ImageHeif => write!(f, "image/heif"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
