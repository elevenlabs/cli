pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionCreateProcedureRequest {
    /// Name of the procedure.
    #[serde(default)]
    pub name: StrippedStr,
    /// When the receptionist should run the procedure, in natural language. The receptionist matches caller requests against it, e.g. 'When the caller wants to book an appointment'.
    #[serde(default)]
    pub trigger: StrippedStr,
    /// Step-by-step instructions, as markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl ReceptionCreateProcedureRequest {
    pub fn builder() -> ReceptionCreateProcedureRequestBuilder {
        <ReceptionCreateProcedureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionCreateProcedureRequestBuilder {
    name: Option<StrippedStr>,
    trigger: Option<StrippedStr>,
    body: Option<String>,
}

impl ReceptionCreateProcedureRequestBuilder {
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

    /// Consumes the builder and constructs a [`ReceptionCreateProcedureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionCreateProcedureRequestBuilder::name)
    /// - [`trigger`](ReceptionCreateProcedureRequestBuilder::trigger)
    pub fn build(self) -> Result<ReceptionCreateProcedureRequest, BuildError> {
        Ok(ReceptionCreateProcedureRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            trigger: self.trigger.ok_or_else(|| BuildError::missing_field("trigger"))?,
            body: self.body,
        })
    }
}

