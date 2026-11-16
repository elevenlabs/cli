pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubagentConfigOutput {
    /// Agents this agent can hand a single turn to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<SubAgentOutput>>,
    /// Lets this agent hand a single turn to a fresh copy of itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_subagent: Option<SelfSubagent>,
}

impl SubagentConfigOutput {
    pub fn builder() -> SubagentConfigOutputBuilder {
        <SubagentConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubagentConfigOutputBuilder {
    agents: Option<Vec<SubAgentOutput>>,
    self_subagent: Option<SelfSubagent>,
}

impl SubagentConfigOutputBuilder {
    pub fn agents(mut self, value: Vec<SubAgentOutput>) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn self_subagent(mut self, value: SelfSubagent) -> Self {
        self.self_subagent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubagentConfigOutput`].
    pub fn build(self) -> Result<SubagentConfigOutput, BuildError> {
        Ok(SubagentConfigOutput {
            agents: self.agents,
            self_subagent: self.self_subagent,
        })
    }
}
