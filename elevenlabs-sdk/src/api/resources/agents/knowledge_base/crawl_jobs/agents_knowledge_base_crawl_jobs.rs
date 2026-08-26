use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CrawlJobsClient {
    pub http_client: HttpClient,
}

impl CrawlJobsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a list of ongoing and recent crawl jobs for the user.
    ///
    /// # Arguments
    ///
    /// * `include_job_ids` - Ids of additional crawl jobs to retrieve
    /// * `page_size` - How many documents to return at maximum. Can not exceed 100, defaults to 30.
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .knowledge_base
    ///         .crawl_jobs
    ///         .list(
    ///             &AgentsKnowledgeBaseCrawlJobsListQueryRequest {
    ///                 include_job_ids: vec![Some("include_job_ids".to_string())],
    ///                 page_size: Some(1),
    ///                 cursor: Some("cursor".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AgentsKnowledgeBaseCrawlJobsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCrawlJobsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/knowledge-base/crawl",
                None,
                QueryBuilder::new()
                    .string_array("include_job_ids", request.include_job_ids.clone())
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a crawl job to crawl the given URL with specified depth and page limits.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .knowledge_base
    ///         .crawl_jobs
    ///         .create(
    ///             &BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPost {
    ///                 url: "url".to_string(),
    ///                 max_depth: None,
    ///                 max_pages: None,
    ///                 pattern: None,
    ///                 sitemap_urls: None,
    ///                 parent_folder_id: None,
    ///                 enable_auto_sync: None,
    ///                 auto_remove: None,
    ///                 minimum_frequency_days: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &BodyCreateCrawlJobV1ConvaiKnowledgeBaseCrawlPost,
        options: Option<RequestOptions>,
    ) -> Result<CreateCrawlJobResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/knowledge-base/crawl",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get details about a specific crawl job including status and progress.
    ///
    /// # Arguments
    ///
    /// * `crawl_job_id` - The id of the crawl job to retrieve
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .knowledge_base
    ///         .crawl_jobs
    ///         .get(&"crawl_job_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        crawl_job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetCrawlJobResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/knowledge-base/crawl/{}", crawl_job_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancel a crawl job and delete all associated documents and folders.
    ///
    /// # Arguments
    ///
    /// * `crawl_job_id` - The id of the crawl job to retrieve
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .knowledge_base
    ///         .crawl_jobs
    ///         .cancel(&"crawl_job_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        crawl_job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/knowledge-base/crawl/{}/cancel", crawl_job_id),
                None,
                None,
                options,
            )
            .await
    }
}
