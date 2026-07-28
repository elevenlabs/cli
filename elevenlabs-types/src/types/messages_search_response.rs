pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MessagesSearchResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ListResponseMeta>,
    #[serde(default)]
    pub results: Vec<MessagesSearchResult>,
    /// Cursor for the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether there are more results available
    #[serde(default)]
    pub has_more: bool,
}

impl MessagesSearchResponse {
    pub fn builder() -> MessagesSearchResponseBuilder {
        <MessagesSearchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessagesSearchResponseBuilder {
    meta: Option<ListResponseMeta>,
    results: Option<Vec<MessagesSearchResult>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl MessagesSearchResponseBuilder {
    pub fn meta(mut self, value: ListResponseMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<MessagesSearchResult>) -> Self {
        self.results = Some(value);
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

    /// Consumes the builder and constructs a [`MessagesSearchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](MessagesSearchResponseBuilder::results)
    /// - [`has_more`](MessagesSearchResponseBuilder::has_more)
    pub fn build(self) -> Result<MessagesSearchResponse, BuildError> {
        Ok(MessagesSearchResponse {
            meta: self.meta,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
