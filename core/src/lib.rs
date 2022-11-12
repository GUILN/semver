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
#[derive(Debug)]
pub enum SemanticType {
    Fix(IsBreaking),
    Feature(IsBreaking),
    Refactoring(IsBreaking),
}

impl PartialEq for SemanticType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Fix(l_isbreaking), Self::Fix(r_isbreaking)) if l_isbreaking == r_isbreaking => {
                true
            }
            (Self::Feature(l_isbreaking), Self::Feature(r_isbreaking))
                if l_isbreaking == r_isbreaking =>
            {
                true
            }
            (Self::Refactoring(l_isbreaking), Self::Refactoring(r_isbreaking))
                if l_isbreaking == r_isbreaking =>
            {
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug)]
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
}

pub fn parse_comment(comment: &str) -> Result<SemanticComment, SemVerError> {
    let non_breaking_split_number = comment.find(':');
    let breaking_split_number = comment.find('!');

    let split_result = match (non_breaking_split_number, breaking_split_number) {
        (None, Some(n)) => Some((n, true)),
        (Some(n), None) => Some((n, false)),
        _ => None,
    }.ok_or(SemVerError::InvalidFormat)?;

   
    let left_side = &comment[0..split_result.0];
    let right_side = &comment[(split_result.0 + 1)..comment.len()];

    match left_side {
        "feat" => Ok(SemanticComment::new(right_side.trim().to_string(), SemanticType::Feature((split_result.1)))),
        _ => Err(SemVerError::UnexpectedSemanticType(left_side.to_string()))
    } 
}

//#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_comment_retrieves_expected_semantic_type_from_comment_string() {
        let comment = "feat: this is a feature";

        let sem_comment = parse_comment(comment).unwrap();
        assert_eq!(sem_comment.comment, "this is a feature");
        assert_eq!(sem_comment.semantic_type, SemanticType::Feature(false))
    }
}
