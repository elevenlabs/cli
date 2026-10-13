pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentDeploymentHistoryItem {
    /// ID of the deployment
    #[serde(default)]
    pub id: String,
    /// Map of branch IDs to traffic percentages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_percentage_branch_id_map: Option<HashMap<String, f64>>,
    /// Unix timestamp of when the traffic split was applied
    #[serde(default)]
    pub deployed_at_unix_secs: i64,
    /// Whether this is the live traffic split
    #[serde(default)]
    pub is_active: bool,
    /// What caused this traffic split change. Null for deployments created before this was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AgentDeploymentSource>,
    /// Access information for the deployment, including who created it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_info: Option<ResourceAccessInfo>,
}

impl AgentDeploymentHistoryItem {
    pub fn builder() -> AgentDeploymentHistoryItemBuilder {
        <AgentDeploymentHistoryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentDeploymentHistoryItemBuilder {
    id: Option<String>,
    traffic_percentage_branch_id_map: Option<HashMap<String, f64>>,
    deployed_at_unix_secs: Option<i64>,
    is_active: Option<bool>,
    source: Option<AgentDeploymentSource>,
    access_info: Option<ResourceAccessInfo>,
}

impl AgentDeploymentHistoryItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn traffic_percentage_branch_id_map(mut self, value: HashMap<String, f64>) -> Self {
        self.traffic_percentage_branch_id_map = Some(value);
        self
    }

    pub fn deployed_at_unix_secs(mut self, value: i64) -> Self {
        self.deployed_at_unix_secs = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn source(mut self, value: AgentDeploymentSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentDeploymentHistoryItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentDeploymentHistoryItemBuilder::id)
    /// - [`deployed_at_unix_secs`](AgentDeploymentHistoryItemBuilder::deployed_at_unix_secs)
    /// - [`is_active`](AgentDeploymentHistoryItemBuilder::is_active)
    pub fn build(self) -> Result<AgentDeploymentHistoryItem, BuildError> {
        Ok(AgentDeploymentHistoryItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            traffic_percentage_branch_id_map: self.traffic_percentage_branch_id_map,
            deployed_at_unix_secs: self.deployed_at_unix_secs.ok_or_else(|| BuildError::missing_field("deployed_at_unix_secs"))?,
            is_active: self.is_active.ok_or_else(|| BuildError::missing_field("is_active"))?,
            source: self.source,
            access_info: self.access_info,
        })
    }
}
