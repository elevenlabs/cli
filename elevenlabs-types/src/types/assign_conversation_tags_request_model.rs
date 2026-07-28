pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssignConversationTagsRequestModel {
    /// Tag IDs to add to the conversation. Re-assigning an existing tag is a no-op.
    #[serde(default)]
    pub tag_ids: Vec<String>,
}

impl AssignConversationTagsRequestModel {
    pub fn builder() -> AssignConversationTagsRequestModelBuilder {
        <AssignConversationTagsRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssignConversationTagsRequestModelBuilder {
    tag_ids: Option<Vec<String>>,
}

impl AssignConversationTagsRequestModelBuilder {
    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssignConversationTagsRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tag_ids`](AssignConversationTagsRequestModelBuilder::tag_ids)
    pub fn build(self) -> Result<AssignConversationTagsRequestModel, BuildError> {
        Ok(AssignConversationTagsRequestModel {
            tag_ids: self.tag_ids.ok_or_else(|| BuildError::missing_field("tag_ids"))?,
        })
    }
}

