use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RunsClient {
    pub http_client: HttpClient,
}

impl RunsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List this template's runs created through this API, newest first.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The ID of the template, as shown in the ElevenLabs app or by `GET /v1/flows/templates`.
    /// * `cursor` - Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page.
    /// * `page_size` - How many runs to return per page.
    /// * `version_id` - Only return runs of this template version id.
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
    ///         .templates
    ///         .runs
    ///         .list(
    ///             &"template_id".to_string(),
    ///             &FlowsTemplatesRunsListQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 version_id: Some("version_id".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        template_id: &str,
        request: &FlowsTemplatesRunsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TemplateRunListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/flows/templates/{}/runs", template_id),
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .string("version_id", request.version_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Start a run of a flows template. Pass `version_id` to pin a snapshot, or omit it / pass `latest` to run the latest published version. Set input values under `inputs`, keyed by input port id. The response is the run in its initial state, with every output already listed under its port id in `outputs`. Include `webhook` to receive a `flows_template_run` event carrying the finished run once its `status` is `completed` or `failed`; this is the recommended way to wait. Without one, fetch `GET /v1/flows/templates/{template_id}/runs/{run_id}` at a modest interval until the `status` is terminal.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The ID of the template, as shown in the ElevenLabs app or by `GET /v1/flows/templates`.
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
    ///         .templates
    ///         .runs
    ///         .create(
    ///             &"template_id".to_string(),
    ///             &TemplateRunCreateRequest {
    ///                 inputs: HashMap::from([
    ///                     (
    ///                         "prompt".to_string(),
    ///                         TemplateRunInput::String("a corgi on a surfboard".to_string()),
    ///                     ),
    ///                     (
    ///                         "reference".to_string(),
    ///                         TemplateRunInput::String(Default::default()),
    ///                     ),
    ///                 ]),
    ///                 version_id: Some("latest".to_string()),
    ///                 webhook: Some(WebhookTarget::all()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        template_id: &str,
        request: &TemplateRunCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<TemplateRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/flows/templates/{}/runs", template_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a template run: its `status`, rolled up from its outputs, and each output's own status and download URL once completed.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The ID of the template, as shown in the ElevenLabs app or by `GET /v1/flows/templates`.
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
    ///         .templates
    ///         .runs
    ///         .get(&"template_id".to_string(), &"run_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        template_id: &str,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TemplateRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/flows/templates/{}/runs/{}", template_id, run_id),
                None,
                None,
                options,
            )
            .await
    }
}
