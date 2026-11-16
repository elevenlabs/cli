pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A receptionist.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionCustomerFacingAgentResponse {
    /// ID of the ElevenLabs agent behind this receptionist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// The receptionist's voice, language, greeting, rules and other settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ReceptionCustomerFacingConfigOutput>,
    /// Unix timestamp when this receptionist was deactivated. None when active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated_at_unix: Option<i64>,
    /// Why this receptionist was deactivated. None when active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivation_reason: Option<ReceptionCustomerFacingAgentDeactivationReason>,
    /// Display name of the receptionist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// False when the receptionist's underlying agent was deleted; it cannot take calls until it is set up again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_exists: Option<bool>,
    /// Whether this receptionist can configure and use transfer rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_transfer_allowed: Option<bool>,
    /// How the website widget looks. Null when the agent is missing. Unset labels follow the receptionist's language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget: Option<ReceptionWidgetAppearance>,
    /// Whether the receptionist is active. Inactive receptionists take no live calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl ReceptionCustomerFacingAgentResponse {
    pub fn builder() -> ReceptionCustomerFacingAgentResponseBuilder {
        <ReceptionCustomerFacingAgentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionCustomerFacingAgentResponseBuilder {
    agent_id: Option<String>,
    config: Option<ReceptionCustomerFacingConfigOutput>,
    deactivated_at_unix: Option<i64>,
    deactivation_reason: Option<ReceptionCustomerFacingAgentDeactivationReason>,
    name: Option<String>,
    agent_exists: Option<bool>,
    is_transfer_allowed: Option<bool>,
    widget: Option<ReceptionWidgetAppearance>,
    is_active: Option<bool>,
}

impl ReceptionCustomerFacingAgentResponseBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn config(mut self, value: ReceptionCustomerFacingConfigOutput) -> Self {
        self.config = Some(value);
        self
    }

    pub fn deactivated_at_unix(mut self, value: i64) -> Self {
        self.deactivated_at_unix = Some(value);
        self
    }

    pub fn deactivation_reason(mut self, value: ReceptionCustomerFacingAgentDeactivationReason) -> Self {
        self.deactivation_reason = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn agent_exists(mut self, value: bool) -> Self {
        self.agent_exists = Some(value);
        self
    }

    pub fn is_transfer_allowed(mut self, value: bool) -> Self {
        self.is_transfer_allowed = Some(value);
        self
    }

    pub fn widget(mut self, value: ReceptionWidgetAppearance) -> Self {
        self.widget = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionCustomerFacingAgentResponse`].
    pub fn build(self) -> Result<ReceptionCustomerFacingAgentResponse, BuildError> {
        Ok(ReceptionCustomerFacingAgentResponse {
            agent_id: self.agent_id,
            config: self.config,
            deactivated_at_unix: self.deactivated_at_unix,
            deactivation_reason: self.deactivation_reason,
            name: self.name,
            agent_exists: self.agent_exists,
            is_transfer_allowed: self.is_transfer_allowed,
            widget: self.widget,
            is_active: self.is_active,
        })
    }
}
