pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single branch/location extracted from a page.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RawLocationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<LenientOptionalStr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<LenientOptionalStr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_hours: Option<HashMap<String, serde_json::Value>>,
}

impl RawLocationInfo {
    pub fn builder() -> RawLocationInfoBuilder {
        <RawLocationInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RawLocationInfoBuilder {
    name: Option<LenientOptionalStr>,
    address: Option<LenientOptionalStr>,
    operating_hours: Option<HashMap<String, serde_json::Value>>,
}

impl RawLocationInfoBuilder {
    pub fn name(mut self, value: LenientOptionalStr) -> Self {
        self.name = Some(value);
        self
    }

    pub fn address(mut self, value: LenientOptionalStr) -> Self {
        self.address = Some(value);
        self
    }

    pub fn operating_hours(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.operating_hours = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RawLocationInfo`].
    pub fn build(self) -> Result<RawLocationInfo, BuildError> {
        Ok(RawLocationInfo {
            name: self.name,
            address: self.address,
            operating_hours: self.operating_hours,
        })
    }
}
