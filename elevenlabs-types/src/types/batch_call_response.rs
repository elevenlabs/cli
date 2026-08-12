pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BatchCallResponse {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_provider: Option<TelephonyProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_params: Option<BatchCallWhatsAppParams>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default)]
    pub created_at_unix: i64,
    #[serde(default)]
    pub scheduled_time_unix: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default)]
    pub total_calls_dispatched: i64,
    #[serde(default)]
    pub total_calls_scheduled: i64,
    #[serde(default)]
    pub total_calls_finished: i64,
    #[serde(default)]
    pub last_updated_at_unix: i64,
    pub status: BatchCallStatus,
    #[serde(default)]
    pub retry_count: i64,
    #[serde(default)]
    pub telephony_call_config: TelephonyCallConfig,
    /// Maximum number of simultaneous calls for this batch. When set, dispatch is governed by this limit rather than workspace/agent capacity percentages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_concurrency_limit: Option<i64>,
    #[serde(default)]
    pub agent_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
}

impl BatchCallResponse {
    pub fn builder() -> BatchCallResponseBuilder {
        <BatchCallResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchCallResponseBuilder {
    id: Option<String>,
    phone_number_id: Option<String>,
    phone_provider: Option<TelephonyProvider>,
    whatsapp_params: Option<BatchCallWhatsAppParams>,
    name: Option<String>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    environment: Option<String>,
    created_at_unix: Option<i64>,
    scheduled_time_unix: Option<i64>,
    timezone: Option<String>,
    total_calls_dispatched: Option<i64>,
    total_calls_scheduled: Option<i64>,
    total_calls_finished: Option<i64>,
    last_updated_at_unix: Option<i64>,
    status: Option<BatchCallStatus>,
    retry_count: Option<i64>,
    telephony_call_config: Option<TelephonyCallConfig>,
    target_concurrency_limit: Option<i64>,
    agent_name: Option<String>,
    branch_name: Option<String>,
}

impl BatchCallResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.phone_number_id = Some(value.into());
        self
    }

    pub fn phone_provider(mut self, value: TelephonyProvider) -> Self {
        self.phone_provider = Some(value);
        self
    }

    pub fn whatsapp_params(mut self, value: BatchCallWhatsAppParams) -> Self {
        self.whatsapp_params = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
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

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
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

    pub fn total_calls_dispatched(mut self, value: i64) -> Self {
        self.total_calls_dispatched = Some(value);
        self
    }

    pub fn total_calls_scheduled(mut self, value: i64) -> Self {
        self.total_calls_scheduled = Some(value);
        self
    }

    pub fn total_calls_finished(mut self, value: i64) -> Self {
        self.total_calls_finished = Some(value);
        self
    }

    pub fn last_updated_at_unix(mut self, value: i64) -> Self {
        self.last_updated_at_unix = Some(value);
        self
    }

    pub fn status(mut self, value: BatchCallStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn retry_count(mut self, value: i64) -> Self {
        self.retry_count = Some(value);
        self
    }

    pub fn telephony_call_config(mut self, value: TelephonyCallConfig) -> Self {
        self.telephony_call_config = Some(value);
        self
    }

    pub fn target_concurrency_limit(mut self, value: i64) -> Self {
        self.target_concurrency_limit = Some(value);
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn branch_name(mut self, value: impl Into<String>) -> Self {
        self.branch_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchCallResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BatchCallResponseBuilder::id)
    /// - [`name`](BatchCallResponseBuilder::name)
    /// - [`agent_id`](BatchCallResponseBuilder::agent_id)
    /// - [`created_at_unix`](BatchCallResponseBuilder::created_at_unix)
    /// - [`scheduled_time_unix`](BatchCallResponseBuilder::scheduled_time_unix)
    /// - [`total_calls_dispatched`](BatchCallResponseBuilder::total_calls_dispatched)
    /// - [`total_calls_scheduled`](BatchCallResponseBuilder::total_calls_scheduled)
    /// - [`total_calls_finished`](BatchCallResponseBuilder::total_calls_finished)
    /// - [`last_updated_at_unix`](BatchCallResponseBuilder::last_updated_at_unix)
    /// - [`status`](BatchCallResponseBuilder::status)
    /// - [`retry_count`](BatchCallResponseBuilder::retry_count)
    /// - [`telephony_call_config`](BatchCallResponseBuilder::telephony_call_config)
    /// - [`agent_name`](BatchCallResponseBuilder::agent_name)
    pub fn build(self) -> Result<BatchCallResponse, BuildError> {
        Ok(BatchCallResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            phone_number_id: self.phone_number_id,
            phone_provider: self.phone_provider,
            whatsapp_params: self.whatsapp_params,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
            environment: self.environment,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            scheduled_time_unix: self.scheduled_time_unix.ok_or_else(|| BuildError::missing_field("scheduled_time_unix"))?,
            timezone: self.timezone,
            total_calls_dispatched: self.total_calls_dispatched.ok_or_else(|| BuildError::missing_field("total_calls_dispatched"))?,
            total_calls_scheduled: self.total_calls_scheduled.ok_or_else(|| BuildError::missing_field("total_calls_scheduled"))?,
            total_calls_finished: self.total_calls_finished.ok_or_else(|| BuildError::missing_field("total_calls_finished"))?,
            last_updated_at_unix: self.last_updated_at_unix.ok_or_else(|| BuildError::missing_field("last_updated_at_unix"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            retry_count: self.retry_count.ok_or_else(|| BuildError::missing_field("retry_count"))?,
            telephony_call_config: self.telephony_call_config.ok_or_else(|| BuildError::missing_field("telephony_call_config"))?,
            target_concurrency_limit: self.target_concurrency_limit,
            agent_name: self.agent_name.ok_or_else(|| BuildError::missing_field("agent_name"))?,
            branch_name: self.branch_name,
        })
    }
}
