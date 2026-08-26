pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyComposeMusicV1MusicPost {
    /// A simple text prompt to generate a song from. Cannot be used in conjunction with `composition_plan`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// A detailed composition plan to guide music generation. Cannot be used in conjunction with `prompt`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composition_plan: Option<BodyComposeMusicV1MusicPostCompositionPlan>,
    /// The length of the song to generate in milliseconds. Used only in conjunction with `prompt`. Must be between 3000ms and 600000ms. Optional - if not provided, the model will choose a length based on the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_length_ms: Option<i64>,
    /// The model to use for the generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<MusicModelId>,
    /// Random seed to initialize the music generation process. Providing the same seed with the same parameters can help achieve more consistent results, but exact reproducibility is not guaranteed and outputs may change across system updates. Cannot be used in conjunction with prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// If true, guarantees that the generated song will be instrumental. If false, the song may or may not be instrumental depending on the `prompt`. Can only be used with `prompt`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_instrumental: Option<bool>,
    /// The ID of the finetune to use for the generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finetune_id: Option<String>,
    /// Controls how strictly section durations in the `composition_plan` are enforced. Only used with `composition_plan` and only applies to `music_v1`; for `music_v2` section durations are always enforced and this is ignored. When false for `music_v1`, the model may adjust individual section durations for better quality and latency, while preserving the total song duration from the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respect_sections_durations: Option<bool>,
    /// Whether to store the generated song for inpainting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_for_inpainting: Option<bool>,
    /// Whether to sign the generated song with C2PA. Applicable only for mp3 files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_with_c2pa: Option<bool>,
    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. Use "auto" (the default) to let the API pick the best format for the selected model: mp3_44100_128 for v1 models and mp3_48000_192 for v2 models.
    #[serde(skip)]
    pub output_format: Option<MusicComposeRequestOutputFormat>,
}

impl BodyComposeMusicV1MusicPost {
    pub fn builder() -> BodyComposeMusicV1MusicPostBuilder {
        <BodyComposeMusicV1MusicPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyComposeMusicV1MusicPostBuilder {
    prompt: Option<String>,
    composition_plan: Option<BodyComposeMusicV1MusicPostCompositionPlan>,
    music_length_ms: Option<i64>,
    model_id: Option<MusicModelId>,
    seed: Option<i64>,
    force_instrumental: Option<bool>,
    finetune_id: Option<String>,
    respect_sections_durations: Option<bool>,
    store_for_inpainting: Option<bool>,
    sign_with_c2pa: Option<bool>,
    output_format: Option<MusicComposeRequestOutputFormat>,
}

impl BodyComposeMusicV1MusicPostBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn composition_plan(mut self, value: BodyComposeMusicV1MusicPostCompositionPlan) -> Self {
        self.composition_plan = Some(value);
        self
    }

    pub fn music_length_ms(mut self, value: i64) -> Self {
        self.music_length_ms = Some(value);
        self
    }

    pub fn model_id(mut self, value: MusicModelId) -> Self {
        self.model_id = Some(value);
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn force_instrumental(mut self, value: bool) -> Self {
        self.force_instrumental = Some(value);
        self
    }

    pub fn finetune_id(mut self, value: impl Into<String>) -> Self {
        self.finetune_id = Some(value.into());
        self
    }

    pub fn respect_sections_durations(mut self, value: bool) -> Self {
        self.respect_sections_durations = Some(value);
        self
    }

    pub fn store_for_inpainting(mut self, value: bool) -> Self {
        self.store_for_inpainting = Some(value);
        self
    }

    pub fn sign_with_c2pa(mut self, value: bool) -> Self {
        self.sign_with_c2pa = Some(value);
        self
    }

    pub fn output_format(mut self, value: MusicComposeRequestOutputFormat) -> Self {
        self.output_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyComposeMusicV1MusicPost`].
    pub fn build(self) -> Result<BodyComposeMusicV1MusicPost, BuildError> {
        Ok(BodyComposeMusicV1MusicPost {
            prompt: self.prompt,
            composition_plan: self.composition_plan,
            music_length_ms: self.music_length_ms,
            model_id: self.model_id,
            seed: self.seed,
            force_instrumental: self.force_instrumental,
            finetune_id: self.finetune_id,
            respect_sections_durations: self.respect_sections_durations,
            store_for_inpainting: self.store_for_inpainting,
            sign_with_c2pa: self.sign_with_c2pa,
            output_format: self.output_format,
        })
    }
}

