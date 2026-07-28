pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FineTuningResponse {
    /// Whether the user is allowed to fine-tune the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_allowed_to_fine_tune: Option<bool>,
    /// The state of the fine-tuning process for each model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<HashMap<String, FineTuningResponseStateValue>>,
    /// List of verification failures in the fine-tuning process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_failures: Option<Vec<String>>,
    /// The number of verification attempts in the fine-tuning process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_attempts_count: Option<i64>,
    /// Whether a manual verification was requested for the fine-tuning process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_verification_requested: Option<bool>,
    /// The language of the fine-tuning process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The progress of the fine-tuning process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<HashMap<String, Option<f64>>>,
    /// The message of the fine-tuning process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<HashMap<String, Option<String>>>,
    /// The duration of the dataset in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dataset_duration_seconds: Option<f64>,
    /// The number of verification attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_attempts: Option<Vec<VerificationAttemptResponse>>,
    /// List of slice IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slice_ids: Option<Vec<String>>,
    /// The manual verification of the fine-tuning process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_verification: Option<ManualVerificationResponse>,
    /// The maximum number of verification attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_verification_attempts: Option<i64>,
    /// The next maximum verification attempts reset time in Unix milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_max_verification_attempts_reset_unix_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finetuning_state: Option<serde_json::Value>,
}

impl FineTuningResponse {
    pub fn builder() -> FineTuningResponseBuilder {
        <FineTuningResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FineTuningResponseBuilder {
    is_allowed_to_fine_tune: Option<bool>,
    state: Option<HashMap<String, FineTuningResponseStateValue>>,
    verification_failures: Option<Vec<String>>,
    verification_attempts_count: Option<i64>,
    manual_verification_requested: Option<bool>,
    language: Option<String>,
    progress: Option<HashMap<String, Option<f64>>>,
    message: Option<HashMap<String, Option<String>>>,
    dataset_duration_seconds: Option<f64>,
    verification_attempts: Option<Vec<VerificationAttemptResponse>>,
    slice_ids: Option<Vec<String>>,
    manual_verification: Option<ManualVerificationResponse>,
    max_verification_attempts: Option<i64>,
    next_max_verification_attempts_reset_unix_ms: Option<i64>,
    finetuning_state: Option<serde_json::Value>,
}

impl FineTuningResponseBuilder {
    pub fn is_allowed_to_fine_tune(mut self, value: bool) -> Self {
        self.is_allowed_to_fine_tune = Some(value);
        self
    }

    pub fn state(mut self, value: HashMap<String, FineTuningResponseStateValue>) -> Self {
        self.state = Some(value);
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

    pub fn manual_verification_requested(mut self, value: bool) -> Self {
        self.manual_verification_requested = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn progress(mut self, value: HashMap<String, Option<f64>>) -> Self {
        self.progress = Some(value);
        self
    }

    pub fn message(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.message = Some(value);
        self
    }

    pub fn dataset_duration_seconds(mut self, value: f64) -> Self {
        self.dataset_duration_seconds = Some(value);
        self
    }

    pub fn verification_attempts(mut self, value: Vec<VerificationAttemptResponse>) -> Self {
        self.verification_attempts = Some(value);
        self
    }

    pub fn slice_ids(mut self, value: Vec<String>) -> Self {
        self.slice_ids = Some(value);
        self
    }

    pub fn manual_verification(mut self, value: ManualVerificationResponse) -> Self {
        self.manual_verification = Some(value);
        self
    }

    pub fn max_verification_attempts(mut self, value: i64) -> Self {
        self.max_verification_attempts = Some(value);
        self
    }

    pub fn next_max_verification_attempts_reset_unix_ms(mut self, value: i64) -> Self {
        self.next_max_verification_attempts_reset_unix_ms = Some(value);
        self
    }

    pub fn finetuning_state(mut self, value: serde_json::Value) -> Self {
        self.finetuning_state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FineTuningResponse`].
    pub fn build(self) -> Result<FineTuningResponse, BuildError> {
        Ok(FineTuningResponse {
            is_allowed_to_fine_tune: self.is_allowed_to_fine_tune,
            state: self.state,
            verification_failures: self.verification_failures,
            verification_attempts_count: self.verification_attempts_count,
            manual_verification_requested: self.manual_verification_requested,
            language: self.language,
            progress: self.progress,
            message: self.message,
            dataset_duration_seconds: self.dataset_duration_seconds,
            verification_attempts: self.verification_attempts,
            slice_ids: self.slice_ids,
            manual_verification: self.manual_verification,
            max_verification_attempts: self.max_verification_attempts,
            next_max_verification_attempts_reset_unix_ms: self.next_max_verification_attempts_reset_unix_ms,
            finetuning_state: self.finetuning_state,
        })
    }
}
