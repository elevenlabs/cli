pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SegmentCreateResponse {
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub new_segment: String,
}

impl SegmentCreateResponse {
    pub fn builder() -> SegmentCreateResponseBuilder {
        <SegmentCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SegmentCreateResponseBuilder {
    version: Option<i64>,
    new_segment: Option<String>,
}

impl SegmentCreateResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn new_segment(mut self, value: impl Into<String>) -> Self {
        self.new_segment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SegmentCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](SegmentCreateResponseBuilder::version)
    /// - [`new_segment`](SegmentCreateResponseBuilder::new_segment)
    pub fn build(self) -> Result<SegmentCreateResponse, BuildError> {
        Ok(SegmentCreateResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            new_segment: self.new_segment.ok_or_else(|| BuildError::missing_field("new_segment"))?,
        })
    }
}
