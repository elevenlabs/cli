pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetVoicesResponse {
    /// A list of available voices.
    #[serde(default)]
    pub voices: Vec<Voice>,
}

impl GetVoicesResponse {
    pub fn builder() -> GetVoicesResponseBuilder {
        <GetVoicesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetVoicesResponseBuilder {
    voices: Option<Vec<Voice>>,
}

impl GetVoicesResponseBuilder {
    pub fn voices(mut self, value: Vec<Voice>) -> Self {
        self.voices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetVoicesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voices`](GetVoicesResponseBuilder::voices)
    pub fn build(self) -> Result<GetVoicesResponse, BuildError> {
        Ok(GetVoicesResponse {
            voices: self.voices.ok_or_else(|| BuildError::missing_field("voices"))?,
        })
    }
}
