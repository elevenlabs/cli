pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// User-to-User Information envelope for SIP REFER transfers (RFC 7433).
/// 
/// Outbound payloads are hex-encoded (the only encoding RFC 7433 defines). The
/// protocol discriminator axis lets per-platform formats (Talkdesk, Genesys, ...)
/// be expressed by configuration rather than scattered transfer flags. Further
/// axes (ASCII encoding, header name, purpose/content parameters) can be added
/// here without touching the transfer model.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UuiTransferConfig {
    /// UUI payload to send on SIP REFER transfers. Supports inline dynamic variables and is hex-encoded at transfer time.
    #[serde(default)]
    pub data: String,
    /// Optional one-octet protocol discriminator (two hex digits, e.g. '00'). Required by platforms such as Genesys Cloud, which otherwise strip the first octet of the payload. Leave unset for platforms like Talkdesk that expect a bare hex payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_discriminator: Option<String>,
    /// How to attach protocol_discriminator. 'prefix' prepends the octet to the hex payload (User-to-User=XX<hex>;encoding=hex). 'pd_parameter' sends it as a separate parameter (User-to-User=<hex>;pd=XX;encoding=hex). Ignored when protocol_discriminator is unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_discriminator_mode: Option<UuiTransferConfigProtocolDiscriminatorMode>,
}

impl UuiTransferConfig {
    pub fn builder() -> UuiTransferConfigBuilder {
        <UuiTransferConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UuiTransferConfigBuilder {
    data: Option<String>,
    protocol_discriminator: Option<String>,
    protocol_discriminator_mode: Option<UuiTransferConfigProtocolDiscriminatorMode>,
}

impl UuiTransferConfigBuilder {
    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
        self
    }

    pub fn protocol_discriminator(mut self, value: impl Into<String>) -> Self {
        self.protocol_discriminator = Some(value.into());
        self
    }

    pub fn protocol_discriminator_mode(mut self, value: UuiTransferConfigProtocolDiscriminatorMode) -> Self {
        self.protocol_discriminator_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UuiTransferConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](UuiTransferConfigBuilder::data)
    pub fn build(self) -> Result<UuiTransferConfig, BuildError> {
        Ok(UuiTransferConfig {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            protocol_discriminator: self.protocol_discriminator,
            protocol_discriminator_mode: self.protocol_discriminator_mode,
        })
    }
}
