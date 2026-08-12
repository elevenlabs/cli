pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateTextParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub text: String,
}

impl WhatsAppTemplateTextParam {
    pub fn builder() -> WhatsAppTemplateTextParamBuilder {
        <WhatsAppTemplateTextParamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateTextParamBuilder {
    parameter_name: Option<String>,
    r#type: Option<String>,
    text: Option<String>,
}

impl WhatsAppTemplateTextParamBuilder {
    pub fn parameter_name(mut self, value: impl Into<String>) -> Self {
        self.parameter_name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateTextParam`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](WhatsAppTemplateTextParamBuilder::text)
    pub fn build(self) -> Result<WhatsAppTemplateTextParam, BuildError> {
        Ok(WhatsAppTemplateTextParam {
            parameter_name: self.parameter_name,
            r#type: self.r#type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
