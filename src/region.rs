//! ISO 3166-2 subdivisions, held together with their country. United States
//! subdivisions are checked against the full list, others by shape.

use crate::{CountryCode, CountryCodeProblem, Problem};

const UNITED_STATES: &str = "US";
const MAXIMUM_SUBDIVISION_LENGTH: usize = 3;

const UNITED_STATES_SUBDIVISIONS: &[(&str, &str)] = &[
    ("AL", "Alabama"),
    ("AK", "Alaska"),
    ("AS", "American Samoa"),
    ("AZ", "Arizona"),
    ("AR", "Arkansas"),
    ("CA", "California"),
    ("CO", "Colorado"),
    ("CT", "Connecticut"),
    ("DE", "Delaware"),
    ("DC", "District of Columbia"),
    ("FL", "Florida"),
    ("GA", "Georgia"),
    ("GU", "Guam"),
    ("HI", "Hawaii"),
    ("ID", "Idaho"),
    ("IL", "Illinois"),
    ("IN", "Indiana"),
    ("IA", "Iowa"),
    ("KS", "Kansas"),
    ("KY", "Kentucky"),
    ("LA", "Louisiana"),
    ("ME", "Maine"),
    ("MD", "Maryland"),
    ("MA", "Massachusetts"),
    ("MI", "Michigan"),
    ("MN", "Minnesota"),
    ("MS", "Mississippi"),
    ("MO", "Missouri"),
    ("MT", "Montana"),
    ("NE", "Nebraska"),
    ("NV", "Nevada"),
    ("NH", "New Hampshire"),
    ("NJ", "New Jersey"),
    ("NM", "New Mexico"),
    ("NY", "New York"),
    ("NC", "North Carolina"),
    ("ND", "North Dakota"),
    ("MP", "Northern Mariana Islands"),
    ("OH", "Ohio"),
    ("OK", "Oklahoma"),
    ("OR", "Oregon"),
    ("PA", "Pennsylvania"),
    ("PR", "Puerto Rico"),
    ("RI", "Rhode Island"),
    ("SC", "South Carolina"),
    ("SD", "South Dakota"),
    ("TN", "Tennessee"),
    ("TX", "Texas"),
    ("VI", "US Virgin Islands"),
    ("UM", "United States Minor Outlying Islands"),
    ("UT", "Utah"),
    ("VT", "Vermont"),
    ("VA", "Virginia"),
    ("WA", "Washington"),
    ("WV", "West Virginia"),
    ("WI", "Wisconsin"),
    ("WY", "Wyoming"),
];

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SubdivisionCode(String);

impl SubdivisionCode {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn united_states() -> impl Iterator<Item = (Self, &'static str)> {
        UNITED_STATES_SUBDIVISIONS
            .iter()
            .map(|(code, name)| (Self((*code).to_owned()), *name))
    }
}

impl std::fmt::Display for SubdivisionCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Region {
    country: CountryCode,
    subdivision: Option<SubdivisionCode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionProblem {
    Country(CountryCodeProblem),
    MissingSubdivision,
    UnknownSubdivision,
    MalformedSubdivision,
}

impl Problem for RegionProblem {
    fn message(&self) -> String {
        match self {
            Self::Country(problem) => problem.message(),
            Self::MissingSubdivision => "Choose a state.".to_owned(),
            Self::UnknownSubdivision => "Choose a state from the list.".to_owned(),
            Self::MalformedSubdivision => format!(
                "Use the ISO 3166-2 subdivision code: up to {MAXIMUM_SUBDIVISION_LENGTH} letters or digits, for example ON for Ontario."
            ),
        }
    }
}

impl std::fmt::Display for RegionProblem {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message())
    }
}

impl std::error::Error for RegionProblem {}

fn united_states_subdivision_name(code: &str) -> Option<&'static str> {
    UNITED_STATES_SUBDIVISIONS
        .iter()
        .find(|(candidate, _)| *candidate == code)
        .map(|(_, name)| *name)
}

fn is_subdivision_shaped(code: &str) -> bool {
    code.chars().count() <= MAXIMUM_SUBDIVISION_LENGTH
        && code
            .chars()
            .all(|character| character.is_ascii_uppercase() || character.is_ascii_digit())
}

fn united_states_subdivision(code: String) -> Result<Option<SubdivisionCode>, RegionProblem> {
    if code.is_empty() {
        return Err(RegionProblem::MissingSubdivision);
    }
    united_states_subdivision_name(&code)
        .map(|_| Some(SubdivisionCode(code)))
        .ok_or(RegionProblem::UnknownSubdivision)
}

fn other_subdivision(code: String) -> Result<Option<SubdivisionCode>, RegionProblem> {
    if code.is_empty() {
        return Ok(None);
    }
    if is_subdivision_shaped(&code) {
        Ok(Some(SubdivisionCode(code)))
    } else {
        Err(RegionProblem::MalformedSubdivision)
    }
}

impl Region {
    pub fn parse(country: &str, subdivision: &str) -> Result<Self, RegionProblem> {
        let country = CountryCode::parse(country).map_err(RegionProblem::Country)?;
        let code = subdivision.trim().to_uppercase();
        let subdivision = if country.as_str() == UNITED_STATES {
            united_states_subdivision(code)?
        } else {
            other_subdivision(code)?
        };
        Ok(Self {
            country,
            subdivision,
        })
    }

    #[must_use]
    pub fn country(&self) -> &CountryCode {
        &self.country
    }

    #[must_use]
    pub fn subdivision(&self) -> Option<&SubdivisionCode> {
        self.subdivision.as_ref()
    }

    #[must_use]
    pub fn subdivision_name(&self) -> Option<&'static str> {
        let subdivision = self.subdivision.as_ref()?;
        (self.country.as_str() == UNITED_STATES)
            .then(|| united_states_subdivision_name(subdivision.as_str()))
            .flatten()
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct TypedRegion {
    country: String,
    subdivision: Option<String>,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Region {
    fn serialize<Output: serde::Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        TypedRegion {
            country: self.country.to_string(),
            subdivision: self.subdivision.as_ref().map(ToString::to_string),
        }
        .serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'source> serde::Deserialize<'source> for Region {
    fn deserialize<Input: serde::Deserializer<'source>>(
        deserializer: Input,
    ) -> Result<Self, Input::Error> {
        let typed = TypedRegion::deserialize(deserializer)?;
        Self::parse(
            &typed.country,
            typed.subdivision.as_deref().unwrap_or_default(),
        )
        .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holds_every_united_states_subdivision_once() {
        let mut codes: Vec<&str> = UNITED_STATES_SUBDIVISIONS
            .iter()
            .map(|(code, _)| *code)
            .collect();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), 57);
        assert_eq!(SubdivisionCode::united_states().count(), 57);
    }

    #[test]
    fn requires_a_listed_state_in_the_united_states() {
        let florida = Region::parse("us", " fl ").unwrap();
        assert_eq!(florida.subdivision_name(), Some("Florida"));
        assert_eq!(
            Region::parse("US", ""),
            Err(RegionProblem::MissingSubdivision)
        );
        assert_eq!(
            Region::parse("US", "MX"),
            Err(RegionProblem::UnknownSubdivision)
        );
    }

    #[test]
    fn accepts_any_short_subdivision_code_elsewhere() {
        Region::parse("CA", "ON").unwrap();
        Region::parse("TR", "34").unwrap();
        assert_eq!(Region::parse("TR", "").unwrap().subdivision(), None);
        assert_eq!(
            Region::parse("IN", "MAHA"),
            Err(RegionProblem::MalformedSubdivision)
        );
    }

    #[test]
    fn reports_an_unknown_country_first() {
        assert_eq!(
            Region::parse("EL", "ATT"),
            Err(RegionProblem::Country(CountryCodeProblem::Unknown))
        );
    }
}
