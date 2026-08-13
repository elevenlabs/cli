pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GenerationChunkInput {
    /// The text config to be generated for this chunk. Can contain section name in square brackets, e.g. [Verse 1], lyrics lines, and inline directions in curly braces, e.g. {scratching}.
    #[serde(default)]
    pub text: String,
    /// The duration of the chunk in milliseconds. Must be between 3000ms and 120000ms.
    #[serde(default)]
    pub duration_ms: i64,
    /// The styles and musical directions that should be present in this chunk. Use English language for best results. The styles for the first chunk are the most important as they set the overall tone and genre. Styles for subsequent chunks can be used to add nuance, progression, emphasis, or change the direction of the song. Aim to have at least 6-7 styles in early chunks until the direction is established. Generic styles like 'great production quality' are good default styles to append to the list.
    #[serde(default)]
    pub positive_styles: Vec<String>,
    /// The styles and musical directions that should not be present in this chunk. Use English language for best results. Leaving empty is a good default, only use this field if you want to explicitly avoid a particular style or direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_styles: Option<Vec<String>>,
    /// How much the model adheres to the context of its surrounding chunks. Low adherence means the model can deviate from the context and be more creative. High adherence means the model will be more consistent with the context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_adherence: Option<GenerationChunkInputContextAdherence>,
    /// The audio reference to condition the generation on. The first chunk is the most important as it will influence the generation of all subsequent chunks. Thus, if you want to apply conditioning to the entire song, start conditioning from the first chunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditioning_ref: Option<AudioRefChunk>,
    /// How strongly the model adheres to the conditioning reference. Low strength means the model will be more creative and deviate from the reference. High strength means the model will be more consistent with the reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_strength: Option<GenerationChunkInputConditionStrength>,
}

impl GenerationChunkInput {
    pub fn builder() -> GenerationChunkInputBuilder {
        <GenerationChunkInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerationChunkInputBuilder {
    text: Option<String>,
    duration_ms: Option<i64>,
    positive_styles: Option<Vec<String>>,
    negative_styles: Option<Vec<String>>,
    context_adherence: Option<GenerationChunkInputContextAdherence>,
    conditioning_ref: Option<AudioRefChunk>,
    condition_strength: Option<GenerationChunkInputConditionStrength>,
}

impl GenerationChunkInputBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn duration_ms(mut self, value: i64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn positive_styles(mut self, value: Vec<String>) -> Self {
        self.positive_styles = Some(value);
        self
    }

    pub fn negative_styles(mut self, value: Vec<String>) -> Self {
        self.negative_styles = Some(value);
        self
    }

    pub fn context_adherence(mut self, value: GenerationChunkInputContextAdherence) -> Self {
        self.context_adherence = Some(value);
        self
    }

    pub fn conditioning_ref(mut self, value: AudioRefChunk) -> Self {
        self.conditioning_ref = Some(value);
        self
    }

    pub fn condition_strength(mut self, value: GenerationChunkInputConditionStrength) -> Self {
        self.condition_strength = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GenerationChunkInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](GenerationChunkInputBuilder::text)
    /// - [`duration_ms`](GenerationChunkInputBuilder::duration_ms)
    /// - [`positive_styles`](GenerationChunkInputBuilder::positive_styles)
    pub fn build(self) -> Result<GenerationChunkInput, BuildError> {
        Ok(GenerationChunkInput {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            duration_ms: self.duration_ms.ok_or_else(|| BuildError::missing_field("duration_ms"))?,
            positive_styles: self.positive_styles.ok_or_else(|| BuildError::missing_field("positive_styles"))?,
            negative_styles: self.negative_styles,
            context_adherence: self.context_adherence,
            conditioning_ref: self.conditioning_ref,
            condition_strength: self.condition_strength,
        })
    }
}
