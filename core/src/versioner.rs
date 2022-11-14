use crate::{SemVerError, SemanticComment, SemanticVersion, SemanticType};

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
/// # Example
/// ```
/// use core::*;
///
/// assert_eq!(calculate_version("v2.3.5", "fix: this is a fix.".try_into().unwrap()).unwrap(), "v2.3.6");
/// assert_eq!(calculate_version("v2.3.5", "feat: this is a new feature.".try_into().unwrap()).unwrap(), "v2.4.0");
/// assert_eq!(calculate_version("v30.3.5", "fix! this is a breaking fix.".try_into().unwrap()).unwrap(), "v31.0.0");
/// assert_eq!(calculate_version("v2.3.5", "feat! this is a breaking feature.".try_into().unwrap()).unwrap(), "v3.0.0");
/// assert_eq!(calculate_version("v2.3.5", "refact: this is a refactor.".try_into().unwrap()).unwrap(), "v2.3.6");
/// ```
pub fn calculate_version(
    current_version: &str,
    incomming_commit_comment: SemanticComment,
) -> Result<String, SemVerError> {
    let mut semantic_version: SemanticVersion = current_version.try_into()?;

    match incomming_commit_comment.semantic_type {
        SemanticType::Fix(meta) if !meta.is_breaking => semantic_version.patch += 1,
        SemanticType::Refactoring(meta) if !meta.is_breaking => semantic_version.patch += 1,
        SemanticType::Feature(meta) if !meta.is_breaking => {
            semantic_version.minor += 1;
            semantic_version.patch = 0;
        }
        _ => {
            semantic_version.major += 1;
            semantic_version.minor = 0;
            semantic_version.patch = 0;
        }
    }

    Ok(semantic_version.into())
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_calculate_version_calculates_new_version_according_to_expected() {
        let (comment, current_version, expected_version) =
            ("fix: this is a fix", "v2.3.5", "v2.3.6");

        let semantic_comment = comment.try_into().unwrap();
        let new_version = calculate_version(current_version, semantic_comment).unwrap();

        assert_eq!(new_version, expected_version);
        assert_eq!(
            calculate_version("v2.3.5", "fix: this is a fix.".try_into().unwrap()).unwrap(),
            "v2.3.6"
        )
    }
}
