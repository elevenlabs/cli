pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProcedureRequestModel {
    /// Procedure name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Initial procedure content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Procedure type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ProcedureType>,
    /// When the agent should use this procedure. Empty string means this is a sub-procedure that should only start when another procedure references it. If omitted or null, the trigger is derived from the content instead. Also accepts `description` as an alias.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}

impl CreateProcedureRequestModel {
    pub fn builder() -> CreateProcedureRequestModelBuilder {
        <CreateProcedureRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProcedureRequestModelBuilder {
    name: Option<String>,
    content: Option<String>,
    r#type: Option<ProcedureType>,
    trigger: Option<String>,
}

impl CreateProcedureRequestModelBuilder {
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

    /// Consumes the builder and constructs a [`CreateProcedureRequestModel`].
    pub fn build(self) -> Result<CreateProcedureRequestModel, BuildError> {
        Ok(CreateProcedureRequestModel {
            name: self.name,
            content: self.content,
            r#type: self.r#type,
            trigger: self.trigger,
        })
    }
}
