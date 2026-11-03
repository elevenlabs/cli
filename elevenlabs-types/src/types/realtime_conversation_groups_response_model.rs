pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RealtimeConversationGroupsResponseModel {
    #[serde(default)]
    pub conversation_groups: Vec<RealtimeConversationGroupResponseModel>,
    /// Cursor to pass to the next request to fetch the following page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether more conversation groups are available beyond this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl RealtimeConversationGroupsResponseModel {
    pub fn builder() -> RealtimeConversationGroupsResponseModelBuilder {
        <RealtimeConversationGroupsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RealtimeConversationGroupsResponseModelBuilder {
    conversation_groups: Option<Vec<RealtimeConversationGroupResponseModel>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl RealtimeConversationGroupsResponseModelBuilder {
    pub fn conversation_groups(mut self, value: Vec<RealtimeConversationGroupResponseModel>) -> Self {
        self.conversation_groups = Some(value);
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

    /// Consumes the builder and constructs a [`RealtimeConversationGroupsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_groups`](RealtimeConversationGroupsResponseModelBuilder::conversation_groups)
    pub fn build(self) -> Result<RealtimeConversationGroupsResponseModel, BuildError> {
        Ok(RealtimeConversationGroupsResponseModel {
            conversation_groups: self.conversation_groups.ok_or_else(|| BuildError::missing_field("conversation_groups"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
