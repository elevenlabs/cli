pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A WhatsApp user by phone number and/or BSUID; the phone wins where one is needed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppRecipient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsuid: Option<String>,
}

impl WhatsAppRecipient {
    pub fn builder() -> WhatsAppRecipientBuilder {
        <WhatsAppRecipientBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppRecipientBuilder {
    phone: Option<String>,
    bsuid: Option<String>,
}

impl WhatsAppRecipientBuilder {
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn bsuid(mut self, value: impl Into<String>) -> Self {
        self.bsuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppRecipient`].
    pub fn build(self) -> Result<WhatsAppRecipient, BuildError> {
        Ok(WhatsAppRecipient {
            phone: self.phone,
            bsuid: self.bsuid,
        })
    }
}
