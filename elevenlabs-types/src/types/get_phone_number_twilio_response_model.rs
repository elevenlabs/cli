pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPhoneNumberTwilioResponseModel {
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

impl GetPhoneNumberTwilioResponseModel {
    pub fn builder() -> GetPhoneNumberTwilioResponseModelBuilder {
        <GetPhoneNumberTwilioResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPhoneNumberTwilioResponseModelBuilder {
    phone_number: Option<String>,
    label: Option<String>,
    supports_inbound: Option<bool>,
    supports_outbound: Option<bool>,
    phone_number_id: Option<String>,
    assigned_agent: Option<PhoneNumberAgentInfo>,
}

impl GetPhoneNumberTwilioResponseModelBuilder {
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

    /// Consumes the builder and constructs a [`GetPhoneNumberTwilioResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number`](GetPhoneNumberTwilioResponseModelBuilder::phone_number)
    /// - [`label`](GetPhoneNumberTwilioResponseModelBuilder::label)
    /// - [`phone_number_id`](GetPhoneNumberTwilioResponseModelBuilder::phone_number_id)
    pub fn build(self) -> Result<GetPhoneNumberTwilioResponseModel, BuildError> {
        Ok(GetPhoneNumberTwilioResponseModel {
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            supports_inbound: self.supports_inbound,
            supports_outbound: self.supports_outbound,
            phone_number_id: self.phone_number_id.ok_or_else(|| BuildError::missing_field("phone_number_id"))?,
            assigned_agent: self.assigned_agent,
        })
    }
}
