pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VideoToMusicRequest {
    #[serde(default)]
    pub videos: Vec<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<MusicModelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_with_c2pa: Option<bool>,
    #[serde(skip)]
    pub output_format: Option<AllowedOutputFormats>,
}
impl VideoToMusicRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    for file_data in &self.videos {
        form = form.part(
            "videos",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("videos")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Some(ref value) = self.description {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("description", json_str);
        }
    }

    if let Some(ref value) = self.tags {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("tags", json_str);
        }
    }

    if let Some(ref value) = self.model_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("model_id", json_str);
        }
    }

    if let Some(ref value) = self.sign_with_c2pa {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("sign_with_c2pa", json_str);
        }
    }

    form
}
}

impl VideoToMusicRequest {
    pub fn builder() -> VideoToMusicRequestBuilder {
        <VideoToMusicRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VideoToMusicRequestBuilder {
    videos: Option<Vec<Vec<u8>>>,
    description: Option<String>,
    tags: Option<Vec<String>>,
    model_id: Option<MusicModelId>,
    sign_with_c2pa: Option<bool>,
    output_format: Option<AllowedOutputFormats>,
}

impl VideoToMusicRequestBuilder {
    pub fn videos(mut self, value: Vec<Vec<u8>>) -> Self {
        self.videos = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn model_id(mut self, value: MusicModelId) -> Self {
        self.model_id = Some(value);
        self
    }

    pub fn sign_with_c2pa(mut self, value: bool) -> Self {
        self.sign_with_c2pa = Some(value);
        self
    }

    pub fn output_format(mut self, value: AllowedOutputFormats) -> Self {
        self.output_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VideoToMusicRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`videos`](VideoToMusicRequestBuilder::videos)
    pub fn build(self) -> Result<VideoToMusicRequest, BuildError> {
        Ok(VideoToMusicRequest {
            videos: self.videos.ok_or_else(|| BuildError::missing_field("videos"))?,
            description: self.description,
            tags: self.tags,
            model_id: self.model_id,
            sign_with_c2pa: self.sign_with_c2pa,
            output_format: self.output_format,
        })
    }
}
