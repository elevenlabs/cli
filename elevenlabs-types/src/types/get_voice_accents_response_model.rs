pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetVoiceAccentsResponseModel {
    /// A list of available voice accents.
    #[serde(default)]
    pub accents: Vec<VoiceAccentResponseModel>,
}

impl GetVoiceAccentsResponseModel {
    pub fn builder() -> GetVoiceAccentsResponseModelBuilder {
        <GetVoiceAccentsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetVoiceAccentsResponseModelBuilder {
    accents: Option<Vec<VoiceAccentResponseModel>>,
}

impl GetVoiceAccentsResponseModelBuilder {
    pub fn accents(mut self, value: Vec<VoiceAccentResponseModel>) -> Self {
        self.accents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetVoiceAccentsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accents`](GetVoiceAccentsResponseModelBuilder::accents)
    pub fn build(self) -> Result<GetVoiceAccentsResponseModel, BuildError> {
        Ok(GetVoiceAccentsResponseModel {
            accents: self.accents.ok_or_else(|| BuildError::missing_field("accents"))?,
        })
    }
}
