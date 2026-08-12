pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColumnFilter {
    #[serde(default)]
    pub column: String,
    pub operation: ColumnFilterOperation,
    #[serde(default)]
    pub values: Vec<Option<ColumnFilterValuesItem>>,
}

impl ColumnFilter {
    pub fn builder() -> ColumnFilterBuilder {
        <ColumnFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ColumnFilterBuilder {
    column: Option<String>,
    operation: Option<ColumnFilterOperation>,
    values: Option<Vec<Option<ColumnFilterValuesItem>>>,
}

impl ColumnFilterBuilder {
    pub fn column(mut self, value: impl Into<String>) -> Self {
        self.column = Some(value.into());
        self
    }

    pub fn operation(mut self, value: ColumnFilterOperation) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<Option<ColumnFilterValuesItem>>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ColumnFilter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`column`](ColumnFilterBuilder::column)
    /// - [`operation`](ColumnFilterBuilder::operation)
    /// - [`values`](ColumnFilterBuilder::values)
    pub fn build(self) -> Result<ColumnFilter, BuildError> {
        Ok(ColumnFilter {
            column: self.column.ok_or_else(|| BuildError::missing_field("column"))?,
            operation: self.operation.ok_or_else(|| BuildError::missing_field("operation"))?,
            values: self.values.ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
