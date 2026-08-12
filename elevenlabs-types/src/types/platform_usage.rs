pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Per-category breakdown of ``platform_charge`` (the analogue of ``llm_usage``).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PlatformUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_usage: Option<HashMap<String, PlatformCategoryUsage>>,
}

impl PlatformUsage {
    pub fn builder() -> PlatformUsageBuilder {
        <PlatformUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformUsageBuilder {
    category_usage: Option<HashMap<String, PlatformCategoryUsage>>,
}

impl PlatformUsageBuilder {
    pub fn category_usage(mut self, value: HashMap<String, PlatformCategoryUsage>) -> Self {
        self.category_usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlatformUsage`].
    pub fn build(self) -> Result<PlatformUsage, BuildError> {
        Ok(PlatformUsage {
            category_usage: self.category_usage,
        })
    }
}
