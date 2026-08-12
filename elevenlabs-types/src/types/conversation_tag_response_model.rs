pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationTagResponseModel {
    #[serde(default)]
    pub tag_id: String,
    #[serde(default)]
    pub workspace_id: String,
    #[serde(default)]
    pub owner_user_id: String,
    #[serde(default)]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at_unix_secs: i64,
}

impl ConversationTagResponseModel {
    pub fn builder() -> ConversationTagResponseModelBuilder {
        <ConversationTagResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationTagResponseModelBuilder {
    tag_id: Option<String>,
    workspace_id: Option<String>,
    owner_user_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    created_at_unix_secs: Option<i64>,
}

impl ConversationTagResponseModelBuilder {
    pub fn tag_id(mut self, value: impl Into<String>) -> Self {
        self.tag_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn owner_user_id(mut self, value: impl Into<String>) -> Self {
        self.owner_user_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationTagResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tag_id`](ConversationTagResponseModelBuilder::tag_id)
    /// - [`workspace_id`](ConversationTagResponseModelBuilder::workspace_id)
    /// - [`owner_user_id`](ConversationTagResponseModelBuilder::owner_user_id)
    /// - [`title`](ConversationTagResponseModelBuilder::title)
    /// - [`created_at_unix_secs`](ConversationTagResponseModelBuilder::created_at_unix_secs)
    pub fn build(self) -> Result<ConversationTagResponseModel, BuildError> {
        Ok(ConversationTagResponseModel {
            tag_id: self.tag_id.ok_or_else(|| BuildError::missing_field("tag_id"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            owner_user_id: self.owner_user_id.ok_or_else(|| BuildError::missing_field("owner_user_id"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
        })
    }
}
