pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A file or web page the receptionists learn from.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionKnowledgeSource {
    /// Whether the source is an uploaded file or a web page or site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<KnowledgeSourceType>,
    /// The page or site URL for 'url' sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// When the URL was last read, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scraped_at_unix: Option<i64>,
    /// The uploaded file's name for 'file' sources; empty for URLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_filename: Option<String>,
    /// The uploaded file's MIME type; empty for URLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The uploaded file's size in bytes; 0 for URLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size_bytes: Option<i64>,
    /// Where processing stands: 'processing', 'ready' or 'error'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<KnowledgeSourceStatus>,
    /// Why processing failed, when status is 'error'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Machine-readable cause of the failure, when status is 'error'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<IngestFailure>,
    /// Whether the whole site was crawled rather than the single page at source_url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawl_entire_site: Option<bool>,
    /// How many pages were read for 'url' sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scraped_pages_count: Option<i64>,
    /// A short plain-text preview of the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_preview: Option<String>,
    /// When processing started, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_started_at_unix: Option<i64>,
    /// The knowledge source's ID.
    #[serde(default)]
    pub id: String,
    /// When this was created, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<UnixTimestamp>,
    /// When this was last updated, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_unix: Option<UnixTimestamp>,
}

impl ReceptionKnowledgeSource {
    pub fn builder() -> ReceptionKnowledgeSourceBuilder {
        <ReceptionKnowledgeSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionKnowledgeSourceBuilder {
    source_type: Option<KnowledgeSourceType>,
    source_url: Option<String>,
    last_scraped_at_unix: Option<i64>,
    original_filename: Option<String>,
    content_type: Option<String>,
    file_size_bytes: Option<i64>,
    status: Option<KnowledgeSourceStatus>,
    error_message: Option<String>,
    failure_reason: Option<IngestFailure>,
    crawl_entire_site: Option<bool>,
    scraped_pages_count: Option<i64>,
    content_preview: Option<String>,
    processing_started_at_unix: Option<i64>,
    id: Option<String>,
    created_at_unix: Option<UnixTimestamp>,
    updated_at_unix: Option<UnixTimestamp>,
}

impl ReceptionKnowledgeSourceBuilder {
    pub fn source_type(mut self, value: KnowledgeSourceType) -> Self {
        self.source_type = Some(value);
        self
    }

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    pub fn last_scraped_at_unix(mut self, value: i64) -> Self {
        self.last_scraped_at_unix = Some(value);
        self
    }

    pub fn original_filename(mut self, value: impl Into<String>) -> Self {
        self.original_filename = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn file_size_bytes(mut self, value: i64) -> Self {
        self.file_size_bytes = Some(value);
        self
    }

    pub fn status(mut self, value: KnowledgeSourceStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn failure_reason(mut self, value: IngestFailure) -> Self {
        self.failure_reason = Some(value);
        self
    }

    pub fn crawl_entire_site(mut self, value: bool) -> Self {
        self.crawl_entire_site = Some(value);
        self
    }

    pub fn scraped_pages_count(mut self, value: i64) -> Self {
        self.scraped_pages_count = Some(value);
        self
    }

    pub fn content_preview(mut self, value: impl Into<String>) -> Self {
        self.content_preview = Some(value.into());
        self
    }

    pub fn processing_started_at_unix(mut self, value: i64) -> Self {
        self.processing_started_at_unix = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: UnixTimestamp) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn updated_at_unix(mut self, value: UnixTimestamp) -> Self {
        self.updated_at_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionKnowledgeSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReceptionKnowledgeSourceBuilder::id)
    pub fn build(self) -> Result<ReceptionKnowledgeSource, BuildError> {
        Ok(ReceptionKnowledgeSource {
            source_type: self.source_type,
            source_url: self.source_url,
            last_scraped_at_unix: self.last_scraped_at_unix,
            original_filename: self.original_filename,
            content_type: self.content_type,
            file_size_bytes: self.file_size_bytes,
            status: self.status,
            error_message: self.error_message,
            failure_reason: self.failure_reason,
            crawl_entire_site: self.crawl_entire_site,
            scraped_pages_count: self.scraped_pages_count,
            content_preview: self.content_preview,
            processing_started_at_unix: self.processing_started_at_unix,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at_unix: self.created_at_unix,
            updated_at_unix: self.updated_at_unix,
        })
    }
}
