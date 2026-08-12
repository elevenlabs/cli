pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentBranchSummary {
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
    /// ID of the parent branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_branch_id: Option<String>,
    /// Whether a draft exists for the branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_exists: Option<bool>,
    /// Number of calls in the last 7 days
    #[serde(rename = "calls_7d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calls7d: Option<i64>,
    /// Number of commits on this branch not yet on main, relative to their common ancestor. Null if it could not be computed (e.g. no common ancestor, or the branch history exceeds the comparison budget).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_ahead: Option<i64>,
    /// Number of commits on main not yet incorporated into this branch, relative to their common ancestor. Null if it could not be computed (e.g. no common ancestor, or the branch history exceeds the comparison budget).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_behind: Option<i64>,
    /// ID of the branch this branch's tip version was merged into, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_into_branch_id: Option<String>,
}

impl AgentBranchSummary {
    pub fn builder() -> AgentBranchSummaryBuilder {
        <AgentBranchSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentBranchSummaryBuilder {
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
    parent_branch_id: Option<String>,
    draft_exists: Option<bool>,
    calls7d: Option<i64>,
    commits_ahead: Option<i64>,
    commits_behind: Option<i64>,
    merged_into_branch_id: Option<String>,
}

impl AgentBranchSummaryBuilder {
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

    pub fn parent_branch_id(mut self, value: impl Into<String>) -> Self {
        self.parent_branch_id = Some(value.into());
        self
    }

    pub fn draft_exists(mut self, value: bool) -> Self {
        self.draft_exists = Some(value);
        self
    }

    pub fn calls7d(mut self, value: i64) -> Self {
        self.calls7d = Some(value);
        self
    }

    pub fn commits_ahead(mut self, value: i64) -> Self {
        self.commits_ahead = Some(value);
        self
    }

    pub fn commits_behind(mut self, value: i64) -> Self {
        self.commits_behind = Some(value);
        self
    }

    pub fn merged_into_branch_id(mut self, value: impl Into<String>) -> Self {
        self.merged_into_branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentBranchSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentBranchSummaryBuilder::id)
    /// - [`name`](AgentBranchSummaryBuilder::name)
    /// - [`agent_id`](AgentBranchSummaryBuilder::agent_id)
    /// - [`description`](AgentBranchSummaryBuilder::description)
    /// - [`created_at`](AgentBranchSummaryBuilder::created_at)
    /// - [`last_committed_at`](AgentBranchSummaryBuilder::last_committed_at)
    /// - [`is_archived`](AgentBranchSummaryBuilder::is_archived)
    pub fn build(self) -> Result<AgentBranchSummary, BuildError> {
        Ok(AgentBranchSummary {
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
            parent_branch_id: self.parent_branch_id,
            draft_exists: self.draft_exists,
            calls7d: self.calls7d,
            commits_ahead: self.commits_ahead,
            commits_behind: self.commits_behind,
            merged_into_branch_id: self.merged_into_branch_id,
        })
    }
}
