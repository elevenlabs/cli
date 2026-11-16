pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PhoneDestinationInput {
    /// Phone number, SIP URI, or the dynamic variable name that resolves to one.
    #[serde(default)]
    pub destination: String,
}

impl PhoneDestinationInput {
    pub fn builder() -> PhoneDestinationInputBuilder {
        <PhoneDestinationInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneDestinationInputBuilder {
    destination: Option<String>,
}

impl PhoneDestinationInputBuilder {
    pub fn destination(mut self, value: impl Into<String>) -> Self {
        self.destination = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PhoneDestinationInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](PhoneDestinationInputBuilder::destination)
    pub fn build(self) -> Result<PhoneDestinationInput, BuildError> {
        Ok(PhoneDestinationInput {
            destination: self.destination.ok_or_else(|| BuildError::missing_field("destination"))?,
        })
    }
}
