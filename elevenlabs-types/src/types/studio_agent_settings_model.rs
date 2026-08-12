pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StudioAgentSettingsModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_settings: Option<HashMap<String, StudioAgentToolSettingsModel>>,
}

impl StudioAgentSettingsModel {
    pub fn builder() -> StudioAgentSettingsModelBuilder {
        <StudioAgentSettingsModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StudioAgentSettingsModelBuilder {
    tool_settings: Option<HashMap<String, StudioAgentToolSettingsModel>>,
}

impl StudioAgentSettingsModelBuilder {
    pub fn tool_settings(mut self, value: HashMap<String, StudioAgentToolSettingsModel>) -> Self {
        self.tool_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StudioAgentSettingsModel`].
    pub fn build(self) -> Result<StudioAgentSettingsModel, BuildError> {
        Ok(StudioAgentSettingsModel {
            tool_settings: self.tool_settings,
        })
    }
}
