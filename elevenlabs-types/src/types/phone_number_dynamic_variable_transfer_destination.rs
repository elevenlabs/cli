pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PhoneNumberDynamicVariableTransferDestination {
    #[serde(default)]
    pub phone_number: String,
}

impl PhoneNumberDynamicVariableTransferDestination {
    pub fn builder() -> PhoneNumberDynamicVariableTransferDestinationBuilder {
        <PhoneNumberDynamicVariableTransferDestinationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneNumberDynamicVariableTransferDestinationBuilder {
    phone_number: Option<String>,
}

impl PhoneNumberDynamicVariableTransferDestinationBuilder {
    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PhoneNumberDynamicVariableTransferDestination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number`](PhoneNumberDynamicVariableTransferDestinationBuilder::phone_number)
    pub fn build(self) -> Result<PhoneNumberDynamicVariableTransferDestination, BuildError> {
        Ok(PhoneNumberDynamicVariableTransferDestination {
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
        })
    }
}
