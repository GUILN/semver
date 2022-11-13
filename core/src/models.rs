use serde::{Deserialize, Serialize};
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error, PartialEq)]
/// [SemVerError]
///
/// Provides error that can occur when parsing comment.
pub enum SemVerError {
    #[error("The format provided is invalid!")]
    InvalidFormat,
    #[error("Unexpected semantic type")]
    UnexpectedSemanticType(String),
    #[error("error while deserializing")]
    DeserializationError,
}

impl From<serde_json::Error> for SemVerError {
    fn from(_: serde_json::Error) -> Self {
        Self::DeserializationError
    }
}

/// Provides semantic type assumed from the commit message.
/// # Possible breaking values
/// - fix!, feat!, refact!
/// # Possible non breaking values
/// - fix:, feat:, refact:
#[derive(Debug, Serialize, Deserialize)]
pub enum SemanticType {
    Fix(SemanticTypeMetadata),
    Feature(SemanticTypeMetadata),
    Refactoring(SemanticTypeMetadata),
}
/// Holds metadata about the semantic type.
#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticTypeMetadata {
    is_breaking: bool,
}

impl SemanticTypeMetadata {
    pub fn new(is_breaking: bool) -> Self {
        Self { is_breaking }
    }
}

impl PartialEq for SemanticType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Fix(l_meta), Self::Fix(r_meta)) => l_meta.is_breaking == r_meta.is_breaking,
            (Self::Feature(l_meta), Self::Feature(r_meta)) => {
                l_meta.is_breaking == r_meta.is_breaking
            }
            (Self::Refactoring(l_meta), Self::Refactoring(r_meta)) => {
                l_meta.is_breaking == r_meta.is_breaking
            }

            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticComment {
    pub comment: String,
    pub semantic_type: SemanticType,
}

impl SemanticComment {
    pub fn new(comment: String, semantic_type: SemanticType) -> Self {
        Self {
            comment,
            semantic_type,
        }
    }

    /// [`as_json_string`] returns json representation of the structure.
    pub fn as_json_string(&self) -> Result<String, SemVerError> {
        Ok(serde_json::to_string(&self)?)
    }
}

impl PartialEq for SemanticComment {
    fn eq(&self, other: &Self) -> bool {
        self.comment == other.comment && self.semantic_type == other.semantic_type
    }
}
