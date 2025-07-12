use std::{convert::TryFrom, net::Ipv6Addr as StdIpv6Addr, ops::Deref, str::FromStr};

/// String identifying an IPv6 address formatted according to clause 4 of
/// RFC5952. The mixed IPv4 IPv6 notation according to clause 5 of RFC5952
/// shall not be used.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "String identifying an IPv6 address formatted according
/// to clause 4 of RFC5952. The mixed IPv4 IPv6 notation according to clause
/// 5 of RFC5952 shall not be used.\n",
///  "examples": [
///    "2001:db8:85a3::8a2e:370:7334"
///  ],
///  "type": "string",
///  "pattern":
/// "(?=.*^((:|(0?|([1-9a-f][0-9a-f]{0,3}))):)((0?|([1-9a-f][0-9a-f]{0,3})):
/// ){0,6}(:|(0?|([1-9a-f][0-9a-f]{0,3})))$)(?=.*^((([^:]+:){7}([^:
/// ]+))|((([^:]+:)*[^:]+)?::(([^:]+:)*[^:]+)?))$)"
/// }
/// ```
/// </details>
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ipv6Addr(StdIpv6Addr);

impl Default for Ipv6Addr {
	fn default() -> Self {
		Self(StdIpv6Addr::LOCALHOST)
	}
}

/// Deref implementation to access the inner `std::net::Ipv6Addr`.
impl Deref for Ipv6Addr {
	type Target = StdIpv6Addr;
	fn deref(&self) -> &StdIpv6Addr {
		&self.0
	}
}

/// From implementation to convert `Ipv6Addr` into `String`.
impl From<Ipv6Addr> for String {
	fn from(value: Ipv6Addr) -> Self {
		value.0.to_string()
	}
}

/// From implementation to create an `Ipv6Addr` from a reference to another.
impl From<&Ipv6Addr> for Ipv6Addr {
	fn from(value: &Ipv6Addr) -> Self {
		value.clone()
	}
}

impl From<StdIpv6Addr> for Ipv6Addr {
	fn from(value: StdIpv6Addr) -> Self {
		Ipv6Addr(value)
	}
}

impl From<&StdIpv6Addr> for Ipv6Addr {
	fn from(value: &StdIpv6Addr) -> Self {
		Ipv6Addr(*value)
	}
}

/// `FromStr` implementation with validation.
impl FromStr for Ipv6Addr {
	type Err = super::error::ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		// Validate and parse using `std::net::Ipv6Addr`.
		let ip = value
			.parse::<StdIpv6Addr>()
			.map_err(|_| super::error::ConversionError::from("Invalid IPv6 address format"))?;
		// Ensure the address is formatted according to RFC 5952 (clause 4).
		let formatted = ip.to_string();
		if formatted != value.to_lowercase() {
			return Err(super::error::ConversionError::from(
				"IPv6 address does not match RFC 5952 format",
			));
		}
		Ok(Self(ip))
	}
}

/// TryFrom implementations for &str and String.
impl TryFrom<&str> for Ipv6Addr {
	type Error = super::error::ConversionError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
	}
}

impl TryFrom<String> for Ipv6Addr {
	type Error = super::error::ConversionError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.parse()
	}
}

#[cfg(test)]
mod tests {
	use std::convert::TryInto;

	use serde_json;

	use super::*;

	#[test]
	fn test_valid_ipv6_parsing() {
		let valid_ip = "2001:db8:85a3::8a2e:370:7334".parse::<Ipv6Addr>();
		assert!(valid_ip.is_ok());
		assert_eq!(
			valid_ip.unwrap().to_string(),
			"2001:db8:85a3::8a2e:370:7334"
		);
	}

	#[test]
	fn test_invalid_ipv6_parsing() {
		let invalid_ip = "2001:db8::85a3::7334".parse::<Ipv6Addr>();
		assert!(invalid_ip.is_err());

		let invalid_ip2 = "abcd:1234".parse::<Ipv6Addr>();
		assert!(invalid_ip2.is_err());
	}

	#[test]
	fn test_non_canonical_ipv6() {
		// Address is valid but not in canonical RFC 5952 format
		let non_canonical_ip = "2001:0db8:85a3::8a2e:0370:7334".parse::<Ipv6Addr>();
		assert!(non_canonical_ip.is_err());
	}

	#[test]
	fn test_try_from_string() {
		let valid_ip: Result<Ipv6Addr, _> = String::from("2001:db8:85a3::8a2e:370:7334").try_into();
		assert!(valid_ip.is_ok());
		assert_eq!(
			valid_ip.unwrap().to_string(),
			"2001:db8:85a3::8a2e:370:7334"
		);

		let invalid_ip: Result<Ipv6Addr, _> = String::from("::85a3::8a2e").try_into();
		assert!(invalid_ip.is_err());
	}

	#[test]
	fn test_serialize_deserialize_ipv6() {
		let ip = "2001:db8:85a3::8a2e:370:7334".parse::<Ipv6Addr>().unwrap();

		// Serialize
		let json = serde_json::to_string(&ip).unwrap();
		assert_eq!(json, "\"2001:db8:85a3::8a2e:370:7334\"");

		// Deserialize
		let deserialized: Ipv6Addr = serde_json::from_str(&json).unwrap();
		assert_eq!(ip, deserialized);
	}

	#[test]
	fn test_serialize_deserialize_invalid_ipv6() {
		let invalid_json = "\"2001:db8::85a3::7334\"";
		let result: Result<Ipv6Addr, _> = serde_json::from_str(invalid_json);
		assert!(result.is_err());
	}
}
