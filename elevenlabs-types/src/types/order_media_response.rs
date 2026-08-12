pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrderMediaResponse {
    /// The ID of the media file.
    #[serde(default)]
    pub media_id: MediaId,
    /// The original filename of the uploaded media.
    #[serde(default)]
    pub name: String,
    /// The MIME type of the media file (e.g. 'video/mp4').
    #[serde(default)]
    pub content_type: String,
    /// The detected or declared language of the media, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// A time-limited URL to download the media file.
    #[serde(default)]
    pub signed_url: String,
}

impl OrderMediaResponse {
    pub fn builder() -> OrderMediaResponseBuilder {
        <OrderMediaResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderMediaResponseBuilder {
    media_id: Option<MediaId>,
    name: Option<String>,
    content_type: Option<String>,
    language: Option<String>,
    signed_url: Option<String>,
}

impl OrderMediaResponseBuilder {
    pub fn media_id(mut self, value: MediaId) -> Self {
        self.media_id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn signed_url(mut self, value: impl Into<String>) -> Self {
        self.signed_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrderMediaResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`media_id`](OrderMediaResponseBuilder::media_id)
    /// - [`name`](OrderMediaResponseBuilder::name)
    /// - [`content_type`](OrderMediaResponseBuilder::content_type)
    /// - [`signed_url`](OrderMediaResponseBuilder::signed_url)
    pub fn build(self) -> Result<OrderMediaResponse, BuildError> {
        Ok(OrderMediaResponse {
            media_id: self.media_id.ok_or_else(|| BuildError::missing_field("media_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            content_type: self.content_type.ok_or_else(|| BuildError::missing_field("content_type"))?,
            language: self.language,
            signed_url: self.signed_url.ok_or_else(|| BuildError::missing_field("signed_url"))?,
        })
    }
}
