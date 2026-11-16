pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAgentConversationRequest {
    /// The agent that runs the turns.
    pub agent: AgentMessagesPersistedAgent,
    /// The environment to use for resolving environment variables (e.g. 'production', 'staging'). Defaults to the branch's environment, then 'production'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// ID of the end user participating in this conversation (for agent owner's user identification)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variables: Option<HashMap<String, serde_json::Value>>,
}

impl CreateAgentConversationRequest {
    pub fn builder() -> CreateAgentConversationRequestBuilder {
        <CreateAgentConversationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentConversationRequestBuilder {
    agent: Option<AgentMessagesPersistedAgent>,
    environment: Option<String>,
    user_id: Option<String>,
    dynamic_variables: Option<HashMap<String, serde_json::Value>>,
}

impl CreateAgentConversationRequestBuilder {
    pub fn agent(mut self, value: AgentMessagesPersistedAgent) -> Self {
        self.agent = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn dynamic_variables(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.dynamic_variables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentConversationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent`](CreateAgentConversationRequestBuilder::agent)
    pub fn build(self) -> Result<CreateAgentConversationRequest, BuildError> {
        Ok(CreateAgentConversationRequest {
            agent: self.agent.ok_or_else(|| BuildError::missing_field("agent"))?,
            environment: self.environment,
            user_id: self.user_id,
            dynamic_variables: self.dynamic_variables,
        })
    }
}

