pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEnvironmentRequest {
    /// Unique name of the environment, e.g. 'staging'.
    #[serde(default)]
    pub name: String,
}

impl CreateEnvironmentRequest {
    pub fn builder() -> CreateEnvironmentRequestBuilder {
        <CreateEnvironmentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEnvironmentRequestBuilder {
    name: Option<String>,
}

impl CreateEnvironmentRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateEnvironmentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateEnvironmentRequestBuilder::name)
    pub fn build(self) -> Result<CreateEnvironmentRequest, BuildError> {
        Ok(CreateEnvironmentRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

