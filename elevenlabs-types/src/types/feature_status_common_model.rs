pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeatureStatusCommonModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<bool>,
}

impl FeatureStatusCommonModel {
    pub fn builder() -> FeatureStatusCommonModelBuilder {
        <FeatureStatusCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeatureStatusCommonModelBuilder {
    enabled: Option<bool>,
    used: Option<bool>,
}

impl FeatureStatusCommonModelBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn used(mut self, value: bool) -> Self {
        self.used = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeatureStatusCommonModel`].
    pub fn build(self) -> Result<FeatureStatusCommonModel, BuildError> {
        Ok(FeatureStatusCommonModel {
            enabled: self.enabled,
            used: self.used,
        })
    }
}
