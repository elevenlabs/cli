pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SongSection {
    /// The name of the section. Must be between 1 and 100 characters.
    #[serde(default)]
    pub section_name: String,
    /// The styles and musical directions that should be present in this section. Use English language for best result.
    #[serde(default)]
    pub positive_local_styles: Vec<String>,
    /// The styles and musical directions that should not be present in this section. Use English language for best result.
    #[serde(default)]
    pub negative_local_styles: Vec<String>,
    /// The duration of the section in milliseconds. Must be between 3000ms and 120000ms.
    #[serde(default)]
    pub duration_ms: i64,
    /// The lyrics of the section. Max 30 lines per section and max 200 characters per line.
    #[serde(default)]
    pub lines: Vec<String>,
    /// Optional source to extract the section from. Used for inpainting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_from: Option<SectionSource>,
}

impl SongSection {
    pub fn builder() -> SongSectionBuilder {
        <SongSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SongSectionBuilder {
    section_name: Option<String>,
    positive_local_styles: Option<Vec<String>>,
    negative_local_styles: Option<Vec<String>>,
    duration_ms: Option<i64>,
    lines: Option<Vec<String>>,
    source_from: Option<SectionSource>,
}

impl SongSectionBuilder {
    pub fn section_name(mut self, value: impl Into<String>) -> Self {
        self.section_name = Some(value.into());
        self
    }

    pub fn positive_local_styles(mut self, value: Vec<String>) -> Self {
        self.positive_local_styles = Some(value);
        self
    }

    pub fn negative_local_styles(mut self, value: Vec<String>) -> Self {
        self.negative_local_styles = Some(value);
        self
    }

    pub fn duration_ms(mut self, value: i64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<String>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn source_from(mut self, value: SectionSource) -> Self {
        self.source_from = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SongSection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`section_name`](SongSectionBuilder::section_name)
    /// - [`positive_local_styles`](SongSectionBuilder::positive_local_styles)
    /// - [`negative_local_styles`](SongSectionBuilder::negative_local_styles)
    /// - [`duration_ms`](SongSectionBuilder::duration_ms)
    /// - [`lines`](SongSectionBuilder::lines)
    pub fn build(self) -> Result<SongSection, BuildError> {
        Ok(SongSection {
            section_name: self.section_name.ok_or_else(|| BuildError::missing_field("section_name"))?,
            positive_local_styles: self.positive_local_styles.ok_or_else(|| BuildError::missing_field("positive_local_styles"))?,
            negative_local_styles: self.negative_local_styles.ok_or_else(|| BuildError::missing_field("negative_local_styles"))?,
            duration_ms: self.duration_ms.ok_or_else(|| BuildError::missing_field("duration_ms"))?,
            lines: self.lines.ok_or_else(|| BuildError::missing_field("lines"))?,
            source_from: self.source_from,
        })
    }
}
