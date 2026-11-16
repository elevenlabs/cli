pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubagentConfigInput {
    /// Agents this agent can hand a single turn to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<SubAgentInput>>,
    /// Lets this agent hand a single turn to a fresh copy of itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_subagent: Option<SelfSubagent>,
}

impl SubagentConfigInput {
    pub fn builder() -> SubagentConfigInputBuilder {
        <SubagentConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubagentConfigInputBuilder {
    agents: Option<Vec<SubAgentInput>>,
    self_subagent: Option<SelfSubagent>,
}

impl SubagentConfigInputBuilder {
    pub fn agents(mut self, value: Vec<SubAgentInput>) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn self_subagent(mut self, value: SelfSubagent) -> Self {
        self.self_subagent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubagentConfigInput`].
    pub fn build(self) -> Result<SubagentConfigInput, BuildError> {
        Ok(SubagentConfigInput {
            agents: self.agents,
            self_subagent: self.self_subagent,
        })
    }
}
