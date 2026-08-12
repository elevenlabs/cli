use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PhoneNumbersClient {
    pub http_client: HttpClient,
}

impl PhoneNumbersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve all Phone Numbers
    ///
    /// # Arguments
    ///
    /// * `provider` - Filter by telephony provider
    /// * `agent_id` - Filter by assigned agent ID
    /// * `branch_id` - Filter by assigned branch ID
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
    ///         .agents
    ///         .phone_numbers
    ///         .list(
    ///             &AgentsPhoneNumbersListQueryRequest {
    ///                 provider: Some(TelephonyProvider::Twilio),
    ///                 agent_id: Some("agent_id".to_string()),
    ///                 branch_id: Some("branch_id".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AgentsPhoneNumbersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<PhoneNumbersListResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/phone-numbers",
                None,
                QueryBuilder::new()
                    .serialize("provider", request.provider.clone())
                    .string("agent_id", request.agent_id.clone())
                    .string("branch_id", request.branch_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Import Phone Number from provider configuration (Twilio, Exotel, or SIP trunk)
    ///
    /// # Arguments
    ///
    /// * `request` - Create Phone Request Information
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
    ///         .agents
    ///         .phone_numbers
    ///         .create(
    ///             &PhoneNumbersCreateRequestBody::Twilio {
    ///                 data: CreateTwilioPhoneNumberRequest {
    ///                     phone_number: "phone_number".to_string(),
    ///                     label: "label".to_string(),
    ///                     sid: "sid".to_string(),
    ///                     token: "token".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &PhoneNumbersCreateRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<CreatePhoneNumberResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/phone-numbers",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve Phone Number details by ID
    ///
    /// # Arguments
    ///
    /// * `phone_number_id` - The phone number ID. This is returned when a phone number is imported.
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
    ///         .agents
    ///         .phone_numbers
    ///         .get(&"TeaqRRdTcIfIu2i7BYfT".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        phone_number_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PhoneNumbersGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/phone-numbers/{}", phone_number_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete Phone Number by ID
    ///
    /// # Arguments
    ///
    /// * `phone_number_id` - The phone number ID. This is returned when a phone number is imported.
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
    ///         .agents
    ///         .phone_numbers
    ///         .delete(&"TeaqRRdTcIfIu2i7BYfT".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        phone_number_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/phone-numbers/{}", phone_number_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update assigned agent of a phone number
    ///
    /// # Arguments
    ///
    /// * `phone_number_id` - The phone number ID. This is returned when a phone number is imported.
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
    ///         .agents
    ///         .phone_numbers
    ///         .update(
    ///             &"TeaqRRdTcIfIu2i7BYfT".to_string(),
    ///             &UpdatePhoneNumberRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        phone_number_id: &str,
        request: &UpdatePhoneNumberRequest,
        options: Option<RequestOptions>,
    ) -> Result<PhoneNumbersUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/phone-numbers/{}", phone_number_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get SIP messages for a phone number
    ///
    /// # Arguments
    ///
    /// * `phone_number_id` - The phone number ID. This is returned when a phone number is imported.
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
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
    ///         .agents
    ///         .phone_numbers
    ///         .get_sip_messages(
    ///             &"TeaqRRdTcIfIu2i7BYfT".to_string(),
    ///             &AgentsPhoneNumbersGetSipMessagesQueryRequest {
    ///                 page_size: Some(1),
    ///                 cursor: Some("cursor".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_sip_messages(
        &self,
        phone_number_id: &str,
        request: &AgentsPhoneNumbersGetSipMessagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSipLogMessagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/phone-numbers/{}/sip-messages", phone_number_id),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
