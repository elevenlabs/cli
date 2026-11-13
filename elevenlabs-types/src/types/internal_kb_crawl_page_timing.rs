pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Per-document timing data for one crawled page.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InternalKbCrawlPageTiming {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub doc_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_len: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
}

impl InternalKbCrawlPageTiming {
    pub fn builder() -> InternalKbCrawlPageTimingBuilder {
        <InternalKbCrawlPageTimingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InternalKbCrawlPageTimingBuilder {
    url: Option<String>,
    doc_id: Option<String>,
    read_ms: Option<i64>,
    index_ms: Option<i64>,
    html_len: Option<i64>,
    skipped: Option<bool>,
    error: Option<bool>,
}

impl InternalKbCrawlPageTimingBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn doc_id(mut self, value: impl Into<String>) -> Self {
        self.doc_id = Some(value.into());
        self
    }

    pub fn read_ms(mut self, value: i64) -> Self {
        self.read_ms = Some(value);
        self
    }

    pub fn index_ms(mut self, value: i64) -> Self {
        self.index_ms = Some(value);
        self
    }

    pub fn html_len(mut self, value: i64) -> Self {
        self.html_len = Some(value);
        self
    }

    pub fn skipped(mut self, value: bool) -> Self {
        self.skipped = Some(value);
        self
    }

    pub fn error(mut self, value: bool) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InternalKbCrawlPageTiming`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](InternalKbCrawlPageTimingBuilder::url)
    /// - [`doc_id`](InternalKbCrawlPageTimingBuilder::doc_id)
    pub fn build(self) -> Result<InternalKbCrawlPageTiming, BuildError> {
        Ok(InternalKbCrawlPageTiming {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            doc_id: self.doc_id.ok_or_else(|| BuildError::missing_field("doc_id"))?,
            read_ms: self.read_ms,
            index_ms: self.index_ms,
            html_len: self.html_len,
            skipped: self.skipped,
            error: self.error,
        })
    }
}
