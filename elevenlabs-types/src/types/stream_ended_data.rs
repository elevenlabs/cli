pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamEndedData {
    pub status: AgentOutputStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AgentOutputError>,
    /// The actions the agent waits on. Answer each `client_tool_call` with a `client_tool_result` item in `input` to continue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_actions: Option<Vec<ClientToolCallRequiredAction>>,
}

impl StreamEndedData {
    pub fn builder() -> StreamEndedDataBuilder {
        <StreamEndedDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamEndedDataBuilder {
    status: Option<AgentOutputStatus>,
    error: Option<AgentOutputError>,
    required_actions: Option<Vec<ClientToolCallRequiredAction>>,
}

impl StreamEndedDataBuilder {
    pub fn status(mut self, value: AgentOutputStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn error(mut self, value: AgentOutputError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn required_actions(mut self, value: Vec<ClientToolCallRequiredAction>) -> Self {
        self.required_actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamEndedData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](StreamEndedDataBuilder::status)
    pub fn build(self) -> Result<StreamEndedData, BuildError> {
        Ok(StreamEndedData {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            error: self.error,
            required_actions: self.required_actions,
        })
    }
}
