use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ItemsClient {
    pub http_client: HttpClient,
}

impl ItemsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Adds or updates an order item on an open order. Returns the item ID and the quoted price.
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
    ///     client.productions.orders.items.upsert(&OrderID("order_id".to_string()), &BodyUpsertOrderItemV1ProductionsOrdersOrderIDItemsPost {
    ///         request: UpsertOrderItemRequest {
    ///             item: OrderItemRequestInput::Dub {
    ///                 data: DubOrderItemRequest {
    ///                     media_id: MediaID("prodmedia_01jgatk6h0fwxrtbjade61yqhx".to_string()),
    ///                     source_language: "en".to_string(),
    ///                     destination_languages: vec!["hi".to_string(), "fr-FR".to_string(), "de".to_string()],
    ///                     include_captions: true,
    ///                     include_source_captions: false,
    ///                     instructions: Some("Voices don't need to match the originals, prioritize native-sounding voices".to_string()),
    ///                     captions_sdh: Some(false),
    ///                     ..Default::default()
    ///                 }
    ///             },
    ///             item_id: None
    ///         }
    ///     }, None).await;
    /// }
    /// ```
    pub async fn upsert(
        &self,
        order_id: &OrderId,
        request: &BodyUpsertOrderItemV1ProductionsOrdersOrderIdItemsPost,
        options: Option<RequestOptions>,
    ) -> Result<UpsertOrderItemResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/productions/orders/{}/items", order_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Removes an order item from an open order.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The ID of the order.
    /// * `item_id` - The ID of the order item.
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
    ///         .items
    ///         .remove(
    ///             &OrderID("order_id".to_string()),
    ///             &ItemID("item_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn remove(
        &self,
        order_id: &OrderId,
        item_id: &ItemId,
        options: Option<RequestOptions>,
    ) -> Result<RemoveOrderItemResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/productions/orders/{}/items/{}", order_id.0, item_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
