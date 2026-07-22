use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod api_keys;
pub use api_keys::ApiKeysClient;
pub struct ServiceAccountsClient {
    pub http_client: HttpClient,
    pub api_keys: ApiKeysClient,
}

impl ServiceAccountsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            api_keys: ApiKeysClient::new(config.clone())?,
        })
    }

    /// List all service accounts in the workspace
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
    ) -> Result<WorkspaceServiceAccountListResponseModel, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/service-accounts", None, None, options)
            .await
    }

    /// Create a new service account in the workspace. By default, a workspace can have up to 20 service accounts. Enterprise customers may request an increase to this limit, up to 100.
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
        request: &BodyCreateServiceAccountV1ServiceAccountsPost,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceCreateServiceAccountResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/service-accounts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
