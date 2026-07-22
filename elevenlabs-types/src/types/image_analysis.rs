pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ImageAnalysis {
    pub status: ImageAnalysisStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ImageAnalysisResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_ms: Option<i64>,
}

impl ImageAnalysis {
    pub fn builder() -> ImageAnalysisBuilder {
        <ImageAnalysisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageAnalysisBuilder {
    status: Option<ImageAnalysisStatus>,
    data: Option<ImageAnalysisResult>,
    updated_at_ms: Option<i64>,
}

impl ImageAnalysisBuilder {
    pub fn status(mut self, value: ImageAnalysisStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn data(mut self, value: ImageAnalysisResult) -> Self {
        self.data = Some(value);
        self
    }

    pub fn updated_at_ms(mut self, value: i64) -> Self {
        self.updated_at_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImageAnalysis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ImageAnalysisBuilder::status)
    pub fn build(self) -> Result<ImageAnalysis, BuildError> {
        Ok(ImageAnalysis {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            data: self.data,
            updated_at_ms: self.updated_at_ms,
        })
    }
}
