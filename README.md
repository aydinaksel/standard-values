# Standard-Values

Values defined the same way for every business, parsed once into types that
can only hold valid values.

```toml
[dependencies]
standard-values = { git = "https://github.com/aydinaksel/standard-values", tag = "v0.1.0", features = ["serde"] }
```

## Parse, Don't Validate

Each value is built through `parse`, which tidies what was typed and either
returns the value or a problem that explains what is wrong:

```rust,ignore
use standard_values::{PhoneNumber, Problem};

match PhoneNumber::parse("+44 7700 900123") {
    Ok(number) => assert_eq!(number.as_str(), "+447700900123"),
    Err(problem) => println!("{}", problem.message()),
}
```

A function that takes a `PhoneNumber` never has to check it again. The server
parses before saving, and a browser form calls the same `parse` for live
feedback, so both show the same message. Optional fields use
`parse_optional`, which reads blank text as `None` rather than as a problem.

| Type | Standard | Held As |
|---|---|---|
| `PhoneNumber` | ITU-T E.164 | `+447700900123`, with the country it belongs to |
| `EmailAddress` | RFC 5322 dot-atom, as HTML accepts it | Lowercase |
| `DomainName` | RFC 1035 host names | Lowercase |
| `ContainerNumber` | ISO 6346, including the check digit | `CSQU3054383`, with the corrected number on a mismatch |
| `CountryCode` | ISO 3166-1 alpha-2 | `GB`, with its name |
| `CurrencyCode` | ISO 4217 alphabetic codes in current use | `GBP`, with its name, numeric code and decimal places |
| `Region` | ISO 3166-2, checked with its country | Country plus an optional subdivision; United States states are required and listed |

## What Belongs Here

Only values whose rules come from an outside standard. A business's own rules,
such as a house format for websites or the countries an ERP accepts, belong in
that business's repository, built on top of these types.

## Serde

The `serde` feature writes each value as its plain text and reads it back
through `parse`, so invalid text is refused while deserializing. A `Region`
travels as `{"country": "US", "subdivision": "FL"}` and is checked as a pair.
