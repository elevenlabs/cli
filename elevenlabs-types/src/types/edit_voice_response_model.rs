pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EditVoiceResponseModel {
    /// The status of the voice edit request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl EditVoiceResponseModel {
    pub fn builder() -> EditVoiceResponseModelBuilder {
        <EditVoiceResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditVoiceResponseModelBuilder {
    status: Option<String>,
}

impl EditVoiceResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EditVoiceResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](EditVoiceResponseModelBuilder::status)
    pub fn build(self) -> Result<EditVoiceResponseModel, BuildError> {
        Ok(EditVoiceResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
