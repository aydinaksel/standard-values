//! Domain names as RFC 1035 host names: dot-separated labels of letters,
//! digits and inner hyphens. Held in lowercase.

use crate::{Problem, text_value};

const MAXIMUM_LABEL_LENGTH: usize = 63;
const MAXIMUM_DOMAIN_LENGTH: usize = 253;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DomainName(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainNameProblem {
    Empty,
    MissingDot,
    MalformedLabel,
    UnexpectedCharacter { character: char },
    TooLong,
}

impl Problem for DomainNameProblem {
    fn message(&self) -> String {
        match self {
            Self::Empty => "Enter a domain, for example example.com.".to_owned(),
            Self::MissingDot => "The domain needs a dot, for example example.com.".to_owned(),
            Self::MalformedLabel => {
                "Each part of the domain must be filled in and cannot start or end with a hyphen."
                    .to_owned()
            }
            Self::UnexpectedCharacter { character } => {
                format!("'{character}' is not allowed in a domain.")
            }
            Self::TooLong => format!("Longer than the {MAXIMUM_DOMAIN_LENGTH} character limit."),
        }
    }
}

fn is_well_formed_label(label: &str) -> bool {
    !label.is_empty()
        && label.len() <= MAXIMUM_LABEL_LENGTH
        && !label.starts_with('-')
        && !label.ends_with('-')
}

pub(crate) fn check_domain(domain: &str) -> Result<(), DomainNameProblem> {
    if domain.is_empty() {
        return Err(DomainNameProblem::Empty);
    }
    if domain.len() > MAXIMUM_DOMAIN_LENGTH {
        return Err(DomainNameProblem::TooLong);
    }
    if !domain.contains('.') {
        return Err(DomainNameProblem::MissingDot);
    }
    let unexpected = domain
        .chars()
        .find(|character| !character.is_ascii_alphanumeric() && !matches!(character, '.' | '-'));
    if let Some(character) = unexpected {
        return Err(DomainNameProblem::UnexpectedCharacter { character });
    }
    if domain.split('.').all(is_well_formed_label) {
        Ok(())
    } else {
        Err(DomainNameProblem::MalformedLabel)
    }
}

impl DomainName {
    pub fn parse(typed: &str) -> Result<Self, DomainNameProblem> {
        let domain = typed.trim().to_lowercase();
        check_domain(&domain)?;
        Ok(Self(domain))
    }
}

text_value!(DomainName, DomainNameProblem);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_and_lowercases_domains() {
        assert_eq!(
            DomainName::parse(" Example.CO.uk ").map(String::from),
            Ok("example.co.uk".to_owned())
        );
    }

    #[test]
    fn rejects_malformed_domains() {
        assert_eq!(DomainName::parse(""), Err(DomainNameProblem::Empty));
        assert_eq!(
            DomainName::parse("example"),
            Err(DomainNameProblem::MissingDot)
        );
        assert_eq!(
            DomainName::parse("example..com"),
            Err(DomainNameProblem::MalformedLabel)
        );
        assert_eq!(
            DomainName::parse("-example.com"),
            Err(DomainNameProblem::MalformedLabel)
        );
        assert_eq!(
            DomainName::parse("exa_mple.com"),
            Err(DomainNameProblem::UnexpectedCharacter { character: '_' })
        );
    }
}
