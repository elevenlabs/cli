pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationConfigOverride {
    /// If enabled audio will not be processed and only text will be used, use to avoid audio pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_only: Option<bool>,
    /// The maximum duration of a conversation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_seconds: Option<i64>,
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
    max_duration_seconds: Option<i64>,
}

impl ConversationConfigOverrideBuilder {
    pub fn text_only(mut self, value: bool) -> Self {
        self.text_only = Some(value);
        self
    }

    pub fn max_duration_seconds(mut self, value: i64) -> Self {
        self.max_duration_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationConfigOverride`].
    pub fn build(self) -> Result<ConversationConfigOverride, BuildError> {
        Ok(ConversationConfigOverride {
            text_only: self.text_only,
            max_duration_seconds: self.max_duration_seconds,
        })
    }
}
