pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A stored procedure plus whether the receptionist can actually run it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionProcedureResponse {
    /// Name of the procedure.
    #[serde(default)]
    pub name: StrippedStr,
    /// When the receptionist should run the procedure, in natural language. The receptionist matches caller requests against it, e.g. 'When the caller wants to book an appointment'.
    #[serde(default)]
    pub trigger: StrippedStr,
    /// Step-by-step instructions, as markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// ID of the procedure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// When the procedure was created, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<UnixTimestamp>,
    /// Set when the procedure was copied from an integration's template; null for procedures you create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key: Option<String>,
    /// Tool API names the steps point at that the receptionist does not have. While non-empty the procedure is left off calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_tools: Option<Vec<String>>,
}

impl ReceptionProcedureResponse {
    pub fn builder() -> ReceptionProcedureResponseBuilder {
        <ReceptionProcedureResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionProcedureResponseBuilder {
    name: Option<StrippedStr>,
    trigger: Option<StrippedStr>,
    body: Option<String>,
    id: Option<String>,
    created_at_unix: Option<UnixTimestamp>,
    source_key: Option<String>,
    missing_tools: Option<Vec<String>>,
}

impl ReceptionProcedureResponseBuilder {
    pub fn name(mut self, value: StrippedStr) -> Self {
        self.name = Some(value);
        self
    }

    pub fn trigger(mut self, value: StrippedStr) -> Self {
        self.trigger = Some(value);
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: UnixTimestamp) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn source_key(mut self, value: impl Into<String>) -> Self {
        self.source_key = Some(value.into());
        self
    }

    pub fn missing_tools(mut self, value: Vec<String>) -> Self {
        self.missing_tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionProcedureResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionProcedureResponseBuilder::name)
    /// - [`trigger`](ReceptionProcedureResponseBuilder::trigger)
    pub fn build(self) -> Result<ReceptionProcedureResponse, BuildError> {
        Ok(ReceptionProcedureResponse {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            trigger: self.trigger.ok_or_else(|| BuildError::missing_field("trigger"))?,
            body: self.body,
            id: self.id,
            created_at_unix: self.created_at_unix,
            source_key: self.source_key,
            missing_tools: self.missing_tools,
        })
    }
}
