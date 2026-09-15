pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EndProcedureToolResultErrorModel {
    pub status: EndProcedureToolErrorStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_id: Option<String>,
    #[serde(default)]
    pub message: String,
}

impl EndProcedureToolResultErrorModel {
    pub fn builder() -> EndProcedureToolResultErrorModelBuilder {
        <EndProcedureToolResultErrorModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EndProcedureToolResultErrorModelBuilder {
    status: Option<EndProcedureToolErrorStatus>,
    procedure_id: Option<String>,
    message: Option<String>,
}

impl EndProcedureToolResultErrorModelBuilder {
    pub fn status(mut self, value: EndProcedureToolErrorStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn procedure_id(mut self, value: impl Into<String>) -> Self {
        self.procedure_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EndProcedureToolResultErrorModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](EndProcedureToolResultErrorModelBuilder::status)
    /// - [`message`](EndProcedureToolResultErrorModelBuilder::message)
    pub fn build(self) -> Result<EndProcedureToolResultErrorModel, BuildError> {
        Ok(EndProcedureToolResultErrorModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            procedure_id: self.procedure_id,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
