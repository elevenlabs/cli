pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FlowsImageListQueryRequest {
    /// Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many generations to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Only return generations with this lifecycle status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ImageListRequestStatus>,
    /// Only return generations of this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
}

impl FlowsImageListQueryRequest {
    pub fn builder() -> FlowsImageListQueryRequestBuilder {
        <FlowsImageListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlowsImageListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    status: Option<ImageListRequestStatus>,
    model_id: Option<String>,
}

impl FlowsImageListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn status(mut self, value: ImageListRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FlowsImageListQueryRequest`].
    pub fn build(self) -> Result<FlowsImageListQueryRequest, BuildError> {
        Ok(FlowsImageListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            status: self.status,
            model_id: self.model_id,
        })
    }
}

