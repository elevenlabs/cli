pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioIsolationListQueryRequest {
    /// How many history items to return at maximum. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Page number for search pagination (1-based). Only used when search is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Optional search term used for filtering audio isolation history (title/text).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl AudioIsolationListQueryRequest {
    pub fn builder() -> AudioIsolationListQueryRequestBuilder {
        <AudioIsolationListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioIsolationListQueryRequestBuilder {
    page_size: Option<i64>,
    page: Option<i64>,
    search: Option<String>,
}

impl AudioIsolationListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioIsolationListQueryRequest`].
    pub fn build(self) -> Result<AudioIsolationListQueryRequest, BuildError> {
        Ok(AudioIsolationListQueryRequest {
            page_size: self.page_size,
            page: self.page,
            search: self.search,
        })
    }
}

