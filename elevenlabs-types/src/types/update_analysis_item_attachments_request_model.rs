pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateAnalysisItemAttachmentsRequestModel {
    pub source: AnalysisItemSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<Vec<AnalysisItemAttachmentDestination>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach: Option<Vec<AnalysisItemAttachmentDestination>>,
}

impl UpdateAnalysisItemAttachmentsRequestModel {
    pub fn builder() -> UpdateAnalysisItemAttachmentsRequestModelBuilder {
        <UpdateAnalysisItemAttachmentsRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAnalysisItemAttachmentsRequestModelBuilder {
    source: Option<AnalysisItemSource>,
    attach: Option<Vec<AnalysisItemAttachmentDestination>>,
    detach: Option<Vec<AnalysisItemAttachmentDestination>>,
}

impl UpdateAnalysisItemAttachmentsRequestModelBuilder {
    pub fn source(mut self, value: AnalysisItemSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn attach(mut self, value: Vec<AnalysisItemAttachmentDestination>) -> Self {
        self.attach = Some(value);
        self
    }

    pub fn detach(mut self, value: Vec<AnalysisItemAttachmentDestination>) -> Self {
        self.detach = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAnalysisItemAttachmentsRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](UpdateAnalysisItemAttachmentsRequestModelBuilder::source)
    pub fn build(self) -> Result<UpdateAnalysisItemAttachmentsRequestModel, BuildError> {
        Ok(UpdateAnalysisItemAttachmentsRequestModel {
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            attach: self.attach,
            detach: self.detach,
        })
    }
}

