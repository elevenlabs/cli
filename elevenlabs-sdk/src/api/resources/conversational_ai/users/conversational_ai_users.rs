use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct UsersClient {
    pub http_client: HttpClient,
}

impl UsersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get distinct users from conversations with pagination.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    /// * `branch_id` - Filter conversations by branch ID.
    /// * `call_start_before_unix` - Unix timestamp (in seconds) to filter conversations up to this start date.
    /// * `call_start_after_unix` - Unix timestamp (in seconds) to filter conversations after to this start date.
    /// * `search` - Search/filter by user ID (exact match).
    /// * `page_size` - How many users to return at maximum. Defaults to 30.
    /// * `sort_by` - The field to sort the results by. Defaults to last_contact_unix_secs.
    /// * `sort_direction` - The direction to sort the results
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ConversationalAiUsersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationUsersPageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/users",
                None,
                QueryBuilder::new()
                    .string("agent_id", request.agent_id.clone())
                    .string("branch_id", request.branch_id.clone())
                    .int(
                        "call_start_before_unix",
                        request.call_start_before_unix.clone(),
                    )
                    .int(
                        "call_start_after_unix",
                        request.call_start_after_unix.clone(),
                    )
                    .string("search", request.search.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
