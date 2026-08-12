pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConvertProjectResponseModel {
    /// The status of the studio project conversion request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl ConvertProjectResponseModel {
    pub fn builder() -> ConvertProjectResponseModelBuilder {
        <ConvertProjectResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertProjectResponseModelBuilder {
    status: Option<String>,
}

impl ConvertProjectResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvertProjectResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ConvertProjectResponseModelBuilder::status)
    pub fn build(self) -> Result<ConvertProjectResponseModel, BuildError> {
        Ok(ConvertProjectResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
