pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RealtimeConversationGroupResponseModel {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub access_info: ResourceAccessInfo,
    #[serde(default)]
    pub sharing: RealtimeConversationGroupSharingSummary,
    #[serde(default)]
    pub created_at_unix_secs: i64,
}

impl RealtimeConversationGroupResponseModel {
    pub fn builder() -> RealtimeConversationGroupResponseModelBuilder {
        <RealtimeConversationGroupResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RealtimeConversationGroupResponseModelBuilder {
    id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    access_info: Option<ResourceAccessInfo>,
    sharing: Option<RealtimeConversationGroupSharingSummary>,
    created_at_unix_secs: Option<i64>,
}

impl RealtimeConversationGroupResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    pub fn sharing(mut self, value: RealtimeConversationGroupSharingSummary) -> Self {
        self.sharing = Some(value);
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RealtimeConversationGroupResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RealtimeConversationGroupResponseModelBuilder::id)
    /// - [`title`](RealtimeConversationGroupResponseModelBuilder::title)
    /// - [`access_info`](RealtimeConversationGroupResponseModelBuilder::access_info)
    /// - [`sharing`](RealtimeConversationGroupResponseModelBuilder::sharing)
    /// - [`created_at_unix_secs`](RealtimeConversationGroupResponseModelBuilder::created_at_unix_secs)
    pub fn build(self) -> Result<RealtimeConversationGroupResponseModel, BuildError> {
        Ok(RealtimeConversationGroupResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description,
            access_info: self.access_info.ok_or_else(|| BuildError::missing_field("access_info"))?,
            sharing: self.sharing.ok_or_else(|| BuildError::missing_field("sharing"))?,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
        })
    }
}
