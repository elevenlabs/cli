pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPost {
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub from_number: String,
    #[serde(default)]
    pub to_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<TelephonyDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
}

impl BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPost {
    pub fn builder() -> BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPostBuilder {
        <BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPostBuilder {
    agent_id: Option<String>,
    from_number: Option<String>,
    to_number: Option<String>,
    direction: Option<TelephonyDirection>,
    conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
}

impl BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPostBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn from_number(mut self, value: impl Into<String>) -> Self {
        self.from_number = Some(value.into());
        self
    }

    pub fn to_number(mut self, value: impl Into<String>) -> Self {
        self.to_number = Some(value.into());
        self
    }

    pub fn direction(mut self, value: TelephonyDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn conversation_initiation_client_data(mut self, value: ConversationInitiationClientDataRequestInput) -> Self {
        self.conversation_initiation_client_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPostBuilder::agent_id)
    /// - [`from_number`](BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPostBuilder::from_number)
    /// - [`to_number`](BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPostBuilder::to_number)
    pub fn build(self) -> Result<BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPost, BuildError> {
        Ok(BodyRegisterATwilioCallAndReturnTwiMlV1ConvaiTwilioRegisterCallPost {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            from_number: self.from_number.ok_or_else(|| BuildError::missing_field("from_number"))?,
            to_number: self.to_number.ok_or_else(|| BuildError::missing_field("to_number"))?,
            direction: self.direction,
            conversation_initiation_client_data: self.conversation_initiation_client_data,
        })
    }
}

