use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub mod documents;
pub use documents::DocumentsClient;
pub mod document;
pub use document::DocumentClient;
pub struct KnowledgeBaseClient {
    pub http_client: HttpClient,
    pub documents: DocumentsClient,
    pub document: DocumentClient,
}

impl KnowledgeBaseClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            documents: DocumentsClient::new(config.clone())?,
            document: DocumentClient::new(config.clone())?,
        })
    }

    /// Get a list of available knowledge base documents
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many documents to return at maximum. Can not exceed 100, defaults to 30.
    /// * `search` - If specified, the endpoint returns only such knowledge base documents whose names start with this string.
    /// * `show_only_owned_documents` - If set to true, the endpoint will return only documents owned by you (and not shared from somebody else). Deprecated: use created_by_user_id instead.
    /// * `created_by_user_id` - Filter documents by creator user ID. When set, only documents created by this user are returned. Takes precedence over show_only_owned_documents. Use '@me' to refer to the authenticated user.
    /// * `types` - If present, the endpoint will return only documents of the given types.
    /// * `parent_folder_id` - If set, the endpoint will return only documents that are direct children of the given folder.
    /// * `ancestor_folder_id` - If set, the endpoint will return only documents that are descendants of the given folder.
    /// * `folders_first` - Whether folders should be returned first in the list of documents.
    /// * `sort_direction` - The direction to sort the results
    /// * `sort_by` - The field to sort the results by
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ConversationalAiKnowledgeBaseListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetKnowledgeBaseListResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/knowledge-base",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("search", request.search.clone())
                    .bool(
                        "show_only_owned_documents",
                        request.show_only_owned_documents.clone(),
                    )
                    .string("created_by_user_id", request.created_by_user_id.clone())
                    .serialize_array("types", request.types.clone())
                    .string("parent_folder_id", request.parent_folder_id.clone())
                    .string("ancestor_folder_id", request.ancestor_folder_id.clone())
                    .bool("folders_first", request.folders_first.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves and/or creates RAG indexes for multiple knowledge base documents in a single request. Maximum 100 items per request.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_or_create_rag_indexes(
        &self,
        request: &BodyComputeRagIndexesInBatchV1ConvaiKnowledgeBaseRagIndexPost,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, KnowledgeBaseGetOrCreateRagIndexesResponseValue>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/knowledge-base/rag-index",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fuzzy text search over knowledge base document content
    ///
    /// # Arguments
    ///
    /// * `query` - The search query text
    /// * `page_size` - How many documents to return at maximum. Can not exceed 100, defaults to 30.
    /// * `types` - If present, the endpoint will return only documents of the given types.
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn search(
        &self,
        request: &ConversationalAiKnowledgeBaseSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<KnowledgeBaseContentSearchResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/knowledge-base/search",
                None,
                QueryBuilder::new()
                    .structured_query("query", request.query.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize_array("types", request.types.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
