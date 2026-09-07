pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OTelResource {
    #[serde(default)]
    pub attributes: Vec<OTelAttribute>,
}

impl OTelResource {
    pub fn builder() -> OTelResourceBuilder {
        <OTelResourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OTelResourceBuilder {
    attributes: Option<Vec<OTelAttribute>>,
}

impl OTelResourceBuilder {
    pub fn attributes(mut self, value: Vec<OTelAttribute>) -> Self {
        self.attributes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OTelResource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributes`](OTelResourceBuilder::attributes)
    pub fn build(self) -> Result<OTelResource, BuildError> {
        Ok(OTelResource {
            attributes: self.attributes.ok_or_else(|| BuildError::missing_field("attributes"))?,
        })
    }
}
