pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Experiment membership recorded on a conversation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExperimentAssignment {
    /// Experiment key.
    #[serde(default)]
    pub key: String,
    /// Variant identifier: the branch id for server_branch, or the client-supplied variant for client_declared.
    #[serde(default)]
    pub variant: String,
    pub source: ExperimentAssignmentSource,
    /// The AgentExperiment id, when the assignment maps to a registered experiment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
}

impl ExperimentAssignment {
    pub fn builder() -> ExperimentAssignmentBuilder {
        <ExperimentAssignmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperimentAssignmentBuilder {
    key: Option<String>,
    variant: Option<String>,
    source: Option<ExperimentAssignmentSource>,
    experiment_id: Option<String>,
}

impl ExperimentAssignmentBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn variant(mut self, value: impl Into<String>) -> Self {
        self.variant = Some(value.into());
        self
    }

    pub fn source(mut self, value: ExperimentAssignmentSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn experiment_id(mut self, value: impl Into<String>) -> Self {
        self.experiment_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExperimentAssignment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](ExperimentAssignmentBuilder::key)
    /// - [`variant`](ExperimentAssignmentBuilder::variant)
    /// - [`source`](ExperimentAssignmentBuilder::source)
    pub fn build(self) -> Result<ExperimentAssignment, BuildError> {
        Ok(ExperimentAssignment {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            variant: self.variant.ok_or_else(|| BuildError::missing_field("variant"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            experiment_id: self.experiment_id,
        })
    }
}
