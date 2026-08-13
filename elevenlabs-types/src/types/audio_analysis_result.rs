pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioAnalysisResult {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_pacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<AudioSegment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_moments: Option<Vec<AudioKeyMoment>>,
}

impl AudioAnalysisResult {
    pub fn builder() -> AudioAnalysisResultBuilder {
        <AudioAnalysisResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioAnalysisResultBuilder {
    title: Option<String>,
    description: Option<String>,
    content_type: Option<String>,
    overall_pacing: Option<String>,
    segments: Option<Vec<AudioSegment>>,
    key_moments: Option<Vec<AudioKeyMoment>>,
}

impl AudioAnalysisResultBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn overall_pacing(mut self, value: impl Into<String>) -> Self {
        self.overall_pacing = Some(value.into());
        self
    }

    pub fn segments(mut self, value: Vec<AudioSegment>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn key_moments(mut self, value: Vec<AudioKeyMoment>) -> Self {
        self.key_moments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioAnalysisResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](AudioAnalysisResultBuilder::title)
    /// - [`description`](AudioAnalysisResultBuilder::description)
    pub fn build(self) -> Result<AudioAnalysisResult, BuildError> {
        Ok(AudioAnalysisResult {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            content_type: self.content_type,
            overall_pacing: self.overall_pacing,
            segments: self.segments,
            key_moments: self.key_moments,
        })
    }
}
