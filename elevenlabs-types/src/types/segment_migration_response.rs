pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SegmentMigrationResponse {
    #[serde(default)]
    pub version: i64,
}

impl SegmentMigrationResponse {
    pub fn builder() -> SegmentMigrationResponseBuilder {
        <SegmentMigrationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SegmentMigrationResponseBuilder {
    version: Option<i64>,
}

impl SegmentMigrationResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SegmentMigrationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](SegmentMigrationResponseBuilder::version)
    pub fn build(self) -> Result<SegmentMigrationResponse, BuildError> {
        Ok(SegmentMigrationResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
