pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateProcedureDraftRequestModel {
    /// Procedure name
    #[serde(default)]
    pub name: String,
    /// Procedure content
    #[serde(default)]
    pub content: String,
    /// Procedure type
    pub r#type: ProcedureType,
    /// When the agent should use this procedure. Empty string means this is a sub-procedure that should only start when another procedure references it. If omitted or null, the trigger is derived from the content instead. Also accepts `description` as an alias.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}

impl UpdateProcedureDraftRequestModel {
    pub fn builder() -> UpdateProcedureDraftRequestModelBuilder {
        <UpdateProcedureDraftRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateProcedureDraftRequestModelBuilder {
    name: Option<String>,
    content: Option<String>,
    r#type: Option<ProcedureType>,
    trigger: Option<String>,
}

impl UpdateProcedureDraftRequestModelBuilder {
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

    /// Consumes the builder and constructs a [`UpdateProcedureDraftRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateProcedureDraftRequestModelBuilder::name)
    /// - [`content`](UpdateProcedureDraftRequestModelBuilder::content)
    /// - [`r#type`](UpdateProcedureDraftRequestModelBuilder::r#type)
    pub fn build(self) -> Result<UpdateProcedureDraftRequestModel, BuildError> {
        Ok(UpdateProcedureDraftRequestModel {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            trigger: self.trigger,
        })
    }
}

