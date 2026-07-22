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
    ///     client.workspace.auth_connections.list(None).await;
    /// }
    /// ```
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
    ///         .workspace
    ///         .auth_connections
    ///         .create(
    ///             &AuthConnectionsCreateRequestBody::Oauth2ClientCredentials {
    ///                 data: CreateOAuth2ClientCredsRequest {
    ///                     name: "name".to_string(),
    ///                     provider: "provider".to_string(),
    ///                     client_id: "client_id".to_string(),
    ///                     token_url: "token_url".to_string(),
    ///                     client_secret: "client_secret".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .workspace
    ///         .auth_connections
    ///         .delete(&"auth_connection_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
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
    ///         .workspace
    ///         .auth_connections
    ///         .update(
    ///             &"auth_connection_id".to_string(),
    ///             &AuthConnectionsUpdateRequestBody::Oauth2ClientCredentials {
    ///                 data: UpdateOAuth2ClientCredsRequest {
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
