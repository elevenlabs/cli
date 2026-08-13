use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List all webhooks for a workspace
    ///
    /// # Arguments
    ///
    /// * `include_usages` - Whether to include active usages of the webhook, only usable by admins
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
    ///         .webhooks
    ///         .list(
    ///             &WebhooksListQueryRequest {
    ///                 include_usages: Some(false),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &WebhooksListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceWebhookListResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/workspace/webhooks",
                None,
                QueryBuilder::new()
                    .bool("include_usages", request.include_usages.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new webhook for the workspace with the specified authentication type.
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
    ///         .webhooks
    ///         .create(
    ///             &BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPost {
    ///                 settings: WebhookHmacSettings {
    ///                     auth_type: "hmac".to_string(),
    ///                     name: "name".to_string(),
    ///                     webhook_url: "webhook_url".to_string(),
    ///                     request_headers: None,
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &BodyCreateWorkspaceWebhookV1WorkspaceWebhooksPost,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceCreateWebhookResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workspace/webhooks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete the specified workspace webhook
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - The unique ID for the webhook
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
    ///         .webhooks
    ///         .delete(&"G007vmtq9uWYl7SUW9zGS8GZZa1K".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        webhook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteWorkspaceWebhookResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/workspace/webhooks/{}", webhook_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the specified workspace webhook
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - The unique ID for the webhook
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
    ///         .webhooks
    ///         .update(
    ///             &"G007vmtq9uWYl7SUW9zGS8GZZa1K".to_string(),
    ///             &BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIDPatch {
    ///                 is_disabled: true,
    ///                 name: "My Callback Webhook".to_string(),
    ///                 retry_enabled: None,
    ///                 request_headers: None,
    ///                 events: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        webhook_id: &str,
        request: &BodyUpdateWorkspaceWebhookV1WorkspaceWebhooksWebhookIdPatch,
        options: Option<RequestOptions>,
    ) -> Result<PatchWorkspaceWebhookResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/workspace/webhooks/{}", webhook_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
