use std::{convert::TryFrom, ops::Deref, str::FromStr};

use ipnet::Ipv6Net;
/// String identifying an IPv6 address prefix formatted according to clause
/// 4 of RFC 5952. IPv6Prefix data type may contain an individual /128 IPv6
/// address.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "String identifying an IPv6 address prefix formatted
/// according to clause 4 of RFC 5952. IPv6Prefix data type may contain an
/// individual /128 IPv6 address.\n",
///  "examples": [
///    "2001:db8:abcd:12::0/64"
///  ],
///  "type": "string",
///  "pattern":
/// "(?=.*^((:|(0?|([1-9a-f][0-9a-f]{0,3}))):)((0?|([1-9a-f][0-9a-f]{0,3})):
/// ){0,6}(:|(0?|([1-9a-f][0-9a-f]{0,3})))(\\/
/// (([0-9])|([0-9]{2})|(1[0-1][0-9])|(12[0-8])))$)(?=.*^((([^:]+:){7}([^:
/// ]+))|((([^:]+:)*[^:]+)?::(([^:]+:)*[^:]+)?))(\\/.+)$)"
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
pub struct Ipv6Prefix(Ipv6Net);

/// Deref implementation to access the inner `ipnet::Ipv6Net`.
impl Deref for Ipv6Prefix {
	type Target = Ipv6Net;
	fn deref(&self) -> &Ipv6Net {
		&self.0
	}
}

/// From implementation to convert `Ipv6Prefix` into `String`.
impl From<Ipv6Prefix> for String {
	fn from(value: Ipv6Prefix) -> Self {
		value.0.to_string()
	}
}

/// From implementation to create an `Ipv6Prefix` from a reference to another.
impl From<&Ipv6Prefix> for Ipv6Prefix {
	fn from(value: &Ipv6Prefix) -> Self {
		value.clone()
	}
}

/// `FromStr` implementation with validation.
impl FromStr for Ipv6Prefix {
	type Err = super::error::ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let ipv6_net = value.parse::<Ipv6Net>().map_err(|e| {
			super::error::ConversionError::from(format!(
				"Invalid IPv6 prefix format: {}",
				e.to_string()
			))
		})?;
		Ok(Self(ipv6_net))
	}
}

/// TryFrom implementations for &str and String.
impl TryFrom<&str> for Ipv6Prefix {
	type Error = super::error::ConversionError;

	fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
		value.parse()
	}
}

impl TryFrom<String> for Ipv6Prefix {
	type Error = super::error::ConversionError;

	fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
		value.parse()
	}
}

///// Deserialization with validation using Serde.
// impl<'de> Deserialize<'de> for Ipv6Prefix {
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
//
#[cfg(test)]
mod tests {
	use std::convert::TryInto;

	use serde_json;

	use super::*;

	#[test]
	fn test_valid_ipv6_prefix_parsing() {
		let valid_prefix = "2001:db8::/32".parse::<Ipv6Prefix>();
		assert!(valid_prefix.is_ok());
		assert_eq!(valid_prefix.unwrap().to_string(), "2001:db8::/32");
	}

	#[test]
	fn test_invalid_ipv6_prefix_parsing() {
		let invalid_prefix = "2001:db8::/129".parse::<Ipv6Prefix>(); // Invalid prefix length
		assert!(invalid_prefix.is_err());

		let invalid_prefix2 = "invalid_prefix".parse::<Ipv6Prefix>();
		assert!(invalid_prefix2.is_err());
	}

	#[test]
	fn test_ipv6_prefix_default() {
		let default_prefix = Ipv6Prefix::default();
		assert_eq!(default_prefix.to_string(), "::/0");
	}

	#[test]
	fn test_try_from_string() {
		let valid_prefix: Result<Ipv6Prefix, _> = String::from("fd00::/8").try_into();
		assert!(valid_prefix.is_ok());
		assert_eq!(valid_prefix.unwrap().to_string(), "fd00::/8");

		let invalid_prefix: Result<Ipv6Prefix, _> = String::from(":::/64").try_into();
		assert!(invalid_prefix.is_err());
	}

	#[test]
	fn test_serialize_deserialize_ipv6_prefix() {
		let prefix = "2001:db8::/32".parse::<Ipv6Prefix>().unwrap();

		// Serialize
		let json = serde_json::to_string(&prefix).unwrap();
		assert_eq!(json, "\"2001:db8::/32\"");

		// Deserialize
		let deserialized: Ipv6Prefix = serde_json::from_str(&json).unwrap();
		assert_eq!(prefix, deserialized);
	}

	#[test]
	fn test_empty_ipv6_prefix() {
		let invalid_json = "\"2001:db8::/129\"";
		let result: Result<Ipv6Prefix, _> = serde_json::from_str(invalid_json);
		assert!(result.is_err());
	}

	#[test]
	fn test_serialize_deserialize_invalid_ipv6_prefix() {
		let invalid_json = "\"2001:db8::/129\"";
		let result: Result<Ipv6Prefix, _> = serde_json::from_str(invalid_json);
		assert!(result.is_err());
	}
}
