pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EndProcedureToolResultSuccessModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub procedure_id: String,
    #[serde(default)]
    pub procedure_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl EndProcedureToolResultSuccessModel {
    pub fn builder() -> EndProcedureToolResultSuccessModelBuilder {
        <EndProcedureToolResultSuccessModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EndProcedureToolResultSuccessModelBuilder {
    status: Option<String>,
    procedure_id: Option<String>,
    procedure_name: Option<String>,
    message: Option<String>,
}

impl EndProcedureToolResultSuccessModelBuilder {
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

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EndProcedureToolResultSuccessModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`procedure_id`](EndProcedureToolResultSuccessModelBuilder::procedure_id)
    /// - [`procedure_name`](EndProcedureToolResultSuccessModelBuilder::procedure_name)
    pub fn build(self) -> Result<EndProcedureToolResultSuccessModel, BuildError> {
        Ok(EndProcedureToolResultSuccessModel {
            status: self.status,
            procedure_id: self.procedure_id.ok_or_else(|| BuildError::missing_field("procedure_id"))?,
            procedure_name: self.procedure_name.ok_or_else(|| BuildError::missing_field("procedure_name"))?,
            message: self.message,
        })
    }
}
