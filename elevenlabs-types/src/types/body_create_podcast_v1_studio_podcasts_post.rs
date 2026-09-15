pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BodyCreatePodcastV1StudioPodcastsPost {
    /// The ID of the model to be used for this Studio project, you can query GET /v1/models to list all available models.
    #[serde(default)]
    pub model_id: String,
    /// The type of podcast to generate. Can be 'conversation', an interaction between two voices, or 'bulletin', a monologue.
    pub mode: BodyCreatePodcastV1StudioPodcastsPostMode,
    /// The source content for the Podcast.
    pub source: BodyCreatePodcastV1StudioPodcastsPostSource,
    /// Output quality of the generated audio. Must be one of:
    /// 'standard' - standard output format, 128kbps with 44.1kHz sample rate.
    /// 'high' - high quality output format, 192kbps with 44.1kHz sample rate and major improvements on our side.
    /// 'ultra' - ultra quality output format, 192kbps with 44.1kHz sample rate and highest improvements on our side.
    /// 'ultra_lossless' - ultra quality output format, 705.6kbps with 44.1kHz sample rate and highest improvements on our side in a fully lossless format.
    /// If not provided, defaults to the highest quality preset available on your subscription tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_preset: Option<QualityPresetType>,
    /// Duration of the generated podcast. Must be one of:
    /// short - produces podcasts shorter than 3 minutes.
    /// default - produces podcasts roughly between 3-7 minutes.
    /// long - produces podcasts longer than 7 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_scale: Option<BodyCreatePodcastV1StudioPodcastsPostDurationScale>,
    /// An optional language of the Studio project. Two-letter language code (ISO 639-1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The intro text that will always be added to the beginning of the podcast.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// The outro text that will always be added to the end of the podcast.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outro: Option<String>,
    /// Additional instructions prompt for the podcast generation used to adjust the podcast's style and tone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_prompt: Option<String>,
    /// A brief summary or highlights of the Studio project's content, providing key points or themes. This should be between 10 and 70 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<String>>,
    /// A url that will be called by our service when the Studio project is converted. Request will contain a json blob containing the status of the conversion
    /// Messages:
    /// 1. When project was converted successfully:
    /// {
    /// type: "project_conversion_status",
    /// event_timestamp: 1234567890,
    /// data: {
    /// request_id: "1234567890",
    /// project_id: "21m00Tcm4TlvDq8ikWAM",
    /// conversion_status: "success",
    /// project_snapshot_id: "22m00Tcm4TlvDq8ikMAT",
    /// error_details: None,
    /// }
    /// }
    /// 2. When project conversion failed:
    /// {
    /// type: "project_conversion_status",
    /// event_timestamp: 1234567890,
    /// data: {
    /// request_id: "1234567890",
    /// project_id: "21m00Tcm4TlvDq8ikWAM",
    /// conversion_status: "error",
    /// project_snapshot_id: None,
    /// error_details: "Error details if conversion failed"
    /// }
    /// }
    ///
    /// 3. When chapter was converted successfully:
    /// {
    /// type: "chapter_conversion_status",
    /// event_timestamp: 1234567890,
    /// data: {
    /// request_id: "1234567890",
    /// project_id: "21m00Tcm4TlvDq8ikWAM",
    /// chapter_id: "22m00Tcm4TlvDq8ikMAT",
    /// conversion_status: "success",
    /// chapter_snapshot_id: "23m00Tcm4TlvDq8ikMAV",
    /// error_details: None,
    /// }
    /// }
    /// 4. When chapter conversion failed:
    /// {
    /// type: "chapter_conversion_status",
    /// event_timestamp: 1234567890,
    /// data: {
    /// request_id: "1234567890",
    /// project_id: "21m00Tcm4TlvDq8ikWAM",
    /// chapter_id: "22m00Tcm4TlvDq8ikMAT",
    /// conversion_status: "error",
    /// chapter_snapshot_id: None,
    /// error_details: "Error details if conversion failed"
    /// }
    /// }
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// This parameter controls text normalization with four modes: 'auto', 'on', 'apply_english' and 'off'.
    /// When set to 'auto', the system will automatically decide whether to apply text normalization
    /// (e.g., spelling out numbers). With 'on', text normalization will always be applied, while
    /// with 'off', it will be skipped. 'apply_english' is the same as 'on' but will assume that text is in English.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization>,
}

impl BodyCreatePodcastV1StudioPodcastsPost {
    pub fn builder() -> BodyCreatePodcastV1StudioPodcastsPostBuilder {
        <BodyCreatePodcastV1StudioPodcastsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreatePodcastV1StudioPodcastsPostBuilder {
    model_id: Option<String>,
    mode: Option<BodyCreatePodcastV1StudioPodcastsPostMode>,
    source: Option<BodyCreatePodcastV1StudioPodcastsPostSource>,
    quality_preset: Option<QualityPresetType>,
    duration_scale: Option<BodyCreatePodcastV1StudioPodcastsPostDurationScale>,
    language: Option<String>,
    intro: Option<String>,
    outro: Option<String>,
    instructions_prompt: Option<String>,
    highlights: Option<Vec<String>>,
    callback_url: Option<String>,
    apply_text_normalization: Option<BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization>,
}

impl BodyCreatePodcastV1StudioPodcastsPostBuilder {
    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: BodyCreatePodcastV1StudioPodcastsPostMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn source(mut self, value: BodyCreatePodcastV1StudioPodcastsPostSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn quality_preset(mut self, value: QualityPresetType) -> Self {
        self.quality_preset = Some(value);
        self
    }

    pub fn duration_scale(mut self, value: BodyCreatePodcastV1StudioPodcastsPostDurationScale) -> Self {
        self.duration_scale = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn intro(mut self, value: impl Into<String>) -> Self {
        self.intro = Some(value.into());
        self
    }

    pub fn outro(mut self, value: impl Into<String>) -> Self {
        self.outro = Some(value.into());
        self
    }

    pub fn instructions_prompt(mut self, value: impl Into<String>) -> Self {
        self.instructions_prompt = Some(value.into());
        self
    }

    pub fn highlights(mut self, value: Vec<String>) -> Self {
        self.highlights = Some(value);
        self
    }

    pub fn callback_url(mut self, value: impl Into<String>) -> Self {
        self.callback_url = Some(value.into());
        self
    }

    pub fn apply_text_normalization(mut self, value: BodyCreatePodcastV1StudioPodcastsPostApplyTextNormalization) -> Self {
        self.apply_text_normalization = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreatePodcastV1StudioPodcastsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model_id`](BodyCreatePodcastV1StudioPodcastsPostBuilder::model_id)
    /// - [`mode`](BodyCreatePodcastV1StudioPodcastsPostBuilder::mode)
    /// - [`source`](BodyCreatePodcastV1StudioPodcastsPostBuilder::source)
    pub fn build(self) -> Result<BodyCreatePodcastV1StudioPodcastsPost, BuildError> {
        Ok(BodyCreatePodcastV1StudioPodcastsPost {
            model_id: self.model_id.ok_or_else(|| BuildError::missing_field("model_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            quality_preset: self.quality_preset,
            duration_scale: self.duration_scale,
            language: self.language,
            intro: self.intro,
            outro: self.outro,
            instructions_prompt: self.instructions_prompt,
            highlights: self.highlights,
            callback_url: self.callback_url,
            apply_text_normalization: self.apply_text_normalization,
        })
    }
}

