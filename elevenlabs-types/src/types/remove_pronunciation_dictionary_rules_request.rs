pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemovePronunciationDictionaryRulesRequest {
    /// List of strings to remove from the pronunciation dictionary.
    #[serde(default)]
    pub rule_strings: Vec<String>,
}

impl RemovePronunciationDictionaryRulesRequest {
    pub fn builder() -> RemovePronunciationDictionaryRulesRequestBuilder {
        <RemovePronunciationDictionaryRulesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemovePronunciationDictionaryRulesRequestBuilder {
    rule_strings: Option<Vec<String>>,
}

impl RemovePronunciationDictionaryRulesRequestBuilder {
    pub fn rule_strings(mut self, value: Vec<String>) -> Self {
        self.rule_strings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemovePronunciationDictionaryRulesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rule_strings`](RemovePronunciationDictionaryRulesRequestBuilder::rule_strings)
    pub fn build(self) -> Result<RemovePronunciationDictionaryRulesRequest, BuildError> {
        Ok(RemovePronunciationDictionaryRulesRequest {
            rule_strings: self.rule_strings.ok_or_else(|| BuildError::missing_field("rule_strings"))?,
        })
    }
}

