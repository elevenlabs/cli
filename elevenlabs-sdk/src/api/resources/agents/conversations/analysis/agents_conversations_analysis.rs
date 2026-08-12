use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AnalysisClient {
    pub http_client: HttpClient,
}

impl AnalysisClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Run the analysis for a conversation using the agent's current evaluation criteria and data collection settings.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - ID of the conversation
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
    ///         .agents
    ///         .conversations
    ///         .analysis
    ///         .run(&"conversation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn run(
        &self,
        conversation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/conversations/{}/analysis/run", conversation_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Rerun a specific evaluation for a conversation.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - ID of the conversation
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
    ///         .agents
    ///         .conversations
    ///         .analysis
    ///         .run_evaluation(
    ///             &"conversation_id".to_string(),
    ///             &RunConversationEvaluationsRequest {
    ///                 evaluation_id: "evaluation_id".to_string(),
    ///                 scope: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn run_evaluation(
        &self,
        conversation_id: &str,
        request: &RunConversationEvaluationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/conversations/{}/analysis/evaluations/run",
                    conversation_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
