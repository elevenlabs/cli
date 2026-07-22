use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub mod members;
pub use members::MembersClient2;
pub struct GroupsClient {
    pub http_client: HttpClient,
    pub members: MembersClient2,
}

impl GroupsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            members: MembersClient2::new(config.clone())?,
        })
    }

    /// Get all groups in the workspace
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
    ///     client.workspace.groups.list(None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, WorkspaceGroupResponseModel>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/workspace/groups", None, None, options)
            .await
    }

    /// Searches for user groups in the workspace. Multiple or no groups may be returned.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the target group.
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
    ///         .groups
    ///         .search(
    ///             &WorkspaceGroupsSearchQueryRequest {
    ///                 name: "name".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search(
        &self,
        request: &WorkspaceGroupsSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<WorkspaceGroupByNameResponseModel>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/workspace/groups/search",
                None,
                QueryBuilder::new()
                    .string("name", request.name.clone())
                    .build(),
                options,
            )
            .await
    }
}
