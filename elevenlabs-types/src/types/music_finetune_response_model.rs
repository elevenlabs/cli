pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MusicFinetuneResponseModel {
    /// Unique identifier of the finetune.
    #[serde(default)]
    pub id: String,
    /// Name of the finetune.
    #[serde(default)]
    pub name: String,
    /// Tags associated with the finetune.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Primary musical genre of the finetune.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_genre: Option<String>,
    /// The base music model the finetune was trained on.
    #[serde(default)]
    pub model_id: String,
    /// When the finetune was created (UTC).
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Who can access this finetune: `private` (only you), `workspace` (members of your workspace), `public` (ElevenLabs-curated, available to everyone).
    pub visibility: FinetuneVisibility,
    /// Who created the finetune: `self`, `workspace`, or `elevenlabs`.
    pub created_by: FinetuneCreatedBy,
    /// Training lifecycle status: pending, in_progress, completed, failed, and blocked.
    pub status: MusicFinetuneStatus,
    /// Training progress from 0.0 to 1.0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub training_progress: f64,
    /// Reason the finetune failed or was blocked, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<MusicFinetuneFailureReason>,
}

impl MusicFinetuneResponseModel {
    pub fn builder() -> MusicFinetuneResponseModelBuilder {
        <MusicFinetuneResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MusicFinetuneResponseModelBuilder {
    id: Option<String>,
    name: Option<String>,
    tags: Option<Vec<String>>,
    primary_genre: Option<String>,
    model_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    visibility: Option<FinetuneVisibility>,
    created_by: Option<FinetuneCreatedBy>,
    status: Option<MusicFinetuneStatus>,
    training_progress: Option<f64>,
    failure_reason: Option<MusicFinetuneFailureReason>,
}

impl MusicFinetuneResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn primary_genre(mut self, value: impl Into<String>) -> Self {
        self.primary_genre = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn visibility(mut self, value: FinetuneVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn created_by(mut self, value: FinetuneCreatedBy) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn status(mut self, value: MusicFinetuneStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn training_progress(mut self, value: f64) -> Self {
        self.training_progress = Some(value);
        self
    }

    pub fn failure_reason(mut self, value: MusicFinetuneFailureReason) -> Self {
        self.failure_reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MusicFinetuneResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MusicFinetuneResponseModelBuilder::id)
    /// - [`name`](MusicFinetuneResponseModelBuilder::name)
    /// - [`tags`](MusicFinetuneResponseModelBuilder::tags)
    /// - [`model_id`](MusicFinetuneResponseModelBuilder::model_id)
    /// - [`created_at`](MusicFinetuneResponseModelBuilder::created_at)
    /// - [`visibility`](MusicFinetuneResponseModelBuilder::visibility)
    /// - [`created_by`](MusicFinetuneResponseModelBuilder::created_by)
    /// - [`status`](MusicFinetuneResponseModelBuilder::status)
    /// - [`training_progress`](MusicFinetuneResponseModelBuilder::training_progress)
    pub fn build(self) -> Result<MusicFinetuneResponseModel, BuildError> {
        Ok(MusicFinetuneResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
            primary_genre: self.primary_genre,
            model_id: self.model_id.ok_or_else(|| BuildError::missing_field("model_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            visibility: self.visibility.ok_or_else(|| BuildError::missing_field("visibility"))?,
            created_by: self.created_by.ok_or_else(|| BuildError::missing_field("created_by"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            training_progress: self.training_progress.ok_or_else(|| BuildError::missing_field("training_progress"))?,
            failure_reason: self.failure_reason,
        })
    }
}
