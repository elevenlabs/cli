pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MusicFinetunePageResponseModel {
    /// The finetunes in this page.
    #[serde(default)]
    pub finetunes: Vec<MusicFinetuneResponseModel>,
    /// Cursor to pass as `cursor` to fetch the next page; `null` when there are no more results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether more finetunes are available beyond this page.
    #[serde(default)]
    pub has_more: bool,
}

impl MusicFinetunePageResponseModel {
    pub fn builder() -> MusicFinetunePageResponseModelBuilder {
        <MusicFinetunePageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MusicFinetunePageResponseModelBuilder {
    finetunes: Option<Vec<MusicFinetuneResponseModel>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl MusicFinetunePageResponseModelBuilder {
    pub fn finetunes(mut self, value: Vec<MusicFinetuneResponseModel>) -> Self {
        self.finetunes = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MusicFinetunePageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`finetunes`](MusicFinetunePageResponseModelBuilder::finetunes)
    /// - [`has_more`](MusicFinetunePageResponseModelBuilder::has_more)
    pub fn build(self) -> Result<MusicFinetunePageResponseModel, BuildError> {
        Ok(MusicFinetunePageResponseModel {
            finetunes: self.finetunes.ok_or_else(|| BuildError::missing_field("finetunes"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
