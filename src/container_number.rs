//! ISO 6346 container numbers: a three-letter owner code, a one-letter
//! equipment category, a six-digit serial and a check digit.

use crate::{Problem, text_value};

const OWNER_CODE_LENGTH: usize = 3;
const PREFIX_LENGTH: usize = OWNER_CODE_LENGTH + 1;
const SERIAL_LENGTH: usize = 6;
const STEM_LENGTH: usize = PREFIX_LENGTH + SERIAL_LENGTH;
const CONTAINER_NUMBER_LENGTH: usize = STEM_LENGTH + 1;
const STANDARD_EQUIPMENT_CATEGORIES: [char; 3] = ['U', 'J', 'Z'];
const CONTAINER_NUMBER_EXAMPLE: &str = "CSQU3054383";

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContainerNumber(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerNumberProblem {
    Empty,
    WrongShape,
    CheckDigitMismatch { corrected: ContainerNumber },
}

impl Problem for ContainerNumberProblem {
    fn message(&self) -> String {
        match self {
            Self::Empty => {
                format!("Enter a container number, for example {CONTAINER_NUMBER_EXAMPLE}.")
            }
            Self::WrongShape => format!(
                "A container number is four letters followed by seven digits, such as {CONTAINER_NUMBER_EXAMPLE}."
            ),
            Self::CheckDigitMismatch { corrected } => {
                format!("The check digit does not match. Did you mean {corrected}?")
            }
        }
    }
}

fn without_separators(typed: &str) -> String {
    typed
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|character| character.to_ascii_uppercase())
        .collect()
}

fn is_container_shaped(characters: &[char]) -> bool {
    let (prefix, serial_and_check_digit) = characters.split_at(PREFIX_LENGTH.min(characters.len()));
    characters.len() == CONTAINER_NUMBER_LENGTH
        && prefix.iter().all(char::is_ascii_uppercase)
        && serial_and_check_digit.iter().all(char::is_ascii_digit)
}

/// Letters take the values from ten upward, skipping multiples of eleven.
/// See <https://en.wikipedia.org/wiki/ISO_6346#Check_digit>.
fn character_value(character: char) -> Option<u32> {
    if let Some(digit) = character.to_digit(10) {
        return Some(digit);
    }
    let ordinal = u32::from(character).checked_sub(u32::from('A'))?;
    (10..)
        .filter(|value| value % 11 != 0)
        .nth(usize::try_from(ordinal).ok()?)
}

/// Each of the first ten characters is weighted by an ascending power of
/// two, and the total is reduced modulo eleven, writing a remainder of ten
/// as zero. See <https://en.wikipedia.org/wiki/ISO_6346#Check_digit>.
fn check_digit(stem: &[char]) -> Option<u32> {
    let total = stem
        .iter()
        .enumerate()
        .try_fold(0_u32, |total, (position, character)| {
            let weighted =
                character_value(*character)?.checked_shl(u32::try_from(position).ok()?)?;
            total.checked_add(weighted)
        })?;
    Some(total % 11 % 10)
}

impl ContainerNumber {
    pub fn parse(typed: &str) -> Result<Self, ContainerNumberProblem> {
        let number = without_separators(typed);
        if number.is_empty() {
            return Err(ContainerNumberProblem::Empty);
        }
        let characters: Vec<char> = number.chars().collect();
        if !is_container_shaped(&characters) {
            return Err(ContainerNumberProblem::WrongShape);
        }
        let Some((last, stem)) = characters.split_last() else {
            return Err(ContainerNumberProblem::WrongShape);
        };
        let Some(expected) = check_digit(stem) else {
            return Err(ContainerNumberProblem::WrongShape);
        };
        if last.to_digit(10) != Some(expected) {
            let stem: String = stem.iter().collect();
            return Err(ContainerNumberProblem::CheckDigitMismatch {
                corrected: Self(format!("{stem}{expected}")),
            });
        }
        Ok(Self(number))
    }

    pub fn parse_optional(typed: &str) -> Result<Option<Self>, ContainerNumberProblem> {
        if without_separators(typed).is_empty() {
            return Ok(None);
        }
        Self::parse(typed).map(Some)
    }

    #[must_use]
    pub fn prefix(&self) -> &str {
        self.0.get(..PREFIX_LENGTH).unwrap_or_default()
    }

    #[must_use]
    pub fn owner_code(&self) -> &str {
        self.0.get(..OWNER_CODE_LENGTH).unwrap_or_default()
    }

    #[must_use]
    pub fn equipment_category(&self) -> char {
        self.0.chars().nth(OWNER_CODE_LENGTH).unwrap_or_default()
    }

    #[must_use]
    pub fn has_standard_equipment_category(&self) -> bool {
        STANDARD_EQUIPMENT_CATEGORIES.contains(&self.equipment_category())
    }
}

text_value!(ContainerNumber, ContainerNumberProblem);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_known_valid_numbers() {
        for number in ["CSQU3054383", "MSCU1234566", "TGHU7897799", "APZU3741578"] {
            assert!(ContainerNumber::parse(number).is_ok(), "{number}");
        }
    }

    #[test]
    fn drops_separators_and_capitalises() {
        assert_eq!(
            ContainerNumber::parse("  mscu 123456-6 ").map(String::from),
            Ok("MSCU1234566".to_owned())
        );
    }

    #[test]
    fn rejects_every_single_digit_typo() {
        let valid = "CSQU3054383";
        for position in PREFIX_LENGTH..CONTAINER_NUMBER_LENGTH {
            for replacement in '0'..='9' {
                let mut mistyped: Vec<char> = valid.chars().collect();
                if mistyped[position] == replacement {
                    continue;
                }
                mistyped[position] = replacement;
                let mistyped: String = mistyped.into_iter().collect();
                assert!(
                    matches!(
                        ContainerNumber::parse(&mistyped),
                        Err(ContainerNumberProblem::CheckDigitMismatch { .. })
                    ),
                    "{mistyped}"
                );
            }
        }
    }

    #[test]
    fn suggests_the_number_with_the_right_check_digit() {
        let Err(ContainerNumberProblem::CheckDigitMismatch { corrected }) =
            ContainerNumber::parse("CSQU3054389")
        else {
            panic!("expected a check digit mismatch");
        };
        assert_eq!(corrected.as_str(), "CSQU3054383");
    }

    #[test]
    fn accepts_unusual_equipment_categories_but_reports_them() {
        let number = ContainerNumber::parse("CSQA3054383").unwrap();
        assert_eq!(number.owner_code(), "CSQ");
        assert_eq!(number.prefix(), "CSQA");
        assert_eq!(number.equipment_category(), 'A');
        assert!(!number.has_standard_equipment_category());
    }

    #[test]
    fn rejects_anything_that_is_not_the_right_shape() {
        for typed in ["ABC-123/45", "CSQU305438", "CSQU30543834", "1234567890A"] {
            assert_eq!(
                ContainerNumber::parse(typed),
                Err(ContainerNumberProblem::WrongShape),
                "{typed}"
            );
        }
        assert_eq!(
            ContainerNumber::parse(""),
            Err(ContainerNumberProblem::Empty)
        );
        assert_eq!(ContainerNumber::parse_optional(" "), Ok(None));
    }
}
