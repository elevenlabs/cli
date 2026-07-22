use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct SummariesClient2 {
    pub http_client: HttpClient,
}

impl SummariesClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Gets multiple knowledge base document summaries by their IDs.
    ///
    /// # Arguments
    ///
    /// * `document_ids` - The ids of knowledge base documents.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        request: &ConversationalAiKnowledgeBaseDocumentsSummariesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, SummariesGetResponseValue2>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/knowledge-base/summaries",
                None,
                QueryBuilder::new()
                    .string_array("document_ids", request.document_ids.clone())
                    .build(),
                options,
            )
            .await
    }
}
