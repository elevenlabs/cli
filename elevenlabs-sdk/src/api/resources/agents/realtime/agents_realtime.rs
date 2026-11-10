use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RealtimeClient {
    pub http_client: HttpClient,
}

impl RealtimeClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the realtime conversation groups the current user can view.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many conversation groups to return. Can not exceed 100.
    /// * `search` - If specified, only conversation groups whose title or description contains this string (case-insensitive) are returned.
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
    ///         .realtime
    ///         .list_conversation_groups(
    ///             &ListConversationGroupsQueryRequest {
    ///                 page_size: Some(1),
    ///                 search: Some("search".to_string()),
    ///                 cursor: Some("cursor".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_conversation_groups(
        &self,
        request: &ListConversationGroupsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RealtimeConversationGroupsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/realtime/conversation-groups",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("search", request.search.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a realtime conversation group owned by the current user.
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
    ///         .agents
    ///         .realtime
    ///         .create_conversation_group(
    ///             &CreateRealtimeConversationGroupRequestModel {
    ///                 title: "title".to_string(),
    ///                 description: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_conversation_group(
        &self,
        request: &CreateRealtimeConversationGroupRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<RealtimeConversationGroupResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/realtime/conversation-groups",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a realtime conversation group by id.
    ///
    /// # Arguments
    ///
    /// * `conversation_group_id` - The id of a realtime conversation group.
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
    ///         .realtime
    ///         .get_conversation_group(&"conversation_group_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_conversation_group(
        &self,
        conversation_group_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RealtimeConversationGroupResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/realtime/conversation-groups/{}",
                    conversation_group_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a realtime conversation group. Every conversation in it becomes ungrouped, so access granted through the group is revoked.
    ///
    /// # Arguments
    ///
    /// * `conversation_group_id` - The id of a realtime conversation group.
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
    ///         .realtime
    ///         .delete_conversation_group(&"conversation_group_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_conversation_group(
        &self,
        conversation_group_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/convai/realtime/conversation-groups/{}",
                    conversation_group_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Rename or re-describe a realtime conversation group.
    ///
    /// # Arguments
    ///
    /// * `conversation_group_id` - The id of a realtime conversation group.
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
    ///         .realtime
    ///         .update_conversation_group(
    ///             &"conversation_group_id".to_string(),
    ///             &UpdateRealtimeConversationGroupRequestModel {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_conversation_group(
        &self,
        conversation_group_id: &str,
        request: &UpdateRealtimeConversationGroupRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<RealtimeConversationGroupResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/convai/realtime/conversation-groups/{}",
                    conversation_group_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Place a realtime conversation in a conversation group, moving it out of its current group if it has one.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The id of a realtime conversation.
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
    ///         .realtime
    ///         .set_conversation_group(
    ///             &"conversation_id".to_string(),
    ///             &SetConversationGroupRequestModel {
    ///                 conversation_group_id: "conversation_group_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set_conversation_group(
        &self,
        conversation_id: &str,
        request: &SetConversationGroupRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/convai/realtime/conversations/{}/conversation-group",
                    conversation_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove a realtime conversation from its conversation group.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The id of a realtime conversation.
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
    ///         .realtime
    ///         .clear_conversation_group(&"conversation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn clear_conversation_group(
        &self,
        conversation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/convai/realtime/conversations/{}/conversation-group",
                    conversation_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Create an ephemeral client secret for authenticating browser-based Realtime API sessions.
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
    ///         .agents
    ///         .realtime
    ///         .create_client_secret(
    ///             &Some(CreateClientSecretRequest {
    ///                 ..Default::default()
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_client_secret(
        &self,
        request: &Option<CreateClientSecretRequest>,
        options: Option<RequestOptions>,
    ) -> Result<ClientSecretResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/realtime/client_secrets",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
