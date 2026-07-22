use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LanguagesClient {
    pub http_client: HttpClient,
}

impl LanguagesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the available languages for a given order item kind.
    ///
    /// # Arguments
    ///
    /// * `order_item_kind` - The kind of order item.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        order_item_kind: &OrderItemKind,
        options: Option<RequestOptions>,
    ) -> Result<LanguagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/productions/orders/languages/{}", order_item_kind),
                None,
                None,
                options,
            )
            .await
    }
}
