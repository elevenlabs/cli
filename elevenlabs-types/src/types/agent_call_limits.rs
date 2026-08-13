pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentCallLimits {
    /// The maximum number of concurrent conversations. -1 indicates that there is no maximum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_concurrency_limit: Option<i64>,
    /// The maximum number of conversations per day
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_limit: Option<i64>,
    /// Whether to enable bursting. If true, exceeding workspace concurrency limit will be allowed up to 3 times the limit. Calls will be charged at double rate when exceeding the limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bursting_enabled: Option<bool>,
}

impl AgentCallLimits {
    pub fn builder() -> AgentCallLimitsBuilder {
        <AgentCallLimitsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentCallLimitsBuilder {
    agent_concurrency_limit: Option<i64>,
    daily_limit: Option<i64>,
    bursting_enabled: Option<bool>,
}

impl AgentCallLimitsBuilder {
    pub fn agent_concurrency_limit(mut self, value: i64) -> Self {
        self.agent_concurrency_limit = Some(value);
        self
    }

    pub fn daily_limit(mut self, value: i64) -> Self {
        self.daily_limit = Some(value);
        self
    }

    pub fn bursting_enabled(mut self, value: bool) -> Self {
        self.bursting_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentCallLimits`].
    pub fn build(self) -> Result<AgentCallLimits, BuildError> {
        Ok(AgentCallLimits {
            agent_concurrency_limit: self.agent_concurrency_limit,
            daily_limit: self.daily_limit,
            bursting_enabled: self.bursting_enabled,
        })
    }
}
