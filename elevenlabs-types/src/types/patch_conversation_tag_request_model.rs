pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchConversationTagRequestModel {
    /// If provided, replaces the tag title. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// If provided, replaces the tag description. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PatchConversationTagRequestModel {
    pub fn builder() -> PatchConversationTagRequestModelBuilder {
        <PatchConversationTagRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchConversationTagRequestModelBuilder {
    title: Option<String>,
    description: Option<String>,
}

impl PatchConversationTagRequestModelBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PatchConversationTagRequestModel`].
    pub fn build(self) -> Result<PatchConversationTagRequestModel, BuildError> {
        Ok(PatchConversationTagRequestModel {
            title: self.title,
            description: self.description,
        })
    }
}

