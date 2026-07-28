pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentDeploymentRequest {
    /// List of deployment requests
    #[serde(default)]
    pub requests: Vec<AgentDeploymentRequestItem>,
}

impl AgentDeploymentRequest {
    pub fn builder() -> AgentDeploymentRequestBuilder {
        <AgentDeploymentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentDeploymentRequestBuilder {
    requests: Option<Vec<AgentDeploymentRequestItem>>,
}

impl AgentDeploymentRequestBuilder {
    pub fn requests(mut self, value: Vec<AgentDeploymentRequestItem>) -> Self {
        self.requests = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentDeploymentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requests`](AgentDeploymentRequestBuilder::requests)
    pub fn build(self) -> Result<AgentDeploymentRequest, BuildError> {
        Ok(AgentDeploymentRequest {
            requests: self.requests.ok_or_else(|| BuildError::missing_field("requests"))?,
        })
    }
}
