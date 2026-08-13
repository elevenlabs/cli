pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SayNodePromptMessageInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// LLM prompt describing what message should be generated.
    #[serde(default)]
    pub prompt: String,
}

impl SayNodePromptMessageInput {
    pub fn builder() -> SayNodePromptMessageInputBuilder {
        <SayNodePromptMessageInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SayNodePromptMessageInputBuilder {
    r#type: Option<String>,
    prompt: Option<String>,
}

impl SayNodePromptMessageInputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SayNodePromptMessageInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](SayNodePromptMessageInputBuilder::prompt)
    pub fn build(self) -> Result<SayNodePromptMessageInput, BuildError> {
        Ok(SayNodePromptMessageInput {
            r#type: self.r#type,
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
        })
    }
}
