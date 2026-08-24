pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConvAiWebhooks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_call_webhook_id: Option<String>,
    /// List of event types to send via webhook. Options: transcript, audio, call_initiation_failure, unredacted_transcript, unredacted_audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WebhookEventType>>,
    /// Format for transcript webhooks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_format: Option<WebhookTranscriptFormat>,
    /// DEPRECATED: Use 'events' field instead. Whether to send audio data with post-call webhooks for ConvAI conversations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_audio: Option<bool>,
}

impl ConvAiWebhooks {
    pub fn builder() -> ConvAiWebhooksBuilder {
        <ConvAiWebhooksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvAiWebhooksBuilder {
    post_call_webhook_id: Option<String>,
    events: Option<Vec<WebhookEventType>>,
    transcript_format: Option<WebhookTranscriptFormat>,
    send_audio: Option<bool>,
}

impl ConvAiWebhooksBuilder {
    pub fn post_call_webhook_id(mut self, value: impl Into<String>) -> Self {
        self.post_call_webhook_id = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<WebhookEventType>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn transcript_format(mut self, value: WebhookTranscriptFormat) -> Self {
        self.transcript_format = Some(value);
        self
    }

    pub fn send_audio(mut self, value: bool) -> Self {
        self.send_audio = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConvAiWebhooks`].
    pub fn build(self) -> Result<ConvAiWebhooks, BuildError> {
        Ok(ConvAiWebhooks {
            post_call_webhook_id: self.post_call_webhook_id,
            events: self.events,
            transcript_format: self.transcript_format,
            send_audio: self.send_audio,
        })
    }
}
