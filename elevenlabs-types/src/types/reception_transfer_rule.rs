pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A configured phone number that the agent can transfer calls to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionTransferRule {
    /// Name of the transfer destination, e.g. 'Alvin' or 'Billing team'.
    #[serde(default)]
    pub label: StrippedStr,
    /// Number to transfer to. Without a country code it is dialed locally in the business's country, so include the country code for a number abroad.
    #[serde(default)]
    pub phone_number: PhoneNumber,
    /// What the caller does that triggers the transfer, e.g. 'The caller asks to speak to a human or to Alvin'. Describe only the trigger; do not restate the phone number or the word 'transfer'.
    #[serde(default)]
    pub condition: StrippedStr,
    /// DTMF digits automatically played after the transfer destination picks up (e.g. '1' to answer a 'press 1 to accept' screen, or 'www1234' to key an extension after a pause). Valid characters are 0-9, *, #, w (0.5s pause) and W (1s pause). Twilio transfers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_dial_digits: Option<String>,
    /// ID of the transfer rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// When the transfer rule was created, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<UnixTimestamp>,
}

impl ReceptionTransferRule {
    pub fn builder() -> ReceptionTransferRuleBuilder {
        <ReceptionTransferRuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionTransferRuleBuilder {
    label: Option<StrippedStr>,
    phone_number: Option<PhoneNumber>,
    condition: Option<StrippedStr>,
    post_dial_digits: Option<String>,
    id: Option<String>,
    created_at_unix: Option<UnixTimestamp>,
}

impl ReceptionTransferRuleBuilder {
    pub fn label(mut self, value: StrippedStr) -> Self {
        self.label = Some(value);
        self
    }

    pub fn phone_number(mut self, value: PhoneNumber) -> Self {
        self.phone_number = Some(value);
        self
    }

    pub fn condition(mut self, value: StrippedStr) -> Self {
        self.condition = Some(value);
        self
    }

    pub fn post_dial_digits(mut self, value: impl Into<String>) -> Self {
        self.post_dial_digits = Some(value.into());
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

    /// Consumes the builder and constructs a [`ReceptionTransferRule`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](ReceptionTransferRuleBuilder::label)
    /// - [`phone_number`](ReceptionTransferRuleBuilder::phone_number)
    /// - [`condition`](ReceptionTransferRuleBuilder::condition)
    pub fn build(self) -> Result<ReceptionTransferRule, BuildError> {
        Ok(ReceptionTransferRule {
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
            condition: self.condition.ok_or_else(|| BuildError::missing_field("condition"))?,
            post_dial_digits: self.post_dial_digits,
            id: self.id,
            created_at_unix: self.created_at_unix,
        })
    }
}
