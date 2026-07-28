pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentSuccessfulResponseExample {
    #[serde(default)]
    pub response: String,
    pub r#type: String,
}

impl AgentSuccessfulResponseExample {
    pub fn builder() -> AgentSuccessfulResponseExampleBuilder {
        <AgentSuccessfulResponseExampleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSuccessfulResponseExampleBuilder {
    response: Option<String>,
    r#type: Option<String>,
}

impl AgentSuccessfulResponseExampleBuilder {
    pub fn response(mut self, value: impl Into<String>) -> Self {
        self.response = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSuccessfulResponseExample`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response`](AgentSuccessfulResponseExampleBuilder::response)
    /// - [`r#type`](AgentSuccessfulResponseExampleBuilder::r#type)
    pub fn build(self) -> Result<AgentSuccessfulResponseExample, BuildError> {
        Ok(AgentSuccessfulResponseExample {
            response: self.response.ok_or_else(|| BuildError::missing_field("response"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
