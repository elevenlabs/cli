pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Settings to evaluate an agent's performance.
/// Agents are evaluated against a set of criteria, with success being defined as meeting some combination of those criteria.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EvaluationSettingsOutput {
    /// Individual criteria that the agent should be evaluated against
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Vec<PromptEvaluationCriteria>>,
}

impl EvaluationSettingsOutput {
    pub fn builder() -> EvaluationSettingsOutputBuilder {
        <EvaluationSettingsOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationSettingsOutputBuilder {
    criteria: Option<Vec<PromptEvaluationCriteria>>,
}

impl EvaluationSettingsOutputBuilder {
    pub fn criteria(mut self, value: Vec<PromptEvaluationCriteria>) -> Self {
        self.criteria = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluationSettingsOutput`].
    pub fn build(self) -> Result<EvaluationSettingsOutput, BuildError> {
        Ok(EvaluationSettingsOutput {
            criteria: self.criteria,
        })
    }
}
