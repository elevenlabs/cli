pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SayNodePromptMessageOutput {
    pub r#type: String,
    /// LLM prompt describing what message should be generated.
    #[serde(default)]
    pub prompt: String,
}

impl SayNodePromptMessageOutput {
    pub fn builder() -> SayNodePromptMessageOutputBuilder {
        <SayNodePromptMessageOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SayNodePromptMessageOutputBuilder {
    r#type: Option<String>,
    prompt: Option<String>,
}

impl SayNodePromptMessageOutputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SayNodePromptMessageOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](SayNodePromptMessageOutputBuilder::r#type)
    /// - [`prompt`](SayNodePromptMessageOutputBuilder::prompt)
    pub fn build(self) -> Result<SayNodePromptMessageOutput, BuildError> {
        Ok(SayNodePromptMessageOutput {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
        })
    }
}
