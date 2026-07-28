pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiKnowledgeBaseListQueryRequest {
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// If specified, the endpoint returns only such knowledge base documents whose names start with this string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// If set to true, the endpoint will return only documents owned by you (and not shared from somebody else). Deprecated: use created_by_user_id instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_only_owned_documents: Option<bool>,
    /// Filter documents by creator user ID. When set, only documents created by this user are returned. Takes precedence over show_only_owned_documents. Use '@me' to refer to the authenticated user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    /// If present, the endpoint will return only documents of the given types.
    #[serde(default)]
    pub types: Vec<Option<KnowledgeBaseDocumentType>>,
    /// If set, the endpoint will return only documents that are direct children of the given folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// If set, the endpoint will return only documents that are descendants of the given folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestor_folder_id: Option<String>,
    /// Whether folders should be returned first in the list of documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders_first: Option<bool>,
    /// The direction to sort the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
    /// The field to sort the results by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<KnowledgeBaseSortBy>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ConversationalAiKnowledgeBaseListQueryRequest {
    pub fn builder() -> ConversationalAiKnowledgeBaseListQueryRequestBuilder {
        <ConversationalAiKnowledgeBaseListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiKnowledgeBaseListQueryRequestBuilder {
    page_size: Option<i64>,
    search: Option<String>,
    show_only_owned_documents: Option<bool>,
    created_by_user_id: Option<String>,
    types: Option<Vec<Option<KnowledgeBaseDocumentType>>>,
    parent_folder_id: Option<String>,
    ancestor_folder_id: Option<String>,
    folders_first: Option<bool>,
    sort_direction: Option<SortDirection>,
    sort_by: Option<KnowledgeBaseSortBy>,
    cursor: Option<String>,
}

impl ConversationalAiKnowledgeBaseListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn show_only_owned_documents(mut self, value: bool) -> Self {
        self.show_only_owned_documents = Some(value);
        self
    }

    pub fn created_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_user_id = Some(value.into());
        self
    }

    pub fn types(mut self, value: Vec<Option<KnowledgeBaseDocumentType>>) -> Self {
        self.types = Some(value);
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn ancestor_folder_id(mut self, value: impl Into<String>) -> Self {
        self.ancestor_folder_id = Some(value.into());
        self
    }

    pub fn folders_first(mut self, value: bool) -> Self {
        self.folders_first = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: SortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn sort_by(mut self, value: KnowledgeBaseSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiKnowledgeBaseListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`types`](ConversationalAiKnowledgeBaseListQueryRequestBuilder::types)
    pub fn build(self) -> Result<ConversationalAiKnowledgeBaseListQueryRequest, BuildError> {
        Ok(ConversationalAiKnowledgeBaseListQueryRequest {
            page_size: self.page_size,
            search: self.search,
            show_only_owned_documents: self.show_only_owned_documents,
            created_by_user_id: self.created_by_user_id,
            types: self.types.ok_or_else(|| BuildError::missing_field("types"))?,
            parent_folder_id: self.parent_folder_id,
            ancestor_folder_id: self.ancestor_folder_id,
            folders_first: self.folders_first,
            sort_direction: self.sort_direction,
            sort_by: self.sort_by,
            cursor: self.cursor,
        })
    }
}

