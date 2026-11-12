pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetryPolicy {
    /// How long to wait before a short retry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay_seconds: Option<i64>,
    /// Retries a calling window allows after a failed call. Once used up, the next retry waits for the following window, which starts afresh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_short_retries: Option<i64>,
    /// The most calls a lead gets, counting the first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_total_calls: Option<i64>,
    /// The most callbacks to honor for one number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_callbacks: Option<i64>,
    /// The most times a lead follows a request to call another number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redirects: Option<i64>,
    /// Where in a later calling window a pushed-back call lands: at its `start`, at a `random` time, or `rotate`d `rotate_step_seconds` past where the last call in that window failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_placement: Option<IntervalPlacement>,
    /// How far `rotate` placement moves each call. Only used by `rotate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_step_seconds: Option<i64>,
    /// Calls to one number that may fail in a row before the lead moves on to its next listed number, or ends when there is none. An answered call, or a redirect to a new number, restarts the count. Unset leaves only `max_total_calls`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_consecutive_failures_per_number: Option<i64>,
}

impl RetryPolicy {
    pub fn builder() -> RetryPolicyBuilder {
        <RetryPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetryPolicyBuilder {
    retry_delay_seconds: Option<i64>,
    max_short_retries: Option<i64>,
    max_total_calls: Option<i64>,
    max_callbacks: Option<i64>,
    max_redirects: Option<i64>,
    interval_placement: Option<IntervalPlacement>,
    rotate_step_seconds: Option<i64>,
    max_consecutive_failures_per_number: Option<i64>,
}

impl RetryPolicyBuilder {
    pub fn retry_delay_seconds(mut self, value: i64) -> Self {
        self.retry_delay_seconds = Some(value);
        self
    }

    pub fn max_short_retries(mut self, value: i64) -> Self {
        self.max_short_retries = Some(value);
        self
    }

    pub fn max_total_calls(mut self, value: i64) -> Self {
        self.max_total_calls = Some(value);
        self
    }

    pub fn max_callbacks(mut self, value: i64) -> Self {
        self.max_callbacks = Some(value);
        self
    }

    pub fn max_redirects(mut self, value: i64) -> Self {
        self.max_redirects = Some(value);
        self
    }

    pub fn interval_placement(mut self, value: IntervalPlacement) -> Self {
        self.interval_placement = Some(value);
        self
    }

    pub fn rotate_step_seconds(mut self, value: i64) -> Self {
        self.rotate_step_seconds = Some(value);
        self
    }

    pub fn max_consecutive_failures_per_number(mut self, value: i64) -> Self {
        self.max_consecutive_failures_per_number = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetryPolicy`].
    pub fn build(self) -> Result<RetryPolicy, BuildError> {
        Ok(RetryPolicy {
            retry_delay_seconds: self.retry_delay_seconds,
            max_short_retries: self.max_short_retries,
            max_total_calls: self.max_total_calls,
            max_callbacks: self.max_callbacks,
            max_redirects: self.max_redirects,
            interval_placement: self.interval_placement,
            rotate_step_seconds: self.rotate_step_seconds,
            max_consecutive_failures_per_number: self.max_consecutive_failures_per_number,
        })
    }
}
