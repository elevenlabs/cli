pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallView {
    #[serde(default)]
    pub intent_id: String,
    #[serde(default)]
    pub lead_id: String,
    /// `initial` for a lead's first call, else a `retry`, `callback` or `redirect`.
    pub intent_type: CampaignConversationType,
    /// `1` for the first call and `+1` per retry of it; unset for callbacks, redirects and their retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cold_attempt: Option<i64>,
    /// `submitted` calls have been handed off for dialing; `attention` calls are held back by an attention item.
    pub status: CallStatus,
    #[serde(default)]
    pub phone_number: String,
    /// When the call is due to be placed.
    #[serde(default)]
    pub scheduled_time_unix: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// The call's final dialing status, set once it ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<BatchCallRecipientStatus>,
    /// How the call ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_result: Option<CallResult>,
    /// When the call ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome_at_unix: Option<i64>,
    /// What the campaign did after the call, such as queue a retry or a callback, or finish the lead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_disposition: Option<DecisionDisposition>,
    /// Data collection results this call copied into the lead's dynamic variables for later calls, with secrets redacted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collected_dynamic_variables: Option<HashMap<String, serde_json::Value>>,
    /// The message's delivery; set on message campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<LeadDeliveryView>,
}

impl CallView {
    pub fn builder() -> CallViewBuilder {
        <CallViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CallViewBuilder {
    intent_id: Option<String>,
    lead_id: Option<String>,
    intent_type: Option<CampaignConversationType>,
    cold_attempt: Option<i64>,
    status: Option<CallStatus>,
    phone_number: Option<String>,
    scheduled_time_unix: Option<i64>,
    conversation_id: Option<String>,
    outcome: Option<BatchCallRecipientStatus>,
    call_result: Option<CallResult>,
    outcome_at_unix: Option<i64>,
    decision_disposition: Option<DecisionDisposition>,
    collected_dynamic_variables: Option<HashMap<String, serde_json::Value>>,
    delivery: Option<LeadDeliveryView>,
}

impl CallViewBuilder {
    pub fn intent_id(mut self, value: impl Into<String>) -> Self {
        self.intent_id = Some(value.into());
        self
    }

    pub fn lead_id(mut self, value: impl Into<String>) -> Self {
        self.lead_id = Some(value.into());
        self
    }

    pub fn intent_type(mut self, value: CampaignConversationType) -> Self {
        self.intent_type = Some(value);
        self
    }

    pub fn cold_attempt(mut self, value: i64) -> Self {
        self.cold_attempt = Some(value);
        self
    }

    pub fn status(mut self, value: CallStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn scheduled_time_unix(mut self, value: i64) -> Self {
        self.scheduled_time_unix = Some(value);
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn outcome(mut self, value: BatchCallRecipientStatus) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn call_result(mut self, value: CallResult) -> Self {
        self.call_result = Some(value);
        self
    }

    pub fn outcome_at_unix(mut self, value: i64) -> Self {
        self.outcome_at_unix = Some(value);
        self
    }

    pub fn decision_disposition(mut self, value: DecisionDisposition) -> Self {
        self.decision_disposition = Some(value);
        self
    }

    pub fn collected_dynamic_variables(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.collected_dynamic_variables = Some(value);
        self
    }

    pub fn delivery(mut self, value: LeadDeliveryView) -> Self {
        self.delivery = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CallView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`intent_id`](CallViewBuilder::intent_id)
    /// - [`lead_id`](CallViewBuilder::lead_id)
    /// - [`intent_type`](CallViewBuilder::intent_type)
    /// - [`status`](CallViewBuilder::status)
    /// - [`phone_number`](CallViewBuilder::phone_number)
    /// - [`scheduled_time_unix`](CallViewBuilder::scheduled_time_unix)
    pub fn build(self) -> Result<CallView, BuildError> {
        Ok(CallView {
            intent_id: self.intent_id.ok_or_else(|| BuildError::missing_field("intent_id"))?,
            lead_id: self.lead_id.ok_or_else(|| BuildError::missing_field("lead_id"))?,
            intent_type: self.intent_type.ok_or_else(|| BuildError::missing_field("intent_type"))?,
            cold_attempt: self.cold_attempt,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
            scheduled_time_unix: self.scheduled_time_unix.ok_or_else(|| BuildError::missing_field("scheduled_time_unix"))?,
            conversation_id: self.conversation_id,
            outcome: self.outcome,
            call_result: self.call_result,
            outcome_at_unix: self.outcome_at_unix,
            decision_disposition: self.decision_disposition,
            collected_dynamic_variables: self.collected_dynamic_variables,
            delivery: self.delivery,
        })
    }
}
