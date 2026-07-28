pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OpenAiToolChoiceFunction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub function: OpenAiToolChoiceFunctionInner,
}

impl OpenAiToolChoiceFunction {
    pub fn builder() -> OpenAiToolChoiceFunctionBuilder {
        <OpenAiToolChoiceFunctionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiToolChoiceFunctionBuilder {
    r#type: Option<String>,
    function: Option<OpenAiToolChoiceFunctionInner>,
}

impl OpenAiToolChoiceFunctionBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn function(mut self, value: OpenAiToolChoiceFunctionInner) -> Self {
        self.function = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiToolChoiceFunction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`function`](OpenAiToolChoiceFunctionBuilder::function)
    pub fn build(self) -> Result<OpenAiToolChoiceFunction, BuildError> {
        Ok(OpenAiToolChoiceFunction {
            r#type: self.r#type,
            function: self.function.ok_or_else(|| BuildError::missing_field("function"))?,
        })
    }
}
