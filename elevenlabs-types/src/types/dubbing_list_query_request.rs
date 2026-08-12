pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DubbingListQueryRequest {
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many dubs to return at maximum. Can not exceed 200, defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// What state the dub is currently in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dubbing_status: Option<DubbingListRequestDubbingStatus>,
    /// Filter by dubbing status.
    #[serde(default)]
    pub dubbing_statuses: Vec<Option<DubbingListRequestDubbingStatusesItem>>,
    /// Filter by dubbing model generation.
    #[serde(default)]
    pub dubbing_models: Vec<Option<DubbingListRequestDubbingModelsItem>>,
    /// Filter by target language code.
    #[serde(default)]
    pub target_language_codes: Vec<Option<String>>,
    /// Filter by dubbing creation source.
    #[serde(default)]
    pub creation_sources: Vec<Option<DubbingListRequestCreationSourcesItem>>,
    /// Filters who created the resources being listed, whether it was the user running the request or someone else that shared the resource with them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_creator: Option<DubbingListRequestFilterByCreator>,
    /// The field to use for ordering results from this query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<DubbingListRequestOrderBy>,
    /// The order direction to use for results from this query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<DubbingListRequestOrderDirection>,
}

impl DubbingListQueryRequest {
    pub fn builder() -> DubbingListQueryRequestBuilder {
        <DubbingListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    dubbing_status: Option<DubbingListRequestDubbingStatus>,
    dubbing_statuses: Option<Vec<Option<DubbingListRequestDubbingStatusesItem>>>,
    dubbing_models: Option<Vec<Option<DubbingListRequestDubbingModelsItem>>>,
    target_language_codes: Option<Vec<Option<String>>>,
    creation_sources: Option<Vec<Option<DubbingListRequestCreationSourcesItem>>>,
    filter_by_creator: Option<DubbingListRequestFilterByCreator>,
    order_by: Option<DubbingListRequestOrderBy>,
    order_direction: Option<DubbingListRequestOrderDirection>,
}

impl DubbingListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn dubbing_status(mut self, value: DubbingListRequestDubbingStatus) -> Self {
        self.dubbing_status = Some(value);
        self
    }

    pub fn dubbing_statuses(mut self, value: Vec<Option<DubbingListRequestDubbingStatusesItem>>) -> Self {
        self.dubbing_statuses = Some(value);
        self
    }

    pub fn dubbing_models(mut self, value: Vec<Option<DubbingListRequestDubbingModelsItem>>) -> Self {
        self.dubbing_models = Some(value);
        self
    }

    pub fn target_language_codes(mut self, value: Vec<Option<String>>) -> Self {
        self.target_language_codes = Some(value);
        self
    }

    pub fn creation_sources(mut self, value: Vec<Option<DubbingListRequestCreationSourcesItem>>) -> Self {
        self.creation_sources = Some(value);
        self
    }

    pub fn filter_by_creator(mut self, value: DubbingListRequestFilterByCreator) -> Self {
        self.filter_by_creator = Some(value);
        self
    }

    pub fn order_by(mut self, value: DubbingListRequestOrderBy) -> Self {
        self.order_by = Some(value);
        self
    }

    pub fn order_direction(mut self, value: DubbingListRequestOrderDirection) -> Self {
        self.order_direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dubbing_statuses`](DubbingListQueryRequestBuilder::dubbing_statuses)
    /// - [`dubbing_models`](DubbingListQueryRequestBuilder::dubbing_models)
    /// - [`target_language_codes`](DubbingListQueryRequestBuilder::target_language_codes)
    /// - [`creation_sources`](DubbingListQueryRequestBuilder::creation_sources)
    pub fn build(self) -> Result<DubbingListQueryRequest, BuildError> {
        Ok(DubbingListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            dubbing_status: self.dubbing_status,
            dubbing_statuses: self.dubbing_statuses.ok_or_else(|| BuildError::missing_field("dubbing_statuses"))?,
            dubbing_models: self.dubbing_models.ok_or_else(|| BuildError::missing_field("dubbing_models"))?,
            target_language_codes: self.target_language_codes.ok_or_else(|| BuildError::missing_field("target_language_codes"))?,
            creation_sources: self.creation_sources.ok_or_else(|| BuildError::missing_field("creation_sources"))?,
            filter_by_creator: self.filter_by_creator,
            order_by: self.order_by,
            order_direction: self.order_direction,
        })
    }
}

