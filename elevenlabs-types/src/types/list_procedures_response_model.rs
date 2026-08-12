pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListProceduresResponseModel {
    /// Procedures on the branch with their draft-aware metadata.
    #[serde(default)]
    pub procedures: Vec<ProcedureListItemResponseModel>,
}

impl ListProceduresResponseModel {
    pub fn builder() -> ListProceduresResponseModelBuilder {
        <ListProceduresResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProceduresResponseModelBuilder {
    procedures: Option<Vec<ProcedureListItemResponseModel>>,
}

impl ListProceduresResponseModelBuilder {
    pub fn procedures(mut self, value: Vec<ProcedureListItemResponseModel>) -> Self {
        self.procedures = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListProceduresResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`procedures`](ListProceduresResponseModelBuilder::procedures)
    pub fn build(self) -> Result<ListProceduresResponseModel, BuildError> {
        Ok(ListProceduresResponseModel {
            procedures: self.procedures.ok_or_else(|| BuildError::missing_field("procedures"))?,
        })
    }
}
