pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppTemplateLocationParamDetails {
    #[serde(default)]
    pub latitude: String,
    #[serde(default)]
    pub longitude: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub address: String,
}

impl WhatsAppTemplateLocationParamDetails {
    pub fn builder() -> WhatsAppTemplateLocationParamDetailsBuilder {
        <WhatsAppTemplateLocationParamDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppTemplateLocationParamDetailsBuilder {
    latitude: Option<String>,
    longitude: Option<String>,
    name: Option<String>,
    address: Option<String>,
}

impl WhatsAppTemplateLocationParamDetailsBuilder {
    pub fn latitude(mut self, value: impl Into<String>) -> Self {
        self.latitude = Some(value.into());
        self
    }

    pub fn longitude(mut self, value: impl Into<String>) -> Self {
        self.longitude = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppTemplateLocationParamDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`latitude`](WhatsAppTemplateLocationParamDetailsBuilder::latitude)
    /// - [`longitude`](WhatsAppTemplateLocationParamDetailsBuilder::longitude)
    /// - [`name`](WhatsAppTemplateLocationParamDetailsBuilder::name)
    /// - [`address`](WhatsAppTemplateLocationParamDetailsBuilder::address)
    pub fn build(self) -> Result<WhatsAppTemplateLocationParamDetails, BuildError> {
        Ok(WhatsAppTemplateLocationParamDetails {
            latitude: self.latitude.ok_or_else(|| BuildError::missing_field("latitude"))?,
            longitude: self.longitude.ok_or_else(|| BuildError::missing_field("longitude"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            address: self.address.ok_or_else(|| BuildError::missing_field("address"))?,
        })
    }
}
