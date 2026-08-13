pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetConversationTagsPageResponseModel {
    #[serde(default)]
    pub conversation_tags: Vec<ConversationTagResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl GetConversationTagsPageResponseModel {
    pub fn builder() -> GetConversationTagsPageResponseModelBuilder {
        <GetConversationTagsPageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetConversationTagsPageResponseModelBuilder {
    conversation_tags: Option<Vec<ConversationTagResponseModel>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetConversationTagsPageResponseModelBuilder {
    pub fn conversation_tags(mut self, value: Vec<ConversationTagResponseModel>) -> Self {
        self.conversation_tags = Some(value);
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

    /// Consumes the builder and constructs a [`GetConversationTagsPageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_tags`](GetConversationTagsPageResponseModelBuilder::conversation_tags)
    /// - [`has_more`](GetConversationTagsPageResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetConversationTagsPageResponseModel, BuildError> {
        Ok(GetConversationTagsPageResponseModel {
            conversation_tags: self.conversation_tags.ok_or_else(|| BuildError::missing_field("conversation_tags"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
