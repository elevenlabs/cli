pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferToNumberResultExotelSuccessModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub transfer_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl TransferToNumberResultExotelSuccessModel {
    pub fn builder() -> TransferToNumberResultExotelSuccessModelBuilder {
        <TransferToNumberResultExotelSuccessModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferToNumberResultExotelSuccessModelBuilder {
    status: Option<String>,
    transfer_number: Option<String>,
    reason: Option<String>,
    agent_message: Option<String>,
    note: Option<String>,
}

impl TransferToNumberResultExotelSuccessModelBuilder {
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

    pub fn agent_message(mut self, value: impl Into<String>) -> Self {
        self.agent_message = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferToNumberResultExotelSuccessModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_number`](TransferToNumberResultExotelSuccessModelBuilder::transfer_number)
    pub fn build(self) -> Result<TransferToNumberResultExotelSuccessModel, BuildError> {
        Ok(TransferToNumberResultExotelSuccessModel {
            status: self.status,
            transfer_number: self.transfer_number.ok_or_else(|| BuildError::missing_field("transfer_number"))?,
            reason: self.reason,
            agent_message: self.agent_message,
            note: self.note,
        })
    }
}
