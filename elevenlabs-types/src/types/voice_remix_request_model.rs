pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceRemixRequestModel {
    /// Description of the changes to make to the voice.
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
    /// Random number that controls the voice generation. Same seed with same inputs produces same voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Controls how closely the AI follows the prompt. Lower numbers give the AI more freedom to be creative, while higher numbers force it to stick more to the prompt. High numbers can cause voice to sound artificial or robotic. We recommend to use longer, more detailed prompts at lower Guidance Scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub guidance_scale: Option<f64>,
    /// Determines whether the Text to Voice previews should be included in the response. If true, only the generated IDs will be returned which can then be streamed via the /v1/text-to-voice/:generated_voice_id/stream endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_previews: Option<bool>,
    /// The remixing session id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remixing_session_id: Option<String>,
    /// The id of the remixing session iteration where these generations should be attached to. If not provided, a new iteration will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remixing_session_iteration_id: Option<String>,
    /// Controls the balance of prompt versus reference audio when generating voice samples. 0 means almost no prompt influence, 1 means almost no reference audio influence. Only supported when using the eleven_ttv_v3 model.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub prompt_strength: Option<f64>,
    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    #[serde(skip)]
    pub output_format: Option<AllowedOutputFormats>,
}

impl VoiceRemixRequestModel {
    pub fn builder() -> VoiceRemixRequestModelBuilder {
        <VoiceRemixRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceRemixRequestModelBuilder {
    voice_description: Option<String>,
    text: Option<String>,
    auto_generate_text: Option<bool>,
    loudness: Option<f64>,
    seed: Option<i64>,
    guidance_scale: Option<f64>,
    stream_previews: Option<bool>,
    remixing_session_id: Option<String>,
    remixing_session_iteration_id: Option<String>,
    prompt_strength: Option<f64>,
    output_format: Option<AllowedOutputFormats>,
}

impl VoiceRemixRequestModelBuilder {
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

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn guidance_scale(mut self, value: f64) -> Self {
        self.guidance_scale = Some(value);
        self
    }

    pub fn stream_previews(mut self, value: bool) -> Self {
        self.stream_previews = Some(value);
        self
    }

    pub fn remixing_session_id(mut self, value: impl Into<String>) -> Self {
        self.remixing_session_id = Some(value.into());
        self
    }

    pub fn remixing_session_iteration_id(mut self, value: impl Into<String>) -> Self {
        self.remixing_session_iteration_id = Some(value.into());
        self
    }

    pub fn prompt_strength(mut self, value: f64) -> Self {
        self.prompt_strength = Some(value);
        self
    }

    pub fn output_format(mut self, value: AllowedOutputFormats) -> Self {
        self.output_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceRemixRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_description`](VoiceRemixRequestModelBuilder::voice_description)
    pub fn build(self) -> Result<VoiceRemixRequestModel, BuildError> {
        Ok(VoiceRemixRequestModel {
            voice_description: self.voice_description.ok_or_else(|| BuildError::missing_field("voice_description"))?,
            text: self.text,
            auto_generate_text: self.auto_generate_text,
            loudness: self.loudness,
            seed: self.seed,
            guidance_scale: self.guidance_scale,
            stream_previews: self.stream_previews,
            remixing_session_id: self.remixing_session_id,
            remixing_session_iteration_id: self.remixing_session_iteration_id,
            prompt_strength: self.prompt_strength,
            output_format: self.output_format,
        })
    }
}

