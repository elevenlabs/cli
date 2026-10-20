pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateClientSecretRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<OpenAiSessionConfigInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<ExpiresAfter>,
}

impl CreateClientSecretRequest {
    pub fn builder() -> CreateClientSecretRequestBuilder {
        <CreateClientSecretRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateClientSecretRequestBuilder {
    session: Option<OpenAiSessionConfigInput>,
    expires_after: Option<ExpiresAfter>,
}

impl CreateClientSecretRequestBuilder {
    pub fn session(mut self, value: OpenAiSessionConfigInput) -> Self {
        self.session = Some(value);
        self
    }

    pub fn expires_after(mut self, value: ExpiresAfter) -> Self {
        self.expires_after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateClientSecretRequest`].
    pub fn build(self) -> Result<CreateClientSecretRequest, BuildError> {
        Ok(CreateClientSecretRequest {
            session: self.session,
            expires_after: self.expires_after,
        })
    }
}
