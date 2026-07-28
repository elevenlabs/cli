pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SegmentDubResponse {
    #[serde(default)]
    pub version: i64,
}

impl SegmentDubResponse {
    pub fn builder() -> SegmentDubResponseBuilder {
        <SegmentDubResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SegmentDubResponseBuilder {
    version: Option<i64>,
}

impl SegmentDubResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SegmentDubResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](SegmentDubResponseBuilder::version)
    pub fn build(self) -> Result<SegmentDubResponse, BuildError> {
        Ok(SegmentDubResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
