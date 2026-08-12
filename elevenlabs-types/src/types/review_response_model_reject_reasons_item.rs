pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReviewResponseModelRejectReasonsItem {
    LacksStructure,
    DoesntOpen,
    NotLiteraryWork,
    LanguageNotSupported,
    TooShort,
    Duplicate,
    Promotional,
    FormattingIssues,
    LowQuality,
    MetadataIncomplete,
    MetadataInaccurate,
    Typos,
    ReviewError,
    Spam,
    LegalViolation,
    ContentPolicy,
    PublicDomain,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReviewResponseModelRejectReasonsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::LacksStructure => serializer.serialize_str("lacks_structure"),
            Self::DoesntOpen => serializer.serialize_str("doesnt_open"),
            Self::NotLiteraryWork => serializer.serialize_str("not_literary_work"),
            Self::LanguageNotSupported => serializer.serialize_str("language_not_supported"),
            Self::TooShort => serializer.serialize_str("too_short"),
            Self::Duplicate => serializer.serialize_str("duplicate"),
            Self::Promotional => serializer.serialize_str("promotional"),
            Self::FormattingIssues => serializer.serialize_str("formatting_issues"),
            Self::LowQuality => serializer.serialize_str("low_quality"),
            Self::MetadataIncomplete => serializer.serialize_str("metadata_incomplete"),
            Self::MetadataInaccurate => serializer.serialize_str("metadata_inaccurate"),
            Self::Typos => serializer.serialize_str("typos"),
            Self::ReviewError => serializer.serialize_str("review_error"),
            Self::Spam => serializer.serialize_str("spam"),
            Self::LegalViolation => serializer.serialize_str("legal_violation"),
            Self::ContentPolicy => serializer.serialize_str("content_policy"),
            Self::PublicDomain => serializer.serialize_str("public_domain"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReviewResponseModelRejectReasonsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "lacks_structure" => Ok(Self::LacksStructure),
            "doesnt_open" => Ok(Self::DoesntOpen),
            "not_literary_work" => Ok(Self::NotLiteraryWork),
            "language_not_supported" => Ok(Self::LanguageNotSupported),
            "too_short" => Ok(Self::TooShort),
            "duplicate" => Ok(Self::Duplicate),
            "promotional" => Ok(Self::Promotional),
            "formatting_issues" => Ok(Self::FormattingIssues),
            "low_quality" => Ok(Self::LowQuality),
            "metadata_incomplete" => Ok(Self::MetadataIncomplete),
            "metadata_inaccurate" => Ok(Self::MetadataInaccurate),
            "typos" => Ok(Self::Typos),
            "review_error" => Ok(Self::ReviewError),
            "spam" => Ok(Self::Spam),
            "legal_violation" => Ok(Self::LegalViolation),
            "content_policy" => Ok(Self::ContentPolicy),
            "public_domain" => Ok(Self::PublicDomain),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReviewResponseModelRejectReasonsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LacksStructure => write!(f, "lacks_structure"),
            Self::DoesntOpen => write!(f, "doesnt_open"),
            Self::NotLiteraryWork => write!(f, "not_literary_work"),
            Self::LanguageNotSupported => write!(f, "language_not_supported"),
            Self::TooShort => write!(f, "too_short"),
            Self::Duplicate => write!(f, "duplicate"),
            Self::Promotional => write!(f, "promotional"),
            Self::FormattingIssues => write!(f, "formatting_issues"),
            Self::LowQuality => write!(f, "low_quality"),
            Self::MetadataIncomplete => write!(f, "metadata_incomplete"),
            Self::MetadataInaccurate => write!(f, "metadata_inaccurate"),
            Self::Typos => write!(f, "typos"),
            Self::ReviewError => write!(f, "review_error"),
            Self::Spam => write!(f, "spam"),
            Self::LegalViolation => write!(f, "legal_violation"),
            Self::ContentPolicy => write!(f, "content_policy"),
            Self::PublicDomain => write!(f, "public_domain"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
