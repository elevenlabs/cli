use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ApiKeysClient {
    pub http_client: HttpClient,
}

impl ApiKeysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all API keys for a service account
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
        service_account_user_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceApiKeyListResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/service-accounts/{}/api-keys", service_account_user_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Create a new API key for a service account
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
        service_account_user_id: &str,
        request: &BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPost,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceCreateApiKeyResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/service-accounts/{}/api-keys", service_account_user_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an existing API key for a service account
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
        service_account_user_id: &str,
        api_key_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/service-accounts/{}/api-keys/{}",
                    service_account_user_id, api_key_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing API key for a service account
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
        service_account_user_id: &str,
        api_key_id: &str,
        request: &BodyEditServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatch,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/service-accounts/{}/api-keys/{}",
                    service_account_user_id, api_key_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
