use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct MediaClient {
    pub http_client: HttpClient,
}

impl MediaClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Registers a media file with an order, either by uploading it directly or by providing a URL to fetch it from. Exactly one of `media` or `media_url` must be provided. The registered media can then be referenced when adding order items.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The ID of the order to which this media will be attached.
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
    ///         .media
    ///         .register(
    ///             &OrderID("order_id".to_string()),
    ///             &RegisterRequest {
    ///                 media: b"test file content".to_vec(),
    ///                 declared_language: "declared_language".to_string(),
    ///                 media_url: None,
    ///                 media_url_filename: None,
    ///                 media_url_content_type: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn register(
        &self,
        order_id: &OrderId,
        request: &RegisterRequest,
        options: Option<RequestOptions>,
    ) -> Result<RegisterMediaResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("v1/productions/orders/{}/media", order_id.0),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Retrieves metadata and a time-limited download URL for a previously uploaded media file.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The ID of the order.
    /// * `media_id` - The ID of the media file.
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
    ///         .media
    ///         .get(
    ///             &OrderID("order_id".to_string()),
    ///             &MediaID("media_id".to_string()),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        order_id: &OrderId,
        media_id: &MediaId,
        options: Option<RequestOptions>,
    ) -> Result<OrderMediaResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/productions/orders/{}/media/{}", order_id.0, media_id.0),
                None,
                None,
                options,
            )
            .await
    }
}
