pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_all
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAllQueryRequest {
    /// If set to true, legacy premade voices will be included in responses from /v1/voices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_legacy: Option<bool>,
}

impl GetAllQueryRequest {
    pub fn builder() -> GetAllQueryRequestBuilder {
        <GetAllQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAllQueryRequestBuilder {
    show_legacy: Option<bool>,
}

impl GetAllQueryRequestBuilder {
    pub fn show_legacy(mut self, value: bool) -> Self {
        self.show_legacy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAllQueryRequest`].
    pub fn build(self) -> Result<GetAllQueryRequest, BuildError> {
        Ok(GetAllQueryRequest {
            show_legacy: self.show_legacy,
        })
    }
}

