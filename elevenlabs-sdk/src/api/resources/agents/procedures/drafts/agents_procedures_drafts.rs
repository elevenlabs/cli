use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DraftsClient2 {
    pub http_client: HttpClient,
}

impl DraftsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get user's draft for a procedure
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
    /// * `procedure_id` - The procedure ID
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
    ///         .procedures
    ///         .drafts
    ///         .get(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbranch_0901k4aafjxxfxt93gd841r7tv5t".to_string(),
    ///             &"agtprc_6qbpwdq8n01bxhk44bgjy6f10ck3".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        agent_id: &str,
        branch_id: &str,
        procedure_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProcedureDraftResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/agents/{}/branches/{}/procedures/{}/draft",
                    agent_id, branch_id, procedure_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete user's draft for a procedure, resetting to the committed version
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
    /// * `procedure_id` - The procedure ID
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
    ///         .procedures
    ///         .drafts
    ///         .delete(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbranch_0901k4aafjxxfxt93gd841r7tv5t".to_string(),
    ///             &"agtprc_6qbpwdq8n01bxhk44bgjy6f10ck3".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        agent_id: &str,
        branch_id: &str,
        procedure_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/convai/agents/{}/branches/{}/procedures/{}/draft",
                    agent_id, branch_id, procedure_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Create or update user's draft for a procedure
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
    /// * `procedure_id` - The procedure ID
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
    ///         .procedures
    ///         .drafts
    ///         .update(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbranch_0901k4aafjxxfxt93gd841r7tv5t".to_string(),
    ///             &"agtprc_6qbpwdq8n01bxhk44bgjy6f10ck3".to_string(),
    ///             &UpdateProcedureDraftRequestModel {
    ///                 name: "name".to_string(),
    ///                 content: "content".to_string(),
    ///                 r#type: ProcedureType::FreeForm,
    ///                 trigger: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        agent_id: &str,
        branch_id: &str,
        procedure_id: &str,
        request: &UpdateProcedureDraftRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<ProcedureDraftResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/convai/agents/{}/branches/{}/procedures/{}/draft",
                    agent_id, branch_id, procedure_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
