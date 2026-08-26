pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPost {
    /// URL to a page of documentation that the agent will have access to in order to interact with users.
    #[serde(default)]
    pub url: String,
    /// Maximum depth for crawling (1-5), defaults to 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i64>,
    /// Maximum number of pages to crawl (1-10,000), defaults to 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pages: Option<i64>,
    /// If set, only URLs that match this pattern are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// List of URLs to crawl from sitemap (optional, overrides automatic URL discovery).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sitemap_urls: Option<Vec<String>>,
    /// If set, the created document or folder will be placed inside the given folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// Whether to enable auto-sync for this URL document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_sync: Option<bool>,
    /// Whether to automatically remove the document if the URL becomes unavailable. Only applicable when auto-sync is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
    /// Minimum frequency (in days) at which the underlying eligible documents are refreshed. The actual interval may be shorter, never longer. Defaults to 7, tightened to the parent folder's frequency if that is stricter. Only applicable when auto-sync is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_frequency_days: Option<i64>,
}

impl BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPost {
    pub fn builder() -> BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPostBuilder {
        <BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPostBuilder {
    url: Option<String>,
    max_depth: Option<i64>,
    max_pages: Option<i64>,
    pattern: Option<String>,
    sitemap_urls: Option<Vec<String>>,
    parent_folder_id: Option<String>,
    enable_auto_sync: Option<bool>,
    auto_remove: Option<bool>,
    minimum_frequency_days: Option<i64>,
}

impl BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPostBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn max_depth(mut self, value: i64) -> Self {
        self.max_depth = Some(value);
        self
    }

    pub fn max_pages(mut self, value: i64) -> Self {
        self.max_pages = Some(value);
        self
    }

    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.pattern = Some(value.into());
        self
    }

    pub fn sitemap_urls(mut self, value: Vec<String>) -> Self {
        self.sitemap_urls = Some(value);
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn enable_auto_sync(mut self, value: bool) -> Self {
        self.enable_auto_sync = Some(value);
        self
    }

    pub fn auto_remove(mut self, value: bool) -> Self {
        self.auto_remove = Some(value);
        self
    }

    pub fn minimum_frequency_days(mut self, value: i64) -> Self {
        self.minimum_frequency_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPostBuilder::url)
    pub fn build(self) -> Result<BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPost, BuildError> {
        Ok(BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPost {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            max_depth: self.max_depth,
            max_pages: self.max_pages,
            pattern: self.pattern,
            sitemap_urls: self.sitemap_urls,
            parent_folder_id: self.parent_folder_id,
            enable_auto_sync: self.enable_auto_sync,
            auto_remove: self.auto_remove,
            minimum_frequency_days: self.minimum_frequency_days,
        })
    }
}

