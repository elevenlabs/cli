pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OTelAttribute {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: HashMap<String, OTelAttributeValueValue>,
}

impl OTelAttribute {
    pub fn builder() -> OTelAttributeBuilder {
        <OTelAttributeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OTelAttributeBuilder {
    key: Option<String>,
    value: Option<HashMap<String, OTelAttributeValueValue>>,
}

impl OTelAttributeBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: HashMap<String, OTelAttributeValueValue>) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OTelAttribute`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](OTelAttributeBuilder::key)
    /// - [`value`](OTelAttributeBuilder::value)
    pub fn build(self) -> Result<OTelAttribute, BuildError> {
        Ok(OTelAttribute {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
