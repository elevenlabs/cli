use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct StaffClient {
    pub http_client: HttpClient,
}

impl StaffClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all staff for the current workspace
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
    ///         .staff
    ///         .list(Some(
    ///             RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id"),
    ///         ))
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ReceptionStaffResource>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/reception/staff", None, None, options)
            .await
    }

    /// Create a new staff member
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
    ///         .staff
    ///         .create(
    ///             &ReceptionCreateStaffRequest {
    ///                 name: StrippedStr("name".to_string()),
    ///                 groups: None,
    ///                 ungrouped_rank: None,
    ///                 description: None,
    ///                 is_active: None,
    ///                 email: None,
    ///                 phone: None,
    ///                 sms_notifications_opted_in: None,
    ///                 photo_url: None,
    ///                 location_id: None,
    ///                 working_hours: None,
    ///                 display_on_booking_page: None,
    ///                 customer_facing_name: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &ReceptionCreateStaffRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionStaffResource, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reception/staff",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a specific staff member by ID
    ///
    /// # Arguments
    ///
    /// * `staff_id` - ID of the staff member. Returned when you create one; list them with `GET /v1/reception/staff`.
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
    ///         .staff
    ///         .get(
    ///             &"staff_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        staff_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionStaffResource, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/reception/staff/{}", staff_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing staff member. Only the fields sent are changed.
    ///
    /// # Arguments
    ///
    /// * `staff_id` - ID of the staff member. Returned when you create one; list them with `GET /v1/reception/staff`.
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
    ///         .staff
    ///         .update(
    ///             &"staff_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionUpdateStaffRequestPatch {
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        staff_id: &str,
        request: &ReceptionUpdateStaffRequestPatch,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionStaffResource, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/reception/staff/{}", staff_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a staff member
    ///
    /// # Arguments
    ///
    /// * `staff_id` - ID of the staff member. Returned when you create one; list them with `GET /v1/reception/staff`.
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
    ///         .staff
    ///         .delete(
    ///             &"staff_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        staff_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/reception/staff/{}", staff_id),
                None,
                None,
                options,
            )
            .await
    }
}
