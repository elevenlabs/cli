pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Device information.
/// 
/// Spec: https://schema.ocsf.io/1.6.0/objects/device
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceModel {
    /// IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Device hostname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Device type ID (99 = Unknown)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id: Option<i64>,
}

impl DeviceModel {
    pub fn builder() -> DeviceModelBuilder {
        <DeviceModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceModelBuilder {
    ip: Option<String>,
    hostname: Option<String>,
    type_id: Option<i64>,
}

impl DeviceModelBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    pub fn type_id(mut self, value: i64) -> Self {
        self.type_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeviceModel`].
    pub fn build(self) -> Result<DeviceModel, BuildError> {
        Ok(DeviceModel {
            ip: self.ip,
            hostname: self.hostname,
            type_id: self.type_id,
        })
    }
}
