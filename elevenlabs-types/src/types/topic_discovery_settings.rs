pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Per-agent topic-discovery configuration. Cadence and analysis window are
/// managed internally; this only exposes the customer-facing on/off toggle.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TopicDiscoverySettings {
}

impl TopicDiscoverySettings {
    pub fn builder() -> TopicDiscoverySettingsBuilder {
        <TopicDiscoverySettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TopicDiscoverySettingsBuilder {
}

impl TopicDiscoverySettingsBuilder {

    /// Consumes the builder and constructs a [`TopicDiscoverySettings`].
    pub fn build(self) -> Result<TopicDiscoverySettings, BuildError> {
        Ok(TopicDiscoverySettings {
        })
    }
}
