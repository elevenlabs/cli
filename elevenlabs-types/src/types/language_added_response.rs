pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LanguageAddedResponse {
    #[serde(default)]
    pub version: i64,
}

impl LanguageAddedResponse {
    pub fn builder() -> LanguageAddedResponseBuilder {
        <LanguageAddedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguageAddedResponseBuilder {
    version: Option<i64>,
}

impl LanguageAddedResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LanguageAddedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](LanguageAddedResponseBuilder::version)
    pub fn build(self) -> Result<LanguageAddedResponse, BuildError> {
        Ok(LanguageAddedResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
