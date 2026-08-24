pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioRefChunk {
    /// The ID of the song to source the chunk from. You can find the song ID in the response headers when you generate a song.
    #[serde(default)]
    pub song_id: String,
    /// The time range to extract from the song.
    #[serde(default)]
    pub range: TimeRange,
}

impl AudioRefChunk {
    pub fn builder() -> AudioRefChunkBuilder {
        <AudioRefChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioRefChunkBuilder {
    song_id: Option<String>,
    range: Option<TimeRange>,
}

impl AudioRefChunkBuilder {
    pub fn song_id(mut self, value: impl Into<String>) -> Self {
        self.song_id = Some(value.into());
        self
    }

    pub fn range(mut self, value: TimeRange) -> Self {
        self.range = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioRefChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`song_id`](AudioRefChunkBuilder::song_id)
    /// - [`range`](AudioRefChunkBuilder::range)
    pub fn build(self) -> Result<AudioRefChunk, BuildError> {
        Ok(AudioRefChunk {
            song_id: self.song_id.ok_or_else(|| BuildError::missing_field("song_id"))?,
            range: self.range.ok_or_else(|| BuildError::missing_field("range"))?,
        })
    }
}
