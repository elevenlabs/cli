pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Safety evaluation of the agent. Prompt and first message is taken into account.
/// The unsafe reason is provided from the evaluation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SafetyEvaluation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unsafe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_prompt_version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_rule_id: Option<Vec<SafetyRule>>,
}

impl SafetyEvaluation {
    pub fn builder() -> SafetyEvaluationBuilder {
        <SafetyEvaluationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SafetyEvaluationBuilder {
    is_unsafe: Option<bool>,
    llm_reason: Option<String>,
    safety_prompt_version: Option<i64>,
    matched_rule_id: Option<Vec<SafetyRule>>,
}

impl SafetyEvaluationBuilder {
    pub fn is_unsafe(mut self, value: bool) -> Self {
        self.is_unsafe = Some(value);
        self
    }

    pub fn llm_reason(mut self, value: impl Into<String>) -> Self {
        self.llm_reason = Some(value.into());
        self
    }

    pub fn safety_prompt_version(mut self, value: i64) -> Self {
        self.safety_prompt_version = Some(value);
        self
    }

    pub fn matched_rule_id(mut self, value: Vec<SafetyRule>) -> Self {
        self.matched_rule_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SafetyEvaluation`].
    pub fn build(self) -> Result<SafetyEvaluation, BuildError> {
        Ok(SafetyEvaluation {
            is_unsafe: self.is_unsafe,
            llm_reason: self.llm_reason,
            safety_prompt_version: self.safety_prompt_version,
            matched_rule_id: self.matched_rule_id,
        })
    }
}
