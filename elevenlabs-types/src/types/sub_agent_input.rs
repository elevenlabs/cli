pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubAgentInput {
    /// Whether to wait for the reply, or keep going and get it later in the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<SubAgentInputExecutionMode>,
    /// How long the agent may run before it's stopped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<i64>,
    /// The agent to run. Must be in the same workspace.
    #[serde(default)]
    pub agent_id: String,
    /// The branch to run. Defaults to the agent's live deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// What the agent is for. Shown to the LLM when it picks an agent.
    #[serde(default)]
    pub description: String,
    /// Values passed to the agent as dynamic variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ObjectJsonSchemaPropertyInput>,
}

impl SubAgentInput {
    pub fn builder() -> SubAgentInputBuilder {
        <SubAgentInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubAgentInputBuilder {
    execution_mode: Option<SubAgentInputExecutionMode>,
    timeout_secs: Option<i64>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    description: Option<String>,
    parameters: Option<ObjectJsonSchemaPropertyInput>,
}

impl SubAgentInputBuilder {
    pub fn execution_mode(mut self, value: SubAgentInputExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn timeout_secs(mut self, value: i64) -> Self {
        self.timeout_secs = Some(value);
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

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: ObjectJsonSchemaPropertyInput) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubAgentInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](SubAgentInputBuilder::agent_id)
    /// - [`description`](SubAgentInputBuilder::description)
    pub fn build(self) -> Result<SubAgentInput, BuildError> {
        Ok(SubAgentInput {
            execution_mode: self.execution_mode,
            timeout_secs: self.timeout_secs,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            parameters: self.parameters,
        })
    }
}
