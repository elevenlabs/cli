pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Metadata for async conversation delivery (Zendesk, Slack, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AsyncConversationMetadata {
    pub delivery_status: AsyncConversationMetadataDeliveryStatus,
    #[serde(default)]
    pub delivery_timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_error: Option<String>,
    #[serde(default)]
    pub external_system: String,
    #[serde(default)]
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_retry_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_processed_external_message_id: Option<String>,
}

impl AsyncConversationMetadata {
    pub fn builder() -> AsyncConversationMetadataBuilder {
        <AsyncConversationMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsyncConversationMetadataBuilder {
    delivery_status: Option<AsyncConversationMetadataDeliveryStatus>,
    delivery_timestamp: Option<i64>,
    delivery_error: Option<String>,
    external_system: Option<String>,
    external_id: Option<String>,
    external_link: Option<String>,
    retry_count: Option<i64>,
    last_retry_timestamp: Option<i64>,
    last_processed_external_message_id: Option<String>,
}

impl AsyncConversationMetadataBuilder {
    pub fn delivery_status(mut self, value: AsyncConversationMetadataDeliveryStatus) -> Self {
        self.delivery_status = Some(value);
        self
    }

    pub fn delivery_timestamp(mut self, value: i64) -> Self {
        self.delivery_timestamp = Some(value);
        self
    }

    pub fn delivery_error(mut self, value: impl Into<String>) -> Self {
        self.delivery_error = Some(value.into());
        self
    }

    pub fn external_system(mut self, value: impl Into<String>) -> Self {
        self.external_system = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn external_link(mut self, value: impl Into<String>) -> Self {
        self.external_link = Some(value.into());
        self
    }

    pub fn retry_count(mut self, value: i64) -> Self {
        self.retry_count = Some(value);
        self
    }

    pub fn last_retry_timestamp(mut self, value: i64) -> Self {
        self.last_retry_timestamp = Some(value);
        self
    }

    pub fn last_processed_external_message_id(mut self, value: impl Into<String>) -> Self {
        self.last_processed_external_message_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AsyncConversationMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delivery_status`](AsyncConversationMetadataBuilder::delivery_status)
    /// - [`delivery_timestamp`](AsyncConversationMetadataBuilder::delivery_timestamp)
    /// - [`external_system`](AsyncConversationMetadataBuilder::external_system)
    /// - [`external_id`](AsyncConversationMetadataBuilder::external_id)
    pub fn build(self) -> Result<AsyncConversationMetadata, BuildError> {
        Ok(AsyncConversationMetadata {
            delivery_status: self.delivery_status.ok_or_else(|| BuildError::missing_field("delivery_status"))?,
            delivery_timestamp: self.delivery_timestamp.ok_or_else(|| BuildError::missing_field("delivery_timestamp"))?,
            delivery_error: self.delivery_error,
            external_system: self.external_system.ok_or_else(|| BuildError::missing_field("external_system"))?,
            external_id: self.external_id.ok_or_else(|| BuildError::missing_field("external_id"))?,
            external_link: self.external_link,
            retry_count: self.retry_count,
            last_retry_timestamp: self.last_retry_timestamp,
            last_processed_external_message_id: self.last_processed_external_message_id,
        })
    }
}
