pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentOutputError {
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
}

impl AgentOutputError {
    pub fn builder() -> AgentOutputErrorBuilder {
        <AgentOutputErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentOutputErrorBuilder {
    r#type: Option<String>,
    code: Option<String>,
    message: Option<String>,
}

impl AgentOutputErrorBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentOutputError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AgentOutputErrorBuilder::r#type)
    /// - [`code`](AgentOutputErrorBuilder::code)
    /// - [`message`](AgentOutputErrorBuilder::message)
    pub fn build(self) -> Result<AgentOutputError, BuildError> {
        Ok(AgentOutputError {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
