pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetConversationUsersPageResponseModel {
    #[serde(default)]
    pub users: Vec<ConversationUserResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl GetConversationUsersPageResponseModel {
    pub fn builder() -> GetConversationUsersPageResponseModelBuilder {
        <GetConversationUsersPageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetConversationUsersPageResponseModelBuilder {
    users: Option<Vec<ConversationUserResponseModel>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetConversationUsersPageResponseModelBuilder {
    pub fn users(mut self, value: Vec<ConversationUserResponseModel>) -> Self {
        self.users = Some(value);
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

    /// Consumes the builder and constructs a [`GetConversationUsersPageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`users`](GetConversationUsersPageResponseModelBuilder::users)
    /// - [`has_more`](GetConversationUsersPageResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetConversationUsersPageResponseModel, BuildError> {
        Ok(GetConversationUsersPageResponseModel {
            users: self.users.ok_or_else(|| BuildError::missing_field("users"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
