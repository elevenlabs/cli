pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StartSpeakerSeparationResponseModel {
    /// The status of the start speaker seperation request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl StartSpeakerSeparationResponseModel {
    pub fn builder() -> StartSpeakerSeparationResponseModelBuilder {
        <StartSpeakerSeparationResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartSpeakerSeparationResponseModelBuilder {
    status: Option<String>,
}

impl StartSpeakerSeparationResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StartSpeakerSeparationResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](StartSpeakerSeparationResponseModelBuilder::status)
    pub fn build(self) -> Result<StartSpeakerSeparationResponseModel, BuildError> {
        Ok(StartSpeakerSeparationResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
