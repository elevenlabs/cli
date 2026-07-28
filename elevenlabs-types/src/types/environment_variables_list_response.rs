pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EnvironmentVariablesListResponse {
    #[serde(default)]
    pub environment_variables: Vec<EnvironmentVariableResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl EnvironmentVariablesListResponse {
    pub fn builder() -> EnvironmentVariablesListResponseBuilder {
        <EnvironmentVariablesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentVariablesListResponseBuilder {
    environment_variables: Option<Vec<EnvironmentVariableResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl EnvironmentVariablesListResponseBuilder {
    pub fn environment_variables(mut self, value: Vec<EnvironmentVariableResponse>) -> Self {
        self.environment_variables = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentVariablesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment_variables`](EnvironmentVariablesListResponseBuilder::environment_variables)
    /// - [`has_more`](EnvironmentVariablesListResponseBuilder::has_more)
    pub fn build(self) -> Result<EnvironmentVariablesListResponse, BuildError> {
        Ok(EnvironmentVariablesListResponse {
            environment_variables: self.environment_variables.ok_or_else(|| BuildError::missing_field("environment_variables"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
