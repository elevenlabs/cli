pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationChargingCommonModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_discount: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_burst: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_usage: Option<LlmCategoryUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub llm_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_charge: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_charge: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_charge: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_usage: Option<PlatformUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub platform_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub free_minutes_consumed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub free_llm_dollars_consumed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts_usage: Option<ConversationTtsUsageModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr_usage: Option<ConversationAsrUsageModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<AnalysisCharging>,
}

impl ConversationChargingCommonModel {
    pub fn builder() -> ConversationChargingCommonModelBuilder {
        <ConversationChargingCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationChargingCommonModelBuilder {
    dev_discount: Option<bool>,
    is_burst: Option<bool>,
    tier: Option<String>,
    llm_usage: Option<LlmCategoryUsage>,
    llm_price: Option<f64>,
    llm_charge: Option<i64>,
    call_charge: Option<i64>,
    platform_charge: Option<i64>,
    platform_usage: Option<PlatformUsage>,
    platform_price: Option<f64>,
    free_minutes_consumed: Option<f64>,
    free_llm_dollars_consumed: Option<f64>,
    tts_usage: Option<ConversationTtsUsageModel>,
    asr_usage: Option<ConversationAsrUsageModel>,
    analysis: Option<AnalysisCharging>,
}

impl ConversationChargingCommonModelBuilder {
    pub fn dev_discount(mut self, value: bool) -> Self {
        self.dev_discount = Some(value);
        self
    }

    pub fn is_burst(mut self, value: bool) -> Self {
        self.is_burst = Some(value);
        self
    }

    pub fn tier(mut self, value: impl Into<String>) -> Self {
        self.tier = Some(value.into());
        self
    }

    pub fn llm_usage(mut self, value: LlmCategoryUsage) -> Self {
        self.llm_usage = Some(value);
        self
    }

    pub fn llm_price(mut self, value: f64) -> Self {
        self.llm_price = Some(value);
        self
    }

    pub fn llm_charge(mut self, value: i64) -> Self {
        self.llm_charge = Some(value);
        self
    }

    pub fn call_charge(mut self, value: i64) -> Self {
        self.call_charge = Some(value);
        self
    }

    pub fn platform_charge(mut self, value: i64) -> Self {
        self.platform_charge = Some(value);
        self
    }

    pub fn platform_usage(mut self, value: PlatformUsage) -> Self {
        self.platform_usage = Some(value);
        self
    }

    pub fn platform_price(mut self, value: f64) -> Self {
        self.platform_price = Some(value);
        self
    }

    pub fn free_minutes_consumed(mut self, value: f64) -> Self {
        self.free_minutes_consumed = Some(value);
        self
    }

    pub fn free_llm_dollars_consumed(mut self, value: f64) -> Self {
        self.free_llm_dollars_consumed = Some(value);
        self
    }

    pub fn tts_usage(mut self, value: ConversationTtsUsageModel) -> Self {
        self.tts_usage = Some(value);
        self
    }

    pub fn asr_usage(mut self, value: ConversationAsrUsageModel) -> Self {
        self.asr_usage = Some(value);
        self
    }

    pub fn analysis(mut self, value: AnalysisCharging) -> Self {
        self.analysis = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationChargingCommonModel`].
    pub fn build(self) -> Result<ConversationChargingCommonModel, BuildError> {
        Ok(ConversationChargingCommonModel {
            dev_discount: self.dev_discount,
            is_burst: self.is_burst,
            tier: self.tier,
            llm_usage: self.llm_usage,
            llm_price: self.llm_price,
            llm_charge: self.llm_charge,
            call_charge: self.call_charge,
            platform_charge: self.platform_charge,
            platform_usage: self.platform_usage,
            platform_price: self.platform_price,
            free_minutes_consumed: self.free_minutes_consumed,
            free_llm_dollars_consumed: self.free_llm_dollars_consumed,
            tts_usage: self.tts_usage,
            asr_usage: self.asr_usage,
            analysis: self.analysis,
        })
    }
}
