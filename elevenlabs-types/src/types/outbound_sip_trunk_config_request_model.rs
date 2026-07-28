pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OutboundSipTrunkConfigRequestModel {
    /// Hostname or IP the SIP INVITE is sent to.
    #[serde(default)]
    pub address: String,
    /// Protocol to use for SIP transport (signalling layer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<SipTrunkTransportEnum>,
    /// Whether or not to encrypt media (data layer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_encryption: Option<SipMediaEncryptionEnum>,
    /// SIP X-* headers for INVITE request. These headers are sent as-is and may help identify this call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// Map of dynamic variable name to header name for attributes_to_headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_headers: Option<HashMap<String, String>>,
    /// Optional digest authentication credentials (username/password). If not provided, ACL authentication is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<SipTrunkCredentialsRequestModel>,
    /// Media codecs that should be offered in the SDP for outbound calls. If empty, all supported codecs are offered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_codecs: Option<Vec<MediaCodec>>,
}

impl OutboundSipTrunkConfigRequestModel {
    pub fn builder() -> OutboundSipTrunkConfigRequestModelBuilder {
        <OutboundSipTrunkConfigRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutboundSipTrunkConfigRequestModelBuilder {
    address: Option<String>,
    transport: Option<SipTrunkTransportEnum>,
    media_encryption: Option<SipMediaEncryptionEnum>,
    headers: Option<HashMap<String, String>>,
    attributes_to_headers: Option<HashMap<String, String>>,
    credentials: Option<SipTrunkCredentialsRequestModel>,
    enabled_codecs: Option<Vec<MediaCodec>>,
}

impl OutboundSipTrunkConfigRequestModelBuilder {
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

    pub fn credentials(mut self, value: SipTrunkCredentialsRequestModel) -> Self {
        self.credentials = Some(value);
        self
    }

    pub fn enabled_codecs(mut self, value: Vec<MediaCodec>) -> Self {
        self.enabled_codecs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OutboundSipTrunkConfigRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](OutboundSipTrunkConfigRequestModelBuilder::address)
    pub fn build(self) -> Result<OutboundSipTrunkConfigRequestModel, BuildError> {
        Ok(OutboundSipTrunkConfigRequestModel {
            address: self.address.ok_or_else(|| BuildError::missing_field("address"))?,
            transport: self.transport,
            media_encryption: self.media_encryption,
            headers: self.headers,
            attributes_to_headers: self.attributes_to_headers,
            credentials: self.credentials,
            enabled_codecs: self.enabled_codecs,
        })
    }
}
