use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ContentClient {
    pub http_client: HttpClient,
}

impl ContentClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Updates Studio project content.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        project_id: &str,
        request: &UpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<EditProjectResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("v1/studio/projects/{}/content", project_id),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }
}
