pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EndCallTriggerAction {
}

impl EndCallTriggerAction {
    pub fn builder() -> EndCallTriggerActionBuilder {
        <EndCallTriggerActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EndCallTriggerActionBuilder {
}

impl EndCallTriggerActionBuilder {

    /// Consumes the builder and constructs a [`EndCallTriggerAction`].
    pub fn build(self) -> Result<EndCallTriggerAction, BuildError> {
        Ok(EndCallTriggerAction {
        })
    }
}
