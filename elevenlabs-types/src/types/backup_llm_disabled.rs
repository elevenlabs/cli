pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct BackupLlmDisabled {
    pub preference: Option<String>,
}

impl BackupLlmDisabled {
    pub fn builder() -> BackupLlmDisabledBuilder {
        <BackupLlmDisabledBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BackupLlmDisabledBuilder {
    preference: Option<String>,
}

impl BackupLlmDisabledBuilder {
    pub fn preference(mut self, value: impl Into<String>) -> Self {
        self.preference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BackupLlmDisabled`].
    pub fn build(self) -> Result<BackupLlmDisabled, BuildError> {
        Ok(BackupLlmDisabled {
            preference: self.preference,
        })
    }
}
