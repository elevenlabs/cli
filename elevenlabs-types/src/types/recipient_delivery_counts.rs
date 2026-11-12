pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecipientDeliveryCounts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replied: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
}

impl RecipientDeliveryCounts {
    pub fn builder() -> RecipientDeliveryCountsBuilder {
        <RecipientDeliveryCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecipientDeliveryCountsBuilder {
    sent: Option<i64>,
    delivered: Option<i64>,
    read: Option<i64>,
    replied: Option<i64>,
    failed: Option<i64>,
}

impl RecipientDeliveryCountsBuilder {
    pub fn sent(mut self, value: i64) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn delivered(mut self, value: i64) -> Self {
        self.delivered = Some(value);
        self
    }

    pub fn read(mut self, value: i64) -> Self {
        self.read = Some(value);
        self
    }

    pub fn replied(mut self, value: i64) -> Self {
        self.replied = Some(value);
        self
    }

    pub fn failed(mut self, value: i64) -> Self {
        self.failed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecipientDeliveryCounts`].
    pub fn build(self) -> Result<RecipientDeliveryCounts, BuildError> {
        Ok(RecipientDeliveryCounts {
            sent: self.sent,
            delivered: self.delivered,
            read: self.read,
            replied: self.replied,
            failed: self.failed,
        })
    }
}
