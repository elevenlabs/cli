use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod dependent_agents;
pub use dependent_agents::DependentAgentsClient;
pub struct AnalysisItemsClient {
    pub http_client: HttpClient,
    pub dependent_agents: DependentAgentsClient,
}

impl AnalysisItemsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            dependent_agents: DependentAgentsClient::new(config.clone())?,
        })
    }

    /// List analysis items available in the workspace.
    ///
    /// # Arguments
    ///
    /// * `search` - If specified, only analysis items whose names start with this string are returned, ordered by relevance rather than sort_direction.
    /// * `page_size` - How many documents to return at maximum. Can not exceed 100, defaults to 30.
    /// * `kind` - If specified, only analysis items of this kind are returned.
    /// * `analysis_item_ids` - If specified, only analysis items with these IDs are returned.
    /// * `created_by_user_id` - Filter by creator user ID. Use '@me' to refer to the authenticated user.
    /// * `sort_direction` - The direction to sort the results
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
    ///         .agents
    ///         .analysis_items
    ///         .list(
    ///             &AgentsAnalysisItemsListQueryRequest {
    ///                 search: Some("search".to_string()),
    ///                 page_size: Some(1),
    ///                 kind: Some(AnalysisItemKind::Evaluation),
    ///                 analysis_item_ids: vec![Some("analysis_item_ids".to_string())],
    ///                 created_by_user_id: Some("created_by_user_id".to_string()),
    ///                 sort_direction: Some(SortDirection::Asc),
    ///                 cursor: Some("cursor".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AgentsAnalysisItemsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnalysisItemsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/analysis-items",
                None,
                QueryBuilder::new()
                    .string("search", request.search.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("kind", request.kind.clone())
                    .string_array("analysis_item_ids", request.analysis_item_ids.clone())
                    .string("created_by_user_id", request.created_by_user_id.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new analysis item (evaluation criterion or data collection) in the workspace.
    ///
    /// # Arguments
    ///
    /// * `request` - Definition of the analysis item (evaluation or data collection).
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
    ///         .analysis_items
    ///         .create(
    ///             &AnalysisItemsCreateRequestBody::Evaluation {
    ///                 data: UserEvaluationAnalysisItem {
    ///                     name: "name".to_string(),
    ///                     conversation_goal_prompt: "conversation_goal_prompt".to_string(),
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
        request: &AnalysisItemsCreateRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<AnalysisItemResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/analysis-items",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Attach or detach one analysis item on multiple agent branches.
    ///
    /// # Arguments
    ///
    /// * `analysis_item_id` - ID of the requested analysis item.
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
    ///         .analysis_items
    ///         .update_attachments(
    ///             &"analysis_item_id".to_string(),
    ///             &UpdateAnalysisItemAttachmentsRequestModel {
    ///                 source: AnalysisItemSource::User,
    ///                 attach: None,
    ///                 detach: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_attachments(
        &self,
        analysis_item_id: &str,
        request: &UpdateAnalysisItemAttachmentsRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<UpdateAnalysisItemAttachmentsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/analysis-items/{}/attachments", analysis_item_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get an analysis item available in the workspace.
    ///
    /// # Arguments
    ///
    /// * `analysis_item_id` - ID of the requested analysis item.
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
    ///         .analysis_items
    ///         .get(&"analysis_item_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        analysis_item_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AnalysisItemResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/analysis-items/{}", analysis_item_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete an analysis item and all of its versions from the workspace.
    ///
    /// # Arguments
    ///
    /// * `analysis_item_id` - ID of the requested analysis item.
    /// * `force` - If set to true, the analysis item will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents and branches.
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
    ///         .analysis_items
    ///         .delete(
    ///             &"analysis_item_id".to_string(),
    ///             &AgentsAnalysisItemsDeleteQueryRequest {
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
        analysis_item_id: &str,
        request: &AgentsAnalysisItemsDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/analysis-items/{}", analysis_item_id),
                None,
                QueryBuilder::new()
                    .bool("force", request.force.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Update an analysis item. Publishes a new version when the content changes.
    ///
    /// # Arguments
    ///
    /// * `analysis_item_id` - ID of the requested analysis item.
    /// * `request` - Definition of the analysis item (evaluation or data collection).
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
    ///         .analysis_items
    ///         .update(
    ///             &"analysis_item_id".to_string(),
    ///             &AnalysisItemsUpdateRequestBody::Evaluation {
    ///                 data: UserEvaluationAnalysisItem {
    ///                     name: "name".to_string(),
    ///                     conversation_goal_prompt: "conversation_goal_prompt".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        analysis_item_id: &str,
        request: &AnalysisItemsUpdateRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<AnalysisItemResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/analysis-items/{}", analysis_item_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the published versions of an analysis item, newest first.
    ///
    /// # Arguments
    ///
    /// * `analysis_item_id` - ID of the requested analysis item.
    /// * `page_size` - How many documents to return at maximum. Can not exceed 100, defaults to 30.
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
    ///         .agents
    ///         .analysis_items
    ///         .list_versions(
    ///             &"analysis_item_id".to_string(),
    ///             &ListVersionsQueryRequest {
    ///                 page_size: Some(1),
    ///                 cursor: Some("cursor".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_versions(
        &self,
        analysis_item_id: &str,
        request: &ListVersionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AnalysisItemVersionsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/analysis-items/{}/versions", analysis_item_id),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new analysis item from this item's current definition.
    ///
    /// # Arguments
    ///
    /// * `analysis_item_id` - ID of the requested analysis item.
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
    ///         .analysis_items
    ///         .copy(&"analysis_item_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn copy(
        &self,
        analysis_item_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AnalysisItemResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/analysis-items/{}/copy", analysis_item_id),
                None,
                None,
                options,
            )
            .await
    }
}
