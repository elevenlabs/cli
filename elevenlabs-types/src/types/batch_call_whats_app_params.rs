pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchCallWhatsAppParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_phone_number_id: Option<String>,
    #[serde(default)]
    pub whatsapp_call_permission_request_template_name: String,
    #[serde(default)]
    pub whatsapp_call_permission_request_template_language_code: String,
}

impl BatchCallWhatsAppParams {
    pub fn builder() -> BatchCallWhatsAppParamsBuilder {
        <BatchCallWhatsAppParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchCallWhatsAppParamsBuilder {
    whatsapp_phone_number_id: Option<String>,
    whatsapp_call_permission_request_template_name: Option<String>,
    whatsapp_call_permission_request_template_language_code: Option<String>,
}

impl BatchCallWhatsAppParamsBuilder {
    pub fn whatsapp_phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_phone_number_id = Some(value.into());
        self
    }

    pub fn whatsapp_call_permission_request_template_name(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_call_permission_request_template_name = Some(value.into());
        self
    }

    pub fn whatsapp_call_permission_request_template_language_code(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_call_permission_request_template_language_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchCallWhatsAppParams`].
    /// This method will fail if any of the following fields are not set:
    /// - [`whatsapp_call_permission_request_template_name`](BatchCallWhatsAppParamsBuilder::whatsapp_call_permission_request_template_name)
    /// - [`whatsapp_call_permission_request_template_language_code`](BatchCallWhatsAppParamsBuilder::whatsapp_call_permission_request_template_language_code)
    pub fn build(self) -> Result<BatchCallWhatsAppParams, BuildError> {
        Ok(BatchCallWhatsAppParams {
            whatsapp_phone_number_id: self.whatsapp_phone_number_id,
            whatsapp_call_permission_request_template_name: self.whatsapp_call_permission_request_template_name.ok_or_else(|| BuildError::missing_field("whatsapp_call_permission_request_template_name"))?,
            whatsapp_call_permission_request_template_language_code: self.whatsapp_call_permission_request_template_language_code.ok_or_else(|| BuildError::missing_field("whatsapp_call_permission_request_template_language_code"))?,
        })
    }
}
