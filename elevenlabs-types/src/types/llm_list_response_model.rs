pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmListResponseModel {
    /// List of all available LLM models that can be used with agents.
    #[serde(default)]
    pub llms: Vec<LlmInfoModel>,
    /// The default deprecation timing configuration used for models without a custom override.
    #[serde(default)]
    pub default_deprecation_config: LlmDeprecationConfigModel,
}

impl LlmListResponseModel {
    pub fn builder() -> LlmListResponseModelBuilder {
        <LlmListResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmListResponseModelBuilder {
    llms: Option<Vec<LlmInfoModel>>,
    default_deprecation_config: Option<LlmDeprecationConfigModel>,
}

impl LlmListResponseModelBuilder {
    pub fn llms(mut self, value: Vec<LlmInfoModel>) -> Self {
        self.llms = Some(value);
        self
    }

    pub fn default_deprecation_config(mut self, value: LlmDeprecationConfigModel) -> Self {
        self.default_deprecation_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmListResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`llms`](LlmListResponseModelBuilder::llms)
    /// - [`default_deprecation_config`](LlmListResponseModelBuilder::default_deprecation_config)
    pub fn build(self) -> Result<LlmListResponseModel, BuildError> {
        Ok(LlmListResponseModel {
            llms: self.llms.ok_or_else(|| BuildError::missing_field("llms"))?,
            default_deprecation_config: self.default_deprecation_config.ok_or_else(|| BuildError::missing_field("default_deprecation_config"))?,
        })
    }
}
