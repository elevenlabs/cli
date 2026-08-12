pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAuthConnectionsResponse {
    #[serde(default)]
    pub auth_connections: Vec<ListAuthConnectionsResponseAuthConnectionsItem>,
}

impl ListAuthConnectionsResponse {
    pub fn builder() -> ListAuthConnectionsResponseBuilder {
        <ListAuthConnectionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAuthConnectionsResponseBuilder {
    auth_connections: Option<Vec<ListAuthConnectionsResponseAuthConnectionsItem>>,
}

impl ListAuthConnectionsResponseBuilder {
    pub fn auth_connections(mut self, value: Vec<ListAuthConnectionsResponseAuthConnectionsItem>) -> Self {
        self.auth_connections = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAuthConnectionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auth_connections`](ListAuthConnectionsResponseBuilder::auth_connections)
    pub fn build(self) -> Result<ListAuthConnectionsResponse, BuildError> {
        Ok(ListAuthConnectionsResponse {
            auth_connections: self.auth_connections.ok_or_else(|| BuildError::missing_field("auth_connections"))?,
        })
    }
}
