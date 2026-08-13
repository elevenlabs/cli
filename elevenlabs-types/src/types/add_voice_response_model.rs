pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddVoiceResponseModel {
    /// The ID of the voice.
    #[serde(default)]
    pub voice_id: String,
}

impl AddVoiceResponseModel {
    pub fn builder() -> AddVoiceResponseModelBuilder {
        <AddVoiceResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddVoiceResponseModelBuilder {
    voice_id: Option<String>,
}

impl AddVoiceResponseModelBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddVoiceResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_id`](AddVoiceResponseModelBuilder::voice_id)
    pub fn build(self) -> Result<AddVoiceResponseModel, BuildError> {
        Ok(AddVoiceResponseModel {
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
        })
    }
}
