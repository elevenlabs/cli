pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProcedureListItemResponseModel {
    /// Procedure ID
    #[serde(default)]
    pub procedure_id: String,
    /// Version ID of a version of the procedure. None for a procedure never versioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Procedure name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Procedure type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ProcedureType>,
    /// When the agent should use this procedure. Empty string means this is a sub-procedure that should only start when another procedure references it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    /// True when the procedure has unpublished draft changes on this branch (a newly created or edited procedure not yet published). When true, the name, type, and trigger reflect that draft.
    #[serde(default)]
    pub has_draft: bool,
}

impl ProcedureListItemResponseModel {
    pub fn builder() -> ProcedureListItemResponseModelBuilder {
        <ProcedureListItemResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProcedureListItemResponseModelBuilder {
    procedure_id: Option<String>,
    version_id: Option<String>,
    name: Option<String>,
    r#type: Option<ProcedureType>,
    trigger: Option<String>,
    has_draft: Option<bool>,
}

impl ProcedureListItemResponseModelBuilder {
    pub fn procedure_id(mut self, value: impl Into<String>) -> Self {
        self.procedure_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn has_draft(mut self, value: bool) -> Self {
        self.has_draft = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProcedureListItemResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`procedure_id`](ProcedureListItemResponseModelBuilder::procedure_id)
    /// - [`has_draft`](ProcedureListItemResponseModelBuilder::has_draft)
    pub fn build(self) -> Result<ProcedureListItemResponseModel, BuildError> {
        Ok(ProcedureListItemResponseModel {
            procedure_id: self.procedure_id.ok_or_else(|| BuildError::missing_field("procedure_id"))?,
            version_id: self.version_id,
            name: self.name,
            r#type: self.r#type,
            trigger: self.trigger,
            has_draft: self.has_draft.ok_or_else(|| BuildError::missing_field("has_draft"))?,
        })
    }
}
