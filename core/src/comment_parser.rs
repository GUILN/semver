use regex::Regex;
use serde::{Serialize, Deserialize};
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
    pub fn new(is_breaking: bool) -> Self { Self { is_breaking } }
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

/// Parses a comment and returns a [`Result<SemanticComment, SemVerError>`]
/// # Expected format:
/// - <semantic_type>: this is a <semantic_type>.
/// - <semantic_type>! this is a <semantic_type>.
/// 
/// Where <semantic_type> is [`fix`, `feat`, `refact`] and [`:`, `!`] means [`non_breaking`, `breaking`] respectively.
/// 
/// Example
/// ```
/// # use core::*;
/// let your_git_comment = "feat! breaking change feature.";
/// let parsed_comment = parse_comment(your_git_comment).unwrap();
/// assert_eq!(parsed_comment,SemanticComment::new("breaking change feature.".to_string(), SemanticType::Feature(SemanticTypeMetadata::new(true))));
/// ```
pub fn parse_comment(comment: &str) -> Result<SemanticComment, SemVerError> {
    let re = Regex::new(r"^[a-zA-Z0-9_]+(:|!)").unwrap();

    if let Some(mat) = re.find(comment) {
        let prefix_delimiter = mat.end();
        
        let left_side = &comment[0..prefix_delimiter];
        let right_side = &comment[(prefix_delimiter)..comment.len()];

        let is_breaking = left_side.ends_with('!');

        let prefix = &left_side[0..left_side.len() -1];

        match prefix.trim() {
            "feat" => Ok(SemanticComment::new(right_side.trim().to_string(), SemanticType::Feature(SemanticTypeMetadata::new(is_breaking)))),
            "fix" => Ok(SemanticComment::new(right_side.trim().to_string(), SemanticType::Fix(SemanticTypeMetadata::new(is_breaking)))),
            "refact" => Ok(SemanticComment::new(right_side.trim().to_string(), SemanticType::Refactoring(SemanticTypeMetadata::new(is_breaking)))),
            _ => Err(SemVerError::UnexpectedSemanticType(prefix.to_string()))
        }
        
    } else {
        Err(SemVerError::InvalidFormat)
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_comment_retrieves_expected_semantic_type_from_comment_string() {
        let cases = vec![
            ("feat: feature here",SemanticComment::new("feature here".to_string(), SemanticType::Feature(SemanticTypeMetadata::new(false)))),
            ("feat! feature here",SemanticComment::new("feature here".to_string(), SemanticType::Feature(SemanticTypeMetadata::new(true)))),
            ("fix: fix here",SemanticComment::new("fix here".to_string(), SemanticType::Fix(SemanticTypeMetadata::new(false)))),
            ("fix! fix here",SemanticComment::new("fix here".to_string(), SemanticType::Fix(SemanticTypeMetadata::new(true)))),
            ("fix!fix here",SemanticComment::new("fix here".to_string(), SemanticType::Fix(SemanticTypeMetadata::new(true)))),
            ("refact: refactoring here",SemanticComment::new("refactoring here".to_string(), SemanticType::Refactoring(SemanticTypeMetadata::new(false)))),
            ("refact:refactoring here",SemanticComment::new("refactoring here".to_string(), SemanticType::Refactoring(SemanticTypeMetadata::new(false)))),
            ("refact! refactoring here",SemanticComment::new("refactoring here".to_string(), SemanticType::Refactoring(SemanticTypeMetadata::new(true)))),
        ];

        for (comment, expected_sem_com) in cases {
            let sem_comment = parse_comment(comment).unwrap();

            assert_eq!(sem_comment, expected_sem_com);
        }
    }

    #[test]
    fn test_parse_comment_returns_expected_error_when_format_is_invalid() {
        let comment_with_invalid_format = "this is a comment with invalid format".to_string();

        let sem_ver_error = parse_comment(&comment_with_invalid_format).unwrap_err();
        assert_eq!(sem_ver_error, SemVerError::InvalidFormat)
    }
    #[test]
    fn test_parse_comment_returns_expected_error_when_semantic_type_is_not_supported() {
        let comment_with_unsupported_semantic_type = "wop! some work around.".to_string();

        let sem_ver_error = parse_comment(&comment_with_unsupported_semantic_type).unwrap_err();
        assert_eq!(sem_ver_error, SemVerError::UnexpectedSemanticType("wop".to_string()))
    }
}
