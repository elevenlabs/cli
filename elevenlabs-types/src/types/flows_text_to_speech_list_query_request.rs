pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FlowsTextToSpeechListQueryRequest {
    /// Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many generations to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Only return generations with this lifecycle status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TextToSpeechListRequestStatus>,
    /// Only return generations of this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
}

impl FlowsTextToSpeechListQueryRequest {
    pub fn builder() -> FlowsTextToSpeechListQueryRequestBuilder {
        <FlowsTextToSpeechListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlowsTextToSpeechListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    status: Option<TextToSpeechListRequestStatus>,
    model_id: Option<String>,
}

impl FlowsTextToSpeechListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn status(mut self, value: TextToSpeechListRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FlowsTextToSpeechListQueryRequest`].
    pub fn build(self) -> Result<FlowsTextToSpeechListQueryRequest, BuildError> {
        Ok(FlowsTextToSpeechListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            status: self.status,
            model_id: self.model_id,
        })
    }
}

