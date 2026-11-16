pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubAgentOutput {
    /// Whether to wait for the reply, or keep going and get it later in the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<SubAgentOutputExecutionMode>,
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
    pub parameters: Option<ObjectJsonSchemaPropertyOutput>,
}

impl SubAgentOutput {
    pub fn builder() -> SubAgentOutputBuilder {
        <SubAgentOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubAgentOutputBuilder {
    execution_mode: Option<SubAgentOutputExecutionMode>,
    timeout_secs: Option<i64>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    description: Option<String>,
    parameters: Option<ObjectJsonSchemaPropertyOutput>,
}

impl SubAgentOutputBuilder {
    pub fn execution_mode(mut self, value: SubAgentOutputExecutionMode) -> Self {
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

    pub fn parameters(mut self, value: ObjectJsonSchemaPropertyOutput) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubAgentOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](SubAgentOutputBuilder::agent_id)
    /// - [`description`](SubAgentOutputBuilder::description)
    pub fn build(self) -> Result<SubAgentOutput, BuildError> {
        Ok(SubAgentOutput {
            execution_mode: self.execution_mode,
            timeout_secs: self.timeout_secs,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            parameters: self.parameters,
        })
    }
}
