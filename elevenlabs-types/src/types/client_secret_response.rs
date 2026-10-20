pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClientSecretResponse {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub expires_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<OpenAiSessionConfigOutput>,
}

impl ClientSecretResponse {
    pub fn builder() -> ClientSecretResponseBuilder {
        <ClientSecretResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientSecretResponseBuilder {
    value: Option<String>,
    expires_at: Option<i64>,
    session: Option<OpenAiSessionConfigOutput>,
}

impl ClientSecretResponseBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: i64) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn session(mut self, value: OpenAiSessionConfigOutput) -> Self {
        self.session = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientSecretResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](ClientSecretResponseBuilder::value)
    /// - [`expires_at`](ClientSecretResponseBuilder::expires_at)
    pub fn build(self) -> Result<ClientSecretResponse, BuildError> {
        Ok(ClientSecretResponse {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            session: self.session,
        })
    }
}
