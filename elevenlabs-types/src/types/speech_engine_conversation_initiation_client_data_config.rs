pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpeechEngineConversationInitiationClientDataConfig {
    /// Whether the first message can be overridden by the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_message: Option<bool>,
}

impl SpeechEngineConversationInitiationClientDataConfig {
    pub fn builder() -> SpeechEngineConversationInitiationClientDataConfigBuilder {
        <SpeechEngineConversationInitiationClientDataConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechEngineConversationInitiationClientDataConfigBuilder {
    first_message: Option<bool>,
}

impl SpeechEngineConversationInitiationClientDataConfigBuilder {
    pub fn first_message(mut self, value: bool) -> Self {
        self.first_message = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeechEngineConversationInitiationClientDataConfig`].
    pub fn build(self) -> Result<SpeechEngineConversationInitiationClientDataConfig, BuildError> {
        Ok(SpeechEngineConversationInitiationClientDataConfig {
            first_message: self.first_message,
        })
    }
}
