pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationTokenResponseModel {
    /// The ID of the agent
    #[serde(default)]
    pub agent_id: String,
    /// The token for the agent
    #[serde(default)]
    pub conversation_token: String,
    /// The expiration time of the token in unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time_unix_secs: Option<i64>,
    /// The ID of the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// The purpose of the token
    pub purpose: ConversationTokenPurpose,
    /// The user ID of the entity who requested the token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_requester_user_id: Option<String>,
}

impl ConversationTokenResponseModel {
    pub fn builder() -> ConversationTokenResponseModelBuilder {
        <ConversationTokenResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationTokenResponseModelBuilder {
    agent_id: Option<String>,
    conversation_token: Option<String>,
    expiration_time_unix_secs: Option<i64>,
    conversation_id: Option<String>,
    purpose: Option<ConversationTokenPurpose>,
    token_requester_user_id: Option<String>,
}

impl ConversationTokenResponseModelBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn conversation_token(mut self, value: impl Into<String>) -> Self {
        self.conversation_token = Some(value.into());
        self
    }

    pub fn expiration_time_unix_secs(mut self, value: i64) -> Self {
        self.expiration_time_unix_secs = Some(value);
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: ConversationTokenPurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    pub fn token_requester_user_id(mut self, value: impl Into<String>) -> Self {
        self.token_requester_user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationTokenResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](ConversationTokenResponseModelBuilder::agent_id)
    /// - [`conversation_token`](ConversationTokenResponseModelBuilder::conversation_token)
    /// - [`purpose`](ConversationTokenResponseModelBuilder::purpose)
    pub fn build(self) -> Result<ConversationTokenResponseModel, BuildError> {
        Ok(ConversationTokenResponseModel {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            conversation_token: self.conversation_token.ok_or_else(|| BuildError::missing_field("conversation_token"))?,
            expiration_time_unix_secs: self.expiration_time_unix_secs,
            conversation_id: self.conversation_id,
            purpose: self.purpose.ok_or_else(|| BuildError::missing_field("purpose"))?,
            token_requester_user_id: self.token_requester_user_id,
        })
    }
}
