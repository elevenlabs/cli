pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionUpdateCustomerFacingAgentRequest {
    /// Receptionist settings to change; omitted settings keep their value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ReceptionCustomerFacingConfigPatch>,
    /// New display name for the receptionist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New website widget appearance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget: Option<ReceptionWidgetAppearance>,
}

impl ReceptionUpdateCustomerFacingAgentRequest {
    pub fn builder() -> ReceptionUpdateCustomerFacingAgentRequestBuilder {
        <ReceptionUpdateCustomerFacingAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionUpdateCustomerFacingAgentRequestBuilder {
    config: Option<ReceptionCustomerFacingConfigPatch>,
    name: Option<String>,
    widget: Option<ReceptionWidgetAppearance>,
}

impl ReceptionUpdateCustomerFacingAgentRequestBuilder {
    pub fn config(mut self, value: ReceptionCustomerFacingConfigPatch) -> Self {
        self.config = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn widget(mut self, value: ReceptionWidgetAppearance) -> Self {
        self.widget = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionUpdateCustomerFacingAgentRequest`].
    pub fn build(self) -> Result<ReceptionUpdateCustomerFacingAgentRequest, BuildError> {
        Ok(ReceptionUpdateCustomerFacingAgentRequest {
            config: self.config,
            name: self.name,
            widget: self.widget,
        })
    }
}

