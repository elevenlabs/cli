pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConflictResolution {
    /// `id` of the conflict this resolves, as the merge or rebase preview reported it.
    #[serde(default)]
    pub conflict_id: String,
    /// Keep the source value, the target value, or `custom_value`. On a list of entities, the side settles the entities both branches changed; ones only one branch changed still merge in.
    pub choice: ConflictResolutionChoice,
    /// Text to keep when `choice` is `custom`, or for a procedure conflict the id of the procedure version to pin. Only conflicts with `accepts_custom_value` take one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_value: Option<String>,
}

impl ConflictResolution {
    pub fn builder() -> ConflictResolutionBuilder {
        <ConflictResolutionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConflictResolutionBuilder {
    conflict_id: Option<String>,
    choice: Option<ConflictResolutionChoice>,
    custom_value: Option<String>,
}

impl ConflictResolutionBuilder {
    pub fn conflict_id(mut self, value: impl Into<String>) -> Self {
        self.conflict_id = Some(value.into());
        self
    }

    pub fn choice(mut self, value: ConflictResolutionChoice) -> Self {
        self.choice = Some(value);
        self
    }

    pub fn custom_value(mut self, value: impl Into<String>) -> Self {
        self.custom_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConflictResolution`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conflict_id`](ConflictResolutionBuilder::conflict_id)
    /// - [`choice`](ConflictResolutionBuilder::choice)
    pub fn build(self) -> Result<ConflictResolution, BuildError> {
        Ok(ConflictResolution {
            conflict_id: self.conflict_id.ok_or_else(|| BuildError::missing_field("conflict_id"))?,
            choice: self.choice.ok_or_else(|| BuildError::missing_field("choice"))?,
            custom_value: self.custom_value,
        })
    }
}
