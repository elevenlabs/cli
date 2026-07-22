pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsrConversationalConfigOverrideConfig {
    /// Whether to allow overriding the keywords field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<bool>,
}

impl AsrConversationalConfigOverrideConfig {
    pub fn builder() -> AsrConversationalConfigOverrideConfigBuilder {
        <AsrConversationalConfigOverrideConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsrConversationalConfigOverrideConfigBuilder {
    keywords: Option<bool>,
}

impl AsrConversationalConfigOverrideConfigBuilder {
    pub fn keywords(mut self, value: bool) -> Self {
        self.keywords = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AsrConversationalConfigOverrideConfig`].
    pub fn build(self) -> Result<AsrConversationalConfigOverrideConfig, BuildError> {
        Ok(AsrConversationalConfigOverrideConfig {
            keywords: self.keywords,
        })
    }
}
