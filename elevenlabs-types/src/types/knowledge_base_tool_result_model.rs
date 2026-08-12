pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseToolResultModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<KnowledgeBaseToolStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl KnowledgeBaseToolResultModel {
    pub fn builder() -> KnowledgeBaseToolResultModelBuilder {
        <KnowledgeBaseToolResultModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseToolResultModelBuilder {
    status: Option<KnowledgeBaseToolStatus>,
    chunk_count: Option<i64>,
    message: Option<String>,
}

impl KnowledgeBaseToolResultModelBuilder {
    pub fn status(mut self, value: KnowledgeBaseToolStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn chunk_count(mut self, value: i64) -> Self {
        self.chunk_count = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseToolResultModel`].
    pub fn build(self) -> Result<KnowledgeBaseToolResultModel, BuildError> {
        Ok(KnowledgeBaseToolResultModel {
            status: self.status,
            chunk_count: self.chunk_count,
            message: self.message,
        })
    }
}
