pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTestSuiteInvocationRequestModel {
    /// New name for the test invocation. Set to null to restore the generated title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateTestSuiteInvocationRequestModel {
    pub fn builder() -> UpdateTestSuiteInvocationRequestModelBuilder {
        <UpdateTestSuiteInvocationRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTestSuiteInvocationRequestModelBuilder {
    name: Option<String>,
}

impl UpdateTestSuiteInvocationRequestModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateTestSuiteInvocationRequestModel`].
    pub fn build(self) -> Result<UpdateTestSuiteInvocationRequestModel, BuildError> {
        Ok(UpdateTestSuiteInvocationRequestModel {
            name: self.name,
        })
    }
}

