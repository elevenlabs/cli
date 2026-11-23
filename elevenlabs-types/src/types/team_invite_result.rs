pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TeamInviteResult {
    #[serde(default)]
    pub email: String,
    pub outcome: TeamInviteOutcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_status: Option<String>,
}

impl TeamInviteResult {
    pub fn builder() -> TeamInviteResultBuilder {
        <TeamInviteResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamInviteResultBuilder {
    email: Option<String>,
    outcome: Option<TeamInviteOutcome>,
    error_status: Option<String>,
}

impl TeamInviteResultBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn outcome(mut self, value: TeamInviteOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn error_status(mut self, value: impl Into<String>) -> Self {
        self.error_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TeamInviteResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](TeamInviteResultBuilder::email)
    /// - [`outcome`](TeamInviteResultBuilder::outcome)
    pub fn build(self) -> Result<TeamInviteResult, BuildError> {
        Ok(TeamInviteResult {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            outcome: self.outcome.ok_or_else(|| BuildError::missing_field("outcome"))?,
            error_status: self.error_status,
        })
    }
}
