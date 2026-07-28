pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetKnowledgeBaseListResponseModel {
    #[serde(default)]
    pub documents: Vec<GetKnowledgeBaseListResponseModelDocumentsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl GetKnowledgeBaseListResponseModel {
    pub fn builder() -> GetKnowledgeBaseListResponseModelBuilder {
        <GetKnowledgeBaseListResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetKnowledgeBaseListResponseModelBuilder {
    documents: Option<Vec<GetKnowledgeBaseListResponseModelDocumentsItem>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetKnowledgeBaseListResponseModelBuilder {
    pub fn documents(mut self, value: Vec<GetKnowledgeBaseListResponseModelDocumentsItem>) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetKnowledgeBaseListResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](GetKnowledgeBaseListResponseModelBuilder::documents)
    /// - [`has_more`](GetKnowledgeBaseListResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetKnowledgeBaseListResponseModel, BuildError> {
        Ok(GetKnowledgeBaseListResponseModel {
            documents: self.documents.ok_or_else(|| BuildError::missing_field("documents"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
