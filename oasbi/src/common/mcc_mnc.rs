use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use super::{error::ConversionError, impl_try_from_strings, display_for_newtype};

/// Mobile Country Code part of the PLMN, comprising 3 digits, as defined in
/// clause 9.3.3.5 of 3GPP TS 38.413.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "Mobile Country Code part of the PLMN, comprising 3
/// digits, as defined in clause 9.3.3.5 of 3GPP TS 38.413. \n",
///  "type": "string",
///  "pattern": "^\\d{3}$"
/// }
/// ```
/// </details>
#[derive(
	Copy,
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	Default,
	SerializeDisplay,
	DeserializeFromStr,
)]
pub struct Mcc(u16); // 3 digits can fit in a u16.

impl TryFrom<u16> for Mcc {
	type Error = ConversionError;
	fn try_from(value: u16) -> Result<Self, Self::Error> {
		if value > 99 && value < 1000 {
			Ok(Mcc(value))
		} else {
			Err("Mcc must be a 3 digit (100 to 999)")?
		}
	}
}

/// Implements `FromStr` for `Mcc`.
impl FromStr for Mcc {
	type Err = ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		u16::from_str_radix(value, 10)
			.map_err(|e| format!("Mcc Parsing Error: {}", e.to_string()))?
			.try_into()
	}
}

impl_try_from_strings!(Mcc);
display_for_newtype!(Mcc);

#[derive(
	Copy,
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	Default,
	SerializeDisplay,
	DeserializeFromStr,
)]
pub struct Mnc(u16); // 2 or 3 digits can fit in a u16.


impl TryFrom<u16> for Mnc {
	type Error = ConversionError;
	fn try_from(value: u16) -> Result<Self, Self::Error> {
		if value > 9 && value < 1000 {
			Ok(Mnc(value))
		} else {
			Err("Mnc must be a 2-3 digit (10 to 999)")?
		}
	}
}

/// Implements `FromStr` for `Mnc`.
impl FromStr for Mnc {
	type Err = ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		u16::from_str_radix(value, 10)
			.map_err(|e| format!("Mnc Parsing Error: {}", e.to_string()))?
			.try_into()
	}
}

impl_try_from_strings!(Mnc);
display_for_newtype!(Mnc);

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json;

	#[test]
	fn test_mcc_valid() {
		let mcc = Mcc::from_str("310").unwrap();
		assert_eq!(mcc.to_string(), "310");
	}

	#[test]
	fn test_mcc_invalid() {
		assert!(Mcc::from_str("31").is_err()); // Too short
		assert!(Mcc::from_str("3100").is_err()); // Too long
		assert!(Mcc::from_str("ABC").is_err()); // Non-numeric
	}

	#[test]
	fn test_mcc_serde() {
		let mcc = Mcc::from_str("310").unwrap();
		let serialized = serde_json::to_string(&mcc).unwrap();
		assert_eq!(serialized, "\"310\"");

		let deserialized: Mcc = serde_json::from_str(&serialized).unwrap();
		assert_eq!(mcc, deserialized);
	}

	#[test]
	fn test_mnc_valid() {
		let mnc_2_digits = Mnc::from_str("10").unwrap();
		assert_eq!(mnc_2_digits.to_string(), "10");

		let mnc_3_digits = Mnc::from_str("123").unwrap();
		assert_eq!(mnc_3_digits.to_string(), "123");
	}

	#[test]
	fn test_mnc_invalid() {
		assert!(Mnc::from_str("1").is_err()); // Too short
		assert!(Mnc::from_str("1234").is_err()); // Too long
		assert!(Mnc::from_str("AB").is_err()); // Non-numeric
	}

	#[test]
	fn test_mnc_serde() {
		let mnc = Mnc::from_str("123").unwrap();
		let serialized = serde_json::to_string(&mnc).unwrap();
		assert_eq!(serialized, "\"123\"");

		let deserialized: Mnc = serde_json::from_str(&serialized).unwrap();
		assert_eq!(mnc, deserialized);
	}
}
