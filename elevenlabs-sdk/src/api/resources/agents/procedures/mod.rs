use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod drafts;
pub use drafts::DraftsClient2;
pub struct ProceduresClient {
    pub http_client: HttpClient,
    pub drafts: DraftsClient2,
}

impl ProceduresClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            drafts: DraftsClient2::new(config.clone())?,
        })
    }

    /// List the agent's procedures on a branch with their procedure_id, version_id, name, type, trigger, and has_draft. has_draft is true when a procedure has unpublished draft changes on this branch; its name/type/trigger then reflect that draft. Does not return procedure content -- use Get Procedure to read a procedure's body.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
    /// * `agent_version_id` - The agent version ID to retrieve the procedure for.
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
    ///         .list(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbranch_0901k4aafjxxfxt93gd841r7tv5t".to_string(),
    ///             &AgentsProceduresListQueryRequest {
    ///                 agent_version_id: Some("agent_version_id".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        agent_id: &str,
        branch_id: &str,
        request: &AgentsProceduresListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListProceduresResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/agents/{}/branches/{}/procedures",
                    agent_id, branch_id
                ),
                None,
                QueryBuilder::new()
                    .string("agent_version_id", request.agent_version_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new procedure for the agent on a branch.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
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
    ///         .create(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbranch_0901k4aafjxxfxt93gd841r7tv5t".to_string(),
    ///             &Some(CreateProcedureRequestModel {
    ///                 ..Default::default()
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        agent_id: &str,
        branch_id: &str,
        request: &Option<CreateProcedureRequestModel>,
        options: Option<RequestOptions>,
    ) -> Result<CreateProcedureResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/agents/{}/branches/{}/procedures",
                    agent_id, branch_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Compile procedure drafts into a workflow.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
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
    ///         .compile(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbranch_0901k4aafjxxfxt93gd841r7tv5t".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn compile(
        &self,
        agent_id: &str,
        branch_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CompileProceduresResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/agents/{}/branches/{}/procedures/compile",
                    agent_id, branch_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve a procedure at a specific version or the current branch HEAD.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent ID to get the procedure draft from
    /// * `branch_id` - Branch ID to get the procedure draft from
    /// * `procedure_id` - The procedure ID
    /// * `version_id` - The version ID to retrieve. If omitted, returns the version at branch HEAD.
    /// * `agent_version_id` - The agent version ID to retrieve the procedure for.
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
    ///         .get(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbranch_0901k4aafjxxfxt93gd841r7tv5t".to_string(),
    ///             &"agtprc_6qbpwdq8n01bxhk44bgjy6f10ck3".to_string(),
    ///             &AgentsProceduresGetQueryRequest {
    ///                 version_id: Some("version_id".to_string()),
    ///                 agent_version_id: Some("agent_version_id".to_string()),
    ///                 ..Default::default()
    ///             },
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
        request: &AgentsProceduresGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProcedureAtVersionResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/agents/{}/branches/{}/procedures/{}",
                    agent_id, branch_id, procedure_id
                ),
                None,
                QueryBuilder::new()
                    .string("version_id", request.version_id.clone())
                    .string("agent_version_id", request.agent_version_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Remove a procedure from the agent's draft working set. Removing a folder cascades to its entire subtree, rejected if any procedure outside the subtree hands off into it.
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
    ///         .remove(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbranch_0901k4aafjxxfxt93gd841r7tv5t".to_string(),
    ///             &"agtprc_6qbpwdq8n01bxhk44bgjy6f10ck3".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn remove(
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
                    "v1/convai/agents/{}/branches/{}/procedures/{}",
                    agent_id, branch_id, procedure_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
