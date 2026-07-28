pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VideoAnalysisResult {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_pacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<VideoSubject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<VideoSegment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_moments: Option<Vec<VideoKeyMoment>>,
}

impl VideoAnalysisResult {
    pub fn builder() -> VideoAnalysisResultBuilder {
        <VideoAnalysisResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VideoAnalysisResultBuilder {
    title: Option<String>,
    description: Option<String>,
    content_type: Option<String>,
    overall_pacing: Option<String>,
    subjects: Option<Vec<VideoSubject>>,
    segments: Option<Vec<VideoSegment>>,
    key_moments: Option<Vec<VideoKeyMoment>>,
}

impl VideoAnalysisResultBuilder {
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

    pub fn subjects(mut self, value: Vec<VideoSubject>) -> Self {
        self.subjects = Some(value);
        self
    }

    pub fn segments(mut self, value: Vec<VideoSegment>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn key_moments(mut self, value: Vec<VideoKeyMoment>) -> Self {
        self.key_moments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VideoAnalysisResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](VideoAnalysisResultBuilder::title)
    /// - [`description`](VideoAnalysisResultBuilder::description)
    pub fn build(self) -> Result<VideoAnalysisResult, BuildError> {
        Ok(VideoAnalysisResult {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            content_type: self.content_type,
            overall_pacing: self.overall_pacing,
            subjects: self.subjects,
            segments: self.segments,
            key_moments: self.key_moments,
        })
    }
}
