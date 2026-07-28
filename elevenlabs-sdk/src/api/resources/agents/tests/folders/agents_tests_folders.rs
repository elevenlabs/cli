use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct FoldersClient {
    pub http_client: HttpClient,
}

impl FoldersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a folder for organizing agent tests.
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
    ///         .tests
    ///         .folders
    ///         .create(
    ///             &BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPost {
    ///                 name: "name".to_string(),
    ///                 parent_folder_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &BodyCreateAgentTestFolderV1ConvaiAgentTestingFoldersPost,
        options: Option<RequestOptions>,
    ) -> Result<CreateAgentTestFolderResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/agent-testing/folders",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Gets an agent test folder by ID, including its folder path.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The folder ID.
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
    ///         .tests
    ///         .folders
    ///         .get(&"tfld_7301khxdkycse5f88fzjdtrterzm".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        folder_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentTestFolderResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agent-testing/folders/{}", folder_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes an agent test folder by ID. Use force=true to delete a non-empty folder and all its contents.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The folder ID.
    /// * `force` - Force delete. Required for deleting non-empty folders.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .tests
    ///         .folders
    ///         .delete(
    ///             &"tfld_7301khxdkycse5f88fzjdtrterzm".to_string(),
    ///             &AgentsTestsFoldersDeleteQueryRequest {
    ///                 force: Some(true),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        folder_id: &str,
        request: &AgentsTestsFoldersDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/agent-testing/folders/{}", folder_id),
                None,
                QueryBuilder::new()
                    .bool("force", request.force.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates an agent test folder. Currently only supports updating the folder name.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The folder ID.
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
    ///         .tests
    ///         .folders
    ///         .update(
    ///             &"tfld_7301khxdkycse5f88fzjdtrterzm".to_string(),
    ///             &BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIDPatch {
    ///                 name: "name".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        folder_id: &str,
        request: &BodyUpdateAgentTestFolderV1ConvaiAgentTestingFoldersFolderIdPatch,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentTestFolderResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/agent-testing/folders/{}", folder_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
