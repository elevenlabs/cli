pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FlowsTemplatesGetQueryRequest {
    /// How many of each template's published versions to return, newest first. `has_more_versions` tells you when a template has more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions_per_template: Option<i64>,
}

impl FlowsTemplatesGetQueryRequest {
    pub fn builder() -> FlowsTemplatesGetQueryRequestBuilder {
        <FlowsTemplatesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlowsTemplatesGetQueryRequestBuilder {
    versions_per_template: Option<i64>,
}

impl FlowsTemplatesGetQueryRequestBuilder {
    pub fn versions_per_template(mut self, value: i64) -> Self {
        self.versions_per_template = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FlowsTemplatesGetQueryRequest`].
    pub fn build(self) -> Result<FlowsTemplatesGetQueryRequest, BuildError> {
        Ok(FlowsTemplatesGetQueryRequest {
            versions_per_template: self.versions_per_template,
        })
    }
}

