pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct CloseConnection {
    /// End the stream with an empty string
    pub text: String,
}

impl CloseConnection {
    pub fn builder() -> CloseConnectionBuilder {
        <CloseConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CloseConnectionBuilder {
    text: Option<String>,
}

impl CloseConnectionBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CloseConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](CloseConnectionBuilder::text)
    pub fn build(self) -> Result<CloseConnection, BuildError> {
        Ok(CloseConnection {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
