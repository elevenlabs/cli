pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TicketSearchMatchResponseModel {
    /// Which of the ticket's texts matched.
    pub field: TicketSearchMatchField,
    /// Whitespace-collapsed excerpt around the match, with an ellipsis where it was cut.
    #[serde(default)]
    pub snippet: String,
    /// Offset of the match in `snippet`.
    #[serde(default)]
    pub highlight_start: i64,
    /// Exclusive end offset of the match in `snippet`.
    #[serde(default)]
    pub highlight_end: i64,
    /// The commented turn, set when `field` is 'turn_comment'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_index: Option<i64>,
}

impl TicketSearchMatchResponseModel {
    pub fn builder() -> TicketSearchMatchResponseModelBuilder {
        <TicketSearchMatchResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketSearchMatchResponseModelBuilder {
    field: Option<TicketSearchMatchField>,
    snippet: Option<String>,
    highlight_start: Option<i64>,
    highlight_end: Option<i64>,
    turn_index: Option<i64>,
}

impl TicketSearchMatchResponseModelBuilder {
    pub fn field(mut self, value: TicketSearchMatchField) -> Self {
        self.field = Some(value);
        self
    }

    pub fn snippet(mut self, value: impl Into<String>) -> Self {
        self.snippet = Some(value.into());
        self
    }

    pub fn highlight_start(mut self, value: i64) -> Self {
        self.highlight_start = Some(value);
        self
    }

    pub fn highlight_end(mut self, value: i64) -> Self {
        self.highlight_end = Some(value);
        self
    }

    pub fn turn_index(mut self, value: i64) -> Self {
        self.turn_index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketSearchMatchResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](TicketSearchMatchResponseModelBuilder::field)
    /// - [`snippet`](TicketSearchMatchResponseModelBuilder::snippet)
    /// - [`highlight_start`](TicketSearchMatchResponseModelBuilder::highlight_start)
    /// - [`highlight_end`](TicketSearchMatchResponseModelBuilder::highlight_end)
    pub fn build(self) -> Result<TicketSearchMatchResponseModel, BuildError> {
        Ok(TicketSearchMatchResponseModel {
            field: self.field.ok_or_else(|| BuildError::missing_field("field"))?,
            snippet: self.snippet.ok_or_else(|| BuildError::missing_field("snippet"))?,
            highlight_start: self.highlight_start.ok_or_else(|| BuildError::missing_field("highlight_start"))?,
            highlight_end: self.highlight_end.ok_or_else(|| BuildError::missing_field("highlight_end"))?,
            turn_index: self.turn_index,
        })
    }
}
