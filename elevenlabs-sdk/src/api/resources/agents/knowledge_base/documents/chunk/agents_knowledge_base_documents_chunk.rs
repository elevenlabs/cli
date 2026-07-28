use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ChunkClient {
    pub http_client: HttpClient,
}

impl ChunkClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get details about a specific documentation part used by RAG.
    ///
    /// # Arguments
    ///
    /// * `documentation_id` - The id of a document from the knowledge base. This is returned on document addition.
    /// * `chunk_id` - The id of a document RAG chunk from the knowledge base.
    /// * `embedding_model` - The embedding model used to retrieve the chunk.
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
    ///         .agents
    ///         .knowledge_base
    ///         .documents
    ///         .chunk
    ///         .get(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &"chunk_id".to_string(),
    ///             &AgentsKnowledgeBaseDocumentsChunkGetQueryRequest {
    ///                 embedding_model: Some(EmbeddingModelEnum::E5Mistral7BInstruct),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        documentation_id: &str,
        chunk_id: &str,
        request: &AgentsKnowledgeBaseDocumentsChunkGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<KnowledgeBaseDocumentChunkResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/knowledge-base/{}/chunk/{}",
                    documentation_id, chunk_id
                ),
                None,
                QueryBuilder::new()
                    .serialize("embedding_model", request.embedding_model.clone())
                    .build(),
                options,
            )
            .await
    }
}
