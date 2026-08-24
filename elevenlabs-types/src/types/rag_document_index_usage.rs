pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RagDocumentIndexUsage {
    #[serde(default)]
    pub used_bytes: i64,
}

impl RagDocumentIndexUsage {
    pub fn builder() -> RagDocumentIndexUsageBuilder {
        <RagDocumentIndexUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RagDocumentIndexUsageBuilder {
    used_bytes: Option<i64>,
}

impl RagDocumentIndexUsageBuilder {
    pub fn used_bytes(mut self, value: i64) -> Self {
        self.used_bytes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RagDocumentIndexUsage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`used_bytes`](RagDocumentIndexUsageBuilder::used_bytes)
    pub fn build(self) -> Result<RagDocumentIndexUsage, BuildError> {
        Ok(RagDocumentIndexUsage {
            used_bytes: self.used_bytes.ok_or_else(|| BuildError::missing_field("used_bytes"))?,
        })
    }
}
