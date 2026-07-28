pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VerificationAttemptResponse {
    /// The text of the verification attempt.
    #[serde(default)]
    pub text: String,
    /// The date of the verification attempt in Unix time.
    #[serde(default)]
    pub date_unix: i64,
    /// Whether the verification attempt was accepted.
    #[serde(default)]
    pub accepted: bool,
    /// The similarity of the verification attempt.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub similarity: f64,
    /// The Levenshtein distance of the verification attempt.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub levenshtein_distance: f64,
    /// The recording of the verification attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording: Option<RecordingResponse>,
}

impl VerificationAttemptResponse {
    pub fn builder() -> VerificationAttemptResponseBuilder {
        <VerificationAttemptResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerificationAttemptResponseBuilder {
    text: Option<String>,
    date_unix: Option<i64>,
    accepted: Option<bool>,
    similarity: Option<f64>,
    levenshtein_distance: Option<f64>,
    recording: Option<RecordingResponse>,
}

impl VerificationAttemptResponseBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn date_unix(mut self, value: i64) -> Self {
        self.date_unix = Some(value);
        self
    }

    pub fn accepted(mut self, value: bool) -> Self {
        self.accepted = Some(value);
        self
    }

    pub fn similarity(mut self, value: f64) -> Self {
        self.similarity = Some(value);
        self
    }

    pub fn levenshtein_distance(mut self, value: f64) -> Self {
        self.levenshtein_distance = Some(value);
        self
    }

    pub fn recording(mut self, value: RecordingResponse) -> Self {
        self.recording = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VerificationAttemptResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](VerificationAttemptResponseBuilder::text)
    /// - [`date_unix`](VerificationAttemptResponseBuilder::date_unix)
    /// - [`accepted`](VerificationAttemptResponseBuilder::accepted)
    /// - [`similarity`](VerificationAttemptResponseBuilder::similarity)
    /// - [`levenshtein_distance`](VerificationAttemptResponseBuilder::levenshtein_distance)
    pub fn build(self) -> Result<VerificationAttemptResponse, BuildError> {
        Ok(VerificationAttemptResponse {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            date_unix: self.date_unix.ok_or_else(|| BuildError::missing_field("date_unix"))?,
            accepted: self.accepted.ok_or_else(|| BuildError::missing_field("accepted"))?,
            similarity: self.similarity.ok_or_else(|| BuildError::missing_field("similarity"))?,
            levenshtein_distance: self.levenshtein_distance.ok_or_else(|| BuildError::missing_field("levenshtein_distance"))?,
            recording: self.recording,
        })
    }
}
