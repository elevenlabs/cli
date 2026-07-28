pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PronunciationDictionaryAliasRuleRequestModel {
    /// The string to replace. Must be a non-empty string.
    #[serde(default)]
    pub string_to_replace: String,
    /// Whether the rule should match case-sensitively.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    /// Whether the rule should only match at word boundaries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_boundaries: Option<bool>,
    /// The alias for the string to be replaced.
    #[serde(default)]
    pub alias: String,
}

impl PronunciationDictionaryAliasRuleRequestModel {
    pub fn builder() -> PronunciationDictionaryAliasRuleRequestModelBuilder {
        <PronunciationDictionaryAliasRuleRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationDictionaryAliasRuleRequestModelBuilder {
    string_to_replace: Option<String>,
    case_sensitive: Option<bool>,
    word_boundaries: Option<bool>,
    alias: Option<String>,
}

impl PronunciationDictionaryAliasRuleRequestModelBuilder {
    pub fn string_to_replace(mut self, value: impl Into<String>) -> Self {
        self.string_to_replace = Some(value.into());
        self
    }

    pub fn case_sensitive(mut self, value: bool) -> Self {
        self.case_sensitive = Some(value);
        self
    }

    pub fn word_boundaries(mut self, value: bool) -> Self {
        self.word_boundaries = Some(value);
        self
    }

    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PronunciationDictionaryAliasRuleRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`string_to_replace`](PronunciationDictionaryAliasRuleRequestModelBuilder::string_to_replace)
    /// - [`alias`](PronunciationDictionaryAliasRuleRequestModelBuilder::alias)
    pub fn build(self) -> Result<PronunciationDictionaryAliasRuleRequestModel, BuildError> {
        Ok(PronunciationDictionaryAliasRuleRequestModel {
            string_to_replace: self.string_to_replace.ok_or_else(|| BuildError::missing_field("string_to_replace"))?,
            case_sensitive: self.case_sensitive,
            word_boundaries: self.word_boundaries,
            alias: self.alias.ok_or_else(|| BuildError::missing_field("alias"))?,
        })
    }
}
