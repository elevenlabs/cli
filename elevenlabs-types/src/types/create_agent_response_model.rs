pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentResponseModel {
    /// ID of the created agent
    #[serde(default)]
    pub agent_id: String,
}

impl CreateAgentResponseModel {
    pub fn builder() -> CreateAgentResponseModelBuilder {
        <CreateAgentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentResponseModelBuilder {
    agent_id: Option<String>,
}

impl CreateAgentResponseModelBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](CreateAgentResponseModelBuilder::agent_id)
    pub fn build(self) -> Result<CreateAgentResponseModel, BuildError> {
        Ok(CreateAgentResponseModel {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
        })
    }
}
