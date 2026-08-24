pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SafetyResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blocked_ivc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blocked_non_ivc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_safety_evaluation: Option<bool>,
}

impl SafetyResponseModel {
    pub fn builder() -> SafetyResponseModelBuilder {
        <SafetyResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SafetyResponseModelBuilder {
    is_blocked_ivc: Option<bool>,
    is_blocked_non_ivc: Option<bool>,
    ignore_safety_evaluation: Option<bool>,
}

impl SafetyResponseModelBuilder {
    pub fn is_blocked_ivc(mut self, value: bool) -> Self {
        self.is_blocked_ivc = Some(value);
        self
    }

    pub fn is_blocked_non_ivc(mut self, value: bool) -> Self {
        self.is_blocked_non_ivc = Some(value);
        self
    }

    pub fn ignore_safety_evaluation(mut self, value: bool) -> Self {
        self.ignore_safety_evaluation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SafetyResponseModel`].
    pub fn build(self) -> Result<SafetyResponseModel, BuildError> {
        Ok(SafetyResponseModel {
            is_blocked_ivc: self.is_blocked_ivc,
            is_blocked_non_ivc: self.is_blocked_non_ivc,
            ignore_safety_evaluation: self.ignore_safety_evaluation,
        })
    }
}
