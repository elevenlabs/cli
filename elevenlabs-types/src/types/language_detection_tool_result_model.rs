pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LanguageDetectionToolResultModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl LanguageDetectionToolResultModel {
    pub fn builder() -> LanguageDetectionToolResultModelBuilder {
        <LanguageDetectionToolResultModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguageDetectionToolResultModelBuilder {
    status: Option<String>,
    reason: Option<String>,
    language: Option<String>,
}

impl LanguageDetectionToolResultModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LanguageDetectionToolResultModel`].
    pub fn build(self) -> Result<LanguageDetectionToolResultModel, BuildError> {
        Ok(LanguageDetectionToolResultModel {
            status: self.status,
            reason: self.reason,
            language: self.language,
        })
    }
}
