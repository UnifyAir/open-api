use std::{fmt::Display, str::FromStr};

use ascii::AsciiString;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use super::{
	deref_for_newtype,
	display_for_newtype,
	error::ConversionError,
	impl_try_from_strings,
};

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
pub struct Mcc(AsciiString); // 3 digits can fit in a u16.

fn convert_to_ascii_string(value: u16) -> AsciiString {
	let bytes = value.to_string().into_bytes();
	// SAFETY: Converting a number to string will only produce ASCII digits (0-9)
	unsafe { AsciiString::from_ascii_unchecked(bytes) }
}

impl TryFrom<u16> for Mcc {
	type Error = ConversionError;
	fn try_from(value: u16) -> Result<Self, Self::Error> {
		if value > 99 && value < 1000 {
			let int_ascii = convert_to_ascii_string(value);
			Ok(Mcc(int_ascii))
		} else {
			Err("Mcc must be a 3 digit (100 to 999)")?
		}
	}
}

/// Implements `FromStr` for `Mcc`.
impl FromStr for Mcc {
	type Err = ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		if value.len() != 3 {
			return Err("Mcc must be exactly 3 digits".into());
		}
		if !value.chars().all(|c| c.is_ascii_digit()) {
			return Err("Mcc must contain only digits".into());
		}
		// SAFETY: `value` is a valid ASCII string as all characters are digits
		let mcc_ascii = unsafe { AsciiString::from_ascii_unchecked(value.as_bytes()) };
		Ok(Mcc(mcc_ascii))
	}
}

impl Mcc {
	pub unsafe fn from_str_unchecked(value: String) -> Self {
		let ascii = AsciiString::from_ascii_unchecked(value.into_bytes());
		Mcc(ascii)
	}
}

impl_try_from_strings!(Mcc);
display_for_newtype!(Mcc);
deref_for_newtype!(Mcc, AsciiString);

#[derive(
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
pub struct Mnc(AsciiString); // 2 or 3 digits can fit in a u16.

impl TryFrom<u16> for Mnc {
	type Error = ConversionError;
	fn try_from(value: u16) -> Result<Self, Self::Error> {
		if value > 9 && value < 1000 {
			let int_ascii = convert_to_ascii_string(value);
			Ok(Mnc(int_ascii))
		} else {
			Err("Mnc must be a 2-3 digit (10 to 999)")?
		}
	}
}

/// Implements `FromStr` for `Mnc`.
impl FromStr for Mnc {
	type Err = ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		if value.len() < 2 || value.len() > 3 {
			return Err("Mnc must be 2 or 3 digits".into());
		}
		if !value.chars().all(|c| c.is_ascii_digit()) {
			return Err("Mnc must contain only digits".into());
		}
		// SAFETY: `value` is a valid ASCII string as all characters are digits
		let ascii = unsafe { AsciiString::from_ascii_unchecked(value.as_bytes()) };
		Ok(Mnc(ascii))
	}
}

impl Mnc {
	pub unsafe fn from_str_unchecked(value: String) -> Self {
		let ascii = AsciiString::from_ascii_unchecked(value.into_bytes());
		Mnc(ascii)
	}
}

impl_try_from_strings!(Mnc);
display_for_newtype!(Mnc);
deref_for_newtype!(Mnc, AsciiString);

#[cfg(test)]
mod tests {
	use serde_json;

	use super::*;

	fn assert_mcc_valid(input: &str) {
		let mcc = Mcc::from_str(input).unwrap();
		assert_eq!(mcc.to_string(), input);
	}

	fn assert_mnc_valid(input: &str) {
		let mnc = Mnc::from_str(input).unwrap();
		assert_eq!(mnc.to_string(), input);
	}

	#[test]
	fn test_mcc_valid() {
		assert_mcc_valid("310");
		assert_mcc_valid("001");
		assert_mcc_valid("010");
		assert_mcc_valid("100");
		assert_mcc_valid("999");
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
		assert_mnc_valid("10");
		assert_mnc_valid("123");
		assert_mnc_valid("001");
		assert_mnc_valid("010");
		assert_mnc_valid("01");
		assert_mnc_valid("999");
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
