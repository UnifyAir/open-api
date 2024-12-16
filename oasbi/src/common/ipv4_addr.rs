use std::{convert::TryFrom, net::Ipv4Addr as StdIpv4Addr, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ipv4Addr(StdIpv4Addr);

impl Default for Ipv4Addr {
	fn default() -> Self {
		Self(StdIpv4Addr::LOCALHOST)
	}
}

/// Deref implementation to access the inner `std::net::Ipv4Addr`.
impl Deref for Ipv4Addr {
	type Target = StdIpv4Addr;
	fn deref(&self) -> &StdIpv4Addr {
		&self.0
	}
}

/// From implementation to convert `Ipv4Addr` into `String`.
impl From<Ipv4Addr> for String {
	fn from(value: Ipv4Addr) -> Self {
		value.0.to_string()
	}
}

/// From implementation to create an `Ipv4Addr` from a reference to another.
impl From<&Ipv4Addr> for Ipv4Addr {
	fn from(value: &Ipv4Addr) -> Self {
		value.clone()
	}
}

impl From<StdIpv4Addr> for Ipv4Addr {
	fn from(value: StdIpv4Addr) -> Self {
		Self(value)
	}
}

/// `FromStr` implementation with validation.
impl FromStr for Ipv4Addr {
	type Err = super::error::ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		// Validate and parse using `std::net::Ipv4Addr`.
		let ip = value
			.parse::<StdIpv4Addr>()
			.map_err(|_| super::error::ConversionError::from("Invalid IPv4 address format"))?;
		Ok(Self(ip))
	}
}

/// TryFrom implementations for &str and String.
impl TryFrom<&str> for Ipv4Addr {
	type Error = super::error::ConversionError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
	}
}

impl TryFrom<String> for Ipv4Addr {
	type Error = super::error::ConversionError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.parse()
	}
}

#[cfg(test)]
mod serde_tests {
	use std::convert::TryInto;

	use serde_json;

	use super::*;

	#[test]
	fn test_serialize_valid_ipv4() {
		let ip = "192.168.1.1".parse::<Ipv4Addr>().unwrap();
		let serialized = serde_json::to_string(&ip).unwrap();
		assert_eq!(serialized, "\"192.168.1.1\"");
	}

	#[test]
	fn test_deserialize_valid_ipv4() {
		let json = "\"10.0.0.1\"";
		let deserialized: Ipv4Addr = serde_json::from_str(json).unwrap();
		assert_eq!(deserialized.to_string(), "10.0.0.1");
	}

	#[test]
	fn test_serialize_and_deserialize_round_trip() {
		let original_ip = "172.16.0.1".parse::<Ipv4Addr>().unwrap();
		let serialized = serde_json::to_string(&original_ip).unwrap();
		let deserialized: Ipv4Addr = serde_json::from_str(&serialized).unwrap();
		assert_eq!(original_ip, deserialized);
	}

	#[test]
	fn test_deserialize_invalid_ipv4() {
		// Invalid IP format
		let invalid_json = "\"999.999.999.999\"";
		let result: Result<Ipv4Addr, _> = serde_json::from_str(invalid_json);
		assert!(result.is_err());

		// Not an IP at all
		let invalid_json = "\"not-an-ip\"";
		let result: Result<Ipv4Addr, _> = serde_json::from_str(invalid_json);
		assert!(result.is_err());
	}

	#[test]
	fn test_serialize_deserialize_with_nested_struct() {
		#[derive(Serialize, Deserialize, Debug, PartialEq)]
		struct NetworkConfig {
			gateway: Ipv4Addr,
			subnet: Ipv4Addr,
		}

		let config = NetworkConfig {
			gateway: "192.168.1.1".parse().unwrap(),
			subnet: "255.255.255.0".parse().unwrap(),
		};

		let serialized = serde_json::to_string(&config).unwrap();
		assert_eq!(
			serialized,
			"{\"gateway\":\"192.168.1.1\",\"subnet\":\"255.255.255.0\"}"
		);

		let deserialized: NetworkConfig = serde_json::from_str(&serialized).unwrap();
		assert_eq!(config, deserialized);
	}

	#[test]
	fn test_deserialize_invalid_format() {
		// Completely invalid JSON
		let invalid_json = "{invalid json}";
		let result: Result<Ipv4Addr, _> = serde_json::from_str(invalid_json);
		assert!(result.is_err());
	}

	#[test]
	fn test_valid_ipv4_parsing() {
		let valid_ip = "192.168.1.1".parse::<Ipv4Addr>();
		assert!(valid_ip.is_ok());
		assert_eq!(valid_ip.unwrap().to_string(), "192.168.1.1");
	}

	#[test]
	fn test_invalid_ipv4_parsing() {
		let invalid_ip = "999.999.999.999".parse::<Ipv4Addr>();
		assert!(invalid_ip.is_err());

		let invalid_ip2 = "abcd".parse::<Ipv4Addr>();
		assert!(invalid_ip2.is_err());

		let invalid_ip3 = "".parse::<Ipv4Addr>();
		assert!(invalid_ip3.is_err());
	}

	#[test]
	fn test_try_from_string() {
		let valid_ip: Result<Ipv4Addr, _> = String::from("10.0.0.1").try_into();
		assert!(valid_ip.is_ok());
		assert_eq!(valid_ip.unwrap().to_string(), "10.0.0.1");

		let invalid_ip: Result<Ipv4Addr, _> = String::from("256.256.256.256").try_into();
		assert!(invalid_ip.is_err());
	}

	#[test]
	fn test_try_from_str() {
		let valid_ip: Result<Ipv4Addr, _> = Ipv4Addr::try_from("172.16.0.1");
		assert!(valid_ip.is_ok());
		assert_eq!(valid_ip.unwrap().to_string(), "172.16.0.1");

		let invalid_ip: Result<Ipv4Addr, _> = Ipv4Addr::try_from("not-an-ip");
		assert!(invalid_ip.is_err());
	}

	#[test]
	fn test_deref() {
		let ip = "192.168.1.1".parse::<Ipv4Addr>().unwrap();
		assert_eq!(ip.to_string(), "192.168.1.1"); // `to_string()` from `std::net::Ipv4Addr`.
	}

	#[test]
	fn test_serialization() {
		let ip = "127.0.0.1".parse::<Ipv4Addr>().unwrap();
		let json = serde_json::to_string(&ip).unwrap();
		assert_eq!(json, "\"127.0.0.1\"");
	}

	#[test]
	fn test_deserialization() {
		let json = "\"192.168.1.1\"";
		let ip: Ipv4Addr = serde_json::from_str(json).unwrap();
		assert_eq!(ip.to_string(), "192.168.1.1");
	}

	#[test]
	fn test_deserialization_invalid() {
		let invalid_json = "\"999.999.999.999\"";
		let result: Result<Ipv4Addr, _> = serde_json::from_str(invalid_json);
		assert!(result.is_err());

		let invalid_format_json = "\"not-an-ip\"";
		let result: Result<Ipv4Addr, _> = serde_json::from_str(invalid_format_json);
		assert!(result.is_err());
	}

	#[test]
	fn test_clone_and_eq() {
		let ip1 = "10.0.0.1".parse::<Ipv4Addr>().unwrap();
		let ip2 = ip1.clone();
		assert_eq!(ip1, ip2);
	}

	#[test]
	fn test_ordering() {
		let ip1 = "10.0.0.1".parse::<Ipv4Addr>().unwrap();
		let ip2 = "10.0.0.2".parse::<Ipv4Addr>().unwrap();
		assert!(ip1 < ip2);
	}

	#[test]
	fn test_hash() {
		use std::collections::HashSet;
		let mut set = HashSet::new();

		let ip1 = "10.0.0.1".parse::<Ipv4Addr>().unwrap();
		let ip2 = "10.0.0.2".parse::<Ipv4Addr>().unwrap();

		set.insert(ip1.clone());
		assert!(set.contains(&ip1));
		assert!(!set.contains(&ip2));

		set.insert(ip2.clone());
		assert!(set.contains(&ip2));
	}
}
