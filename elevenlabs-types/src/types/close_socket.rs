pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload to signal closing the entire WebSocket connection.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct CloseSocket {
    /// If true, closes all contexts and closes the entire WebSocket connection. Any context that was previously set to flush will wait to flush before closing.
    pub close_socket: Option<bool>,
}

impl CloseSocket {
    pub fn builder() -> CloseSocketBuilder {
        <CloseSocketBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CloseSocketBuilder {
    close_socket: Option<bool>,
}

impl CloseSocketBuilder {
    pub fn close_socket(mut self, value: bool) -> Self {
        self.close_socket = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CloseSocket`].
    pub fn build(self) -> Result<CloseSocket, BuildError> {
        Ok(CloseSocket {
            close_socket: self.close_socket,
        })
    }
}
