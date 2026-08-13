use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ImageClient {
    pub http_client: HttpClient,
}

impl ImageClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the image generations created through this API, newest first.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page.
    /// * `page_size` - How many generations to return per page.
    /// * `status` - Only return generations with this lifecycle status.
    /// * `model_id` - Only return generations of this model.
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
    ///         .flows
    ///         .image
    ///         .list(
    ///             &FlowsImageListQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 status: Some(ImageListRequestStatus::Pending),
    ///                 model_id: Some("model_id".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &FlowsImageListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MediaGenerationListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/flows/image",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("status", request.status.clone())
                    .string("model_id", request.model_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Start an image generation with the selected model.
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
    ///     client.flows.image.create(&ImageGenerationRequest::BytedanceSeedream5Lite {
    ///         data: BytedanceSeedream5LiteRequest {
    ///             prompt: "A corgi in a tiny lifeguard chair on a sunlit beach at golden hour, photorealistic".to_string(),
    ///             ..Default::default()
    ///         }
    ///     }, None).await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &ImageGenerationRequest,
        options: Option<RequestOptions>,
    ) -> Result<MediaGenerationCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/flows/image",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the status of an image generation, and retrieve its output URL once completed.
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
    ///         .flows
    ///         .image
    ///         .get(&"generation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        generation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MediaGenerationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/flows/image/{}", generation_id),
                None,
                None,
                options,
            )
            .await
    }
}
