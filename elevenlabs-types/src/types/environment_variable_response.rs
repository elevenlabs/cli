pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentVariableResponse {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub created_at_unix_secs: i64,
    #[serde(default)]
    pub updated_at_unix_secs: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    pub r#type: EnvironmentVariableResponseType,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub workspace_id: String,
    pub values: EnvironmentVariableResponseValues,
}

impl EnvironmentVariableResponse {
    pub fn builder() -> EnvironmentVariableResponseBuilder {
        <EnvironmentVariableResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentVariableResponseBuilder {
    label: Option<String>,
    created_at_unix_secs: Option<i64>,
    updated_at_unix_secs: Option<i64>,
    created_by_user_id: Option<String>,
    r#type: Option<EnvironmentVariableResponseType>,
    id: Option<String>,
    workspace_id: Option<String>,
    values: Option<EnvironmentVariableResponseValues>,
}

impl EnvironmentVariableResponseBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn updated_at_unix_secs(mut self, value: i64) -> Self {
        self.updated_at_unix_secs = Some(value);
        self
    }

    pub fn created_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_user_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: EnvironmentVariableResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn values(mut self, value: EnvironmentVariableResponseValues) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentVariableResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](EnvironmentVariableResponseBuilder::label)
    /// - [`created_at_unix_secs`](EnvironmentVariableResponseBuilder::created_at_unix_secs)
    /// - [`updated_at_unix_secs`](EnvironmentVariableResponseBuilder::updated_at_unix_secs)
    /// - [`r#type`](EnvironmentVariableResponseBuilder::r#type)
    /// - [`id`](EnvironmentVariableResponseBuilder::id)
    /// - [`workspace_id`](EnvironmentVariableResponseBuilder::workspace_id)
    /// - [`values`](EnvironmentVariableResponseBuilder::values)
    pub fn build(self) -> Result<EnvironmentVariableResponse, BuildError> {
        Ok(EnvironmentVariableResponse {
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            updated_at_unix_secs: self.updated_at_unix_secs.ok_or_else(|| BuildError::missing_field("updated_at_unix_secs"))?,
            created_by_user_id: self.created_by_user_id,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            values: self.values.ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
