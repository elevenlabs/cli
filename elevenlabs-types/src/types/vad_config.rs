pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VadConfig {
}

impl VadConfig {
    pub fn builder() -> VadConfigBuilder {
        <VadConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VadConfigBuilder {
}

impl VadConfigBuilder {

    /// Consumes the builder and constructs a [`VadConfig`].
    pub fn build(self) -> Result<VadConfig, BuildError> {
        Ok(VadConfig {
        })
    }
}
