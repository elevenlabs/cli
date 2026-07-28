pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCrawlJobsResponseModel {
    #[serde(default)]
    pub crawl_jobs: Vec<GetCrawlJobResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl ListCrawlJobsResponseModel {
    pub fn builder() -> ListCrawlJobsResponseModelBuilder {
        <ListCrawlJobsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCrawlJobsResponseModelBuilder {
    crawl_jobs: Option<Vec<GetCrawlJobResponseModel>>,
    next_cursor: Option<String>,
}

impl ListCrawlJobsResponseModelBuilder {
    pub fn crawl_jobs(mut self, value: Vec<GetCrawlJobResponseModel>) -> Self {
        self.crawl_jobs = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCrawlJobsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`crawl_jobs`](ListCrawlJobsResponseModelBuilder::crawl_jobs)
    pub fn build(self) -> Result<ListCrawlJobsResponseModel, BuildError> {
        Ok(ListCrawlJobsResponseModel {
            crawl_jobs: self.crawl_jobs.ok_or_else(|| BuildError::missing_field("crawl_jobs"))?,
            next_cursor: self.next_cursor,
        })
    }
}
