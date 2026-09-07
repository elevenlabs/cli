pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OTelScope {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
}

impl OTelScope {
    pub fn builder() -> OTelScopeBuilder {
        <OTelScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OTelScopeBuilder {
    name: Option<String>,
    version: Option<String>,
}

impl OTelScopeBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OTelScope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](OTelScopeBuilder::name)
    /// - [`version`](OTelScopeBuilder::version)
    pub fn build(self) -> Result<OTelScope, BuildError> {
        Ok(OTelScope {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
