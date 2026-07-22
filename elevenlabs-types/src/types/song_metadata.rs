pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SongMetadata {
    /// The title of the song
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The description of the song
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The genres of the song
    #[serde(default)]
    pub genres: Vec<String>,
    /// The languages of the song
    #[serde(default)]
    pub languages: Vec<String>,
    /// Whether the song is explicit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_explicit: Option<bool>,
}

impl SongMetadata {
    pub fn builder() -> SongMetadataBuilder {
        <SongMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SongMetadataBuilder {
    title: Option<String>,
    description: Option<String>,
    genres: Option<Vec<String>>,
    languages: Option<Vec<String>>,
    is_explicit: Option<bool>,
}

impl SongMetadataBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn genres(mut self, value: Vec<String>) -> Self {
        self.genres = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn is_explicit(mut self, value: bool) -> Self {
        self.is_explicit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SongMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`genres`](SongMetadataBuilder::genres)
    /// - [`languages`](SongMetadataBuilder::languages)
    pub fn build(self) -> Result<SongMetadata, BuildError> {
        Ok(SongMetadata {
            title: self.title,
            description: self.description,
            genres: self.genres.ok_or_else(|| BuildError::missing_field("genres"))?,
            languages: self.languages.ok_or_else(|| BuildError::missing_field("languages"))?,
            is_explicit: self.is_explicit,
        })
    }
}
