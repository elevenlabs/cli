pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChapterStatisticsResponse {
    /// The number of unconverted characters.
    #[serde(default)]
    pub characters_unconverted: i64,
    /// The number of converted characters.
    #[serde(default)]
    pub characters_converted: i64,
    /// The number of converted paragraphs.
    #[serde(default)]
    pub paragraphs_converted: i64,
    /// The number of unconverted paragraphs.
    #[serde(default)]
    pub paragraphs_unconverted: i64,
    /// The number of credits needed to convert the remaining paragraphs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits_needed_to_convert: Option<i64>,
    /// Per-voice breakdown of character counts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_statistics: Option<Vec<VoiceStatisticsResponseModel>>,
}

impl ChapterStatisticsResponse {
    pub fn builder() -> ChapterStatisticsResponseBuilder {
        <ChapterStatisticsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterStatisticsResponseBuilder {
    characters_unconverted: Option<i64>,
    characters_converted: Option<i64>,
    paragraphs_converted: Option<i64>,
    paragraphs_unconverted: Option<i64>,
    credits_needed_to_convert: Option<i64>,
    voice_statistics: Option<Vec<VoiceStatisticsResponseModel>>,
}

impl ChapterStatisticsResponseBuilder {
    pub fn characters_unconverted(mut self, value: i64) -> Self {
        self.characters_unconverted = Some(value);
        self
    }

    pub fn characters_converted(mut self, value: i64) -> Self {
        self.characters_converted = Some(value);
        self
    }

    pub fn paragraphs_converted(mut self, value: i64) -> Self {
        self.paragraphs_converted = Some(value);
        self
    }

    pub fn paragraphs_unconverted(mut self, value: i64) -> Self {
        self.paragraphs_unconverted = Some(value);
        self
    }

    pub fn credits_needed_to_convert(mut self, value: i64) -> Self {
        self.credits_needed_to_convert = Some(value);
        self
    }

    pub fn voice_statistics(mut self, value: Vec<VoiceStatisticsResponseModel>) -> Self {
        self.voice_statistics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChapterStatisticsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`characters_unconverted`](ChapterStatisticsResponseBuilder::characters_unconverted)
    /// - [`characters_converted`](ChapterStatisticsResponseBuilder::characters_converted)
    /// - [`paragraphs_converted`](ChapterStatisticsResponseBuilder::paragraphs_converted)
    /// - [`paragraphs_unconverted`](ChapterStatisticsResponseBuilder::paragraphs_unconverted)
    pub fn build(self) -> Result<ChapterStatisticsResponse, BuildError> {
        Ok(ChapterStatisticsResponse {
            characters_unconverted: self.characters_unconverted.ok_or_else(|| BuildError::missing_field("characters_unconverted"))?,
            characters_converted: self.characters_converted.ok_or_else(|| BuildError::missing_field("characters_converted"))?,
            paragraphs_converted: self.paragraphs_converted.ok_or_else(|| BuildError::missing_field("paragraphs_converted"))?,
            paragraphs_unconverted: self.paragraphs_unconverted.ok_or_else(|| BuildError::missing_field("paragraphs_unconverted"))?,
            credits_needed_to_convert: self.credits_needed_to_convert,
            voice_statistics: self.voice_statistics,
        })
    }
}
