pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TriggeredGuardrailCommonModel {
    pub guardrail_type: GuardrailType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_name: Option<String>,
}

impl TriggeredGuardrailCommonModel {
    pub fn builder() -> TriggeredGuardrailCommonModelBuilder {
        <TriggeredGuardrailCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TriggeredGuardrailCommonModelBuilder {
    guardrail_type: Option<GuardrailType>,
    guardrail_name: Option<String>,
}

impl TriggeredGuardrailCommonModelBuilder {
    pub fn guardrail_type(mut self, value: GuardrailType) -> Self {
        self.guardrail_type = Some(value);
        self
    }

    pub fn guardrail_name(mut self, value: impl Into<String>) -> Self {
        self.guardrail_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TriggeredGuardrailCommonModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`guardrail_type`](TriggeredGuardrailCommonModelBuilder::guardrail_type)
    pub fn build(self) -> Result<TriggeredGuardrailCommonModel, BuildError> {
        Ok(TriggeredGuardrailCommonModel {
            guardrail_type: self.guardrail_type.ok_or_else(|| BuildError::missing_field("guardrail_type"))?,
            guardrail_name: self.guardrail_name,
        })
    }
}
