pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Coordinates of a clip inside a Studio project: the payload of a studio_clip reference.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StudioClipLocator {
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub chapter_id: String,
    pub clip_type: StudioClipLocatorClipType,
    #[serde(default)]
    pub clip_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
}

impl StudioClipLocator {
    pub fn builder() -> StudioClipLocatorBuilder {
        <StudioClipLocatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StudioClipLocatorBuilder {
    project_id: Option<String>,
    chapter_id: Option<String>,
    clip_type: Option<StudioClipLocatorClipType>,
    clip_id: Option<String>,
    block_id: Option<String>,
    preview_url: Option<String>,
}

impl StudioClipLocatorBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn chapter_id(mut self, value: impl Into<String>) -> Self {
        self.chapter_id = Some(value.into());
        self
    }

    pub fn clip_type(mut self, value: StudioClipLocatorClipType) -> Self {
        self.clip_type = Some(value);
        self
    }

    pub fn clip_id(mut self, value: impl Into<String>) -> Self {
        self.clip_id = Some(value.into());
        self
    }

    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }

    pub fn preview_url(mut self, value: impl Into<String>) -> Self {
        self.preview_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StudioClipLocator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](StudioClipLocatorBuilder::project_id)
    /// - [`chapter_id`](StudioClipLocatorBuilder::chapter_id)
    /// - [`clip_type`](StudioClipLocatorBuilder::clip_type)
    /// - [`clip_id`](StudioClipLocatorBuilder::clip_id)
    pub fn build(self) -> Result<StudioClipLocator, BuildError> {
        Ok(StudioClipLocator {
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            chapter_id: self.chapter_id.ok_or_else(|| BuildError::missing_field("chapter_id"))?,
            clip_type: self.clip_type.ok_or_else(|| BuildError::missing_field("clip_type"))?,
            clip_id: self.clip_id.ok_or_else(|| BuildError::missing_field("clip_id"))?,
            block_id: self.block_id,
            preview_url: self.preview_url,
        })
    }
}
