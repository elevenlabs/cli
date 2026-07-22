pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPhoneNumberExotelResponseModel {
    /// Phone number
    #[serde(default)]
    pub phone_number: String,
    /// Label for the phone number
    #[serde(default)]
    pub label: String,
    /// This field is deprecated and will be removed in the future. Whether this phone number supports inbound calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inbound: Option<bool>,
    /// This field is deprecated and will be removed in the future. Whether this phone number supports outbound calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_outbound: Option<bool>,
    /// The ID of the phone number
    #[serde(default)]
    pub phone_number_id: String,
    /// The agent that is assigned to the phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_agent: Option<PhoneNumberAgentInfo>,
}

impl GetPhoneNumberExotelResponseModel {
    pub fn builder() -> GetPhoneNumberExotelResponseModelBuilder {
        <GetPhoneNumberExotelResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPhoneNumberExotelResponseModelBuilder {
    phone_number: Option<String>,
    label: Option<String>,
    supports_inbound: Option<bool>,
    supports_outbound: Option<bool>,
    phone_number_id: Option<String>,
    assigned_agent: Option<PhoneNumberAgentInfo>,
}

impl GetPhoneNumberExotelResponseModelBuilder {
    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn supports_inbound(mut self, value: bool) -> Self {
        self.supports_inbound = Some(value);
        self
    }

    pub fn supports_outbound(mut self, value: bool) -> Self {
        self.supports_outbound = Some(value);
        self
    }

    pub fn phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.phone_number_id = Some(value.into());
        self
    }

    pub fn assigned_agent(mut self, value: PhoneNumberAgentInfo) -> Self {
        self.assigned_agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPhoneNumberExotelResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number`](GetPhoneNumberExotelResponseModelBuilder::phone_number)
    /// - [`label`](GetPhoneNumberExotelResponseModelBuilder::label)
    /// - [`phone_number_id`](GetPhoneNumberExotelResponseModelBuilder::phone_number_id)
    pub fn build(self) -> Result<GetPhoneNumberExotelResponseModel, BuildError> {
        Ok(GetPhoneNumberExotelResponseModel {
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            supports_inbound: self.supports_inbound,
            supports_outbound: self.supports_outbound,
            phone_number_id: self.phone_number_id.ok_or_else(|| BuildError::missing_field("phone_number_id"))?,
            assigned_agent: self.assigned_agent,
        })
    }
}
