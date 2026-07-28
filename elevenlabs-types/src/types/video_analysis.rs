pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VideoAnalysis {
    pub status: VideoAnalysisStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<VideoAnalysisResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_ms: Option<i64>,
}

impl VideoAnalysis {
    pub fn builder() -> VideoAnalysisBuilder {
        <VideoAnalysisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VideoAnalysisBuilder {
    status: Option<VideoAnalysisStatus>,
    data: Option<VideoAnalysisResult>,
    updated_at_ms: Option<i64>,
}

impl VideoAnalysisBuilder {
    pub fn status(mut self, value: VideoAnalysisStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn data(mut self, value: VideoAnalysisResult) -> Self {
        self.data = Some(value);
        self
    }

    pub fn updated_at_ms(mut self, value: i64) -> Self {
        self.updated_at_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VideoAnalysis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](VideoAnalysisBuilder::status)
    pub fn build(self) -> Result<VideoAnalysis, BuildError> {
        Ok(VideoAnalysis {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            data: self.data,
            updated_at_ms: self.updated_at_ms,
        })
    }
}
