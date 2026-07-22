pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FocusGuardrail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

impl FocusGuardrail {
    pub fn builder() -> FocusGuardrailBuilder {
        <FocusGuardrailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FocusGuardrailBuilder {
    is_enabled: Option<bool>,
}

impl FocusGuardrailBuilder {
    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FocusGuardrail`].
    pub fn build(self) -> Result<FocusGuardrail, BuildError> {
        Ok(FocusGuardrail {
            is_enabled: self.is_enabled,
        })
    }
}
