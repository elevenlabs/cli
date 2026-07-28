pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VadConfigWorkflowOverride {
}

impl VadConfigWorkflowOverride {
    pub fn builder() -> VadConfigWorkflowOverrideBuilder {
        <VadConfigWorkflowOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VadConfigWorkflowOverrideBuilder {
}

impl VadConfigWorkflowOverrideBuilder {

    /// Consumes the builder and constructs a [`VadConfigWorkflowOverride`].
    pub fn build(self) -> Result<VadConfigWorkflowOverride, BuildError> {
        Ok(VadConfigWorkflowOverride {
        })
    }
}
