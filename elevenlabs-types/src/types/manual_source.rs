pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ManualSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub created_by_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ManualSource {
    pub fn builder() -> ManualSourceBuilder {
        <ManualSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManualSourceBuilder {
    r#type: Option<String>,
    created_by_user_id: Option<String>,
    notes: Option<String>,
}

impl ManualSourceBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn created_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_user_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ManualSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_by_user_id`](ManualSourceBuilder::created_by_user_id)
    pub fn build(self) -> Result<ManualSource, BuildError> {
        Ok(ManualSource {
            r#type: self.r#type,
            created_by_user_id: self.created_by_user_id.ok_or_else(|| BuildError::missing_field("created_by_user_id"))?,
            notes: self.notes,
        })
    }
}
