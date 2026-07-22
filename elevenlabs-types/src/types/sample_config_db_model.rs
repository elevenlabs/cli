pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SampleConfigDbModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sample: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<SampleConfigDbModelParentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_ids: Option<Vec<String>>,
}

impl SampleConfigDbModel {
    pub fn builder() -> SampleConfigDbModelBuilder {
        <SampleConfigDbModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SampleConfigDbModelBuilder {
    is_sample: Option<bool>,
    parent_id: Option<String>,
    parent_type: Option<SampleConfigDbModelParentType>,
    chapter_ids: Option<Vec<String>>,
}

impl SampleConfigDbModelBuilder {
    pub fn is_sample(mut self, value: bool) -> Self {
        self.is_sample = Some(value);
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn parent_type(mut self, value: SampleConfigDbModelParentType) -> Self {
        self.parent_type = Some(value);
        self
    }

    pub fn chapter_ids(mut self, value: Vec<String>) -> Self {
        self.chapter_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SampleConfigDbModel`].
    pub fn build(self) -> Result<SampleConfigDbModel, BuildError> {
        Ok(SampleConfigDbModel {
            is_sample: self.is_sample,
            parent_id: self.parent_id,
            parent_type: self.parent_type,
            chapter_ids: self.chapter_ids,
        })
    }
}
