pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentMessagesPersistedAgent {
    pub r#type: String,
    #[serde(default)]
    pub agent_id: String,
    /// ID of the agent branch to use for this conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Changes to the agent configuration for this conversation. The agent must allow each field that is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<AgentMessagesOverrides>,
}

impl AgentMessagesPersistedAgent {
    pub fn builder() -> AgentMessagesPersistedAgentBuilder {
        <AgentMessagesPersistedAgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentMessagesPersistedAgentBuilder {
    r#type: Option<String>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    overrides: Option<AgentMessagesOverrides>,
}

impl AgentMessagesPersistedAgentBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
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

    pub fn overrides(mut self, value: AgentMessagesOverrides) -> Self {
        self.overrides = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentMessagesPersistedAgent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AgentMessagesPersistedAgentBuilder::r#type)
    /// - [`agent_id`](AgentMessagesPersistedAgentBuilder::agent_id)
    pub fn build(self) -> Result<AgentMessagesPersistedAgent, BuildError> {
        Ok(AgentMessagesPersistedAgent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
            overrides: self.overrides,
        })
    }
}
