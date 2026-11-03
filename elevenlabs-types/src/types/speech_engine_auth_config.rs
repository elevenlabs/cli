pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpeechEngineAuthConfig {
    /// If true, starting a conversation with the speech engine requires a signed URL or token from an authorized caller, rather than only its ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth: Option<bool>,
}

impl SpeechEngineAuthConfig {
    pub fn builder() -> SpeechEngineAuthConfigBuilder {
        <SpeechEngineAuthConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechEngineAuthConfigBuilder {
    enable_auth: Option<bool>,
}

impl SpeechEngineAuthConfigBuilder {
    pub fn enable_auth(mut self, value: bool) -> Self {
        self.enable_auth = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeechEngineAuthConfig`].
    pub fn build(self) -> Result<SpeechEngineAuthConfig, BuildError> {
        Ok(SpeechEngineAuthConfig {
            enable_auth: self.enable_auth,
        })
    }
}
