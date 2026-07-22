pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentTestResponseModel {
    #[serde(default)]
    pub id: String,
}

impl CreateAgentTestResponseModel {
    pub fn builder() -> CreateAgentTestResponseModelBuilder {
        <CreateAgentTestResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentTestResponseModelBuilder {
    id: Option<String>,
}

impl CreateAgentTestResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentTestResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateAgentTestResponseModelBuilder::id)
    pub fn build(self) -> Result<CreateAgentTestResponseModel, BuildError> {
        Ok(CreateAgentTestResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
