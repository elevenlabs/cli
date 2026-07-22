pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceSampleVisualWaveformResponseModel {
    /// The ID of the sample.
    #[serde(default)]
    pub sample_id: String,
    /// The visual waveform of the sample, represented as a list of floats.
    #[serde(default)]
    pub visual_waveform: Vec<f64>,
}

impl VoiceSampleVisualWaveformResponseModel {
    pub fn builder() -> VoiceSampleVisualWaveformResponseModelBuilder {
        <VoiceSampleVisualWaveformResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceSampleVisualWaveformResponseModelBuilder {
    sample_id: Option<String>,
    visual_waveform: Option<Vec<f64>>,
}

impl VoiceSampleVisualWaveformResponseModelBuilder {
    pub fn sample_id(mut self, value: impl Into<String>) -> Self {
        self.sample_id = Some(value.into());
        self
    }

    pub fn visual_waveform(mut self, value: Vec<f64>) -> Self {
        self.visual_waveform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceSampleVisualWaveformResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sample_id`](VoiceSampleVisualWaveformResponseModelBuilder::sample_id)
    /// - [`visual_waveform`](VoiceSampleVisualWaveformResponseModelBuilder::visual_waveform)
    pub fn build(self) -> Result<VoiceSampleVisualWaveformResponseModel, BuildError> {
        Ok(VoiceSampleVisualWaveformResponseModel {
            sample_id: self.sample_id.ok_or_else(|| BuildError::missing_field("sample_id"))?,
            visual_waveform: self.visual_waveform.ok_or_else(|| BuildError::missing_field("visual_waveform"))?,
        })
    }
}
