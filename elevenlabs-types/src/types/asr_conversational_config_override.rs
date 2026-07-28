pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsrConversationalConfigOverride {
    /// Keywords to boost prediction probability for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

impl AsrConversationalConfigOverride {
    pub fn builder() -> AsrConversationalConfigOverrideBuilder {
        <AsrConversationalConfigOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsrConversationalConfigOverrideBuilder {
    keywords: Option<Vec<String>>,
}

impl AsrConversationalConfigOverrideBuilder {
    pub fn keywords(mut self, value: Vec<String>) -> Self {
        self.keywords = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AsrConversationalConfigOverride`].
    pub fn build(self) -> Result<AsrConversationalConfigOverride, BuildError> {
        Ok(AsrConversationalConfigOverride {
            keywords: self.keywords,
        })
    }
}
