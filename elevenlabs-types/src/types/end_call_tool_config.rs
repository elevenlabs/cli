pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EndCallToolConfig {
}

impl EndCallToolConfig {
    pub fn builder() -> EndCallToolConfigBuilder {
        <EndCallToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EndCallToolConfigBuilder {
}

impl EndCallToolConfigBuilder {

    /// Consumes the builder and constructs a [`EndCallToolConfig`].
    pub fn build(self) -> Result<EndCallToolConfig, BuildError> {
        Ok(EndCallToolConfig {
        })
    }
}
