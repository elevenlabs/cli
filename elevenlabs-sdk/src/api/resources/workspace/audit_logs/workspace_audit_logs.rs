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
