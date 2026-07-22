use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AuthConnectionsClient {
    pub http_client: HttpClient,
}

impl AuthConnectionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all auth connections for the workspace
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ListAuthConnectionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/workspace/auth-connections",
                None,
                None,
                options,
            )
            .await
    }

    /// Create a new OAuth2 auth connection for the workspace
    ///
    /// # Arguments
    ///
    /// * `request` - Auth connection to create
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &AuthConnectionsCreateRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<AuthConnectionsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workspace/auth-connections",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an auth connection
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        auth_connection_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/workspace/auth-connections/{}", auth_connection_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an auth connection
    ///
    /// # Arguments
    ///
    /// * `request` - Updated auth connection fields
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        auth_connection_id: &str,
        request: &AuthConnectionsUpdateRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<AuthConnectionsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/workspace/auth-connections/{}", auth_connection_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
