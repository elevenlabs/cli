pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceGroupByNameResponseModel {
    /// The name of the workspace group.
    #[serde(default)]
    pub name: String,
    /// The ID of the workspace group.
    #[serde(default)]
    pub id: String,
    /// The emails of the members of the workspace group.
    #[serde(default)]
    pub members_emails: Vec<String>,
}

impl WorkspaceGroupByNameResponseModel {
    pub fn builder() -> WorkspaceGroupByNameResponseModelBuilder {
        <WorkspaceGroupByNameResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceGroupByNameResponseModelBuilder {
    name: Option<String>,
    id: Option<String>,
    members_emails: Option<Vec<String>>,
}

impl WorkspaceGroupByNameResponseModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn members_emails(mut self, value: Vec<String>) -> Self {
        self.members_emails = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceGroupByNameResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](WorkspaceGroupByNameResponseModelBuilder::name)
    /// - [`id`](WorkspaceGroupByNameResponseModelBuilder::id)
    /// - [`members_emails`](WorkspaceGroupByNameResponseModelBuilder::members_emails)
    pub fn build(self) -> Result<WorkspaceGroupByNameResponseModel, BuildError> {
        Ok(WorkspaceGroupByNameResponseModel {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            members_emails: self.members_emails.ok_or_else(|| BuildError::missing_field("members_emails"))?,
        })
    }
}
