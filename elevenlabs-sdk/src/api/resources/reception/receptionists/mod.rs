use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod rules;
pub use rules::RulesClient2;
pub mod procedures;
pub use procedures::ProceduresClient2;
pub mod transfer_rules;
pub use transfer_rules::TransferRulesClient;
pub struct ReceptionistsClient {
    pub http_client: HttpClient,
    pub rules: RulesClient2,
    pub procedures: ProceduresClient2,
    pub transfer_rules: TransferRulesClient,
}

impl ReceptionistsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            rules: RulesClient2::new(config.clone())?,
            procedures: ProceduresClient2::new(config.clone())?,
            transfer_rules: TransferRulesClient::new(config.clone())?,
        })
    }

    /// List every customer-facing receptionist for the workspace.
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
    ///         .reception
    ///         .receptionists
    ///         .list(Some(
    ///             RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id"),
    ///         ))
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ReceptionCustomerFacingAgentResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/reception/customer-facing-agents",
                None,
                None,
                options,
            )
            .await
    }

    /// Update a receptionist's display name, config and/or widget appearance.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
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
    ///         .reception
    ///         .receptionists
    ///         .update(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionUpdateCustomerFacingAgentRequest {
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        agent_id: &str,
        request: &ReceptionUpdateCustomerFacingAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionCustomerFacingAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/reception/customer-facing-agents/{}", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Re-activate a deactivated receptionist. Fails with `quota_exceeded` when the plan has no free receptionist slot.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
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
    ///         .reception
    ///         .receptionists
    ///         .activate(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn activate(
        &self,
        agent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionCustomerFacingAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/reception/customer-facing-agents/{}/activate", agent_id),
                None,
                None,
                options,
            )
            .await
    }
}
