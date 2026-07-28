pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Composition plan for the `music_v1` model. Using this field with any other model will result in an error.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MusicPrompt {
    /// The styles and musical directions that should be present in the entire song. Use English language for best result.
    #[serde(default)]
    pub positive_global_styles: Vec<String>,
    /// The styles and musical directions that should not be present in the entire song. Use English language for best result.
    #[serde(default)]
    pub negative_global_styles: Vec<String>,
    /// The sections of the song.
    #[serde(default)]
    pub sections: Vec<SongSection>,
}

impl MusicPrompt {
    pub fn builder() -> MusicPromptBuilder {
        <MusicPromptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MusicPromptBuilder {
    positive_global_styles: Option<Vec<String>>,
    negative_global_styles: Option<Vec<String>>,
    sections: Option<Vec<SongSection>>,
}

impl MusicPromptBuilder {
    pub fn positive_global_styles(mut self, value: Vec<String>) -> Self {
        self.positive_global_styles = Some(value);
        self
    }

    pub fn negative_global_styles(mut self, value: Vec<String>) -> Self {
        self.negative_global_styles = Some(value);
        self
    }

    pub fn sections(mut self, value: Vec<SongSection>) -> Self {
        self.sections = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MusicPrompt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`positive_global_styles`](MusicPromptBuilder::positive_global_styles)
    /// - [`negative_global_styles`](MusicPromptBuilder::negative_global_styles)
    /// - [`sections`](MusicPromptBuilder::sections)
    pub fn build(self) -> Result<MusicPrompt, BuildError> {
        Ok(MusicPrompt {
            positive_global_styles: self.positive_global_styles.ok_or_else(|| BuildError::missing_field("positive_global_styles"))?,
            negative_global_styles: self.negative_global_styles.ok_or_else(|| BuildError::missing_field("negative_global_styles"))?,
            sections: self.sections.ok_or_else(|| BuildError::missing_field("sections"))?,
        })
    }
}
