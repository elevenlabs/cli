pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentSummaryResponseModel {
    /// The ID of the agent
    #[serde(default)]
    pub agent_id: String,
    /// The name of the agent
    #[serde(default)]
    pub name: String,
    /// Agent tags used to categorize the agent
    #[serde(default)]
    pub tags: Vec<String>,
    /// The creation time of the agent in unix seconds
    #[serde(default)]
    pub created_at_unix_secs: i64,
    /// The access information of the agent
    pub access_info: ResourceAccessInfo,
    /// The time of the most recent call in unix seconds, null if no calls have been made
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_call_time_unix_secs: Option<i64>,
    /// Whether the agent is archived
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

impl AgentSummaryResponseModel {
    pub fn builder() -> AgentSummaryResponseModelBuilder {
        <AgentSummaryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSummaryResponseModelBuilder {
    agent_id: Option<String>,
    name: Option<String>,
    tags: Option<Vec<String>>,
    created_at_unix_secs: Option<i64>,
    access_info: Option<ResourceAccessInfo>,
    last_call_time_unix_secs: Option<i64>,
    archived: Option<bool>,
}

impl AgentSummaryResponseModelBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    pub fn last_call_time_unix_secs(mut self, value: i64) -> Self {
        self.last_call_time_unix_secs = Some(value);
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentSummaryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](AgentSummaryResponseModelBuilder::agent_id)
    /// - [`name`](AgentSummaryResponseModelBuilder::name)
    /// - [`tags`](AgentSummaryResponseModelBuilder::tags)
    /// - [`created_at_unix_secs`](AgentSummaryResponseModelBuilder::created_at_unix_secs)
    /// - [`access_info`](AgentSummaryResponseModelBuilder::access_info)
    pub fn build(self) -> Result<AgentSummaryResponseModel, BuildError> {
        Ok(AgentSummaryResponseModel {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            access_info: self.access_info.ok_or_else(|| BuildError::missing_field("access_info"))?,
            last_call_time_unix_secs: self.last_call_time_unix_secs,
            archived: self.archived,
        })
    }
}
