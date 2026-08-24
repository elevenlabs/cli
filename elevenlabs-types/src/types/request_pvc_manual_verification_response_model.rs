pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestPvcManualVerificationResponseModel {
    /// The status of the request PVC manual verification request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl RequestPvcManualVerificationResponseModel {
    pub fn builder() -> RequestPvcManualVerificationResponseModelBuilder {
        <RequestPvcManualVerificationResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestPvcManualVerificationResponseModelBuilder {
    status: Option<String>,
}

impl RequestPvcManualVerificationResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RequestPvcManualVerificationResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](RequestPvcManualVerificationResponseModelBuilder::status)
    pub fn build(self) -> Result<RequestPvcManualVerificationResponseModel, BuildError> {
        Ok(RequestPvcManualVerificationResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
