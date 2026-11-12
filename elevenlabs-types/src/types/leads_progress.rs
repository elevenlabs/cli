pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LeadsProgress {
    #[serde(default)]
    pub total: i64,
    /// Unfinished leads whose first call hasn't been placed yet.
    #[serde(default)]
    pub uncalled: i64,
    /// Unfinished leads whose next call an attention item holds back.
    #[serde(default)]
    pub leads_needing_attention: i64,
    /// Other unfinished leads: a call under way, awaiting analysis or a follow-up.
    #[serde(default)]
    pub active: i64,
    /// Leads that won't be called again.
    #[serde(default)]
    pub finished: i64,
    /// Finished leads by why they finished. Missing keys mean none.
    #[serde(default)]
    pub finished_by_disposition: HashMap<String, i64>,
}

impl LeadsProgress {
    pub fn builder() -> LeadsProgressBuilder {
        <LeadsProgressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadsProgressBuilder {
    total: Option<i64>,
    uncalled: Option<i64>,
    leads_needing_attention: Option<i64>,
    active: Option<i64>,
    finished: Option<i64>,
    finished_by_disposition: Option<HashMap<String, i64>>,
}

impl LeadsProgressBuilder {
    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn uncalled(mut self, value: i64) -> Self {
        self.uncalled = Some(value);
        self
    }

    pub fn leads_needing_attention(mut self, value: i64) -> Self {
        self.leads_needing_attention = Some(value);
        self
    }

    pub fn active(mut self, value: i64) -> Self {
        self.active = Some(value);
        self
    }

    pub fn finished(mut self, value: i64) -> Self {
        self.finished = Some(value);
        self
    }

    pub fn finished_by_disposition(mut self, value: HashMap<String, i64>) -> Self {
        self.finished_by_disposition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LeadsProgress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](LeadsProgressBuilder::total)
    /// - [`uncalled`](LeadsProgressBuilder::uncalled)
    /// - [`leads_needing_attention`](LeadsProgressBuilder::leads_needing_attention)
    /// - [`active`](LeadsProgressBuilder::active)
    /// - [`finished`](LeadsProgressBuilder::finished)
    /// - [`finished_by_disposition`](LeadsProgressBuilder::finished_by_disposition)
    pub fn build(self) -> Result<LeadsProgress, BuildError> {
        Ok(LeadsProgress {
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
            uncalled: self.uncalled.ok_or_else(|| BuildError::missing_field("uncalled"))?,
            leads_needing_attention: self.leads_needing_attention.ok_or_else(|| BuildError::missing_field("leads_needing_attention"))?,
            active: self.active.ok_or_else(|| BuildError::missing_field("active"))?,
            finished: self.finished.ok_or_else(|| BuildError::missing_field("finished"))?,
            finished_by_disposition: self.finished_by_disposition.ok_or_else(|| BuildError::missing_field("finished_by_disposition"))?,
        })
    }
}
