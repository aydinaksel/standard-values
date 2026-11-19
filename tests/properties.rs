#![cfg(test)]

use proptest::prelude::*;
use standard_values::{
    ContainerNumber, ContainerNumberProblem, CountryCode, CurrencyCode, DomainName, EmailAddress,
    PhoneNumber, Region,
};

const ISO_6346_LETTER_VALUES: [(char, u32); 26] = [
    ('A', 10), ('B', 12), ('C', 13), ('D', 14), ('E', 15), ('F', 16), ('G', 17), ('H', 18),
    ('I', 19), ('J', 20), ('K', 21), ('L', 23), ('M', 24), ('N', 25), ('O', 26), ('P', 27),
    ('Q', 28), ('R', 29), ('S', 30), ('T', 31), ('U', 32), ('V', 34), ('W', 35), ('X', 36),
    ('Y', 37), ('Z', 38),
];
const SEPARATORS: [&str; 4] = ["", " ", "-", " - "];

fn iso_6346_value(character: char) -> u32 {
    character.to_digit(10).unwrap_or_else(|| {
        ISO_6346_LETTER_VALUES
            .iter()
            .find(|(letter, _)| *letter == character)
            .map(|(_, value)| *value)
            .unwrap()
    })
}

fn iso_6346_check_digit(stem: &str) -> u32 {
    let total: u32 = stem
        .chars()
        .zip(0..)
        .map(|(character, position)| {
            iso_6346_value(character)
                .checked_mul(2_u32.checked_pow(position).unwrap())
                .unwrap()
        })
        .sum();
    total.checked_rem(11).and_then(|remainder| remainder.checked_rem(10)).unwrap()
}

fn container_stem() -> impl Strategy<Value = String> {
    "[A-Z]{4}[0-9]{6}"
}

fn valid_container_number() -> impl Strategy<Value = String> {
    container_stem().prop_map(|stem| {
        let check_digit = iso_6346_check_digit(&stem);
        format!("{stem}{check_digit}")
    })
}

fn email_local_part() -> impl Strategy<Value = String> {
    "[A-Za-z0-9!#$%&'*+/=?^_`{|}~-]{1,20}(\\.[A-Za-z0-9!#$%&'*+/=?^_`{|}~-]{1,20}){0,2}"
}

fn domain_label() -> impl Strategy<Value = String> {
    "[A-Za-z0-9]([A-Za-z0-9-]{0,20}[A-Za-z0-9])?"
}

fn domain() -> impl Strategy<Value = String> {
    prop::collection::vec(domain_label(), 2..=4).prop_map(|labels| labels.join("."))
}

fn is_ascii_lowercase_text(text: &str) -> bool {
    !text.chars().any(|character| character.is_ascii_uppercase())
}

proptest! {
    #[test]
    fn every_number_with_its_iso_6346_check_digit_is_accepted(number in valid_container_number()) {
        let parsed = ContainerNumber::parse(&number).unwrap();
        prop_assert_eq!(parsed.as_str(), number.as_str());
        prop_assert_eq!(parsed.prefix(), number.get(..4).unwrap());
        prop_assert_eq!(parsed.owner_code(), number.get(..3).unwrap());
    }

    #[test]
    fn container_numbers_agree_with_an_independent_iso_6346_implementation(
        stem in container_stem(),
        last_digit in 0_u32..10,
    ) {
        let number = format!("{stem}{last_digit}");
        let expected = iso_6346_check_digit(&stem);
        let outcome = ContainerNumber::parse(&number);
        if last_digit == expected {
            prop_assert_eq!(outcome.map(String::from), Ok(number));
        } else {
            let corrected = ContainerNumber::parse(&format!("{stem}{expected}")).unwrap();
            prop_assert_eq!(outcome, Err(ContainerNumberProblem::CheckDigitMismatch { corrected }));
        }
    }

    #[test]
    fn a_container_number_reads_the_same_with_separators_and_lowercase(
        number in valid_container_number(),
        separator in prop::sample::select(SEPARATORS.to_vec()),
    ) {
        let typed = format!(
            "{} {}{separator}{}",
            number.get(..4).unwrap().to_lowercase(),
            number.get(4..10).unwrap(),
            number.get(10..).unwrap()
        );
        prop_assert_eq!(ContainerNumber::parse(&typed).map(String::from), Ok(number));
    }

    #[test]
    fn an_email_address_is_stored_trimmed_and_lowercased(
        local_part in email_local_part(),
        domain in domain(),
    ) {
        let typed = format!("  {local_part}@{domain} ");
        let parsed = EmailAddress::parse(&typed).unwrap();
        let expected = format!("{local_part}@{domain}").to_lowercase();
        prop_assert_eq!(parsed.as_str(), expected.as_str());
        prop_assert_eq!(parsed.domain(), domain.to_lowercase());
        prop_assert!(DomainName::parse(parsed.domain()).is_ok());
    }

    #[test]
    fn any_text_parses_or_is_refused_without_panicking(typed in ".*") {
        let _container = ContainerNumber::parse(&typed);
        let _email = EmailAddress::parse(&typed);
        let _domain = DomainName::parse(&typed);
        let _phone = PhoneNumber::parse(&typed);
        let _country = CountryCode::parse(&typed);
        let _currency = CurrencyCode::parse(&typed);
        let _region = Region::parse(&typed, &typed);
    }

    #[test]
    fn parsing_a_stored_value_gives_back_the_same_value(typed in ".{0,40}") {
        if let Ok(number) = ContainerNumber::parse(&typed) {
            prop_assert_eq!(ContainerNumber::parse(number.as_str()), Ok(number));
        }
        if let Ok(address) = EmailAddress::parse(&typed) {
            prop_assert!(is_ascii_lowercase_text(address.as_str()));
            prop_assert_eq!(EmailAddress::parse(address.as_str()), Ok(address));
        }
        if let Ok(domain) = DomainName::parse(&typed) {
            prop_assert!(is_ascii_lowercase_text(domain.as_str()));
            prop_assert_eq!(DomainName::parse(domain.as_str()), Ok(domain));
        }
        if let Ok(number) = PhoneNumber::parse(&typed) {
            prop_assert_eq!(PhoneNumber::parse(number.as_str()), Ok(number));
        }
    }

    #[test]
    fn stored_email_addresses_and_domains_round_trip(
        local_part in email_local_part(),
        domain in domain(),
    ) {
        let address = EmailAddress::parse(&format!("{local_part}@{domain}")).unwrap();
        prop_assert_eq!(EmailAddress::parse(address.as_str()), Ok(address.clone()));
        let domain = DomainName::parse(&domain).unwrap();
        prop_assert_eq!(DomainName::parse(domain.as_str()), Ok(domain));
    }

    #[test]
    fn a_phone_number_is_stored_without_its_separators(
        national in "[1-9][0-9]{6,12}",
        separator in prop::sample::select(vec![" ", "-", ".", " (0) "]),
    ) {
        let digits = format!("44{national}");
        let typed = format!("+44{separator}{national}");
        let expected = format!("+{digits}");
        prop_assert_eq!(PhoneNumber::parse(&typed).map(String::from), Ok(expected));
    }

    #[test]
    fn a_blank_optional_value_is_none(blank in "[ \t]{0,5}") {
        prop_assert_eq!(ContainerNumber::parse_optional(&blank), Ok(None));
        prop_assert_eq!(EmailAddress::parse_optional(&blank), Ok(None));
        prop_assert_eq!(PhoneNumber::parse_optional(&blank), Ok(None));
        prop_assert_eq!(CountryCode::parse_optional(&blank), Ok(None));
        prop_assert_eq!(CurrencyCode::parse_optional(&blank), Ok(None));
    }

    #[test]
    fn a_three_letter_code_is_a_currency_exactly_when_iso_4217_lists_it(code in "[A-Za-z]{3}") {
        let listed = CurrencyCode::all().any(|currency| currency.as_str() == code.to_uppercase());
        prop_assert_eq!(CurrencyCode::parse(&code).is_ok(), listed);
    }

    #[test]
    fn a_two_letter_code_is_a_country_exactly_when_iso_3166_lists_it(code in "[A-Za-z]{2}") {
        let listed = CountryCode::all().any(|country| country.as_str() == code.to_uppercase());
        prop_assert_eq!(CountryCode::parse(&code).is_ok(), listed);
    }
}

#[test]
fn every_listed_currency_and_country_reads_back_in_any_case() {
    for currency in CurrencyCode::all() {
        let typed = format!(" {} ", currency.as_str().to_lowercase());
        assert_eq!(CurrencyCode::parse(&typed), Ok(currency));
    }
    for country in CountryCode::all() {
        let typed = country.as_str().to_lowercase();
        assert_eq!(CountryCode::parse(&typed), Ok(country));
    }
}
