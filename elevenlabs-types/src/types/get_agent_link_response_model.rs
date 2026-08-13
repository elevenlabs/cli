pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAgentLinkResponseModel {
    /// The ID of the agent
    #[serde(default)]
    pub agent_id: String,
    /// The token data for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<ConversationTokenResponseModel>,
}

impl GetAgentLinkResponseModel {
    pub fn builder() -> GetAgentLinkResponseModelBuilder {
        <GetAgentLinkResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgentLinkResponseModelBuilder {
    agent_id: Option<String>,
    token: Option<ConversationTokenResponseModel>,
}

impl GetAgentLinkResponseModelBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn token(mut self, value: ConversationTokenResponseModel) -> Self {
        self.token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAgentLinkResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](GetAgentLinkResponseModelBuilder::agent_id)
    pub fn build(self) -> Result<GetAgentLinkResponseModel, BuildError> {
        Ok(GetAgentLinkResponseModel {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            token: self.token,
        })
    }
}
