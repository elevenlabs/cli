pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateConversationTagRequestModel {
    /// Display title of the tag.
    #[serde(default)]
    pub title: String,
    /// Optional free-text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateConversationTagRequestModel {
    pub fn builder() -> CreateConversationTagRequestModelBuilder {
        <CreateConversationTagRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConversationTagRequestModelBuilder {
    title: Option<String>,
    description: Option<String>,
}

impl CreateConversationTagRequestModelBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConversationTagRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](CreateConversationTagRequestModelBuilder::title)
    pub fn build(self) -> Result<CreateConversationTagRequestModel, BuildError> {
        Ok(CreateConversationTagRequestModel {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description,
        })
    }
}

