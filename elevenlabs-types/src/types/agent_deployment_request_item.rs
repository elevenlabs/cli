pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentDeploymentRequestItem {
    /// ID of the branch to deploy
    #[serde(default)]
    pub branch_id: String,
    #[serde(default)]
    pub deployment_strategy: AgentDeploymentPercentageStrategy,
}

impl AgentDeploymentRequestItem {
    pub fn builder() -> AgentDeploymentRequestItemBuilder {
        <AgentDeploymentRequestItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentDeploymentRequestItemBuilder {
    branch_id: Option<String>,
    deployment_strategy: Option<AgentDeploymentPercentageStrategy>,
}

impl AgentDeploymentRequestItemBuilder {
    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn deployment_strategy(mut self, value: AgentDeploymentPercentageStrategy) -> Self {
        self.deployment_strategy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentDeploymentRequestItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branch_id`](AgentDeploymentRequestItemBuilder::branch_id)
    /// - [`deployment_strategy`](AgentDeploymentRequestItemBuilder::deployment_strategy)
    pub fn build(self) -> Result<AgentDeploymentRequestItem, BuildError> {
        Ok(AgentDeploymentRequestItem {
            branch_id: self.branch_id.ok_or_else(|| BuildError::missing_field("branch_id"))?,
            deployment_strategy: self.deployment_strategy.ok_or_else(|| BuildError::missing_field("deployment_strategy"))?,
        })
    }
}
