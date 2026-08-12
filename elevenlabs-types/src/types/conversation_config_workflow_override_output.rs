pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationConfigWorkflowOverrideOutput {
    /// If enabled audio will not be processed and only text will be used, use to avoid audio pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_only: Option<bool>,
    /// The maximum duration of a conversation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_seconds: Option<i64>,
    /// The events that will be sent to the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_events: Option<Vec<ClientEvent>>,
    /// Configuration for file input (image/PDF uploads) during conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_input: Option<FileInputConfigWorkflowOverride>,
    /// Enable real-time monitoring of conversations via WebSocket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_enabled: Option<bool>,
    /// The events that will be sent to monitoring connections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_events: Option<Vec<ClientEvent>>,
    /// Configuration for background sound during conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_sound: Option<BackgroundSoundConfigWorkflowOverride>,
    /// When enabled and knowledge base content is present, the LLM is instructed to report which sources it used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_attribution: Option<bool>,
}

impl ConversationConfigWorkflowOverrideOutput {
    pub fn builder() -> ConversationConfigWorkflowOverrideOutputBuilder {
        <ConversationConfigWorkflowOverrideOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationConfigWorkflowOverrideOutputBuilder {
    text_only: Option<bool>,
    max_duration_seconds: Option<i64>,
    client_events: Option<Vec<ClientEvent>>,
    file_input: Option<FileInputConfigWorkflowOverride>,
    monitoring_enabled: Option<bool>,
    monitoring_events: Option<Vec<ClientEvent>>,
    background_sound: Option<BackgroundSoundConfigWorkflowOverride>,
    source_attribution: Option<bool>,
}

impl ConversationConfigWorkflowOverrideOutputBuilder {
    pub fn text_only(mut self, value: bool) -> Self {
        self.text_only = Some(value);
        self
    }

    pub fn max_duration_seconds(mut self, value: i64) -> Self {
        self.max_duration_seconds = Some(value);
        self
    }

    pub fn client_events(mut self, value: Vec<ClientEvent>) -> Self {
        self.client_events = Some(value);
        self
    }

    pub fn file_input(mut self, value: FileInputConfigWorkflowOverride) -> Self {
        self.file_input = Some(value);
        self
    }

    pub fn monitoring_enabled(mut self, value: bool) -> Self {
        self.monitoring_enabled = Some(value);
        self
    }

    pub fn monitoring_events(mut self, value: Vec<ClientEvent>) -> Self {
        self.monitoring_events = Some(value);
        self
    }

    pub fn background_sound(mut self, value: BackgroundSoundConfigWorkflowOverride) -> Self {
        self.background_sound = Some(value);
        self
    }

    pub fn source_attribution(mut self, value: bool) -> Self {
        self.source_attribution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationConfigWorkflowOverrideOutput`].
    pub fn build(self) -> Result<ConversationConfigWorkflowOverrideOutput, BuildError> {
        Ok(ConversationConfigWorkflowOverrideOutput {
            text_only: self.text_only,
            max_duration_seconds: self.max_duration_seconds,
            client_events: self.client_events,
            file_input: self.file_input,
            monitoring_enabled: self.monitoring_enabled,
            monitoring_events: self.monitoring_events,
            background_sound: self.background_sound,
            source_attribution: self.source_attribution,
        })
    }
}
