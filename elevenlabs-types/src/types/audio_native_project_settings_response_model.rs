pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioNativeProjectSettingsResponseModel {
    /// The title of the project.
    #[serde(default)]
    pub title: String,
    /// The image of the project.
    #[serde(default)]
    pub image: String,
    /// The author of the project.
    #[serde(default)]
    pub author: String,
    /// Whether the project is small.
    #[serde(default)]
    pub small: bool,
    /// The text color of the project.
    #[serde(default)]
    pub text_color: String,
    /// The background color of the project.
    #[serde(default)]
    pub background_color: String,
    /// The sessionization of the project. Specifies for how many minutes to persist the session across page reloads.
    #[serde(default)]
    pub sessionization: i64,
    /// The path of the audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_path: Option<String>,
    /// The URL of the audio file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_url: Option<String>,
    /// Current state of the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AudioNativeProjectSettingsResponseModelStatus>,
}

impl AudioNativeProjectSettingsResponseModel {
    pub fn builder() -> AudioNativeProjectSettingsResponseModelBuilder {
        <AudioNativeProjectSettingsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioNativeProjectSettingsResponseModelBuilder {
    title: Option<String>,
    image: Option<String>,
    author: Option<String>,
    small: Option<bool>,
    text_color: Option<String>,
    background_color: Option<String>,
    sessionization: Option<i64>,
    audio_path: Option<String>,
    audio_url: Option<String>,
    status: Option<AudioNativeProjectSettingsResponseModelStatus>,
}

impl AudioNativeProjectSettingsResponseModelBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn image(mut self, value: impl Into<String>) -> Self {
        self.image = Some(value.into());
        self
    }

    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.author = Some(value.into());
        self
    }

    pub fn small(mut self, value: bool) -> Self {
        self.small = Some(value);
        self
    }

    pub fn text_color(mut self, value: impl Into<String>) -> Self {
        self.text_color = Some(value.into());
        self
    }

    pub fn background_color(mut self, value: impl Into<String>) -> Self {
        self.background_color = Some(value.into());
        self
    }

    pub fn sessionization(mut self, value: i64) -> Self {
        self.sessionization = Some(value);
        self
    }

    pub fn audio_path(mut self, value: impl Into<String>) -> Self {
        self.audio_path = Some(value.into());
        self
    }

    pub fn audio_url(mut self, value: impl Into<String>) -> Self {
        self.audio_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: AudioNativeProjectSettingsResponseModelStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioNativeProjectSettingsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](AudioNativeProjectSettingsResponseModelBuilder::title)
    /// - [`image`](AudioNativeProjectSettingsResponseModelBuilder::image)
    /// - [`author`](AudioNativeProjectSettingsResponseModelBuilder::author)
    /// - [`small`](AudioNativeProjectSettingsResponseModelBuilder::small)
    /// - [`text_color`](AudioNativeProjectSettingsResponseModelBuilder::text_color)
    /// - [`background_color`](AudioNativeProjectSettingsResponseModelBuilder::background_color)
    /// - [`sessionization`](AudioNativeProjectSettingsResponseModelBuilder::sessionization)
    pub fn build(self) -> Result<AudioNativeProjectSettingsResponseModel, BuildError> {
        Ok(AudioNativeProjectSettingsResponseModel {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            image: self.image.ok_or_else(|| BuildError::missing_field("image"))?,
            author: self.author.ok_or_else(|| BuildError::missing_field("author"))?,
            small: self.small.ok_or_else(|| BuildError::missing_field("small"))?,
            text_color: self.text_color.ok_or_else(|| BuildError::missing_field("text_color"))?,
            background_color: self.background_color.ok_or_else(|| BuildError::missing_field("background_color"))?,
            sessionization: self.sessionization.ok_or_else(|| BuildError::missing_field("sessionization"))?,
            audio_path: self.audio_path,
            audio_url: self.audio_url,
            status: self.status,
        })
    }
}
