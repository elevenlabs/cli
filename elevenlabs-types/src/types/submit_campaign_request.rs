pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitCampaignRequest {
    /// The campaign's name.
    #[serde(default)]
    pub call_name: String,
    /// The agent that places the calls.
    #[serde(default)]
    pub agent_id: String,
    /// Who to call. Each recipient becomes a lead of the campaign.
    #[serde(default)]
    pub recipients: Vec<SubmitCampaignRecipient>,
    /// When to start calling; omit to start now. Calls still wait for the next calling window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time_unix: Option<i64>,
    /// IANA timezone to read the calling windows in, overriding the policy's.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The agent branch to call with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// The environment to resolve the agent's environment variables in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// The most calls to place at once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_concurrency_limit: Option<i64>,
    /// Makes the request safe to retry: a repeat with the same key returns the campaign it created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// Policy to snapshot onto the campaign. When set, it wins over the workspace default and any per-agent override; when omitted, the workspace configuration must exist and supplies the policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<PolicyConfigInput>,
    /// `simple` runs one attempt per recipient with always-on calling windows and nothing collected, and cannot be combined with `policy`. `advanced` uses `policy` or the workspace configuration, and is the default for phone campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<CampaignVariant>,
    pub channel_config: SubmitCampaignRequestChannelConfig,
}

impl SubmitCampaignRequest {
    pub fn builder() -> SubmitCampaignRequestBuilder {
        <SubmitCampaignRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitCampaignRequestBuilder {
    call_name: Option<String>,
    agent_id: Option<String>,
    recipients: Option<Vec<SubmitCampaignRecipient>>,
    scheduled_time_unix: Option<i64>,
    timezone: Option<String>,
    branch_id: Option<String>,
    environment: Option<String>,
    target_concurrency_limit: Option<i64>,
    idempotency_key: Option<String>,
    policy: Option<PolicyConfigInput>,
    variant: Option<CampaignVariant>,
    channel_config: Option<SubmitCampaignRequestChannelConfig>,
}

impl SubmitCampaignRequestBuilder {
    pub fn call_name(mut self, value: impl Into<String>) -> Self {
        self.call_name = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: Vec<SubmitCampaignRecipient>) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn scheduled_time_unix(mut self, value: i64) -> Self {
        self.scheduled_time_unix = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
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

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn policy(mut self, value: PolicyConfigInput) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn variant(mut self, value: CampaignVariant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn channel_config(mut self, value: SubmitCampaignRequestChannelConfig) -> Self {
        self.channel_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitCampaignRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`call_name`](SubmitCampaignRequestBuilder::call_name)
    /// - [`agent_id`](SubmitCampaignRequestBuilder::agent_id)
    /// - [`recipients`](SubmitCampaignRequestBuilder::recipients)
    /// - [`channel_config`](SubmitCampaignRequestBuilder::channel_config)
    pub fn build(self) -> Result<SubmitCampaignRequest, BuildError> {
        Ok(SubmitCampaignRequest {
            call_name: self.call_name.ok_or_else(|| BuildError::missing_field("call_name"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            recipients: self.recipients.ok_or_else(|| BuildError::missing_field("recipients"))?,
            scheduled_time_unix: self.scheduled_time_unix,
            timezone: self.timezone,
            branch_id: self.branch_id,
            environment: self.environment,
            target_concurrency_limit: self.target_concurrency_limit,
            idempotency_key: self.idempotency_key,
            policy: self.policy,
            variant: self.variant,
            channel_config: self.channel_config.ok_or_else(|| BuildError::missing_field("channel_config"))?,
        })
    }
}

