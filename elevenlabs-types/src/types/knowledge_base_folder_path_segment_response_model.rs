pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseFolderPathSegmentResponseModel {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl KnowledgeBaseFolderPathSegmentResponseModel {
    pub fn builder() -> KnowledgeBaseFolderPathSegmentResponseModelBuilder {
        <KnowledgeBaseFolderPathSegmentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseFolderPathSegmentResponseModelBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl KnowledgeBaseFolderPathSegmentResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseFolderPathSegmentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](KnowledgeBaseFolderPathSegmentResponseModelBuilder::id)
    pub fn build(self) -> Result<KnowledgeBaseFolderPathSegmentResponseModel, BuildError> {
        Ok(KnowledgeBaseFolderPathSegmentResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
        })
    }
}
