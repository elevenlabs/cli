//! One client's access token, held in memory and refreshed lazily. Thread-safe.

use std::sync::{Condvar, Mutex, MutexGuard};

use super::clock;
use super::error::WifError;
use super::exchange::Exchanger;

const MAX_REFRESH_MARGIN_SECONDS: f64 = 300.0;
const HARD_FLOOR_SECONDS: f64 = 10.0;

#[derive(Default)]
struct State {
    token: Option<String>,
    expires_at: f64,
    lifetime: f64,
    in_flight: bool,
    /// Bumped when an exchange finishes; waiters read `last_error` for their flight.
    generation: u64,
    last_error: Option<WifError>,
}

impl State {
    fn remaining(&self, now: f64) -> Option<(&str, f64)> {
        self.token.as_deref().map(|t| (t, self.expires_at - now))
    }
}

pub struct TokenManager {
    exchanger: Exchanger,
    state: Mutex<State>,
    done: Condvar,
}

impl std::fmt::Debug for TokenManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TokenManager")
    }
}

impl TokenManager {
    pub fn new(exchanger: Exchanger) -> Self {
        Self {
            exchanger,
            state: Mutex::new(State::default()),
            done: Condvar::new(),
        }
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|p| p.into_inner())
    }

    /// The cached token, or a fresh one; concurrent callers share one exchange.
    pub fn get_token(&self) -> Result<String, WifError> {
        loop {
            let now = clock::now()?;
            let mut st = self.lock();
            let margin = MAX_REFRESH_MARGIN_SECONDS
                .min(st.lifetime / 2.0)
                .max(HARD_FLOOR_SECONDS);
            if let Some((token, left)) = st.remaining(now) {
                if left >= margin {
                    return Ok(token.to_string());
                }
            }
            let flight = st.generation;
            if st.in_flight {
                while st.in_flight && st.generation == flight {
                    st = self.done.wait(st).unwrap_or_else(|p| p.into_inner());
                }
            } else {
                st.in_flight = true;
                drop(st);
                let outcome = self.refresh();
                st = self.lock();
                match outcome {
                    Ok(()) => st.last_error = None,
                    Err(e) => st.last_error = Some(e),
                }
                st.in_flight = false;
                st.generation += 1;
                self.done.notify_all();
            }

            let now = clock::now()?;
            let error = st.last_error.clone();
            match (error, st.remaining(now)) {
                // Any unexpired token right after a refresh, even one shorter than the margin.
                (None, Some((token, left))) if left > 0.0 => return Ok(token.to_string()),
                (None, _) => continue,
                // Soft failure: the old token is still good for a while.
                (Some(_), Some((token, left))) if left > HARD_FLOOR_SECONDS => {
                    return Ok(token.to_string())
                }
                (Some(e), _) => return Err(e),
            }
        }
    }

    fn refresh(&self) -> Result<(), WifError> {
        let started = clock::now()?;
        let result = match run_blocking(self.exchanger.exchange()) {
            Ok(r) => r,
            Err(e) => {
                let now = clock::now().unwrap_or(f64::MAX);
                let st = self.lock();
                if st
                    .remaining(now)
                    .is_some_and(|(_, left)| left > HARD_FLOOR_SECONDS)
                {
                    super::warn(&format!(
                        "ElevenLabs WIF token refresh failed ({}); using the current token",
                        e.category
                    ));
                }
                tracing::debug!(pool = %self.exchanger.pool, resource = %self.exchanger.resource, outcome = e.category, "ElevenLabs WIF exchange failed");
                return Err(e);
            }
        };
        tracing::debug!(
            pool = %self.exchanger.pool,
            resource = %self.exchanger.resource,
            scope = %result.scope,
            expires_in = result.expires_in as u64,
            outcome = "ok",
            "ElevenLabs WIF exchange succeeded"
        );
        let mut st = self.lock();
        st.token = Some(result.access_token);
        st.expires_at = started + result.expires_in;
        st.lifetime = result.expires_in;
        Ok(())
    }
}

/// Runs a future to completion on a private runtime in its own thread.
///
/// `AuthProvider::apply` is synchronous and may be called from inside the CLI's runtime,
/// where blocking on that runtime is not allowed; a separate thread works from any context.
fn run_blocking<F>(future: F) -> F::Output
where
    F: std::future::Future + Send,
    F::Output: Send,
{
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("a current-thread runtime always builds")
                    .block_on(future)
            })
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
    })
}
