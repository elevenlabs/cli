pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostDialDigitsDynamicVariable {
    /// The dynamic variable name to resolve
    #[serde(default)]
    pub value: String,
}

impl PostDialDigitsDynamicVariable {
    pub fn builder() -> PostDialDigitsDynamicVariableBuilder {
        <PostDialDigitsDynamicVariableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostDialDigitsDynamicVariableBuilder {
    value: Option<String>,
}

impl PostDialDigitsDynamicVariableBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostDialDigitsDynamicVariable`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](PostDialDigitsDynamicVariableBuilder::value)
    pub fn build(self) -> Result<PostDialDigitsDynamicVariable, BuildError> {
        Ok(PostDialDigitsDynamicVariable {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
