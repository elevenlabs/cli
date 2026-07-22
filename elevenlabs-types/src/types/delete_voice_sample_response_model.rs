pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteVoiceSampleResponseModel {
    /// The status of the voice sample deletion request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl DeleteVoiceSampleResponseModel {
    pub fn builder() -> DeleteVoiceSampleResponseModelBuilder {
        <DeleteVoiceSampleResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteVoiceSampleResponseModelBuilder {
    status: Option<String>,
}

impl DeleteVoiceSampleResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteVoiceSampleResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DeleteVoiceSampleResponseModelBuilder::status)
    pub fn build(self) -> Result<DeleteVoiceSampleResponseModel, BuildError> {
        Ok(DeleteVoiceSampleResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
