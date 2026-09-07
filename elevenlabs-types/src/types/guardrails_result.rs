pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GuardrailsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered: Option<bool>,
}

impl GuardrailsResult {
    pub fn builder() -> GuardrailsResultBuilder {
        <GuardrailsResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GuardrailsResultBuilder {
    triggered: Option<bool>,
}

impl GuardrailsResultBuilder {
    pub fn triggered(mut self, value: bool) -> Self {
        self.triggered = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GuardrailsResult`].
    pub fn build(self) -> Result<GuardrailsResult, BuildError> {
        Ok(GuardrailsResult {
            triggered: self.triggered,
        })
    }
}
