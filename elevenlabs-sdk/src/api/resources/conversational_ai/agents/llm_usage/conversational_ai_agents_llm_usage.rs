use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LlmUsageClient2 {
    pub http_client: HttpClient,
}

impl LlmUsageClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Calculates expected number of LLM tokens needed for the specified agent.
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
        agent_id: &str,
        request: &LlmUsageCalculatorRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<LlmUsageCalculatorResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/agent/{}/llm-usage/calculate", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
