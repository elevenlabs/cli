pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LlmDeprecationConfigModel {
    /// Number of days before the provider deprecation date when warnings start being shown.
    #[serde(default)]
    pub warning_start_days: i64,
    /// Number of days before the provider deprecation date when traffic starts being routed to the replacement model.
    #[serde(default)]
    pub fallback_start_days: i64,
    /// Number of days before the provider deprecation date when all traffic is routed to the replacement model.
    #[serde(default)]
    pub fallback_complete_days: i64,
    /// Percentage of traffic routed to the replacement model when fallback begins.
    #[serde(default)]
    pub fallback_start_percentage: i64,
    /// Percentage of traffic routed to the replacement model when fallback is complete.
    #[serde(default)]
    pub fallback_complete_percentage: i64,
}

impl LlmDeprecationConfigModel {
    pub fn builder() -> LlmDeprecationConfigModelBuilder {
        <LlmDeprecationConfigModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmDeprecationConfigModelBuilder {
    warning_start_days: Option<i64>,
    fallback_start_days: Option<i64>,
    fallback_complete_days: Option<i64>,
    fallback_start_percentage: Option<i64>,
    fallback_complete_percentage: Option<i64>,
}

impl LlmDeprecationConfigModelBuilder {
    pub fn warning_start_days(mut self, value: i64) -> Self {
        self.warning_start_days = Some(value);
        self
    }

    pub fn fallback_start_days(mut self, value: i64) -> Self {
        self.fallback_start_days = Some(value);
        self
    }

    pub fn fallback_complete_days(mut self, value: i64) -> Self {
        self.fallback_complete_days = Some(value);
        self
    }

    pub fn fallback_start_percentage(mut self, value: i64) -> Self {
        self.fallback_start_percentage = Some(value);
        self
    }

    pub fn fallback_complete_percentage(mut self, value: i64) -> Self {
        self.fallback_complete_percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmDeprecationConfigModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`warning_start_days`](LlmDeprecationConfigModelBuilder::warning_start_days)
    /// - [`fallback_start_days`](LlmDeprecationConfigModelBuilder::fallback_start_days)
    /// - [`fallback_complete_days`](LlmDeprecationConfigModelBuilder::fallback_complete_days)
    /// - [`fallback_start_percentage`](LlmDeprecationConfigModelBuilder::fallback_start_percentage)
    /// - [`fallback_complete_percentage`](LlmDeprecationConfigModelBuilder::fallback_complete_percentage)
    pub fn build(self) -> Result<LlmDeprecationConfigModel, BuildError> {
        Ok(LlmDeprecationConfigModel {
            warning_start_days: self.warning_start_days.ok_or_else(|| BuildError::missing_field("warning_start_days"))?,
            fallback_start_days: self.fallback_start_days.ok_or_else(|| BuildError::missing_field("fallback_start_days"))?,
            fallback_complete_days: self.fallback_complete_days.ok_or_else(|| BuildError::missing_field("fallback_complete_days"))?,
            fallback_start_percentage: self.fallback_start_percentage.ok_or_else(|| BuildError::missing_field("fallback_start_percentage"))?,
            fallback_complete_percentage: self.fallback_complete_percentage.ok_or_else(|| BuildError::missing_field("fallback_complete_percentage"))?,
        })
    }
}
