use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ApiKeysClient2 {
    pub http_client: HttpClient,
}

impl ApiKeysClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Disable the API key used to authenticate this request. Requires the query parameter `api_key_name=self` as an explicit confirmation.
    ///
    /// # Arguments
    ///
    /// * `api_key_name` - Must be set to `self` to disable the API key used to authenticate this request. Required as an explicit confirmation to avoid accidentally disabling the wrong key.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn disable(
        &self,
        request: &DisableQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workspaces/api-keys/disable",
                None,
                QueryBuilder::new()
                    .string("api_key_name", request.api_key_name.clone())
                    .build(),
                options,
            )
            .await
    }
}
