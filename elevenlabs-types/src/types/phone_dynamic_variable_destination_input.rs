pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PhoneDynamicVariableDestinationInput {
    /// Phone number, SIP URI, or the dynamic variable name that resolves to one.
    #[serde(default)]
    pub destination: String,
}

impl PhoneDynamicVariableDestinationInput {
    pub fn builder() -> PhoneDynamicVariableDestinationInputBuilder {
        <PhoneDynamicVariableDestinationInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneDynamicVariableDestinationInputBuilder {
    destination: Option<String>,
}

impl PhoneDynamicVariableDestinationInputBuilder {
    pub fn destination(mut self, value: impl Into<String>) -> Self {
        self.destination = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PhoneDynamicVariableDestinationInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](PhoneDynamicVariableDestinationInputBuilder::destination)
    pub fn build(self) -> Result<PhoneDynamicVariableDestinationInput, BuildError> {
        Ok(PhoneDynamicVariableDestinationInput {
            destination: self.destination.ok_or_else(|| BuildError::missing_field("destination"))?,
        })
    }
}
