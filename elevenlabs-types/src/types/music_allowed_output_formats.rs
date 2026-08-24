pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum MusicAllowedOutputFormats {
        AllowedOutputFormats(AllowedOutputFormats),

        MusicOnlyOutputFormats(MusicOnlyOutputFormats),
}

impl MusicAllowedOutputFormats {
    pub fn is_allowed_output_formats(&self) -> bool {
        matches!(self, Self::AllowedOutputFormats(_))
    }

    pub fn is_music_only_output_formats(&self) -> bool {
        matches!(self, Self::MusicOnlyOutputFormats(_))
    }


    pub fn as_allowed_output_formats(&self) -> Option<&AllowedOutputFormats> {
        match self {
                    Self::AllowedOutputFormats(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_allowed_output_formats(self) -> Option<AllowedOutputFormats> {
        match self {
                    Self::AllowedOutputFormats(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_music_only_output_formats(&self) -> Option<&MusicOnlyOutputFormats> {
        match self {
                    Self::MusicOnlyOutputFormats(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_music_only_output_formats(self) -> Option<MusicOnlyOutputFormats> {
        match self {
                    Self::MusicOnlyOutputFormats(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for MusicAllowedOutputFormats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllowedOutputFormats(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::MusicOnlyOutputFormats(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
