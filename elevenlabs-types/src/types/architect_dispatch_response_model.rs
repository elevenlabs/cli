pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ArchitectDispatchResponseModel {
    #[serde(default)]
    pub architect_chat_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub dispatched_at_unix_secs: i64,
}

impl ArchitectDispatchResponseModel {
    pub fn builder() -> ArchitectDispatchResponseModelBuilder {
        <ArchitectDispatchResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArchitectDispatchResponseModelBuilder {
    architect_chat_id: Option<String>,
    user_id: Option<String>,
    dispatched_at_unix_secs: Option<i64>,
}

impl ArchitectDispatchResponseModelBuilder {
    pub fn architect_chat_id(mut self, value: impl Into<String>) -> Self {
        self.architect_chat_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn dispatched_at_unix_secs(mut self, value: i64) -> Self {
        self.dispatched_at_unix_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ArchitectDispatchResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`architect_chat_id`](ArchitectDispatchResponseModelBuilder::architect_chat_id)
    /// - [`user_id`](ArchitectDispatchResponseModelBuilder::user_id)
    /// - [`dispatched_at_unix_secs`](ArchitectDispatchResponseModelBuilder::dispatched_at_unix_secs)
    pub fn build(self) -> Result<ArchitectDispatchResponseModel, BuildError> {
        Ok(ArchitectDispatchResponseModel {
            architect_chat_id: self.architect_chat_id.ok_or_else(|| BuildError::missing_field("architect_chat_id"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            dispatched_at_unix_secs: self.dispatched_at_unix_secs.ok_or_else(|| BuildError::missing_field("dispatched_at_unix_secs"))?,
        })
    }
}
