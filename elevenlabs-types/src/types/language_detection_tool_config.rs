pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LanguageDetectionToolConfig {
}

impl LanguageDetectionToolConfig {
    pub fn builder() -> LanguageDetectionToolConfigBuilder {
        <LanguageDetectionToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguageDetectionToolConfigBuilder {
}

impl LanguageDetectionToolConfigBuilder {

    /// Consumes the builder and constructs a [`LanguageDetectionToolConfig`].
    pub fn build(self) -> Result<LanguageDetectionToolConfig, BuildError> {
        Ok(LanguageDetectionToolConfig {
        })
    }
}
