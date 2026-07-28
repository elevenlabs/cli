pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationConfigOverride {
    /// If enabled audio will not be processed and only text will be used, use to avoid audio pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_only: Option<bool>,
}

impl ConversationConfigOverride {
    pub fn builder() -> ConversationConfigOverrideBuilder {
        <ConversationConfigOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationConfigOverrideBuilder {
    text_only: Option<bool>,
}

impl ConversationConfigOverrideBuilder {
    pub fn text_only(mut self, value: bool) -> Self {
        self.text_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationConfigOverride`].
    pub fn build(self) -> Result<ConversationConfigOverride, BuildError> {
        Ok(ConversationConfigOverride {
            text_only: self.text_only,
        })
    }
}
