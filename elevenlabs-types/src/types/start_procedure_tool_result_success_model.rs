pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StartProcedureToolResultSuccessModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub procedure_id: String,
    #[serde(default)]
    pub procedure_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_entry_workflow_node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_return_workflow_node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl StartProcedureToolResultSuccessModel {
    pub fn builder() -> StartProcedureToolResultSuccessModelBuilder {
        <StartProcedureToolResultSuccessModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartProcedureToolResultSuccessModelBuilder {
    status: Option<String>,
    procedure_id: Option<String>,
    procedure_name: Option<String>,
    procedure_entry_workflow_node: Option<String>,
    procedure_return_workflow_node: Option<String>,
    message: Option<String>,
}

impl StartProcedureToolResultSuccessModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn procedure_id(mut self, value: impl Into<String>) -> Self {
        self.procedure_id = Some(value.into());
        self
    }

    pub fn procedure_name(mut self, value: impl Into<String>) -> Self {
        self.procedure_name = Some(value.into());
        self
    }

    pub fn procedure_entry_workflow_node(mut self, value: impl Into<String>) -> Self {
        self.procedure_entry_workflow_node = Some(value.into());
        self
    }

    pub fn procedure_return_workflow_node(mut self, value: impl Into<String>) -> Self {
        self.procedure_return_workflow_node = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StartProcedureToolResultSuccessModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`procedure_id`](StartProcedureToolResultSuccessModelBuilder::procedure_id)
    /// - [`procedure_name`](StartProcedureToolResultSuccessModelBuilder::procedure_name)
    pub fn build(self) -> Result<StartProcedureToolResultSuccessModel, BuildError> {
        Ok(StartProcedureToolResultSuccessModel {
            status: self.status,
            procedure_id: self.procedure_id.ok_or_else(|| BuildError::missing_field("procedure_id"))?,
            procedure_name: self.procedure_name.ok_or_else(|| BuildError::missing_field("procedure_name"))?,
            procedure_entry_workflow_node: self.procedure_entry_workflow_node,
            procedure_return_workflow_node: self.procedure_return_workflow_node,
            message: self.message,
        })
    }
}
