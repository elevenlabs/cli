pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Voice {
    /// The ID of the voice.
    #[serde(default)]
    pub voice_id: String,
    /// The name of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of samples associated with the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samples: Option<Vec<VoiceSample>>,
    /// The category of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<VoiceCategory>,
    /// Fine-tuning information for the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine_tuning: Option<FineTuningResponse>,
    /// Labels associated with the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// The description of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The preview URL of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
    /// The tiers the voice is available for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_for_tiers: Option<Vec<String>>,
    /// The settings of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<VoiceSettings>,
    /// The sharing information of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing: Option<VoiceSharingResponse>,
    /// The base model IDs for high-quality voices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_quality_base_model_ids: Option<Vec<String>>,
    /// The verified languages of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_languages: Option<Vec<VerifiedVoiceLanguageResponseModel>>,
    /// The IDs of collections this voice belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<Vec<String>>,
    /// The safety controls of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_control: Option<VoiceSafetyControl>,
    /// The voice verification of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_verification: Option<VoiceVerificationResponse>,
    /// The permission on the resource of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_on_resource: Option<String>,
    /// Whether the voice is owned by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owner: Option<bool>,
    /// Whether the voice is legacy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_legacy: Option<bool>,
    /// Whether the voice is mixed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mixed: Option<bool>,
    /// Timestamp when the voice was marked as favorite in Unix time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorited_at_unix: Option<i64>,
    /// The creation time of the voice in Unix time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<i64>,
    /// Whether the voice is bookmarked by the current user. Only relevant for community (library-copied) voices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bookmarked: Option<bool>,
    /// The recording quality of the voice as determined by the review pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_quality: Option<VoiceRecordingQuality>,
    /// The review pipeline status of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labelling_status: Option<VoiceLabellingStatus>,
    /// The reason for the recording quality assessment, as determined by the review pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_quality_reason: Option<String>,
}

impl Voice {
    pub fn builder() -> VoiceBuilder {
        <VoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceBuilder {
    voice_id: Option<String>,
    name: Option<String>,
    samples: Option<Vec<VoiceSample>>,
    category: Option<VoiceCategory>,
    fine_tuning: Option<FineTuningResponse>,
    labels: Option<HashMap<String, String>>,
    description: Option<String>,
    preview_url: Option<String>,
    available_for_tiers: Option<Vec<String>>,
    settings: Option<VoiceSettings>,
    sharing: Option<VoiceSharingResponse>,
    high_quality_base_model_ids: Option<Vec<String>>,
    verified_languages: Option<Vec<VerifiedVoiceLanguageResponseModel>>,
    collection_ids: Option<Vec<String>>,
    safety_control: Option<VoiceSafetyControl>,
    voice_verification: Option<VoiceVerificationResponse>,
    permission_on_resource: Option<String>,
    is_owner: Option<bool>,
    is_legacy: Option<bool>,
    is_mixed: Option<bool>,
    favorited_at_unix: Option<i64>,
    created_at_unix: Option<i64>,
    is_bookmarked: Option<bool>,
    recording_quality: Option<VoiceRecordingQuality>,
    labelling_status: Option<VoiceLabellingStatus>,
    recording_quality_reason: Option<String>,
}

impl VoiceBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn samples(mut self, value: Vec<VoiceSample>) -> Self {
        self.samples = Some(value);
        self
    }

    pub fn category(mut self, value: VoiceCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn fine_tuning(mut self, value: FineTuningResponse) -> Self {
        self.fine_tuning = Some(value);
        self
    }

    pub fn labels(mut self, value: HashMap<String, String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn preview_url(mut self, value: impl Into<String>) -> Self {
        self.preview_url = Some(value.into());
        self
    }

    pub fn available_for_tiers(mut self, value: Vec<String>) -> Self {
        self.available_for_tiers = Some(value);
        self
    }

    pub fn settings(mut self, value: VoiceSettings) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn sharing(mut self, value: VoiceSharingResponse) -> Self {
        self.sharing = Some(value);
        self
    }

    pub fn high_quality_base_model_ids(mut self, value: Vec<String>) -> Self {
        self.high_quality_base_model_ids = Some(value);
        self
    }

    pub fn verified_languages(mut self, value: Vec<VerifiedVoiceLanguageResponseModel>) -> Self {
        self.verified_languages = Some(value);
        self
    }

    pub fn collection_ids(mut self, value: Vec<String>) -> Self {
        self.collection_ids = Some(value);
        self
    }

    pub fn safety_control(mut self, value: VoiceSafetyControl) -> Self {
        self.safety_control = Some(value);
        self
    }

    pub fn voice_verification(mut self, value: VoiceVerificationResponse) -> Self {
        self.voice_verification = Some(value);
        self
    }

    pub fn permission_on_resource(mut self, value: impl Into<String>) -> Self {
        self.permission_on_resource = Some(value.into());
        self
    }

    pub fn is_owner(mut self, value: bool) -> Self {
        self.is_owner = Some(value);
        self
    }

    pub fn is_legacy(mut self, value: bool) -> Self {
        self.is_legacy = Some(value);
        self
    }

    pub fn is_mixed(mut self, value: bool) -> Self {
        self.is_mixed = Some(value);
        self
    }

    pub fn favorited_at_unix(mut self, value: i64) -> Self {
        self.favorited_at_unix = Some(value);
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn is_bookmarked(mut self, value: bool) -> Self {
        self.is_bookmarked = Some(value);
        self
    }

    pub fn recording_quality(mut self, value: VoiceRecordingQuality) -> Self {
        self.recording_quality = Some(value);
        self
    }

    pub fn labelling_status(mut self, value: VoiceLabellingStatus) -> Self {
        self.labelling_status = Some(value);
        self
    }

    pub fn recording_quality_reason(mut self, value: impl Into<String>) -> Self {
        self.recording_quality_reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Voice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_id`](VoiceBuilder::voice_id)
    pub fn build(self) -> Result<Voice, BuildError> {
        Ok(Voice {
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            name: self.name,
            samples: self.samples,
            category: self.category,
            fine_tuning: self.fine_tuning,
            labels: self.labels,
            description: self.description,
            preview_url: self.preview_url,
            available_for_tiers: self.available_for_tiers,
            settings: self.settings,
            sharing: self.sharing,
            high_quality_base_model_ids: self.high_quality_base_model_ids,
            verified_languages: self.verified_languages,
            collection_ids: self.collection_ids,
            safety_control: self.safety_control,
            voice_verification: self.voice_verification,
            permission_on_resource: self.permission_on_resource,
            is_owner: self.is_owner,
            is_legacy: self.is_legacy,
            is_mixed: self.is_mixed,
            favorited_at_unix: self.favorited_at_unix,
            created_at_unix: self.created_at_unix,
            is_bookmarked: self.is_bookmarked,
            recording_quality: self.recording_quality,
            labelling_status: self.labelling_status,
            recording_quality_reason: self.recording_quality_reason,
        })
    }
}
