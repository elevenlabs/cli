pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingMediaReference {
    #[serde(default)]
    pub src: String,
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub bucket_name: String,
    #[serde(default)]
    pub random_path_slug: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub duration_secs: f64,
    #[serde(default)]
    pub is_audio: bool,
    #[serde(default)]
    pub url: String,
}

impl DubbingMediaReference {
    pub fn builder() -> DubbingMediaReferenceBuilder {
        <DubbingMediaReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingMediaReferenceBuilder {
    src: Option<String>,
    content_type: Option<String>,
    bucket_name: Option<String>,
    random_path_slug: Option<String>,
    duration_secs: Option<f64>,
    is_audio: Option<bool>,
    url: Option<String>,
}

impl DubbingMediaReferenceBuilder {
    pub fn src(mut self, value: impl Into<String>) -> Self {
        self.src = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn bucket_name(mut self, value: impl Into<String>) -> Self {
        self.bucket_name = Some(value.into());
        self
    }

    pub fn random_path_slug(mut self, value: impl Into<String>) -> Self {
        self.random_path_slug = Some(value.into());
        self
    }

    pub fn duration_secs(mut self, value: f64) -> Self {
        self.duration_secs = Some(value);
        self
    }

    pub fn is_audio(mut self, value: bool) -> Self {
        self.is_audio = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingMediaReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`src`](DubbingMediaReferenceBuilder::src)
    /// - [`content_type`](DubbingMediaReferenceBuilder::content_type)
    /// - [`bucket_name`](DubbingMediaReferenceBuilder::bucket_name)
    /// - [`random_path_slug`](DubbingMediaReferenceBuilder::random_path_slug)
    /// - [`duration_secs`](DubbingMediaReferenceBuilder::duration_secs)
    /// - [`is_audio`](DubbingMediaReferenceBuilder::is_audio)
    /// - [`url`](DubbingMediaReferenceBuilder::url)
    pub fn build(self) -> Result<DubbingMediaReference, BuildError> {
        Ok(DubbingMediaReference {
            src: self.src.ok_or_else(|| BuildError::missing_field("src"))?,
            content_type: self.content_type.ok_or_else(|| BuildError::missing_field("content_type"))?,
            bucket_name: self.bucket_name.ok_or_else(|| BuildError::missing_field("bucket_name"))?,
            random_path_slug: self.random_path_slug.ok_or_else(|| BuildError::missing_field("random_path_slug"))?,
            duration_secs: self.duration_secs.ok_or_else(|| BuildError::missing_field("duration_secs"))?,
            is_audio: self.is_audio.ok_or_else(|| BuildError::missing_field("is_audio"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
