pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetronomeUsageColumnFilter {
    pub column: MetronomeUsageColumnFilterColumn,
    pub operation: MetronomeUsageColumnFilterOperation,
    #[serde(default)]
    pub values: Vec<Option<MetronomeUsageColumnFilterValuesItem>>,
}

impl MetronomeUsageColumnFilter {
    pub fn builder() -> MetronomeUsageColumnFilterBuilder {
        <MetronomeUsageColumnFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetronomeUsageColumnFilterBuilder {
    column: Option<MetronomeUsageColumnFilterColumn>,
    operation: Option<MetronomeUsageColumnFilterOperation>,
    values: Option<Vec<Option<MetronomeUsageColumnFilterValuesItem>>>,
}

impl MetronomeUsageColumnFilterBuilder {
    pub fn column(mut self, value: MetronomeUsageColumnFilterColumn) -> Self {
        self.column = Some(value);
        self
    }

    pub fn operation(mut self, value: MetronomeUsageColumnFilterOperation) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<Option<MetronomeUsageColumnFilterValuesItem>>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MetronomeUsageColumnFilter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`column`](MetronomeUsageColumnFilterBuilder::column)
    /// - [`operation`](MetronomeUsageColumnFilterBuilder::operation)
    /// - [`values`](MetronomeUsageColumnFilterBuilder::values)
    pub fn build(self) -> Result<MetronomeUsageColumnFilter, BuildError> {
        Ok(MetronomeUsageColumnFilter {
            column: self.column.ok_or_else(|| BuildError::missing_field("column"))?,
            operation: self.operation.ok_or_else(|| BuildError::missing_field("operation"))?,
            values: self.values.ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
