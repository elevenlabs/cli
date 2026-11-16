pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A phone number held by the workspace.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionPhoneAssignment {
    /// Phone number in E.164 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<PhoneNumber>,
    /// True when the phone is bring-your-own: registered by the workspace using their own telephony provider (Twilio credentials or SIP trunk). BYO phones are never part of the shared pool, do not count against the plan's phone number limit, and are deleted (rather than returned to the pool) on release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_byo: Option<bool>,
    /// Telephony provider for this phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ReceptionPhoneProvider>,
    /// US/Canada NANP area code (NPA) as an integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanp_area_code: Option<i64>,
    /// The receptionist that answers this number. Null when the workspace holds the number but no receptionist answers it yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_facing_agent_id: Option<String>,
    /// Unix timestamp when this number was deactivated; it is released 14 days later. Null when active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated_at_unix: Option<i64>,
    /// Why this number was deactivated. Null when active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivation_reason: Option<ReceptionPhoneAssignmentDeactivationReason>,
    /// Null for numbers that need no paperwork. `held` while the workspace's own application is under review, `approved` once it passed and the number is fully usable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatory_status: Option<ReceptionPhoneRegulatoryStatus>,
    /// The regulated number type this number was issued as. Null for numbers that need no paperwork.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatory_number_type: Option<ApplicationNumberType>,
    /// ISO country the regulated number was issued in. Null for numbers that need no paperwork.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatory_country: Option<CountryCode>,
    /// The phone number assignment's ID.
    #[serde(default)]
    pub id: String,
    /// When this was created, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<UnixTimestamp>,
    /// When this was last updated, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_unix: Option<UnixTimestamp>,
    /// Whether the number is active; false during the 14 days before a deactivated number is released.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Whether inbound calls from the public may reach the receptionist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts_public_calls: Option<bool>,
}

impl ReceptionPhoneAssignment {
    pub fn builder() -> ReceptionPhoneAssignmentBuilder {
        <ReceptionPhoneAssignmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionPhoneAssignmentBuilder {
    phone_number: Option<PhoneNumber>,
    is_byo: Option<bool>,
    provider: Option<ReceptionPhoneProvider>,
    nanp_area_code: Option<i64>,
    customer_facing_agent_id: Option<String>,
    deactivated_at_unix: Option<i64>,
    deactivation_reason: Option<ReceptionPhoneAssignmentDeactivationReason>,
    regulatory_status: Option<ReceptionPhoneRegulatoryStatus>,
    regulatory_number_type: Option<ApplicationNumberType>,
    regulatory_country: Option<CountryCode>,
    id: Option<String>,
    created_at_unix: Option<UnixTimestamp>,
    updated_at_unix: Option<UnixTimestamp>,
    is_active: Option<bool>,
    accepts_public_calls: Option<bool>,
}

impl ReceptionPhoneAssignmentBuilder {
    pub fn phone_number(mut self, value: PhoneNumber) -> Self {
        self.phone_number = Some(value);
        self
    }

    pub fn is_byo(mut self, value: bool) -> Self {
        self.is_byo = Some(value);
        self
    }

    pub fn provider(mut self, value: ReceptionPhoneProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn nanp_area_code(mut self, value: i64) -> Self {
        self.nanp_area_code = Some(value);
        self
    }

    pub fn customer_facing_agent_id(mut self, value: impl Into<String>) -> Self {
        self.customer_facing_agent_id = Some(value.into());
        self
    }

    pub fn deactivated_at_unix(mut self, value: i64) -> Self {
        self.deactivated_at_unix = Some(value);
        self
    }

    pub fn deactivation_reason(mut self, value: ReceptionPhoneAssignmentDeactivationReason) -> Self {
        self.deactivation_reason = Some(value);
        self
    }

    pub fn regulatory_status(mut self, value: ReceptionPhoneRegulatoryStatus) -> Self {
        self.regulatory_status = Some(value);
        self
    }

    pub fn regulatory_number_type(mut self, value: ApplicationNumberType) -> Self {
        self.regulatory_number_type = Some(value);
        self
    }

    pub fn regulatory_country(mut self, value: CountryCode) -> Self {
        self.regulatory_country = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: UnixTimestamp) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn updated_at_unix(mut self, value: UnixTimestamp) -> Self {
        self.updated_at_unix = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn accepts_public_calls(mut self, value: bool) -> Self {
        self.accepts_public_calls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionPhoneAssignment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReceptionPhoneAssignmentBuilder::id)
    pub fn build(self) -> Result<ReceptionPhoneAssignment, BuildError> {
        Ok(ReceptionPhoneAssignment {
            phone_number: self.phone_number,
            is_byo: self.is_byo,
            provider: self.provider,
            nanp_area_code: self.nanp_area_code,
            customer_facing_agent_id: self.customer_facing_agent_id,
            deactivated_at_unix: self.deactivated_at_unix,
            deactivation_reason: self.deactivation_reason,
            regulatory_status: self.regulatory_status,
            regulatory_number_type: self.regulatory_number_type,
            regulatory_country: self.regulatory_country,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at_unix: self.created_at_unix,
            updated_at_unix: self.updated_at_unix,
            is_active: self.is_active,
            accepts_public_calls: self.accepts_public_calls,
        })
    }
}
