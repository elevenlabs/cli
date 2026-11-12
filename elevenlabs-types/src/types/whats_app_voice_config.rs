pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppVoiceConfig {
    /// The approved template that asks the contact for permission to call.
    #[serde(default)]
    pub whatsapp_call_permission_request_template_name: String,
    /// The language code of that template.
    #[serde(default)]
    pub whatsapp_call_permission_request_template_language_code: String,
}

impl WhatsAppVoiceConfig {
    pub fn builder() -> WhatsAppVoiceConfigBuilder {
        <WhatsAppVoiceConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppVoiceConfigBuilder {
    whatsapp_call_permission_request_template_name: Option<String>,
    whatsapp_call_permission_request_template_language_code: Option<String>,
}

impl WhatsAppVoiceConfigBuilder {
    pub fn whatsapp_call_permission_request_template_name(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_call_permission_request_template_name = Some(value.into());
        self
    }

    pub fn whatsapp_call_permission_request_template_language_code(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_call_permission_request_template_language_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppVoiceConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`whatsapp_call_permission_request_template_name`](WhatsAppVoiceConfigBuilder::whatsapp_call_permission_request_template_name)
    /// - [`whatsapp_call_permission_request_template_language_code`](WhatsAppVoiceConfigBuilder::whatsapp_call_permission_request_template_language_code)
    pub fn build(self) -> Result<WhatsAppVoiceConfig, BuildError> {
        Ok(WhatsAppVoiceConfig {
            whatsapp_call_permission_request_template_name: self.whatsapp_call_permission_request_template_name.ok_or_else(|| BuildError::missing_field("whatsapp_call_permission_request_template_name"))?,
            whatsapp_call_permission_request_template_language_code: self.whatsapp_call_permission_request_template_language_code.ok_or_else(|| BuildError::missing_field("whatsapp_call_permission_request_template_language_code"))?,
        })
    }
}
