use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LlmUsageClient {
    pub http_client: HttpClient,
}

impl LlmUsageClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a list of LLM models and the expected cost for using them based on the provided values.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn calculate(
        &self,
        request: &LlmUsageCalculatorPublicRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<LlmUsageCalculatorResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/llm-usage/calculate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
