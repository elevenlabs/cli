pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProcedureRefResponseModel {
    /// Procedure ID
    #[serde(default)]
    pub procedure_id: String,
    /// Version ID of a version of the procedure. None for a procedure never versioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Procedure name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Procedure type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ProcedureType>,
    /// When the agent should use this procedure. Empty string means this is a sub-procedure that should only start when another procedure references it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    /// Tool IDs referenced in the procedure content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_tool_ids: Option<Vec<String>>,
    /// Knowledge base IDs referenced in the procedure content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_kb_ids: Option<Vec<String>>,
    /// Procedure IDs referenced in the procedure content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_procedure_ids: Option<Vec<String>>,
    /// Dynamic variable names used in the procedure content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_dynamic_variables: Option<Vec<String>>,
}

impl ProcedureRefResponseModel {
    pub fn builder() -> ProcedureRefResponseModelBuilder {
        <ProcedureRefResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProcedureRefResponseModelBuilder {
    procedure_id: Option<String>,
    version_id: Option<String>,
    name: Option<String>,
    r#type: Option<ProcedureType>,
    trigger: Option<String>,
    referenced_tool_ids: Option<Vec<String>>,
    referenced_kb_ids: Option<Vec<String>>,
    referenced_procedure_ids: Option<Vec<String>>,
    referenced_dynamic_variables: Option<Vec<String>>,
}

impl ProcedureRefResponseModelBuilder {
    pub fn procedure_id(mut self, value: impl Into<String>) -> Self {
        self.procedure_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ProcedureType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn trigger(mut self, value: impl Into<String>) -> Self {
        self.trigger = Some(value.into());
        self
    }

    pub fn referenced_tool_ids(mut self, value: Vec<String>) -> Self {
        self.referenced_tool_ids = Some(value);
        self
    }

    pub fn referenced_kb_ids(mut self, value: Vec<String>) -> Self {
        self.referenced_kb_ids = Some(value);
        self
    }

    pub fn referenced_procedure_ids(mut self, value: Vec<String>) -> Self {
        self.referenced_procedure_ids = Some(value);
        self
    }

    pub fn referenced_dynamic_variables(mut self, value: Vec<String>) -> Self {
        self.referenced_dynamic_variables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProcedureRefResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`procedure_id`](ProcedureRefResponseModelBuilder::procedure_id)
    pub fn build(self) -> Result<ProcedureRefResponseModel, BuildError> {
        Ok(ProcedureRefResponseModel {
            procedure_id: self.procedure_id.ok_or_else(|| BuildError::missing_field("procedure_id"))?,
            version_id: self.version_id,
            name: self.name,
            r#type: self.r#type,
            trigger: self.trigger,
            referenced_tool_ids: self.referenced_tool_ids,
            referenced_kb_ids: self.referenced_kb_ids,
            referenced_procedure_ids: self.referenced_procedure_ids,
            referenced_dynamic_variables: self.referenced_dynamic_variables,
        })
    }
}
