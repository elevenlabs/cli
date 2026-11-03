pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRealtimeConversationGroupRequestModel {
    /// Name of the conversation group. Names are unique per workspace, ignoring case and extra whitespace, and at most 64 characters.
    #[serde(default)]
    pub title: String,
    /// Optional description of the conversation group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateRealtimeConversationGroupRequestModel {
    pub fn builder() -> CreateRealtimeConversationGroupRequestModelBuilder {
        <CreateRealtimeConversationGroupRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRealtimeConversationGroupRequestModelBuilder {
    title: Option<String>,
    description: Option<String>,
}

impl CreateRealtimeConversationGroupRequestModelBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateRealtimeConversationGroupRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](CreateRealtimeConversationGroupRequestModelBuilder::title)
    pub fn build(self) -> Result<CreateRealtimeConversationGroupRequestModel, BuildError> {
        Ok(CreateRealtimeConversationGroupRequestModel {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description,
        })
    }
}

