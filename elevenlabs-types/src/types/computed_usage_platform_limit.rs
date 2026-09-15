pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Platform limit with usage computed externally. Example: PVCs use the count_owned_pro_voices_in_billing_group function to compute the usage.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComputedUsagePlatformLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ComputedUsagePlatformLimit {
    pub fn builder() -> ComputedUsagePlatformLimitBuilder {
        <ComputedUsagePlatformLimitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComputedUsagePlatformLimitBuilder {
    limit: Option<i64>,
}

impl ComputedUsagePlatformLimitBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ComputedUsagePlatformLimit`].
    pub fn build(self) -> Result<ComputedUsagePlatformLimit, BuildError> {
        Ok(ComputedUsagePlatformLimit {
            limit: self.limit,
        })
    }
}
