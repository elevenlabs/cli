pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Allows the agent to detect when a voicemail system is encountered.
/// 
/// This tool should be invoked by the LLM when it detects that the call has been
/// answered by a voicemail system rather than a human. If a voicemail message
/// is configured, it will be played; otherwise the call will end immediately.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoicemailDetectionToolConfig {
    /// Optional message to leave on voicemail when detected. If not provided, the call will end immediately when voicemail is detected. Supports dynamic variables (e.g., {{system__time}}, {{system__call_duration_secs}}, {{custom_variable}}).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voicemail_message: Option<String>,
}

impl VoicemailDetectionToolConfig {
    pub fn builder() -> VoicemailDetectionToolConfigBuilder {
        <VoicemailDetectionToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoicemailDetectionToolConfigBuilder {
    voicemail_message: Option<String>,
}

impl VoicemailDetectionToolConfigBuilder {
    pub fn voicemail_message(mut self, value: impl Into<String>) -> Self {
        self.voicemail_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoicemailDetectionToolConfig`].
    pub fn build(self) -> Result<VoicemailDetectionToolConfig, BuildError> {
        Ok(VoicemailDetectionToolConfig {
            voicemail_message: self.voicemail_message,
        })
    }
}
