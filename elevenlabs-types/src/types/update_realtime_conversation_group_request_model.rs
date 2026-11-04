pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateRealtimeConversationGroupRequestModel {
    /// Name of the conversation group. Names are unique per workspace, ignoring case and extra whitespace, and at most 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// New description of the conversation group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateRealtimeConversationGroupRequestModel {
    pub fn builder() -> UpdateRealtimeConversationGroupRequestModelBuilder {
        <UpdateRealtimeConversationGroupRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRealtimeConversationGroupRequestModelBuilder {
    title: Option<String>,
    description: Option<String>,
}

impl UpdateRealtimeConversationGroupRequestModelBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateRealtimeConversationGroupRequestModel`].
    pub fn build(self) -> Result<UpdateRealtimeConversationGroupRequestModel, BuildError> {
        Ok(UpdateRealtimeConversationGroupRequestModel {
            title: self.title,
            description: self.description,
        })
    }
}

