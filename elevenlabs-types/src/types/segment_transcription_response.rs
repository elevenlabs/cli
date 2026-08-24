pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SegmentTranscriptionResponse {
    #[serde(default)]
    pub version: i64,
}

impl SegmentTranscriptionResponse {
    pub fn builder() -> SegmentTranscriptionResponseBuilder {
        <SegmentTranscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SegmentTranscriptionResponseBuilder {
    version: Option<i64>,
}

impl SegmentTranscriptionResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SegmentTranscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](SegmentTranscriptionResponseBuilder::version)
    pub fn build(self) -> Result<SegmentTranscriptionResponse, BuildError> {
        Ok(SegmentTranscriptionResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
