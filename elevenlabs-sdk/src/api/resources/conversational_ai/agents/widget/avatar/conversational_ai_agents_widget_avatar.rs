use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AvatarClient {
    pub http_client: HttpClient,
}

impl AvatarClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Sets the avatar for an agent displayed in the widget
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        agent_id: &str,
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostAgentAvatarResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("v1/convai/agents/{}/avatar", agent_id),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }
}
