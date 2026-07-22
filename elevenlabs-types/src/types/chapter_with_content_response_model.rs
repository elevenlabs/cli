pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterWithContentResponseModel {
    /// The ID of the chapter.
    #[serde(default)]
    pub chapter_id: String,
    /// The name of the chapter.
    #[serde(default)]
    pub name: String,
    /// The last conversion date of the chapter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_conversion_date_unix: Option<i64>,
    /// The conversion progress of the chapter.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub conversion_progress: Option<f64>,
    /// Whether the chapter can be downloaded.
    #[serde(default)]
    pub can_be_downloaded: bool,
    /// The state of the chapter.
    pub state: ChapterWithContentResponseModelState,
    /// Whether the chapter has a video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_video: Option<bool>,
    /// Whether the chapter has any visual content (video, image, or text clips).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visual_content: Option<bool>,
    /// List of voice ids used by the chapter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_ids: Option<Vec<String>>,
    /// The statistics of the chapter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<ChapterStatisticsResponse>,
    /// The last conversion error of the chapter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_conversion_error: Option<String>,
    #[serde(default)]
    pub content: ChapterContentResponseModel,
}

impl ChapterWithContentResponseModel {
    pub fn builder() -> ChapterWithContentResponseModelBuilder {
        <ChapterWithContentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterWithContentResponseModelBuilder {
    chapter_id: Option<String>,
    name: Option<String>,
    last_conversion_date_unix: Option<i64>,
    conversion_progress: Option<f64>,
    can_be_downloaded: Option<bool>,
    state: Option<ChapterWithContentResponseModelState>,
    has_video: Option<bool>,
    has_visual_content: Option<bool>,
    voice_ids: Option<Vec<String>>,
    statistics: Option<ChapterStatisticsResponse>,
    last_conversion_error: Option<String>,
    content: Option<ChapterContentResponseModel>,
}

impl ChapterWithContentResponseModelBuilder {
    pub fn chapter_id(mut self, value: impl Into<String>) -> Self {
        self.chapter_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn last_conversion_date_unix(mut self, value: i64) -> Self {
        self.last_conversion_date_unix = Some(value);
        self
    }

    pub fn conversion_progress(mut self, value: f64) -> Self {
        self.conversion_progress = Some(value);
        self
    }

    pub fn can_be_downloaded(mut self, value: bool) -> Self {
        self.can_be_downloaded = Some(value);
        self
    }

    pub fn state(mut self, value: ChapterWithContentResponseModelState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn has_video(mut self, value: bool) -> Self {
        self.has_video = Some(value);
        self
    }

    pub fn has_visual_content(mut self, value: bool) -> Self {
        self.has_visual_content = Some(value);
        self
    }

    pub fn voice_ids(mut self, value: Vec<String>) -> Self {
        self.voice_ids = Some(value);
        self
    }

    pub fn statistics(mut self, value: ChapterStatisticsResponse) -> Self {
        self.statistics = Some(value);
        self
    }

    pub fn last_conversion_error(mut self, value: impl Into<String>) -> Self {
        self.last_conversion_error = Some(value.into());
        self
    }

    pub fn content(mut self, value: ChapterContentResponseModel) -> Self {
        self.content = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChapterWithContentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapter_id`](ChapterWithContentResponseModelBuilder::chapter_id)
    /// - [`name`](ChapterWithContentResponseModelBuilder::name)
    /// - [`can_be_downloaded`](ChapterWithContentResponseModelBuilder::can_be_downloaded)
    /// - [`state`](ChapterWithContentResponseModelBuilder::state)
    /// - [`content`](ChapterWithContentResponseModelBuilder::content)
    pub fn build(self) -> Result<ChapterWithContentResponseModel, BuildError> {
        Ok(ChapterWithContentResponseModel {
            chapter_id: self.chapter_id.ok_or_else(|| BuildError::missing_field("chapter_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            last_conversion_date_unix: self.last_conversion_date_unix,
            conversion_progress: self.conversion_progress,
            can_be_downloaded: self.can_be_downloaded.ok_or_else(|| BuildError::missing_field("can_be_downloaded"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            has_video: self.has_video,
            has_visual_content: self.has_visual_content,
            voice_ids: self.voice_ids,
            statistics: self.statistics,
            last_conversion_error: self.last_conversion_error,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
