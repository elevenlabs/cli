pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SegmentUpdateResponse {
    #[serde(default)]
    pub version: i64,
}

impl SegmentUpdateResponse {
    pub fn builder() -> SegmentUpdateResponseBuilder {
        <SegmentUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SegmentUpdateResponseBuilder {
    version: Option<i64>,
}

impl SegmentUpdateResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SegmentUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](SegmentUpdateResponseBuilder::version)
    pub fn build(self) -> Result<SegmentUpdateResponse, BuildError> {
        Ok(SegmentUpdateResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
