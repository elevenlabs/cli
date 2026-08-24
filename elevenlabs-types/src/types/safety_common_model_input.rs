pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Safety object that has the information of safety evaluations based on used voice.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SafetyCommonModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ivc: Option<SafetyEvaluation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_ivc: Option<SafetyEvaluation>,
}

impl SafetyCommonModelInput {
    pub fn builder() -> SafetyCommonModelInputBuilder {
        <SafetyCommonModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SafetyCommonModelInputBuilder {
    ivc: Option<SafetyEvaluation>,
    non_ivc: Option<SafetyEvaluation>,
}

impl SafetyCommonModelInputBuilder {
    pub fn ivc(mut self, value: SafetyEvaluation) -> Self {
        self.ivc = Some(value);
        self
    }

    pub fn non_ivc(mut self, value: SafetyEvaluation) -> Self {
        self.non_ivc = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SafetyCommonModelInput`].
    pub fn build(self) -> Result<SafetyCommonModelInput, BuildError> {
        Ok(SafetyCommonModelInput {
            ivc: self.ivc,
            non_ivc: self.non_ivc,
        })
    }
}
