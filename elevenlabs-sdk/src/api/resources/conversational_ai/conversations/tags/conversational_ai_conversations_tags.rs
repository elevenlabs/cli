use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TagsClient {
    pub http_client: HttpClient,
}

impl TagsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Assign one or more conversation tags to a conversation. Tags that are already assigned are ignored. Tags must belong to the same workspace.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn assign(
        &self,
        conversation_id: &str,
        request: &AssignConversationTagsRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/conversations/{}/tags", conversation_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove a single conversation tag from a conversation.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn unassign(
        &self,
        conversation_id: &str,
        tag_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/convai/conversations/{}/tags/{}",
                    conversation_id, tag_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// List conversation tags for the workspace, ordered by most recently created first.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many conversation tags to return. Can not exceed 100.
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ConversationalAiConversationsTagsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationTagsPageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/tags",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new conversation tag for the workspace.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreateConversationTagRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<ConversationTagResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/tags",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a conversation tag by ID.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        tag_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConversationTagResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/tags/{}", tag_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a conversation tag. Restricted to the tag owner or a workspace admin.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn delete(
        &self,
        tag_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/tags/{}", tag_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a conversation tag's title and/or description. Restricted to the tag owner or a workspace admin.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        tag_id: &str,
        request: &PatchConversationTagRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<ConversationTagResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/tags/{}", tag_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
