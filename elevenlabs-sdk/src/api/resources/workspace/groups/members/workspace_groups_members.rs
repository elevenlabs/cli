use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct MembersClient2 {
    pub http_client: HttpClient,
}

impl MembersClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Removes a member from the specified group. Requires `group_members_manage` permission.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the target group.
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
    ///         .members
    ///         .remove(
    ///             &"group_id".to_string(),
    ///             &BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIDMembersRemovePost {
    ///                 email: "email".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn remove(
        &self,
        group_id: &str,
        request: &BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIdMembersRemovePost,
        options: Option<RequestOptions>,
    ) -> Result<DeleteWorkspaceGroupMemberResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workspace/groups/{}/members/remove", group_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds a member of your workspace to the specified group. Requires `group_members_manage` permission.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the target group.
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
    ///         .members
    ///         .add(
    ///             &"group_id".to_string(),
    ///             &AddMemberToGroupRequest {
    ///                 email: "email".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add(
        &self,
        group_id: &str,
        request: &AddMemberToGroupRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddWorkspaceGroupMemberResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/workspace/groups/{}/members", group_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
