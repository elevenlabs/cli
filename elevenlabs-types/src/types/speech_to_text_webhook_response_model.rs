pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpeechToTextWebhookResponseModel {
    /// The message of the webhook response.
    #[serde(default)]
    pub message: String,
    /// The request ID of the webhook response.
    #[serde(default)]
    pub request_id: String,
    /// The transcription ID of the webhook response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_id: Option<String>,
}

impl SpeechToTextWebhookResponseModel {
    pub fn builder() -> SpeechToTextWebhookResponseModelBuilder {
        <SpeechToTextWebhookResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechToTextWebhookResponseModelBuilder {
    message: Option<String>,
    request_id: Option<String>,
    transcription_id: Option<String>,
}

impl SpeechToTextWebhookResponseModelBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn transcription_id(mut self, value: impl Into<String>) -> Self {
        self.transcription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpeechToTextWebhookResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](SpeechToTextWebhookResponseModelBuilder::message)
    /// - [`request_id`](SpeechToTextWebhookResponseModelBuilder::request_id)
    pub fn build(self) -> Result<SpeechToTextWebhookResponseModel, BuildError> {
        Ok(SpeechToTextWebhookResponseModel {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            transcription_id: self.transcription_id,
        })
    }
}
