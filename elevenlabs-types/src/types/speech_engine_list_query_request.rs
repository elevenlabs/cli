pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpeechEngineListQueryRequest {
    /// How many Speech Engines to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Search term to filter Speech Engines by name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// The direction to sort the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
    /// The field to sort the results by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<AgentSortBy>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl SpeechEngineListQueryRequest {
    pub fn builder() -> SpeechEngineListQueryRequestBuilder {
        <SpeechEngineListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechEngineListQueryRequestBuilder {
    page_size: Option<i64>,
    search: Option<String>,
    sort_direction: Option<SortDirection>,
    sort_by: Option<AgentSortBy>,
    cursor: Option<String>,
}

impl SpeechEngineListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn sort_direction(mut self, value: SortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn sort_by(mut self, value: AgentSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpeechEngineListQueryRequest`].
    pub fn build(self) -> Result<SpeechEngineListQueryRequest, BuildError> {
        Ok(SpeechEngineListQueryRequest {
            page_size: self.page_size,
            search: self.search,
            sort_direction: self.sort_direction,
            sort_by: self.sort_by,
            cursor: self.cursor,
        })
    }
}

