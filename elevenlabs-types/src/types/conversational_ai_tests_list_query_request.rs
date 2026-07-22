pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiTestsListQueryRequest {
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many Tests to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Search query to filter tests by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Filter by parent folder ID. Use 'root' to get items in the root folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// If present, the endpoint will return only tests/folders of the given types.
    #[serde(default)]
    pub types: Vec<Option<TestType>>,
    /// Deprecated. Use the `types` query param and include `folder` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_folders: Option<bool>,
    /// Sort mode for listing tests. Use 'folders_first' to place folders before tests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_mode: Option<TestsListRequestSortMode>,
    /// Filter test visibility. Use `shared_with_me` to return only tests/folders shared with the current user that they did not create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_mode: Option<TestSharingMode>,
}

impl ConversationalAiTestsListQueryRequest {
    pub fn builder() -> ConversationalAiTestsListQueryRequestBuilder {
        <ConversationalAiTestsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiTestsListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    search: Option<String>,
    parent_folder_id: Option<String>,
    types: Option<Vec<Option<TestType>>>,
    include_folders: Option<bool>,
    sort_mode: Option<TestsListRequestSortMode>,
    sharing_mode: Option<TestSharingMode>,
}

impl ConversationalAiTestsListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    pub fn types(mut self, value: Vec<Option<TestType>>) -> Self {
        self.types = Some(value);
        self
    }

    pub fn include_folders(mut self, value: bool) -> Self {
        self.include_folders = Some(value);
        self
    }

    pub fn sort_mode(mut self, value: TestsListRequestSortMode) -> Self {
        self.sort_mode = Some(value);
        self
    }

    pub fn sharing_mode(mut self, value: TestSharingMode) -> Self {
        self.sharing_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiTestsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`types`](ConversationalAiTestsListQueryRequestBuilder::types)
    pub fn build(self) -> Result<ConversationalAiTestsListQueryRequest, BuildError> {
        Ok(ConversationalAiTestsListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            search: self.search,
            parent_folder_id: self.parent_folder_id,
            types: self.types.ok_or_else(|| BuildError::missing_field("types"))?,
            include_folders: self.include_folders,
            sort_mode: self.sort_mode,
            sharing_mode: self.sharing_mode,
        })
    }
}

