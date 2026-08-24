use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ResourcesClient {
    pub http_client: HttpClient,
}

impl ResourcesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Gets the metadata of a resource by ID.
    ///
    /// # Arguments
    ///
    /// * `resource_id` - The ID of the target resource.
    /// * `resource_type` - Resource type of the target resource.
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
    ///         .workspace
    ///         .resources
    ///         .get(
    ///             &"resource_id".to_string(),
    ///             &WorkspaceResourcesGetQueryRequest {
    ///                 resource_type: WorkspaceResourceType::Voice,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        resource_id: &str,
        request: &WorkspaceResourcesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ResourceMetadataResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/workspace/resources/{}", resource_id),
                None,
                QueryBuilder::new()
                    .serialize("resource_type", Some(request.resource_type.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Grants a role (one of 'admin', 'editor', 'commenter', or 'viewer') on a workspace resource to a user, group, or workspace (service account) API key. This overrides any existing role the target has on the resource. To target a user or service account, pass only the user email; the user must be in your workspace. To target a group, pass only the group id. To target a workspace (service account) API key, pass the api key id; the resource will be shared with the service account associated with that key. You must have admin access to the resource to share it.
    ///
    /// # Arguments
    ///
    /// * `resource_id` - The ID of the target resource.
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
    ///         .workspace
    ///         .resources
    ///         .share(
    ///             &"resource_id".to_string(),
    ///             &BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIDSharePost {
    ///                 role: BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIDSharePostRole::Admin,
    ///                 resource_type: WorkspaceResourceType::Voice,
    ///                 user_email: None,
    ///                 group_id: None,
    ///                 workspace_api_key_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn share(
        &self,
        resource_id: &str,
        request: &BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePost,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workspace/resources/{}/share", resource_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Removes any existing role on a workspace resource from a user, group, or workspace (service account) API key. To target a user or service account, pass only the user email; the user must be in your workspace. To target a group, pass only the group id. To target a workspace (service account) API key, pass the api key id; the resource will be unshared from the service account associated with that key. You must have admin access to the resource to unshare it. You cannot remove permissions from the user who created the resource.
    ///
    /// # Arguments
    ///
    /// * `resource_id` - The ID of the target resource.
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
    ///         .workspace
    ///         .resources
    ///         .unshare(
    ///             &"resource_id".to_string(),
    ///             &BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIDUnsharePost {
    ///                 resource_type: WorkspaceResourceType::Voice,
    ///                 user_email: None,
    ///                 group_id: None,
    ///                 workspace_api_key_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn unshare(
        &self,
        resource_id: &str,
        request: &BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIdUnsharePost,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workspace/resources/{}/unshare", resource_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
