pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChapterContentParagraphTtsNodeInputModel {
    pub r#type: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub voice_id: String,
}

impl ChapterContentParagraphTtsNodeInputModel {
    pub fn builder() -> ChapterContentParagraphTtsNodeInputModelBuilder {
        <ChapterContentParagraphTtsNodeInputModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterContentParagraphTtsNodeInputModelBuilder {
    r#type: Option<String>,
    text: Option<String>,
    voice_id: Option<String>,
}

impl ChapterContentParagraphTtsNodeInputModelBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChapterContentParagraphTtsNodeInputModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ChapterContentParagraphTtsNodeInputModelBuilder::r#type)
    /// - [`text`](ChapterContentParagraphTtsNodeInputModelBuilder::text)
    /// - [`voice_id`](ChapterContentParagraphTtsNodeInputModelBuilder::voice_id)
    pub fn build(self) -> Result<ChapterContentParagraphTtsNodeInputModel, BuildError> {
        Ok(ChapterContentParagraphTtsNodeInputModel {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
        })
    }
}
