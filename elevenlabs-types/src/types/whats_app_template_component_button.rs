pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateComponentButton {
    #[serde(default)]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl WhatsAppTemplateComponentButton {
    pub fn builder() -> WhatsAppTemplateComponentButtonBuilder {
        <WhatsAppTemplateComponentButtonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateComponentButtonBuilder {
    r#type: Option<String>,
    text: Option<String>,
    url: Option<String>,
}

impl WhatsAppTemplateComponentButtonBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateComponentButton`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](WhatsAppTemplateComponentButtonBuilder::r#type)
    pub fn build(self) -> Result<WhatsAppTemplateComponentButton, BuildError> {
        Ok(WhatsAppTemplateComponentButton {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            text: self.text,
            url: self.url,
        })
    }
}
