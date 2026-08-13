pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct BackupLlmDefault {
    pub preference: Option<String>,
}

impl BackupLlmDefault {
    pub fn builder() -> BackupLlmDefaultBuilder {
        <BackupLlmDefaultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BackupLlmDefaultBuilder {
    preference: Option<String>,
}

impl BackupLlmDefaultBuilder {
    pub fn preference(mut self, value: impl Into<String>) -> Self {
        self.preference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BackupLlmDefault`].
    pub fn build(self) -> Result<BackupLlmDefault, BuildError> {
        Ok(BackupLlmDefault {
            preference: self.preference,
        })
    }
}
