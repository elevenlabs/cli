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
    pub async fn list(
        &self,
        request: &ConversationalAiSecretsListQueryRequest,
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
