use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DeploymentsClient {
    pub http_client: HttpClient,
}

impl DeploymentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a new deployment for an agent
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
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
    ///         .conversational_ai
    ///         .agents
    ///         .deployments
    ///         .create(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIDDeploymentsPost {
    ///                 deployment_request: AgentDeploymentRequest {
    ///                     requests: vec![AgentDeploymentRequestItem {
    ///                         branch_id: "agtbrch_8901k4t9z5defmb8vh3e9361y7nj".to_string(),
    ///                         deployment_strategy: AgentDeploymentPercentageStrategy {
    ///                             r#type: Some("percentage".to_string()),
    ///                             traffic_percentage: 0.5,
    ///                             ..Default::default()
    ///                         },
    ///                         ..Default::default()
    ///                     }],
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
        agent_id: &str,
        request: &BodyCreateOrUpdateDeploymentsV1ConvaiAgentsAgentIdDeploymentsPost,
        options: Option<RequestOptions>,
    ) -> Result<AgentDeploymentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/agents/{}/deployments", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
