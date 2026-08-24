pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldConflict {
    /// Identifier of the conflicting field relative to its section: a dot-path within conversation_config/platform_settings, or a procedure id.
    #[serde(default)]
    pub path: String,
    /// Which config section this path belongs to.
    pub section: ConflictSection,
    /// Value at the common ancestor (merge base).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_value: Option<serde_json::Value>,
    /// Value on the source branch tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_value: Option<serde_json::Value>,
    /// Value on the target branch tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<serde_json::Value>,
}

impl FieldConflict {
    pub fn builder() -> FieldConflictBuilder {
        <FieldConflictBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FieldConflictBuilder {
    path: Option<String>,
    section: Option<ConflictSection>,
    base_value: Option<serde_json::Value>,
    source_value: Option<serde_json::Value>,
    target_value: Option<serde_json::Value>,
}

impl FieldConflictBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn section(mut self, value: ConflictSection) -> Self {
        self.section = Some(value);
        self
    }

    pub fn base_value(mut self, value: serde_json::Value) -> Self {
        self.base_value = Some(value);
        self
    }

    pub fn source_value(mut self, value: serde_json::Value) -> Self {
        self.source_value = Some(value);
        self
    }

    pub fn target_value(mut self, value: serde_json::Value) -> Self {
        self.target_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FieldConflict`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](FieldConflictBuilder::path)
    /// - [`section`](FieldConflictBuilder::section)
    pub fn build(self) -> Result<FieldConflict, BuildError> {
        Ok(FieldConflict {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            section: self.section.ok_or_else(|| BuildError::missing_field("section"))?,
            base_value: self.base_value,
            source_value: self.source_value,
            target_value: self.target_value,
        })
    }
}
