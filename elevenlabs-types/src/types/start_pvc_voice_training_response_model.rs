pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StartPvcVoiceTrainingResponseModel {
    /// The status of the start PVC voice training request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl StartPvcVoiceTrainingResponseModel {
    pub fn builder() -> StartPvcVoiceTrainingResponseModelBuilder {
        <StartPvcVoiceTrainingResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartPvcVoiceTrainingResponseModelBuilder {
    status: Option<String>,
}

impl StartPvcVoiceTrainingResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StartPvcVoiceTrainingResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](StartPvcVoiceTrainingResponseModelBuilder::status)
    pub fn build(self) -> Result<StartPvcVoiceTrainingResponseModel, BuildError> {
        Ok(StartPvcVoiceTrainingResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
