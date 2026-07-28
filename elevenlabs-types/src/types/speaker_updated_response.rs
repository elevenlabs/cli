pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpeakerUpdatedResponse {
    #[serde(default)]
    pub version: i64,
}

impl SpeakerUpdatedResponse {
    pub fn builder() -> SpeakerUpdatedResponseBuilder {
        <SpeakerUpdatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeakerUpdatedResponseBuilder {
    version: Option<i64>,
}

impl SpeakerUpdatedResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeakerUpdatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](SpeakerUpdatedResponseBuilder::version)
    pub fn build(self) -> Result<SpeakerUpdatedResponse, BuildError> {
        Ok(SpeakerUpdatedResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
