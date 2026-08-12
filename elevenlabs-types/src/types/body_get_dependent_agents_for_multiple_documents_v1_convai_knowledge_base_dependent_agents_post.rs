pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost {
    /// The ids of documents or folders from the knowledge base.
    #[serde(default)]
    pub document_ids: Vec<String>,
    /// Type of dependent agents to return.
    #[serde(skip)]
    pub dependent_type: Option<KnowledgeBaseDependentType>,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip)]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip)]
    pub cursor: Option<String>,
}

impl BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost {
    pub fn builder() -> BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPostBuilder {
        <BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPostBuilder {
    document_ids: Option<Vec<String>>,
    dependent_type: Option<KnowledgeBaseDependentType>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPostBuilder {
    pub fn document_ids(mut self, value: Vec<String>) -> Self {
        self.document_ids = Some(value);
        self
    }

    pub fn dependent_type(mut self, value: KnowledgeBaseDependentType) -> Self {
        self.dependent_type = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_ids`](BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPostBuilder::document_ids)
    pub fn build(self) -> Result<BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost, BuildError> {
        Ok(BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost {
            document_ids: self.document_ids.ok_or_else(|| BuildError::missing_field("document_ids"))?,
            dependent_type: self.dependent_type,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

