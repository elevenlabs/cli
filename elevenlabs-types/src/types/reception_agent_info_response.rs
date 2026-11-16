pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The business profile, with the current time in the business's timezone.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionAgentInfoResponse {
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
    pub currency: Option<ReceptionAgentInfoResponseCurrency>,
    /// Default business hours. Each location's own operating_hours decide when that location is open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_hours: Option<WorkingHours>,
    /// Ambient sound played while any receptionist runs a tool (e.g. checking availability or looking up a client), filling the silence callers would otherwise hear. One sound for the whole workspace, because custom webhook and MCP tools are shared by every receptionist. Off when null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound: Option<ToolCallSoundType>,
    /// ID of the business profile.
    #[serde(default)]
    pub id: String,
    /// When the business profile was created, as a Unix timestamp in seconds.
    #[serde(default)]
    pub created_at_unix: i64,
    /// When the business profile was last updated, as a Unix timestamp in seconds.
    #[serde(default)]
    pub updated_at_unix: i64,
    /// Current time in workspace timezone, computed on each request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_now: Option<WorkspaceNow>,
}

impl ReceptionAgentInfoResponse {
    pub fn builder() -> ReceptionAgentInfoResponseBuilder {
        <ReceptionAgentInfoResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionAgentInfoResponseBuilder {
    business_name: Option<String>,
    business_country: Option<CountryCode>,
    business_description: Option<String>,
    timezone: Option<String>,
    currency: Option<ReceptionAgentInfoResponseCurrency>,
    operating_hours: Option<WorkingHours>,
    tool_call_sound: Option<ToolCallSoundType>,
    id: Option<String>,
    created_at_unix: Option<i64>,
    updated_at_unix: Option<i64>,
    workspace_now: Option<WorkspaceNow>,
}

impl ReceptionAgentInfoResponseBuilder {
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

    pub fn currency(mut self, value: ReceptionAgentInfoResponseCurrency) -> Self {
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn updated_at_unix(mut self, value: i64) -> Self {
        self.updated_at_unix = Some(value);
        self
    }

    pub fn workspace_now(mut self, value: WorkspaceNow) -> Self {
        self.workspace_now = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionAgentInfoResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReceptionAgentInfoResponseBuilder::id)
    /// - [`created_at_unix`](ReceptionAgentInfoResponseBuilder::created_at_unix)
    /// - [`updated_at_unix`](ReceptionAgentInfoResponseBuilder::updated_at_unix)
    pub fn build(self) -> Result<ReceptionAgentInfoResponse, BuildError> {
        Ok(ReceptionAgentInfoResponse {
            business_name: self.business_name,
            business_country: self.business_country,
            business_description: self.business_description,
            timezone: self.timezone,
            currency: self.currency,
            operating_hours: self.operating_hours,
            tool_call_sound: self.tool_call_sound,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            updated_at_unix: self.updated_at_unix.ok_or_else(|| BuildError::missing_field("updated_at_unix"))?,
            workspace_now: self.workspace_now,
        })
    }
}
