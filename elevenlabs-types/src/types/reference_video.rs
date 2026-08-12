pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferenceVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio_clip: Option<StudioClipReference>,
}

impl ReferenceVideo {
    pub fn builder() -> ReferenceVideoBuilder {
        <ReferenceVideoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferenceVideoBuilder {
    generation_id: Option<String>,
    content_asset_id: Option<String>,
    template_node_id: Option<String>,
    studio_clip: Option<StudioClipReference>,
}

impl ReferenceVideoBuilder {
    pub fn generation_id(mut self, value: impl Into<String>) -> Self {
        self.generation_id = Some(value.into());
        self
    }

    pub fn content_asset_id(mut self, value: impl Into<String>) -> Self {
        self.content_asset_id = Some(value.into());
        self
    }

    pub fn template_node_id(mut self, value: impl Into<String>) -> Self {
        self.template_node_id = Some(value.into());
        self
    }

    pub fn studio_clip(mut self, value: StudioClipReference) -> Self {
        self.studio_clip = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReferenceVideo`].
    pub fn build(self) -> Result<ReferenceVideo, BuildError> {
        Ok(ReferenceVideo {
            generation_id: self.generation_id,
            content_asset_id: self.content_asset_id,
            template_node_id: self.template_node_id,
            studio_clip: self.studio_clip,
        })
    }
}
