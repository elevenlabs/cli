pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum MusicOutputFormat {
        MusicAllowedOutputFormats(MusicAllowedOutputFormats),

        String(String),
}

impl MusicOutputFormat {
    pub fn is_music_allowed_output_formats(&self) -> bool {
        matches!(self, Self::MusicAllowedOutputFormats(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }


    pub fn as_music_allowed_output_formats(&self) -> Option<&MusicAllowedOutputFormats> {
        match self {
                    Self::MusicAllowedOutputFormats(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_music_allowed_output_formats(self) -> Option<MusicAllowedOutputFormats> {
        match self {
                    Self::MusicAllowedOutputFormats(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }
}
