pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LlmInfoModelOutput {
    /// The model identifier.
    pub llm: Llm,
    /// Whether this is a pinned checkpoint version of a model rather than a top-level alias.
    #[serde(default)]
    pub is_checkpoint: bool,
    /// Maximum number of output tokens the model can generate.
    #[serde(default)]
    pub max_tokens_limit: i64,
    /// Maximum number of input context tokens the model supports.
    #[serde(default)]
    pub max_context_limit: i64,
    /// Whether the model supports image file inputs during conversations.
    #[serde(default)]
    pub supports_image_input: bool,
    /// Whether the model supports document (PDF) file inputs during conversations.
    #[serde(default)]
    pub supports_document_input: bool,
    /// Whether the model supports calling multiple tools in parallel.
    #[serde(default)]
    pub supports_parallel_tool_calls: bool,
    /// Available reasoning effort levels for this model. Null if the model does not support configurable reasoning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_reasoning_efforts: Option<Vec<LlmReasoningEffort>>,
    /// Deprecation information if this model is deprecated or scheduled for deprecation. Null if the model is not affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_info: Option<LlmDeprecationInfoModel>,
    /// Regional processing surcharge details if this model has additional costs in the current deployment region. Null if no surcharge applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_processing_surcharge: Option<RegionalProcessingSurchargeInfo>,
}

impl LlmInfoModelOutput {
    pub fn builder() -> LlmInfoModelOutputBuilder {
        <LlmInfoModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmInfoModelOutputBuilder {
    llm: Option<Llm>,
    is_checkpoint: Option<bool>,
    max_tokens_limit: Option<i64>,
    max_context_limit: Option<i64>,
    supports_image_input: Option<bool>,
    supports_document_input: Option<bool>,
    supports_parallel_tool_calls: Option<bool>,
    available_reasoning_efforts: Option<Vec<LlmReasoningEffort>>,
    deprecation_info: Option<LlmDeprecationInfoModel>,
    regional_processing_surcharge: Option<RegionalProcessingSurchargeInfo>,
}

impl LlmInfoModelOutputBuilder {
    pub fn llm(mut self, value: Llm) -> Self {
        self.llm = Some(value);
        self
    }

    pub fn is_checkpoint(mut self, value: bool) -> Self {
        self.is_checkpoint = Some(value);
        self
    }

    pub fn max_tokens_limit(mut self, value: i64) -> Self {
        self.max_tokens_limit = Some(value);
        self
    }

    pub fn max_context_limit(mut self, value: i64) -> Self {
        self.max_context_limit = Some(value);
        self
    }

    pub fn supports_image_input(mut self, value: bool) -> Self {
        self.supports_image_input = Some(value);
        self
    }

    pub fn supports_document_input(mut self, value: bool) -> Self {
        self.supports_document_input = Some(value);
        self
    }

    pub fn supports_parallel_tool_calls(mut self, value: bool) -> Self {
        self.supports_parallel_tool_calls = Some(value);
        self
    }

    pub fn available_reasoning_efforts(mut self, value: Vec<LlmReasoningEffort>) -> Self {
        self.available_reasoning_efforts = Some(value);
        self
    }

    pub fn deprecation_info(mut self, value: LlmDeprecationInfoModel) -> Self {
        self.deprecation_info = Some(value);
        self
    }

    pub fn regional_processing_surcharge(mut self, value: RegionalProcessingSurchargeInfo) -> Self {
        self.regional_processing_surcharge = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmInfoModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`llm`](LlmInfoModelOutputBuilder::llm)
    /// - [`is_checkpoint`](LlmInfoModelOutputBuilder::is_checkpoint)
    /// - [`max_tokens_limit`](LlmInfoModelOutputBuilder::max_tokens_limit)
    /// - [`max_context_limit`](LlmInfoModelOutputBuilder::max_context_limit)
    /// - [`supports_image_input`](LlmInfoModelOutputBuilder::supports_image_input)
    /// - [`supports_document_input`](LlmInfoModelOutputBuilder::supports_document_input)
    /// - [`supports_parallel_tool_calls`](LlmInfoModelOutputBuilder::supports_parallel_tool_calls)
    pub fn build(self) -> Result<LlmInfoModelOutput, BuildError> {
        Ok(LlmInfoModelOutput {
            llm: self.llm.ok_or_else(|| BuildError::missing_field("llm"))?,
            is_checkpoint: self.is_checkpoint.ok_or_else(|| BuildError::missing_field("is_checkpoint"))?,
            max_tokens_limit: self.max_tokens_limit.ok_or_else(|| BuildError::missing_field("max_tokens_limit"))?,
            max_context_limit: self.max_context_limit.ok_or_else(|| BuildError::missing_field("max_context_limit"))?,
            supports_image_input: self.supports_image_input.ok_or_else(|| BuildError::missing_field("supports_image_input"))?,
            supports_document_input: self.supports_document_input.ok_or_else(|| BuildError::missing_field("supports_document_input"))?,
            supports_parallel_tool_calls: self.supports_parallel_tool_calls.ok_or_else(|| BuildError::missing_field("supports_parallel_tool_calls"))?,
            available_reasoning_efforts: self.available_reasoning_efforts,
            deprecation_info: self.deprecation_info,
            regional_processing_surcharge: self.regional_processing_surcharge,
        })
    }
}
