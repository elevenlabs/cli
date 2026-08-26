pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProcedureVersionRef {
    /// Procedure ID
    #[serde(default)]
    pub procedure_id: String,
    /// Version ID of the procedure version.
    #[serde(default)]
    pub version_id: String,
}

impl ProcedureVersionRef {
    pub fn builder() -> ProcedureVersionRefBuilder {
        <ProcedureVersionRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProcedureVersionRefBuilder {
    procedure_id: Option<String>,
    version_id: Option<String>,
}

impl ProcedureVersionRefBuilder {
    pub fn procedure_id(mut self, value: impl Into<String>) -> Self {
        self.procedure_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProcedureVersionRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`procedure_id`](ProcedureVersionRefBuilder::procedure_id)
    /// - [`version_id`](ProcedureVersionRefBuilder::version_id)
    pub fn build(self) -> Result<ProcedureVersionRef, BuildError> {
        Ok(ProcedureVersionRef {
            procedure_id: self.procedure_id.ok_or_else(|| BuildError::missing_field("procedure_id"))?,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
