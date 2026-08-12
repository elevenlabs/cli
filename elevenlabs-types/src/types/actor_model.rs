pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// OCSF Actor object - describes the entity that performed the action.
/// 
/// Spec: https://schema.ocsf.io/1.6.0/objects/actor
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ActorModel {
    /// User who performed the action
    #[serde(default)]
    pub user: UserModel,
    /// Client application or service name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// Client application unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_uid: Option<String>,
    /// Session information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<HashMap<String, serde_json::Value>>,
}

impl ActorModel {
    pub fn builder() -> ActorModelBuilder {
        <ActorModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActorModelBuilder {
    user: Option<UserModel>,
    app_name: Option<String>,
    app_uid: Option<String>,
    session: Option<HashMap<String, serde_json::Value>>,
}

impl ActorModelBuilder {
    pub fn user(mut self, value: UserModel) -> Self {
        self.user = Some(value);
        self
    }

    pub fn app_name(mut self, value: impl Into<String>) -> Self {
        self.app_name = Some(value.into());
        self
    }

    pub fn app_uid(mut self, value: impl Into<String>) -> Self {
        self.app_uid = Some(value.into());
        self
    }

    pub fn session(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.session = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ActorModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user`](ActorModelBuilder::user)
    pub fn build(self) -> Result<ActorModel, BuildError> {
        Ok(ActorModel {
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            app_name: self.app_name,
            app_uid: self.app_uid,
            session: self.session,
        })
    }
}
