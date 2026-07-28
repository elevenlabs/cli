pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum KnowledgeBaseContentSearchResultDocument {
        #[serde(rename = "file")]
        #[non_exhaustive]
        File {
            #[serde(flatten)]
            data: GetKnowledgeBaseSummaryFileResponseModel,
        },

        #[serde(rename = "folder")]
        #[non_exhaustive]
        Folder {
            #[serde(flatten)]
            data: GetKnowledgeBaseSummaryFolderResponseModel,
        },

        #[serde(rename = "text")]
        #[non_exhaustive]
        Text {
            #[serde(flatten)]
            data: GetKnowledgeBaseSummaryTextResponseModel,
        },

        #[serde(rename = "url")]
        #[non_exhaustive]
        Url {
            #[serde(flatten)]
            data: GetKnowledgeBaseSummaryUrlResponseModel,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl KnowledgeBaseContentSearchResultDocument {
    pub fn file(data: GetKnowledgeBaseSummaryFileResponseModel) -> Self {
        Self::File { data }
    }

    pub fn folder(data: GetKnowledgeBaseSummaryFolderResponseModel) -> Self {
        Self::Folder { data }
    }

    pub fn text(data: GetKnowledgeBaseSummaryTextResponseModel) -> Self {
        Self::Text { data }
    }

    pub fn url(data: GetKnowledgeBaseSummaryUrlResponseModel) -> Self {
        Self::Url { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
