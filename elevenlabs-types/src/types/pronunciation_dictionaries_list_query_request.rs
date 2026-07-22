pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PronunciationDictionariesListQueryRequest {
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many pronunciation dictionaries to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Which field to sort by, one of 'created_at_unix' or 'name'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<PronunciationDictionariesListRequestSort>,
    /// Which direction to sort the voices in. 'ascending' or 'descending'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    /// Whether to include archived pronunciation dictionaries in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
}

impl PronunciationDictionariesListQueryRequest {
    pub fn builder() -> PronunciationDictionariesListQueryRequestBuilder {
        <PronunciationDictionariesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationDictionariesListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    sort: Option<PronunciationDictionariesListRequestSort>,
    sort_direction: Option<String>,
    include_archived: Option<bool>,
}

impl PronunciationDictionariesListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: PronunciationDictionariesListRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: impl Into<String>) -> Self {
        self.sort_direction = Some(value.into());
        self
    }

    pub fn include_archived(mut self, value: bool) -> Self {
        self.include_archived = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PronunciationDictionariesListQueryRequest`].
    pub fn build(self) -> Result<PronunciationDictionariesListQueryRequest, BuildError> {
        Ok(PronunciationDictionariesListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            sort: self.sort,
            sort_direction: self.sort_direction,
            include_archived: self.include_archived,
        })
    }
}

