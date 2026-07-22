pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DependentPhoneNumberIdentifier {
    #[serde(default)]
    pub phone_number_id: String,
    #[serde(default)]
    pub phone_number: String,
    #[serde(default)]
    pub label: String,
    pub provider: TelephonyProvider,
}

impl DependentPhoneNumberIdentifier {
    pub fn builder() -> DependentPhoneNumberIdentifierBuilder {
        <DependentPhoneNumberIdentifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DependentPhoneNumberIdentifierBuilder {
    phone_number_id: Option<String>,
    phone_number: Option<String>,
    label: Option<String>,
    provider: Option<TelephonyProvider>,
}

impl DependentPhoneNumberIdentifierBuilder {
    pub fn phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.phone_number_id = Some(value.into());
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn provider(mut self, value: TelephonyProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DependentPhoneNumberIdentifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number_id`](DependentPhoneNumberIdentifierBuilder::phone_number_id)
    /// - [`phone_number`](DependentPhoneNumberIdentifierBuilder::phone_number)
    /// - [`label`](DependentPhoneNumberIdentifierBuilder::label)
    /// - [`provider`](DependentPhoneNumberIdentifierBuilder::provider)
    pub fn build(self) -> Result<DependentPhoneNumberIdentifier, BuildError> {
        Ok(DependentPhoneNumberIdentifier {
            phone_number_id: self.phone_number_id.ok_or_else(|| BuildError::missing_field("phone_number_id"))?,
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}
