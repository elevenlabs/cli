use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ServicesClient {
    pub http_client: HttpClient,
}

impl ServicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all services for the current workspace with optional filters
    ///
    /// # Arguments
    ///
    /// * `staff_id` - Only services this staff member performs.
    /// * `asset_id` - Only services that book this asset.
    /// * `is_active` - Only active (`true`) or inactive (`false`) services.
    /// * `kind` - Only services of this kind.
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
    ///         .services
    ///         .list(
    ///             &ReceptionServicesListQueryRequest {
    ///                 staff_id: Some("staff_id".to_string()),
    ///                 asset_id: Some("asset_id".to_string()),
    ///                 is_active: Some(true),
    ///                 kind: Some(ReceptionServiceKind::Classic),
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ReceptionServicesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ReceptionService>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/reception/services",
                None,
                QueryBuilder::new()
                    .string("staff_id", request.staff_id.clone())
                    .string("asset_id", request.asset_id.clone())
                    .bool("is_active", request.is_active.clone())
                    .serialize("kind", request.kind.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new service
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
    ///         .services
    ///         .create(
    ///             &ReceptionCreateServiceRequest {
    ///                 name: StrippedStr("name".to_string()),
    ///                 pricing_mode: None,
    ///                 intake_questions: None,
    ///                 groups: None,
    ///                 ungrouped_rank: None,
    ///                 description: None,
    ///                 variants: None,
    ///                 kind: None,
    ///                 buffer_minutes: None,
    ///                 service_location: None,
    ///                 add_ons: None,
    ///                 max_participants: None,
    ///                 capacity: None,
    ///                 is_active: None,
    ///                 any_staff_member: None,
    ///                 staff_ids: None,
    ///                 asset_ids: None,
    ///                 advance_booking_minutes: None,
    ///                 location_ids: None,
    ///                 display_on_booking_page: None,
    ///                 travel_minutes: None,
    ///                 travel_before: None,
    ///                 travel_after: None,
    ///                 promotions: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &ReceptionCreateServiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionService, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reception/services",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a specific service by ID
    ///
    /// # Arguments
    ///
    /// * `service_id` - ID of the service. Returned when you create one; list them with `GET /v1/reception/services`.
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
    ///         .services
    ///         .get(
    ///             &"service_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        service_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionService, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/reception/services/{}", service_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing service. Only the fields sent are changed.
    ///
    /// # Arguments
    ///
    /// * `service_id` - ID of the service. Returned when you create one; list them with `GET /v1/reception/services`.
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
    ///         .services
    ///         .update(
    ///             &"service_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionUpdateServiceRequestPatch {
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        service_id: &str,
        request: &ReceptionUpdateServiceRequestPatch,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionServiceUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/reception/services/{}", service_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a service
    ///
    /// # Arguments
    ///
    /// * `service_id` - ID of the service. Returned when you create one; list them with `GET /v1/reception/services`.
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
    ///         .services
    ///         .delete(
    ///             &"service_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        service_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/reception/services/{}", service_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Save a service update that changes the price of existing bookings. Optionally reprice future bookings.
    ///
    /// # Arguments
    ///
    /// * `service_id` - ID of the service. Returned when you create one; list them with `GET /v1/reception/services`.
    /// * `reprice` - Also update the price on the service's future bookings.
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
    ///         .services
    ///         .confirm_update(
    ///             &"service_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionConfirmServiceUpdateRequest {
    ///                 reprice: Some(true),
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn confirm_update(
        &self,
        service_id: &str,
        request: &ReceptionConfirmServiceUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionServiceUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/reception/services/{}/confirm-update", service_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool("reprice", request.reprice.clone())
                    .build(),
                options,
            )
            .await
    }
}
