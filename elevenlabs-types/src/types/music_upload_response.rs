pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for music upload endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MusicUploadResponse {
    /// Unique identifier for the uploaded song
    #[serde(default)]
    pub song_id: String,
    /// The composition plan extracted from the uploaded song. Only present if `extract_composition_plan` was provided in the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composition_plan: Option<MusicUploadResponseCompositionPlan>,
    /// Word-level timestamps transcribed from the uploaded song. Only present if `with_timestamps` was True in the request body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words_timestamps: Option<Vec<WordTimestamp>>,
    /// A low-resolution waveform of the uploaded song, for showing a preview of it. Holds 4 values per second of audio, from -1000 to 1000. Stereo is mixed down to a single channel. Only present if `with_waveform_visual` was True in the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waveform_visual: Option<Vec<i64>>,
}

impl MusicUploadResponse {
    pub fn builder() -> MusicUploadResponseBuilder {
        <MusicUploadResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MusicUploadResponseBuilder {
    song_id: Option<String>,
    composition_plan: Option<MusicUploadResponseCompositionPlan>,
    words_timestamps: Option<Vec<WordTimestamp>>,
    waveform_visual: Option<Vec<i64>>,
}

impl MusicUploadResponseBuilder {
    pub fn song_id(mut self, value: impl Into<String>) -> Self {
        self.song_id = Some(value.into());
        self
    }

    pub fn composition_plan(mut self, value: MusicUploadResponseCompositionPlan) -> Self {
        self.composition_plan = Some(value);
        self
    }

    pub fn words_timestamps(mut self, value: Vec<WordTimestamp>) -> Self {
        self.words_timestamps = Some(value);
        self
    }

    pub fn waveform_visual(mut self, value: Vec<i64>) -> Self {
        self.waveform_visual = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MusicUploadResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`song_id`](MusicUploadResponseBuilder::song_id)
    pub fn build(self) -> Result<MusicUploadResponse, BuildError> {
        Ok(MusicUploadResponse {
            song_id: self.song_id.ok_or_else(|| BuildError::missing_field("song_id"))?,
            composition_plan: self.composition_plan,
            words_timestamps: self.words_timestamps,
            waveform_visual: self.waveform_visual,
        })
    }
}
