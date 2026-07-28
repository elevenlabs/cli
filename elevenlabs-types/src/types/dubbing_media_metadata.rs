pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingMediaMetadata {
    /// The content type of the media.
    #[serde(default)]
    pub content_type: String,
    /// The duration of the media in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub duration: f64,
}

impl DubbingMediaMetadata {
    pub fn builder() -> DubbingMediaMetadataBuilder {
        <DubbingMediaMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingMediaMetadataBuilder {
    content_type: Option<String>,
    duration: Option<f64>,
}

impl DubbingMediaMetadataBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn duration(mut self, value: f64) -> Self {
        self.duration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingMediaMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_type`](DubbingMediaMetadataBuilder::content_type)
    /// - [`duration`](DubbingMediaMetadataBuilder::duration)
    pub fn build(self) -> Result<DubbingMediaMetadata, BuildError> {
        Ok(DubbingMediaMetadata {
            content_type: self.content_type.ok_or_else(|| BuildError::missing_field("content_type"))?,
            duration: self.duration.ok_or_else(|| BuildError::missing_field("duration"))?,
        })
    }
}
