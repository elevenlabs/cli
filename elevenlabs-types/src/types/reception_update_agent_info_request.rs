pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionUpdateAgentInfoRequest {
    /// Name of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// ISO 3166-1 alpha-2 country code where the business is based (e.g. 'US', 'PL', 'DE').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_country: Option<CountryCode>,
    /// What the business does, given to the receptionist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_description: Option<String>,
    /// IANA timezone of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Currency code for all prices (ISO 4217)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<ReceptionUpdateAgentInfoRequestCurrency>,
    /// Default business hours. Each location's own operating_hours decide when that location is open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_hours: Option<WorkingHours>,
    /// Ambient sound played while any receptionist runs a tool (e.g. checking availability or looking up a client), filling the silence callers would otherwise hear. One sound for the whole workspace, because custom webhook and MCP tools are shared by every receptionist. Off when null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound: Option<ToolCallSoundType>,
}

impl ReceptionUpdateAgentInfoRequest {
    pub fn builder() -> ReceptionUpdateAgentInfoRequestBuilder {
        <ReceptionUpdateAgentInfoRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionUpdateAgentInfoRequestBuilder {
    business_name: Option<String>,
    business_country: Option<CountryCode>,
    business_description: Option<String>,
    timezone: Option<String>,
    currency: Option<ReceptionUpdateAgentInfoRequestCurrency>,
    operating_hours: Option<WorkingHours>,
    tool_call_sound: Option<ToolCallSoundType>,
}

impl ReceptionUpdateAgentInfoRequestBuilder {
    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn business_country(mut self, value: CountryCode) -> Self {
        self.business_country = Some(value);
        self
    }

    pub fn business_description(mut self, value: impl Into<String>) -> Self {
        self.business_description = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn currency(mut self, value: ReceptionUpdateAgentInfoRequestCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn operating_hours(mut self, value: WorkingHours) -> Self {
        self.operating_hours = Some(value);
        self
    }

    pub fn tool_call_sound(mut self, value: ToolCallSoundType) -> Self {
        self.tool_call_sound = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionUpdateAgentInfoRequest`].
    pub fn build(self) -> Result<ReceptionUpdateAgentInfoRequest, BuildError> {
        Ok(ReceptionUpdateAgentInfoRequest {
            business_name: self.business_name,
            business_country: self.business_country,
            business_description: self.business_description,
            timezone: self.timezone,
            currency: self.currency,
            operating_hours: self.operating_hours,
            tool_call_sound: self.tool_call_sound,
        })
    }
}

