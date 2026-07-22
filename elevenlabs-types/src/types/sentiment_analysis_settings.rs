pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SentimentAnalysisSettings {
}

impl SentimentAnalysisSettings {
    pub fn builder() -> SentimentAnalysisSettingsBuilder {
        <SentimentAnalysisSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SentimentAnalysisSettingsBuilder {
}

impl SentimentAnalysisSettingsBuilder {

    /// Consumes the builder and constructs a [`SentimentAnalysisSettings`].
    pub fn build(self) -> Result<SentimentAnalysisSettings, BuildError> {
        Ok(SentimentAnalysisSettings {
        })
    }
}
