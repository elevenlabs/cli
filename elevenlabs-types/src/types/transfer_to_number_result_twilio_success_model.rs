pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferToNumberResultTwilioSuccessModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub transfer_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_message: Option<String>,
    #[serde(default)]
    pub agent_message: String,
    #[serde(default)]
    pub conference_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_dial_digits: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl TransferToNumberResultTwilioSuccessModel {
    pub fn builder() -> TransferToNumberResultTwilioSuccessModelBuilder {
        <TransferToNumberResultTwilioSuccessModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferToNumberResultTwilioSuccessModelBuilder {
    status: Option<String>,
    transfer_number: Option<String>,
    reason: Option<String>,
    client_message: Option<String>,
    agent_message: Option<String>,
    conference_name: Option<String>,
    post_dial_digits: Option<String>,
    note: Option<String>,
}

impl TransferToNumberResultTwilioSuccessModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn transfer_number(mut self, value: impl Into<String>) -> Self {
        self.transfer_number = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn client_message(mut self, value: impl Into<String>) -> Self {
        self.client_message = Some(value.into());
        self
    }

    pub fn agent_message(mut self, value: impl Into<String>) -> Self {
        self.agent_message = Some(value.into());
        self
    }

    pub fn conference_name(mut self, value: impl Into<String>) -> Self {
        self.conference_name = Some(value.into());
        self
    }

    pub fn post_dial_digits(mut self, value: impl Into<String>) -> Self {
        self.post_dial_digits = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferToNumberResultTwilioSuccessModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_number`](TransferToNumberResultTwilioSuccessModelBuilder::transfer_number)
    /// - [`agent_message`](TransferToNumberResultTwilioSuccessModelBuilder::agent_message)
    /// - [`conference_name`](TransferToNumberResultTwilioSuccessModelBuilder::conference_name)
    pub fn build(self) -> Result<TransferToNumberResultTwilioSuccessModel, BuildError> {
        Ok(TransferToNumberResultTwilioSuccessModel {
            status: self.status,
            transfer_number: self.transfer_number.ok_or_else(|| BuildError::missing_field("transfer_number"))?,
            reason: self.reason,
            client_message: self.client_message,
            agent_message: self.agent_message.ok_or_else(|| BuildError::missing_field("agent_message"))?,
            conference_name: self.conference_name.ok_or_else(|| BuildError::missing_field("conference_name"))?,
            post_dial_digits: self.post_dial_digits,
            note: self.note,
        })
    }
}
