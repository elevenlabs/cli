pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePhoneNumberResponseModel {
    /// Phone entity ID
    #[serde(default)]
    pub phone_number_id: String,
}

impl CreatePhoneNumberResponseModel {
    pub fn builder() -> CreatePhoneNumberResponseModelBuilder {
        <CreatePhoneNumberResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePhoneNumberResponseModelBuilder {
    phone_number_id: Option<String>,
}

impl CreatePhoneNumberResponseModelBuilder {
    pub fn phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.phone_number_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePhoneNumberResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number_id`](CreatePhoneNumberResponseModelBuilder::phone_number_id)
    pub fn build(self) -> Result<CreatePhoneNumberResponseModel, BuildError> {
        Ok(CreatePhoneNumberResponseModel {
            phone_number_id: self.phone_number_id.ok_or_else(|| BuildError::missing_field("phone_number_id"))?,
        })
    }
}
