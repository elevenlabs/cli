use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct KnowledgeSourcesClient {
    pub http_client: HttpClient,
}

impl KnowledgeSourcesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List all knowledge sources for the current workspace
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
    ///         .reception
    ///         .knowledge_sources
    ///         .list(Some(
    ///             RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id"),
    ///         ))
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ReceptionKnowledgeSource>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/reception/knowledge-sources",
                None,
                None,
                options,
            )
            .await
    }

    /// Upload a file as a knowledge source
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
    ///         .reception
    ///         .knowledge_sources
    ///         .upload(
    ///             &UploadRequest2 {
    ///                 file: b"test file content".to_vec(),
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upload(
        &self,
        request: &UploadRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionKnowledgeSource, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/reception/knowledge-sources",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Get a specific knowledge source by ID
    ///
    /// # Arguments
    ///
    /// * `source_id` - ID of the knowledge source. Returned when you add one; list them with `GET /v1/reception/knowledge-sources`.
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
    ///         .reception
    ///         .knowledge_sources
    ///         .get(
    ///             &"ksource_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        source_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionKnowledgeSource, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/reception/knowledge-sources/{}", source_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a knowledge source
    ///
    /// # Arguments
    ///
    /// * `source_id` - ID of the knowledge source. Returned when you add one; list them with `GET /v1/reception/knowledge-sources`.
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
    ///         .reception
    ///         .knowledge_sources
    ///         .delete(
    ///             &"ksource_4801jx8k3m5n7p9q2r4s6t8v0w2y".to_string(),
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        source_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/reception/knowledge-sources/{}", source_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Add a URL as a knowledge source
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
    ///         .reception
    ///         .knowledge_sources
    ///         .add_url(
    ///             &AddUrlRequest {
    ///                 url: "https://example.com".to_string(),
    ///                 crawl_entire_site: None,
    ///             },
    ///             Some(RequestOptions::new().additional_header("xi-workspace-id", "xi-workspace-id")),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_url(
        &self,
        request: &AddUrlRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionKnowledgeSource, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reception/knowledge-sources/urls",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
