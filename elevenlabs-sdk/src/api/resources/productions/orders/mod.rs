use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod media;
pub use media::MediaClient;
pub mod items;
pub use items::ItemsClient;
pub mod deliverables;
pub use deliverables::DeliverablesClient;
pub mod languages;
pub use languages::LanguagesClient;
pub struct OrdersClient {
    pub http_client: HttpClient,
    pub media: MediaClient,
    pub items: ItemsClient,
    pub deliverables: DeliverablesClient,
    pub languages: LanguagesClient,
}

impl OrdersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            media: MediaClient::new(config.clone())?,
            items: ItemsClient::new(config.clone())?,
            deliverables: DeliverablesClient::new(config.clone())?,
            languages: LanguagesClient::new(config.clone())?,
        })
    }

    /// Lists Productions orders in the workspace. Supports filtering by status and date range, with pagination.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Maximum number of orders to return per page.
    /// * `offset` - Number of orders to skip for pagination.
    /// * `status` - Filter orders by one or more statuses.
    /// * `start_date` - Filter orders created on or after this date.
    /// * `end_date` - Filter orders created on or before this date.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ProductionsOrdersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOrdersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/productions/orders",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("offset", request.offset.clone())
                    .serialize_array("status", request.status.clone())
                    .datetime("start_date", request.start_date.clone())
                    .datetime("end_date", request.end_date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new Productions order in the workspace. The order starts in the open state and can be configured with items before submission.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &Option<CreateOrderRequest>,
        options: Option<RequestOptions>,
    ) -> Result<CreateOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/productions/orders",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves full details for a Productions order.
    ///
    /// Quote and pricing information may not be available immediately; if you wish to see the quote before submission, you may need to poll the order details until it is ready.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The ID of the order.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        order_id: &OrderId,
        options: Option<RequestOptions>,
    ) -> Result<OrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/productions/orders/{}", order_id.0),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an open order.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The ID of the order.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        order_id: &OrderId,
        request: &BodyUpdateOrderV1ProductionsOrdersOrderIdPatch,
        options: Option<RequestOptions>,
    ) -> Result<UpdateOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/productions/orders/{}", order_id.0),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Submits an open order for processing. The order must have at least one item. Once submitted, items can no longer be modified.
    ///
    /// Upon submission, the workspace will be charged for the order. The quote is based on information extracted from the uploaded media, such as its duration. The quote may not be available immediately; if you wish to see the quote before submission, you may need to poll the order details until the quote is ready.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The ID of the order.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn submit(
        &self,
        order_id: &OrderId,
        options: Option<RequestOptions>,
    ) -> Result<SubmitOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/productions/orders/{}/submit", order_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
