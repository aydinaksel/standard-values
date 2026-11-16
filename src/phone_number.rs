//! Phone numbers in ITU-T E.164 form: a plus sign, a country calling code
//! and the national number, with no separators.

use crate::calling_code::calling_code_of;
use crate::{Problem, text_value};

const MINIMUM_PHONE_DIGITS: usize = 8;
const MAXIMUM_PHONE_DIGITS: usize = 15;
const SEPARATORS: [char; 6] = [' ', '-', '(', ')', '.', '/'];

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhoneNumber(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhoneNumberProblem {
    Empty,
    MissingPlusSign,
    UnexpectedCharacter { character: char },
    LeadingZero,
    UnassignedCountryCode,
    TooShort,
    TooLong,
}

impl Problem for PhoneNumberProblem {
    fn message(&self) -> String {
        match self {
            Self::Empty => "Enter a phone number, for example +447700900123.".to_owned(),
            Self::MissingPlusSign => {
                "Start with the country code, for example +447700900123.".to_owned()
            }
            Self::UnexpectedCharacter { character } => {
                format!("Remove the '{character}'. Digits only, after the leading plus sign.")
            }
            Self::LeadingZero => {
                "A country code cannot start with zero. Drop the trunk prefix.".to_owned()
            }
            Self::UnassignedCountryCode => {
                "No country uses this dialling code. Check the digits after the plus sign."
                    .to_owned()
            }
            Self::TooShort => {
                format!("Too short. A full number has at least {MINIMUM_PHONE_DIGITS} digits.")
            }
            Self::TooLong => {
                format!("Too long. E.164 allows at most {MAXIMUM_PHONE_DIGITS} digits.")
            }
        }
    }
}

fn without_separators(typed: &str) -> String {
    typed
        .chars()
        .filter(|character| !SEPARATORS.contains(character))
        .collect()
}

impl PhoneNumber {
    pub fn parse(typed: &str) -> Result<Self, PhoneNumberProblem> {
        let number = without_separators(typed);
        if number.is_empty() {
            return Err(PhoneNumberProblem::Empty);
        }
        let Some(digits) = number.strip_prefix('+') else {
            return Err(PhoneNumberProblem::MissingPlusSign);
        };
        if let Some(character) = digits.chars().find(|character| !character.is_ascii_digit()) {
            return Err(PhoneNumberProblem::UnexpectedCharacter { character });
        }
        if digits.starts_with('0') {
            return Err(PhoneNumberProblem::LeadingZero);
        }
        if calling_code_of(digits).is_none() {
            return Err(PhoneNumberProblem::UnassignedCountryCode);
        }
        match digits.len() {
            0..MINIMUM_PHONE_DIGITS => Err(PhoneNumberProblem::TooShort),
            length if length > MAXIMUM_PHONE_DIGITS => Err(PhoneNumberProblem::TooLong),
            _ => Ok(Self(number)),
        }
    }

    pub fn parse_optional(typed: &str) -> Result<Option<Self>, PhoneNumberProblem> {
        if without_separators(typed).is_empty() {
            return Ok(None);
        }
        Self::parse(typed).map(Some)
    }

    #[must_use]
    pub fn calling_code(&self) -> &'static str {
        self.digits()
            .and_then(calling_code_of)
            .map_or("", |calling_code| calling_code.code)
    }

    #[must_use]
    pub fn country(&self) -> &'static str {
        self.digits()
            .and_then(calling_code_of)
            .map_or("", |calling_code| calling_code.country)
    }

    fn digits(&self) -> Option<&str> {
        self.0.strip_prefix('+')
    }
}

text_value!(PhoneNumber, PhoneNumberProblem);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_international_numbers() {
        for number in [
            "+13055551234",
            "+902125551234",
            "+442071838750",
            "+61255501234",
        ] {
            assert!(PhoneNumber::parse(number).is_ok(), "{number}");
        }
    }

    #[test]
    fn drops_the_separators_people_type() {
        assert_eq!(
            PhoneNumber::parse("+1 (305) 555-1234").map(String::from),
            Ok("+13055551234".to_owned())
        );
        assert_eq!(
            PhoneNumber::parse(" +90 212 555 12 34 ").map(String::from),
            Ok("+902125551234".to_owned())
        );
    }

    #[test]
    fn names_the_country_behind_the_number() {
        let number = PhoneNumber::parse("+902125551234").unwrap();
        assert_eq!(number.calling_code(), "90");
        assert_eq!(number.country(), "Turkey");
    }

    #[test]
    fn rejects_numbers_without_a_proper_country_code() {
        assert_eq!(
            PhoneNumber::parse("(305) 555-1234"),
            Err(PhoneNumberProblem::MissingPlusSign)
        );
        assert_eq!(
            PhoneNumber::parse("+02125551234"),
            Err(PhoneNumberProblem::LeadingZero)
        );
        assert_eq!(
            PhoneNumber::parse("+2105551234"),
            Err(PhoneNumberProblem::UnassignedCountryCode)
        );
    }

    #[test]
    fn rejects_numbers_outside_the_e164_length() {
        assert_eq!(
            PhoneNumber::parse("+1305555"),
            Err(PhoneNumberProblem::TooShort)
        );
        assert_eq!(
            PhoneNumber::parse("+1234567890123456"),
            Err(PhoneNumberProblem::TooLong)
        );
    }

    #[test]
    fn names_the_character_that_does_not_belong() {
        assert_eq!(
            PhoneNumber::parse("+1305555123x"),
            Err(PhoneNumberProblem::UnexpectedCharacter { character: 'x' })
        );
    }

    #[test]
    fn treats_blank_as_absent_when_the_number_is_optional() {
        assert_eq!(PhoneNumber::parse_optional(" - "), Ok(None));
        assert_eq!(PhoneNumber::parse(""), Err(PhoneNumberProblem::Empty));
    }
}
