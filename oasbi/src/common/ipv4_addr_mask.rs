use std::{convert::TryFrom, ops::Deref, str::FromStr};

use ipnet::Ipv4Net;
/// "String identifying a IPv4 address mask formatted in the 'dotted
/// decimal' notation as defined in RFC 1166."
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "\"String identifying a IPv4 address mask formatted in
/// the 'dotted decimal' notation as defined in RFC 1166.\"\n",
///  "examples": [
///    "198.51.0.0/16"
///  ],
///  "type": "string",
///  "pattern":
/// "^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.
/// ){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])(\\/
/// ([0-9]|[1-2][0-9]|3[0-2]))$"
/// }
/// ```
/// </details>
use serde::{Deserialize, Serialize};

#[derive(
	Serialize,
	Deserialize,
	Clone,
	Debug,
	Eq,
	Hash,
	Ord,
	PartialEq,
	PartialOrd,
	smart_default::SmartDefault,
)]
pub struct Ipv4AddrMask(Ipv4Net);

/// Deref implementation to access the inner `ipnet::Ipv4Net`.
impl Deref for Ipv4AddrMask {
	type Target = Ipv4Net;
	fn deref(&self) -> &Ipv4Net {
		&self.0
	}
}

/// From implementation to convert `Ipv4AddrMask` into `String`.
impl From<Ipv4AddrMask> for String {
	fn from(value: Ipv4AddrMask) -> Self {
		value.0.to_string()
	}
}

/// From implementation to create an `Ipv4AddrMask` from a reference to another.
impl From<&Ipv4AddrMask> for Ipv4AddrMask {
	fn from(value: &Ipv4AddrMask) -> Self {
		value.clone()
	}
}

/// `FromStr` implementation with validation.
impl FromStr for Ipv4AddrMask {
	type Err = super::error::ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let net = value.parse::<Ipv4Net>().map_err(|e| {
			super::error::ConversionError::from(format!(
				"Invalid IPv4 address/mask format: {:?}",
				e.to_string()
			))
		})?;
		Ok(Self(net))
	}
}

/// TryFrom implementations for &str and String.
impl TryFrom<&str> for Ipv4AddrMask {
	type Error = super::error::ConversionError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
	}
}

impl TryFrom<String> for Ipv4AddrMask {
	type Error = super::error::ConversionError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.parse()
	}
}

impl ::std::convert::TryFrom<&String> for Ipv4AddrMask {
	type Error = super::error::ConversionError;
	fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
		value.parse()
	}
}

///// Serialize implementation for Ipv4AddrMask.
// impl Serialize for Ipv4AddrMask {
// 	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// 										  where
// 											  S: serde::Serializer,
// 	{
// 		serializer.serialize_str(&self.0.to_string())
// 	}
//}
//
///// Deserialization with validation using Serde.
// impl<'de> Deserialize<'de> for Ipv4AddrMask {
// 	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
// 									   where
// 										   D: serde::Deserializer<'de>,
// 	{
// 		let value: String = String::deserialize(deserializer)?;
// 		value.parse().map_err(|e: super::error::ConversionError| {
// 			serde::de::Error::custom(e.to_string())
// 		})
// 	}
//}

#[cfg(test)]
mod tests {
	use std::convert::TryInto;

	use serde_json;

	use super::*;

	#[test]
	fn test_valid_ipv4addrmask_parsing() {
		let valid_net = "192.168.1.0/24".parse::<Ipv4AddrMask>();
		assert!(valid_net.is_ok());
		assert_eq!(valid_net.unwrap().to_string(), "192.168.1.0/24");
	}

	#[test]
	fn test_invalid_ipv4addrmask_parsing() {
		let invalid_net = "192.168.1.0/33".parse::<Ipv4AddrMask>();
		assert!(invalid_net.is_err());

		let invalid_net2 = "192.168.1.256/24".parse::<Ipv4AddrMask>();
		assert!(invalid_net2.is_err());

		let invalid_net3 = "not-an-ip/24".parse::<Ipv4AddrMask>();
		assert!(invalid_net3.is_err());
	}

	#[test]
	fn test_default_ipv4addrmask() {
		let default_net = Ipv4AddrMask::default();
		assert_eq!(default_net.to_string(), "0.0.0.0/0");
	}

	#[test]
	fn test_try_from_string() {
		let valid_net: Result<Ipv4AddrMask, _> = String::from("10.0.0.0/8").try_into();
		assert!(valid_net.is_ok());
		assert_eq!(valid_net.unwrap().to_string(), "10.0.0.0/8");

		let invalid_net: Result<Ipv4AddrMask, _> = String::from("10.0.0.0/33").try_into();
		assert!(invalid_net.is_err());
	}

	#[test]
	fn test_serialize_ipv4addrmask() {
		let net = "192.168.1.0/24".parse::<Ipv4AddrMask>().unwrap();
		let serialized = serde_json::to_string(&net).unwrap();
		assert_eq!(serialized, "\"192.168.1.0/24\"");
	}

	#[test]
	fn test_deserialize_ipv4addrmask() {
		let json = "\"192.168.1.0/24\"";
		let deserialized: Ipv4AddrMask = serde_json::from_str(json).unwrap();
		assert_eq!(deserialized.to_string(), "192.168.1.0/24");
	}

	#[test]
	fn test_serialize_deserialize_invalid_ipv4addrmask() {
		let invalid_json = "\"192.168.1.0/33\"";
		let result: Result<Ipv4AddrMask, _> = serde_json::from_str(invalid_json);
		assert!(result.is_err());
	}
}
