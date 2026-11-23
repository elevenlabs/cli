use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct StockClient {
    pub http_client: HttpClient,
}

impl StockClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Set the on-hand quantity of a counted product at one location. Rejected with a 409 if it changed since `expected_on_hand` was read.
    ///
    /// # Arguments
    ///
    /// * `product_id` - ID of the product. Returned when you create one; list them with `GET /v1/reception/products`.
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
    ///         .products
    ///         .stock
    ///         .set(
    ///             &"product_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionUpdateProductStockRequest {
    ///                 location_id: "location_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///                 on_hand: 24,
    ///                 expected_on_hand: 25,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set(
        &self,
        product_id: &str,
        request: &ReceptionUpdateProductStockRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/reception/products/{}/stock", product_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Add to or subtract from the on-hand quantity of a counted product at one location, without overwriting orders taken at the same time.
    ///
    /// # Arguments
    ///
    /// * `product_id` - ID of the product. Returned when you create one; list them with `GET /v1/reception/products`.
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
    ///         .products
    ///         .stock
    ///         .adjust(
    ///             &"product_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionAdjustProductStockRequest {
    ///                 location_id: "location_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///                 delta: -2,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn adjust(
        &self,
        product_id: &str,
        request: &ReceptionAdjustProductStockRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/reception/products/{}/stock/adjust", product_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
