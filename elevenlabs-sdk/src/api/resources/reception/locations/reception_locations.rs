use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct LocationsClient {
    pub http_client: HttpClient,
}

impl LocationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List all business locations.
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
    ///         .locations
    ///         .list(Some(
    ///             RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id"),
    ///         ))
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ReceptionLocationResponse>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/reception/locations", None, None, options)
            .await
    }

    /// Add a new business location.
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
    ///         .locations
    ///         .create(
    ///             &ReceptionLocationRequest {
    ///                 name: StrippedStr("name".to_string()),
    ///                 address: None,
    ///                 notes: None,
    ///                 operating_hours: None,
    ///                 customer_facing_name: None,
    ///                 max_concurrent_appointments: None,
    ///                 is_active: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &ReceptionLocationRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionLocationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reception/locations",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a single location by ID.
    ///
    /// # Arguments
    ///
    /// * `location_id` - ID of the location. Returned when you create one; list them with `GET /v1/reception/locations`.
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
    ///         .locations
    ///         .get(
    ///             &"location_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        location_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionLocationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/reception/locations/{}", location_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing location. Only the fields sent are changed.
    ///
    /// # Arguments
    ///
    /// * `location_id` - ID of the location. Returned when you create one; list them with `GET /v1/reception/locations`.
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
    ///         .locations
    ///         .update(
    ///             &"location_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionLocationRequestPatch {
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        location_id: &str,
        request: &ReceptionLocationRequestPatch,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionLocationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/reception/locations/{}", location_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a location.
    ///
    /// # Arguments
    ///
    /// * `location_id` - ID of the location. Returned when you create one; list them with `GET /v1/reception/locations`.
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
    ///         .locations
    ///         .delete(
    ///             &"location_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        location_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/reception/locations/{}", location_id),
                None,
                None,
                options,
            )
            .await
    }
}
