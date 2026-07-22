pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteSampleResponse {
    /// The status of the sample deletion request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl DeleteSampleResponse {
    pub fn builder() -> DeleteSampleResponseBuilder {
        <DeleteSampleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteSampleResponseBuilder {
    status: Option<String>,
}

impl DeleteSampleResponseBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteSampleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DeleteSampleResponseBuilder::status)
    pub fn build(self) -> Result<DeleteSampleResponse, BuildError> {
        Ok(DeleteSampleResponse {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
