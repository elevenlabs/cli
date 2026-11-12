pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AttentionView {
    #[serde(default)]
    pub attention_id: String,
    #[serde(default)]
    pub campaign_id: String,
    /// The cause, such as `agent_unavailable` or `phone_number_unavailable`.
    #[serde(default)]
    pub reason: String,
    /// What went wrong and how to fix it.
    #[serde(default)]
    pub detail: String,
    pub status: AttentionStatus,
    /// Whether a retry places the calls the item holds back. Items that can't be retried are closed by resolving them.
    #[serde(default)]
    pub retryable: bool,
    /// The note recorded on resolving the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(default)]
    pub created_at_unix: i64,
    #[serde(default)]
    pub updated_at_unix: i64,
}

impl AttentionView {
    pub fn builder() -> AttentionViewBuilder {
        <AttentionViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttentionViewBuilder {
    attention_id: Option<String>,
    campaign_id: Option<String>,
    reason: Option<String>,
    detail: Option<String>,
    status: Option<AttentionStatus>,
    retryable: Option<bool>,
    resolution: Option<String>,
    created_at_unix: Option<i64>,
    updated_at_unix: Option<i64>,
}

impl AttentionViewBuilder {
    pub fn attention_id(mut self, value: impl Into<String>) -> Self {
        self.attention_id = Some(value.into());
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn status(mut self, value: AttentionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn retryable(mut self, value: bool) -> Self {
        self.retryable = Some(value);
        self
    }

    pub fn resolution(mut self, value: impl Into<String>) -> Self {
        self.resolution = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn updated_at_unix(mut self, value: i64) -> Self {
        self.updated_at_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AttentionView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attention_id`](AttentionViewBuilder::attention_id)
    /// - [`campaign_id`](AttentionViewBuilder::campaign_id)
    /// - [`reason`](AttentionViewBuilder::reason)
    /// - [`detail`](AttentionViewBuilder::detail)
    /// - [`status`](AttentionViewBuilder::status)
    /// - [`retryable`](AttentionViewBuilder::retryable)
    /// - [`created_at_unix`](AttentionViewBuilder::created_at_unix)
    /// - [`updated_at_unix`](AttentionViewBuilder::updated_at_unix)
    pub fn build(self) -> Result<AttentionView, BuildError> {
        Ok(AttentionView {
            attention_id: self.attention_id.ok_or_else(|| BuildError::missing_field("attention_id"))?,
            campaign_id: self.campaign_id.ok_or_else(|| BuildError::missing_field("campaign_id"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            detail: self.detail.ok_or_else(|| BuildError::missing_field("detail"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            retryable: self.retryable.ok_or_else(|| BuildError::missing_field("retryable"))?,
            resolution: self.resolution,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            updated_at_unix: self.updated_at_unix.ok_or_else(|| BuildError::missing_field("updated_at_unix"))?,
        })
    }
}
