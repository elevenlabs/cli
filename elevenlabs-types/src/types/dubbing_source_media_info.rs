pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Metadata about the project's source media.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingSourceMediaInfo {
    /// Original filename of the uploaded source media (null for URL sources).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Duration of the source media in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub duration_s: Option<f64>,
    /// Whether the source media contains a video stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_video: Option<bool>,
    /// MIME type of the uploaded source media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

impl DubbingSourceMediaInfo {
    pub fn builder() -> DubbingSourceMediaInfoBuilder {
        <DubbingSourceMediaInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingSourceMediaInfoBuilder {
    filename: Option<String>,
    duration_s: Option<f64>,
    has_video: Option<bool>,
    mime_type: Option<String>,
}

impl DubbingSourceMediaInfoBuilder {
    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn duration_s(mut self, value: f64) -> Self {
        self.duration_s = Some(value);
        self
    }

    pub fn has_video(mut self, value: bool) -> Self {
        self.has_video = Some(value);
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingSourceMediaInfo`].
    pub fn build(self) -> Result<DubbingSourceMediaInfo, BuildError> {
        Ok(DubbingSourceMediaInfo {
            filename: self.filename,
            duration_s: self.duration_s,
            has_video: self.has_video,
            mime_type: self.mime_type,
        })
    }
}
