pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InboundSipTrunkConfigRequestModel {
    /// List of IP addresses that are allowed to use the trunk. Each item in the list can be an individual IP address or a Classless Inter-Domain Routing notation representing a CIDR block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_addresses: Option<Vec<String>>,
    /// List of phone numbers that are allowed to use the trunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_numbers: Option<Vec<String>>,
    /// Whether or not to encrypt media (data layer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_encryption: Option<SipMediaEncryptionEnum>,
    /// Optional digest authentication credentials (username/password).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<SipTrunkCredentialsRequestModel>,
    /// Domains of remote SIP servers used to validate TLS certificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domains: Option<Vec<String>>,
    /// Map of dynamic variable name to header name for attributes_to_headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_headers: Option<HashMap<String, String>>,
}

impl InboundSipTrunkConfigRequestModel {
    pub fn builder() -> InboundSipTrunkConfigRequestModelBuilder {
        <InboundSipTrunkConfigRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InboundSipTrunkConfigRequestModelBuilder {
    allowed_addresses: Option<Vec<String>>,
    allowed_numbers: Option<Vec<String>>,
    media_encryption: Option<SipMediaEncryptionEnum>,
    credentials: Option<SipTrunkCredentialsRequestModel>,
    remote_domains: Option<Vec<String>>,
    attributes_to_headers: Option<HashMap<String, String>>,
}

impl InboundSipTrunkConfigRequestModelBuilder {
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

    pub fn credentials(mut self, value: SipTrunkCredentialsRequestModel) -> Self {
        self.credentials = Some(value);
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

    /// Consumes the builder and constructs a [`InboundSipTrunkConfigRequestModel`].
    pub fn build(self) -> Result<InboundSipTrunkConfigRequestModel, BuildError> {
        Ok(InboundSipTrunkConfigRequestModel {
            allowed_addresses: self.allowed_addresses,
            allowed_numbers: self.allowed_numbers,
            media_encryption: self.media_encryption,
            credentials: self.credentials,
            remote_domains: self.remote_domains,
            attributes_to_headers: self.attributes_to_headers,
        })
    }
}
