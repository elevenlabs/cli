pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SayNodeLiteralMessageOutput {
    pub r#type: String,
    /// Literal text message to be spoken by the agent.
    #[serde(default)]
    pub text: String,
}

impl SayNodeLiteralMessageOutput {
    pub fn builder() -> SayNodeLiteralMessageOutputBuilder {
        <SayNodeLiteralMessageOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SayNodeLiteralMessageOutputBuilder {
    r#type: Option<String>,
    text: Option<String>,
}

impl SayNodeLiteralMessageOutputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SayNodeLiteralMessageOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](SayNodeLiteralMessageOutputBuilder::r#type)
    /// - [`text`](SayNodeLiteralMessageOutputBuilder::text)
    pub fn build(self) -> Result<SayNodeLiteralMessageOutput, BuildError> {
        Ok(SayNodeLiteralMessageOutput {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
