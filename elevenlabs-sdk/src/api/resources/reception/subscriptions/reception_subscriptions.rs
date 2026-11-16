use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct SubscriptionsClient {
    pub http_client: HttpClient,
}

impl SubscriptionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Subscribe a reseller-attributed workspace to one of the reseller's tiers
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
    ///         .subscriptions
    ///         .assign(
    ///             &"3f2c9a7e1b4d4c8e9a6f0b2d5e7c1a93".to_string(),
    ///             &AssignResellerTierRequest {
    ///                 tier_id: "tier_id".to_string(),
    ///                 billing_period: None,
    ///                 idempotency_key: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn assign(
        &self,
        workspace_id: &str,
        request: &AssignResellerTierRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/reception/reseller/workspaces/{}/subscription",
                    workspace_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// End a reseller-attributed workspace's subscription at the end of the current billing period
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
    ///         .subscriptions
    ///         .unassign(&"3f2c9a7e1b4d4c8e9a6f0b2d5e7c1a93".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn unassign(
        &self,
        workspace_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/reception/reseller/workspaces/{}/subscription",
                    workspace_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Move a reseller-attributed workspace between the reseller's tiers. Upgrades apply immediately; downgrades and lateral moves take effect at the end of the billing period.
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
    ///         .subscriptions
    ///         .change(
    ///             &"3f2c9a7e1b4d4c8e9a6f0b2d5e7c1a93".to_string(),
    ///             &ChangeResellerTierRequest {
    ///                 tier_id: "tier_id".to_string(),
    ///                 billing_period: None,
    ///                 idempotency_key: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn change(
        &self,
        workspace_id: &str,
        request: &ChangeResellerTierRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/reception/reseller/workspaces/{}/subscription",
                    workspace_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Undo a pending unassignment so the subscription renews normally
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
    ///         .subscriptions
    ///         .restore(&"3f2c9a7e1b4d4c8e9a6f0b2d5e7c1a93".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn restore(
        &self,
        workspace_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/reception/reseller/workspaces/{}/subscription/restore",
                    workspace_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Drop a scheduled downgrade, leaving the workspace on its current tier
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
    ///         .subscriptions
    ///         .cancel_pending_downgrade(&"3f2c9a7e1b4d4c8e9a6f0b2d5e7c1a93".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_pending_downgrade(
        &self,
        workspace_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/reception/reseller/workspaces/{}/subscription/cancel-downgrade",
                    workspace_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
