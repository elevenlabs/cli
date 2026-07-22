pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PhoneNumberTransferDestination {
    #[serde(default)]
    pub phone_number: String,
}

impl PhoneNumberTransferDestination {
    pub fn builder() -> PhoneNumberTransferDestinationBuilder {
        <PhoneNumberTransferDestinationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneNumberTransferDestinationBuilder {
    phone_number: Option<String>,
}

impl PhoneNumberTransferDestinationBuilder {
    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PhoneNumberTransferDestination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number`](PhoneNumberTransferDestinationBuilder::phone_number)
    pub fn build(self) -> Result<PhoneNumberTransferDestination, BuildError> {
        Ok(PhoneNumberTransferDestination {
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
        })
    }
}
