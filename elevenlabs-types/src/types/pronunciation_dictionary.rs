pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PronunciationDictionary {
    /// List of pronunciation rules. Rule can be either:
    /// an alias rule: {'string_to_replace': 'a', 'type': 'alias', 'alias': 'b', }
    /// or a phoneme rule: {'string_to_replace': 'a', 'type': 'phoneme', 'phoneme': 'b', 'alphabet': 'ipa' }
    #[serde(default)]
    pub rules: Vec<PronunciationDictionaryRule>,
}

impl PronunciationDictionary {
    pub fn builder() -> PronunciationDictionaryBuilder {
        <PronunciationDictionaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationDictionaryBuilder {
    rules: Option<Vec<PronunciationDictionaryRule>>,
}

impl PronunciationDictionaryBuilder {
    pub fn rules(mut self, value: Vec<PronunciationDictionaryRule>) -> Self {
        self.rules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PronunciationDictionary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rules`](PronunciationDictionaryBuilder::rules)
    pub fn build(self) -> Result<PronunciationDictionary, BuildError> {
        Ok(PronunciationDictionary {
            rules: self.rules.ok_or_else(|| BuildError::missing_field("rules"))?,
        })
    }
}

