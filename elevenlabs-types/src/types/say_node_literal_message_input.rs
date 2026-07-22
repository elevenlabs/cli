pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SayNodeLiteralMessageInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Literal text message to be spoken by the agent.
    #[serde(default)]
    pub text: String,
}

impl SayNodeLiteralMessageInput {
    pub fn builder() -> SayNodeLiteralMessageInputBuilder {
        <SayNodeLiteralMessageInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SayNodeLiteralMessageInputBuilder {
    r#type: Option<String>,
    text: Option<String>,
}

impl SayNodeLiteralMessageInputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SayNodeLiteralMessageInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](SayNodeLiteralMessageInputBuilder::text)
    pub fn build(self) -> Result<SayNodeLiteralMessageInput, BuildError> {
        Ok(SayNodeLiteralMessageInput {
            r#type: self.r#type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
