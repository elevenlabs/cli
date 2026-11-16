use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ProceduresClient2 {
    pub http_client: HttpClient,
}

impl ProceduresClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the procedures (SOPs) on the named receptionist, with the tools each one is missing.
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
    ///         .procedures
    ///         .list(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        agent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ReceptionProcedureResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/reception/customer-facing-agents/{}/procedures",
                    agent_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Add a procedure (SOP) to the named receptionist.
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
    ///         .procedures
    ///         .create(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionCreateProcedureRequest {
    ///                 name: StrippedStr("name".to_string()),
    ///                 trigger: StrippedStr("trigger".to_string()),
    ///                 body: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        agent_id: &str,
        request: &ReceptionCreateProcedureRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionProcedure, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/reception/customer-facing-agents/{}/procedures",
                    agent_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Replace an existing procedure on the named receptionist. Send every field: an omitted `body` resets to empty.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
    /// * `procedure_id` - ID of the procedure. Returned when you add one; list them with `GET /v1/reception/customer-facing-agents/{agent_id}/procedures`.
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
    ///         .procedures
    ///         .update(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &"agtprc_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionUpdateProcedureRequest {
    ///                 name: StrippedStr("name".to_string()),
    ///                 trigger: StrippedStr("trigger".to_string()),
    ///                 body: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        agent_id: &str,
        procedure_id: &str,
        request: &ReceptionUpdateProcedureRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionProcedure, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/reception/customer-facing-agents/{}/procedures/{}",
                    agent_id, procedure_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a procedure from the named receptionist.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the receptionist. List them with `GET /v1/reception/customer-facing-agents`.
    /// * `procedure_id` - ID of the procedure. Returned when you add one; list them with `GET /v1/reception/customer-facing-agents/{agent_id}/procedures`.
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
    ///         .procedures
    ///         .delete(
    ///             &"agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &"agtprc_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        agent_id: &str,
        procedure_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/reception/customer-facing-agents/{}/procedures/{}",
                    agent_id, procedure_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
