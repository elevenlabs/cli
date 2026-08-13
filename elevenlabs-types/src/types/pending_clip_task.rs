pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PendingClipTask {
    pub r#type: PendingClipTaskType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl PendingClipTask {
    pub fn builder() -> PendingClipTaskBuilder {
        <PendingClipTaskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PendingClipTaskBuilder {
    r#type: Option<PendingClipTaskType>,
    progress: Option<f64>,
    started_at_ms: Option<i64>,
    updated_at_ms: Option<i64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl PendingClipTaskBuilder {
    pub fn r#type(mut self, value: PendingClipTaskType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn progress(mut self, value: f64) -> Self {
        self.progress = Some(value);
        self
    }

    pub fn started_at_ms(mut self, value: i64) -> Self {
        self.started_at_ms = Some(value);
        self
    }

    pub fn updated_at_ms(mut self, value: i64) -> Self {
        self.updated_at_ms = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PendingClipTask`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PendingClipTaskBuilder::r#type)
    pub fn build(self) -> Result<PendingClipTask, BuildError> {
        Ok(PendingClipTask {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            progress: self.progress,
            started_at_ms: self.started_at_ms,
            updated_at_ms: self.updated_at_ms,
            metadata: self.metadata,
        })
    }
}
