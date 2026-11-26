pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HistoryListQueryRequest {
    /// How many history items to return at maximum. Can not exceed 1000, defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// After which ID to start fetching, use this parameter to paginate across a large collection of history items. In case this parameter is not provided history items will be fetched starting from the most recently created one ordered descending by their creation date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after_history_item_id: Option<String>,
    /// ID of the voice to be filtered for. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// Search term used for filtering history items. If provided, source becomes required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// Unix timestamp to filter history items before this date (exclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_before_unix: Option<i64>,
    /// Unix timestamp to filter history items after this date (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_after_unix: Option<i64>,
    /// Sort direction for the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<HistoryListRequestSortDirection>,
    /// search term used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Source of the generated history item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<HistoryListRequestSource>,
    /// Sources of the generated history items, matching any of them. Cannot be combined with source.
    #[serde(default)]
    pub sources: Vec<Option<HistoryListRequestSourcesItem>>,
}

impl HistoryListQueryRequest {
    pub fn builder() -> HistoryListQueryRequestBuilder {
        <HistoryListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryListQueryRequestBuilder {
    page_size: Option<i64>,
    start_after_history_item_id: Option<String>,
    voice_id: Option<String>,
    model_id: Option<String>,
    date_before_unix: Option<i64>,
    date_after_unix: Option<i64>,
    sort_direction: Option<HistoryListRequestSortDirection>,
    search: Option<String>,
    source: Option<HistoryListRequestSource>,
    sources: Option<Vec<Option<HistoryListRequestSourcesItem>>>,
}

impl HistoryListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn start_after_history_item_id(mut self, value: impl Into<String>) -> Self {
        self.start_after_history_item_id = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn date_before_unix(mut self, value: i64) -> Self {
        self.date_before_unix = Some(value);
        self
    }

    pub fn date_after_unix(mut self, value: i64) -> Self {
        self.date_after_unix = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: HistoryListRequestSortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn source(mut self, value: HistoryListRequestSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<Option<HistoryListRequestSourcesItem>>) -> Self {
        self.sources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HistoryListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sources`](HistoryListQueryRequestBuilder::sources)
    pub fn build(self) -> Result<HistoryListQueryRequest, BuildError> {
        Ok(HistoryListQueryRequest {
            page_size: self.page_size,
            start_after_history_item_id: self.start_after_history_item_id,
            voice_id: self.voice_id,
            model_id: self.model_id,
            date_before_unix: self.date_before_unix,
            date_after_unix: self.date_after_unix,
            sort_direction: self.sort_direction,
            search: self.search,
            source: self.source,
            sources: self.sources.ok_or_else(|| BuildError::missing_field("sources"))?,
        })
    }
}

