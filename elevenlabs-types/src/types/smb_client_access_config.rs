pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SmbClientAccessConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_secure_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_verification_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_verification_email_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_verification_sms_enabled: Option<bool>,
}

impl SmbClientAccessConfig {
    pub fn builder() -> SmbClientAccessConfigBuilder {
        <SmbClientAccessConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SmbClientAccessConfigBuilder {
    enable_secure_mode: Option<bool>,
    identity_verification_enabled: Option<bool>,
    identity_verification_email_enabled: Option<bool>,
    identity_verification_sms_enabled: Option<bool>,
}

impl SmbClientAccessConfigBuilder {
    pub fn enable_secure_mode(mut self, value: bool) -> Self {
        self.enable_secure_mode = Some(value);
        self
    }

    pub fn identity_verification_enabled(mut self, value: bool) -> Self {
        self.identity_verification_enabled = Some(value);
        self
    }

    pub fn identity_verification_email_enabled(mut self, value: bool) -> Self {
        self.identity_verification_email_enabled = Some(value);
        self
    }

    pub fn identity_verification_sms_enabled(mut self, value: bool) -> Self {
        self.identity_verification_sms_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SmbClientAccessConfig`].
    pub fn build(self) -> Result<SmbClientAccessConfig, BuildError> {
        Ok(SmbClientAccessConfig {
            enable_secure_mode: self.enable_secure_mode,
            identity_verification_enabled: self.identity_verification_enabled,
            identity_verification_email_enabled: self.identity_verification_email_enabled,
            identity_verification_sms_enabled: self.identity_verification_sms_enabled,
        })
    }
}
