pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIdRenderLanguagePost {
    /// The type of the render. One of ['mp4', 'aac', 'mp3', 'wav', 'aaf', 'tracks_zip', 'clips_zip']
    pub render_type: RenderType,
    /// Whether to normalize the volume of the rendered audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalize_volume: Option<bool>,
}

impl BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIdRenderLanguagePost {
    pub fn builder() -> BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIdRenderLanguagePostBuilder {
        <BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIdRenderLanguagePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIdRenderLanguagePostBuilder {
    render_type: Option<RenderType>,
    normalize_volume: Option<bool>,
}

impl BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIdRenderLanguagePostBuilder {
    pub fn render_type(mut self, value: RenderType) -> Self {
        self.render_type = Some(value);
        self
    }

    pub fn normalize_volume(mut self, value: bool) -> Self {
        self.normalize_volume = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIdRenderLanguagePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`render_type`](BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIdRenderLanguagePostBuilder::render_type)
    pub fn build(self) -> Result<BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIdRenderLanguagePost, BuildError> {
        Ok(BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIdRenderLanguagePost {
            render_type: self.render_type.ok_or_else(|| BuildError::missing_field("render_type"))?,
            normalize_volume: self.normalize_volume,
        })
    }
}

