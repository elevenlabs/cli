pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WidgetTextContentsTranslation {
    /// The source text each translated field was derived from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<HashMap<String, String>>,
    /// The last auto-translated output for each translated field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<HashMap<String, String>>,
}

impl WidgetTextContentsTranslation {
    pub fn builder() -> WidgetTextContentsTranslationBuilder {
        <WidgetTextContentsTranslationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WidgetTextContentsTranslationBuilder {
    source: Option<HashMap<String, String>>,
    text: Option<HashMap<String, String>>,
}

impl WidgetTextContentsTranslationBuilder {
    pub fn source(mut self, value: HashMap<String, String>) -> Self {
        self.source = Some(value);
        self
    }

    pub fn text(mut self, value: HashMap<String, String>) -> Self {
        self.text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WidgetTextContentsTranslation`].
    pub fn build(self) -> Result<WidgetTextContentsTranslation, BuildError> {
        Ok(WidgetTextContentsTranslation {
            source: self.source,
            text: self.text,
        })
    }
}
