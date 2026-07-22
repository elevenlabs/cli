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
