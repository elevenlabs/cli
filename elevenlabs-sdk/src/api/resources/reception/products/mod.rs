use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub mod stock;
pub use stock::StockClient;
pub struct ProductsClient {
    pub http_client: HttpClient,
    pub stock: StockClient,
}

impl ProductsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            stock: StockClient::new(config.clone())?,
        })
    }

    /// List the workspace's products, optionally filtered by group or active state.
    ///
    /// # Arguments
    ///
    /// * `group` - Only products filed under the group with this label.
    /// * `is_active` - Only active (`true`) or inactive (`false`) products.
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
    ///         .list(
    ///             &ReceptionProductsListQueryRequest {
    ///                 group: Some("group".to_string()),
    ///                 is_active: Some(true),
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ReceptionProductsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ReceptionProduct>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/reception/products",
                None,
                QueryBuilder::new()
                    .string("group", request.group.clone())
                    .bool("is_active", request.is_active.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a product the receptionist can quote and take orders for. A product created with `track_quantity` starts with nothing on hand; add stock with the stock endpoints.
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
    ///         .products
    ///         .create(
    ///             &ReceptionCreateProductRequest {
    ///                 name: StrippedStr("name".to_string()),
    ///                 pricing_mode: None,
    ///                 intake_questions: None,
    ///                 groups: None,
    ///                 ungrouped_rank: None,
    ///                 description: None,
    ///                 price: None,
    ///                 is_active: None,
    ///                 orderable: None,
    ///                 allowed_fulfillment_types: None,
    ///                 location_ids: None,
    ///                 track_quantity: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &ReceptionCreateProductRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reception/products",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a product by ID.
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
    ///         .get(
    ///             &"product_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        product_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/reception/products/{}", product_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a product's catalog fields. Only the fields sent are changed. On-hand quantities are not part of this call; use the stock endpoints.
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
    ///         .update(
    ///             &"product_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             &ReceptionUpdateProductRequestPatch {
    ///                 ..Default::default()
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        product_id: &str,
        request: &ReceptionUpdateProductRequestPatch,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionProduct, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/reception/products/{}", product_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a product.
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
    ///         .delete(
    ///             &"product_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        product_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/reception/products/{}", product_id),
                None,
                None,
                options,
            )
            .await
    }
}
