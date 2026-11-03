pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitTestGuardrailTriggerEvaluationModel {
    /// Match any guardrail triggered by the generated turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl UnitTestGuardrailTriggerEvaluationModel {
    pub fn builder() -> UnitTestGuardrailTriggerEvaluationModelBuilder {
        <UnitTestGuardrailTriggerEvaluationModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitTestGuardrailTriggerEvaluationModelBuilder {
    r#type: Option<String>,
}

impl UnitTestGuardrailTriggerEvaluationModelBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnitTestGuardrailTriggerEvaluationModel`].
    pub fn build(self) -> Result<UnitTestGuardrailTriggerEvaluationModel, BuildError> {
        Ok(UnitTestGuardrailTriggerEvaluationModel {
            r#type: self.r#type,
        })
    }
}
