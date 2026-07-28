pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SectionSource {
    /// The ID of the song to source the section from. You can find the song ID in the response headers when you generate a song.
    #[serde(default)]
    pub song_id: String,
    /// The range to extract from the source song.
    #[serde(default)]
    pub range: TimeRange,
    /// The ranges to exclude from the 'range'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_ranges: Option<Vec<TimeRange>>,
}

impl SectionSource {
    pub fn builder() -> SectionSourceBuilder {
        <SectionSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SectionSourceBuilder {
    song_id: Option<String>,
    range: Option<TimeRange>,
    negative_ranges: Option<Vec<TimeRange>>,
}

impl SectionSourceBuilder {
    pub fn song_id(mut self, value: impl Into<String>) -> Self {
        self.song_id = Some(value.into());
        self
    }

    pub fn range(mut self, value: TimeRange) -> Self {
        self.range = Some(value);
        self
    }

    pub fn negative_ranges(mut self, value: Vec<TimeRange>) -> Self {
        self.negative_ranges = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SectionSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`song_id`](SectionSourceBuilder::song_id)
    /// - [`range`](SectionSourceBuilder::range)
    pub fn build(self) -> Result<SectionSource, BuildError> {
        Ok(SectionSource {
            song_id: self.song_id.ok_or_else(|| BuildError::missing_field("song_id"))?,
            range: self.range.ok_or_else(|| BuildError::missing_field("range"))?,
            negative_ranges: self.negative_ranges,
        })
    }
}
