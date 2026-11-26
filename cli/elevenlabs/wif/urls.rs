//! HTTPS-only rule: plain http only on loopback, so the conformance harness can run.

use reqwest::Url;

use super::error::{config_invalid, WifError};

pub fn is_loopback(host: &str) -> bool {
    host == "localhost"
        || host
            .trim_start_matches('[')
            .trim_end_matches(']')
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback())
}

pub fn require_https_or_loopback(url: &str, what: &str) -> Result<(), WifError> {
    let ok = Url::parse(url).is_ok_and(|u| match u.scheme() {
        "https" => true,
        "http" => u.host_str().is_some_and(is_loopback),
        _ => false,
    });
    if ok {
        Ok(())
    } else {
        Err(config_invalid(format!(
            "{what} must be https (plain http only on loopback)"
        )))
    }
}
