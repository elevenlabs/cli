pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PhoneDynamicVariableDestinationOutput {
    /// Phone number, SIP URI, or the dynamic variable name that resolves to one.
    #[serde(default)]
    pub destination: String,
}

impl PhoneDynamicVariableDestinationOutput {
    pub fn builder() -> PhoneDynamicVariableDestinationOutputBuilder {
        <PhoneDynamicVariableDestinationOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneDynamicVariableDestinationOutputBuilder {
    destination: Option<String>,
}

impl PhoneDynamicVariableDestinationOutputBuilder {
    pub fn destination(mut self, value: impl Into<String>) -> Self {
        self.destination = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PhoneDynamicVariableDestinationOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](PhoneDynamicVariableDestinationOutputBuilder::destination)
    pub fn build(self) -> Result<PhoneDynamicVariableDestinationOutput, BuildError> {
        Ok(PhoneDynamicVariableDestinationOutput {
            destination: self.destination.ok_or_else(|| BuildError::missing_field("destination"))?,
        })
    }
}
