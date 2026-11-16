#![cfg(feature = "serde")]

use standard_values::{ContainerNumber, CurrencyCode, EmailAddress, PhoneNumber, Region};

#[test]
fn values_travel_as_their_plain_text() {
    let number = PhoneNumber::parse("+44 7700 900123").unwrap();
    assert_eq!(serde_json::to_string(&number).unwrap(), "\"+447700900123\"");
    let email: EmailAddress = serde_json::from_str("\"Sales@Example.com\"").unwrap();
    assert_eq!(email.as_str(), "sales@example.com");
}

#[test]
fn invalid_text_is_refused_while_deserializing() {
    let refused = serde_json::from_str::<ContainerNumber>("\"CSQU3054389\"").unwrap_err();
    assert!(refused.to_string().contains("Did you mean CSQU3054383?"));
    serde_json::from_str::<PhoneNumber>("\"07700 900123\"").unwrap_err();
}

#[test]
fn regions_are_checked_as_a_pair() {
    let florida: Region = serde_json::from_str(r#"{"country":"US","subdivision":"FL"}"#).unwrap();
    assert_eq!(florida.subdivision_name(), Some("Florida"));
    serde_json::from_str::<Region>(r#"{"country":"US","subdivision":null}"#).unwrap_err();
}

#[test]
fn currency_codes_travel_as_their_code_and_withdrawn_ones_are_refused() {
    let euro: CurrencyCode = serde_json::from_str("\"eur\"").unwrap();
    assert_eq!(serde_json::to_string(&euro).unwrap(), "\"EUR\"");
    serde_json::from_str::<CurrencyCode>("\"BGN\"").unwrap_err();
}
