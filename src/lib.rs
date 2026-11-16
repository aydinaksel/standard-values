mod calling_code;
mod container_number;
mod country;
mod currency_code;
mod domain_name;
mod email_address;
mod phone_number;
mod problem;
mod region;

pub use container_number::{ContainerNumber, ContainerNumberProblem};
pub use country::{CountryCode, CountryCodeProblem};
pub use currency_code::{CurrencyCode, CurrencyCodeProblem};
pub use domain_name::{DomainName, DomainNameProblem};
pub use email_address::{EmailAddress, EmailAddressProblem};
pub use phone_number::{PhoneNumber, PhoneNumberProblem};
pub use problem::Problem;
pub use region::{Region, RegionProblem, SubdivisionCode};

macro_rules! text_value {
    ($value:ident, $problem:ident) => {
        impl $value {
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $value {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl AsRef<str> for $value {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl From<$value> for String {
            fn from(value: $value) -> Self {
                value.0
            }
        }

        impl std::str::FromStr for $value {
            type Err = $problem;

            fn from_str(typed: &str) -> Result<Self, Self::Err> {
                Self::parse(typed)
            }
        }

        impl TryFrom<&str> for $value {
            type Error = $problem;

            fn try_from(typed: &str) -> Result<Self, Self::Error> {
                Self::parse(typed)
            }
        }

        impl TryFrom<String> for $value {
            type Error = $problem;

            fn try_from(typed: String) -> Result<Self, Self::Error> {
                Self::parse(&typed)
            }
        }

        impl std::fmt::Display for $problem {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(&crate::Problem::message(self))
            }
        }

        impl std::error::Error for $problem {}

        #[cfg(feature = "serde")]
        impl serde::Serialize for $value {
            fn serialize<Output: serde::Serializer>(
                &self,
                serializer: Output,
            ) -> Result<Output::Ok, Output::Error> {
                serializer.serialize_str(&self.0)
            }
        }

        #[cfg(feature = "serde")]
        impl<'source> serde::Deserialize<'source> for $value {
            fn deserialize<Input: serde::Deserializer<'source>>(
                deserializer: Input,
            ) -> Result<Self, Input::Error> {
                let typed = <String as serde::Deserialize>::deserialize(deserializer)?;
                Self::parse(&typed).map_err(serde::de::Error::custom)
            }
        }
    };
}

pub(crate) use text_value;
