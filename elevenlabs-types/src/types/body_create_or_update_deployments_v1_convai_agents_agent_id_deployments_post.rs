pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIdDeploymentsPost {
    /// Request to create a new deployment
    #[serde(default)]
    pub deployment_request: AgentDeploymentRequest,
}

impl BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIdDeploymentsPost {
    pub fn builder() -> BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIdDeploymentsPostBuilder {
        <BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIdDeploymentsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIdDeploymentsPostBuilder {
    deployment_request: Option<AgentDeploymentRequest>,
}

impl BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIdDeploymentsPostBuilder {
    pub fn deployment_request(mut self, value: AgentDeploymentRequest) -> Self {
        self.deployment_request = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIdDeploymentsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deployment_request`](BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIdDeploymentsPostBuilder::deployment_request)
    pub fn build(self) -> Result<BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIdDeploymentsPost, BuildError> {
        Ok(BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIdDeploymentsPost {
            deployment_request: self.deployment_request.ok_or_else(|| BuildError::missing_field("deployment_request"))?,
        })
    }
}

