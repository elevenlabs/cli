pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceDesignRequest {
    /// Description to use for the created voice.
    #[serde(default)]
    pub voice_description: String,
    /// Text to generate, text length has to be between 100 and 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Whether to automatically generate a text suitable for the voice description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_generate_text: Option<bool>,
    /// Controls the volume level of the generated voice. -1 is quietest, 1 is loudest, 0 corresponds to roughly -24 LUFS.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub loudness: Option<f64>,
    /// Higher quality results in better voice output but less variety.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub quality: Option<f64>,
    /// Random number that controls the voice generation. Same seed with same inputs produces same voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Controls how closely the AI follows the prompt. Lower numbers give the AI more freedom to be creative, while higher numbers force it to stick more to the prompt. High numbers can cause voice to sound artificial or robotic. We recommend to use longer, more detailed prompts at lower Guidance Scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub guidance_scale: Option<f64>,
    /// Whether to enhance the voice description using AI to add more detail and improve voice generation quality. When enabled, the system will automatically expand simple prompts into more detailed voice descriptions. Defaults to False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_enhance: Option<bool>,
    /// The output format of the generated audio.
    #[serde(skip)]
    pub output_format: Option<AllowedOutputFormats>,
}

impl VoiceDesignRequest {
    pub fn builder() -> VoiceDesignRequestBuilder {
        <VoiceDesignRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceDesignRequestBuilder {
    voice_description: Option<String>,
    text: Option<String>,
    auto_generate_text: Option<bool>,
    loudness: Option<f64>,
    quality: Option<f64>,
    seed: Option<i64>,
    guidance_scale: Option<f64>,
    should_enhance: Option<bool>,
    output_format: Option<AllowedOutputFormats>,
}

impl VoiceDesignRequestBuilder {
    pub fn voice_description(mut self, value: impl Into<String>) -> Self {
        self.voice_description = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn auto_generate_text(mut self, value: bool) -> Self {
        self.auto_generate_text = Some(value);
        self
    }

    pub fn loudness(mut self, value: f64) -> Self {
        self.loudness = Some(value);
        self
    }

    pub fn quality(mut self, value: f64) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn guidance_scale(mut self, value: f64) -> Self {
        self.guidance_scale = Some(value);
        self
    }

    pub fn should_enhance(mut self, value: bool) -> Self {
        self.should_enhance = Some(value);
        self
    }

    pub fn output_format(mut self, value: AllowedOutputFormats) -> Self {
        self.output_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceDesignRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_description`](VoiceDesignRequestBuilder::voice_description)
    pub fn build(self) -> Result<VoiceDesignRequest, BuildError> {
        Ok(VoiceDesignRequest {
            voice_description: self.voice_description.ok_or_else(|| BuildError::missing_field("voice_description"))?,
            text: self.text,
            auto_generate_text: self.auto_generate_text,
            loudness: self.loudness,
            quality: self.quality,
            seed: self.seed,
            guidance_scale: self.guidance_scale,
            should_enhance: self.should_enhance,
            output_format: self.output_format,
        })
    }
}

