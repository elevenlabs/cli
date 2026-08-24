pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetAgentEmbedResponseModel {
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub widget_config: WidgetConfigResponse,
}

impl GetAgentEmbedResponseModel {
    pub fn builder() -> GetAgentEmbedResponseModelBuilder {
        <GetAgentEmbedResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgentEmbedResponseModelBuilder {
    agent_id: Option<String>,
    widget_config: Option<WidgetConfigResponse>,
}

impl GetAgentEmbedResponseModelBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn widget_config(mut self, value: WidgetConfigResponse) -> Self {
        self.widget_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAgentEmbedResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](GetAgentEmbedResponseModelBuilder::agent_id)
    /// - [`widget_config`](GetAgentEmbedResponseModelBuilder::widget_config)
    pub fn build(self) -> Result<GetAgentEmbedResponseModel, BuildError> {
        Ok(GetAgentEmbedResponseModel {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            widget_config: self.widget_config.ok_or_else(|| BuildError::missing_field("widget_config"))?,
        })
    }
}
