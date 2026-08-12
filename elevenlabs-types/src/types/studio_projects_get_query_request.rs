pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StudioProjectsGetQueryRequest {
    /// The share ID of the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
}

impl StudioProjectsGetQueryRequest {
    pub fn builder() -> StudioProjectsGetQueryRequestBuilder {
        <StudioProjectsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StudioProjectsGetQueryRequestBuilder {
    share_id: Option<String>,
}

impl StudioProjectsGetQueryRequestBuilder {
    pub fn share_id(mut self, value: impl Into<String>) -> Self {
        self.share_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StudioProjectsGetQueryRequest`].
    pub fn build(self) -> Result<StudioProjectsGetQueryRequest, BuildError> {
        Ok(StudioProjectsGetQueryRequest {
            share_id: self.share_id,
        })
    }
}

