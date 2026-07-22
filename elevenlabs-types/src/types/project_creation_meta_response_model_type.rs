pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of the project creation action.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectCreationMetaResponseModelType {
    Blank,
    GeneratePodcast,
    AutoAssignVoices,
    DubVideo,
    ImportSpeech,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProjectCreationMetaResponseModelType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Blank => serializer.serialize_str("blank"),
            Self::GeneratePodcast => serializer.serialize_str("generate_podcast"),
            Self::AutoAssignVoices => serializer.serialize_str("auto_assign_voices"),
            Self::DubVideo => serializer.serialize_str("dub_video"),
            Self::ImportSpeech => serializer.serialize_str("import_speech"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectCreationMetaResponseModelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "blank" => Ok(Self::Blank),
            "generate_podcast" => Ok(Self::GeneratePodcast),
            "auto_assign_voices" => Ok(Self::AutoAssignVoices),
            "dub_video" => Ok(Self::DubVideo),
            "import_speech" => Ok(Self::ImportSpeech),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProjectCreationMetaResponseModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blank => write!(f, "blank"),
            Self::GeneratePodcast => write!(f, "generate_podcast"),
            Self::AutoAssignVoices => write!(f, "auto_assign_voices"),
            Self::DubVideo => write!(f, "dub_video"),
            Self::ImportSpeech => write!(f, "import_speech"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
