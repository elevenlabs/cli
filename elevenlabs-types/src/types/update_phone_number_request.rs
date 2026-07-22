pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePhoneNumberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_trunk_config: Option<InboundSipTrunkConfigRequestModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_trunk_config: Option<OutboundSipTrunkConfigRequestModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livekit_stack: Option<LivekitStackType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_sip_messages: Option<bool>,
    /// Environment to use for resolving environment variables on calls to this number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Agent branch to use for calls to this number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
}

impl UpdatePhoneNumberRequest {
    pub fn builder() -> UpdatePhoneNumberRequestBuilder {
        <UpdatePhoneNumberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePhoneNumberRequestBuilder {
    agent_id: Option<String>,
    label: Option<String>,
    inbound_trunk_config: Option<InboundSipTrunkConfigRequestModel>,
    outbound_trunk_config: Option<OutboundSipTrunkConfigRequestModel>,
    livekit_stack: Option<LivekitStackType>,
    store_sip_messages: Option<bool>,
    environment: Option<String>,
    branch_id: Option<String>,
}

impl UpdatePhoneNumberRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn inbound_trunk_config(mut self, value: InboundSipTrunkConfigRequestModel) -> Self {
        self.inbound_trunk_config = Some(value);
        self
    }

    pub fn outbound_trunk_config(mut self, value: OutboundSipTrunkConfigRequestModel) -> Self {
        self.outbound_trunk_config = Some(value);
        self
    }

    pub fn livekit_stack(mut self, value: LivekitStackType) -> Self {
        self.livekit_stack = Some(value);
        self
    }

    pub fn store_sip_messages(mut self, value: bool) -> Self {
        self.store_sip_messages = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePhoneNumberRequest`].
    pub fn build(self) -> Result<UpdatePhoneNumberRequest, BuildError> {
        Ok(UpdatePhoneNumberRequest {
            agent_id: self.agent_id,
            label: self.label,
            inbound_trunk_config: self.inbound_trunk_config,
            outbound_trunk_config: self.outbound_trunk_config,
            livekit_stack: self.livekit_stack,
            store_sip_messages: self.store_sip_messages,
            environment: self.environment,
            branch_id: self.branch_id,
        })
    }
}

