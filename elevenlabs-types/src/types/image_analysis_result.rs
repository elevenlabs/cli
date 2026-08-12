pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImageAnalysisResult {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood_and_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composition_notes: Option<String>,
    /// Readable text overlaid or shown in the image, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<ImageSubject>>,
}

impl ImageAnalysisResult {
    pub fn builder() -> ImageAnalysisResultBuilder {
        <ImageAnalysisResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageAnalysisResultBuilder {
    title: Option<String>,
    description: Option<String>,
    content_type: Option<String>,
    mood_and_style: Option<String>,
    composition_notes: Option<String>,
    visible_text: Option<String>,
    subjects: Option<Vec<ImageSubject>>,
}

impl ImageAnalysisResultBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn mood_and_style(mut self, value: impl Into<String>) -> Self {
        self.mood_and_style = Some(value.into());
        self
    }

    pub fn composition_notes(mut self, value: impl Into<String>) -> Self {
        self.composition_notes = Some(value.into());
        self
    }

    pub fn visible_text(mut self, value: impl Into<String>) -> Self {
        self.visible_text = Some(value.into());
        self
    }

    pub fn subjects(mut self, value: Vec<ImageSubject>) -> Self {
        self.subjects = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImageAnalysisResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](ImageAnalysisResultBuilder::title)
    /// - [`description`](ImageAnalysisResultBuilder::description)
    pub fn build(self) -> Result<ImageAnalysisResult, BuildError> {
        Ok(ImageAnalysisResult {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            content_type: self.content_type,
            mood_and_style: self.mood_and_style,
            composition_notes: self.composition_notes,
            visible_text: self.visible_text,
            subjects: self.subjects,
        })
    }
}
