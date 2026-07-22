use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ChunksClient {
    pub http_client: HttpClient,
}

impl ChunksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all RAG chunks for a specific knowledge base document.
    ///
    /// # Arguments
    ///
    /// * `documentation_id` - The id of a document from the knowledge base. This is returned on document addition.
    /// * `embedding_model` - The embedding model used to retrieve the chunk.
    /// * `page_size` - How many documents to return at maximum. Can not exceed 100, defaults to 30.
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        documentation_id: &str,
        request: &ConversationalAiKnowledgeBaseDocumentsChunksListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<KnowledgeBaseDocumentChunksResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/knowledge-base/{}/chunks", documentation_id),
                None,
                QueryBuilder::new()
                    .serialize("embedding_model", Some(request.embedding_model.clone()))
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
