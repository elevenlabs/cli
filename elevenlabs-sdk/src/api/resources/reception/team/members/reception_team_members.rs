use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct MembersClient {
    pub http_client: HttpClient,
}

impl MembersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the workspace's members, suspended members last.
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
    ///         .reception
    ///         .team
    ///         .members
    ///         .list(Some(
    ///             RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id"),
    ///         ))
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ReceptionTeamMemberResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/reception/team/members",
                None,
                None,
                options,
            )
            .await
    }

    /// Remove a member from the workspace. Their seat goes to a suspended member, if any.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID of the member to remove
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
    ///         .reception
    ///         .team
    ///         .members
    ///         .remove(
    ///             &"user_id".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn remove(
        &self,
        user_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/reception/team/members/{}", user_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Promote a member to admin or demote an admin to member.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID of the member to update
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
    ///         .reception
    ///         .team
    ///         .members
    ///         .update_role(
    ///             &"user_id".to_string(),
    ///             &BodySmbUpdateTeamMemberRole {
    ///                 role: BodySmbUpdateTeamMemberRoleRole::WorkspaceAdmin,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_role(
        &self,
        user_id: &str,
        request: &BodySmbUpdateTeamMemberRole,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/reception/team/members/{}", user_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
