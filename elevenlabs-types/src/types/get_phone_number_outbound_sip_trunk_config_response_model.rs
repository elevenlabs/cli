pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// SIP Trunk configuration details for a phone number
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPhoneNumberOutboundSipTrunkConfigResponseModel {
    /// Hostname or IP the SIP INVITE is sent to
    #[serde(default)]
    pub address: String,
    /// Protocol to use for SIP transport
    pub transport: SipTrunkTransportEnum,
    /// Whether or not to encrypt media (data layer).
    pub media_encryption: SipMediaEncryptionEnum,
    /// SIP headers for INVITE request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// Map of dynamic variable name to header name for attributes_to_headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_headers: Option<HashMap<String, String>>,
    /// Whether authentication credentials are configured
    #[serde(default)]
    pub has_auth_credentials: bool,
    /// SIP trunk username (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Whether a LiveKit SIP outbound trunk is configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_outbound_trunk: Option<bool>,
    /// Media codecs that are offered in the SDP for outbound calls. If empty, all supported codecs are offered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_codecs: Option<Vec<MediaCodec>>,
}

impl GetPhoneNumberOutboundSipTrunkConfigResponseModel {
    pub fn builder() -> GetPhoneNumberOutboundSipTrunkConfigResponseModelBuilder {
        <GetPhoneNumberOutboundSipTrunkConfigResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPhoneNumberOutboundSipTrunkConfigResponseModelBuilder {
    address: Option<String>,
    transport: Option<SipTrunkTransportEnum>,
    media_encryption: Option<SipMediaEncryptionEnum>,
    headers: Option<HashMap<String, String>>,
    attributes_to_headers: Option<HashMap<String, String>>,
    has_auth_credentials: Option<bool>,
    username: Option<String>,
    has_outbound_trunk: Option<bool>,
    enabled_codecs: Option<Vec<MediaCodec>>,
}

impl GetPhoneNumberOutboundSipTrunkConfigResponseModelBuilder {
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn transport(mut self, value: SipTrunkTransportEnum) -> Self {
        self.transport = Some(value);
        self
    }

    pub fn media_encryption(mut self, value: SipMediaEncryptionEnum) -> Self {
        self.media_encryption = Some(value);
        self
    }

    pub fn headers(mut self, value: HashMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn attributes_to_headers(mut self, value: HashMap<String, String>) -> Self {
        self.attributes_to_headers = Some(value);
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

    pub fn has_outbound_trunk(mut self, value: bool) -> Self {
        self.has_outbound_trunk = Some(value);
        self
    }

    pub fn enabled_codecs(mut self, value: Vec<MediaCodec>) -> Self {
        self.enabled_codecs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPhoneNumberOutboundSipTrunkConfigResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](GetPhoneNumberOutboundSipTrunkConfigResponseModelBuilder::address)
    /// - [`transport`](GetPhoneNumberOutboundSipTrunkConfigResponseModelBuilder::transport)
    /// - [`media_encryption`](GetPhoneNumberOutboundSipTrunkConfigResponseModelBuilder::media_encryption)
    /// - [`has_auth_credentials`](GetPhoneNumberOutboundSipTrunkConfigResponseModelBuilder::has_auth_credentials)
    pub fn build(self) -> Result<GetPhoneNumberOutboundSipTrunkConfigResponseModel, BuildError> {
        Ok(GetPhoneNumberOutboundSipTrunkConfigResponseModel {
            address: self.address.ok_or_else(|| BuildError::missing_field("address"))?,
            transport: self.transport.ok_or_else(|| BuildError::missing_field("transport"))?,
            media_encryption: self.media_encryption.ok_or_else(|| BuildError::missing_field("media_encryption"))?,
            headers: self.headers,
            attributes_to_headers: self.attributes_to_headers,
            has_auth_credentials: self.has_auth_credentials.ok_or_else(|| BuildError::missing_field("has_auth_credentials"))?,
            username: self.username,
            has_outbound_trunk: self.has_outbound_trunk,
            enabled_codecs: self.enabled_codecs,
        })
    }
}
