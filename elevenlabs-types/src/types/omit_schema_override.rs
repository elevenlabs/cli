pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OmitSchemaOverride {
}

impl OmitSchemaOverride {
    pub fn builder() -> OmitSchemaOverrideBuilder {
        <OmitSchemaOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OmitSchemaOverrideBuilder {
}

impl OmitSchemaOverrideBuilder {

    /// Consumes the builder and constructs a [`OmitSchemaOverride`].
    pub fn build(self) -> Result<OmitSchemaOverride, BuildError> {
        Ok(OmitSchemaOverride {
        })
    }
}
