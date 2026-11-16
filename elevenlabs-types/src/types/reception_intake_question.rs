pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A question the receptionist asks before booking or ordering this item.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionIntakeQuestion {
    /// Stable identifier referenced by stored answers. Survives label edits so history stays attributable to the right question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The question as the agent should ask it, e.g. 'How many rooms?'. Phrased by the owner; the agent may rephrase for conversational flow but must not change what is being asked.
    #[serde(default)]
    pub label: StrippedStr,
    /// The answer's shape: free text, a number, or one of options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<IntakeQuestionKind>,
    /// Allowed answers for `kind='choice'`. Must be empty for the other kinds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<StrippedStr>>,
    /// Enforced on the booking page. On calls the receptionist asks, but still books if the caller won't answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Retired questions are deactivated rather than deleted, so answers already stored against them keep resolving.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl ReceptionIntakeQuestion {
    pub fn builder() -> ReceptionIntakeQuestionBuilder {
        <ReceptionIntakeQuestionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionIntakeQuestionBuilder {
    id: Option<String>,
    label: Option<StrippedStr>,
    kind: Option<IntakeQuestionKind>,
    options: Option<Vec<StrippedStr>>,
    required: Option<bool>,
    is_active: Option<bool>,
}

impl ReceptionIntakeQuestionBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: StrippedStr) -> Self {
        self.label = Some(value);
        self
    }

    pub fn kind(mut self, value: IntakeQuestionKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<StrippedStr>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionIntakeQuestion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](ReceptionIntakeQuestionBuilder::label)
    pub fn build(self) -> Result<ReceptionIntakeQuestion, BuildError> {
        Ok(ReceptionIntakeQuestion {
            id: self.id,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            kind: self.kind,
            options: self.options,
            required: self.required,
            is_active: self.is_active,
        })
    }
}
