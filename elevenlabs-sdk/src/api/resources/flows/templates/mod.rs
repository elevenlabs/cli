use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod runs;
pub use runs::RunsClient;
pub struct TemplatesClient {
    pub http_client: HttpClient,
    pub runs: RunsClient,
}

impl TemplatesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            runs: RunsClient::new(config.clone())?,
        })
    }

    /// List the published flows templates in your workspace, together with each runnable version's inputs and outputs. Use the ids here as `template_id` / `version_id` on `POST /v1/flows/templates/{template_id}/runs`. Versions built on models that are not available to you through the API are left out, so `versions` is empty when none of a template's published versions is runnable through this API. Templates shared with you by link, or published to Explore from another workspace, are not listed but can still be fetched and run by `template_id`.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page.
    /// * `page_size` - How many templates to return per page. Lower than the run list's ceiling because each row expands its versions' input and output schemas.
    /// * `versions_per_template` - How many of each template's published versions to return, newest first. `has_more_versions` tells you when a template has more.
    /// * `search` - Only return templates whose name or description contains this text.
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
    ///         .list(
    ///             &FlowsTemplatesListQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 versions_per_template: Some(1),
    ///                 search: Some("search".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &FlowsTemplatesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TemplateListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/flows/templates",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .int(
                        "versions_per_template",
                        request.versions_per_template.clone(),
                    )
                    .string("search", request.search.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve one flows template, together with each runnable version's inputs and outputs. `versions` is empty when no published version is runnable through this API. Works for any template you can open, including templates shared with you by link or published to Explore from another workspace, which `GET /v1/flows/templates` does not list.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The ID of the template, as shown in the ElevenLabs app or by `GET /v1/flows/templates`.
    /// * `versions_per_template` - How many of each template's published versions to return, newest first. `has_more_versions` tells you when a template has more.
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
    ///         .get(
    ///             &"template_id".to_string(),
    ///             &FlowsTemplatesGetQueryRequest {
    ///                 versions_per_template: Some(1),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        template_id: &str,
        request: &FlowsTemplatesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TemplateSummary, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/flows/templates/{}", template_id),
                None,
                QueryBuilder::new()
                    .int(
                        "versions_per_template",
                        request.versions_per_template.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }
}
