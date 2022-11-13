use crate::{SemanticComment, SemanticVersion, SemVerError};

/// [`calculate_version`] calculates the next semantic version given the semantic comment.
/// Expected semantic version format
/// **`v<major>.<minor>.<patch>`**
/// ## Rules for calculation
/// - `fix`     increments `<patch>`, for non breaking change.
/// - `refact`  increments `<patch>`, for non breaking change.
/// - `feat`    increments `<minor>`, for non breaking change.
/// - for breaking changes: `feat`, `refact` and `fix` changes `<major>`.
/// - Every time most significant number in version increments, the numbers below will zero.
/// ### Rules - Example 
/// Given the current version: `v1.2.3`
/// #### If `incomming_commit_comment` is non breaking, for a:
/// - fix:      `v1.2.4` 
/// - refact:   `v1.2.4` 
/// - feat:     `v1.3.0` 
/// #### If `incomming_commit_comment` is a breaking comment, for a:
/// - fix:      `v2.0.0` 
/// - refact:   `v2.0.0` 
/// - feat:     `v2.0.0`
///  
pub fn calculate_version(current_version: &str, incomming_commit_comment: SemanticComment) -> Result<String, SemVerError> {
    let mut semantic_version: SemanticVersion = current_version.try_into()?;

    match incomming_commit_comment.semantic_type {
        crate::SemanticType::Fix(meta) if !meta.is_breaking => semantic_version.patch += 1,
        crate::SemanticType::Refactoring(meta) if !meta.is_breaking => semantic_version.patch += 1,
        crate::SemanticType::Feature(meta) if !meta.is_breaking => semantic_version.minor += 1,
        _ => {
            semantic_version.major += 1;
            semantic_version.minor = 0;
            semantic_version.patch = 0;
        }
    }

    Ok(semantic_version.into())
}

mod test {
    use crate::parse_comment;

    use super::*;

    #[test]
    fn test_calculate_version_calculates_new_version_according_to_expected() {
        let (comment, current_version, expected_version) = ("fix: this is a fix", "v2.3.5", "v2.3.6");


        let semantic_comment = parse_comment(comment).unwrap();
        let new_version = calculate_version(current_version, semantic_comment).unwrap();

        assert_eq!(new_version, expected_version)
    }
}
