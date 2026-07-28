use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EnvironmentVariablesClient {
    pub http_client: HttpClient,
}

impl EnvironmentVariablesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List all environment variables for the workspace with optional filtering
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor from previous response
    /// * `page_size` - Number of items to return (1-100)
    /// * `label` - Filter by exact label match
    /// * `environment` - Filter to only return variables that have this environment. When specified, the values dict in the response will only contain this environment.
    /// * `type_` - Filter by variable type
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
    ///         .environment_variables
    ///         .list(
    ///             &EnvironmentVariablesListQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 label: Some("label".to_string()),
    ///                 environment: Some("environment".to_string()),
    ///                 r#type: Some(EnvironmentVariablesListRequestType::String),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &EnvironmentVariablesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<EnvironmentVariablesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/environment-variables",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .string("label", request.label.clone())
                    .string("environment", request.environment.clone())
                    .serialize("type", request.r#type.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new environment variable for the workspace
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
    ///         .environment_variables
    ///         .create(
    ///             &EnvironmentVariablesCreateRequestBody::r#String {
    ///                 data: CreateStringEnvironmentVariableRequest {
    ///                     label: "label".to_string(),
    ///                     values: HashMap::from([("key".to_string(), "value".to_string())]),
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
        request: &EnvironmentVariablesCreateRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<EnvironmentVariableResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/environment-variables",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a specific environment variable by ID
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
    ///         .environment_variables
    ///         .get(&"env_var_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        env_var_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<EnvironmentVariableResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/environment-variables/{}", env_var_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Replace an environment variable's values. Use null to remove an environment (except production).
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
    ///         .environment_variables
    ///         .update(
    ///             &"env_var_id".to_string(),
    ///             &UpdateEnvironmentVariableRequest {
    ///                 values: HashMap::from([]),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        env_var_id: &str,
        request: &UpdateEnvironmentVariableRequest,
        options: Option<RequestOptions>,
    ) -> Result<EnvironmentVariableResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/environment-variables/{}", env_var_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
