pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentDeploymentResponse {
    /// Map of branch IDs to traffic percentages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_percentage_branch_id_map: Option<HashMap<String, f64>>,
}

impl AgentDeploymentResponse {
    pub fn builder() -> AgentDeploymentResponseBuilder {
        <AgentDeploymentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentDeploymentResponseBuilder {
    traffic_percentage_branch_id_map: Option<HashMap<String, f64>>,
}

impl AgentDeploymentResponseBuilder {
    pub fn traffic_percentage_branch_id_map(mut self, value: HashMap<String, f64>) -> Self {
        self.traffic_percentage_branch_id_map = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentDeploymentResponse`].
    pub fn build(self) -> Result<AgentDeploymentResponse, BuildError> {
        Ok(AgentDeploymentResponse {
            traffic_percentage_branch_id_map: self.traffic_percentage_branch_id_map,
        })
    }
}
