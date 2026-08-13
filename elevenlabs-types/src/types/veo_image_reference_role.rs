pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How the model uses the image: `subject` places its subject or scene elements into the video; `style` transfers its visual style.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VeoImageReferenceRole {
    Subject,
    Style,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VeoImageReferenceRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Subject => serializer.serialize_str("subject"),
            Self::Style => serializer.serialize_str("style"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VeoImageReferenceRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "subject" => Ok(Self::Subject),
            "style" => Ok(Self::Style),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VeoImageReferenceRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Subject => write!(f, "subject"),
            Self::Style => write!(f, "style"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
