pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookUsageType {
    ConvAiAgentSettings,
    ConvAiSettings,
    VoiceLibraryRemovalNotices,
    SpeechToText,
    AgentQaEvaluations,
    ConvAiAlerting,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookUsageType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ConvAiAgentSettings => serializer.serialize_str("ConvAI Agent Settings"),
            Self::ConvAiSettings => serializer.serialize_str("ConvAI Settings"),
            Self::VoiceLibraryRemovalNotices => serializer.serialize_str("Voice Library Removal Notices"),
            Self::SpeechToText => serializer.serialize_str("Speech to Text"),
            Self::AgentQaEvaluations => serializer.serialize_str("Agent QA Evaluations"),
            Self::ConvAiAlerting => serializer.serialize_str("ConvAI Alerting"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookUsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ConvAI Agent Settings" => Ok(Self::ConvAiAgentSettings),
            "ConvAI Settings" => Ok(Self::ConvAiSettings),
            "Voice Library Removal Notices" => Ok(Self::VoiceLibraryRemovalNotices),
            "Speech to Text" => Ok(Self::SpeechToText),
            "Agent QA Evaluations" => Ok(Self::AgentQaEvaluations),
            "ConvAI Alerting" => Ok(Self::ConvAiAlerting),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookUsageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConvAiAgentSettings => write!(f, "ConvAI Agent Settings"),
            Self::ConvAiSettings => write!(f, "ConvAI Settings"),
            Self::VoiceLibraryRemovalNotices => write!(f, "Voice Library Removal Notices"),
            Self::SpeechToText => write!(f, "Speech to Text"),
            Self::AgentQaEvaluations => write!(f, "Agent QA Evaluations"),
            Self::ConvAiAlerting => write!(f, "ConvAI Alerting"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
