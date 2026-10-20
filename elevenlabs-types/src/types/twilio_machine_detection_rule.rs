pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Action to take on a live call for the given AMD verdicts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TwilioMachineDetectionRule {
    pub action: TwilioMachineDetectionAction,
    /// Verdicts that trigger the action.
    #[serde(default)]
    pub answered_by: Vec<TwilioAnsweredBy>,
}

impl TwilioMachineDetectionRule {
    pub fn builder() -> TwilioMachineDetectionRuleBuilder {
        <TwilioMachineDetectionRuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwilioMachineDetectionRuleBuilder {
    action: Option<TwilioMachineDetectionAction>,
    answered_by: Option<Vec<TwilioAnsweredBy>>,
}

impl TwilioMachineDetectionRuleBuilder {
    pub fn action(mut self, value: TwilioMachineDetectionAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn answered_by(mut self, value: Vec<TwilioAnsweredBy>) -> Self {
        self.answered_by = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwilioMachineDetectionRule`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](TwilioMachineDetectionRuleBuilder::action)
    /// - [`answered_by`](TwilioMachineDetectionRuleBuilder::answered_by)
    pub fn build(self) -> Result<TwilioMachineDetectionRule, BuildError> {
        Ok(TwilioMachineDetectionRule {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            answered_by: self.answered_by.ok_or_else(|| BuildError::missing_field("answered_by"))?,
        })
    }
}
