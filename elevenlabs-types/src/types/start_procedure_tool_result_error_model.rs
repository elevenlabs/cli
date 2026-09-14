pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StartProcedureToolResultErrorModel {
    pub status: StartProcedureToolErrorStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_id: Option<String>,
    #[serde(default)]
    pub message: String,
}

impl StartProcedureToolResultErrorModel {
    pub fn builder() -> StartProcedureToolResultErrorModelBuilder {
        <StartProcedureToolResultErrorModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartProcedureToolResultErrorModelBuilder {
    status: Option<StartProcedureToolErrorStatus>,
    procedure_id: Option<String>,
    message: Option<String>,
}

impl StartProcedureToolResultErrorModelBuilder {
    pub fn status(mut self, value: StartProcedureToolErrorStatus) -> Self {
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

    /// Consumes the builder and constructs a [`StartProcedureToolResultErrorModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](StartProcedureToolResultErrorModelBuilder::status)
    /// - [`message`](StartProcedureToolResultErrorModelBuilder::message)
    pub fn build(self) -> Result<StartProcedureToolResultErrorModel, BuildError> {
        Ok(StartProcedureToolResultErrorModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            procedure_id: self.procedure_id,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
