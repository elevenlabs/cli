pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HistoryAlignmentsResponseModel {
    /// The alignment of the text.
    #[serde(default)]
    pub alignment: HistoryAlignmentResponseModel,
    /// The normalized alignment of the text.
    #[serde(default)]
    pub normalized_alignment: HistoryAlignmentResponseModel,
}

impl HistoryAlignmentsResponseModel {
    pub fn builder() -> HistoryAlignmentsResponseModelBuilder {
        <HistoryAlignmentsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryAlignmentsResponseModelBuilder {
    alignment: Option<HistoryAlignmentResponseModel>,
    normalized_alignment: Option<HistoryAlignmentResponseModel>,
}

impl HistoryAlignmentsResponseModelBuilder {
    pub fn alignment(mut self, value: HistoryAlignmentResponseModel) -> Self {
        self.alignment = Some(value);
        self
    }

    pub fn normalized_alignment(mut self, value: HistoryAlignmentResponseModel) -> Self {
        self.normalized_alignment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HistoryAlignmentsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alignment`](HistoryAlignmentsResponseModelBuilder::alignment)
    /// - [`normalized_alignment`](HistoryAlignmentsResponseModelBuilder::normalized_alignment)
    pub fn build(self) -> Result<HistoryAlignmentsResponseModel, BuildError> {
        Ok(HistoryAlignmentsResponseModel {
            alignment: self.alignment.ok_or_else(|| BuildError::missing_field("alignment"))?,
            normalized_alignment: self.normalized_alignment.ok_or_else(|| BuildError::missing_field("normalized_alignment"))?,
        })
    }
}
