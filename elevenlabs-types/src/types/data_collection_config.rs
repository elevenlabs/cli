pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The agent's data collection items that steer follow-up calls. Each defaults
/// to the built-in item for its purpose; `null` turns it off.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DataCollectionConfig {
    /// The item holding when the contact asked to be called back.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_time: Option<String>,
    /// The item holding another number the contact asked to be called on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_phone_number: Option<String>,
    /// The item that is true when the contact asked not to be called again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
    /// Copies data collection results into dynamic variables for later calls: each key is a data collection item ID and its value the variable's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<HashMap<String, String>>,
}

impl DataCollectionConfig {
    pub fn builder() -> DataCollectionConfigBuilder {
        <DataCollectionConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataCollectionConfigBuilder {
    callback_time: Option<String>,
    redirect_phone_number: Option<String>,
    stop: Option<String>,
    mappings: Option<HashMap<String, String>>,
}

impl DataCollectionConfigBuilder {
    pub fn callback_time(mut self, value: impl Into<String>) -> Self {
        self.callback_time = Some(value.into());
        self
    }

    pub fn redirect_phone_number(mut self, value: impl Into<String>) -> Self {
        self.redirect_phone_number = Some(value.into());
        self
    }

    pub fn stop(mut self, value: impl Into<String>) -> Self {
        self.stop = Some(value.into());
        self
    }

    pub fn mappings(mut self, value: HashMap<String, String>) -> Self {
        self.mappings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DataCollectionConfig`].
    pub fn build(self) -> Result<DataCollectionConfig, BuildError> {
        Ok(DataCollectionConfig {
            callback_time: self.callback_time,
            redirect_phone_number: self.redirect_phone_number,
            stop: self.stop,
            mappings: self.mappings,
        })
    }
}
