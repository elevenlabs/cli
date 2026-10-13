pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferToNumberResultAmazonConnectSuccessModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Why the agent escalated, as sent to Amazon Connect with the Escalate outcome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Message spoken to the caller before the session was handed back to Amazon Connect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_message: Option<String>,
    /// Set when the agent echoed an invalid option and the only configured one was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl TransferToNumberResultAmazonConnectSuccessModel {
    pub fn builder() -> TransferToNumberResultAmazonConnectSuccessModelBuilder {
        <TransferToNumberResultAmazonConnectSuccessModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferToNumberResultAmazonConnectSuccessModelBuilder {
    status: Option<String>,
    reason: Option<String>,
    client_message: Option<String>,
    note: Option<String>,
}

impl TransferToNumberResultAmazonConnectSuccessModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
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

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferToNumberResultAmazonConnectSuccessModel`].
    pub fn build(self) -> Result<TransferToNumberResultAmazonConnectSuccessModel, BuildError> {
        Ok(TransferToNumberResultAmazonConnectSuccessModel {
            status: self.status,
            reason: self.reason,
            client_message: self.client_message,
            note: self.note,
        })
    }
}
