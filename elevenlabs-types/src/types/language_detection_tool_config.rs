pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LanguageDetectionToolConfig {
    /// If no language switch happens in the first 2 user turns, later attempts fail and the conversation stays in the current language. If the language switches during those turns, later switching stays available. Enable to reduce the possibility of false switching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_at_conversation_start: Option<bool>,
}

impl LanguageDetectionToolConfig {
    pub fn builder() -> LanguageDetectionToolConfigBuilder {
        <LanguageDetectionToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguageDetectionToolConfigBuilder {
    only_at_conversation_start: Option<bool>,
}

impl LanguageDetectionToolConfigBuilder {
    pub fn only_at_conversation_start(mut self, value: bool) -> Self {
        self.only_at_conversation_start = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LanguageDetectionToolConfig`].
    pub fn build(self) -> Result<LanguageDetectionToolConfig, BuildError> {
        Ok(LanguageDetectionToolConfig {
            only_at_conversation_start: self.only_at_conversation_start,
        })
    }
}
