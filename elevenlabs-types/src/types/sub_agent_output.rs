pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubAgentOutput {
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(default)]
    pub description: String,
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
    agent_id: Option<String>,
    branch_id: Option<String>,
    description: Option<String>,
    parameters: Option<ObjectJsonSchemaPropertyOutput>,
}

impl SubAgentOutputBuilder {
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
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            parameters: self.parameters,
        })
    }
}
