pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplate {
    #[serde(default)]
    pub name: String,
    /// https://developers.facebook.com/documentation/business-messaging/whatsapp/templates/supported-languages
    #[serde(default)]
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_format: Option<WhatsAppTemplateParameterFormat>,
    #[serde(default)]
    pub components: Vec<WhatsAppTemplateComponent>,
    /// https://developers.facebook.com/docs/graph-api/reference/whats-app-business-account/message_templates/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl WhatsAppTemplate {
    pub fn builder() -> WhatsAppTemplateBuilder {
        <WhatsAppTemplateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateBuilder {
    name: Option<String>,
    language: Option<String>,
    parameter_format: Option<WhatsAppTemplateParameterFormat>,
    components: Option<Vec<WhatsAppTemplateComponent>>,
    category: Option<String>,
}

impl WhatsAppTemplateBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn parameter_format(mut self, value: WhatsAppTemplateParameterFormat) -> Self {
        self.parameter_format = Some(value);
        self
    }

    pub fn components(mut self, value: Vec<WhatsAppTemplateComponent>) -> Self {
        self.components = Some(value);
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](WhatsAppTemplateBuilder::name)
    /// - [`language`](WhatsAppTemplateBuilder::language)
    /// - [`components`](WhatsAppTemplateBuilder::components)
    pub fn build(self) -> Result<WhatsAppTemplate, BuildError> {
        Ok(WhatsAppTemplate {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            parameter_format: self.parameter_format,
            components: self.components.ok_or_else(|| BuildError::missing_field("components"))?,
            category: self.category,
        })
    }
}
