pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DubbingProjectResponse {
    /// Unique identifier of the dubbing project.
    #[serde(default)]
    pub project_id: String,
    /// Lifecycle status of the project: `queued` before the source is picked up, `preparing` while it is transcribed, `ready` once transcription is done and language targets can start, or `failed`. A project is never reported as `processing` — that value belongs to language targets.
    pub status: DubbingProjectResponseStatus,
    /// The free-form string you supplied as `reference` when creating the project, or null if you supplied none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// BCP-47 language tag of the source media (null if auto-detected).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    /// Dubbing model every language target of this project is dubbed with. Fixed at create time and not selectable per language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// Source media metadata, populated once the source has been fetched and decoded (shortly after create, before the project is `ready`); null until then.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<DubbingSourceMediaInfo>,
    /// Identifiers of the language targets under this project. Populated when a single project is fetched, and on create when `target_language` creates one. Always empty in list responses — list the project's language targets instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_ids: Option<Vec<String>>,
    /// IDs of the workspace webhooks notified as this project and its languages reach `ready`, `completed`, or `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_ids: Option<Vec<String>>,
    /// Monotonic counter incremented whenever the source transcript is edited (segment add/edit/delete).
    #[serde(default)]
    pub revision: i64,
    /// Why the project failed; null unless `status` is `failed`. Also null for the few projects that failed before failure reporting was introduced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<DubbingError>,
    /// Non-fatal conditions raised while preparing the source, empty when there are none. Reflects the latest preparation. Conditions raised while dubbing a particular language are reported on that language instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<VoicesNotPermittedWarning>>,
    /// When the project was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// When the project was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl DubbingProjectResponse {
    pub fn builder() -> DubbingProjectResponseBuilder {
        <DubbingProjectResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingProjectResponseBuilder {
    project_id: Option<String>,
    status: Option<DubbingProjectResponseStatus>,
    reference: Option<String>,
    source_language: Option<String>,
    model_id: Option<String>,
    media: Option<DubbingSourceMediaInfo>,
    language_ids: Option<Vec<String>>,
    webhook_ids: Option<Vec<String>>,
    revision: Option<i64>,
    error: Option<DubbingError>,
    warnings: Option<Vec<VoicesNotPermittedWarning>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl DubbingProjectResponseBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: DubbingProjectResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn source_language(mut self, value: impl Into<String>) -> Self {
        self.source_language = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn media(mut self, value: DubbingSourceMediaInfo) -> Self {
        self.media = Some(value);
        self
    }

    pub fn language_ids(mut self, value: Vec<String>) -> Self {
        self.language_ids = Some(value);
        self
    }

    pub fn webhook_ids(mut self, value: Vec<String>) -> Self {
        self.webhook_ids = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
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

    /// Consumes the builder and constructs a [`DubbingProjectResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](DubbingProjectResponseBuilder::project_id)
    /// - [`status`](DubbingProjectResponseBuilder::status)
    /// - [`revision`](DubbingProjectResponseBuilder::revision)
    /// - [`created_at`](DubbingProjectResponseBuilder::created_at)
    /// - [`updated_at`](DubbingProjectResponseBuilder::updated_at)
    pub fn build(self) -> Result<DubbingProjectResponse, BuildError> {
        Ok(DubbingProjectResponse {
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            reference: self.reference,
            source_language: self.source_language,
            model_id: self.model_id,
            media: self.media,
            language_ids: self.language_ids,
            webhook_ids: self.webhook_ids,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            error: self.error,
            warnings: self.warnings,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
