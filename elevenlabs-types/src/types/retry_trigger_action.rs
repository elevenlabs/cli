pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetryTriggerAction {
    /// Custom feedback to inject into the agent when retrying after guardrail trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
}

impl RetryTriggerAction {
    pub fn builder() -> RetryTriggerActionBuilder {
        <RetryTriggerActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetryTriggerActionBuilder {
    feedback: Option<String>,
}

impl RetryTriggerActionBuilder {
    pub fn feedback(mut self, value: impl Into<String>) -> Self {
        self.feedback = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetryTriggerAction`].
    pub fn build(self) -> Result<RetryTriggerAction, BuildError> {
        Ok(RetryTriggerAction {
            feedback: self.feedback,
        })
    }
}
