pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnvironmentResponse {
    /// Name of the environment, e.g. 'staging'.
    #[serde(default)]
    pub name: String,
    /// When the environment was created or one of its values last changed. Unset when neither is known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_unix_secs: Option<i64>,
    /// Unset for 'production' and when the creator is not known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
}

impl EnvironmentResponse {
    pub fn builder() -> EnvironmentResponseBuilder {
        <EnvironmentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentResponseBuilder {
    name: Option<String>,
    updated_at_unix_secs: Option<i64>,
    created_by_user_id: Option<String>,
}

impl EnvironmentResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    /// Consumes the builder and constructs a [`EnvironmentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](EnvironmentResponseBuilder::name)
    pub fn build(self) -> Result<EnvironmentResponse, BuildError> {
        Ok(EnvironmentResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            updated_at_unix_secs: self.updated_at_unix_secs,
            created_by_user_id: self.created_by_user_id,
        })
    }
}
