pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Used to reference an auth connection from the workspace's auth connection store.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct AuthConnectionLocator {
    pub auth_connection_id: String,
}

impl AuthConnectionLocator {
    pub fn builder() -> AuthConnectionLocatorBuilder {
        <AuthConnectionLocatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthConnectionLocatorBuilder {
    auth_connection_id: Option<String>,
}

impl AuthConnectionLocatorBuilder {
    pub fn auth_connection_id(mut self, value: impl Into<String>) -> Self {
        self.auth_connection_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthConnectionLocator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auth_connection_id`](AuthConnectionLocatorBuilder::auth_connection_id)
    pub fn build(self) -> Result<AuthConnectionLocator, BuildError> {
        Ok(AuthConnectionLocator {
            auth_connection_id: self.auth_connection_id.ok_or_else(|| BuildError::missing_field("auth_connection_id"))?,
        })
    }
}
