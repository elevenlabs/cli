use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AuditLogsClient {
    pub http_client: HttpClient,
}

impl AuditLogsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the audit log for the workspace. Requires enterprise tier and the audit_log_read permission.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of entries per page
    /// * `cursor` - Cursor for the next page (from previous response)
    /// * `time_from_unix_ms` - Only include entries at or after this time (ms since epoch)
    /// * `time_to_unix_ms` - Only include entries at or before this time (ms since epoch)
    /// * `actor_uid` - Filter by actor user ID
    /// * `class_name` - Filter by OCSF event class name (e.g. Account Change)
    /// * `activity_name` - Filter by audit activity name (e.g. Subscription Creation)
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
    ///         .audit_logs
    ///         .list(
    ///             &WorkspaceAuditLogsListQueryRequest {
    ///                 limit: Some(1),
    ///                 cursor: Some("cursor".to_string()),
    ///                 time_from_unix_ms: Some(1),
    ///                 time_to_unix_ms: Some(1),
    ///                 actor_uid: Some("actor_uid".to_string()),
    ///                 class_name: Some("class_name".to_string()),
    ///                 activity_name: Some("activity_name".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &WorkspaceAuditLogsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceAuditLogsPageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/workspace/audit-logs",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .int("time_from_unix_ms", request.time_from_unix_ms.clone())
                    .int("time_to_unix_ms", request.time_to_unix_ms.clone())
                    .string("actor_uid", request.actor_uid.clone())
                    .string("class_name", request.class_name.clone())
                    .string("activity_name", request.activity_name.clone())
                    .build(),
                options,
            )
            .await
    }
}
