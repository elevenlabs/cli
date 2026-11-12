pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Each section set here replaces the whole matching section of the policy.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentPolicyOverrideOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calling_periods: Option<Vec<CallingPeriod>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<DataCollectionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followup_context: Option<FollowupContextPolicy>,
}

impl AgentPolicyOverrideOutput {
    pub fn builder() -> AgentPolicyOverrideOutputBuilder {
        <AgentPolicyOverrideOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentPolicyOverrideOutputBuilder {
    timezone: Option<String>,
    retry: Option<RetryPolicy>,
    calling_periods: Option<Vec<CallingPeriod>>,
    data_collection: Option<DataCollectionConfig>,
    followup_context: Option<FollowupContextPolicy>,
}

impl AgentPolicyOverrideOutputBuilder {
    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn retry(mut self, value: RetryPolicy) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn calling_periods(mut self, value: Vec<CallingPeriod>) -> Self {
        self.calling_periods = Some(value);
        self
    }

    pub fn data_collection(mut self, value: DataCollectionConfig) -> Self {
        self.data_collection = Some(value);
        self
    }

    pub fn followup_context(mut self, value: FollowupContextPolicy) -> Self {
        self.followup_context = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentPolicyOverrideOutput`].
    pub fn build(self) -> Result<AgentPolicyOverrideOutput, BuildError> {
        Ok(AgentPolicyOverrideOutput {
            timezone: self.timezone,
            retry: self.retry,
            calling_periods: self.calling_periods,
            data_collection: self.data_collection,
            followup_context: self.followup_context,
        })
    }
}
