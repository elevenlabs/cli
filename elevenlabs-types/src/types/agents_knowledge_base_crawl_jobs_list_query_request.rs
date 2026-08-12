pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsKnowledgeBaseCrawlJobsListQueryRequest {
    /// Ids of additional crawl jobs to retrieve
    #[serde(default)]
    pub include_job_ids: Vec<Option<String>>,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsKnowledgeBaseCrawlJobsListQueryRequest {
    pub fn builder() -> AgentsKnowledgeBaseCrawlJobsListQueryRequestBuilder {
        <AgentsKnowledgeBaseCrawlJobsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsKnowledgeBaseCrawlJobsListQueryRequestBuilder {
    include_job_ids: Option<Vec<Option<String>>>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl AgentsKnowledgeBaseCrawlJobsListQueryRequestBuilder {
    pub fn include_job_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.include_job_ids = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsKnowledgeBaseCrawlJobsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`include_job_ids`](AgentsKnowledgeBaseCrawlJobsListQueryRequestBuilder::include_job_ids)
    pub fn build(self) -> Result<AgentsKnowledgeBaseCrawlJobsListQueryRequest, BuildError> {
        Ok(AgentsKnowledgeBaseCrawlJobsListQueryRequest {
            include_job_ids: self.include_job_ids.ok_or_else(|| BuildError::missing_field("include_job_ids"))?,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

