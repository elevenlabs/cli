pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LeadView {
    #[serde(default)]
    pub lead_id: String,
    /// The `id` given to this recipient when the campaign was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    /// The number the lead's next call dials. It changes when the lead moves on to another listed number or the contact asks to be called on a different one.
    #[serde(default)]
    pub phone_number: String,
    /// The lead's listed numbers, in calling order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,
    /// The position in `phone_numbers` of the number being called.
    #[serde(default)]
    pub phone_numbers_index: i64,
    /// The contact's identity: the `user_id` submitted in `conversation_initiation_client_data`, else the lead's phone number.
    #[serde(default)]
    pub user_id: String,
    /// Calls placed, counted against the policy's `max_total_calls`.
    #[serde(default)]
    pub attempt_count: i64,
    /// Failed calls to the current number; resets on a new number.
    #[serde(default)]
    pub failure_count: i64,
    /// Callbacks the contact asked for.
    #[serde(default)]
    pub callback_count: i64,
    /// Times the contact asked to be called on another number.
    #[serde(default)]
    pub redirect_count: i64,
    /// Set when the contact asked not to be called, the lead was removed, or the campaign stopped. The lead isn't called again.
    #[serde(default)]
    pub do_not_call: bool,
    /// Why the lead won't be called again; unset while it may be.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_disposition: Option<TerminalDisposition>,
    /// When the lead joined the campaign; unset on older leads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<i64>,
    #[serde(default)]
    pub updated_at_unix: i64,
    /// The dynamic variables submitted for the lead, with secrets redacted.
    #[serde(default)]
    pub dynamic_variables: HashMap<String, serde_json::Value>,
    /// The conversation overrides submitted for the lead.
    #[serde(default)]
    pub overrides: FlattenedConfigOverrides,
    /// The lead's latest message delivery; set on message campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<LeadDeliveryView>,
}

impl LeadView {
    pub fn builder() -> LeadViewBuilder {
        <LeadViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadViewBuilder {
    lead_id: Option<String>,
    recipient_id: Option<String>,
    phone_number: Option<String>,
    phone_numbers: Option<Vec<String>>,
    phone_numbers_index: Option<i64>,
    user_id: Option<String>,
    attempt_count: Option<i64>,
    failure_count: Option<i64>,
    callback_count: Option<i64>,
    redirect_count: Option<i64>,
    do_not_call: Option<bool>,
    terminal_disposition: Option<TerminalDisposition>,
    created_at_unix: Option<i64>,
    updated_at_unix: Option<i64>,
    dynamic_variables: Option<HashMap<String, serde_json::Value>>,
    overrides: Option<FlattenedConfigOverrides>,
    delivery: Option<LeadDeliveryView>,
}

impl LeadViewBuilder {
    pub fn lead_id(mut self, value: impl Into<String>) -> Self {
        self.lead_id = Some(value.into());
        self
    }

    pub fn recipient_id(mut self, value: impl Into<String>) -> Self {
        self.recipient_id = Some(value.into());
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn phone_numbers(mut self, value: Vec<String>) -> Self {
        self.phone_numbers = Some(value);
        self
    }

    pub fn phone_numbers_index(mut self, value: i64) -> Self {
        self.phone_numbers_index = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn attempt_count(mut self, value: i64) -> Self {
        self.attempt_count = Some(value);
        self
    }

    pub fn failure_count(mut self, value: i64) -> Self {
        self.failure_count = Some(value);
        self
    }

    pub fn callback_count(mut self, value: i64) -> Self {
        self.callback_count = Some(value);
        self
    }

    pub fn redirect_count(mut self, value: i64) -> Self {
        self.redirect_count = Some(value);
        self
    }

    pub fn do_not_call(mut self, value: bool) -> Self {
        self.do_not_call = Some(value);
        self
    }

    pub fn terminal_disposition(mut self, value: TerminalDisposition) -> Self {
        self.terminal_disposition = Some(value);
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

    pub fn dynamic_variables(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.dynamic_variables = Some(value);
        self
    }

    pub fn overrides(mut self, value: FlattenedConfigOverrides) -> Self {
        self.overrides = Some(value);
        self
    }

    pub fn delivery(mut self, value: LeadDeliveryView) -> Self {
        self.delivery = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LeadView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lead_id`](LeadViewBuilder::lead_id)
    /// - [`phone_number`](LeadViewBuilder::phone_number)
    /// - [`phone_numbers_index`](LeadViewBuilder::phone_numbers_index)
    /// - [`user_id`](LeadViewBuilder::user_id)
    /// - [`attempt_count`](LeadViewBuilder::attempt_count)
    /// - [`failure_count`](LeadViewBuilder::failure_count)
    /// - [`callback_count`](LeadViewBuilder::callback_count)
    /// - [`redirect_count`](LeadViewBuilder::redirect_count)
    /// - [`do_not_call`](LeadViewBuilder::do_not_call)
    /// - [`updated_at_unix`](LeadViewBuilder::updated_at_unix)
    /// - [`dynamic_variables`](LeadViewBuilder::dynamic_variables)
    /// - [`overrides`](LeadViewBuilder::overrides)
    pub fn build(self) -> Result<LeadView, BuildError> {
        Ok(LeadView {
            lead_id: self.lead_id.ok_or_else(|| BuildError::missing_field("lead_id"))?,
            recipient_id: self.recipient_id,
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
            phone_numbers: self.phone_numbers,
            phone_numbers_index: self.phone_numbers_index.ok_or_else(|| BuildError::missing_field("phone_numbers_index"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            attempt_count: self.attempt_count.ok_or_else(|| BuildError::missing_field("attempt_count"))?,
            failure_count: self.failure_count.ok_or_else(|| BuildError::missing_field("failure_count"))?,
            callback_count: self.callback_count.ok_or_else(|| BuildError::missing_field("callback_count"))?,
            redirect_count: self.redirect_count.ok_or_else(|| BuildError::missing_field("redirect_count"))?,
            do_not_call: self.do_not_call.ok_or_else(|| BuildError::missing_field("do_not_call"))?,
            terminal_disposition: self.terminal_disposition,
            created_at_unix: self.created_at_unix,
            updated_at_unix: self.updated_at_unix.ok_or_else(|| BuildError::missing_field("updated_at_unix"))?,
            dynamic_variables: self.dynamic_variables.ok_or_else(|| BuildError::missing_field("dynamic_variables"))?,
            overrides: self.overrides.ok_or_else(|| BuildError::missing_field("overrides"))?,
            delivery: self.delivery,
        })
    }
}
