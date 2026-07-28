use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SecretsClient {
    pub http_client: HttpClient,
}

impl SecretsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all workspace secrets for the user
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many documents to return at maximum. Can not exceed 100. If not provided, returns all secrets.
    /// * `dependency_limit` - Maximum number of dependent resources (tools, agents, phone numbers) to return per secret. Can not exceed 100.
    /// * `search` - If specified, returns only secrets whose names start with this string.
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
    ///         .secrets
    ///         .list(
    ///             &AgentsSecretsListQueryRequest {
    ///                 page_size: Some(1),
    ///                 dependency_limit: Some(1),
    ///                 search: Some("search".to_string()),
    ///                 cursor: Some("cursor".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AgentsSecretsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetWorkspaceSecretsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/secrets",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("dependency_limit", request.dependency_limit.clone())
                    .string("search", request.search.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new secret for the workspace
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
    ///         .secrets
    ///         .create(
    ///             &PostWorkspaceSecretRequest {
    ///                 r#type: "new".to_string(),
    ///                 name: "name".to_string(),
    ///                 value: "value".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &PostWorkspaceSecretRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostWorkspaceSecretResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/secrets",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a workspace secret by ID
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
    ///         .secrets
    ///         .get(&"secret_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        secret_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConvAiWorkspaceStoredSecretConfig, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/secrets/{}", secret_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a workspace secret if it's not in use
    ///
    /// # Arguments
    ///
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
    ///         .secrets
    ///         .delete(&"secret_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        secret_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/secrets/{}", secret_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing secret for the workspace
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
    ///         .secrets
    ///         .update(
    ///             &"secret_id".to_string(),
    ///             &PatchWorkspaceSecretRequest {
    ///                 r#type: "update".to_string(),
    ///                 name: "name".to_string(),
    ///                 value: "value".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        secret_id: &str,
        request: &PatchWorkspaceSecretRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostWorkspaceSecretResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/secrets/{}", secret_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get paginated list of resources that depend on a specific secret, filtered by resource type.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many dependency items to return per page.
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
    ///         .secrets
    ///         .get_dependencies(
    ///             &"secret_id".to_string(),
    ///             &SecretDependencyResourceType::Tools,
    ///             &GetDependenciesQueryRequest {
    ///                 page_size: Some(1),
    ///                 cursor: Some("cursor".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_dependencies(
        &self,
        secret_id: &str,
        resource_type: &SecretDependencyResourceType,
        request: &GetDependenciesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSecretDependenciesResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/secrets/{}/dependencies/{}",
                    secret_id, resource_type
                ),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
