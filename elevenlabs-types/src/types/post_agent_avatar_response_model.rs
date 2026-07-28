pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostAgentAvatarResponseModel {
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

impl PostAgentAvatarResponseModel {
    pub fn builder() -> PostAgentAvatarResponseModelBuilder {
        <PostAgentAvatarResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostAgentAvatarResponseModelBuilder {
    agent_id: Option<String>,
    avatar_url: Option<String>,
}

impl PostAgentAvatarResponseModelBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostAgentAvatarResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](PostAgentAvatarResponseModelBuilder::agent_id)
    pub fn build(self) -> Result<PostAgentAvatarResponseModel, BuildError> {
        Ok(PostAgentAvatarResponseModel {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            avatar_url: self.avatar_url,
        })
    }
}
