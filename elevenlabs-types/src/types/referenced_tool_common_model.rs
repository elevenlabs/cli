pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Reference to a tool for unit test evaluation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReferencedToolCommonModel {
    /// The ID of the tool
    #[serde(default)]
    pub id: String,
    /// The type of the tool
    pub r#type: ReferencedToolCommonModelType,
}

impl ReferencedToolCommonModel {
    pub fn builder() -> ReferencedToolCommonModelBuilder {
        <ReferencedToolCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferencedToolCommonModelBuilder {
    id: Option<String>,
    r#type: Option<ReferencedToolCommonModelType>,
}

impl ReferencedToolCommonModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ReferencedToolCommonModelType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReferencedToolCommonModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReferencedToolCommonModelBuilder::id)
    /// - [`r#type`](ReferencedToolCommonModelBuilder::r#type)
    pub fn build(self) -> Result<ReferencedToolCommonModel, BuildError> {
        Ok(ReferencedToolCommonModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
