use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SingleUseClient {
    pub http_client: HttpClient,
}

impl SingleUseClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Generate a time limited single-use token with embedded authentication for frontend clients.
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
        token_type: &SingleUseTokenType,
        options: Option<RequestOptions>,
    ) -> Result<SingleUseTokenResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/single-use-token/{}", token_type),
                None,
                None,
                options,
            )
            .await
    }
}
