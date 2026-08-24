pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationUserResponseModel {
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub last_contact_unix_secs: i64,
    #[serde(default)]
    pub first_contact_unix_secs: i64,
    #[serde(default)]
    pub conversation_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_contact_agent_id: Option<String>,
    #[serde(default)]
    pub last_contact_conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_contact_agent_name: Option<String>,
    #[serde(default)]
    pub sentiment: SentimentAggregate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_frustrated_conversations: Option<Vec<FrustratedConversationRef>>,
}

impl ConversationUserResponseModel {
    pub fn builder() -> ConversationUserResponseModelBuilder {
        <ConversationUserResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationUserResponseModelBuilder {
    user_id: Option<String>,
    last_contact_unix_secs: Option<i64>,
    first_contact_unix_secs: Option<i64>,
    conversation_count: Option<i64>,
    last_contact_agent_id: Option<String>,
    last_contact_conversation_id: Option<String>,
    last_contact_agent_name: Option<String>,
    sentiment: Option<SentimentAggregate>,
    most_frustrated_conversations: Option<Vec<FrustratedConversationRef>>,
}

impl ConversationUserResponseModelBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn last_contact_unix_secs(mut self, value: i64) -> Self {
        self.last_contact_unix_secs = Some(value);
        self
    }

    pub fn first_contact_unix_secs(mut self, value: i64) -> Self {
        self.first_contact_unix_secs = Some(value);
        self
    }

    pub fn conversation_count(mut self, value: i64) -> Self {
        self.conversation_count = Some(value);
        self
    }

    pub fn last_contact_agent_id(mut self, value: impl Into<String>) -> Self {
        self.last_contact_agent_id = Some(value.into());
        self
    }

    pub fn last_contact_conversation_id(mut self, value: impl Into<String>) -> Self {
        self.last_contact_conversation_id = Some(value.into());
        self
    }

    pub fn last_contact_agent_name(mut self, value: impl Into<String>) -> Self {
        self.last_contact_agent_name = Some(value.into());
        self
    }

    pub fn sentiment(mut self, value: SentimentAggregate) -> Self {
        self.sentiment = Some(value);
        self
    }

    pub fn most_frustrated_conversations(mut self, value: Vec<FrustratedConversationRef>) -> Self {
        self.most_frustrated_conversations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationUserResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](ConversationUserResponseModelBuilder::user_id)
    /// - [`last_contact_unix_secs`](ConversationUserResponseModelBuilder::last_contact_unix_secs)
    /// - [`first_contact_unix_secs`](ConversationUserResponseModelBuilder::first_contact_unix_secs)
    /// - [`conversation_count`](ConversationUserResponseModelBuilder::conversation_count)
    /// - [`last_contact_conversation_id`](ConversationUserResponseModelBuilder::last_contact_conversation_id)
    /// - [`sentiment`](ConversationUserResponseModelBuilder::sentiment)
    pub fn build(self) -> Result<ConversationUserResponseModel, BuildError> {
        Ok(ConversationUserResponseModel {
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            last_contact_unix_secs: self.last_contact_unix_secs.ok_or_else(|| BuildError::missing_field("last_contact_unix_secs"))?,
            first_contact_unix_secs: self.first_contact_unix_secs.ok_or_else(|| BuildError::missing_field("first_contact_unix_secs"))?,
            conversation_count: self.conversation_count.ok_or_else(|| BuildError::missing_field("conversation_count"))?,
            last_contact_agent_id: self.last_contact_agent_id,
            last_contact_conversation_id: self.last_contact_conversation_id.ok_or_else(|| BuildError::missing_field("last_contact_conversation_id"))?,
            last_contact_agent_name: self.last_contact_agent_name,
            sentiment: self.sentiment.ok_or_else(|| BuildError::missing_field("sentiment"))?,
            most_frustrated_conversations: self.most_frustrated_conversations,
        })
    }
}
