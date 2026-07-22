pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LlmDeprecationInfoModel {
    /// The identifier of the deprecated LLM model.
    pub llm: Llm,
    /// Whether this model is currently deprecated. True if the model is immediately deprecated or within the warning period.
    #[serde(default)]
    pub is_deprecated: bool,
    /// Whether this model is currently in the warning period before deprecation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_in_warning_period: Option<bool>,
    /// Whether traffic is currently being progressively routed to the replacement model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_in_fallback_period: Option<bool>,
    /// Current percentage of traffic being routed to the replacement model (0-100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_percentage: Option<i64>,
    /// The date when the model provider will deprecate this model. Null for immediately deprecated models.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub provider_deprecation_date: Option<DateTime<FixedOffset>>,
    /// The model that replaces this deprecated model. Traffic will be automatically routed to this model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_model: Option<Llm>,
    /// Custom deprecation timing configuration for this model. Null if using the default configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_config: Option<LlmDeprecationConfigModel>,
}

impl LlmDeprecationInfoModel {
    pub fn builder() -> LlmDeprecationInfoModelBuilder {
        <LlmDeprecationInfoModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmDeprecationInfoModelBuilder {
    llm: Option<Llm>,
    is_deprecated: Option<bool>,
    is_in_warning_period: Option<bool>,
    is_in_fallback_period: Option<bool>,
    fallback_percentage: Option<i64>,
    provider_deprecation_date: Option<DateTime<FixedOffset>>,
    replacement_model: Option<Llm>,
    deprecation_config: Option<LlmDeprecationConfigModel>,
}

impl LlmDeprecationInfoModelBuilder {
    pub fn llm(mut self, value: Llm) -> Self {
        self.llm = Some(value);
        self
    }

    pub fn is_deprecated(mut self, value: bool) -> Self {
        self.is_deprecated = Some(value);
        self
    }

    pub fn is_in_warning_period(mut self, value: bool) -> Self {
        self.is_in_warning_period = Some(value);
        self
    }

    pub fn is_in_fallback_period(mut self, value: bool) -> Self {
        self.is_in_fallback_period = Some(value);
        self
    }

    pub fn fallback_percentage(mut self, value: i64) -> Self {
        self.fallback_percentage = Some(value);
        self
    }

    pub fn provider_deprecation_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.provider_deprecation_date = Some(value);
        self
    }

    pub fn replacement_model(mut self, value: Llm) -> Self {
        self.replacement_model = Some(value);
        self
    }

    pub fn deprecation_config(mut self, value: LlmDeprecationConfigModel) -> Self {
        self.deprecation_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmDeprecationInfoModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`llm`](LlmDeprecationInfoModelBuilder::llm)
    /// - [`is_deprecated`](LlmDeprecationInfoModelBuilder::is_deprecated)
    pub fn build(self) -> Result<LlmDeprecationInfoModel, BuildError> {
        Ok(LlmDeprecationInfoModel {
            llm: self.llm.ok_or_else(|| BuildError::missing_field("llm"))?,
            is_deprecated: self.is_deprecated.ok_or_else(|| BuildError::missing_field("is_deprecated"))?,
            is_in_warning_period: self.is_in_warning_period,
            is_in_fallback_period: self.is_in_fallback_period,
            fallback_percentage: self.fallback_percentage,
            provider_deprecation_date: self.provider_deprecation_date,
            replacement_model: self.replacement_model,
            deprecation_config: self.deprecation_config,
        })
    }
}
