pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddKnowledgeBaseResponseModel {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    /// The folder path segments leading to this entity, from root to parent folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<Vec<KnowledgeBaseFolderPathSegmentSummaryResponseModel>>,
}

impl AddKnowledgeBaseResponseModel {
    pub fn builder() -> AddKnowledgeBaseResponseModelBuilder {
        <AddKnowledgeBaseResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddKnowledgeBaseResponseModelBuilder {
    id: Option<String>,
    name: Option<String>,
    folder_path: Option<Vec<KnowledgeBaseFolderPathSegmentSummaryResponseModel>>,
}

impl AddKnowledgeBaseResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn folder_path(mut self, value: Vec<KnowledgeBaseFolderPathSegmentSummaryResponseModel>) -> Self {
        self.folder_path = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddKnowledgeBaseResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AddKnowledgeBaseResponseModelBuilder::id)
    /// - [`name`](AddKnowledgeBaseResponseModelBuilder::name)
    pub fn build(self) -> Result<AddKnowledgeBaseResponseModel, BuildError> {
        Ok(AddKnowledgeBaseResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            folder_path: self.folder_path,
        })
    }
}
