pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DubbingLanguageResponse {
    /// Unique identifier of the language target.
    #[serde(default)]
    pub language_id: String,
    /// Identifier of the parent dubbing project.
    #[serde(default)]
    pub project_id: String,
    /// BCP-47 language tag this target is dubbed into.
    #[serde(default)]
    pub target_language: String,
    /// Lifecycle status: 'queued' (waiting on the project), 'processing', 'completed', 'stale' (source/transcript changed), or 'failed'.
    pub status: DubbingLanguageResponseStatus,
    /// Effective dubbing model id (target override or project default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// Voice settings applied to the whole language, or null if unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
    /// Signed output URLs; null until the target has produced an output (present once 'completed', and kept while 'stale' -- compare `output_revision` against `revision` to tell whether the output is up to date).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<DubbingLanguageOutputs>,
    /// Monotonic counter incremented whenever this target's transcript changes (a source edit affecting it, or an edit to its translation).
    #[serde(default)]
    pub revision: i64,
    /// The `revision` the current dubbed output was generated from; equal to `revision` when up to date, less than it when 'stale'. Null until a generation has completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_revision: Option<i64>,
    /// Why this language failed; null unless `status` is 'failed', and also null for the few languages that failed before failure reporting was introduced. A code of 'project_failed' means the parent project failed, so read the project for the underlying cause.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<DubbingError>,
    /// Non-fatal conditions raised while dubbing this language, empty when there are none. Reflects the latest generation. Conditions raised while preparing the source are reported on the project instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<VoicesNotPermittedWarning>>,
    /// When the language target was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When the language target was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl DubbingLanguageResponse {
    pub fn builder() -> DubbingLanguageResponseBuilder {
        <DubbingLanguageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingLanguageResponseBuilder {
    language_id: Option<String>,
    project_id: Option<String>,
    target_language: Option<String>,
    status: Option<DubbingLanguageResponseStatus>,
    model_id: Option<String>,
    voice_settings: Option<VoiceSettings>,
    outputs: Option<DubbingLanguageOutputs>,
    revision: Option<i64>,
    output_revision: Option<i64>,
    error: Option<DubbingError>,
    warnings: Option<Vec<VoicesNotPermittedWarning>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl DubbingLanguageResponseBuilder {
    pub fn language_id(mut self, value: impl Into<String>) -> Self {
        self.language_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn target_language(mut self, value: impl Into<String>) -> Self {
        self.target_language = Some(value.into());
        self
    }

    pub fn status(mut self, value: DubbingLanguageResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn voice_settings(mut self, value: VoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn outputs(mut self, value: DubbingLanguageOutputs) -> Self {
        self.outputs = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn output_revision(mut self, value: i64) -> Self {
        self.output_revision = Some(value);
        self
    }

    pub fn error(mut self, value: DubbingError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<VoicesNotPermittedWarning>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingLanguageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language_id`](DubbingLanguageResponseBuilder::language_id)
    /// - [`project_id`](DubbingLanguageResponseBuilder::project_id)
    /// - [`target_language`](DubbingLanguageResponseBuilder::target_language)
    /// - [`status`](DubbingLanguageResponseBuilder::status)
    /// - [`revision`](DubbingLanguageResponseBuilder::revision)
    /// - [`created_at`](DubbingLanguageResponseBuilder::created_at)
    /// - [`updated_at`](DubbingLanguageResponseBuilder::updated_at)
    pub fn build(self) -> Result<DubbingLanguageResponse, BuildError> {
        Ok(DubbingLanguageResponse {
            language_id: self.language_id.ok_or_else(|| BuildError::missing_field("language_id"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            target_language: self.target_language.ok_or_else(|| BuildError::missing_field("target_language"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            model_id: self.model_id,
            voice_settings: self.voice_settings,
            outputs: self.outputs,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            output_revision: self.output_revision,
            error: self.error,
            warnings: self.warnings,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
