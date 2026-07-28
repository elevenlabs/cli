pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteVoiceResponseModel {
    /// The status of the voice deletion request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl DeleteVoiceResponseModel {
    pub fn builder() -> DeleteVoiceResponseModelBuilder {
        <DeleteVoiceResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteVoiceResponseModelBuilder {
    status: Option<String>,
}

impl DeleteVoiceResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteVoiceResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DeleteVoiceResponseModelBuilder::status)
    pub fn build(self) -> Result<DeleteVoiceResponseModel, BuildError> {
        Ok(DeleteVoiceResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
