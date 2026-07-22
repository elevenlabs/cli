pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentBranchResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub last_committed_at: i64,
    #[serde(default)]
    pub is_archived: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<BranchProtectionStatus>,
    /// Access information for the branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_info: Option<ResourceAccessInfo>,
    /// Percentage of traffic live on the branch
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub current_live_percentage: Option<f64>,
    /// Parent branch of the branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_branch: Option<AgentBranchBasicInfo>,
    /// Most recent versions on the branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_versions: Option<Vec<AgentVersionMetadata>>,
}

impl AgentBranchResponse {
    pub fn builder() -> AgentBranchResponseBuilder {
        <AgentBranchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentBranchResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    agent_id: Option<String>,
    description: Option<String>,
    created_at: Option<i64>,
    last_committed_at: Option<i64>,
    is_archived: Option<bool>,
    protection_status: Option<BranchProtectionStatus>,
    access_info: Option<ResourceAccessInfo>,
    current_live_percentage: Option<f64>,
    parent_branch: Option<AgentBranchBasicInfo>,
    most_recent_versions: Option<Vec<AgentVersionMetadata>>,
}

impl AgentBranchResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn last_committed_at(mut self, value: i64) -> Self {
        self.last_committed_at = Some(value);
        self
    }

    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = Some(value);
        self
    }

    pub fn protection_status(mut self, value: BranchProtectionStatus) -> Self {
        self.protection_status = Some(value);
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    pub fn current_live_percentage(mut self, value: f64) -> Self {
        self.current_live_percentage = Some(value);
        self
    }

    pub fn parent_branch(mut self, value: AgentBranchBasicInfo) -> Self {
        self.parent_branch = Some(value);
        self
    }

    pub fn most_recent_versions(mut self, value: Vec<AgentVersionMetadata>) -> Self {
        self.most_recent_versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentBranchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentBranchResponseBuilder::id)
    /// - [`name`](AgentBranchResponseBuilder::name)
    /// - [`agent_id`](AgentBranchResponseBuilder::agent_id)
    /// - [`description`](AgentBranchResponseBuilder::description)
    /// - [`created_at`](AgentBranchResponseBuilder::created_at)
    /// - [`last_committed_at`](AgentBranchResponseBuilder::last_committed_at)
    /// - [`is_archived`](AgentBranchResponseBuilder::is_archived)
    pub fn build(self) -> Result<AgentBranchResponse, BuildError> {
        Ok(AgentBranchResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            last_committed_at: self.last_committed_at.ok_or_else(|| BuildError::missing_field("last_committed_at"))?,
            is_archived: self.is_archived.ok_or_else(|| BuildError::missing_field("is_archived"))?,
            protection_status: self.protection_status,
            access_info: self.access_info,
            current_live_percentage: self.current_live_percentage,
            parent_branch: self.parent_branch,
            most_recent_versions: self.most_recent_versions,
        })
    }
}
