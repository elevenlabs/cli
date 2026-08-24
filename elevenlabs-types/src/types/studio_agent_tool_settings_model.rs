pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StudioAgentToolSettingsModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_confirmation: Option<bool>,
}

impl StudioAgentToolSettingsModel {
    pub fn builder() -> StudioAgentToolSettingsModelBuilder {
        <StudioAgentToolSettingsModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StudioAgentToolSettingsModelBuilder {
    skip_confirmation: Option<bool>,
}

impl StudioAgentToolSettingsModelBuilder {
    pub fn skip_confirmation(mut self, value: bool) -> Self {
        self.skip_confirmation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StudioAgentToolSettingsModel`].
    pub fn build(self) -> Result<StudioAgentToolSettingsModel, BuildError> {
        Ok(StudioAgentToolSettingsModel {
            skip_confirmation: self.skip_confirmation,
        })
    }
}
