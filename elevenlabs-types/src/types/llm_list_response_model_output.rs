pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmListResponseModelOutput {
    /// List of all available LLM models that can be used with agents.
    #[serde(default)]
    pub llms: Vec<LlmInfoModelOutput>,
    /// The default deprecation timing configuration used for models without a custom override.
    #[serde(default)]
    pub default_deprecation_config: LlmDeprecationConfigModel,
}

impl LlmListResponseModelOutput {
    pub fn builder() -> LlmListResponseModelOutputBuilder {
        <LlmListResponseModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmListResponseModelOutputBuilder {
    llms: Option<Vec<LlmInfoModelOutput>>,
    default_deprecation_config: Option<LlmDeprecationConfigModel>,
}

impl LlmListResponseModelOutputBuilder {
    pub fn llms(mut self, value: Vec<LlmInfoModelOutput>) -> Self {
        self.llms = Some(value);
        self
    }

    pub fn default_deprecation_config(mut self, value: LlmDeprecationConfigModel) -> Self {
        self.default_deprecation_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmListResponseModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`llms`](LlmListResponseModelOutputBuilder::llms)
    /// - [`default_deprecation_config`](LlmListResponseModelOutputBuilder::default_deprecation_config)
    pub fn build(self) -> Result<LlmListResponseModelOutput, BuildError> {
        Ok(LlmListResponseModelOutput {
            llms: self.llms.ok_or_else(|| BuildError::missing_field("llms"))?,
            default_deprecation_config: self.default_deprecation_config.ok_or_else(|| BuildError::missing_field("default_deprecation_config"))?,
        })
    }
}
