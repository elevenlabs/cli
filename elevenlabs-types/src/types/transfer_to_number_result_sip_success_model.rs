pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferToNumberResultSipSuccessModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub transfer_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl TransferToNumberResultSipSuccessModel {
    pub fn builder() -> TransferToNumberResultSipSuccessModelBuilder {
        <TransferToNumberResultSipSuccessModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferToNumberResultSipSuccessModelBuilder {
    status: Option<String>,
    transfer_number: Option<String>,
    reason: Option<String>,
    note: Option<String>,
}

impl TransferToNumberResultSipSuccessModelBuilder {
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

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferToNumberResultSipSuccessModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_number`](TransferToNumberResultSipSuccessModelBuilder::transfer_number)
    pub fn build(self) -> Result<TransferToNumberResultSipSuccessModel, BuildError> {
        Ok(TransferToNumberResultSipSuccessModel {
            status: self.status,
            transfer_number: self.transfer_number.ok_or_else(|| BuildError::missing_field("transfer_number"))?,
            reason: self.reason,
            note: self.note,
        })
    }
}
