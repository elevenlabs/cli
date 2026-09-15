pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhoneNumberTransfer {
    /// Custom SIP headers to include when transferring the call. Each header can be either a static value or a dynamic variable reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_sip_headers: Option<Vec<PhoneNumberTransferCustomSipHeadersItem>>,
    pub transfer_destination: PhoneNumberTransferTransferDestination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<TransferTypeEnum>,
    /// When True, a ringing tone is played on the original call leg while a SIP REFER transfer completes. The tone is carried over RTP to the SIP peer executing the REFER, so disable this if the receiving system (e.g. an SBC or contact center) should not hear it. When disabled the caller hears silence until the transfer completes. SIP REFER transfers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_refer_play_dialtone: Option<bool>,
    /// User-to-User Information (RFC 7433) to attach to SIP REFER transfers. Carries call context such as CRM identifiers or escalation reason across the transfer boundary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uui: Option<UuiTransferConfig>,
    /// DTMF digits to send after call connects (e.g., 'ww1234' for extension). Can be either a static value or a dynamic variable reference. Use 'w' for 0.5s pause. Only supported for Twilio transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_dial_digits: Option<PhoneNumberTransferPostDialDigits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub condition: String,
}

impl PhoneNumberTransfer {
    pub fn builder() -> PhoneNumberTransferBuilder {
        <PhoneNumberTransferBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneNumberTransferBuilder {
    custom_sip_headers: Option<Vec<PhoneNumberTransferCustomSipHeadersItem>>,
    transfer_destination: Option<PhoneNumberTransferTransferDestination>,
    transfer_type: Option<TransferTypeEnum>,
    sip_refer_play_dialtone: Option<bool>,
    uui: Option<UuiTransferConfig>,
    post_dial_digits: Option<PhoneNumberTransferPostDialDigits>,
    phone_number: Option<String>,
    condition: Option<String>,
}

impl PhoneNumberTransferBuilder {
    pub fn custom_sip_headers(mut self, value: Vec<PhoneNumberTransferCustomSipHeadersItem>) -> Self {
        self.custom_sip_headers = Some(value);
        self
    }

    pub fn transfer_destination(mut self, value: PhoneNumberTransferTransferDestination) -> Self {
        self.transfer_destination = Some(value);
        self
    }

    pub fn transfer_type(mut self, value: TransferTypeEnum) -> Self {
        self.transfer_type = Some(value);
        self
    }

    pub fn sip_refer_play_dialtone(mut self, value: bool) -> Self {
        self.sip_refer_play_dialtone = Some(value);
        self
    }

    pub fn uui(mut self, value: UuiTransferConfig) -> Self {
        self.uui = Some(value);
        self
    }

    pub fn post_dial_digits(mut self, value: PhoneNumberTransferPostDialDigits) -> Self {
        self.post_dial_digits = Some(value);
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn condition(mut self, value: impl Into<String>) -> Self {
        self.condition = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PhoneNumberTransfer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_destination`](PhoneNumberTransferBuilder::transfer_destination)
    /// - [`condition`](PhoneNumberTransferBuilder::condition)
    pub fn build(self) -> Result<PhoneNumberTransfer, BuildError> {
        Ok(PhoneNumberTransfer {
            custom_sip_headers: self.custom_sip_headers,
            transfer_destination: self.transfer_destination.ok_or_else(|| BuildError::missing_field("transfer_destination"))?,
            transfer_type: self.transfer_type,
            sip_refer_play_dialtone: self.sip_refer_play_dialtone,
            uui: self.uui,
            post_dial_digits: self.post_dial_digits,
            phone_number: self.phone_number,
            condition: self.condition.ok_or_else(|| BuildError::missing_field("condition"))?,
        })
    }
}
