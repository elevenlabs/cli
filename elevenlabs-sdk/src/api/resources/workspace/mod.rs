use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod audit_logs;
pub use audit_logs::AuditLogsClient;
pub mod auth_connections;
pub use auth_connections::AuthConnectionsClient;
pub mod groups;
pub use groups::GroupsClient;
pub mod invites;
pub use invites::InvitesClient;
pub mod members;
pub use members::MembersClient;
pub mod resources;
pub use resources::ResourcesClient;
pub mod usage;
pub use usage::UsageClient2;
pub mod analytics;
pub use analytics::AnalyticsClient2;
pub struct WorkspaceClient {
    pub http_client: HttpClient,
    pub audit_logs: AuditLogsClient,
    pub auth_connections: AuthConnectionsClient,
    pub groups: GroupsClient,
    pub invites: InvitesClient,
    pub members: MembersClient,
    pub resources: ResourcesClient,
    pub usage: UsageClient2,
    pub analytics: AnalyticsClient2,
}

impl WorkspaceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            audit_logs: AuditLogsClient::new(config.clone())?,
            auth_connections: AuthConnectionsClient::new(config.clone())?,
            groups: GroupsClient::new(config.clone())?,
            invites: InvitesClient::new(config.clone())?,
            members: MembersClient::new(config.clone())?,
            resources: ResourcesClient::new(config.clone())?,
            usage: UsageClient2::new(config.clone())?,
            analytics: AnalyticsClient2::new(config.clone())?,
        })
    }

    /// Set the workspace-wide Third-Party Disabling policy. When set, it forces, for every API key in the workspace, whether the holder of a key (potentially a third party who found it) may disable it via the self-disable endpoint or when it leaks publicly — overriding each key's own setting. Pass `true` to allow it for all keys, `false` to forbid it for all keys, or `null` to clear the override so per-key values and the plan default apply again. Workspace admins only.
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
    ///         .workspace
    ///         .set_third_party_disabling_policy(
    ///             &BodySetWorkspaceThirdPartyDisablingPolicyV1WorkspacesAPIKeysThirdPartyDisablingPost {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set_third_party_disabling_policy(
        &self,
        request: &BodySetWorkspaceThirdPartyDisablingPolicyV1WorkspacesApiKeysThirdPartyDisablingPost,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workspaces/api-keys/third-party-disabling",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
