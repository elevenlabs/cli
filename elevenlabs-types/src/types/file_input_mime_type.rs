pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileInputMimeType {
    ImagePng,
    ImageJpeg,
    ImageGif,
    ImageWebp,
    ApplicationPdf,
    TextPlain,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FileInputMimeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ImagePng => serializer.serialize_str("image/png"),
            Self::ImageJpeg => serializer.serialize_str("image/jpeg"),
            Self::ImageGif => serializer.serialize_str("image/gif"),
            Self::ImageWebp => serializer.serialize_str("image/webp"),
            Self::ApplicationPdf => serializer.serialize_str("application/pdf"),
            Self::TextPlain => serializer.serialize_str("text/plain"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FileInputMimeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "image/png" => Ok(Self::ImagePng),
            "image/jpeg" => Ok(Self::ImageJpeg),
            "image/gif" => Ok(Self::ImageGif),
            "image/webp" => Ok(Self::ImageWebp),
            "application/pdf" => Ok(Self::ApplicationPdf),
            "text/plain" => Ok(Self::TextPlain),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FileInputMimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImagePng => write!(f, "image/png"),
            Self::ImageJpeg => write!(f, "image/jpeg"),
            Self::ImageGif => write!(f, "image/gif"),
            Self::ImageWebp => write!(f, "image/webp"),
            Self::ApplicationPdf => write!(f, "application/pdf"),
            Self::TextPlain => write!(f, "text/plain"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
