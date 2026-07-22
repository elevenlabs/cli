pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BaseTurnConfig {
    /// Maximum wait time for the user's reply before re-engaging the user
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub turn_timeout: Option<f64>,
    /// How long the agent will wait for the user to start the conversation if the first message is empty. If not set, uses the regular turn_timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub initial_wait_time: Option<f64>,
    /// Maximum wait time since the user last spoke before terminating the call
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub silence_end_call_timeout: Option<f64>,
    /// Controls how eager the agent is to respond. Low = less eager (waits longer), Standard = default eagerness, High = more eager (responds sooner)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_eagerness: Option<TurnEagerness>,
    /// Controls if the agent should be more patient when user is spelling numbers and named entities. Auto = model based, Off = never wait extra
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spelling_patience: Option<SpellingPatience>,
    /// When enabled, starts generating LLM responses during silence before full turn confidence is reached, reducing perceived latency. May increase LLM costs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speculative_turn: Option<bool>,
    /// When enabled, if VAD detects no speech, attempts to re-transcribe accumulated audio at turn timeout. Disables silence discount billing for affected turns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retranscribe_on_turn_timeout: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_model: Option<TurnModel>,
    /// List of terms that should not trigger an interruption when spoken by the user (e.g. 'gotcha', 'understood'). Uses case-insensitive exact matching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruption_ignore_terms: Option<Vec<String>>,
    /// Language codes for which preset ignore-term categories have been activated. Stored explicitly so display is not inferred from term overlap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruption_ignore_term_languages: Option<Vec<String>>,
    /// When interruptions are disabled, still transcribe what the user says so it can carry into the next turn. When off, user speech during a non-interruptible turn is ignored and won't trigger a turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcribe_on_disabled_interruptions: Option<bool>,
}

impl BaseTurnConfig {
    pub fn builder() -> BaseTurnConfigBuilder {
        <BaseTurnConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BaseTurnConfigBuilder {
    turn_timeout: Option<f64>,
    initial_wait_time: Option<f64>,
    silence_end_call_timeout: Option<f64>,
    turn_eagerness: Option<TurnEagerness>,
    spelling_patience: Option<SpellingPatience>,
    speculative_turn: Option<bool>,
    retranscribe_on_turn_timeout: Option<bool>,
    turn_model: Option<TurnModel>,
    interruption_ignore_terms: Option<Vec<String>>,
    interruption_ignore_term_languages: Option<Vec<String>>,
    transcribe_on_disabled_interruptions: Option<bool>,
}

impl BaseTurnConfigBuilder {
    pub fn turn_timeout(mut self, value: f64) -> Self {
        self.turn_timeout = Some(value);
        self
    }

    pub fn initial_wait_time(mut self, value: f64) -> Self {
        self.initial_wait_time = Some(value);
        self
    }

    pub fn silence_end_call_timeout(mut self, value: f64) -> Self {
        self.silence_end_call_timeout = Some(value);
        self
    }

    pub fn turn_eagerness(mut self, value: TurnEagerness) -> Self {
        self.turn_eagerness = Some(value);
        self
    }

    pub fn spelling_patience(mut self, value: SpellingPatience) -> Self {
        self.spelling_patience = Some(value);
        self
    }

    pub fn speculative_turn(mut self, value: bool) -> Self {
        self.speculative_turn = Some(value);
        self
    }

    pub fn retranscribe_on_turn_timeout(mut self, value: bool) -> Self {
        self.retranscribe_on_turn_timeout = Some(value);
        self
    }

    pub fn turn_model(mut self, value: TurnModel) -> Self {
        self.turn_model = Some(value);
        self
    }

    pub fn interruption_ignore_terms(mut self, value: Vec<String>) -> Self {
        self.interruption_ignore_terms = Some(value);
        self
    }

    pub fn interruption_ignore_term_languages(mut self, value: Vec<String>) -> Self {
        self.interruption_ignore_term_languages = Some(value);
        self
    }

    pub fn transcribe_on_disabled_interruptions(mut self, value: bool) -> Self {
        self.transcribe_on_disabled_interruptions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BaseTurnConfig`].
    pub fn build(self) -> Result<BaseTurnConfig, BuildError> {
        Ok(BaseTurnConfig {
            turn_timeout: self.turn_timeout,
            initial_wait_time: self.initial_wait_time,
            silence_end_call_timeout: self.silence_end_call_timeout,
            turn_eagerness: self.turn_eagerness,
            spelling_patience: self.spelling_patience,
            speculative_turn: self.speculative_turn,
            retranscribe_on_turn_timeout: self.retranscribe_on_turn_timeout,
            turn_model: self.turn_model,
            interruption_ignore_terms: self.interruption_ignore_terms,
            interruption_ignore_term_languages: self.interruption_ignore_term_languages,
            transcribe_on_disabled_interruptions: self.transcribe_on_disabled_interruptions,
        })
    }
}
