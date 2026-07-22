pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct KnowledgeBaseContentSearchResponseModel {
    #[serde(default)]
    pub results: Vec<KnowledgeBaseContentSearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl KnowledgeBaseContentSearchResponseModel {
    pub fn builder() -> KnowledgeBaseContentSearchResponseModelBuilder {
        <KnowledgeBaseContentSearchResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseContentSearchResponseModelBuilder {
    results: Option<Vec<KnowledgeBaseContentSearchResult>>,
    next_cursor: Option<String>,
}

impl KnowledgeBaseContentSearchResponseModelBuilder {
    pub fn results(mut self, value: Vec<KnowledgeBaseContentSearchResult>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseContentSearchResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](KnowledgeBaseContentSearchResponseModelBuilder::results)
    pub fn build(self) -> Result<KnowledgeBaseContentSearchResponseModel, BuildError> {
        Ok(KnowledgeBaseContentSearchResponseModel {
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
            next_cursor: self.next_cursor,
        })
    }
}
