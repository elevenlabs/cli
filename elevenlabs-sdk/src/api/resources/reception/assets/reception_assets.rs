use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct AssetsClient2 {
    pub http_client: HttpClient,
}

impl AssetsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the workspace's bookable assets, such as rooms or equipment.
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
    ///         .assets
    ///         .list(Some(
    ///             RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id"),
    ///         ))
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ReceptionAssetResource>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/reception/assets", None, None, options)
            .await
    }

    /// Create a bookable asset, such as a room or a piece of equipment.
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
    ///         .assets
    ///         .create(
    ///             &ReceptionCreateAssetRequest {
    ///                 name: StrippedStr("name".to_string()),
    ///                 groups: None,
    ///                 ungrouped_rank: None,
    ///                 description: None,
    ///                 is_active: None,
    ///                 available_hours: None,
    ///                 photo_url: None,
    ///                 location_id: None,
    ///                 customer_facing_name: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &ReceptionCreateAssetRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionAssetResource, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reception/assets",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get an asset by ID.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - ID of the asset. Returned when you create one; list them with `GET /v1/reception/assets`.
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
    ///         .assets
    ///         .get(
    ///             &"asset_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionAssetResource, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/reception/assets/{}", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an asset. Only the fields sent are changed.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - ID of the asset. Returned when you create one; list them with `GET /v1/reception/assets`.
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
    ///         .assets
    ///         .update(
    ///             &"asset_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionUpdateAssetRequestPatch {
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        asset_id: &str,
        request: &ReceptionUpdateAssetRequestPatch,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionAssetResource, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/reception/assets/{}", asset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an asset.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - ID of the asset. Returned when you create one; list them with `GET /v1/reception/assets`.
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
    ///         .assets
    ///         .delete(
    ///             &"asset_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/reception/assets/{}", asset_id),
                None,
                None,
                options,
            )
            .await
    }
}
