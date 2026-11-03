pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetConversationGroupRequestModel {
    /// The conversation group to place the conversation in. A conversation belongs to at most one group, so this moves it out of any current group.
    #[serde(default)]
    pub conversation_group_id: String,
}

impl SetConversationGroupRequestModel {
    pub fn builder() -> SetConversationGroupRequestModelBuilder {
        <SetConversationGroupRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetConversationGroupRequestModelBuilder {
    conversation_group_id: Option<String>,
}

impl SetConversationGroupRequestModelBuilder {
    pub fn conversation_group_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetConversationGroupRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_group_id`](SetConversationGroupRequestModelBuilder::conversation_group_id)
    pub fn build(self) -> Result<SetConversationGroupRequestModel, BuildError> {
        Ok(SetConversationGroupRequestModel {
            conversation_group_id: self.conversation_group_id.ok_or_else(|| BuildError::missing_field("conversation_group_id"))?,
        })
    }
}

