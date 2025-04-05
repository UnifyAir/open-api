#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};

#[allow(unused_imports)]
pub use crate::progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use crate::progenitor_client::{RequestBuilderExt, encode_path};

/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {

	/// Error types.
	pub use crate::common::*;

	/// Contains identities representing those UEs potentially affected by a
	/// data-loss event at the UDR
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains identities representing those UEs potentially
	/// affected by a data-loss event at the UDR",
	///  "type": "object",
	///  "properties": {
	///    "dnnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnn"
	///      },
	///      "minItems": 1
	///    },
	///    "gpsiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
	///      },
	///      "minItems": 1
	///    },
	///    "resetIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "udrGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DataRestorationNotification {
		#[serde(rename = "dnnList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnn_list: Vec<Dnn>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "resetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub reset_ids: Vec<String>,
		#[serde(rename = "sNssaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub s_nssai_list: Vec<Snssai>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
		#[serde(
			rename = "udrGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub udr_group_id: Option<NfGroupId>,
	}

	impl From<&DataRestorationNotification> for DataRestorationNotification {
		fn from(value: &DataRestorationNotification) -> Self {
			value.clone()
		}
	}

	/// A range of GPSIs (subscriber identities), either based on a numeric
	/// range, or based on regular-expression matching
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A range of GPSIs (subscriber identities), either based
	/// on a numeric range, or based on regular-expression matching\n",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "type": "string",
	///      "pattern": "^[0-9]+$"
	///    },
	///    "pattern": {
	///      "type": "string"
	///    },
	///    "start": {
	///      "type": "string",
	///      "pattern": "^[0-9]+$"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct IdentityRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<IdentityRangeEnd>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pattern: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<IdentityRangeStart>,
	}

	impl From<&IdentityRange> for IdentityRange {
		fn from(value: &IdentityRange) -> Self {
			value.clone()
		}
	}

	/// IdentityRangeEnd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]+$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked,
	)]
	pub struct IdentityRangeEnd(String);

	impl ::std::ops::Deref for IdentityRangeEnd {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<IdentityRangeEnd> for String {
		fn from(value: IdentityRangeEnd) -> Self {
			value.0
		}
	}

	impl From<&IdentityRangeEnd> for IdentityRangeEnd {
		fn from(value: &IdentityRangeEnd) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for IdentityRangeEnd {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for IdentityRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for IdentityRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for IdentityRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for IdentityRangeEnd {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// IdentityRangeStart
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]+$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked,
	)]
	pub struct IdentityRangeStart(String);

	impl ::std::ops::Deref for IdentityRangeStart {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<IdentityRangeStart> for String {
		fn from(value: IdentityRangeStart) -> Self {
			value.0
		}
	}

	impl From<&IdentityRangeStart> for IdentityRangeStart {
		fn from(value: &IdentityRangeStart) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for IdentityRangeStart {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for IdentityRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for IdentityRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for IdentityRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for IdentityRangeStart {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// Contains the NFGroupIds for the requested NF types. The NFType is the
	/// key of the map.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the NFGroupIds for the requested NF types. The
	/// NFType is the key of the map.",
	///  "type": "object",
	///  "minProperties": 1,
	///  "additionalProperties": {
	///    "$ref": "#/components/schemas/NfGroupId"
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfGroupIdMapResult(pub ::std::collections::HashMap<String, NfGroupId>);

	impl ::std::ops::Deref for NfGroupIdMapResult {
		type Target = ::std::collections::HashMap<String, NfGroupId>;
		fn deref(&self) -> &::std::collections::HashMap<String, NfGroupId> {
			&self.0
		}
	}

	impl From<NfGroupIdMapResult> for ::std::collections::HashMap<String, NfGroupId> {
		fn from(value: NfGroupIdMapResult) -> Self {
			value.0
		}
	}

	impl From<&NfGroupIdMapResult> for NfGroupIdMapResult {
		fn from(value: &NfGroupIdMapResult) -> Self {
			value.clone()
		}
	}

	impl From<::std::collections::HashMap<String, NfGroupId>> for NfGroupIdMapResult {
		fn from(value: ::std::collections::HashMap<String, NfGroupId>) -> Self {
			Self(value)
		}
	}

	/// Represents the Subscription Identifier SUPI or GPSI or IMPI or IMPU.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the Subscription Identifier SUPI or GPSI or
	/// IMPI or IMPU.",
	///  "type": "string",
	///  "pattern":
	/// "^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|impi-.
	/// +|impu-.+|.+)$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked,
	)]
	pub struct SubscriberId(String);

	impl ::std::ops::Deref for SubscriberId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SubscriberId> for String {
		fn from(value: SubscriberId) -> Self {
			value.0
		}
	}

	impl From<&SubscriberId> for SubscriberId {
		fn from(value: &SubscriberId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SubscriberId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|impi-.+|impu-.+|.\
				 +)$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@\
				            ]+|impi-.+|impu-.+|.+)$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SubscriberId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SubscriberId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SubscriberId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SubscriberId {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// A range of SUPIs (subscriber identities), either based on a numeric
	/// range, or based on regular-expression matching
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A range of SUPIs (subscriber identities), either based
	/// on a numeric range, or based on regular-expression matching\n",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "type": "string",
	///      "pattern": "^[0-9]+$"
	///    },
	///    "pattern": {
	///      "type": "string"
	///    },
	///    "start": {
	///      "type": "string",
	///      "pattern": "^[0-9]+$"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SupiRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<SupiRangeEnd>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pattern: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<SupiRangeStart>,
	}

	impl From<&SupiRange> for SupiRange {
		fn from(value: &SupiRange) -> Self {
			value.clone()
		}
	}

	/// SupiRangeEnd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]+$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked,
	)]
	pub struct SupiRangeEnd(String);

	impl ::std::ops::Deref for SupiRangeEnd {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SupiRangeEnd> for String {
		fn from(value: SupiRangeEnd) -> Self {
			value.0
		}
	}

	impl From<&SupiRangeEnd> for SupiRangeEnd {
		fn from(value: &SupiRangeEnd) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SupiRangeEnd {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SupiRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SupiRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SupiRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SupiRangeEnd {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// SupiRangeStart
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]+$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
		NewUnchecked,
	)]
	pub struct SupiRangeStart(String);

	impl ::std::ops::Deref for SupiRangeStart {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SupiRangeStart> for String {
		fn from(value: SupiRangeStart) -> Self {
			value.0
		}
	}

	impl From<&SupiRangeStart> for SupiRangeStart {
		fn from(value: &SupiRangeStart) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SupiRangeStart {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SupiRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SupiRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SupiRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SupiRangeStart {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// String providing an URI formatted according to RFC 3986.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String providing an URI formatted according to RFC
	/// 3986.",
	///  "type": "string"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub struct Uri(pub String);

	impl ::std::ops::Deref for Uri {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Uri> for String {
		fn from(value: Uri) -> Self {
			value.0
		}
	}

	impl From<&Uri> for Uri {
		fn from(value: &Uri) -> Self {
			value.clone()
		}
	}

	impl From<String> for Uri {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Uri {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for Uri {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}
}
