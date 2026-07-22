pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Additional properties describing a Tool to clients.
/// 
/// NOTE: all properties in ToolAnnotations are **hints**.
/// They are not guaranteed to provide a faithful description of
/// tool behavior (including descriptive properties like `title`).
/// 
/// Clients should never make tool use decisions based on ToolAnnotations
/// received from untrusted servers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolAnnotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "readOnlyHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_hint: Option<bool>,
    #[serde(rename = "destructiveHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destructive_hint: Option<bool>,
    #[serde(rename = "idempotentHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotent_hint: Option<bool>,
    #[serde(rename = "openWorldHint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_world_hint: Option<bool>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ToolAnnotations {
    pub fn builder() -> ToolAnnotationsBuilder {
        <ToolAnnotationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolAnnotationsBuilder {
    title: Option<String>,
    read_only_hint: Option<bool>,
    destructive_hint: Option<bool>,
    idempotent_hint: Option<bool>,
    open_world_hint: Option<bool>,
}

impl ToolAnnotationsBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn read_only_hint(mut self, value: bool) -> Self {
        self.read_only_hint = Some(value);
        self
    }

    pub fn destructive_hint(mut self, value: bool) -> Self {
        self.destructive_hint = Some(value);
        self
    }

    pub fn idempotent_hint(mut self, value: bool) -> Self {
        self.idempotent_hint = Some(value);
        self
    }

    pub fn open_world_hint(mut self, value: bool) -> Self {
        self.open_world_hint = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolAnnotations`].
    pub fn build(self) -> Result<ToolAnnotations, BuildError> {
        Ok(ToolAnnotations {
            title: self.title,
            read_only_hint: self.read_only_hint,
            destructive_hint: self.destructive_hint,
            idempotent_hint: self.idempotent_hint,
            open_world_hint: self.open_world_hint,
            extra: Default::default(),
        })
    }
}
