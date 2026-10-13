pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for export
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportQueryRequest {
    /// Only export the first N recipients; used to preview the columns. Omit to export every recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ExportQueryRequest {
    pub fn builder() -> ExportQueryRequestBuilder {
        <ExportQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportQueryRequestBuilder {
    limit: Option<i64>,
}

impl ExportQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportQueryRequest`].
    pub fn build(self) -> Result<ExportQueryRequest, BuildError> {
        Ok(ExportQueryRequest {
            limit: self.limit,
        })
    }
}

