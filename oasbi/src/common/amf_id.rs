use std::{fmt, ops::Deref, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use super::error::ConversionError;

/// String identifying the AMF ID composed of AMF Region ID (8 bits), AMF
/// Set ID (10 bits) and AMF  Pointer (6 bits) as specified in clause 2.10.1
/// of 3GPP TS 23.003. It is encoded as a string of  6 hexadecimal
/// characters (i.e., 24 bits).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "String identifying the AMF ID composed of AMF Region ID
/// (8 bits), AMF Set ID (10 bits) and AMF  Pointer (6 bits) as specified in
/// clause 2.10.1 of 3GPP TS 23.003. It is encoded as a string of  6
/// hexadecimal characters (i.e., 24 bits). \n",
///  "type": "string",
///  "pattern": "^[A-Fa-f0-9]{6}$"
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
pub struct AmfId {
	pub region_id: AmfRegionId,
	pub pointer_id: u8, // 6 bits but fits in u8
	pub set_id: AmfSetId,
}

impl AmfId {
	/// Encodes the `AmfId` as a 6-character hexadecimal string.
	pub fn to_hex(&self) -> String {
		let combined = ((self.region_id.0 as u32) << 16)
			| ((self.set_id.0 as u32) << 6)
			| (self.pointer_id as u32);
		format!("{:06X}", combined)
	}

	/// Creates an `AmfId` from a 6-character hexadecimal string.
	pub fn from_hex(hex: &str) -> Result<Self, ConversionError> {
		let value =
			u32::from_str_radix(hex, 16).map_err(|_| "Failed to parse hexadecimal for AmfId")?;

		Self::try_from(value)
	}

}

impl TryFrom<u32> for AmfId {
	type Error = ConversionError;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		if value > 0xff_ffff {
			return Err("AmfId must be a 24-bit value (0 to 0xFF_FFFF)")?;
		}

		let region_id = ((value >> 16) & 0xFF) as u8;
		let set_id = ((value >> 6) & 0x3FF) as u16;
		let pointer_id = (value & 0x3F) as u8;

		Ok(Self {
			region_id: AmfRegionId(region_id),
			set_id: AmfSetId(set_id),
			pointer_id,
		})
	}
}

/// Implements `FromStr` for `AmfId` to parse from a hexadecimal string.
impl FromStr for AmfId {
	type Err = ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::from_hex(value)
	}
}

/// Implements `fmt::Display` for `AmfId` to format as a hexadecimal string.
impl fmt::Display for AmfId {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

/// String identifying the AMF Set ID (10 bits) as specified in clause
/// 2.10.1 of 3GPP TS 23.003.  It is encoded as a string of 3 hexadecimal
/// characters where the first character is limited to  values 0 to 3 (i.e.
/// 10 bits).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "String identifying the AMF Set ID (10 bits) as
/// specified in clause 2.10.1 of 3GPP TS 23.003.  It is encoded as a string
/// of 3 hexadecimal characters where the first character is limited to
/// values 0 to 3 (i.e. 10 bits).\n",
///  "type": "string",
///  "pattern": "^[0-3][A-Fa-f0-9]{2}$"
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
pub struct AmfSetId(u16); // 10 bits can fit in a u16.

impl AmfSetId {
	/// Encodes the `AmfSetId` as a 3-character hexadecimal string.
	pub fn to_hex(&self) -> String {
		format!("{:03X}", self.0)
	}

	/// Creates an `AmfSetId` from a 3-character hexadecimal string.
	pub fn from_hex(hex: &str) -> Result<Self, ConversionError> {
		let value =
			u16::from_str_radix(hex, 16).map_err(|_| "AmfSetId: Failed to parse hexadecimal")?;
		Self::try_from(value)
	}

}

impl TryFrom<u16> for AmfSetId {
	type Error = ConversionError;
	fn try_from(value: u16) -> Result<Self, Self::Error> {
		if value > 0x3FF {
			return Err("AMF Set ID must be a 10-bit value (0 to 0x3FF)")?;
		}

		Ok(Self(value))
	}
}

/// Implements `FromStr` for `AmfSetId`.
impl FromStr for AmfSetId {
	type Err = ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::from_hex(value)
	}
}

/// Implements `fmt::Display` for `AmfSetId`.
impl fmt::Display for AmfSetId {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

impl Deref for AmfSetId {
	type Target = u16;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

/// String identifying the AMF Region ID (8 bits) as specified in clause
/// 2.10.1 of 3GPP TS 23.003.  It is encoded as a string of 2 hexadecimal
/// characters
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/// {
///  "description": "String identifying the AMF Region ID (8 bits) as specified in clause
/// 2.10.1 of 3GPP TS 23.003.  It is encoded as a string of 2 hexadecimal
/// characters\n",
///  "type": "string",
///  "pattern": "^[A-Fa-f0-9]{2}$"
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
pub struct AmfRegionId(u8); // 8 bits can fit in a u8.

impl AmfRegionId {
	/// Encodes the `AmfRegionId` as a 2-character hexadecimal string.
	pub fn to_hex(&self) -> String {
		format!("{:02X}", self.0)
	}

	/// Creates an `AmfRegionId` from a 2-character hexadecimal string.
	pub fn from_hex(hex: &str) -> Result<Self, ConversionError> {
		let value = u8::from_str_radix(hex, 16).map_err(|_| "Failed to parse hexadecimal")?;
		Ok(Self(value))
	}

	pub fn new(value: u8) -> Self {
		Self(value)
	}
}

/// Implements `FromStr` for `AmfRegionId`.
impl FromStr for AmfRegionId {
	type Err = ConversionError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Self::from_hex(value)
	}
}

/// Implements `fmt::Display` for `AmfRegionId`.
impl fmt::Display for AmfRegionId {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

impl Deref for AmfRegionId {
	type Target = u8;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[cfg(test)]
mod tests {
	use serde::{Deserialize, Serialize};
	use serde_json;

	use super::*;

	#[test]
	fn test_amfid_serialize() {
		let amf_id = AmfId {
			region_id: AmfRegionId(0x12),
			set_id: AmfSetId(0x345),
			pointer_id: 0x3F,
		};
		let serialized = serde_json::to_string(&amf_id).unwrap();
		assert_eq!(serialized, "\"12D17F\"");
	}

	#[test]
	fn test_amfid_deserialize() {
		let json = "\"12D17F\"";
		let amf_id: AmfId = serde_json::from_str(json).unwrap();
		assert_eq!(amf_id.region_id.0, 0x12);
		assert_eq!(amf_id.set_id.0, 0x345);
		assert_eq!(amf_id.pointer_id, 0x3F);
	}

	#[test]
	fn test_amfid_invalid_deserialize() {
		let invalid_json = "\"ZZZZZZ\"";
		let result: Result<AmfId, _> = serde_json::from_str(invalid_json);
		assert!(result.is_err());
	}

	#[test]
	fn test_amfid_serialize_nested_struct() {
		#[derive(Serialize, Deserialize, Debug, PartialEq)]
		struct Config {
			amf_id: AmfId,
		}

		let config = Config {
			amf_id: AmfId {
				region_id: AmfRegionId(0x01),
				set_id: AmfSetId(0x1AA),
				pointer_id: 0x3C,
			},
		};

		let serialized = serde_json::to_string(&config).unwrap();
		assert_eq!(serialized, "{\"amf_id\":\"016ABC\"}");

		let deserialized: Config = serde_json::from_str(&serialized).unwrap();
		assert_eq!(config, deserialized);
	}

	#[test]
	fn test_amf_set_id_valid() {
		let amf_set_id = AmfSetId::from_str("1A3").unwrap();
		assert_eq!(amf_set_id.to_hex(), "1A3");
		assert_eq!(amf_set_id.to_string(), "1A3");
	}

	#[test]
	fn test_amf_set_id_invalid() {
		assert!(AmfSetId::from_str("4A3").is_err()); // First character > 3
		assert!(AmfSetId::from_str("1A3F").is_err()); // Too long
		assert!(AmfSetId::from_str("ZZZ").is_err()); // Invalid characters
	}

	#[test]
	fn test_amf_region_id_valid() {
		let amf_region_id = AmfRegionId::from_str("1F").unwrap();
		assert_eq!(amf_region_id.to_hex(), "1F");
		assert_eq!(amf_region_id.to_string(), "1F");
	}

	#[test]
	fn test_amf_region_id_invalid() {
		assert!(AmfRegionId::from_str("1F3").is_err()); // Too long
		assert!(AmfRegionId::from_str("ZZ").is_err()); // Invalid characters
	}

	#[test]
	fn test_amf_set_id_serde() {
		let amf_set_id = AmfSetId::from_str("1A3").unwrap();
		let serialized = serde_json::to_string(&amf_set_id).unwrap();
		assert_eq!(serialized, "\"1A3\"");

		let deserialized: AmfSetId = serde_json::from_str(&serialized).unwrap();
		assert_eq!(amf_set_id, deserialized);
	}

	#[test]
	fn test_amf_region_id_serde() {
		let amf_region_id = AmfRegionId::from_str("1F").unwrap();
		let serialized = serde_json::to_string(&amf_region_id).unwrap();
		assert_eq!(serialized, "\"1F\"");

		let deserialized: AmfRegionId = serde_json::from_str(&serialized).unwrap();
		assert_eq!(amf_region_id, deserialized);
	}
}
