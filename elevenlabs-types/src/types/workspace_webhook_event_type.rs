pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkspaceWebhookEventType {
    VoiceLibraryRemovalNotice,
    SpeechToText,
    AgentQa,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkspaceWebhookEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::VoiceLibraryRemovalNotice => serializer.serialize_str("voice_library_removal_notice"),
            Self::SpeechToText => serializer.serialize_str("speech_to_text"),
            Self::AgentQa => serializer.serialize_str("agent_qa"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkspaceWebhookEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "voice_library_removal_notice" => Ok(Self::VoiceLibraryRemovalNotice),
            "speech_to_text" => Ok(Self::SpeechToText),
            "agent_qa" => Ok(Self::AgentQa),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkspaceWebhookEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VoiceLibraryRemovalNotice => write!(f, "voice_library_removal_notice"),
            Self::SpeechToText => write!(f, "speech_to_text"),
            Self::AgentQa => write!(f, "agent_qa"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
