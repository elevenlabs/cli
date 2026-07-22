pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeBaseContentSearchResult {
    pub document: KnowledgeBaseContentSearchResultDocument,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_snippet: Option<Vec<SearchHighlightSegment>>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub score: f64,
}

impl KnowledgeBaseContentSearchResult {
    pub fn builder() -> KnowledgeBaseContentSearchResultBuilder {
        <KnowledgeBaseContentSearchResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseContentSearchResultBuilder {
    document: Option<KnowledgeBaseContentSearchResultDocument>,
    search_snippet: Option<Vec<SearchHighlightSegment>>,
    score: Option<f64>,
}

impl KnowledgeBaseContentSearchResultBuilder {
    pub fn document(mut self, value: KnowledgeBaseContentSearchResultDocument) -> Self {
        self.document = Some(value);
        self
    }

    pub fn search_snippet(mut self, value: Vec<SearchHighlightSegment>) -> Self {
        self.search_snippet = Some(value);
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseContentSearchResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document`](KnowledgeBaseContentSearchResultBuilder::document)
    /// - [`score`](KnowledgeBaseContentSearchResultBuilder::score)
    pub fn build(self) -> Result<KnowledgeBaseContentSearchResult, BuildError> {
        Ok(KnowledgeBaseContentSearchResult {
            document: self.document.ok_or_else(|| BuildError::missing_field("document"))?,
            search_snippet: self.search_snippet,
            score: self.score.ok_or_else(|| BuildError::missing_field("score"))?,
        })
    }
}
