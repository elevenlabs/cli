pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for structured music generation endpoint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DetailedMusicResponse {
    /// The composition plan used to generate the song
    pub composition_plan: DetailedMusicResponseCompositionPlan,
    /// The metadata of the generated song
    #[serde(default)]
    pub song_metadata: SongMetadata,
    /// The timestamps of the words in the generated song
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words_timestamps: Option<Vec<WordTimestamp>>,
}

impl DetailedMusicResponse {
    pub fn builder() -> DetailedMusicResponseBuilder {
        <DetailedMusicResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetailedMusicResponseBuilder {
    composition_plan: Option<DetailedMusicResponseCompositionPlan>,
    song_metadata: Option<SongMetadata>,
    words_timestamps: Option<Vec<WordTimestamp>>,
}

impl DetailedMusicResponseBuilder {
    pub fn composition_plan(mut self, value: DetailedMusicResponseCompositionPlan) -> Self {
        self.composition_plan = Some(value);
        self
    }

    pub fn song_metadata(mut self, value: SongMetadata) -> Self {
        self.song_metadata = Some(value);
        self
    }

    pub fn words_timestamps(mut self, value: Vec<WordTimestamp>) -> Self {
        self.words_timestamps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DetailedMusicResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`composition_plan`](DetailedMusicResponseBuilder::composition_plan)
    /// - [`song_metadata`](DetailedMusicResponseBuilder::song_metadata)
    pub fn build(self) -> Result<DetailedMusicResponse, BuildError> {
        Ok(DetailedMusicResponse {
            composition_plan: self.composition_plan.ok_or_else(|| BuildError::missing_field("composition_plan"))?,
            song_metadata: self.song_metadata.ok_or_else(|| BuildError::missing_field("song_metadata"))?,
            words_timestamps: self.words_timestamps,
        })
    }
}
