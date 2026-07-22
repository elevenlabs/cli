pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmListResponseModelInput {
    /// List of all available LLM models that can be used with agents.
    #[serde(default)]
    pub llms: Vec<LlmInfoModelInput>,
    /// The default deprecation timing configuration used for models without a custom override.
    #[serde(default)]
    pub default_deprecation_config: LlmDeprecationConfigModel,
}

impl LlmListResponseModelInput {
    pub fn builder() -> LlmListResponseModelInputBuilder {
        <LlmListResponseModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmListResponseModelInputBuilder {
    llms: Option<Vec<LlmInfoModelInput>>,
    default_deprecation_config: Option<LlmDeprecationConfigModel>,
}

impl LlmListResponseModelInputBuilder {
    pub fn llms(mut self, value: Vec<LlmInfoModelInput>) -> Self {
        self.llms = Some(value);
        self
    }

    pub fn default_deprecation_config(mut self, value: LlmDeprecationConfigModel) -> Self {
        self.default_deprecation_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmListResponseModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`llms`](LlmListResponseModelInputBuilder::llms)
    /// - [`default_deprecation_config`](LlmListResponseModelInputBuilder::default_deprecation_config)
    pub fn build(self) -> Result<LlmListResponseModelInput, BuildError> {
        Ok(LlmListResponseModelInput {
            llms: self.llms.ok_or_else(|| BuildError::missing_field("llms"))?,
            default_deprecation_config: self.default_deprecation_config.ok_or_else(|| BuildError::missing_field("default_deprecation_config"))?,
        })
    }
}
