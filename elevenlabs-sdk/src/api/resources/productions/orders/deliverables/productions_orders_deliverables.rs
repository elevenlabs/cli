use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DeliverablesClient {
    pub http_client: HttpClient,
}

impl DeliverablesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves the delivered files for a completed order. Returns an empty list if the order is not yet completed.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The ID of the order.
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
    ///         .productions
    ///         .orders
    ///         .deliverables
    ///         .list(&OrderID("order_id".to_string()), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        order_id: &OrderId,
        options: Option<RequestOptions>,
    ) -> Result<OrderDeliverablesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/productions/orders/{}/deliverables", order_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
