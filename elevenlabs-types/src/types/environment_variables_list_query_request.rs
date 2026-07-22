pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnvironmentVariablesListQueryRequest {
    /// Pagination cursor from previous response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Number of items to return (1-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Filter by exact label match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Filter to only return variables that have this environment. When specified, the values dict in the response will only contain this environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Filter by variable type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EnvironmentVariablesListRequestType>,
}

impl EnvironmentVariablesListQueryRequest {
    pub fn builder() -> EnvironmentVariablesListQueryRequestBuilder {
        <EnvironmentVariablesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentVariablesListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    label: Option<String>,
    environment: Option<String>,
    r#type: Option<EnvironmentVariablesListRequestType>,
}

impl EnvironmentVariablesListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: EnvironmentVariablesListRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentVariablesListQueryRequest`].
    pub fn build(self) -> Result<EnvironmentVariablesListQueryRequest, BuildError> {
        Ok(EnvironmentVariablesListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            label: self.label,
            environment: self.environment,
            r#type: self.r#type,
        })
    }
}

