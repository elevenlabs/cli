pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateComponent {
    #[serde(default)]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<WhatsAppTemplateComponentButton>>,
}

impl WhatsAppTemplateComponent {
    pub fn builder() -> WhatsAppTemplateComponentBuilder {
        <WhatsAppTemplateComponentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateComponentBuilder {
    r#type: Option<String>,
    format: Option<String>,
    text: Option<String>,
    buttons: Option<Vec<WhatsAppTemplateComponentButton>>,
}

impl WhatsAppTemplateComponentBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.format = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn buttons(mut self, value: Vec<WhatsAppTemplateComponentButton>) -> Self {
        self.buttons = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateComponent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](WhatsAppTemplateComponentBuilder::r#type)
    pub fn build(self) -> Result<WhatsAppTemplateComponent, BuildError> {
        Ok(WhatsAppTemplateComponent {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            format: self.format,
            text: self.text,
            buttons: self.buttons,
        })
    }
}
