pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookSendResultResponseModel {
    /// The delivery ID, also sent in the ElevenLabs-Webhook-Delivery-Id header.
    #[serde(default)]
    pub delivery_id: String,
    /// What happened to this attempt.
    pub outcome: WebhookOutcome,
    /// The HTTP status your endpoint returned, if it answered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i64>,
    /// How long your endpoint took to answer, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i64>,
    /// The Content-Type your endpoint answered with, if it answered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_type: Option<String>,
    /// The size of your endpoint's response body in bytes, if it answered. The body itself is never stored or returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body_bytes: Option<i64>,
    /// Where a failure came from: recipient, configuration, external_transport or timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_origin: Option<String>,
}

impl WebhookSendResultResponseModel {
    pub fn builder() -> WebhookSendResultResponseModelBuilder {
        <WebhookSendResultResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookSendResultResponseModelBuilder {
    delivery_id: Option<String>,
    outcome: Option<WebhookOutcome>,
    response_code: Option<i64>,
    latency_ms: Option<i64>,
    response_content_type: Option<String>,
    response_body_bytes: Option<i64>,
    failure_origin: Option<String>,
}

impl WebhookSendResultResponseModelBuilder {
    pub fn delivery_id(mut self, value: impl Into<String>) -> Self {
        self.delivery_id = Some(value.into());
        self
    }

    pub fn outcome(mut self, value: WebhookOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn response_code(mut self, value: i64) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn latency_ms(mut self, value: i64) -> Self {
        self.latency_ms = Some(value);
        self
    }

    pub fn response_content_type(mut self, value: impl Into<String>) -> Self {
        self.response_content_type = Some(value.into());
        self
    }

    pub fn response_body_bytes(mut self, value: i64) -> Self {
        self.response_body_bytes = Some(value);
        self
    }

    pub fn failure_origin(mut self, value: impl Into<String>) -> Self {
        self.failure_origin = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookSendResultResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delivery_id`](WebhookSendResultResponseModelBuilder::delivery_id)
    /// - [`outcome`](WebhookSendResultResponseModelBuilder::outcome)
    pub fn build(self) -> Result<WebhookSendResultResponseModel, BuildError> {
        Ok(WebhookSendResultResponseModel {
            delivery_id: self.delivery_id.ok_or_else(|| BuildError::missing_field("delivery_id"))?,
            outcome: self.outcome.ok_or_else(|| BuildError::missing_field("outcome"))?,
            response_code: self.response_code,
            latency_ms: self.latency_ms,
            response_content_type: self.response_content_type,
            response_body_bytes: self.response_body_bytes,
            failure_origin: self.failure_origin,
        })
    }
}
