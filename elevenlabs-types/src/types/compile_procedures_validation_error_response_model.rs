pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompileProceduresValidationErrorResponseModel {
    /// Validation errors keyed by procedure ID.
    #[serde(default)]
    pub errors: HashMap<String, Vec<ProcedureValidationError>>,
}

impl CompileProceduresValidationErrorResponseModel {
    pub fn builder() -> CompileProceduresValidationErrorResponseModelBuilder {
        <CompileProceduresValidationErrorResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompileProceduresValidationErrorResponseModelBuilder {
    errors: Option<HashMap<String, Vec<ProcedureValidationError>>>,
}

impl CompileProceduresValidationErrorResponseModelBuilder {
    pub fn errors(mut self, value: HashMap<String, Vec<ProcedureValidationError>>) -> Self {
        self.errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompileProceduresValidationErrorResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`errors`](CompileProceduresValidationErrorResponseModelBuilder::errors)
    pub fn build(self) -> Result<CompileProceduresValidationErrorResponseModel, BuildError> {
        Ok(CompileProceduresValidationErrorResponseModel {
            errors: self.errors.ok_or_else(|| BuildError::missing_field("errors"))?,
        })
    }
}
