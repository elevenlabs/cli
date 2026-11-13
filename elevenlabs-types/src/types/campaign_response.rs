pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CampaignResponse {
    #[serde(default)]
    pub campaign_id: String,
    /// The campaign's name.
    #[serde(default)]
    pub call_name: String,
    #[serde(default)]
    pub agent_id: String,
    pub channel_config: CampaignResponseChannelConfig,
    /// `attention` means an open attention item holds back some of the campaign's calls while the rest go ahead. `stopping` means queued calls are cancelled and calls already placed are finishing.
    pub status: CampaignStatus,
    /// The policy the campaign took when it was created.
    #[serde(default)]
    pub policy: CampaignPolicySnapshot,
    pub variant: CampaignVariant,
    /// Whether the campaign was created in the app or through the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CampaignSource>,
    #[serde(default)]
    pub created_at_unix: i64,
    /// When the campaign starts calling; unset to start once created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time_unix: Option<i64>,
    #[serde(default)]
    pub updated_at_unix: i64,
    /// The agent branch the campaign calls with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// The environment the agent's environment variables resolve in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// The most calls the campaign places at once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_concurrency_limit: Option<i64>,
    pub channel: OutboundChannel,
    pub mode: OutboundMode,
    #[serde(default)]
    pub counts: CampaignSummaryCounts,
    #[serde(default)]
    pub agent_name: String,
}

impl CampaignResponse {
    pub fn builder() -> CampaignResponseBuilder {
        <CampaignResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignResponseBuilder {
    campaign_id: Option<String>,
    call_name: Option<String>,
    agent_id: Option<String>,
    channel_config: Option<CampaignResponseChannelConfig>,
    status: Option<CampaignStatus>,
    policy: Option<CampaignPolicySnapshot>,
    variant: Option<CampaignVariant>,
    source: Option<CampaignSource>,
    created_at_unix: Option<i64>,
    scheduled_time_unix: Option<i64>,
    updated_at_unix: Option<i64>,
    branch_id: Option<String>,
    environment: Option<String>,
    target_concurrency_limit: Option<i64>,
    channel: Option<OutboundChannel>,
    mode: Option<OutboundMode>,
    counts: Option<CampaignSummaryCounts>,
    agent_name: Option<String>,
}

impl CampaignResponseBuilder {
    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn call_name(mut self, value: impl Into<String>) -> Self {
        self.call_name = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn channel_config(mut self, value: CampaignResponseChannelConfig) -> Self {
        self.channel_config = Some(value);
        self
    }

    pub fn status(mut self, value: CampaignStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn policy(mut self, value: CampaignPolicySnapshot) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn variant(mut self, value: CampaignVariant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn source(mut self, value: CampaignSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn scheduled_time_unix(mut self, value: i64) -> Self {
        self.scheduled_time_unix = Some(value);
        self
    }

    pub fn updated_at_unix(mut self, value: i64) -> Self {
        self.updated_at_unix = Some(value);
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn target_concurrency_limit(mut self, value: i64) -> Self {
        self.target_concurrency_limit = Some(value);
        self
    }

    pub fn channel(mut self, value: OutboundChannel) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn mode(mut self, value: OutboundMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn counts(mut self, value: CampaignSummaryCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CampaignResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_id`](CampaignResponseBuilder::campaign_id)
    /// - [`call_name`](CampaignResponseBuilder::call_name)
    /// - [`agent_id`](CampaignResponseBuilder::agent_id)
    /// - [`channel_config`](CampaignResponseBuilder::channel_config)
    /// - [`status`](CampaignResponseBuilder::status)
    /// - [`policy`](CampaignResponseBuilder::policy)
    /// - [`variant`](CampaignResponseBuilder::variant)
    /// - [`created_at_unix`](CampaignResponseBuilder::created_at_unix)
    /// - [`updated_at_unix`](CampaignResponseBuilder::updated_at_unix)
    /// - [`channel`](CampaignResponseBuilder::channel)
    /// - [`mode`](CampaignResponseBuilder::mode)
    /// - [`counts`](CampaignResponseBuilder::counts)
    /// - [`agent_name`](CampaignResponseBuilder::agent_name)
    pub fn build(self) -> Result<CampaignResponse, BuildError> {
        Ok(CampaignResponse {
            campaign_id: self.campaign_id.ok_or_else(|| BuildError::missing_field("campaign_id"))?,
            call_name: self.call_name.ok_or_else(|| BuildError::missing_field("call_name"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            channel_config: self.channel_config.ok_or_else(|| BuildError::missing_field("channel_config"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            policy: self.policy.ok_or_else(|| BuildError::missing_field("policy"))?,
            variant: self.variant.ok_or_else(|| BuildError::missing_field("variant"))?,
            source: self.source,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            scheduled_time_unix: self.scheduled_time_unix,
            updated_at_unix: self.updated_at_unix.ok_or_else(|| BuildError::missing_field("updated_at_unix"))?,
            branch_id: self.branch_id,
            environment: self.environment,
            target_concurrency_limit: self.target_concurrency_limit,
            channel: self.channel.ok_or_else(|| BuildError::missing_field("channel"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            counts: self.counts.ok_or_else(|| BuildError::missing_field("counts"))?,
            agent_name: self.agent_name.ok_or_else(|| BuildError::missing_field("agent_name"))?,
        })
    }
}
