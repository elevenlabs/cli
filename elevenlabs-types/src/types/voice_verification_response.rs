pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceVerificationResponse {
    /// Whether the voice requires verification.
    #[serde(default)]
    pub requires_verification: bool,
    /// Whether the voice has been verified.
    #[serde(default)]
    pub is_verified: bool,
    /// List of verification failures.
    #[serde(default)]
    pub verification_failures: Vec<String>,
    /// The number of verification attempts.
    #[serde(default)]
    pub verification_attempts_count: i64,
    /// The language of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Number of times a verification was attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_attempts: Option<Vec<VerificationAttemptResponse>>,
}

impl VoiceVerificationResponse {
    pub fn builder() -> VoiceVerificationResponseBuilder {
        <VoiceVerificationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceVerificationResponseBuilder {
    requires_verification: Option<bool>,
    is_verified: Option<bool>,
    verification_failures: Option<Vec<String>>,
    verification_attempts_count: Option<i64>,
    language: Option<String>,
    verification_attempts: Option<Vec<VerificationAttemptResponse>>,
}

impl VoiceVerificationResponseBuilder {
    pub fn requires_verification(mut self, value: bool) -> Self {
        self.requires_verification = Some(value);
        self
    }

    pub fn is_verified(mut self, value: bool) -> Self {
        self.is_verified = Some(value);
        self
    }

    pub fn verification_failures(mut self, value: Vec<String>) -> Self {
        self.verification_failures = Some(value);
        self
    }

    pub fn verification_attempts_count(mut self, value: i64) -> Self {
        self.verification_attempts_count = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn verification_attempts(mut self, value: Vec<VerificationAttemptResponse>) -> Self {
        self.verification_attempts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceVerificationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requires_verification`](VoiceVerificationResponseBuilder::requires_verification)
    /// - [`is_verified`](VoiceVerificationResponseBuilder::is_verified)
    /// - [`verification_failures`](VoiceVerificationResponseBuilder::verification_failures)
    /// - [`verification_attempts_count`](VoiceVerificationResponseBuilder::verification_attempts_count)
    pub fn build(self) -> Result<VoiceVerificationResponse, BuildError> {
        Ok(VoiceVerificationResponse {
            requires_verification: self.requires_verification.ok_or_else(|| BuildError::missing_field("requires_verification"))?,
            is_verified: self.is_verified.ok_or_else(|| BuildError::missing_field("is_verified"))?,
            verification_failures: self.verification_failures.ok_or_else(|| BuildError::missing_field("verification_failures"))?,
            verification_attempts_count: self.verification_attempts_count.ok_or_else(|| BuildError::missing_field("verification_attempts_count"))?,
            language: self.language,
            verification_attempts: self.verification_attempts,
        })
    }
}
