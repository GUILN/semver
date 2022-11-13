use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::{convert::TryFrom, num::ParseIntError};

#[non_exhaustive]
#[derive(Debug, Error, PartialEq)]
/// [SemVerError]
///
/// Provides error that can occur when parsing comment.
pub enum SemVerError {
    #[error("The format provided is invalid!")]
    InvalidCommentFormat,
    #[error("Unexpected semantic type")]
    UnexpectedSemanticType(String),
    #[error("error while deserializing")]
    DeserializationError,
    #[error("invalid version format")]
    InvalidVersionFormat(String),
    #[error("error when converting version numbers")]
    ErrorWhenConveringVersionNumber,
}

impl From<serde_json::Error> for SemVerError {
    fn from(_: serde_json::Error) -> Self {
        Self::DeserializationError
    }
}

impl From<ParseIntError> for SemVerError {
    fn from(_: ParseIntError) -> Self {
        Self::ErrorWhenConveringVersionNumber
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

/// [`SemantiVersion`] provides a structure to hold version string.
/// 
/// **expected format:** `v.1.0.0`.
#[derive(Debug, PartialEq)]
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Default for SemanticVersion {
    fn default() -> Self {
        Self { major: 0, minor: 0, patch: 0 }
    }
}

/// 
/// # Example
/// ```
/// # use core::*;
/// assert_eq!(SemanticVersion::try_from("v.1.2.3").unwrap(), SemanticVersion{ major: 1, minor: 2, patch: 3 });
/// assert_eq!(SemanticVersion::try_from("v.40.2.8").unwrap(), SemanticVersion{ major: 40, minor: 2, patch: 8 });
/// assert_eq!(SemanticVersion::try_from("v.1.300.3").unwrap(), SemanticVersion{ major: 1, minor: 300, patch: 3 });
/// 
/// assert_eq!(SemanticVersion::try_from("version-1").unwrap_err(), SemVerError::InvalidVersionFormat("version-1".to_string()));
/// assert_eq!(SemanticVersion::try_from("v34.34.2").unwrap_err(), SemVerError::InvalidVersionFormat("v34.34.2".to_string()));
/// ```
impl TryFrom<&str> for SemanticVersion {
    type Error = SemVerError;

    fn try_from(version_str: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"v(\.[0-9]+){3}").unwrap();
        if !re.is_match(version_str) {
            return Err(SemVerError::InvalidVersionFormat(version_str.to_string()));
        }

        let version_numbers = &version_str[2..version_str.len()];
        let version_numbers_vector: Vec<&str> = version_numbers.split(".").collect();

        Ok(SemanticVersion{
            major: version_numbers_vector[0].parse()?,
            minor: version_numbers_vector[1].parse()?,
            patch: version_numbers_vector[2].parse()?,
        })
    }
}

mod test {
    use super::*;

    #[test]
    fn semantic_version_try_from_creates_right_semantic_version_from_version_string() {
        let semantic_version = SemanticVersion::try_from("v.1.2.3").unwrap();
        assert_eq!(semantic_version, SemanticVersion{ major: 1, minor: 2, patch: 3 });
    }
}
