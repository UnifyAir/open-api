use std::{convert::TryFrom, ops::Deref, str::FromStr};

use fqdn::FQDN;
/// Fully Qualified Domain Name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "Fully Qualified Domain Name",
///  "type": "string",
///  "maxLength": 253,
///  "minLength": 4,
///  "pattern":
/// "^([0-9A-Za-z]([-0-9A-Za-z]{0,61}[0-9A-Za-z])?\\.)+[A-Za-z]{2,63}\\.?$"
/// }
/// ```
/// </details>
use serde::Deserialize;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Fqdn(FQDN);

/// Default implementation for Fqdn.
impl Default for Fqdn {
	fn default() -> Self {
		// Example default FQDN, replace with your desired default if applicable.
		Self(FQDN::default())
	}
}

impl serde::Serialize for Fqdn {
	fn serialize<S>(
		&self,
		serializer: S,
	) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let fqdn_str = self.0.to_string();
		serde::Serializer::serialize_newtype_struct(serializer, "Fqdn", &fqdn_str)
	}
}

/// Deref implementation to access the inner `fqdn::FQDN`.
impl Deref for Fqdn {
	type Target = FQDN;
	fn deref(&self) -> &FQDN {
		&self.0
	}
}

/// From implementation to convert `Fqdn` into `String`.
impl From<Fqdn> for String {
	fn from(value: Fqdn) -> Self {
		value.0.to_string()
	}
}

/// From implementation to create an `Fqdn` from a reference to another.
impl From<&Fqdn> for Fqdn {
	fn from(value: &Fqdn) -> Self {
		value.clone()
	}
}

/// `FromStr` implementation with validation.
impl FromStr for Fqdn {
	type Err = super::error::ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		if value.len() > 253 {
			return Err("FQDN is longer than 253 characters".into());
		}
		if value.len() < 4 {
			return Err("FQDN is shorter than 4 characters".into());
		}
		let fqdn = value
			.parse::<FQDN>()
			.map_err(|e| Self::Err::from(format!("Invalid FQDN format: {:?}", e.to_string())))?;
		Ok(Self(fqdn))
	}
}

/// TryFrom implementations for &str and String.
impl TryFrom<&str> for Fqdn {
	type Error = super::error::ConversionError;

	fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
		value.parse()
	}
}

impl TryFrom<String> for Fqdn {
	type Error = super::error::ConversionError;

	fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
		value.parse()
	}
}

/// Deserialization with validation using Serde.
impl<'de> Deserialize<'de> for Fqdn {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let value: String = String::deserialize(deserializer)?;
		value
			.parse()
			.map_err(|e: super::error::ConversionError| serde::de::Error::custom(e.to_string()))
	}
}

#[cfg(test)]
mod tests {
	use std::convert::TryInto;

	use serde_json;

	use super::*;

	#[test]
	fn test_valid_fqdn_parsing() {
		let valid_fqdn = "example.com".parse::<Fqdn>();
		assert_eq!(valid_fqdn.unwrap().to_string(), "example.com");
	}

	#[test]
	fn test_invalid_fqdn_parsing() {
		let invalid_fqdn = "invalid_domain".parse::<Fqdn>();
		assert!(invalid_fqdn.is_err());

		let invalid_fqdn2 = "com".parse::<Fqdn>();
		assert!(invalid_fqdn2.is_err());

		let invalid_fqdn3 = "a".repeat(254).parse::<Fqdn>();
		assert!(invalid_fqdn3.is_err());
	}

	#[test]
	fn test_fqdn_default() {
		let default_fqdn = Fqdn::default();
		assert_eq!(default_fqdn.to_string(), ".");
	}

	#[test]
	fn test_try_from_string() {
		let valid_fqdn: Result<Fqdn, _> = String::from("n3iwf.uniyfair.com").try_into();
		assert!(valid_fqdn.is_ok());
		assert_eq!(valid_fqdn.unwrap().to_string(), "n3iwf.uniyfair.com");

		let invalid_fqdn: Result<Fqdn, _> = String::from("invalid_domain").try_into();
		assert!(invalid_fqdn.is_err());
	}

	#[test]
	fn test_serialize_deserialize_fqdn() {
		let fqdn = "example.com".parse::<Fqdn>().unwrap();

		// Serialize
		let json = serde_json::to_string(&fqdn).unwrap();
		assert_eq!(json, "\"example.com\"");

		// Deserialize
		let deserialized: Fqdn = serde_json::from_str(&json).unwrap();
		assert_eq!(fqdn, deserialized);
	}

	#[test]
	fn test_serialize_deserialize_invalid_fqdn() {
		let invalid_json = "\"invalid_domain\"";
		let result: Result<Fqdn, _> = serde_json::from_str(invalid_json);
		assert!(result.is_err());
	}
}
