pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProcedureDraftRef {
    /// Procedure ID
    #[serde(default)]
    pub procedure_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<serde_json::Value>,
}

impl ProcedureDraftRef {
    pub fn builder() -> ProcedureDraftRefBuilder {
        <ProcedureDraftRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProcedureDraftRefBuilder {
    procedure_id: Option<String>,
    version_id: Option<serde_json::Value>,
}

impl ProcedureDraftRefBuilder {
    pub fn procedure_id(mut self, value: impl Into<String>) -> Self {
        self.procedure_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: serde_json::Value) -> Self {
        self.version_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProcedureDraftRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`procedure_id`](ProcedureDraftRefBuilder::procedure_id)
    pub fn build(self) -> Result<ProcedureDraftRef, BuildError> {
        Ok(ProcedureDraftRef {
            procedure_id: self.procedure_id.ok_or_else(|| BuildError::missing_field("procedure_id"))?,
            version_id: self.version_id,
        })
    }
}
