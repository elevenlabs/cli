pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentFailureResponseExample {
    #[serde(default)]
    pub response: String,
    pub r#type: String,
}

impl AgentFailureResponseExample {
    pub fn builder() -> AgentFailureResponseExampleBuilder {
        <AgentFailureResponseExampleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentFailureResponseExampleBuilder {
    response: Option<String>,
    r#type: Option<String>,
}

impl AgentFailureResponseExampleBuilder {
    pub fn response(mut self, value: impl Into<String>) -> Self {
        self.response = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentFailureResponseExample`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response`](AgentFailureResponseExampleBuilder::response)
    /// - [`r#type`](AgentFailureResponseExampleBuilder::r#type)
    pub fn build(self) -> Result<AgentFailureResponseExample, BuildError> {
        Ok(AgentFailureResponseExample {
            response: self.response.ok_or_else(|| BuildError::missing_field("response"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
