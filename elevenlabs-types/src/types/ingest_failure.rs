pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Why processing a knowledge source failed.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IngestFailure {
    BlockedUrl,
    DnsFailure,
    HostUnreachable,
    AccessRefused,
    CrawlTimeout,
    NoPages,
    NoContent,
    UpstreamTimeout,
    UpstreamError,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for IngestFailure {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BlockedUrl => serializer.serialize_str("blocked_url"),
            Self::DnsFailure => serializer.serialize_str("dns_failure"),
            Self::HostUnreachable => serializer.serialize_str("host_unreachable"),
            Self::AccessRefused => serializer.serialize_str("access_refused"),
            Self::CrawlTimeout => serializer.serialize_str("crawl_timeout"),
            Self::NoPages => serializer.serialize_str("no_pages"),
            Self::NoContent => serializer.serialize_str("no_content"),
            Self::UpstreamTimeout => serializer.serialize_str("upstream_timeout"),
            Self::UpstreamError => serializer.serialize_str("upstream_error"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for IngestFailure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "blocked_url" => Ok(Self::BlockedUrl),
            "dns_failure" => Ok(Self::DnsFailure),
            "host_unreachable" => Ok(Self::HostUnreachable),
            "access_refused" => Ok(Self::AccessRefused),
            "crawl_timeout" => Ok(Self::CrawlTimeout),
            "no_pages" => Ok(Self::NoPages),
            "no_content" => Ok(Self::NoContent),
            "upstream_timeout" => Ok(Self::UpstreamTimeout),
            "upstream_error" => Ok(Self::UpstreamError),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for IngestFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlockedUrl => write!(f, "blocked_url"),
            Self::DnsFailure => write!(f, "dns_failure"),
            Self::HostUnreachable => write!(f, "host_unreachable"),
            Self::AccessRefused => write!(f, "access_refused"),
            Self::CrawlTimeout => write!(f, "crawl_timeout"),
            Self::NoPages => write!(f, "no_pages"),
            Self::NoContent => write!(f, "no_content"),
            Self::UpstreamTimeout => write!(f, "upstream_timeout"),
            Self::UpstreamError => write!(f, "upstream_error"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
