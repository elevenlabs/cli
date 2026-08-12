pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Safety object that has the information of safety evaluations based on used voice.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SafetyCommonModelOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ivc: Option<SafetyEvaluation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_ivc: Option<SafetyEvaluation>,
}

impl SafetyCommonModelOutput {
    pub fn builder() -> SafetyCommonModelOutputBuilder {
        <SafetyCommonModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SafetyCommonModelOutputBuilder {
    ivc: Option<SafetyEvaluation>,
    non_ivc: Option<SafetyEvaluation>,
}

impl SafetyCommonModelOutputBuilder {
    pub fn ivc(mut self, value: SafetyEvaluation) -> Self {
        self.ivc = Some(value);
        self
    }

    pub fn non_ivc(mut self, value: SafetyEvaluation) -> Self {
        self.non_ivc = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SafetyCommonModelOutput`].
    pub fn build(self) -> Result<SafetyCommonModelOutput, BuildError> {
        Ok(SafetyCommonModelOutput {
            ivc: self.ivc,
            non_ivc: self.non_ivc,
        })
    }
}
