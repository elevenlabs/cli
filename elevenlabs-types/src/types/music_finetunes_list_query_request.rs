pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MusicFinetunesListQueryRequest {
    /// Used for fetching the next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many finetunes to return. Max 100, default 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Filter by visibility. 'private' returns private finetunes; 'workspace' returns workspace-shared finetunes; 'public' returns public finetunes, which are currently ElevenLabs curated finetunes. Omit to return all accessible finetunes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<FinetuneVisibility>,
    /// Filter by creator. 'self' returns finetunes you created; 'workspace' returns finetunes created by workspace teammates; 'elevenlabs' returns ElevenLabs curated finetunes. Omit to return finetunes from all creators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<FinetuneCreatedBy>,
    /// Sort by field (created_at or name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<FinetunesListRequestSort>,
    /// Sort direction (asc or desc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<FinetunesListRequestSortDirection>,
}

impl MusicFinetunesListQueryRequest {
    pub fn builder() -> MusicFinetunesListQueryRequestBuilder {
        <MusicFinetunesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MusicFinetunesListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    visibility: Option<FinetuneVisibility>,
    created_by: Option<FinetuneCreatedBy>,
    sort: Option<FinetunesListRequestSort>,
    sort_direction: Option<FinetunesListRequestSortDirection>,
}

impl MusicFinetunesListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn visibility(mut self, value: FinetuneVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn created_by(mut self, value: FinetuneCreatedBy) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn sort(mut self, value: FinetunesListRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: FinetunesListRequestSortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MusicFinetunesListQueryRequest`].
    pub fn build(self) -> Result<MusicFinetunesListQueryRequest, BuildError> {
        Ok(MusicFinetunesListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            visibility: self.visibility,
            created_by: self.created_by,
            sort: self.sort,
            sort_direction: self.sort_direction,
        })
    }
}

