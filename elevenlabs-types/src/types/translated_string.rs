pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TranslatedString {
    #[serde(default)]
    pub value: String,
}

impl TranslatedString {
    pub fn builder() -> TranslatedStringBuilder {
        <TranslatedStringBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslatedStringBuilder {
    value: Option<String>,
}

impl TranslatedStringBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslatedString`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](TranslatedStringBuilder::value)
    pub fn build(self) -> Result<TranslatedString, BuildError> {
        Ok(TranslatedString {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
