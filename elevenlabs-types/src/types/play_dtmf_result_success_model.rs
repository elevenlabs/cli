pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlayDtmfResultSuccessModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub dtmf_tones: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl PlayDtmfResultSuccessModel {
    pub fn builder() -> PlayDtmfResultSuccessModelBuilder {
        <PlayDtmfResultSuccessModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlayDtmfResultSuccessModelBuilder {
    status: Option<String>,
    dtmf_tones: Option<String>,
    reason: Option<String>,
}

impl PlayDtmfResultSuccessModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn dtmf_tones(mut self, value: impl Into<String>) -> Self {
        self.dtmf_tones = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlayDtmfResultSuccessModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dtmf_tones`](PlayDtmfResultSuccessModelBuilder::dtmf_tones)
    pub fn build(self) -> Result<PlayDtmfResultSuccessModel, BuildError> {
        Ok(PlayDtmfResultSuccessModel {
            status: self.status,
            dtmf_tones: self.dtmf_tones.ok_or_else(|| BuildError::missing_field("dtmf_tones"))?,
            reason: self.reason,
        })
    }
}
