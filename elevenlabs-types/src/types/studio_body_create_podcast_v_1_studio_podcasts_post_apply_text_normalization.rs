pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization {
    Auto,
    On,
    Off,
    ApplyEnglish,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::On => serializer.serialize_str("on"),
            Self::Off => serializer.serialize_str("off"),
            Self::ApplyEnglish => serializer.serialize_str("apply_english"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "apply_english" => Ok(Self::ApplyEnglish),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
            Self::ApplyEnglish => write!(f, "apply_english"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
