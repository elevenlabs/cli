pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyGenerateCompositionPlanV1MusicPlanPost {
    /// A simple text prompt to compose a plan from.
    #[serde(default)]
    pub prompt: String,
    /// The length of the composition plan to generate in milliseconds. Must be between 3000ms and 600000ms. Optional - if not provided, the model will choose a length based on the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_length_ms: Option<i64>,
    /// An optional composition plan to use as a source for the new composition plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_composition_plan: Option<BodyGenerateCompositionPlanV1MusicPlanPostSourceCompositionPlan>,
    /// The model to use for the generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<MusicModelId>,
}

impl BodyGenerateCompositionPlanV1MusicPlanPost {
    pub fn builder() -> BodyGenerateCompositionPlanV1MusicPlanPostBuilder {
        <BodyGenerateCompositionPlanV1MusicPlanPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyGenerateCompositionPlanV1MusicPlanPostBuilder {
    prompt: Option<String>,
    music_length_ms: Option<i64>,
    source_composition_plan: Option<BodyGenerateCompositionPlanV1MusicPlanPostSourceCompositionPlan>,
    model_id: Option<MusicModelId>,
}

impl BodyGenerateCompositionPlanV1MusicPlanPostBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn music_length_ms(mut self, value: i64) -> Self {
        self.music_length_ms = Some(value);
        self
    }

    pub fn source_composition_plan(mut self, value: BodyGenerateCompositionPlanV1MusicPlanPostSourceCompositionPlan) -> Self {
        self.source_composition_plan = Some(value);
        self
    }

    pub fn model_id(mut self, value: MusicModelId) -> Self {
        self.model_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyGenerateCompositionPlanV1MusicPlanPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](BodyGenerateCompositionPlanV1MusicPlanPostBuilder::prompt)
    pub fn build(self) -> Result<BodyGenerateCompositionPlanV1MusicPlanPost, BuildError> {
        Ok(BodyGenerateCompositionPlanV1MusicPlanPost {
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            music_length_ms: self.music_length_ms,
            source_composition_plan: self.source_composition_plan,
            model_id: self.model_id,
        })
    }
}

