use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod executions;
pub use executions::ExecutionsClient;
pub struct ToolsClient {
    pub http_client: HttpClient,
    pub executions: ExecutionsClient,
}

impl ToolsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            executions: ExecutionsClient::new(config.clone())?,
        })
    }

    /// Get all available tools in the workspace.
    ///
    /// # Arguments
    ///
    /// * `search` - If specified, the endpoint returns only tools whose names start with this string.
    /// * `page_size` - How many documents to return at maximum. Can not exceed 100, defaults to 30.
    /// * `show_only_owned_documents` - If set to true, the endpoint will return only tools owned by you (and not shared from somebody else). Deprecated: use created_by_user_id instead.
    /// * `created_by_user_id` - Filter tools by creator user ID. When set, only tools created by this user are returned. Takes precedence over show_only_owned_documents. Use '@me' to refer to the authenticated user.
    /// * `types` - If present, the endpoint will return only tools of the given types.
    /// * `sort_direction` - The direction to sort the results
    /// * `sort_by` - The field to sort the results by
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
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
    ///         .tools
    ///         .list(
    ///             &ConversationalAiToolsListQueryRequest {
    ///                 search: Some("search".to_string()),
    ///                 page_size: Some(1),
    ///                 show_only_owned_documents: Some(true),
    ///                 created_by_user_id: Some("created_by_user_id".to_string()),
    ///                 types: vec![Some(ToolTypeFilter::Webhook)],
    ///                 sort_direction: Some(SortDirection::Asc),
    ///                 sort_by: Some(ToolSortBy::Name),
    ///                 cursor: Some("cursor".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ConversationalAiToolsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ToolsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/tools",
                None,
                QueryBuilder::new()
                    .string("search", request.search.clone())
                    .int("page_size", request.page_size.clone())
                    .bool(
                        "show_only_owned_documents",
                        request.show_only_owned_documents.clone(),
                    )
                    .string("created_by_user_id", request.created_by_user_id.clone())
                    .serialize_array("types", request.types.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new tool to the available tools in the workspace.
    ///
    /// # Arguments
    ///
    /// * `request` - A tool that an agent can provide to LLM.
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
    ///         .tools
    ///         .create(
    ///             &ToolRequestModel {
    ///                 tool_config: ToolRequestModelToolConfig::Client {
    ///                     data: ClientToolConfigInput {
    ///                         name: "name".to_string(),
    ///                         description: "description".to_string(),
    ///                         expects_response: Some(false),
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///                 response_mocks: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &ToolRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<ToolResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/tools",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get tool that is available in the workspace.
    ///
    /// # Arguments
    ///
    /// * `tool_id` - ID of the requested tool.
    /// * `environment` - Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production.
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
    ///         .tools
    ///         .get(
    ///             &"tool_id".to_string(),
    ///             &ConversationalAiToolsGetQueryRequest {
    ///                 environment: Some("environment".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        tool_id: &str,
        request: &ConversationalAiToolsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ToolResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/tools/{}", tool_id),
                None,
                QueryBuilder::new()
                    .string("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete tool from the workspace.
    ///
    /// # Arguments
    ///
    /// * `tool_id` - ID of the requested tool.
    /// * `force` - If set to true, the tool will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents and branches.
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
    ///         .tools
    ///         .delete(
    ///             &"tool_id".to_string(),
    ///             &ConversationalAiToolsDeleteQueryRequest {
    ///                 force: Some(true),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        tool_id: &str,
        request: &ConversationalAiToolsDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/tools/{}", tool_id),
                None,
                QueryBuilder::new()
                    .bool("force", request.force.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Update tool that is available in the workspace.
    ///
    /// # Arguments
    ///
    /// * `tool_id` - ID of the requested tool.
    /// * `request` - A tool that an agent can provide to LLM.
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
    ///         .tools
    ///         .update(
    ///             &"tool_id".to_string(),
    ///             &ToolRequestModel {
    ///                 tool_config: ToolRequestModelToolConfig::Client {
    ///                     data: ClientToolConfigInput {
    ///                         name: "name".to_string(),
    ///                         description: "description".to_string(),
    ///                         expects_response: Some(false),
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///                 response_mocks: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        tool_id: &str,
        request: &ToolRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<ToolResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/tools/{}", tool_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a list of agents depending on this tool
    ///
    /// # Arguments
    ///
    /// * `tool_id` - ID of the requested tool.
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `page_size` - How many documents to return at maximum. Can not exceed 100, defaults to 30.
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
    ///         .tools
    ///         .get_dependent_agents(
    ///             &"tool_id".to_string(),
    ///             &GetDependentAgentsQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_dependent_agents(
        &self,
        tool_id: &str,
        request: &GetDependentAgentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetToolDependentAgentsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/tools/{}/dependent-agents", tool_id),
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .build(),
                options,
            )
            .await
    }
}
