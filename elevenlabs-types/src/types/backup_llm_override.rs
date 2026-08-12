pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BackupLlmOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<String>,
    #[serde(default)]
    pub order: Vec<Llm>,
}

impl BackupLlmOverride {
    pub fn builder() -> BackupLlmOverrideBuilder {
        <BackupLlmOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BackupLlmOverrideBuilder {
    preference: Option<String>,
    order: Option<Vec<Llm>>,
}

impl BackupLlmOverrideBuilder {
    pub fn preference(mut self, value: impl Into<String>) -> Self {
        self.preference = Some(value.into());
        self
    }

    pub fn order(mut self, value: Vec<Llm>) -> Self {
        self.order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BackupLlmOverride`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order`](BackupLlmOverrideBuilder::order)
    pub fn build(self) -> Result<BackupLlmOverride, BuildError> {
        Ok(BackupLlmOverride {
            preference: self.preference,
            order: self.order.ok_or_else(|| BuildError::missing_field("order"))?,
        })
    }
}
