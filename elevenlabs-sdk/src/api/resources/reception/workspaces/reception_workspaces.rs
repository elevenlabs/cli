use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WorkspacesClient {
    pub http_client: HttpClient,
}

impl WorkspacesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List workspaces attributed to the caller's reseller, with cursor-based pagination
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor from previous response
    /// * `page_size` - Number of workspaces per page
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
    ///         .workspaces
    ///         .list(
    ///             &ReceptionWorkspacesListQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ReceptionWorkspacesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ResellerWorkspacesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/reception/reseller/workspaces",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a workspace for one of your customers, optionally with a tier, a phone number and team invites.
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
    ///         .workspaces
    ///         .create(
    ///             &ProvisionResellerWorkspaceRequest {
    ///                 business_name: "business_name".to_string(),
    ///                 subscription: None,
    ///                 phone_number: None,
    ///                 members: None,
    ///                 complete_onboarding: None,
    ///                 idempotency_key: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &ProvisionResellerWorkspaceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProvisionResellerWorkspaceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reception/reseller/workspaces",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Finish setting up the workspace: create its receptionist and assistant from the business profile, reusing a receptionist from a setup already started, and connect your phone number, if you supply one. Safe to retry.
    ///
    /// # Arguments
    ///
    /// * `workspace_id` - ID of the customer workspace. Returned when you provision one; list them with `GET /v1/reception/reseller/workspaces`.
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
    ///         .workspaces
    ///         .complete_onboarding(&"3f2c9a7e1b4d4c8e9a6f0b2d5e7c1a93".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn complete_onboarding(
        &self,
        workspace_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ResellerWorkspaceOnboardingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/reception/reseller/workspaces/{}/complete-onboarding",
                    workspace_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
