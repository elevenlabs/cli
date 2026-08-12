pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioIsolationHistoryItemResponseModel {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub created_at_unix: i64,
    #[serde(default)]
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub duration_seconds: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_video_url: Option<String>,
    #[serde(default)]
    pub supports_video: bool,
    #[serde(default)]
    pub processing: bool,
    #[serde(default)]
    pub video_processing_failed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_b64: Option<String>,
}

impl AudioIsolationHistoryItemResponseModel {
    pub fn builder() -> AudioIsolationHistoryItemResponseModelBuilder {
        <AudioIsolationHistoryItemResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioIsolationHistoryItemResponseModelBuilder {
    id: Option<String>,
    title: Option<String>,
    created_at_unix: Option<i64>,
    format: Option<String>,
    duration_seconds: Option<f64>,
    download_url: Option<String>,
    icon_url: Option<String>,
    source_video_url: Option<String>,
    supports_video: Option<bool>,
    processing: Option<bool>,
    video_processing_failed: Option<bool>,
    preview_b64: Option<String>,
}

impl AudioIsolationHistoryItemResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.format = Some(value.into());
        self
    }

    pub fn duration_seconds(mut self, value: f64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn download_url(mut self, value: impl Into<String>) -> Self {
        self.download_url = Some(value.into());
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn source_video_url(mut self, value: impl Into<String>) -> Self {
        self.source_video_url = Some(value.into());
        self
    }

    pub fn supports_video(mut self, value: bool) -> Self {
        self.supports_video = Some(value);
        self
    }

    pub fn processing(mut self, value: bool) -> Self {
        self.processing = Some(value);
        self
    }

    pub fn video_processing_failed(mut self, value: bool) -> Self {
        self.video_processing_failed = Some(value);
        self
    }

    pub fn preview_b64(mut self, value: impl Into<String>) -> Self {
        self.preview_b64 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioIsolationHistoryItemResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AudioIsolationHistoryItemResponseModelBuilder::id)
    /// - [`created_at_unix`](AudioIsolationHistoryItemResponseModelBuilder::created_at_unix)
    /// - [`format`](AudioIsolationHistoryItemResponseModelBuilder::format)
    /// - [`supports_video`](AudioIsolationHistoryItemResponseModelBuilder::supports_video)
    /// - [`processing`](AudioIsolationHistoryItemResponseModelBuilder::processing)
    /// - [`video_processing_failed`](AudioIsolationHistoryItemResponseModelBuilder::video_processing_failed)
    pub fn build(self) -> Result<AudioIsolationHistoryItemResponseModel, BuildError> {
        Ok(AudioIsolationHistoryItemResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self.title,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            format: self.format.ok_or_else(|| BuildError::missing_field("format"))?,
            duration_seconds: self.duration_seconds,
            download_url: self.download_url,
            icon_url: self.icon_url,
            source_video_url: self.source_video_url,
            supports_video: self.supports_video.ok_or_else(|| BuildError::missing_field("supports_video"))?,
            processing: self.processing.ok_or_else(|| BuildError::missing_field("processing"))?,
            video_processing_failed: self.video_processing_failed.ok_or_else(|| BuildError::missing_field("video_processing_failed"))?,
            preview_b64: self.preview_b64,
        })
    }
}
