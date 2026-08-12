pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SegmentDeleteResponse {
    #[serde(default)]
    pub version: i64,
}

impl SegmentDeleteResponse {
    pub fn builder() -> SegmentDeleteResponseBuilder {
        <SegmentDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SegmentDeleteResponseBuilder {
    version: Option<i64>,
}

impl SegmentDeleteResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SegmentDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](SegmentDeleteResponseBuilder::version)
    pub fn build(self) -> Result<SegmentDeleteResponse, BuildError> {
        Ok(SegmentDeleteResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
