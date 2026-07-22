use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DocumentClient {
    pub http_client: HttpClient,
}

impl DocumentClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Update the source file of a file document. The document name, content, and metadata are updated to reflect the new file. Any manual content edits will be overwritten.
    ///
    /// # Arguments
    ///
    /// * `documentation_id` - The id of a document from the knowledge base. This is returned on document addition.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_file(
        &self,
        documentation_id: &str,
        request: &UpdateFileRequest,
        options: Option<RequestOptions>,
    ) -> Result<DocumentUpdateFileResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::PATCH,
                &format!("v1/convai/knowledge-base/{}/update-file", documentation_id),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Manually refresh a URL document by re-fetching its content from the source URL.
    ///
    /// # Arguments
    ///
    /// * `documentation_id` - The id of a document from the knowledge base. This is returned on document addition.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn refresh(
        &self,
        documentation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DocumentRefreshResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/knowledge-base/{}/refresh", documentation_id),
                None,
                None,
                options,
            )
            .await
    }

    /// In case the document is not RAG indexed, it triggers rag indexing task, otherwise it just returns the current status.
    ///
    /// # Arguments
    ///
    /// * `documentation_id` - The id of a document from the knowledge base. This is returned on document addition.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn compute_rag_index(
        &self,
        documentation_id: &str,
        request: &RagIndexRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<RagDocumentIndexResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/knowledge-base/{}/rag-index", documentation_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
