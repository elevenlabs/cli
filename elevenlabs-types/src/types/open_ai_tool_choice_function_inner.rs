pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OpenAiToolChoiceFunctionInner {
    #[serde(default)]
    pub name: String,
}

impl OpenAiToolChoiceFunctionInner {
    pub fn builder() -> OpenAiToolChoiceFunctionInnerBuilder {
        <OpenAiToolChoiceFunctionInnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiToolChoiceFunctionInnerBuilder {
    name: Option<String>,
}

impl OpenAiToolChoiceFunctionInnerBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OpenAiToolChoiceFunctionInner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](OpenAiToolChoiceFunctionInnerBuilder::name)
    pub fn build(self) -> Result<OpenAiToolChoiceFunctionInner, BuildError> {
        Ok(OpenAiToolChoiceFunctionInner {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
