use std::fmt::Display;

use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
/// [SemVerError]
/// 
/// Provides error that can occur when parsing comment.
pub enum SemVerError {
    #[error("The format provided is invalid!")]
    InvalidFormat,
    #[error("Unexpected semantic type")]
    UnexpectedSemanticType(String),
}

/// Provides type to inform if the [`SemanticType`] was anotated with:
/// 
/// : -> Non breaking. Eg.: `feat:`
/// ! -> Breaking.     Eg.: `feat!`
type IsBreaking = bool;

/// Provides semantic type assumed from the commit message.
/// # Possible breaking values
/// - fix!, feat!, refact!
/// # Possible non breaking values
/// - fix:, feat:, refact:
# [derive(Debug)]
pub enum SemanticType {
    Fix(IsBreaking),
    Feature(IsBreaking),
    Refactoring(IsBreaking),
}

#[derive(Debug)]
pub struct SemanticComment {
    pub comment: String,
    pub semantic_type: SemanticType,
}

pub fn parse_comment(comment: &str) -> Result<SemanticComment, SemVerError> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_comment_retrieves_expected_semantic_type_from_comment_string() {
        let comment = "feat: this is a feature";

        let sem_comment = parse_comment(comment);

        
    }
}
