pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SimulationLibrarySettings {
}

impl SimulationLibrarySettings {
    pub fn builder() -> SimulationLibrarySettingsBuilder {
        <SimulationLibrarySettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SimulationLibrarySettingsBuilder {
}

impl SimulationLibrarySettingsBuilder {

    /// Consumes the builder and constructs a [`SimulationLibrarySettings`].
    pub fn build(self) -> Result<SimulationLibrarySettings, BuildError> {
        Ok(SimulationLibrarySettings {
        })
    }
}
