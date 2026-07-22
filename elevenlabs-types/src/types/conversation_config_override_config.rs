pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationConfigOverrideConfig {
    /// Whether to allow overriding the text_only field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_only: Option<bool>,
}

impl ConversationConfigOverrideConfig {
    pub fn builder() -> ConversationConfigOverrideConfigBuilder {
        <ConversationConfigOverrideConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationConfigOverrideConfigBuilder {
    text_only: Option<bool>,
}

impl ConversationConfigOverrideConfigBuilder {
    pub fn text_only(mut self, value: bool) -> Self {
        self.text_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationConfigOverrideConfig`].
    pub fn build(self) -> Result<ConversationConfigOverrideConfig, BuildError> {
        Ok(ConversationConfigOverrideConfig {
            text_only: self.text_only,
        })
    }
}
