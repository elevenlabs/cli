pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAgentsPageResponseModel {
    /// A list of agents and their metadata
    #[serde(default)]
    pub agents: Vec<AgentSummaryResponseModel>,
    /// The next cursor to paginate through the agents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether there are more agents to paginate through
    #[serde(default)]
    pub has_more: bool,
}

impl GetAgentsPageResponseModel {
    pub fn builder() -> GetAgentsPageResponseModelBuilder {
        <GetAgentsPageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgentsPageResponseModelBuilder {
    agents: Option<Vec<AgentSummaryResponseModel>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetAgentsPageResponseModelBuilder {
    pub fn agents(mut self, value: Vec<AgentSummaryResponseModel>) -> Self {
        self.agents = Some(value);
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

    /// Consumes the builder and constructs a [`GetAgentsPageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agents`](GetAgentsPageResponseModelBuilder::agents)
    /// - [`has_more`](GetAgentsPageResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetAgentsPageResponseModel, BuildError> {
        Ok(GetAgentsPageResponseModel {
            agents: self.agents.ok_or_else(|| BuildError::missing_field("agents"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
