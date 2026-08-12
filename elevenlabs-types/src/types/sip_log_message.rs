pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SipLogMessage {
    #[serde(default)]
    pub call_id: String,
    #[serde(default)]
    pub phone_numbers: Vec<String>,
    #[serde(default)]
    pub local_address: String,
    #[serde(default)]
    pub remote_address: String,
    #[serde(default)]
    pub transport: String,
    #[serde(default)]
    pub raw_message: String,
    #[serde(default)]
    pub error_message: String,
    pub direction: SipLogMessageDirection,
    #[serde(default)]
    pub created_at_unix_micro: i64,
}

impl SipLogMessage {
    pub fn builder() -> SipLogMessageBuilder {
        <SipLogMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipLogMessageBuilder {
    call_id: Option<String>,
    phone_numbers: Option<Vec<String>>,
    local_address: Option<String>,
    remote_address: Option<String>,
    transport: Option<String>,
    raw_message: Option<String>,
    error_message: Option<String>,
    direction: Option<SipLogMessageDirection>,
    created_at_unix_micro: Option<i64>,
}

impl SipLogMessageBuilder {
    pub fn call_id(mut self, value: impl Into<String>) -> Self {
        self.call_id = Some(value.into());
        self
    }

    pub fn phone_numbers(mut self, value: Vec<String>) -> Self {
        self.phone_numbers = Some(value);
        self
    }

    pub fn local_address(mut self, value: impl Into<String>) -> Self {
        self.local_address = Some(value.into());
        self
    }

    pub fn remote_address(mut self, value: impl Into<String>) -> Self {
        self.remote_address = Some(value.into());
        self
    }

    pub fn transport(mut self, value: impl Into<String>) -> Self {
        self.transport = Some(value.into());
        self
    }

    pub fn raw_message(mut self, value: impl Into<String>) -> Self {
        self.raw_message = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn direction(mut self, value: SipLogMessageDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn created_at_unix_micro(mut self, value: i64) -> Self {
        self.created_at_unix_micro = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SipLogMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`call_id`](SipLogMessageBuilder::call_id)
    /// - [`phone_numbers`](SipLogMessageBuilder::phone_numbers)
    /// - [`local_address`](SipLogMessageBuilder::local_address)
    /// - [`remote_address`](SipLogMessageBuilder::remote_address)
    /// - [`transport`](SipLogMessageBuilder::transport)
    /// - [`raw_message`](SipLogMessageBuilder::raw_message)
    /// - [`error_message`](SipLogMessageBuilder::error_message)
    /// - [`direction`](SipLogMessageBuilder::direction)
    /// - [`created_at_unix_micro`](SipLogMessageBuilder::created_at_unix_micro)
    pub fn build(self) -> Result<SipLogMessage, BuildError> {
        Ok(SipLogMessage {
            call_id: self.call_id.ok_or_else(|| BuildError::missing_field("call_id"))?,
            phone_numbers: self.phone_numbers.ok_or_else(|| BuildError::missing_field("phone_numbers"))?,
            local_address: self.local_address.ok_or_else(|| BuildError::missing_field("local_address"))?,
            remote_address: self.remote_address.ok_or_else(|| BuildError::missing_field("remote_address"))?,
            transport: self.transport.ok_or_else(|| BuildError::missing_field("transport"))?,
            raw_message: self.raw_message.ok_or_else(|| BuildError::missing_field("raw_message"))?,
            error_message: self.error_message.ok_or_else(|| BuildError::missing_field("error_message"))?,
            direction: self.direction.ok_or_else(|| BuildError::missing_field("direction"))?,
            created_at_unix_micro: self.created_at_unix_micro.ok_or_else(|| BuildError::missing_field("created_at_unix_micro"))?,
        })
    }
}
