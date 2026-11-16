use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PhoneNumbersClient2 {
    pub http_client: HttpClient,
}

impl PhoneNumbersClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the workspace's phone numbers and the receptionist each one rings. A number with no `customer_facing_agent_id` is held by the workspace but not attached to a receptionist yet.
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
    ///         .phone_numbers
    ///         .list(Some(
    ///             RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id"),
    ///         ))
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ReceptionPhoneAssignment>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/reception/phone-numbers/assignments",
                None,
                None,
                options,
            )
            .await
    }

    /// Give a receptionist a phone number. A number the workspace already holds but has not attached is used first. Otherwise, in the US and Canada, a number with one of the requested `area_codes` is taken from the pool or bought, falling back to any number in the business's country. In countries that require paperwork, the number is issued right away but takes public calls only once the workspace's own registration is approved (see `accepts_public_calls`). Counts toward the plan's phone number limit, and free plans can take only a few new numbers a day. Returns 404 when no number is available and 403 `telephony_managed_by_reseller` when the reseller supplies the workspace's numbers from its own Twilio account or SIP trunk and none is free.
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
    ///         .phone_numbers
    ///         .create(
    ///             &CreatePhoneAssignmentRequest {
    ///                 customer_facing_agent_id: "agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///                 area_codes: None,
    ///                 number_type: None,
    ///                 idempotency_key: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePhoneAssignmentRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionPhoneAssignment, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reception/phone-numbers/assignments",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove a phone number from the workspace. A pool number goes back to the pool it came from. A number the business brought from its own Twilio account, or one issued in a country that requires paperwork, is released at Twilio and cannot be recovered. Returns 403 `telephony_managed_by_reseller` when the reseller supplies the workspace's numbers from its own Twilio account or SIP trunk.
    ///
    /// # Arguments
    ///
    /// * `phone_assignment_id` - ID of the phone number assignment. Returned when you assign a number; list them with `GET /v1/reception/phone-numbers/assignments`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .phone_numbers
    ///         .delete(
    ///             &"phone_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        phone_assignment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/reception/phone-numbers/assignments/{}",
                    phone_assignment_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Move a phone number to a different receptionist in the same workspace, which answers its calls from then on. Returns 409 `phone_number_not_attached` for a number no receptionist answers yet (attach it with `POST /v1/reception/phone-numbers/assignments`).
    ///
    /// # Arguments
    ///
    /// * `phone_assignment_id` - ID of the phone number assignment. Returned when you assign a number; list them with `GET /v1/reception/phone-numbers/assignments`.
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
    ///         .phone_numbers
    ///         .update(
    ///             &"phone_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReassignPhoneRequest {
    ///                 customer_facing_agent_id: "agent_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        phone_assignment_id: &str,
        request: &ReassignPhoneRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionPhoneAssignment, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/reception/phone-numbers/assignments/{}",
                    phone_assignment_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
