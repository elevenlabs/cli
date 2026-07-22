pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EditVoiceSettingsResponseModel {
    /// The status of the voice settings edit request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl EditVoiceSettingsResponseModel {
    pub fn builder() -> EditVoiceSettingsResponseModelBuilder {
        <EditVoiceSettingsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditVoiceSettingsResponseModelBuilder {
    status: Option<String>,
}

impl EditVoiceSettingsResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EditVoiceSettingsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](EditVoiceSettingsResponseModelBuilder::status)
    pub fn build(self) -> Result<EditVoiceSettingsResponseModel, BuildError> {
        Ok(EditVoiceSettingsResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
