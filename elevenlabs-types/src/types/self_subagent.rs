pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The agent itself, run on the same branch with none of the conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SelfSubagent {
    /// Whether to wait for the reply, or keep going and get it later in the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<SelfSubagentExecutionMode>,
    /// How long the agent may run before it's stopped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<i64>,
    /// When the agent should hand work to itself. Defaults to a built-in description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl SelfSubagent {
    pub fn builder() -> SelfSubagentBuilder {
        <SelfSubagentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SelfSubagentBuilder {
    execution_mode: Option<SelfSubagentExecutionMode>,
    timeout_secs: Option<i64>,
    description: Option<String>,
}

impl SelfSubagentBuilder {
    pub fn execution_mode(mut self, value: SelfSubagentExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn timeout_secs(mut self, value: i64) -> Self {
        self.timeout_secs = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SelfSubagent`].
    pub fn build(self) -> Result<SelfSubagent, BuildError> {
        Ok(SelfSubagent {
            execution_mode: self.execution_mode,
            timeout_secs: self.timeout_secs,
            description: self.description,
        })
    }
}
