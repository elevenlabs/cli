pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsToolsListQueryRequest {
    /// If specified, the endpoint returns only tools whose names start with this string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// If set to true, the endpoint will return only tools owned by you (and not shared from somebody else). Deprecated: use created_by_user_id instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_only_owned_documents: Option<bool>,
    /// Filter tools by creator user ID. When set, only tools created by this user are returned. Takes precedence over show_only_owned_documents. Use '@me' to refer to the authenticated user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    /// If present, the endpoint will return only tools of the given types.
    #[serde(default)]
    pub types: Vec<Option<ToolTypeFilter>>,
    /// The direction to sort the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
    /// The field to sort the results by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<ToolSortBy>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsToolsListQueryRequest {
    pub fn builder() -> AgentsToolsListQueryRequestBuilder {
        <AgentsToolsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsToolsListQueryRequestBuilder {
    search: Option<String>,
    page_size: Option<i64>,
    show_only_owned_documents: Option<bool>,
    created_by_user_id: Option<String>,
    types: Option<Vec<Option<ToolTypeFilter>>>,
    sort_direction: Option<SortDirection>,
    sort_by: Option<ToolSortBy>,
    cursor: Option<String>,
}

impl AgentsToolsListQueryRequestBuilder {
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
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

    pub fn types(mut self, value: Vec<Option<ToolTypeFilter>>) -> Self {
        self.types = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: SortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn sort_by(mut self, value: ToolSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsToolsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`types`](AgentsToolsListQueryRequestBuilder::types)
    pub fn build(self) -> Result<AgentsToolsListQueryRequest, BuildError> {
        Ok(AgentsToolsListQueryRequest {
            search: self.search,
            page_size: self.page_size,
            show_only_owned_documents: self.show_only_owned_documents,
            created_by_user_id: self.created_by_user_id,
            types: self.types.ok_or_else(|| BuildError::missing_field("types"))?,
            sort_direction: self.sort_direction,
            sort_by: self.sort_by,
            cursor: self.cursor,
        })
    }
}

