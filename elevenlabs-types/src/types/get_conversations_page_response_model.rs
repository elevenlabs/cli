pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetConversationsPageResponseModel {
    #[serde(default)]
    pub conversations: Vec<ConversationSummaryResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl GetConversationsPageResponseModel {
    pub fn builder() -> GetConversationsPageResponseModelBuilder {
        <GetConversationsPageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetConversationsPageResponseModelBuilder {
    conversations: Option<Vec<ConversationSummaryResponseModel>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetConversationsPageResponseModelBuilder {
    pub fn conversations(mut self, value: Vec<ConversationSummaryResponseModel>) -> Self {
        self.conversations = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetConversationsPageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversations`](GetConversationsPageResponseModelBuilder::conversations)
    /// - [`has_more`](GetConversationsPageResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetConversationsPageResponseModel, BuildError> {
        Ok(GetConversationsPageResponseModel {
            conversations: self.conversations.ok_or_else(|| BuildError::missing_field("conversations"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
