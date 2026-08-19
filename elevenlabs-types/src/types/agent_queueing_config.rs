pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentQueueingConfig {
    /// Hold callers in a wait queue when the agent is at its concurrency limit, instead of rejecting them immediately
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Maximum time a caller can wait in the queue before being rejected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_timeout_seconds: Option<i64>,
}

impl AgentQueueingConfig {
    pub fn builder() -> AgentQueueingConfigBuilder {
        <AgentQueueingConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentQueueingConfigBuilder {
    enabled: Option<bool>,
    wait_timeout_seconds: Option<i64>,
}

impl AgentQueueingConfigBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn wait_timeout_seconds(mut self, value: i64) -> Self {
        self.wait_timeout_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentQueueingConfig`].
    pub fn build(self) -> Result<AgentQueueingConfig, BuildError> {
        Ok(AgentQueueingConfig {
            enabled: self.enabled,
            wait_timeout_seconds: self.wait_timeout_seconds,
        })
    }
}
