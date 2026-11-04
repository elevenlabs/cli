pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The template a business-initiated conversation opened with, and how far it got.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppOpeningMessage {
    #[serde(default)]
    pub template_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wamid: Option<String>,
    #[serde(default)]
    pub sent_at_unix: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replied_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable: Option<bool>,
}

impl WhatsAppOpeningMessage {
    pub fn builder() -> WhatsAppOpeningMessageBuilder {
        <WhatsAppOpeningMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppOpeningMessageBuilder {
    template_name: Option<String>,
    wamid: Option<String>,
    sent_at_unix: Option<i64>,
    delivered_at_unix: Option<i64>,
    read_at_unix: Option<i64>,
    replied_at_unix: Option<i64>,
    failed_at_unix: Option<i64>,
    error_code: Option<String>,
    pricing_category: Option<String>,
    billable: Option<bool>,
}

impl WhatsAppOpeningMessageBuilder {
    pub fn template_name(mut self, value: impl Into<String>) -> Self {
        self.template_name = Some(value.into());
        self
    }

    pub fn wamid(mut self, value: impl Into<String>) -> Self {
        self.wamid = Some(value.into());
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

    pub fn error_code(mut self, value: impl Into<String>) -> Self {
        self.error_code = Some(value.into());
        self
    }

    pub fn pricing_category(mut self, value: impl Into<String>) -> Self {
        self.pricing_category = Some(value.into());
        self
    }

    pub fn billable(mut self, value: bool) -> Self {
        self.billable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppOpeningMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`template_name`](WhatsAppOpeningMessageBuilder::template_name)
    /// - [`sent_at_unix`](WhatsAppOpeningMessageBuilder::sent_at_unix)
    pub fn build(self) -> Result<WhatsAppOpeningMessage, BuildError> {
        Ok(WhatsAppOpeningMessage {
            template_name: self.template_name.ok_or_else(|| BuildError::missing_field("template_name"))?,
            wamid: self.wamid,
            sent_at_unix: self.sent_at_unix.ok_or_else(|| BuildError::missing_field("sent_at_unix"))?,
            delivered_at_unix: self.delivered_at_unix,
            read_at_unix: self.read_at_unix,
            replied_at_unix: self.replied_at_unix,
            failed_at_unix: self.failed_at_unix,
            error_code: self.error_code,
            pricing_category: self.pricing_category,
            billable: self.billable,
        })
    }
}
