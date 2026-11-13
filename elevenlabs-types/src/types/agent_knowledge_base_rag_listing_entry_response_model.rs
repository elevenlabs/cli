pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentKnowledgeBaseRagListingEntryResponseModel {
    /// ID of the document or folder.
    #[serde(default)]
    pub id: String,
    /// Name of the document or folder.
    #[serde(default)]
    pub name: String,
    /// Whether the entry is a folder.
    #[serde(default)]
    pub is_folder: bool,
}

impl AgentKnowledgeBaseRagListingEntryResponseModel {
    pub fn builder() -> AgentKnowledgeBaseRagListingEntryResponseModelBuilder {
        <AgentKnowledgeBaseRagListingEntryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentKnowledgeBaseRagListingEntryResponseModelBuilder {
    id: Option<String>,
    name: Option<String>,
    is_folder: Option<bool>,
}

impl AgentKnowledgeBaseRagListingEntryResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_folder(mut self, value: bool) -> Self {
        self.is_folder = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentKnowledgeBaseRagListingEntryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentKnowledgeBaseRagListingEntryResponseModelBuilder::id)
    /// - [`name`](AgentKnowledgeBaseRagListingEntryResponseModelBuilder::name)
    /// - [`is_folder`](AgentKnowledgeBaseRagListingEntryResponseModelBuilder::is_folder)
    pub fn build(self) -> Result<AgentKnowledgeBaseRagListingEntryResponseModel, BuildError> {
        Ok(AgentKnowledgeBaseRagListingEntryResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_folder: self.is_folder.ok_or_else(|| BuildError::missing_field("is_folder"))?,
        })
    }
}
