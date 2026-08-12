pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPost {
    #[serde(default)]
    pub call_name: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub recipients: Vec<OutboundCallRecipient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_phone_number_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_params: Option<BatchCallWhatsAppParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephony_call_config: Option<TelephonyCallConfig>,
    /// Maximum number of simultaneous calls for this batch. When set, dispatch is governed by this limit rather than workspace/agent capacity percentages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_concurrency_limit: Option<i64>,
}

impl BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPost {
    pub fn builder() -> BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPostBuilder {
        <BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPostBuilder {
    call_name: Option<String>,
    agent_id: Option<String>,
    recipients: Option<Vec<OutboundCallRecipient>>,
    scheduled_time_unix: Option<i64>,
    agent_phone_number_id: Option<String>,
    whatsapp_params: Option<BatchCallWhatsAppParams>,
    timezone: Option<String>,
    branch_id: Option<String>,
    environment: Option<String>,
    telephony_call_config: Option<TelephonyCallConfig>,
    target_concurrency_limit: Option<i64>,
}

impl BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPostBuilder {
    pub fn call_name(mut self, value: impl Into<String>) -> Self {
        self.call_name = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: Vec<OutboundCallRecipient>) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn scheduled_time_unix(mut self, value: i64) -> Self {
        self.scheduled_time_unix = Some(value);
        self
    }

    pub fn agent_phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.agent_phone_number_id = Some(value.into());
        self
    }

    pub fn whatsapp_params(mut self, value: BatchCallWhatsAppParams) -> Self {
        self.whatsapp_params = Some(value);
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

    pub fn telephony_call_config(mut self, value: TelephonyCallConfig) -> Self {
        self.telephony_call_config = Some(value);
        self
    }

    pub fn target_concurrency_limit(mut self, value: i64) -> Self {
        self.target_concurrency_limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`call_name`](BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPostBuilder::call_name)
    /// - [`agent_id`](BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPostBuilder::agent_id)
    /// - [`recipients`](BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPostBuilder::recipients)
    pub fn build(self) -> Result<BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPost, BuildError> {
        Ok(BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPost {
            call_name: self.call_name.ok_or_else(|| BuildError::missing_field("call_name"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            recipients: self.recipients.ok_or_else(|| BuildError::missing_field("recipients"))?,
            scheduled_time_unix: self.scheduled_time_unix,
            agent_phone_number_id: self.agent_phone_number_id,
            whatsapp_params: self.whatsapp_params,
            timezone: self.timezone,
            branch_id: self.branch_id,
            environment: self.environment,
            telephony_call_config: self.telephony_call_config,
            target_concurrency_limit: self.target_concurrency_limit,
        })
    }
}

