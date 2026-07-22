use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod folders;
pub use folders::FoldersClient;
pub mod invocations;
pub use invocations::InvocationsClient;
pub struct TestsClient {
    pub http_client: HttpClient,
    pub folders: FoldersClient,
    pub invocations: InvocationsClient,
}

impl TestsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            folders: FoldersClient::new(config.clone())?,
            invocations: InvocationsClient::new(config.clone())?,
        })
    }

    /// Creates a new agent response test.
    ///
    /// # Arguments
    ///
    /// * `request` - Create Chat Response Test Request Information
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &TestsCreateRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<CreateAgentTestResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/agent-testing/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Moves multiple tests or folders from one folder to another.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn move_(
        &self,
        request: &BodyBulkMoveTestsToFolderV1ConvaiAgentTestingBulkMovePost,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/agent-testing/bulk-move",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Gets an agent response test by ID.
    ///
    /// # Arguments
    ///
    /// * `test_id` - The id of a chat response test. This is returned on test creation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        test_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TestsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agent-testing/{}", test_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an agent response test by ID.
    ///
    /// # Arguments
    ///
    /// * `test_id` - The id of a chat response test. This is returned on test creation.
    /// * `request` - Agent test to update
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        test_id: &str,
        request: &TestsUpdateRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<TestsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/convai/agent-testing/{}", test_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes an agent response test by ID.
    ///
    /// # Arguments
    ///
    /// * `test_id` - The id of a chat response test. This is returned on test creation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        test_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/agent-testing/{}", test_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Gets multiple agent response tests by their IDs. Returns a dictionary mapping test IDs to test summaries.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn summaries(
        &self,
        request: &ListTestsByIdsRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<GetTestsSummariesByIdsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/agent-testing/summaries",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists all agent response tests with pagination support and optional search filtering.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `page_size` - How many Tests to return at maximum. Can not exceed 100, defaults to 30.
    /// * `search` - Search query to filter tests by name.
    /// * `parent_folder_id` - Filter by parent folder ID. Use 'root' to get items in the root folder.
    /// * `types` - If present, the endpoint will return only tests/folders of the given types.
    /// * `include_folders` - Deprecated. Use the `types` query param and include `folder` instead.
    /// * `sort_mode` - Sort mode for listing tests. Use 'folders_first' to place folders before tests.
    /// * `sharing_mode` - Filter test visibility. Use `shared_with_me` to return only tests/folders shared with the current user that they did not create.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ConversationalAiTestsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTestsPageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/agent-testing",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .string("search", request.search.clone())
                    .string("parent_folder_id", request.parent_folder_id.clone())
                    .serialize_array("types", request.types.clone())
                    .bool("include_folders", request.include_folders.clone())
                    .serialize("sort_mode", request.sort_mode.clone())
                    .serialize("sharing_mode", request.sharing_mode.clone())
                    .build(),
                options,
            )
            .await
    }
}
