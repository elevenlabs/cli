use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WhatsappAccountsClient {
    pub http_client: HttpClient,
}

impl WhatsappAccountsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a WhatsApp account
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        phone_number_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetWhatsAppAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/whatsapp-accounts/{}", phone_number_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a WhatsApp account
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        phone_number_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/whatsapp-accounts/{}", phone_number_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a WhatsApp account
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        phone_number_id: &str,
        request: &UpdateWhatsAppAccountRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/whatsapp-accounts/{}", phone_number_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List all WhatsApp accounts
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Filter by assigned agent ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ConversationalAiWhatsappAccountsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWhatsAppAccountsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/whatsapp-accounts",
                None,
                QueryBuilder::new()
                    .string("agent_id", request.agent_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
