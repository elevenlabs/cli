use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct HolidaysClient {
    pub http_client: HttpClient,
}

impl HolidaysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all holidays/time-off entries for the current workspace with optional filters
    ///
    /// # Arguments
    ///
    /// * `start_unix` - Filter entries overlapping from this timestamp
    /// * `end_unix` - Filter entries overlapping before this timestamp
    /// * `time_off_type` - Filter by type: location, staff, or asset
    /// * `resource_id` - Filter by associated resource ID (staff/asset)
    /// * `location_id` - Filter by location ID
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
    ///         .holidays
    ///         .list(
    ///             &ReceptionHolidaysListQueryRequest {
    ///                 start_unix: Some(1),
    ///                 end_unix: Some(1),
    ///                 time_off_type: Some(TimeOffType::Staff),
    ///                 resource_id: Some("resource_id".to_string()),
    ///                 location_id: Some("location_id".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ReceptionHolidaysListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<HolidayResponseModel>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/reception/holidays",
                None,
                QueryBuilder::new()
                    .int("start_unix", request.start_unix.clone())
                    .int("end_unix", request.end_unix.clone())
                    .serialize("time_off_type", request.time_off_type.clone())
                    .string("resource_id", request.resource_id.clone())
                    .string("location_id", request.location_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new holiday
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
    ///         .holidays
    ///         .create(
    ///             &ReceptionCreateHolidayRequest {
    ///                 name: "Christmas Eve".to_string(),
    ///                 start_date: DateModel {
    ///                     year: 2026,
    ///                     month: 12,
    ///                     day: 25,
    ///                     ..Default::default()
    ///                 },
    ///                 end_date: DateModel {
    ///                     year: 2026,
    ///                     month: 12,
    ///                     day: 25,
    ///                     ..Default::default()
    ///                 },
    ///                 time_off_type: TimeOffType::Staff,
    ///                 holiday_type: None,
    ///                 override_hours: None,
    ///                 resource_id: None,
    ///                 location_id: None,
    ///                 reason: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &ReceptionCreateHolidayRequest,
        options: Option<RequestOptions>,
    ) -> Result<HolidayResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reception/holidays",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a specific holiday by ID
    ///
    /// # Arguments
    ///
    /// * `holiday_id` - ID of the holiday or time-off entry. Returned when you create one; list them with `GET /v1/reception/holidays`.
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
    ///         .holidays
    ///         .get(
    ///             &"holiday_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        holiday_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HolidayResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/reception/holidays/{}", holiday_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an existing holiday. Only the fields sent are changed.
    ///
    /// # Arguments
    ///
    /// * `holiday_id` - ID of the holiday or time-off entry. Returned when you create one; list them with `GET /v1/reception/holidays`.
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
    ///         .holidays
    ///         .update(
    ///             &"holiday_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionUpdateHolidayRequestPatch {
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        holiday_id: &str,
        request: &ReceptionUpdateHolidayRequestPatch,
        options: Option<RequestOptions>,
    ) -> Result<HolidayResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/reception/holidays/{}", holiday_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a holiday
    ///
    /// # Arguments
    ///
    /// * `holiday_id` - ID of the holiday or time-off entry. Returned when you create one; list them with `GET /v1/reception/holidays`.
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
    ///         .holidays
    ///         .delete(
    ///             &"holiday_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        holiday_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/reception/holidays/{}", holiday_id),
                None,
                None,
                options,
            )
            .await
    }
}
