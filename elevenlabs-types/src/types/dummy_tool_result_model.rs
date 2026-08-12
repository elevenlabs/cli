pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DummyToolResultModel {
}

impl DummyToolResultModel {
    pub fn builder() -> DummyToolResultModelBuilder {
        <DummyToolResultModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DummyToolResultModelBuilder {
}

impl DummyToolResultModelBuilder {

    /// Consumes the builder and constructs a [`DummyToolResultModel`].
    pub fn build(self) -> Result<DummyToolResultModel, BuildError> {
        Ok(DummyToolResultModel {
        })
    }
}
