pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProcedureResponseModel {
    /// ID of the created procedure
    #[serde(default)]
    pub procedure_id: String,
}

impl CreateProcedureResponseModel {
    pub fn builder() -> CreateProcedureResponseModelBuilder {
        <CreateProcedureResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProcedureResponseModelBuilder {
    procedure_id: Option<String>,
}

impl CreateProcedureResponseModelBuilder {
    pub fn procedure_id(mut self, value: impl Into<String>) -> Self {
        self.procedure_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateProcedureResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`procedure_id`](CreateProcedureResponseModelBuilder::procedure_id)
    pub fn build(self) -> Result<CreateProcedureResponseModel, BuildError> {
        Ok(CreateProcedureResponseModel {
            procedure_id: self.procedure_id.ok_or_else(|| BuildError::missing_field("procedure_id"))?,
        })
    }
}
