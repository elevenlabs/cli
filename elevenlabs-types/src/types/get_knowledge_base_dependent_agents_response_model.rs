pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetKnowledgeBaseDependentAgentsResponseModel {
    #[serde(default)]
    pub agents: Vec<GetKnowledgeBaseDependentAgentsResponseModelAgentsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<DependentBranchInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl GetKnowledgeBaseDependentAgentsResponseModel {
    pub fn builder() -> GetKnowledgeBaseDependentAgentsResponseModelBuilder {
        <GetKnowledgeBaseDependentAgentsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetKnowledgeBaseDependentAgentsResponseModelBuilder {
    agents: Option<Vec<GetKnowledgeBaseDependentAgentsResponseModelAgentsItem>>,
    branches: Option<Vec<DependentBranchInfo>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetKnowledgeBaseDependentAgentsResponseModelBuilder {
    pub fn agents(mut self, value: Vec<GetKnowledgeBaseDependentAgentsResponseModelAgentsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`GetKnowledgeBaseDependentAgentsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agents`](GetKnowledgeBaseDependentAgentsResponseModelBuilder::agents)
    /// - [`has_more`](GetKnowledgeBaseDependentAgentsResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetKnowledgeBaseDependentAgentsResponseModel, BuildError> {
        Ok(GetKnowledgeBaseDependentAgentsResponseModel {
            agents: self.agents.ok_or_else(|| BuildError::missing_field("agents"))?,
            branches: self.branches,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
