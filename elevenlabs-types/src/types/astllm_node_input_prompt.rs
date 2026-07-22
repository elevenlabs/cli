pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AstllmNodeInputPrompt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The prompt to evaluate to a boolean value. Deprecated. Use a boolean schema instead.
    #[serde(default)]
    pub prompt: String,
}

impl AstllmNodeInputPrompt {
    pub fn builder() -> AstllmNodeInputPromptBuilder {
        <AstllmNodeInputPromptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AstllmNodeInputPromptBuilder {
    r#type: Option<String>,
    prompt: Option<String>,
}

impl AstllmNodeInputPromptBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AstllmNodeInputPrompt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](AstllmNodeInputPromptBuilder::prompt)
    pub fn build(self) -> Result<AstllmNodeInputPrompt, BuildError> {
        Ok(AstllmNodeInputPrompt {
            r#type: self.r#type,
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
        })
    }
}
