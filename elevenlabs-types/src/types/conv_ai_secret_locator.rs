pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Used to reference a secret from the agent's secret store.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ConvAiSecretLocator {
    pub secret_id: String,
}

impl ConvAiSecretLocator {
    pub fn builder() -> ConvAiSecretLocatorBuilder {
        <ConvAiSecretLocatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvAiSecretLocatorBuilder {
    secret_id: Option<String>,
}

impl ConvAiSecretLocatorBuilder {
    pub fn secret_id(mut self, value: impl Into<String>) -> Self {
        self.secret_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvAiSecretLocator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`secret_id`](ConvAiSecretLocatorBuilder::secret_id)
    pub fn build(self) -> Result<ConvAiSecretLocator, BuildError> {
        Ok(ConvAiSecretLocator {
            secret_id: self.secret_id.ok_or_else(|| BuildError::missing_field("secret_id"))?,
        })
    }
}
