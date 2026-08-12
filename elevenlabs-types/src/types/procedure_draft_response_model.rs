pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProcedureDraftResponseModel {
    /// Procedure ID
    #[serde(default)]
    pub procedure_id: String,
    /// Procedure name
    #[serde(default)]
    pub name: String,
    /// Procedure content
    #[serde(default)]
    pub content: String,
    /// Procedure type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ProcedureType>,
    /// When the agent should use this procedure. Empty string means this is a sub-procedure that should only start when another procedure references it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}

impl ProcedureDraftResponseModel {
    pub fn builder() -> ProcedureDraftResponseModelBuilder {
        <ProcedureDraftResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProcedureDraftResponseModelBuilder {
    procedure_id: Option<String>,
    name: Option<String>,
    content: Option<String>,
    r#type: Option<ProcedureType>,
    trigger: Option<String>,
}

impl ProcedureDraftResponseModelBuilder {
    pub fn procedure_id(mut self, value: impl Into<String>) -> Self {
        self.procedure_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ProcedureType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn trigger(mut self, value: impl Into<String>) -> Self {
        self.trigger = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProcedureDraftResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`procedure_id`](ProcedureDraftResponseModelBuilder::procedure_id)
    /// - [`name`](ProcedureDraftResponseModelBuilder::name)
    /// - [`content`](ProcedureDraftResponseModelBuilder::content)
    pub fn build(self) -> Result<ProcedureDraftResponseModel, BuildError> {
        Ok(ProcedureDraftResponseModel {
            procedure_id: self.procedure_id.ok_or_else(|| BuildError::missing_field("procedure_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type,
            trigger: self.trigger,
        })
    }
}
