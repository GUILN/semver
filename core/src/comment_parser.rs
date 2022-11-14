use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{SemVerError, SemanticComment, SemanticType, SemanticTypeMetadata};

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
/// let parsed_comment: SemanticComment = "feat! breaking change feature.".try_into().unwrap();
/// assert_eq!(parsed_comment,SemanticComment::new("breaking change feature.".to_string(), SemanticType::Feature(SemanticTypeMetadata::new(true))));
/// 
/// let parsed_comment = SemanticComment::try_from("fix: some fix.").unwrap();
/// assert_eq!(parsed_comment,SemanticComment::new("some fix.".to_string(), SemanticType::Fix(SemanticTypeMetadata::new(false))));
/// ```
impl TryFrom<&str> for SemanticComment {
    type Error = SemVerError;

    fn try_from(comment: &str) -> Result<Self, Self::Error> {
    let re = Regex::new(r"^[a-zA-Z0-9_]+(:|!)").unwrap();

    if let Some(mat) = re.find(comment) {
        let prefix_delimiter = mat.end();

        let left_side = &comment[0..prefix_delimiter];
        let right_side = &comment[(prefix_delimiter)..comment.len()];

        let is_breaking = left_side.ends_with('!');

        let prefix = &left_side[0..left_side.len() - 1];

        match prefix.trim() {
            "feat" => Ok(SemanticComment::new(
                right_side.trim().to_string(),
                SemanticType::Feature(SemanticTypeMetadata::new(is_breaking)),
            )),
            "fix" => Ok(SemanticComment::new(
                right_side.trim().to_string(),
                SemanticType::Fix(SemanticTypeMetadata::new(is_breaking)),
            )),
            "refact" => Ok(SemanticComment::new(
                right_side.trim().to_string(),
                SemanticType::Refactoring(SemanticTypeMetadata::new(is_breaking)),
            )),
            _ => Err(SemVerError::UnexpectedSemanticType(prefix.to_string())),
        }
    } else {
        Err(SemVerError::InvalidCommentFormat)
    }

    }
}

#[cfg(test)]
mod test {
    use crate::{SemanticType, SemanticTypeMetadata};

    use super::*;

    #[test]
    fn test_parse_comment_retrieves_expected_semantic_type_from_comment_string() {
        let cases = vec![
            (
                "feat: feature here",
                SemanticComment::new(
                    "feature here".to_string(),
                    SemanticType::Feature(SemanticTypeMetadata::new(false)),
                ),
            ),
            (
                "feat! feature here",
                SemanticComment::new(
                    "feature here".to_string(),
                    SemanticType::Feature(SemanticTypeMetadata::new(true)),
                ),
            ),
            (
                "fix: fix here",
                SemanticComment::new(
                    "fix here".to_string(),
                    SemanticType::Fix(SemanticTypeMetadata::new(false)),
                ),
            ),
            (
                "fix! fix here",
                SemanticComment::new(
                    "fix here".to_string(),
                    SemanticType::Fix(SemanticTypeMetadata::new(true)),
                ),
            ),
            (
                "fix!fix here",
                SemanticComment::new(
                    "fix here".to_string(),
                    SemanticType::Fix(SemanticTypeMetadata::new(true)),
                ),
            ),
            (
                "refact: refactoring here",
                SemanticComment::new(
                    "refactoring here".to_string(),
                    SemanticType::Refactoring(SemanticTypeMetadata::new(false)),
                ),
            ),
            (
                "refact:refactoring here",
                SemanticComment::new(
                    "refactoring here".to_string(),
                    SemanticType::Refactoring(SemanticTypeMetadata::new(false)),
                ),
            ),
            (
                "refact! refactoring here",
                SemanticComment::new(
                    "refactoring here".to_string(),
                    SemanticType::Refactoring(SemanticTypeMetadata::new(true)),
                ),
            ),
        ];

        for (comment, expected_sem_com) in cases {
            let sem_comment = SemanticComment::try_from(comment).unwrap();

            assert_eq!(sem_comment, expected_sem_com);
        }
    }

    #[test]
    fn test_parse_comment_returns_expected_error_when_format_is_invalid() {

        let sem_ver_error = SemanticComment::try_from("this is a comment with invalid format").unwrap_err();
        assert_eq!(sem_ver_error, SemVerError::InvalidCommentFormat)
    }
    #[test]
    fn test_parse_comment_returns_expected_error_when_semantic_type_is_not_supported() {

        let sem_ver_error = SemanticComment::try_from("wop! some work around.").unwrap_err();
        assert_eq!(
            sem_ver_error,
            SemVerError::UnexpectedSemanticType("wop".to_string())
        )
    }
}
