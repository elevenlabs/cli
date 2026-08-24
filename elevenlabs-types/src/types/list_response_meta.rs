pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListResponseMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl ListResponseMeta {
    pub fn builder() -> ListResponseMetaBuilder {
        <ListResponseMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseMetaBuilder {
    total: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
}

impl ListResponseMetaBuilder {
    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListResponseMeta`].
    pub fn build(self) -> Result<ListResponseMeta, BuildError> {
        Ok(ListResponseMeta {
            total: self.total,
            page: self.page,
            page_size: self.page_size,
        })
    }
}
