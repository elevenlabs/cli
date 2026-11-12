pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LeadDeliveryView {
    pub status: BatchCallRecipientStatus,
    /// The furthest the message got, or `failed` if sending it failed.
    pub stage: LeadDeliveryViewStage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replied_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at_unix: Option<i64>,
    /// Why the message failed to reach the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl LeadDeliveryView {
    pub fn builder() -> LeadDeliveryViewBuilder {
        <LeadDeliveryViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadDeliveryViewBuilder {
    status: Option<BatchCallRecipientStatus>,
    stage: Option<LeadDeliveryViewStage>,
    sent_at_unix: Option<i64>,
    delivered_at_unix: Option<i64>,
    read_at_unix: Option<i64>,
    replied_at_unix: Option<i64>,
    failed_at_unix: Option<i64>,
    error_message: Option<String>,
}

impl LeadDeliveryViewBuilder {
    pub fn status(mut self, value: BatchCallRecipientStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn stage(mut self, value: LeadDeliveryViewStage) -> Self {
        self.stage = Some(value);
        self
    }

    pub fn sent_at_unix(mut self, value: i64) -> Self {
        self.sent_at_unix = Some(value);
        self
    }

    pub fn delivered_at_unix(mut self, value: i64) -> Self {
        self.delivered_at_unix = Some(value);
        self
    }

    pub fn read_at_unix(mut self, value: i64) -> Self {
        self.read_at_unix = Some(value);
        self
    }

    pub fn replied_at_unix(mut self, value: i64) -> Self {
        self.replied_at_unix = Some(value);
        self
    }

    pub fn failed_at_unix(mut self, value: i64) -> Self {
        self.failed_at_unix = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeadDeliveryView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](LeadDeliveryViewBuilder::status)
    /// - [`stage`](LeadDeliveryViewBuilder::stage)
    pub fn build(self) -> Result<LeadDeliveryView, BuildError> {
        Ok(LeadDeliveryView {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            stage: self.stage.ok_or_else(|| BuildError::missing_field("stage"))?,
            sent_at_unix: self.sent_at_unix,
            delivered_at_unix: self.delivered_at_unix,
            read_at_unix: self.read_at_unix,
            replied_at_unix: self.replied_at_unix,
            failed_at_unix: self.failed_at_unix,
            error_message: self.error_message,
        })
    }
}
