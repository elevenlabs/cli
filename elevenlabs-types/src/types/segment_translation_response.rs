pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SegmentTranslationResponse {
    #[serde(default)]
    pub version: i64,
}

impl SegmentTranslationResponse {
    pub fn builder() -> SegmentTranslationResponseBuilder {
        <SegmentTranslationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SegmentTranslationResponseBuilder {
    version: Option<i64>,
}

impl SegmentTranslationResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SegmentTranslationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](SegmentTranslationResponseBuilder::version)
    pub fn build(self) -> Result<SegmentTranslationResponse, BuildError> {
        Ok(SegmentTranslationResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
