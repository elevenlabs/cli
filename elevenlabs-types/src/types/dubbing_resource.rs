pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingResource {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub source_language: String,
    #[serde(default)]
    pub target_languages: Vec<String>,
    #[serde(default)]
    pub input: DubbingMediaReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<DubbingMediaReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground: Option<DubbingMediaReference>,
    #[serde(default)]
    pub speaker_tracks: HashMap<String, SpeakerTrack>,
    #[serde(default)]
    pub speaker_segments: HashMap<String, SpeakerSegment>,
    #[serde(default)]
    pub renders: HashMap<String, Render>,
}

impl DubbingResource {
    pub fn builder() -> DubbingResourceBuilder {
        <DubbingResourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingResourceBuilder {
    id: Option<String>,
    version: Option<i64>,
    source_language: Option<String>,
    target_languages: Option<Vec<String>>,
    input: Option<DubbingMediaReference>,
    background: Option<DubbingMediaReference>,
    foreground: Option<DubbingMediaReference>,
    speaker_tracks: Option<HashMap<String, SpeakerTrack>>,
    speaker_segments: Option<HashMap<String, SpeakerSegment>>,
    renders: Option<HashMap<String, Render>>,
}

impl DubbingResourceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn source_language(mut self, value: impl Into<String>) -> Self {
        self.source_language = Some(value.into());
        self
    }

    pub fn target_languages(mut self, value: Vec<String>) -> Self {
        self.target_languages = Some(value);
        self
    }

    pub fn input(mut self, value: DubbingMediaReference) -> Self {
        self.input = Some(value);
        self
    }

    pub fn background(mut self, value: DubbingMediaReference) -> Self {
        self.background = Some(value);
        self
    }

    pub fn foreground(mut self, value: DubbingMediaReference) -> Self {
        self.foreground = Some(value);
        self
    }

    pub fn speaker_tracks(mut self, value: HashMap<String, SpeakerTrack>) -> Self {
        self.speaker_tracks = Some(value);
        self
    }

    pub fn speaker_segments(mut self, value: HashMap<String, SpeakerSegment>) -> Self {
        self.speaker_segments = Some(value);
        self
    }

    pub fn renders(mut self, value: HashMap<String, Render>) -> Self {
        self.renders = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingResource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DubbingResourceBuilder::id)
    /// - [`version`](DubbingResourceBuilder::version)
    /// - [`source_language`](DubbingResourceBuilder::source_language)
    /// - [`target_languages`](DubbingResourceBuilder::target_languages)
    /// - [`input`](DubbingResourceBuilder::input)
    /// - [`speaker_tracks`](DubbingResourceBuilder::speaker_tracks)
    /// - [`speaker_segments`](DubbingResourceBuilder::speaker_segments)
    /// - [`renders`](DubbingResourceBuilder::renders)
    pub fn build(self) -> Result<DubbingResource, BuildError> {
        Ok(DubbingResource {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            source_language: self.source_language.ok_or_else(|| BuildError::missing_field("source_language"))?,
            target_languages: self.target_languages.ok_or_else(|| BuildError::missing_field("target_languages"))?,
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
            background: self.background,
            foreground: self.foreground,
            speaker_tracks: self.speaker_tracks.ok_or_else(|| BuildError::missing_field("speaker_tracks"))?,
            speaker_segments: self.speaker_segments.ok_or_else(|| BuildError::missing_field("speaker_segments"))?,
            renders: self.renders.ok_or_else(|| BuildError::missing_field("renders"))?,
        })
    }
}
