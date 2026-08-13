use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct VideoClient {
    pub http_client: HttpClient,
}

impl VideoClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the video generations created through this API, newest first.
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
    ///         .video
    ///         .list(
    ///             &FlowsVideoListQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 status: Some(VideoListRequestStatus::Pending),
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
        request: &FlowsVideoListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MediaGenerationListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/flows/video",
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

    /// Start a video generation with the selected model.
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
    ///     client.flows.video.create(&VideoGenerationRequest::BytedanceSeedanceV2 {
    ///         data: BytedanceSeedance2Request {
    ///             prompt: "A corgi rides a tiny surfboard across a sunlit wave at golden hour, cinematic".to_string(),
    ///             ..Default::default()
    ///         }
    ///     }, None).await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &VideoGenerationRequest,
        options: Option<RequestOptions>,
    ) -> Result<MediaGenerationCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/flows/video",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve the status of a video generation, and retrieve its output URL once completed.
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
    ///         .video
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
                &format!("v1/flows/video/{}", generation_id),
                None,
                None,
                options,
            )
            .await
    }
}
