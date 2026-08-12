pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StudioClipReference {
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub chapter_id: String,
    pub clip_type: StudioClipReferenceClipType,
    #[serde(default)]
    pub clip_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
}

impl StudioClipReference {
    pub fn builder() -> StudioClipReferenceBuilder {
        <StudioClipReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StudioClipReferenceBuilder {
    project_id: Option<String>,
    chapter_id: Option<String>,
    clip_type: Option<StudioClipReferenceClipType>,
    clip_id: Option<String>,
    block_id: Option<String>,
    preview_url: Option<String>,
}

impl StudioClipReferenceBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn chapter_id(mut self, value: impl Into<String>) -> Self {
        self.chapter_id = Some(value.into());
        self
    }

    pub fn clip_type(mut self, value: StudioClipReferenceClipType) -> Self {
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

    /// Consumes the builder and constructs a [`StudioClipReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](StudioClipReferenceBuilder::project_id)
    /// - [`chapter_id`](StudioClipReferenceBuilder::chapter_id)
    /// - [`clip_type`](StudioClipReferenceBuilder::clip_type)
    /// - [`clip_id`](StudioClipReferenceBuilder::clip_id)
    pub fn build(self) -> Result<StudioClipReference, BuildError> {
        Ok(StudioClipReference {
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            chapter_id: self.chapter_id.ok_or_else(|| BuildError::missing_field("chapter_id"))?,
            clip_type: self.clip_type.ok_or_else(|| BuildError::missing_field("clip_type"))?,
            clip_id: self.clip_id.ok_or_else(|| BuildError::missing_field("clip_id"))?,
            block_id: self.block_id,
            preview_url: self.preview_url,
        })
    }
}
