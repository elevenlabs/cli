pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPhoneNumberInboundSipTrunkConfigResponseModel {
    /// List of IP addresses that are allowed to use the trunk. Each item in the list can be an individual IP address or a Classless Inter-Domain Routing notation representing a CIDR block.
    #[serde(default)]
    pub allowed_addresses: Vec<String>,
    /// List of phone numbers that are allowed to use the trunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_numbers: Option<Vec<String>>,
    pub media_encryption: SipMediaEncryptionEnum,
    /// Whether authentication credentials are configured
    #[serde(default)]
    pub has_auth_credentials: bool,
    /// SIP trunk username (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Domains of remote SIP servers used to validate TLS certificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domains: Option<Vec<String>>,
    /// Map of dynamic variable name to header name for attributes_to_headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_headers: Option<HashMap<String, String>>,
}

impl GetPhoneNumberInboundSipTrunkConfigResponseModel {
    pub fn builder() -> GetPhoneNumberInboundSipTrunkConfigResponseModelBuilder {
        <GetPhoneNumberInboundSipTrunkConfigResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPhoneNumberInboundSipTrunkConfigResponseModelBuilder {
    allowed_addresses: Option<Vec<String>>,
    allowed_numbers: Option<Vec<String>>,
    media_encryption: Option<SipMediaEncryptionEnum>,
    has_auth_credentials: Option<bool>,
    username: Option<String>,
    remote_domains: Option<Vec<String>>,
    attributes_to_headers: Option<HashMap<String, String>>,
}

impl GetPhoneNumberInboundSipTrunkConfigResponseModelBuilder {
    pub fn allowed_addresses(mut self, value: Vec<String>) -> Self {
        self.allowed_addresses = Some(value);
        self
    }

    pub fn allowed_numbers(mut self, value: Vec<String>) -> Self {
        self.allowed_numbers = Some(value);
        self
    }

    pub fn media_encryption(mut self, value: SipMediaEncryptionEnum) -> Self {
        self.media_encryption = Some(value);
        self
    }

    pub fn has_auth_credentials(mut self, value: bool) -> Self {
        self.has_auth_credentials = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn remote_domains(mut self, value: Vec<String>) -> Self {
        self.remote_domains = Some(value);
        self
    }

    pub fn attributes_to_headers(mut self, value: HashMap<String, String>) -> Self {
        self.attributes_to_headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPhoneNumberInboundSipTrunkConfigResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allowed_addresses`](GetPhoneNumberInboundSipTrunkConfigResponseModelBuilder::allowed_addresses)
    /// - [`media_encryption`](GetPhoneNumberInboundSipTrunkConfigResponseModelBuilder::media_encryption)
    /// - [`has_auth_credentials`](GetPhoneNumberInboundSipTrunkConfigResponseModelBuilder::has_auth_credentials)
    pub fn build(self) -> Result<GetPhoneNumberInboundSipTrunkConfigResponseModel, BuildError> {
        Ok(GetPhoneNumberInboundSipTrunkConfigResponseModel {
            allowed_addresses: self.allowed_addresses.ok_or_else(|| BuildError::missing_field("allowed_addresses"))?,
            allowed_numbers: self.allowed_numbers,
            media_encryption: self.media_encryption.ok_or_else(|| BuildError::missing_field("media_encryption"))?,
            has_auth_credentials: self.has_auth_credentials.ok_or_else(|| BuildError::missing_field("has_auth_credentials"))?,
            username: self.username,
            remote_domains: self.remote_domains,
            attributes_to_headers: self.attributes_to_headers,
        })
    }
}
