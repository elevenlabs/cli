pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCrawlJobResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CrawlType>,
    #[serde(default)]
    pub seed_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default)]
    pub max_depth: i64,
    #[serde(default)]
    pub max_pages: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CrawlStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages_identified: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages_scraped: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages_skipped: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages_failed: Option<i64>,
    #[serde(default)]
    pub root_folder_id: String,
    #[serde(default)]
    pub updated_at: i64,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub created_at: i64,
}

impl GetCrawlJobResponseModel {
    pub fn builder() -> GetCrawlJobResponseModelBuilder {
        <GetCrawlJobResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCrawlJobResponseModelBuilder {
    r#type: Option<CrawlType>,
    seed_url: Option<String>,
    pattern: Option<String>,
    max_depth: Option<i64>,
    max_pages: Option<i64>,
    status: Option<CrawlStatus>,
    pages_identified: Option<i64>,
    pages_scraped: Option<i64>,
    pages_skipped: Option<i64>,
    pages_failed: Option<i64>,
    root_folder_id: Option<String>,
    updated_at: Option<i64>,
    id: Option<String>,
    created_at: Option<i64>,
}

impl GetCrawlJobResponseModelBuilder {
    pub fn r#type(mut self, value: CrawlType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn seed_url(mut self, value: impl Into<String>) -> Self {
        self.seed_url = Some(value.into());
        self
    }

    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.pattern = Some(value.into());
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

    pub fn status(mut self, value: CrawlStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn pages_identified(mut self, value: i64) -> Self {
        self.pages_identified = Some(value);
        self
    }

    pub fn pages_scraped(mut self, value: i64) -> Self {
        self.pages_scraped = Some(value);
        self
    }

    pub fn pages_skipped(mut self, value: i64) -> Self {
        self.pages_skipped = Some(value);
        self
    }

    pub fn pages_failed(mut self, value: i64) -> Self {
        self.pages_failed = Some(value);
        self
    }

    pub fn root_folder_id(mut self, value: impl Into<String>) -> Self {
        self.root_folder_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: i64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCrawlJobResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`seed_url`](GetCrawlJobResponseModelBuilder::seed_url)
    /// - [`max_depth`](GetCrawlJobResponseModelBuilder::max_depth)
    /// - [`max_pages`](GetCrawlJobResponseModelBuilder::max_pages)
    /// - [`root_folder_id`](GetCrawlJobResponseModelBuilder::root_folder_id)
    /// - [`updated_at`](GetCrawlJobResponseModelBuilder::updated_at)
    /// - [`id`](GetCrawlJobResponseModelBuilder::id)
    /// - [`created_at`](GetCrawlJobResponseModelBuilder::created_at)
    pub fn build(self) -> Result<GetCrawlJobResponseModel, BuildError> {
        Ok(GetCrawlJobResponseModel {
            r#type: self.r#type,
            seed_url: self.seed_url.ok_or_else(|| BuildError::missing_field("seed_url"))?,
            pattern: self.pattern,
            max_depth: self.max_depth.ok_or_else(|| BuildError::missing_field("max_depth"))?,
            max_pages: self.max_pages.ok_or_else(|| BuildError::missing_field("max_pages"))?,
            status: self.status,
            pages_identified: self.pages_identified,
            pages_scraped: self.pages_scraped,
            pages_skipped: self.pages_skipped,
            pages_failed: self.pages_failed,
            root_folder_id: self.root_folder_id.ok_or_else(|| BuildError::missing_field("root_folder_id"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
