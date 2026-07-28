use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct FilesClient {
    pub http_client: HttpClient,
}

impl FilesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Upload an image or PDF file for a conversation. Returns a unique file ID that can be used to reference the file in the conversation.
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
    ///         .conversational_ai
    ///         .conversations
    ///         .files
    ///         .create(
    ///             &"conversation_id".to_string(),
    ///             &CreateRequest {
    ///                 file: b"test file content".to_vec(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        conversation_id: &str,
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConvAiFileUploadResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("v1/convai/conversations/{}/files", conversation_id),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Remove a file upload from a conversation. Only possible if the file hasn't already been used in the conversation.
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
    ///         .conversational_ai
    ///         .conversations
    ///         .files
    ///         .delete(&"conversation_id".to_string(), &"file_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        conversation_id: &str,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConvAiFileUploadResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/convai/conversations/{}/files/{}",
                    conversation_id, file_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
