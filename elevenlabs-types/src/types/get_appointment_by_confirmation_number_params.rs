pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Look up an appointment by the booking confirmation number the caller quotes.
/// 
/// The confirmation number is the 8-character code shown on the booking
/// confirmation page (e.g. ``#01ABCDEF``). Callers may read it back with or
/// without the leading ``#`` and with varied spacing; the tool normalizes
/// the input and does a prefix match on the stored calendar item id.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAppointmentByConfirmationNumberParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl GetAppointmentByConfirmationNumberParams {
    pub fn builder() -> GetAppointmentByConfirmationNumberParamsBuilder {
        <GetAppointmentByConfirmationNumberParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAppointmentByConfirmationNumberParamsBuilder {
    smb_tool_type: Option<String>,
}

impl GetAppointmentByConfirmationNumberParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAppointmentByConfirmationNumberParams`].
    pub fn build(self) -> Result<GetAppointmentByConfirmationNumberParams, BuildError> {
        Ok(GetAppointmentByConfirmationNumberParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
