pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Platform limit with usage stored in Firestore. Example: credit usage tracked in Firestore.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StoredUsagePlatformLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Current usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
}

impl StoredUsagePlatformLimit {
    pub fn builder() -> StoredUsagePlatformLimitBuilder {
        <StoredUsagePlatformLimitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StoredUsagePlatformLimitBuilder {
    limit: Option<i64>,
    usage: Option<i64>,
}

impl StoredUsagePlatformLimitBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn usage(mut self, value: i64) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StoredUsagePlatformLimit`].
    pub fn build(self) -> Result<StoredUsagePlatformLimit, BuildError> {
        Ok(StoredUsagePlatformLimit {
            limit: self.limit,
            usage: self.usage,
        })
    }
}
