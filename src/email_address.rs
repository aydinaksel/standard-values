//! Email addresses in the dot-atom subset of RFC 5322 that the HTML living
//! standard accepts for `<input type="email">`: no quoted local parts and no
//! address literals. Held in lowercase.

use crate::domain_name::{DomainNameProblem, check_domain};
use crate::{Problem, text_value};

const MAXIMUM_EMAIL_LENGTH: usize = 254;
const MAXIMUM_LOCAL_PART_LENGTH: usize = 64;
const LOCAL_PART_SYMBOLS: &str = "!#$%&'*+/=?^_`{|}~-";

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmailAddress(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailAddressProblem {
    Empty,
    MissingAtSign,
    ManyAtSigns,
    EmptyLocalPart,
    LocalPartTooLong,
    UnexpectedLocalPartCharacter { character: char },
    MisplacedLocalPartDot,
    MissingDomainDot,
    MalformedDomainLabel,
    UnexpectedDomainCharacter { character: char },
    TooLong,
}

impl Problem for EmailAddressProblem {
    fn message(&self) -> String {
        match self {
            Self::Empty => "Enter an email address.".to_owned(),
            Self::MissingAtSign => "An email address needs an @.".to_owned(),
            Self::ManyAtSigns => "An email address can only hold one @.".to_owned(),
            Self::EmptyLocalPart => "Add the part before the @.".to_owned(),
            Self::LocalPartTooLong => {
                format!("The part before the @ is over {MAXIMUM_LOCAL_PART_LENGTH} characters.")
            }
            Self::UnexpectedLocalPartCharacter { character } => {
                format!("'{character}' is not allowed before the @.")
            }
            Self::MisplacedLocalPartDot => {
                "Dots before the @ cannot lead, trail, or repeat.".to_owned()
            }
            Self::MissingDomainDot => "The domain needs a dot, for example example.com.".to_owned(),
            Self::MalformedDomainLabel => {
                "Each part of the domain must be filled in and cannot start or end with a hyphen."
                    .to_owned()
            }
            Self::UnexpectedDomainCharacter { character } => {
                format!("'{character}' is not allowed in the domain.")
            }
            Self::TooLong => format!("Longer than the {MAXIMUM_EMAIL_LENGTH} character limit."),
        }
    }
}

fn check_local_part(local_part: &str) -> Result<(), EmailAddressProblem> {
    if local_part.is_empty() {
        return Err(EmailAddressProblem::EmptyLocalPart);
    }
    if local_part.len() > MAXIMUM_LOCAL_PART_LENGTH {
        return Err(EmailAddressProblem::LocalPartTooLong);
    }
    if local_part.starts_with('.') || local_part.ends_with('.') || local_part.contains("..") {
        return Err(EmailAddressProblem::MisplacedLocalPartDot);
    }
    let unexpected = local_part.chars().find(|character| {
        !character.is_ascii_alphanumeric()
            && *character != '.'
            && !LOCAL_PART_SYMBOLS.contains(*character)
    });
    match unexpected {
        Some(character) => Err(EmailAddressProblem::UnexpectedLocalPartCharacter { character }),
        None => Ok(()),
    }
}

fn email_problem_of(domain_problem: DomainNameProblem) -> EmailAddressProblem {
    match domain_problem {
        DomainNameProblem::Empty | DomainNameProblem::MissingDot => {
            EmailAddressProblem::MissingDomainDot
        }
        DomainNameProblem::MalformedLabel => EmailAddressProblem::MalformedDomainLabel,
        DomainNameProblem::UnexpectedCharacter { character } => {
            EmailAddressProblem::UnexpectedDomainCharacter { character }
        }
        DomainNameProblem::TooLong => EmailAddressProblem::TooLong,
    }
}

impl EmailAddress {
    pub fn parse(typed: &str) -> Result<Self, EmailAddressProblem> {
        let address = typed.trim().to_lowercase();
        if address.is_empty() {
            return Err(EmailAddressProblem::Empty);
        }
        if address.len() > MAXIMUM_EMAIL_LENGTH {
            return Err(EmailAddressProblem::TooLong);
        }
        let mut parts = address.split('@');
        let local_part = parts.next().unwrap_or_default();
        let Some(domain) = parts.next() else {
            return Err(EmailAddressProblem::MissingAtSign);
        };
        if parts.next().is_some() {
            return Err(EmailAddressProblem::ManyAtSigns);
        }
        check_local_part(local_part)?;
        check_domain(domain).map_err(email_problem_of)?;
        Ok(Self(address))
    }

    pub fn parse_optional(typed: &str) -> Result<Option<Self>, EmailAddressProblem> {
        if typed.trim().is_empty() {
            return Ok(None);
        }
        Self::parse(typed).map(Some)
    }

    #[must_use]
    pub fn domain(&self) -> &str {
        self.0.split_once('@').map_or("", |(_, domain)| domain)
    }
}

text_value!(EmailAddress, EmailAddressProblem);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_addresses_people_actually_use() {
        for address in [
            "sales@example.com",
            "first.last@example.co.uk",
            "quote+lead@sub.example.com",
            "a@b.co",
        ] {
            assert!(EmailAddress::parse(address).is_ok(), "{address}");
        }
    }

    #[test]
    fn holds_addresses_in_lowercase() {
        let address = EmailAddress::parse("  Sales@Example.COM ").unwrap();
        assert_eq!(address.as_str(), "sales@example.com");
        assert_eq!(address.domain(), "example.com");
    }

    #[test]
    fn rejects_addresses_missing_their_parts() {
        assert_eq!(
            EmailAddress::parse("sales.example.com"),
            Err(EmailAddressProblem::MissingAtSign)
        );
        assert_eq!(
            EmailAddress::parse("@example.com"),
            Err(EmailAddressProblem::EmptyLocalPart)
        );
        assert_eq!(
            EmailAddress::parse("sales@example"),
            Err(EmailAddressProblem::MissingDomainDot)
        );
        assert_eq!(
            EmailAddress::parse("sales@@example.com"),
            Err(EmailAddressProblem::ManyAtSigns)
        );
    }

    #[test]
    fn rejects_malformed_dots_and_hyphens() {
        for (address, problem) in [
            (
                ".sales@example.com",
                EmailAddressProblem::MisplacedLocalPartDot,
            ),
            (
                "first..last@example.com",
                EmailAddressProblem::MisplacedLocalPartDot,
            ),
            (
                "sales@example..com",
                EmailAddressProblem::MalformedDomainLabel,
            ),
            (
                "sales@-example.com",
                EmailAddressProblem::MalformedDomainLabel,
            ),
        ] {
            assert_eq!(EmailAddress::parse(address), Err(problem), "{address}");
        }
    }

    #[test]
    fn names_the_character_that_does_not_belong() {
        assert_eq!(
            EmailAddress::parse("sa les@example.com"),
            Err(EmailAddressProblem::UnexpectedLocalPartCharacter { character: ' ' })
        );
        assert_eq!(
            EmailAddress::parse("sales@exa_mple.com"),
            Err(EmailAddressProblem::UnexpectedDomainCharacter { character: '_' })
        );
    }

    #[test]
    fn treats_blank_as_absent_when_the_address_is_optional() {
        assert_eq!(EmailAddress::parse_optional("  "), Ok(None));
        assert_eq!(EmailAddress::parse(""), Err(EmailAddressProblem::Empty));
    }
}
