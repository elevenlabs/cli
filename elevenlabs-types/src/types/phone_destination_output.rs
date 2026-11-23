pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PhoneDestinationOutput {
    /// Phone number, SIP URI, or the dynamic variable name that resolves to one.
    #[serde(default)]
    pub destination: String,
}

impl PhoneDestinationOutput {
    pub fn builder() -> PhoneDestinationOutputBuilder {
        <PhoneDestinationOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneDestinationOutputBuilder {
    destination: Option<String>,
}

impl PhoneDestinationOutputBuilder {
    pub fn destination(mut self, value: impl Into<String>) -> Self {
        self.destination = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PhoneDestinationOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](PhoneDestinationOutputBuilder::destination)
    pub fn build(self) -> Result<PhoneDestinationOutput, BuildError> {
        Ok(PhoneDestinationOutput {
            destination: self.destination.ok_or_else(|| BuildError::missing_field("destination"))?,
        })
    }
}
