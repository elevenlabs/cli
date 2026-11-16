pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Agents and branches that use an agent as a subagent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetAgentSubagentDependentsResponseModel {
    #[serde(default)]
    pub agents: Vec<GetAgentSubagentDependentsResponseModelAgentsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<DependentBranchInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl GetAgentSubagentDependentsResponseModel {
    pub fn builder() -> GetAgentSubagentDependentsResponseModelBuilder {
        <GetAgentSubagentDependentsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgentSubagentDependentsResponseModelBuilder {
    agents: Option<Vec<GetAgentSubagentDependentsResponseModelAgentsItem>>,
    branches: Option<Vec<DependentBranchInfo>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetAgentSubagentDependentsResponseModelBuilder {
    pub fn agents(mut self, value: Vec<GetAgentSubagentDependentsResponseModelAgentsItem>) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn branches(mut self, value: Vec<DependentBranchInfo>) -> Self {
        self.branches = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAgentSubagentDependentsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agents`](GetAgentSubagentDependentsResponseModelBuilder::agents)
    /// - [`has_more`](GetAgentSubagentDependentsResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetAgentSubagentDependentsResponseModel, BuildError> {
        Ok(GetAgentSubagentDependentsResponseModel {
            agents: self.agents.ok_or_else(|| BuildError::missing_field("agents"))?,
            branches: self.branches,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
