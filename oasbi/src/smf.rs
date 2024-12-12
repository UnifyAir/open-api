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

	/// Error returned in the access token response message
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Error returned in the access token response message",
	///  "type": "object",
	///  "required": [
	///    "error"
	///  ],
	///  "properties": {
	///    "error": {
	///      "type": "string",
	///      "enum": [
	///        "invalid_request",
	///        "invalid_client",
	///        "invalid_grant",
	///        "unauthorized_client",
	///        "unsupported_grant_type",
	///        "invalid_scope"
	///      ]
	///    },
	///    "error_description": {
	///      "type": "string"
	///    },
	///    "error_uri": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AccessTokenErr {
		pub error: AccessTokenErrError,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub error_description: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub error_uri: Option<String>,
	}

	impl From<&AccessTokenErr> for AccessTokenErr {
		fn from(value: &AccessTokenErr) -> Self {
			value.clone()
		}
	}

	/// AccessTokenErrError
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "invalid_request",
	///    "invalid_client",
	///    "invalid_grant",
	///    "unauthorized_client",
	///    "unsupported_grant_type",
	///    "invalid_scope"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Copy,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum AccessTokenErrError {
		#[default]
		#[serde(rename = "invalid_request")]
		InvalidRequest,
		#[serde(rename = "invalid_client")]
		InvalidClient,
		#[serde(rename = "invalid_grant")]
		InvalidGrant,
		#[serde(rename = "unauthorized_client")]
		UnauthorizedClient,
		#[serde(rename = "unsupported_grant_type")]
		UnsupportedGrantType,
		#[serde(rename = "invalid_scope")]
		InvalidScope,
	}

	impl From<&AccessTokenErrError> for AccessTokenErrError {
		fn from(value: &AccessTokenErrError) -> Self {
			value.clone()
		}
	}

	impl ToString for AccessTokenErrError {
		fn to_string(&self) -> String {
			match *self {
				Self::InvalidRequest => "invalid_request".to_string(),
				Self::InvalidClient => "invalid_client".to_string(),
				Self::InvalidGrant => "invalid_grant".to_string(),
				Self::UnauthorizedClient => "unauthorized_client".to_string(),
				Self::UnsupportedGrantType => "unsupported_grant_type".to_string(),
				Self::InvalidScope => "invalid_scope".to_string(),
			}
		}
	}

	impl std::str::FromStr for AccessTokenErrError {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"invalid_request" => Ok(Self::InvalidRequest),
				"invalid_client" => Ok(Self::InvalidClient),
				"invalid_grant" => Ok(Self::InvalidGrant),
				"unauthorized_client" => Ok(Self::UnauthorizedClient),
				"unsupported_grant_type" => Ok(Self::UnsupportedGrantType),
				"invalid_scope" => Ok(Self::InvalidScope),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AccessTokenErrError {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AccessTokenErrError {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AccessTokenErrError {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains information related to the access token request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains information related to the access token
	/// request",
	///  "type": "object",
	///  "required": [
	///    "grant_type",
	///    "nfInstanceId",
	///    "scope"
	///  ],
	///  "properties": {
	///    "grant_type": {
	///      "type": "string",
	///      "enum": [
	///        "client_credentials"
	///      ]
	///    },
	///    "hnrfAccessTokenUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nfType": {
	///      "$ref": "#/components/schemas/NFType"
	///    },
	///    "requesterFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "requesterPlmn": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "requesterPlmnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
	///      },
	///      "minItems": 2
	///    },
	///    "requesterSnpnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnIdNid"
	///      },
	///      "minItems": 1
	///    },
	///    "requesterSnssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "scope": {
	///      "type": "string",
	///      "pattern": "^([a-zA-Z0-9_:-]+)( [a-zA-Z0-9_:-]+)*$"
	///    },
	///    "sourceNfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "targetNfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "targetNfServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "targetNfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "targetNfType": {
	///      "$ref": "#/components/schemas/NFType"
	///    },
	///    "targetNsiList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "targetPlmn": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "targetSnpn": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "targetSnssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AccessTokenReq {
		pub grant_type: AccessTokenReqGrantType,
		#[serde(
			rename = "hnrfAccessTokenUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hnrf_access_token_uri: Option<Uri>,
		#[serde(rename = "nfInstanceId")]
		pub nf_instance_id: NfInstanceId,
		#[serde(rename = "nfType", default, skip_serializing_if = "Option::is_none")]
		pub nf_type: Option<NfType>,
		#[serde(
			rename = "requesterFqdn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub requester_fqdn: Option<Fqdn>,
		#[serde(
			rename = "requesterPlmn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub requester_plmn: Option<PlmnId>,
		#[serde(
			rename = "requesterPlmnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub requester_plmn_list: Vec<PlmnId>,
		#[serde(
			rename = "requesterSnpnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub requester_snpn_list: Vec<PlmnIdNid>,
		#[serde(
			rename = "requesterSnssaiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub requester_snssai_list: Vec<Snssai>,
		pub scope: AccessTokenReqScope,
		#[serde(
			rename = "sourceNfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_nf_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "targetNfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_nf_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "targetNfServiceSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_nf_service_set_id: Option<NfServiceSetId>,
		#[serde(
			rename = "targetNfSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_nf_set_id: Option<NfSetId>,
		#[serde(
			rename = "targetNfType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_nf_type: Option<NfType>,
		#[serde(
			rename = "targetNsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub target_nsi_list: Vec<String>,
		#[serde(
			rename = "targetPlmn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_plmn: Option<PlmnId>,
		#[serde(
			rename = "targetSnpn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_snpn: Option<PlmnIdNid>,
		#[serde(
			rename = "targetSnssaiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub target_snssai_list: Vec<Snssai>,
	}

	impl From<&AccessTokenReq> for AccessTokenReq {
		fn from(value: &AccessTokenReq) -> Self {
			value.clone()
		}
	}

	/// AccessTokenReqGrantType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "client_credentials"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Copy,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum AccessTokenReqGrantType {
		#[default]
		#[serde(rename = "client_credentials")]
		ClientCredentials,
	}

	impl From<&AccessTokenReqGrantType> for AccessTokenReqGrantType {
		fn from(value: &AccessTokenReqGrantType) -> Self {
			value.clone()
		}
	}

	impl ToString for AccessTokenReqGrantType {
		fn to_string(&self) -> String {
			match *self {
				Self::ClientCredentials => "client_credentials".to_string(),
			}
		}
	}

	impl std::str::FromStr for AccessTokenReqGrantType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"client_credentials" => Ok(Self::ClientCredentials),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AccessTokenReqGrantType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AccessTokenReqGrantType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AccessTokenReqGrantType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// AccessTokenReqScope
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^([a-zA-Z0-9_:-]+)( [a-zA-Z0-9_:-]+)*$"
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
	)]
	pub struct AccessTokenReqScope(String);

	impl ::std::ops::Deref for AccessTokenReqScope {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AccessTokenReqScope> for String {
		fn from(value: AccessTokenReqScope) -> Self {
			value.0
		}
	}

	impl From<&AccessTokenReqScope> for AccessTokenReqScope {
		fn from(value: &AccessTokenReqScope) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AccessTokenReqScope {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^([a-zA-Z0-9_:-]+)( [a-zA-Z0-9_:-]+)*$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err(
					"doesn't match pattern \"^([a-zA-Z0-9_:-]+)( [a-zA-Z0-9_:-]+)*$\"".into(),
				);
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for AccessTokenReqScope {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AccessTokenReqScope {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AccessTokenReqScope {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AccessTokenReqScope {
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

	/// Indicates whether the access is  via 3GPP or via non-3GPP.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates whether the access is  via 3GPP or via
	/// non-3GPP.",
	///  "type": "string",
	///  "enum": [
	///    "3GPP_ACCESS",
	///    "NON_3GPP_ACCESS"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Copy,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub enum AccessType {
		#[default]
		#[serde(rename = "3GPP_ACCESS")]
		ThreeGppAccess,
		#[serde(rename = "NON_3GPP_ACCESS")]
		Non3gppAccess,
	}

	impl From<&AccessType> for AccessType {
		fn from(value: &AccessType) -> Self {
			value.clone()
		}
	}

	impl ToString for AccessType {
		fn to_string(&self) -> String {
			match *self {
				Self::ThreeGppAccess => "3GPP_ACCESS".to_string(),
				Self::Non3gppAccess => "NON_3GPP_ACCESS".to_string(),
			}
		}
	}

	impl std::str::FromStr for AccessType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"3GPP_ACCESS" => Ok(Self::ThreeGppAccess),
				"NON_3GPP_ACCESS" => Ok(Self::Non3gppAccess),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AccessType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AccessType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AccessType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents an acknowledgement information of an event notification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an acknowledgement information of an event
	/// notification.",
	///  "type": "object",
	///  "required": [
	///    "ackResult",
	///    "notifId"
	///  ],
	///  "properties": {
	///    "ackResult": {
	///      "$ref": "#/components/schemas/AfResultInfo"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "notifId": {
	///      "type": "string"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AckOfNotify {
		#[serde(rename = "ackResult")]
		pub ack_result: AfResultInfo,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "notifId")]
		pub notif_id: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
	}

	impl From<&AckOfNotify> for AckOfNotify {
		fn from(value: &AckOfNotify) -> Self {
			value.clone()
		}
	}

	/// The enumeration AdditionalQosFlowInfo provides additional QoS flow
	/// information (see clause  9.3.1.12 3GPP TS 38.413 [11]). It shall comply
	/// with the provisions defined in table 5.5.3.12-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration AdditionalQosFlowInfo provides
	/// additional QoS flow information (see clause  9.3.1.12 3GPP TS 38.413
	/// [11]). It shall comply with the provisions defined in table
	/// 5.5.3.12-1.\n",
	///  "anyOf": [
	///    {
	///      "type": "string",
	///      "enum": [
	///        "MORE_LIKELY"
	///      ],
	///      "x-allow-unknown": true
	///    },
	///    {
	///      "$ref": "#/components/schemas/NullValue"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum AdditionalQosFlowInfo {
		#[default]
		Variant0(AdditionalQosFlowInfoVariant0),
		Variant1(NullValue),
	}

	impl From<&AdditionalQosFlowInfo> for AdditionalQosFlowInfo {
		fn from(value: &AdditionalQosFlowInfo) -> Self {
			value.clone()
		}
	}

	impl From<AdditionalQosFlowInfoVariant0> for AdditionalQosFlowInfo {
		fn from(value: AdditionalQosFlowInfoVariant0) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<NullValue> for AdditionalQosFlowInfo {
		fn from(value: NullValue) -> Self {
			Self::Variant1(value)
		}
	}

	/// AdditionalQosFlowInfoVariant0
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "MORE_LIKELY"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum AdditionalQosFlowInfoVariant0 {
		#[default]
		#[serde(rename = "MORE_LIKELY")]
		MoreLikely,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AdditionalQosFlowInfoVariant0> for AdditionalQosFlowInfoVariant0 {
		fn from(value: &AdditionalQosFlowInfoVariant0) -> Self {
			value.clone()
		}
	}

	impl ToString for AdditionalQosFlowInfoVariant0 {
		fn to_string(&self) -> String {
			match *self {
				Self::MoreLikely => "MORE_LIKELY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AdditionalQosFlowInfoVariant0 {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MORE_LIKELY" => Ok(Self::MoreLikely),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AdditionalQosFlowInfoVariant0 {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AdditionalQosFlowInfoVariant0 {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AdditionalQosFlowInfoVariant0 {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// indicates first, second or third additional indirect data forwarding
	/// tunnel
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "indicates first, second or third additional indirect
	/// data forwarding tunnel",
	///  "type": "integer",
	///  "maximum": 3.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AdditionalTnlNb(pub i64);

	impl ::std::ops::Deref for AdditionalTnlNb {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<AdditionalTnlNb> for i64 {
		fn from(value: AdditionalTnlNb) -> Self {
			value.0
		}
	}

	impl From<&AdditionalTnlNb> for AdditionalTnlNb {
		fn from(value: &AdditionalTnlNb) -> Self {
			value.clone()
		}
	}

	impl From<i64> for AdditionalTnlNb {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AdditionalTnlNb {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for AdditionalTnlNb {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AdditionalTnlNb {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AdditionalTnlNb {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for AdditionalTnlNb {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// IP address and/or FQDN.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "IP address and/or FQDN.",
	///  "type": "object",
	///  "properties": {
	///    "fqdn": {
	///      "description": "Indicates an FQDN.",
	///      "type": "string"
	///    },
	///    "ipAddr": {
	///      "$ref": "#/components/schemas/IpAddr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AddrFqdn {
		/// Indicates an FQDN.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub fqdn: Option<String>,
		#[serde(rename = "ipAddr", default, skip_serializing_if = "Option::is_none")]
		pub ip_addr: Option<IpAddr>,
	}

	impl From<&AddrFqdn> for AddrFqdn {
		fn from(value: &AddrFqdn) -> Self {
			value.clone()
		}
	}

	/// AF Coordination Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "AF Coordination Information",
	///  "type": "object",
	///  "properties": {
	///    "notificationInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NotificationInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "sourceDnai": {
	///      "$ref": "#/components/schemas/Dnai"
	///    },
	///    "sourceUeIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "sourceUeIpv6Prefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AfCoordinationInfo {
		#[serde(
			rename = "notificationInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub notification_info_list: Vec<NotificationInfo>,
		#[serde(
			rename = "sourceDnai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_dnai: Option<Dnai>,
		#[serde(
			rename = "sourceUeIpv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_ue_ipv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "sourceUeIpv6Prefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_ue_ipv6_prefix: Option<Ipv6Prefix>,
	}

	impl From<&AfCoordinationInfo> for AfCoordinationInfo {
		fn from(value: &AfCoordinationInfo) -> Self {
			value.clone()
		}
	}

	/// Identifies the result of application layer handling.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the result of application layer handling.",
	///  "type": "object",
	///  "required": [
	///    "afStatus"
	///  ],
	///  "properties": {
	///    "afStatus": {
	///      "$ref": "#/components/schemas/AfResultStatus"
	///    },
	///    "easIpReplaceInfos": {
	///      "description": "Contains EAS IP replacement information.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EasIpReplacementInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "trafficRoute": {
	///      "$ref": "#/components/schemas/RouteToLocation"
	///    },
	///    "upBuffInd": {
	///      "description": "If present and set to \"true\" it indicates that
	/// buffering of uplink traffic to the target DNAI is needed.\n",
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AfResultInfo {
		#[serde(rename = "afStatus")]
		pub af_status: AfResultStatus,
		/// Contains EAS IP replacement information.
		#[serde(
			rename = "easIpReplaceInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub eas_ip_replace_infos: Vec<EasIpReplacementInfo>,
		#[serde(
			rename = "trafficRoute",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub traffic_route: Option<RouteToLocation>,
		/// If present and set to "true" it indicates that buffering of uplink
		/// traffic to the target DNAI is needed.
		#[serde(rename = "upBuffInd", default, skip_serializing_if = "Option::is_none")]
		pub up_buff_ind: Option<bool>,
	}

	impl From<&AfResultInfo> for AfResultInfo {
		fn from(value: &AfResultInfo) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - SUCCESS: The application layer is ready or the relocation is
	///   completed.
	/// - TEMPORARY_CONGESTION: The application relocation fails due to
	///   temporary congestion.
	/// - RELOC_NO_ALLOWED: The application relocation fails because application
	///   relocation is not allowed.
	/// - OTHER: The application relocation fails due to other reason.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- SUCCESS: The application layer
	/// is ready or the relocation is completed.\n- TEMPORARY_CONGESTION: The
	/// application relocation fails due to temporary congestion.\n-
	/// RELOC_NO_ALLOWED: The application relocation fails because application
	/// relocation is not allowed.\n- OTHER: The application relocation fails
	/// due to other reason.\n",
	///  "type": "string",
	///  "enum": [
	///    "SUCCESS",
	///    "TEMPORARY_CONGESTION",
	///    "RELOC_NO_ALLOWED",
	///    "OTHER"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum AfResultStatus {
		#[default]
		#[serde(rename = "SUCCESS")]
		Success,
		#[serde(rename = "TEMPORARY_CONGESTION")]
		TemporaryCongestion,
		#[serde(rename = "RELOC_NO_ALLOWED")]
		RelocNoAllowed,
		#[serde(rename = "OTHER")]
		Other,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AfResultStatus> for AfResultStatus {
		fn from(value: &AfResultStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for AfResultStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Success => "SUCCESS".to_string(),
				Self::TemporaryCongestion => "TEMPORARY_CONGESTION".to_string(),
				Self::RelocNoAllowed => "RELOC_NO_ALLOWED".to_string(),
				Self::Other => "OTHER".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AfResultStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SUCCESS" => Ok(Self::Success),
				"TEMPORARY_CONGESTION" => Ok(Self::TemporaryCongestion),
				"RELOC_NO_ALLOWED" => Ok(Self::RelocNoAllowed),
				"OTHER" => Ok(Self::Other),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AfResultStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AfResultStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AfResultStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Alternative QoS Profile
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Alternative QoS Profile",
	///  "type": "object",
	///  "required": [
	///    "index"
	///  ],
	///  "properties": {
	///    "guaFbrDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "guaFbrUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "index": {
	///      "type": "integer",
	///      "maximum": 8.0,
	///      "minimum": 1.0
	///    },
	///    "packetDelayBudget": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "packetErrRate": {
	///      "$ref": "#/components/schemas/PacketErrRate"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AlternativeQosProfile {
		#[serde(rename = "guaFbrDl", default, skip_serializing_if = "Option::is_none")]
		pub gua_fbr_dl: Option<BitRate>,
		#[serde(rename = "guaFbrUl", default, skip_serializing_if = "Option::is_none")]
		pub gua_fbr_ul: Option<BitRate>,
		pub index: i64,
		#[serde(
			rename = "packetDelayBudget",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub packet_delay_budget: Option<PacketDelBudget>,
		#[serde(
			rename = "packetErrRate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub packet_err_rate: Option<PacketErrRate>,
	}

	impl From<&AlternativeQosProfile> for AlternativeQosProfile {
		fn from(value: &AlternativeQosProfile) -> Self {
			value.clone()
		}
	}

	/// Contains the maximum aggregated uplink and downlink bit rates.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the maximum aggregated uplink and downlink bit
	/// rates.",
	///  "type": "object",
	///  "required": [
	///    "downlink",
	///    "uplink"
	///  ],
	///  "properties": {
	///    "downlink": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "uplink": {
	///      "$ref": "#/components/schemas/BitRate"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Ambr {
		pub downlink: BitRate,
		pub uplink: BitRate,
	}

	impl From<&Ambr> for Ambr {
		fn from(value: &Ambr) -> Self {
			value.clone()
		}
	}

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
	pub struct AmfId(String);

	impl ::std::ops::Deref for AmfId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AmfId> for String {
		fn from(value: AmfId) -> Self {
			value.0
		}
	}

	impl From<&AmfId> for AmfId {
		fn from(value: &AmfId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AmfId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{6}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{6}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for AmfId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AmfId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AmfId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AmfId {
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

	/// Anchor SMF supported features
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Anchor SMF supported features",
	///  "type": "object",
	///  "properties": {
	///    "psetrSupportInd": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AnchorSmfFeatures {
		#[serde(
			rename = "psetrSupportInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub psetr_support_ind: Option<bool>,
	}

	impl From<&AnchorSmfFeatures> for AnchorSmfFeatures {
		fn from(value: &AnchorSmfFeatures) -> Self {
			value.clone()
		}
	}

	/// Contains the APN rate control status e.g. of the AMF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the APN rate control status e.g. of the AMF.",
	///  "type": "object",
	///  "properties": {
	///    "remainExReportsDl": {
	///      "description": "When present, it shall indicate the number of
	/// additional exception reports the AF is allowed to send downlink in the
	/// given time unit for the given APN (all PDN connections of the UE to this
	/// APN, see clause 4.7.7.3 in 3GPP TS 23.401.\n",
	///      "type": "integer",
	///      "minimum": 0.0
	///    },
	///    "remainExReportsUl": {
	///      "description": "When present, it shall indicate the number of
	/// additional exception reports the UE is allowed to send uplink in the
	/// given time unit for the given APN (all PDN connections of the UE to this
	/// APN, see clause 4.7.7.3 in 3GPP TS 23.401.\n",
	///      "type": "integer",
	///      "minimum": 0.0
	///    },
	///    "remainPacketsDl": {
	///      "description": "When present, it shall contain the number of
	/// packets the UE is allowed to send uplink in the given time unit for the
	/// given APN (all PDN connections of the UE to this APN see clause 4.7.7.3
	/// in 3GPP TS 23.401.\n",
	///      "type": "integer",
	///      "minimum": 0.0
	///    },
	///    "remainPacketsUl": {
	///      "description": "When present, it shall contain the number of
	/// packets the UE is allowed to send uplink in the given time unit for the
	/// given APN (all PDN connections of the UE to this APN see clause 4.7.7.3
	/// in 3GPP TS 23.401.\n",
	///      "type": "integer",
	///      "minimum": 0.0
	///    },
	///    "validityTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ApnRateStatus {
		/// When present, it shall indicate the number of additional exception
		/// reports the AF is allowed to send downlink in the  given time unit
		/// for the given APN (all PDN connections of the UE to this APN, see
		/// clause 4.7.7.3 in 3GPP TS 23.401.
		#[serde(
			rename = "remainExReportsDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remain_ex_reports_dl: Option<u64>,
		/// When present, it shall indicate the number of additional exception
		/// reports the UE is allowed to send uplink in the given time unit for
		/// the given APN (all PDN connections of the UE to this APN, see clause
		/// 4.7.7.3 in 3GPP TS 23.401.
		#[serde(
			rename = "remainExReportsUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remain_ex_reports_ul: Option<u64>,
		/// When present, it shall contain the number of packets the UE is
		/// allowed to send uplink in the given time unit for the given APN (all
		/// PDN connections of the UE to this APN see clause 4.7.7.3 in 3GPP TS
		/// 23.401.
		#[serde(
			rename = "remainPacketsDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remain_packets_dl: Option<u64>,
		/// When present, it shall contain the number of packets the UE is
		/// allowed to send uplink in the given time unit for the given APN (all
		/// PDN connections of the UE to this APN see clause 4.7.7.3 in 3GPP TS
		/// 23.401.
		#[serde(
			rename = "remainPacketsUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remain_packets_ul: Option<u64>,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&ApnRateStatus> for ApnRateStatus {
		fn from(value: &ApnRateStatus) -> Self {
			value.clone()
		}
	}

	/// String providing an application identifier.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String providing an application identifier.",
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
	pub struct ApplicationId(pub String);

	impl ::std::ops::Deref for ApplicationId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ApplicationId> for String {
		fn from(value: ApplicationId) -> Self {
			value.0
		}
	}

	impl From<&ApplicationId> for ApplicationId {
		fn from(value: &ApplicationId) -> Self {
			value.clone()
		}
	}

	impl From<String> for ApplicationId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ApplicationId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for ApplicationId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Possible values are:
	/// - DNN_CC: Indicates the DNN based congestion control.
	/// - SNSSAI_CC: Indicates the S-NSSAI based congestion control.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- DNN_CC: Indicates the DNN based
	/// congestion control.\n- SNSSAI_CC: Indicates the S-NSSAI based congestion
	/// control.\n",
	///  "type": "string",
	///  "enum": [
	///    "DNN_CC",
	///    "SNSSAI_CC"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum AppliedSmccType {
		#[default]
		#[serde(rename = "DNN_CC")]
		DnnCc,
		#[serde(rename = "SNSSAI_CC")]
		SnssaiCc,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AppliedSmccType> for AppliedSmccType {
		fn from(value: &AppliedSmccType) -> Self {
			value.clone()
		}
	}

	impl ToString for AppliedSmccType {
		fn to_string(&self) -> String {
			match *self {
				Self::DnnCc => "DNN_CC".to_string(),
				Self::SnssaiCc => "SNSSAI_CC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AppliedSmccType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DNN_CC" => Ok(Self::DnnCc),
				"SNSSAI_CC" => Ok(Self::SnssaiCc),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AppliedSmccType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AppliedSmccType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AppliedSmccType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains Allocation and Retention Priority information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains Allocation and Retention Priority
	/// information.",
	///  "type": "object",
	///  "required": [
	///    "preemptCap",
	///    "preemptVuln",
	///    "priorityLevel"
	///  ],
	///  "properties": {
	///    "preemptCap": {
	///      "$ref": "#/components/schemas/PreemptionCapability"
	///    },
	///    "preemptVuln": {
	///      "$ref": "#/components/schemas/PreemptionVulnerability"
	///    },
	///    "priorityLevel": {
	///      "$ref": "#/components/schemas/ArpPriorityLevel"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Arp {
		#[serde(rename = "preemptCap")]
		pub preempt_cap: PreemptionCapability,
		#[serde(rename = "preemptVuln")]
		pub preempt_vuln: PreemptionVulnerability,
		#[serde(rename = "priorityLevel")]
		pub priority_level: ArpPriorityLevel,
	}

	impl From<&Arp> for Arp {
		fn from(value: &Arp) -> Self {
			value.clone()
		}
	}

	/// nullable true shall not be used for this attribute. Unsigned integer
	/// indicating the ARP Priority Level (see clause 5.7.2.2 of 3GPP TS 23.501,
	/// within the range 1 to 15.Values are ordered in decreasing order of
	/// priority, i.e. with 1 as the highest priority and 15 as the lowest
	/// priority.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "nullable true shall not be used for this attribute.
	/// Unsigned integer indicating the ARP Priority Level (see clause 5.7.2.2
	/// of 3GPP TS 23.501, within the range 1 to 15.Values are ordered in
	/// decreasing order of priority, i.e. with 1 as the highest priority and 15
	/// as the lowest priority. \n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 15.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ArpPriorityLevel(pub Option<i64>);

	impl ::std::ops::Deref for ArpPriorityLevel {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<ArpPriorityLevel> for Option<i64> {
		fn from(value: ArpPriorityLevel) -> Self {
			value.0
		}
	}

	impl From<&ArpPriorityLevel> for ArpPriorityLevel {
		fn from(value: &ArpPriorityLevel) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for ArpPriorityLevel {
		fn from(value: Option<i64>) -> Self {
			Self(value)
		}
	}

	/// Unsigned integer indicating Averaging Window (see clause 5.7.3.6 and
	/// 5.7.4 of 3GPP TS 23.501), expressed in milliseconds.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer indicating Averaging Window (see
	/// clause 5.7.3.6 and 5.7.4 of 3GPP TS 23.501), expressed in
	/// milliseconds.",
	///  "default": 2000,
	///  "type": "integer",
	///  "maximum": 4095.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AverWindow(pub i64);

	impl ::std::ops::Deref for AverWindow {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<AverWindow> for i64 {
		fn from(value: AverWindow) -> Self {
			value.0
		}
	}

	impl From<&AverWindow> for AverWindow {
		fn from(value: &AverWindow) -> Self {
			value.clone()
		}
	}

	impl From<i64> for AverWindow {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AverWindow {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for AverWindow {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AverWindow {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AverWindow {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for AverWindow {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Provides details of the Backup AMF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Provides details of the Backup AMF.",
	///  "type": "object",
	///  "required": [
	///    "backupAmf"
	///  ],
	///  "properties": {
	///    "backupAmf": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "guamiList": {
	///      "description": "If present, this IE shall contain the list of GUAMI(s) (supported by the AMF) for which the backupAmf IE applies.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Guami"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BackupAmfInfo {
		#[serde(rename = "backupAmf")]
		pub backup_amf: Fqdn,
		/// If present, this IE shall contain the list of GUAMI(s) (supported by
		/// the AMF) for which the backupAmf IE applies.
		#[serde(rename = "guamiList", default, skip_serializing_if = "Vec::is_empty")]
		pub guami_list: Vec<crate::common::common_models::Guami>,
	}

	impl From<&BackupAmfInfo> for BackupAmfInfo {
		fn from(value: &BackupAmfInfo) -> Self {
			value.clone()
		}
	}

	/// Parameters "replaceableInd" and "rechargeableInd" are only included if
	/// the value of Parameter "batteryInd" is true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Parameters \"replaceableInd\" and \"rechargeableInd\" are only included if the value of Parameter \"batteryInd\" is true.\n",
	///  "type": "object",
	///  "properties": {
	///    "batteryInd": {
	///      "description": "This IE shall indicate whether the UE is battery
	/// powered or not. true: the UE is battery powered; false or absent: the UE
	/// is not battery powered\n",
	///      "type": "boolean"
	///    },
	///    "rechargeableInd": {
	///      "description": "This IE shall indicate whether the battery of the
	/// UE is rechargeable or not. true: the battery of UE is rechargeable;
	/// false or absent: the battery of the UE is not rechargeable.\n",
	///      "type": "boolean"
	///    },
	///    "replaceableInd": {
	///      "description": "This IE shall indicate whether the battery of the
	/// UE is replaceable or not. true: the battery of the UE is replaceable;
	/// false or absent: the battery of the UE is not replaceable.\n",
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BatteryIndication {
		/// This IE shall indicate whether the UE is battery powered or not.
		/// true: the UE is battery powered; false or absent: the UE is not
		/// battery powered
		#[serde(
			rename = "batteryInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub battery_ind: Option<bool>,
		/// This IE shall indicate whether the battery of the UE is rechargeable
		/// or not. true: the battery of UE is rechargeable; false or absent:
		/// the battery of the UE is not rechargeable.
		#[serde(
			rename = "rechargeableInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rechargeable_ind: Option<bool>,
		/// This IE shall indicate whether the battery of the UE is replaceable
		/// or not. true: the battery of the UE is replaceable; false or absent:
		/// the battery of the UE is not replaceable.
		#[serde(
			rename = "replaceableInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub replaceable_ind: Option<bool>,
	}

	impl From<&BatteryIndication> for BatteryIndication {
		fn from(value: &BatteryIndication) -> Self {
			value.clone()
		}
	}

	/// String representing a bit rate; the prefixes follow the standard symbols
	/// from The International System of Units, and represent x1000 multipliers,
	/// with the exception that prefix "K" is used to represent the standard
	/// symbol "k".
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing a bit rate; the prefixes follow the
	/// standard symbols from The International System of Units, and represent
	/// x1000 multipliers, with the exception that prefix \"K\" is used to
	/// represent the standard symbol \"k\".\n",
	///  "type": "string",
	///  "pattern": "^\\d+(\\.\\d+)? (bps|Kbps|Mbps|Gbps|Tbps)$"
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
	)]
	pub struct BitRate(String);

	impl ::std::ops::Deref for BitRate {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<BitRate> for String {
		fn from(value: BitRate) -> Self {
			value.0
		}
	}

	impl From<&BitRate> for BitRate {
		fn from(value: &BitRate) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for BitRate {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^\\d+(\\.\\d+)? (bps|Kbps|Mbps|Gbps|Tbps)$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err(
					"doesn't match pattern \"^\\d+(\\.\\d+)? (bps|Kbps|Mbps|Gbps|Tbps)$\"".into(),
				);
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for BitRate {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for BitRate {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for BitRate {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for BitRate {
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

	/// string with format 'bytes' as defined in OpenAPI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'bytes' as defined in OpenAPI",
	///  "type": "string",
	///  "format": "byte"
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
	pub struct Bytes(pub String);

	impl ::std::ops::Deref for Bytes {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Bytes> for String {
		fn from(value: Bytes) -> Self {
			value.0
		}
	}

	impl From<&Bytes> for Bytes {
		fn from(value: &Bytes) -> Self {
			value.clone()
		}
	}

	impl From<String> for Bytes {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Bytes {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for Bytes {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Cause information. Possible values are
	/// - REL_DUE_TO_HO
	/// - EPS_FALLBACK
	/// - REL_DUE_TO_UP_SEC
	/// - DNN_CONGESTION
	/// - S_NSSAI_CONGESTION
	/// - REL_DUE_TO_REACTIVATION
	/// - 5G_AN_NOT_RESPONDING
	/// - REL_DUE_TO_SLICE_NOT_AVAILABLE
	/// - REL_DUE_TO_DUPLICATE_SESSION_ID
	/// - PDU_SESSION_STATUS_MISMATCH
	/// - HO_FAILURE
	/// - INSUFFICIENT_UP_RESOURCES
	/// - PDU_SESSION_HANDED_OVER
	/// - PDU_SESSION_RESUMED
	/// - CN_ASSISTED_RAN_PARAMETER_TUNING
	/// - ISMF_CONTEXT_TRANSFER
	/// - SMF_CONTEXT_TRANSFER
	/// - REL_DUE_TO_PS_TO_CS_HO
	/// - REL_DUE_TO_SUBSCRIPTION_CHANGE
	/// - HO_CANCEL
	/// - REL_DUE_TO_SLICE_NOT_AUTHORIZED
	/// - PDU_SESSION_HAND_OVER_FAILURE
	/// - DDN_FAILURE_STATUS
	/// - REL_DUE_TO_CP_ONLY_NOT_APPLICABLE
	/// - NOT_SUPPORTED_WITH_ISMF
	/// - CHANGED_ANCHOR_SMF
	/// - CHANGED_INTERMEDIATE_SMF
	/// - TARGET_DNAI_NOTIFICATION
	/// - REL_DUE_TO_VPLMN_QOS_FAILURE
	/// - REL_DUE_TO_SMF_NOT_SUPPORT_PSETR
	/// - REL_DUE_TO_SNPN_SNPN_MOBILITY
	/// - REL_DUE_TO_NO_HR_AGREEMENT
	/// - REL_DUE_TO_UNSPECIFIED_REASON
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Cause information. Possible values are\n-
	/// REL_DUE_TO_HO\n- EPS_FALLBACK\n- REL_DUE_TO_UP_SEC\n- DNN_CONGESTION\n-
	/// S_NSSAI_CONGESTION\n- REL_DUE_TO_REACTIVATION\n- 5G_AN_NOT_RESPONDING\n-
	/// REL_DUE_TO_SLICE_NOT_AVAILABLE\n- REL_DUE_TO_DUPLICATE_SESSION_ID\n-
	/// PDU_SESSION_STATUS_MISMATCH\n- HO_FAILURE\n-
	/// INSUFFICIENT_UP_RESOURCES\n- PDU_SESSION_HANDED_OVER\n-
	/// PDU_SESSION_RESUMED\n- CN_ASSISTED_RAN_PARAMETER_TUNING\n-
	/// ISMF_CONTEXT_TRANSFER\n- SMF_CONTEXT_TRANSFER\n-
	/// REL_DUE_TO_PS_TO_CS_HO\n- REL_DUE_TO_SUBSCRIPTION_CHANGE\n- HO_CANCEL\n-
	/// REL_DUE_TO_SLICE_NOT_AUTHORIZED\n- PDU_SESSION_HAND_OVER_FAILURE\n-
	/// DDN_FAILURE_STATUS\n- REL_DUE_TO_CP_ONLY_NOT_APPLICABLE\n-
	/// NOT_SUPPORTED_WITH_ISMF\n- CHANGED_ANCHOR_SMF\n-
	/// CHANGED_INTERMEDIATE_SMF\n- TARGET_DNAI_NOTIFICATION\n-
	/// REL_DUE_TO_VPLMN_QOS_FAILURE\n- REL_DUE_TO_SMF_NOT_SUPPORT_PSETR\n-
	/// REL_DUE_TO_SNPN_SNPN_MOBILITY\n- REL_DUE_TO_NO_HR_AGREEMENT\n-
	/// REL_DUE_TO_UNSPECIFIED_REASON\n",
	///  "type": "string",
	///  "enum": [
	///    "REL_DUE_TO_HO",
	///    "EPS_FALLBACK",
	///    "REL_DUE_TO_UP_SEC",
	///    "DNN_CONGESTION",
	///    "S_NSSAI_CONGESTION",
	///    "REL_DUE_TO_REACTIVATION",
	///    "5G_AN_NOT_RESPONDING",
	///    "REL_DUE_TO_SLICE_NOT_AVAILABLE",
	///    "REL_DUE_TO_DUPLICATE_SESSION_ID",
	///    "PDU_SESSION_STATUS_MISMATCH",
	///    "HO_FAILURE",
	///    "INSUFFICIENT_UP_RESOURCES",
	///    "PDU_SESSION_HANDED_OVER",
	///    "PDU_SESSION_RESUMED",
	///    "CN_ASSISTED_RAN_PARAMETER_TUNING",
	///    "ISMF_CONTEXT_TRANSFER",
	///    "SMF_CONTEXT_TRANSFER",
	///    "REL_DUE_TO_PS_TO_CS_HO",
	///    "REL_DUE_TO_SUBSCRIPTION_CHANGE",
	///    "HO_CANCEL",
	///    "REL_DUE_TO_SLICE_NOT_AUTHORIZED",
	///    "PDU_SESSION_HAND_OVER_FAILURE",
	///    "DDN_FAILURE_STATUS",
	///    "REL_DUE_TO_CP_ONLY_NOT_APPLICABLE",
	///    "NOT_SUPPORTED_WITH_ISMF",
	///    "CHANGED_ANCHOR_SMF",
	///    "CHANGED_INTERMEDIATE_SMF",
	///    "TARGET_DNAI_NOTIFICATION",
	///    "REL_DUE_TO_VPLMN_QOS_FAILURE",
	///    "REL_DUE_TO_SMF_NOT_SUPPORT_PSETR",
	///    "REL_DUE_TO_SNPN_SNPN_MOBILITY",
	///    "REL_DUE_TO_NO_HR_AGREEMENT",
	///    "REL_DUE_TO_UNSPECIFIED_REASON"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum Cause {
		#[default]
		#[serde(rename = "REL_DUE_TO_HO")]
		RelDueToHo,
		#[serde(rename = "EPS_FALLBACK")]
		EpsFallback,
		#[serde(rename = "REL_DUE_TO_UP_SEC")]
		RelDueToUpSec,
		#[serde(rename = "DNN_CONGESTION")]
		DnnCongestion,
		#[serde(rename = "S_NSSAI_CONGESTION")]
		SNssaiCongestion,
		#[serde(rename = "REL_DUE_TO_REACTIVATION")]
		RelDueToReactivation,
		#[serde(rename = "5G_AN_NOT_RESPONDING")]
		FiveGAnNotResponding,
		#[serde(rename = "REL_DUE_TO_SLICE_NOT_AVAILABLE")]
		RelDueToSliceNotAvailable,
		#[serde(rename = "REL_DUE_TO_DUPLICATE_SESSION_ID")]
		RelDueToDuplicateSessionId,
		#[serde(rename = "PDU_SESSION_STATUS_MISMATCH")]
		PduSessionStatusMismatch,
		#[serde(rename = "HO_FAILURE")]
		HoFailure,
		#[serde(rename = "INSUFFICIENT_UP_RESOURCES")]
		InsufficientUpResources,
		#[serde(rename = "PDU_SESSION_HANDED_OVER")]
		PduSessionHandedOver,
		#[serde(rename = "PDU_SESSION_RESUMED")]
		PduSessionResumed,
		#[serde(rename = "CN_ASSISTED_RAN_PARAMETER_TUNING")]
		CnAssistedRanParameterTuning,
		#[serde(rename = "ISMF_CONTEXT_TRANSFER")]
		IsmfContextTransfer,
		#[serde(rename = "SMF_CONTEXT_TRANSFER")]
		SmfContextTransfer,
		#[serde(rename = "REL_DUE_TO_PS_TO_CS_HO")]
		RelDueToPsToCsHo,
		#[serde(rename = "REL_DUE_TO_SUBSCRIPTION_CHANGE")]
		RelDueToSubscriptionChange,
		#[serde(rename = "HO_CANCEL")]
		HoCancel,
		#[serde(rename = "REL_DUE_TO_SLICE_NOT_AUTHORIZED")]
		RelDueToSliceNotAuthorized,
		#[serde(rename = "PDU_SESSION_HAND_OVER_FAILURE")]
		PduSessionHandOverFailure,
		#[serde(rename = "DDN_FAILURE_STATUS")]
		DdnFailureStatus,
		#[serde(rename = "REL_DUE_TO_CP_ONLY_NOT_APPLICABLE")]
		RelDueToCpOnlyNotApplicable,
		#[serde(rename = "NOT_SUPPORTED_WITH_ISMF")]
		NotSupportedWithIsmf,
		#[serde(rename = "CHANGED_ANCHOR_SMF")]
		ChangedAnchorSmf,
		#[serde(rename = "CHANGED_INTERMEDIATE_SMF")]
		ChangedIntermediateSmf,
		#[serde(rename = "TARGET_DNAI_NOTIFICATION")]
		TargetDnaiNotification,
		#[serde(rename = "REL_DUE_TO_VPLMN_QOS_FAILURE")]
		RelDueToVplmnQosFailure,
		#[serde(rename = "REL_DUE_TO_SMF_NOT_SUPPORT_PSETR")]
		RelDueToSmfNotSupportPsetr,
		#[serde(rename = "REL_DUE_TO_SNPN_SNPN_MOBILITY")]
		RelDueToSnpnSnpnMobility,
		#[serde(rename = "REL_DUE_TO_NO_HR_AGREEMENT")]
		RelDueToNoHrAgreement,
		#[serde(rename = "REL_DUE_TO_UNSPECIFIED_REASON")]
		RelDueToUnspecifiedReason,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&Cause> for Cause {
		fn from(value: &Cause) -> Self {
			value.clone()
		}
	}

	impl ToString for Cause {
		fn to_string(&self) -> String {
			match *self {
				Self::RelDueToHo => "REL_DUE_TO_HO".to_string(),
				Self::EpsFallback => "EPS_FALLBACK".to_string(),
				Self::RelDueToUpSec => "REL_DUE_TO_UP_SEC".to_string(),
				Self::DnnCongestion => "DNN_CONGESTION".to_string(),
				Self::SNssaiCongestion => "S_NSSAI_CONGESTION".to_string(),
				Self::RelDueToReactivation => "REL_DUE_TO_REACTIVATION".to_string(),
				Self::FiveGAnNotResponding => "5G_AN_NOT_RESPONDING".to_string(),
				Self::RelDueToSliceNotAvailable => "REL_DUE_TO_SLICE_NOT_AVAILABLE".to_string(),
				Self::RelDueToDuplicateSessionId => "REL_DUE_TO_DUPLICATE_SESSION_ID".to_string(),
				Self::PduSessionStatusMismatch => "PDU_SESSION_STATUS_MISMATCH".to_string(),
				Self::HoFailure => "HO_FAILURE".to_string(),
				Self::InsufficientUpResources => "INSUFFICIENT_UP_RESOURCES".to_string(),
				Self::PduSessionHandedOver => "PDU_SESSION_HANDED_OVER".to_string(),
				Self::PduSessionResumed => "PDU_SESSION_RESUMED".to_string(),
				Self::CnAssistedRanParameterTuning => {
					"CN_ASSISTED_RAN_PARAMETER_TUNING".to_string()
				}
				Self::IsmfContextTransfer => "ISMF_CONTEXT_TRANSFER".to_string(),
				Self::SmfContextTransfer => "SMF_CONTEXT_TRANSFER".to_string(),
				Self::RelDueToPsToCsHo => "REL_DUE_TO_PS_TO_CS_HO".to_string(),
				Self::RelDueToSubscriptionChange => "REL_DUE_TO_SUBSCRIPTION_CHANGE".to_string(),
				Self::HoCancel => "HO_CANCEL".to_string(),
				Self::RelDueToSliceNotAuthorized => "REL_DUE_TO_SLICE_NOT_AUTHORIZED".to_string(),
				Self::PduSessionHandOverFailure => "PDU_SESSION_HAND_OVER_FAILURE".to_string(),
				Self::DdnFailureStatus => "DDN_FAILURE_STATUS".to_string(),
				Self::RelDueToCpOnlyNotApplicable => {
					"REL_DUE_TO_CP_ONLY_NOT_APPLICABLE".to_string()
				}
				Self::NotSupportedWithIsmf => "NOT_SUPPORTED_WITH_ISMF".to_string(),
				Self::ChangedAnchorSmf => "CHANGED_ANCHOR_SMF".to_string(),
				Self::ChangedIntermediateSmf => "CHANGED_INTERMEDIATE_SMF".to_string(),
				Self::TargetDnaiNotification => "TARGET_DNAI_NOTIFICATION".to_string(),
				Self::RelDueToVplmnQosFailure => "REL_DUE_TO_VPLMN_QOS_FAILURE".to_string(),
				Self::RelDueToSmfNotSupportPsetr => "REL_DUE_TO_SMF_NOT_SUPPORT_PSETR".to_string(),
				Self::RelDueToSnpnSnpnMobility => "REL_DUE_TO_SNPN_SNPN_MOBILITY".to_string(),
				Self::RelDueToNoHrAgreement => "REL_DUE_TO_NO_HR_AGREEMENT".to_string(),
				Self::RelDueToUnspecifiedReason => "REL_DUE_TO_UNSPECIFIED_REASON".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for Cause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REL_DUE_TO_HO" => Ok(Self::RelDueToHo),
				"EPS_FALLBACK" => Ok(Self::EpsFallback),
				"REL_DUE_TO_UP_SEC" => Ok(Self::RelDueToUpSec),
				"DNN_CONGESTION" => Ok(Self::DnnCongestion),
				"S_NSSAI_CONGESTION" => Ok(Self::SNssaiCongestion),
				"REL_DUE_TO_REACTIVATION" => Ok(Self::RelDueToReactivation),
				"5G_AN_NOT_RESPONDING" => Ok(Self::FiveGAnNotResponding),
				"REL_DUE_TO_SLICE_NOT_AVAILABLE" => Ok(Self::RelDueToSliceNotAvailable),
				"REL_DUE_TO_DUPLICATE_SESSION_ID" => Ok(Self::RelDueToDuplicateSessionId),
				"PDU_SESSION_STATUS_MISMATCH" => Ok(Self::PduSessionStatusMismatch),
				"HO_FAILURE" => Ok(Self::HoFailure),
				"INSUFFICIENT_UP_RESOURCES" => Ok(Self::InsufficientUpResources),
				"PDU_SESSION_HANDED_OVER" => Ok(Self::PduSessionHandedOver),
				"PDU_SESSION_RESUMED" => Ok(Self::PduSessionResumed),
				"CN_ASSISTED_RAN_PARAMETER_TUNING" => Ok(Self::CnAssistedRanParameterTuning),
				"ISMF_CONTEXT_TRANSFER" => Ok(Self::IsmfContextTransfer),
				"SMF_CONTEXT_TRANSFER" => Ok(Self::SmfContextTransfer),
				"REL_DUE_TO_PS_TO_CS_HO" => Ok(Self::RelDueToPsToCsHo),
				"REL_DUE_TO_SUBSCRIPTION_CHANGE" => Ok(Self::RelDueToSubscriptionChange),
				"HO_CANCEL" => Ok(Self::HoCancel),
				"REL_DUE_TO_SLICE_NOT_AUTHORIZED" => Ok(Self::RelDueToSliceNotAuthorized),
				"PDU_SESSION_HAND_OVER_FAILURE" => Ok(Self::PduSessionHandOverFailure),
				"DDN_FAILURE_STATUS" => Ok(Self::DdnFailureStatus),
				"REL_DUE_TO_CP_ONLY_NOT_APPLICABLE" => Ok(Self::RelDueToCpOnlyNotApplicable),
				"NOT_SUPPORTED_WITH_ISMF" => Ok(Self::NotSupportedWithIsmf),
				"CHANGED_ANCHOR_SMF" => Ok(Self::ChangedAnchorSmf),
				"CHANGED_INTERMEDIATE_SMF" => Ok(Self::ChangedIntermediateSmf),
				"TARGET_DNAI_NOTIFICATION" => Ok(Self::TargetDnaiNotification),
				"REL_DUE_TO_VPLMN_QOS_FAILURE" => Ok(Self::RelDueToVplmnQosFailure),
				"REL_DUE_TO_SMF_NOT_SUPPORT_PSETR" => Ok(Self::RelDueToSmfNotSupportPsetr),
				"REL_DUE_TO_SNPN_SNPN_MOBILITY" => Ok(Self::RelDueToSnpnSnpnMobility),
				"REL_DUE_TO_NO_HR_AGREEMENT" => Ok(Self::RelDueToNoHrAgreement),
				"REL_DUE_TO_UNSPECIFIED_REASON" => Ok(Self::RelDueToUnspecifiedReason),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for Cause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Cause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Cause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains a Cell Global Identification as defined in 3GPP TS 23.003,
	/// clause 4.3.1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a Cell Global Identification as defined in
	/// 3GPP TS 23.003, clause 4.3.1.",
	///  "type": "object",
	///  "required": [
	///    "cellId",
	///    "lac",
	///    "plmnId"
	///  ],
	///  "properties": {
	///    "cellId": {
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{4}$"
	///    },
	///    "lac": {
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{4}$"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CellGlobalId {
		#[serde(rename = "cellId")]
		pub cell_id: CellGlobalIdCellId,
		pub lac: CellGlobalIdLac,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
	}

	impl From<&CellGlobalId> for CellGlobalId {
		fn from(value: &CellGlobalId) -> Self {
			value.clone()
		}
	}

	/// CellGlobalIdCellId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{4}$"
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
	)]
	pub struct CellGlobalIdCellId(String);

	impl ::std::ops::Deref for CellGlobalIdCellId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<CellGlobalIdCellId> for String {
		fn from(value: CellGlobalIdCellId) -> Self {
			value.0
		}
	}

	impl From<&CellGlobalIdCellId> for CellGlobalIdCellId {
		fn from(value: &CellGlobalIdCellId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for CellGlobalIdCellId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{4}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{4}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for CellGlobalIdCellId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for CellGlobalIdCellId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for CellGlobalIdCellId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CellGlobalIdCellId {
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

	/// CellGlobalIdLac
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{4}$"
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
	)]
	pub struct CellGlobalIdLac(String);

	impl ::std::ops::Deref for CellGlobalIdLac {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<CellGlobalIdLac> for String {
		fn from(value: CellGlobalIdLac) -> Self {
			value.0
		}
	}

	impl From<&CellGlobalIdLac> for CellGlobalIdLac {
		fn from(value: &CellGlobalIdLac) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for CellGlobalIdLac {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{4}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{4}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for CellGlobalIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for CellGlobalIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for CellGlobalIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CellGlobalIdLac {
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

	/// Contains the addresses of the charging functions.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the addresses of the charging functions.",
	///  "type": "object",
	///  "required": [
	///    "primaryChfAddress"
	///  ],
	///  "properties": {
	///    "primaryChfAddress": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "primaryChfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "primaryChfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "secondaryChfAddress": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "secondaryChfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "secondaryChfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ChargingInformation {
		#[serde(rename = "primaryChfAddress")]
		pub primary_chf_address: Uri,
		#[serde(
			rename = "primaryChfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub primary_chf_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "primaryChfSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub primary_chf_set_id: Option<NfSetId>,
		#[serde(
			rename = "secondaryChfAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub secondary_chf_address: Option<Uri>,
		#[serde(
			rename = "secondaryChfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub secondary_chf_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "secondaryChfSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub secondary_chf_set_id: Option<NfSetId>,
	}

	impl From<&ChargingInformation> for ChargingInformation {
		fn from(value: &ChargingInformation) -> Self {
			value.clone()
		}
	}

	/// SMF derived CN assisted RAN parameters tuning
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "SMF derived CN assisted RAN parameters tuning",
	///  "type": "object",
	///  "properties": {
	///    "batteryIndication": {
	///      "$ref": "#/components/schemas/BatteryIndication"
	///    },
	///    "communicationDurationTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "periodicTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "scheduledCommunicationTime": {
	///      "$ref": "#/components/schemas/ScheduledCommunicationTime"
	///    },
	///    "scheduledCommunicationType": {
	///      "$ref": "#/components/schemas/ScheduledCommunicationType"
	///    },
	///    "stationaryIndication": {
	///      "$ref": "#/components/schemas/StationaryIndication"
	///    },
	///    "trafficProfile": {
	///      "$ref": "#/components/schemas/TrafficProfile"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CnAssistedRanPara {
		#[serde(
			rename = "batteryIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub battery_indication: Option<BatteryIndication>,
		#[serde(
			rename = "communicationDurationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub communication_duration_time: Option<DurationSec>,
		#[serde(
			rename = "periodicTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub periodic_time: Option<DurationSec>,
		#[serde(
			rename = "scheduledCommunicationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub scheduled_communication_time: Option<ScheduledCommunicationTime>,
		#[serde(
			rename = "scheduledCommunicationType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub scheduled_communication_type: Option<ScheduledCommunicationType>,
		#[serde(
			rename = "stationaryIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub stationary_indication: Option<StationaryIndication>,
		#[serde(
			rename = "trafficProfile",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub traffic_profile: Option<TrafficProfile>,
	}

	impl From<&CnAssistedRanPara> for CnAssistedRanPara {
		fn from(value: &CnAssistedRanPara) -> Self {
			value.clone()
		}
	}

	/// Describes a communication failure detected by AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes a communication failure detected by AMF",
	///  "type": "object",
	///  "properties": {
	///    "nasReleaseCode": {
	///      "type": "string"
	///    },
	///    "ranReleaseCode": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CommunicationFailure {
		#[serde(
			rename = "nasReleaseCode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nas_release_code: Option<String>,
		#[serde(
			rename = "ranReleaseCode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ran_release_code: Option<NgApCause>,
	}

	impl From<&CommunicationFailure> for CommunicationFailure {
		fn from(value: &CommunicationFailure) -> Self {
			value.clone()
		}
	}

	/// string with format 'date-time' as defined in OpenAPI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'date-time' as defined in OpenAPI.",
	///  "type": "string",
	///  "format": "date-time"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DateTime(pub chrono::DateTime<chrono::offset::Utc>);

	impl ::std::ops::Deref for DateTime {
		type Target = chrono::DateTime<chrono::offset::Utc>;
		fn deref(&self) -> &chrono::DateTime<chrono::offset::Utc> {
			&self.0
		}
	}

	impl From<DateTime> for chrono::DateTime<chrono::offset::Utc> {
		fn from(value: DateTime) -> Self {
			value.0
		}
	}

	impl From<&DateTime> for DateTime {
		fn from(value: &DateTime) -> Self {
			value.clone()
		}
	}

	impl From<chrono::DateTime<chrono::offset::Utc>> for DateTime {
		fn from(value: chrono::DateTime<chrono::offset::Utc>) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for DateTime {
		type Err = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for DateTime {
		type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DateTime {
		type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DateTime {
		type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for DateTime {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// integer between and including 1 and 7 denoting a weekday. 1 shall
	/// indicate Monday, and the subsequent weekdays  shall be indicated with
	/// the next higher numbers. 7 shall indicate Sunday.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "integer between and including 1 and 7 denoting a
	/// weekday. 1 shall indicate Monday, and the subsequent weekdays  shall be
	/// indicated with the next higher numbers. 7 shall indicate Sunday.\n",
	///  "type": "integer",
	///  "maximum": 7.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DayOfWeek(pub i64);

	impl ::std::ops::Deref for DayOfWeek {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<DayOfWeek> for i64 {
		fn from(value: DayOfWeek) -> Self {
			value.0
		}
	}

	impl From<&DayOfWeek> for DayOfWeek {
		fn from(value: &DayOfWeek) -> Self {
			value.clone()
		}
	}

	impl From<i64> for DayOfWeek {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for DayOfWeek {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for DayOfWeek {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DayOfWeek {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DayOfWeek {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for DayOfWeek {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains a Traffic Descriptor.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a Traffic Descriptor.",
	///  "type": "object",
	///  "properties": {
	///    "ipv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "macAddr": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    },
	///    "portNumber": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DddTrafficDescriptor {
		#[serde(rename = "ipv4Addr", default, skip_serializing_if = "Option::is_none")]
		pub ipv4_addr: Option<Ipv4Addr>,
		#[serde(rename = "ipv6Addr", default, skip_serializing_if = "Option::is_none")]
		pub ipv6_addr: Option<Ipv6Addr>,
		#[serde(rename = "macAddr", default, skip_serializing_if = "Option::is_none")]
		pub mac_addr: Option<MacAddr48>,
		#[serde(
			rename = "portNumber",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub port_number: Option<Uinteger>,
	}

	impl From<&DddTrafficDescriptor> for DddTrafficDescriptor {
		fn from(value: &DddTrafficDescriptor) -> Self {
			value.clone()
		}
	}

	/// DDN Failure Subscription Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "DDN Failure Subscription Information",
	///  "type": "object",
	///  "required": [
	///    "notifyCorrelationId"
	///  ],
	///  "properties": {
	///    "dddTrafficDescriptorList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DddTrafficDescriptor"
	///      },
	///      "minItems": 1
	///    },
	///    "notifyCorrelationId": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DdnFailureSubInfo {
		#[serde(
			rename = "dddTrafficDescriptorList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ddd_traffic_descriptor_list: Vec<DddTrafficDescriptor>,
		#[serde(rename = "notifyCorrelationId")]
		pub notify_correlation_id: String,
	}

	impl From<&DdnFailureSubInfo> for DdnFailureSubInfo {
		fn from(value: &DdnFailureSubInfo) -> Self {
			value.clone()
		}
	}

	/// DDN Failure Subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "DDN Failure Subscription",
	///  "type": "object",
	///  "properties": {
	///    "ddnFailureSubsInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ddnFailureSubsInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DdnFailureSubInfo"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DdnFailureSubs {
		#[serde(rename = "ddnFailureSubsInd", default)]
		pub ddn_failure_subs_ind: bool,
		#[serde(
			rename = "ddnFailureSubsInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ddn_failure_subs_info_list: Vec<DdnFailureSubInfo>,
	}

	impl From<&DdnFailureSubs> for DdnFailureSubs {
		fn from(value: &DdnFailureSubs) -> Self {
			value.clone()
		}
	}

	/// Additional information in an error response to a Deliver Request.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Additional information in an error response to a
	/// Deliver Request.",
	///  "type": "object",
	///  "properties": {
	///    "maxWaitingTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DeliverAddInfo {
		#[serde(
			rename = "maxWaitingTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_waiting_time: Option<DurationSec>,
	}

	impl From<&DeliverAddInfo> for DeliverAddInfo {
		fn from(value: &DeliverAddInfo) -> Self {
			value.clone()
		}
	}

	/// Representation of the payload in an error response to a Deliver Request.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Representation of the payload in an error response to a
	/// Deliver Request.",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    {
	///      "$ref": "#/components/schemas/DeliverAddInfo"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DeliverError {
		#[serde(
			rename = "accessTokenError",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_token_error: Option<AccessTokenErr>,
		#[serde(
			rename = "accessTokenRequest",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_token_request: Option<AccessTokenReq>,
		/// A machine-readable application error cause specific to this
		/// occurrence of the problem.  This IE should be present and provide
		/// application-related error information, if available.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<String>,
		/// A human-readable explanation specific to this occurrence of the
		/// problem.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub detail: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub instance: Option<Uri>,
		#[serde(
			rename = "invalidParams",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub invalid_params: Vec<InvalidParam>,
		#[serde(
			rename = "maxWaitingTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_waiting_time: Option<DurationSec>,
		#[serde(rename = "nrfId", default, skip_serializing_if = "Option::is_none")]
		pub nrf_id: Option<Fqdn>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub status: Option<i64>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub title: Option<String>,
		#[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
		pub type_: Option<Uri>,
	}

	impl From<&DeliverError> for DeliverError {
		fn from(value: &DeliverError) -> Self {
			value.clone()
		}
	}

	/// Representation of the payload of a Deliver Request.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Representation of the payload of a Deliver Request.",
	///  "type": "object",
	///  "required": [
	///    "mtData"
	///  ],
	///  "properties": {
	///    "mtData": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DeliverReqData {
		#[serde(rename = "mtData")]
		pub mt_data: RefToBinaryData,
	}

	impl From<&DeliverReqData> for DeliverReqData {
		fn from(value: &DeliverReqData) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - BUFFERED: The first downlink data is buffered with extended buffering
	///   matching the
	///  source of the downlink traffic.
	/// - TRANSMITTED: The first downlink data matching the source of the
	///   downlink traffic is
	///  transmitted after previous buffering or discarding of corresponding
	/// packet(s) because  the UE of the PDU Session becomes ACTIVE, and
	/// buffered data can be delivered to UE.
	/// - DISCARDED: The first downlink data matching the source of the downlink
	///   traffic is
	///  discarded because the Extended Buffering time, as determined by the
	/// SMF, expires or  the amount of downlink data to be buffered is
	/// exceeded.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- BUFFERED: The first downlink
	/// data is buffered with extended buffering matching the\n  source of the
	/// downlink traffic.\n- TRANSMITTED: The first downlink data matching the
	/// source of the downlink traffic is\n  transmitted after previous
	/// buffering or discarding of corresponding packet(s) because\n  the UE of
	/// the PDU Session becomes ACTIVE, and buffered data can be delivered to
	/// UE.\n- DISCARDED: The first downlink data matching the source of the
	/// downlink traffic is\n  discarded because the Extended Buffering time, as
	/// determined by the SMF, expires or\n  the amount of downlink data to be
	/// buffered is exceeded.\n",
	///  "type": "string",
	///  "enum": [
	///    "BUFFERED",
	///    "TRANSMITTED",
	///    "DISCARDED"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum DlDataDeliveryStatus {
		#[default]
		#[serde(rename = "BUFFERED")]
		Buffered,
		#[serde(rename = "TRANSMITTED")]
		Transmitted,
		#[serde(rename = "DISCARDED")]
		Discarded,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DlDataDeliveryStatus> for DlDataDeliveryStatus {
		fn from(value: &DlDataDeliveryStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for DlDataDeliveryStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Buffered => "BUFFERED".to_string(),
				Self::Transmitted => "TRANSMITTED".to_string(),
				Self::Discarded => "DISCARDED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DlDataDeliveryStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"BUFFERED" => Ok(Self::Buffered),
				"TRANSMITTED" => Ok(Self::Transmitted),
				"DISCARDED" => Ok(Self::Discarded),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DlDataDeliveryStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DlDataDeliveryStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DlDataDeliveryStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// DNAI (Data network access identifier), see clause 5.6.7 of 3GPP TS
	/// 23.501.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "DNAI (Data network access identifier), see clause 5.6.7
	/// of 3GPP TS 23.501.",
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
	pub struct Dnai(pub String);

	impl ::std::ops::Deref for Dnai {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Dnai> for String {
		fn from(value: Dnai) -> Self {
			value.0
		}
	}

	impl From<&Dnai> for Dnai {
		fn from(value: &Dnai) -> Self {
			value.clone()
		}
	}

	impl From<String> for Dnai {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Dnai {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for Dnai {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Possible values are:
	/// - EARLY: Early notification of UP path reconfiguration.
	/// - EARLY_LATE: Early and late notification of UP path reconfiguration.
	///   This value shall
	///  only be present in the subscription to the DNAI change event.
	/// - LATE: Late notification of UP path reconfiguration.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- EARLY: Early notification of UP
	/// path reconfiguration.\n- EARLY_LATE: Early and late notification of UP
	/// path reconfiguration. This value shall\n  only be present in the
	/// subscription to the DNAI change event.\n- LATE: Late notification of UP
	/// path reconfiguration. \n",
	///  "type": "string",
	///  "enum": [
	///    "EARLY",
	///    "EARLY_LATE",
	///    "LATE"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum DnaiChangeType {
		#[default]
		#[serde(rename = "EARLY")]
		Early,
		#[serde(rename = "EARLY_LATE")]
		EarlyLate,
		#[serde(rename = "LATE")]
		Late,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DnaiChangeType> for DnaiChangeType {
		fn from(value: &DnaiChangeType) -> Self {
			value.clone()
		}
	}

	impl ToString for DnaiChangeType {
		fn to_string(&self) -> String {
			match *self {
				Self::Early => "EARLY".to_string(),
				Self::EarlyLate => "EARLY_LATE".to_string(),
				Self::Late => "LATE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DnaiChangeType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EARLY" => Ok(Self::Early),
				"EARLY_LATE" => Ok(Self::EarlyLate),
				"LATE" => Ok(Self::Late),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DnaiChangeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DnaiChangeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DnaiChangeType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// DNAI Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "DNAI Information",
	///  "type": "object",
	///  "required": [
	///    "dnai"
	///  ],
	///  "properties": {
	///    "dnai": {
	///      "$ref": "#/components/schemas/Dnai"
	///    },
	///    "noDnaiChangeInd": {
	///      "type": "boolean"
	///    },
	///    "noLocalPsaChangeInd": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnaiInformation {
		pub dnai: Dnai,
		#[serde(
			rename = "noDnaiChangeInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub no_dnai_change_ind: Option<bool>,
		#[serde(
			rename = "noLocalPsaChangeInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub no_local_psa_change_ind: Option<bool>,
	}

	impl From<&DnaiInformation> for DnaiInformation {
		fn from(value: &DnaiInformation) -> Self {
			value.clone()
		}
	}

	/// String representing a Data Network as defined in clause 9A of 3GPP TS
	/// 23.003;  it shall contain either a DNN Network Identifier, or a full DNN
	/// with both the Network  Identifier and Operator Identifier, as specified
	/// in 3GPP TS 23.003 clause 9.1.1 and 9.1.2. It shall be coded as string in
	/// which the labels are separated by dots  (e.g. "Label1.Label2.Label3").
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing a Data Network as defined in clause
	/// 9A of 3GPP TS 23.003;  it shall contain either a DNN Network Identifier,
	/// or a full DNN with both the Network  Identifier and Operator Identifier,
	/// as specified in 3GPP TS 23.003 clause 9.1.1 and 9.1.2. It shall be coded
	/// as string in which the labels are separated by dots  (e.g.
	/// \"Label1.Label2.Label3\").\n",
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
	pub struct Dnn(pub String);

	impl ::std::ops::Deref for Dnn {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Dnn> for String {
		fn from(value: Dnn) -> Self {
			value.0
		}
	}

	impl From<&Dnn> for Dnn {
		fn from(value: &Dnn) -> Self {
			value.clone()
		}
	}

	impl From<String> for Dnn {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Dnn {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for Dnn {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// DNN Selection Mode. Possible values are
	/// - VERIFIED
	/// - UE_DNN_NOT_VERIFIED
	/// - NW_DNN_NOT_VERIFIED
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "DNN Selection Mode. Possible values are\n- VERIFIED\n-
	/// UE_DNN_NOT_VERIFIED\n- NW_DNN_NOT_VERIFIED\n",
	///  "type": "string",
	///  "enum": [
	///    "VERIFIED",
	///    "UE_DNN_NOT_VERIFIED",
	///    "NW_DNN_NOT_VERIFIED"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum DnnSelectionMode {
		#[default]
		#[serde(rename = "VERIFIED")]
		Verified,
		#[serde(rename = "UE_DNN_NOT_VERIFIED")]
		UeDnnNotVerified,
		#[serde(rename = "NW_DNN_NOT_VERIFIED")]
		NwDnnNotVerified,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DnnSelectionMode> for DnnSelectionMode {
		fn from(value: &DnnSelectionMode) -> Self {
			value.clone()
		}
	}

	impl ToString for DnnSelectionMode {
		fn to_string(&self) -> String {
			match *self {
				Self::Verified => "VERIFIED".to_string(),
				Self::UeDnnNotVerified => "UE_DNN_NOT_VERIFIED".to_string(),
				Self::NwDnnNotVerified => "NW_DNN_NOT_VERIFIED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DnnSelectionMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"VERIFIED" => Ok(Self::Verified),
				"UE_DNN_NOT_VERIFIED" => Ok(Self::UeDnnNotVerified),
				"NW_DNN_NOT_VERIFIED" => Ok(Self::NwDnnNotVerified),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DnnSelectionMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DnnSelectionMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DnnSelectionMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data Radio Bearer Identity
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data Radio Bearer Identity",
	///  "type": "integer",
	///  "maximum": 32.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DrbId(pub i64);

	impl ::std::ops::Deref for DrbId {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<DrbId> for i64 {
		fn from(value: DrbId) -> Self {
			value.0
		}
	}

	impl From<&DrbId> for DrbId {
		fn from(value: &DrbId) -> Self {
			value.clone()
		}
	}

	impl From<i64> for DrbId {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for DrbId {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for DrbId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DrbId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DrbId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for DrbId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// indicating a time in seconds.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "indicating a time in seconds.",
	///  "type": "integer"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DurationSec(pub i64);

	impl ::std::ops::Deref for DurationSec {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<DurationSec> for i64 {
		fn from(value: DurationSec) -> Self {
			value.0
		}
	}

	impl From<&DurationSec> for DurationSec {
		fn from(value: &DurationSec) -> Self {
			value.clone()
		}
	}

	impl From<i64> for DurationSec {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for DurationSec {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for DurationSec {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DurationSec {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DurationSec {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for DurationSec {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// It indicates the QoS Characteristics for a Non-standardised or not
	/// pre-configured 5QI for downlink and uplink.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It indicates the QoS Characteristics for a Non-standardised or not pre-configured 5QI for downlink and uplink.\n",
	///  "type": "object",
	///  "required": [
	///    "packetDelayBudget",
	///    "packetErrRate",
	///    "priorityLevel",
	///    "resourceType"
	///  ],
	///  "properties": {
	///    "averWindow": {
	///      "$ref": "#/components/schemas/AverWindow"
	///    },
	///    "cnPacketDelayBudgetDl": {
	///      "$ref": "#/components/schemas/ExtPacketDelBudget"
	///    },
	///    "cnPacketDelayBudgetUl": {
	///      "$ref": "#/components/schemas/ExtPacketDelBudget"
	///    },
	///    "extMaxDataBurstVol": {
	///      "$ref": "#/components/schemas/ExtMaxDataBurstVol"
	///    },
	///    "extPacketDelBudget": {
	///      "$ref": "#/components/schemas/ExtPacketDelBudget"
	///    },
	///    "maxDataBurstVol": {
	///      "$ref": "#/components/schemas/MaxDataBurstVol"
	///    },
	///    "packetDelayBudget": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "packetErrRate": {
	///      "$ref": "#/components/schemas/PacketErrRate"
	///    },
	///    "priorityLevel": {
	///      "$ref": "#/components/schemas/5QiPriorityLevel"
	///    },
	///    "resourceType": {
	///      "$ref": "#/components/schemas/QosResourceType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Dynamic5Qi {
		#[serde(
			rename = "averWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub aver_window: Option<AverWindow>,
		#[serde(
			rename = "cnPacketDelayBudgetDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cn_packet_delay_budget_dl: Option<ExtPacketDelBudget>,
		#[serde(
			rename = "cnPacketDelayBudgetUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cn_packet_delay_budget_ul: Option<ExtPacketDelBudget>,
		#[serde(
			rename = "extMaxDataBurstVol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ext_max_data_burst_vol: Option<ExtMaxDataBurstVol>,
		#[serde(
			rename = "extPacketDelBudget",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ext_packet_del_budget: Option<ExtPacketDelBudget>,
		#[serde(
			rename = "maxDataBurstVol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_data_burst_vol: Option<MaxDataBurstVol>,
		#[serde(rename = "packetDelayBudget")]
		pub packet_delay_budget: PacketDelBudget,
		#[serde(rename = "packetErrRate")]
		pub packet_err_rate: PacketErrRate,
		#[serde(rename = "priorityLevel")]
		pub priority_level: _5qiPriorityLevel,
		#[serde(rename = "resourceType")]
		pub resource_type: QosResourceType,
	}

	impl From<&Dynamic5Qi> for Dynamic5Qi {
		fn from(value: &Dynamic5Qi) -> Self {
			value.clone()
		}
	}

	/// This represents the identifier of the eNB ID as specified in clause
	/// 9.2.1.37 of  3GPP TS 36.413. The string shall be formatted with the
	/// following pattern
	/// '^('MacroeNB-[A-Fa-f0-9]{5}|LMacroeNB-[A-Fa-f0-9]{6}|SMacroeNB-[A-Fa-f0-9]{5}
	/// |HomeeNB-[A-Fa-f0-9]{7})$'. The value of the eNB ID shall be encoded in
	/// hexadecimal representation. Each character in the  string shall take a
	/// value of "0" to "9", "a" to "f" or "A" to "F" and shall represent 4
	/// bits.  The padding 0 shall be added to make multiple nibbles, so the
	/// most significant character  representing the padding 0 if required
	/// together with the 4 most significant bits of the eNB ID  shall appear
	/// first in the string, and the character representing the 4 least
	/// significant bit  of the eNB ID (to form a nibble) shall appear last in
	/// the string.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This represents the identifier of the eNB ID as
	/// specified in clause 9.2.1.37 of  3GPP TS 36.413. The string shall be
	/// formatted with the following pattern
	/// '^('MacroeNB-[A-Fa-f0-9]{5}|LMacroeNB-[A-Fa-f0-9]{6}|SMacroeNB-[A-Fa-f0-9]{5}
	/// |HomeeNB-[A-Fa-f0-9]{7})$'. The value of the eNB ID shall be encoded in
	/// hexadecimal representation. Each character in the  string shall take a
	/// value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall
	/// represent 4 bits.  The padding 0 shall be added to make multiple
	/// nibbles, so the most significant character  representing the padding 0
	/// if required together with the 4 most significant bits of the eNB ID
	/// shall appear first in the string, and the character representing the 4
	/// least significant bit  of the eNB ID (to form a nibble) shall appear
	/// last in the string.\n",
	///  "type": "string",
	///  "pattern":
	/// "^(MacroeNB-[A-Fa-f0-9]{5}|LMacroeNB-[A-Fa-f0-9]{6}|SMacroeNB-[A-Fa-f0-9]{5}|HomeeNB-[A-Fa-f0-9]{7})$"
	///
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
	)]
	pub struct ENbId(String);

	impl ::std::ops::Deref for ENbId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ENbId> for String {
		fn from(value: ENbId) -> Self {
			value.0
		}
	}

	impl From<&ENbId> for ENbId {
		fn from(value: &ENbId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ENbId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^(MacroeNB-[A-Fa-f0-9]{5}|LMacroeNB-[A-Fa-f0-9]{6}|SMacroeNB-[A-Fa-f0-9]{5}|HomeeNB-[A-Fa-f0-9]{7})$").unwrap().find(value).is_none() { return Err("doesn't match pattern \"^(MacroeNB-[A-Fa-f0-9]{5}|LMacroeNB-[A-Fa-f0-9]{6}|SMacroeNB-[A-Fa-f0-9]{5}|HomeeNB-[A-Fa-f0-9]{7})$\"".into()); }
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for ENbId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ENbId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ENbId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ENbId {
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

	/// Contains EAS IP replacement information for a Source and a Target EAS.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains EAS IP replacement information for a Source
	/// and a Target EAS.",
	///  "type": "object",
	///  "required": [
	///    "source",
	///    "target"
	///  ],
	///  "properties": {
	///    "source": {
	///      "$ref": "#/components/schemas/EasServerAddress"
	///    },
	///    "target": {
	///      "$ref": "#/components/schemas/EasServerAddress"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EasIpReplacementInfo {
		pub source: EasServerAddress,
		pub target: EasServerAddress,
	}

	impl From<&EasIpReplacementInfo> for EasIpReplacementInfo {
		fn from(value: &EasIpReplacementInfo) -> Self {
			value.clone()
		}
	}

	/// Represents the IP address and port of an EAS server.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the IP address and port of an EAS server.",
	///  "type": "object",
	///  "required": [
	///    "ip",
	///    "port"
	///  ],
	///  "properties": {
	///    "ip": {
	///      "$ref": "#/components/schemas/IpAddr"
	///    },
	///    "port": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EasServerAddress {
		pub ip: IpAddr,
		pub port: Uinteger,
	}

	impl From<&EasServerAddress> for EasServerAddress {
		fn from(value: &EasServerAddress) -> Self {
			value.clone()
		}
	}

	/// EBI to ARP mapping
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "EBI to ARP mapping",
	///  "type": "object",
	///  "required": [
	///    "arp",
	///    "epsBearerId"
	///  ],
	///  "properties": {
	///    "arp": {
	///      "$ref": "#/components/schemas/Arp"
	///    },
	///    "epsBearerId": {
	///      "$ref": "#/components/schemas/EpsBearerId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EbiArpMapping {
		pub arp: Arp,
		#[serde(rename = "epsBearerId")]
		pub eps_bearer_id: EpsBearerId,
	}

	impl From<&EbiArpMapping> for EbiArpMapping {
		fn from(value: &EbiArpMapping) -> Self {
			value.clone()
		}
	}

	/// Contains the ECGI (E-UTRAN Cell Global Identity), as described in 3GPP
	/// 23.003
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the ECGI (E-UTRAN Cell Global Identity), as
	/// described in 3GPP 23.003",
	///  "type": "object",
	///  "required": [
	///    "eutraCellId",
	///    "plmnId"
	///  ],
	///  "properties": {
	///    "eutraCellId": {
	///      "$ref": "#/components/schemas/EutraCellId"
	///    },
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Ecgi {
		#[serde(rename = "eutraCellId")]
		pub eutra_cell_id: EutraCellId,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
	}

	impl From<&Ecgi> for Ecgi {
		fn from(value: &Ecgi) -> Self {
			value.clone()
		}
	}

	/// EPS Bearer container from SMF to AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "EPS Bearer container from SMF to AMF",
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
	pub struct EpsBearerContainer(pub String);

	impl ::std::ops::Deref for EpsBearerContainer {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<EpsBearerContainer> for String {
		fn from(value: EpsBearerContainer) -> Self {
			value.0
		}
	}

	impl From<&EpsBearerContainer> for EpsBearerContainer {
		fn from(value: &EpsBearerContainer) -> Self {
			value.clone()
		}
	}

	impl From<String> for EpsBearerContainer {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for EpsBearerContainer {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for EpsBearerContainer {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// EPS bearer context status
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "EPS bearer context status",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{4}$"
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
	)]
	pub struct EpsBearerContextStatus(String);

	impl ::std::ops::Deref for EpsBearerContextStatus {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<EpsBearerContextStatus> for String {
		fn from(value: EpsBearerContextStatus) -> Self {
			value.0
		}
	}

	impl From<&EpsBearerContextStatus> for EpsBearerContextStatus {
		fn from(value: &EpsBearerContextStatus) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for EpsBearerContextStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{4}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{4}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for EpsBearerContextStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for EpsBearerContextStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for EpsBearerContextStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for EpsBearerContextStatus {
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

	/// EPS Bearer Identifier
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "EPS Bearer Identifier",
	///  "type": "integer",
	///  "maximum": 15.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EpsBearerId(pub i64);

	impl ::std::ops::Deref for EpsBearerId {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<EpsBearerId> for i64 {
		fn from(value: EpsBearerId) -> Self {
			value.0
		}
	}

	impl From<&EpsBearerId> for EpsBearerId {
		fn from(value: &EpsBearerId) -> Self {
			value.clone()
		}
	}

	impl From<i64> for EpsBearerId {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for EpsBearerId {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for EpsBearerId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for EpsBearerId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for EpsBearerId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for EpsBearerId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// EPS Bearer Information from H-SMF to V-SMF, or from SMF to I-SMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "EPS Bearer Information from H-SMF to V-SMF, or from SMF
	/// to I-SMF",
	///  "type": "object",
	///  "required": [
	///    "bearerLevelQoS",
	///    "ebi",
	///    "pgwS8uFteid"
	///  ],
	///  "properties": {
	///    "bearerLevelQoS": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "ebi": {
	///      "$ref": "#/components/schemas/EpsBearerId"
	///    },
	///    "pgwS8uFteid": {
	///      "$ref": "#/components/schemas/Bytes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EpsBearerInfo {
		#[serde(rename = "bearerLevelQoS")]
		pub bearer_level_qo_s: Bytes,
		pub ebi: EpsBearerId,
		#[serde(rename = "pgwS8uFteid")]
		pub pgw_s8u_fteid: Bytes,
	}

	impl From<&EpsBearerInfo> for EpsBearerInfo {
		fn from(value: &EpsBearerInfo) -> Self {
			value.clone()
		}
	}

	/// EPS Interworking Indication. Possible values are
	/// - NONE
	/// - WITH_N26
	/// - WITHOUT_N26
	/// - IWK_NON_3GPP
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "EPS Interworking Indication. Possible values are\n-
	/// NONE\n- WITH_N26\n- WITHOUT_N26\n- IWK_NON_3GPP\n",
	///  "type": "string",
	///  "enum": [
	///    "NONE",
	///    "WITH_N26",
	///    "WITHOUT_N26",
	///    "IWK_NON_3GPP"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum EpsInterworkingIndication {
		#[default]
		#[serde(rename = "NONE")]
		None,
		#[serde(rename = "WITH_N26")]
		WithN26,
		#[serde(rename = "WITHOUT_N26")]
		WithoutN26,
		#[serde(rename = "IWK_NON_3GPP")]
		IwkNon3gpp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&EpsInterworkingIndication> for EpsInterworkingIndication {
		fn from(value: &EpsInterworkingIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for EpsInterworkingIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::None => "NONE".to_string(),
				Self::WithN26 => "WITH_N26".to_string(),
				Self::WithoutN26 => "WITHOUT_N26".to_string(),
				Self::IwkNon3gpp => "IWK_NON_3GPP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for EpsInterworkingIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NONE" => Ok(Self::None),
				"WITH_N26" => Ok(Self::WithN26),
				"WITHOUT_N26" => Ok(Self::WithoutN26),
				"IWK_NON_3GPP" => Ok(Self::IwkNon3gpp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for EpsInterworkingIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for EpsInterworkingIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for EpsInterworkingIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// UE EPS PDN Connection container from SMF to AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "UE EPS PDN Connection container from SMF to AMF",
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
	pub struct EpsPdnCnxContainer(pub String);

	impl ::std::ops::Deref for EpsPdnCnxContainer {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<EpsPdnCnxContainer> for String {
		fn from(value: EpsPdnCnxContainer) -> Self {
			value.0
		}
	}

	impl From<&EpsPdnCnxContainer> for EpsPdnCnxContainer {
		fn from(value: &EpsPdnCnxContainer) -> Self {
			value.clone()
		}
	}

	impl From<String> for EpsPdnCnxContainer {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for EpsPdnCnxContainer {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for EpsPdnCnxContainer {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// EPS PDN Connection Information from H-SMF to V-SMF, or from SMF to I-SMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "EPS PDN Connection Information from H-SMF to V-SMF, or
	/// from SMF to I-SMF",
	///  "type": "object",
	///  "required": [
	///    "pgwS8cFteid"
	///  ],
	///  "properties": {
	///    "linkedBearerId": {
	///      "$ref": "#/components/schemas/EpsBearerId"
	///    },
	///    "pgwNodeName": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "pgwS8cFteid": {
	///      "$ref": "#/components/schemas/Bytes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EpsPdnCnxInfo {
		#[serde(
			rename = "linkedBearerId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub linked_bearer_id: Option<EpsBearerId>,
		#[serde(
			rename = "pgwNodeName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pgw_node_name: Option<Bytes>,
		#[serde(rename = "pgwS8cFteid")]
		pub pgw_s8c_fteid: Bytes,
	}

	impl From<&EpsPdnCnxInfo> for EpsPdnCnxInfo {
		fn from(value: &EpsPdnCnxInfo) -> Self {
			value.clone()
		}
	}

	/// Identifies an Ethernet flow.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies an Ethernet flow.",
	///  "type": "object",
	///  "required": [
	///    "ethType"
	///  ],
	///  "properties": {
	///    "destMacAddr": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    },
	///    "destMacAddrEnd": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    },
	///    "ethType": {
	///      "type": "string"
	///    },
	///    "fDesc": {
	///      "$ref": "#/components/schemas/FlowDescription"
	///    },
	///    "fDir": {
	///      "$ref": "#/components/schemas/FlowDirection"
	///    },
	///    "sourceMacAddr": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    },
	///    "srcMacAddrEnd": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    },
	///    "vlanTags": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EthFlowDescription {
		#[serde(
			rename = "destMacAddr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dest_mac_addr: Option<MacAddr48>,
		#[serde(
			rename = "destMacAddrEnd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dest_mac_addr_end: Option<MacAddr48>,
		#[serde(rename = "ethType")]
		pub eth_type: String,
		#[serde(rename = "fDesc", default, skip_serializing_if = "Option::is_none")]
		pub f_desc: Option<FlowDescription>,
		#[serde(rename = "fDir", default, skip_serializing_if = "Option::is_none")]
		pub f_dir: Option<FlowDirection>,
		#[serde(
			rename = "sourceMacAddr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_mac_addr: Option<MacAddr48>,
		#[serde(
			rename = "srcMacAddrEnd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub src_mac_addr_end: Option<MacAddr48>,
		#[serde(rename = "vlanTags", default, skip_serializing_if = "Vec::is_empty")]
		pub vlan_tags: Vec<String>,
	}

	impl From<&EthFlowDescription> for EthFlowDescription {
		fn from(value: &EthFlowDescription) -> Self {
			value.clone()
		}
	}

	/// 28-bit string identifying an E-UTRA Cell Id as specified in clause
	/// 9.3.1.9 of  3GPP TS 38.413, in hexadecimal representation. Each
	/// character in the string shall take a  value of "0" to "9", "a" to "f" or
	/// "A" to "F" and shall represent 4 bits. The most  significant character
	/// representing the 4 most significant bits of the Cell Id shall appear
	/// first in the string, and the character representing the 4 least
	/// significant bit of the  Cell Id shall appear last in the string.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "28-bit string identifying an E-UTRA Cell Id as
	/// specified in clause 9.3.1.9 of  3GPP TS 38.413, in hexadecimal
	/// representation. Each character in the string shall take a  value of
	/// \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4
	/// bits. The most  significant character representing the 4 most
	/// significant bits of the Cell Id shall appear  first in the string, and
	/// the character representing the 4 least significant bit of the  Cell Id
	/// shall appear last in the string. \n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{7}$"
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
	)]
	pub struct EutraCellId(String);

	impl ::std::ops::Deref for EutraCellId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<EutraCellId> for String {
		fn from(value: EutraCellId) -> Self {
			value.0
		}
	}

	impl From<&EutraCellId> for EutraCellId {
		fn from(value: &EutraCellId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for EutraCellId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{7}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{7}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for EutraCellId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for EutraCellId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for EutraCellId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for EutraCellId {
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

	/// Contains the E-UTRA user location.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the E-UTRA user location.",
	///  "type": "object",
	///  "required": [
	///    "ecgi",
	///    "tai"
	///  ],
	///  "properties": {
	///    "ageOfLocationInformation": {
	///      "description": "The value represents the elapsed time in minutes
	/// since the last network contact of the mobile station.  Value \"0\"
	/// indicates that the location information was obtained after a successful
	/// paging procedure for Active Location Retrieval when the UE is in idle
	/// mode or after a successful NG-RAN location reporting procedure with the
	/// eNB when the UE is in connected mode.  Any other value than \"0\"
	/// indicates that the location information is the last known one.  See 3GPP
	/// TS 29.002 clause 17.7.8.\n",
	///      "type": "integer",
	///      "maximum": 32767.0,
	///      "minimum": 0.0
	///    },
	///    "ecgi": {
	///      "$ref": "#/components/schemas/Ecgi"
	///    },
	///    "geodeticInformation": {
	///      "description": "Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763 (1999) [24] clause 3.88.2. Only the description of an ellipsoid point with uncertainty circle is allowed to be used.\n",
	///      "type": "string",
	///      "pattern": "^[0-9A-F]{20}$"
	///    },
	///    "geographicalInformation": {
	///      "description": "Refer to geographical Information. See 3GPP TS
	/// 23.032 clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///      "type": "string",
	///      "pattern": "^[0-9A-F]{16}$"
	///    },
	///    "globalENbId": {
	///      "$ref": "#/components/schemas/GlobalRanNodeId"
	///    },
	///    "globalNgenbId": {
	///      "$ref": "#/components/schemas/GlobalRanNodeId"
	///    },
	///    "ignoreEcgi": {
	///      "description": "This flag when present shall indicate that the Ecgi
	/// shall be ignored When present, it shall be set as follows: - true: ecgi
	/// shall be ignored. - false (default): ecgi shall not be ignored.\n",
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ignoreTai": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "tai": {
	///      "$ref": "#/components/schemas/Tai"
	///    },
	///    "ueLocationTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EutraLocation {
		/// The value represents the elapsed time in minutes since the last
		/// network contact of the mobile station.  Value "0" indicates that the
		/// location information was obtained after a successful paging
		/// procedure for Active Location Retrieval when the UE is in idle mode
		/// or after a successful NG-RAN location reporting procedure with the
		/// eNB when the UE is in connected mode.  Any other value than "0"
		/// indicates that the location information is the last known one.  See
		/// 3GPP TS 29.002 clause 17.7.8.
		#[serde(
			rename = "ageOfLocationInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub age_of_location_information: Option<i64>,
		pub ecgi: Ecgi,
		/// Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763
		/// (1999) [24] clause 3.88.2. Only the description of an ellipsoid
		/// point with uncertainty circle is allowed to be used.
		#[serde(
			rename = "geodeticInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub geodetic_information: Option<EutraLocationGeodeticInformation>,
		/// Refer to geographical Information. See 3GPP TS 23.032 clause 7.3.2.
		/// Only the description of an ellipsoid point with uncertainty circle
		/// is allowed to be used.
		#[serde(
			rename = "geographicalInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub geographical_information: Option<EutraLocationGeographicalInformation>,
		#[serde(
			rename = "globalENbId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub global_e_nb_id: Option<GlobalRanNodeId>,
		#[serde(
			rename = "globalNgenbId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub global_ngenb_id: Option<GlobalRanNodeId>,
		/// This flag when present shall indicate that the Ecgi shall be ignored
		/// When present, it shall be set as follows: - true: ecgi shall be
		/// ignored. - false (default): ecgi shall not be ignored.
		#[serde(rename = "ignoreEcgi", default)]
		pub ignore_ecgi: bool,
		#[serde(rename = "ignoreTai", default)]
		pub ignore_tai: bool,
		pub tai: Tai,
		#[serde(
			rename = "ueLocationTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_location_timestamp: Option<DateTime>,
	}

	impl From<&EutraLocation> for EutraLocation {
		fn from(value: &EutraLocation) -> Self {
			value.clone()
		}
	}

	/// Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763
	/// (1999) [24] clause 3.88.2. Only the description of an ellipsoid point
	/// with uncertainty circle is allowed to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763 (1999) [24] clause 3.88.2. Only the description of an ellipsoid point with uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{20}$"
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
	)]
	pub struct EutraLocationGeodeticInformation(String);

	impl ::std::ops::Deref for EutraLocationGeodeticInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<EutraLocationGeodeticInformation> for String {
		fn from(value: EutraLocationGeodeticInformation) -> Self {
			value.0
		}
	}

	impl From<&EutraLocationGeodeticInformation> for EutraLocationGeodeticInformation {
		fn from(value: &EutraLocationGeodeticInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for EutraLocationGeodeticInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{20}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{20}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for EutraLocationGeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for EutraLocationGeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for EutraLocationGeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for EutraLocationGeodeticInformation {
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

	/// Refer to geographical Information. See 3GPP TS 23.032 clause 7.3.2. Only
	/// the description of an ellipsoid point with uncertainty circle is allowed
	/// to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refer to geographical Information. See 3GPP TS 23.032
	/// clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{16}$"
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
	)]
	pub struct EutraLocationGeographicalInformation(String);

	impl ::std::ops::Deref for EutraLocationGeographicalInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<EutraLocationGeographicalInformation> for String {
		fn from(value: EutraLocationGeographicalInformation) -> Self {
			value.0
		}
	}

	impl From<&EutraLocationGeographicalInformation> for EutraLocationGeographicalInformation {
		fn from(value: &EutraLocationGeographicalInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for EutraLocationGeographicalInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{16}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{16}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for EutraLocationGeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for EutraLocationGeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for EutraLocationGeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for EutraLocationGeographicalInformation {
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

	/// Represents a notification related to a single event that occurred.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a notification related to a single event
	/// that occurred.",
	///  "type": "object",
	///  "required": [
	///    "event",
	///    "timeStamp"
	///  ],
	///  "properties": {
	///    "accType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "adIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "adIpv6Prefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "appId": {
	///      "$ref": "#/components/schemas/ApplicationId"
	///    },
	///    "bssId": {
	///      "type": "string"
	///    },
	///    "commFailure": {
	///      "$ref": "#/components/schemas/CommunicationFailure"
	///    },
	///    "dddStatus": {
	///      "$ref": "#/components/schemas/DlDataDeliveryStatus"
	///    },
	///    "dddTraDescriptor": {
	///      "$ref": "#/components/schemas/DddTrafficDescriptor"
	///    },
	///    "dlDelays": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uinteger"
	///      },
	///      "minItems": 1
	///    },
	///    "dnaiChgType": {
	///      "$ref": "#/components/schemas/DnaiChangeType"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "endWlan": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "ethFlowDescs": {
	///      "description": "Descriptor(s) for non-IP traffic. It allows the
	/// encoding of multiple UL and/or DL flows. Each entry of the array
	/// describes a single Ethernet flow.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EthFlowDescription"
	///      },
	///      "minItems": 1
	///    },
	///    "ethfDescs": {
	///      "description": "Contains the UL and/or DL Ethernet flows. Each
	/// entry of the array describes a single Ethernet flow.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EthFlowDescription"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "event": {
	///      "$ref": "#/components/schemas/SmfEvent"
	///    },
	///    "fDescs": {
	///      "description": "Contains the UL and/or DL IP flows. Each entry of
	/// the array describes a single IP flow.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/FlowDescription"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "flowDescs": {
	///      "description": "Descriptor(s) for IP traffic. It allows the
	/// encoding of multiple UL and/or DL flows. Each entry of the array
	/// describes a single IP flow.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/FlowDescription"
	///      },
	///      "minItems": 1
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "ipv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6Addrs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6Prefixes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Prefix"
	///      },
	///      "minItems": 1
	///    },
	///    "maxWaitTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "pdmf": {
	///      "description": "Represents the packet delay measurement failure
	/// indicator.",
	///      "type": "boolean"
	///    },
	///    "pduSeId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "pduSessInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "pduSessType": {
	///      "$ref": "#/components/schemas/PduSessionType"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "qfi": {
	///      "$ref": "#/components/schemas/Qfi"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "reIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "reIpv6Prefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "rtDelays": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uinteger"
	///      },
	///      "minItems": 1
	///    },
	///    "smNasFromSmf": {
	///      "$ref": "#/components/schemas/SmNasFromSmf"
	///    },
	///    "smNasFromUe": {
	///      "$ref": "#/components/schemas/SmNasFromUe"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "sourceDnai": {
	///      "$ref": "#/components/schemas/Dnai"
	///    },
	///    "sourceTraRouting": {
	///      "$ref": "#/components/schemas/RouteToLocation"
	///    },
	///    "sourceUeIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "sourceUeIpv6Prefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "ssId": {
	///      "type": "string"
	///    },
	///    "startWlan": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "targetDnai": {
	///      "$ref": "#/components/schemas/Dnai"
	///    },
	///    "targetTraRouting": {
	///      "$ref": "#/components/schemas/RouteToLocation"
	///    },
	///    "targetUeIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "targetUeIpv6Prefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "timeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "timeWindow": {
	///      "$ref": "#/components/schemas/TimeWindow"
	///    },
	///    "transacInfos": {
	///      "description": "Transaction Information.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TransactionInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "ueIpAddr": {
	///      "$ref": "#/components/schemas/IpAddr"
	///    },
	///    "ueMac": {
	///      "$ref": "#/components/schemas/MacAddr48"
	///    },
	///    "ulDelays": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uinteger"
	///      },
	///      "minItems": 1
	///    },
	///    "upRedTrans": {
	///      "description": "Indicates whether the redundant transmission is
	/// setup or terminated. Set to \"true\" if  the redundant transmission is
	/// setup, otherwise set to \"false\" if the redundant  transmission is
	/// terminated. Default value is set to \"false\".\n",
	///      "type": "boolean"
	///    },
	///    "upfInfo": {
	///      "$ref": "#/components/schemas/UpfInformation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EventNotification {
		#[serde(rename = "accType", default, skip_serializing_if = "Option::is_none")]
		pub acc_type: Option<AccessType>,
		#[serde(
			rename = "adIpv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ad_ipv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "adIpv6Prefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ad_ipv6_prefix: Option<Ipv6Prefix>,
		#[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
		pub app_id: Option<ApplicationId>,
		#[serde(rename = "bssId", default, skip_serializing_if = "Option::is_none")]
		pub bss_id: Option<String>,
		#[serde(
			rename = "commFailure",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub comm_failure: Option<CommunicationFailure>,
		#[serde(rename = "dddStatus", default, skip_serializing_if = "Option::is_none")]
		pub ddd_status: Option<DlDataDeliveryStatus>,
		#[serde(
			rename = "dddTraDescriptor",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ddd_tra_descriptor: Option<DddTrafficDescriptor>,
		#[serde(rename = "dlDelays", default, skip_serializing_if = "Vec::is_empty")]
		pub dl_delays: Vec<Uinteger>,
		#[serde(
			rename = "dnaiChgType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dnai_chg_type: Option<DnaiChangeType>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(rename = "endWlan", default, skip_serializing_if = "Option::is_none")]
		pub end_wlan: Option<DateTime>,
		/// Descriptor(s) for non-IP traffic. It allows the encoding of multiple
		/// UL and/or DL flows. Each entry of the array describes a single
		/// Ethernet flow.
		#[serde(
			rename = "ethFlowDescs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub eth_flow_descs: Vec<EthFlowDescription>,
		/// Contains the UL and/or DL Ethernet flows. Each entry of the array
		/// describes a single Ethernet flow.
		#[serde(rename = "ethfDescs", default, skip_serializing_if = "Vec::is_empty")]
		pub ethf_descs: Vec<EthFlowDescription>,
		pub event: SmfEvent,
		/// Contains the UL and/or DL IP flows. Each entry of the array
		/// describes a single IP flow.
		#[serde(rename = "fDescs", default, skip_serializing_if = "Vec::is_empty")]
		pub f_descs: Vec<FlowDescription>,
		/// Descriptor(s) for IP traffic. It allows the encoding of multiple UL
		/// and/or DL flows. Each entry of the array describes a single IP flow.
		#[serde(rename = "flowDescs", default, skip_serializing_if = "Vec::is_empty")]
		pub flow_descs: Vec<FlowDescription>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "ipv4Addr", default, skip_serializing_if = "Option::is_none")]
		pub ipv4_addr: Option<Ipv4Addr>,
		#[serde(rename = "ipv6Addrs", default, skip_serializing_if = "Vec::is_empty")]
		pub ipv6_addrs: Vec<Ipv6Addr>,
		#[serde(
			rename = "ipv6Prefixes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_prefixes: Vec<Ipv6Prefix>,
		#[serde(
			rename = "maxWaitTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_wait_time: Option<DateTime>,
		/// Represents the packet delay measurement failure indicator.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pdmf: Option<bool>,
		#[serde(rename = "pduSeId", default, skip_serializing_if = "Option::is_none")]
		pub pdu_se_id: Option<PduSessionId>,
		#[serde(
			rename = "pduSessInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pdu_sess_infos: Vec<PduSessionInformation>,
		#[serde(
			rename = "pduSessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_sess_type: Option<PduSessionType>,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub qfi: Option<Qfi>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "reIpv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub re_ipv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "reIpv6Prefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub re_ipv6_prefix: Option<Ipv6Prefix>,
		#[serde(rename = "rtDelays", default, skip_serializing_if = "Vec::is_empty")]
		pub rt_delays: Vec<Uinteger>,
		#[serde(
			rename = "smNasFromSmf",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_nas_from_smf: Option<SmNasFromSmf>,
		#[serde(
			rename = "smNasFromUe",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_nas_from_ue: Option<SmNasFromUe>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(
			rename = "sourceDnai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_dnai: Option<Dnai>,
		#[serde(
			rename = "sourceTraRouting",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_tra_routing: Option<RouteToLocation>,
		#[serde(
			rename = "sourceUeIpv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_ue_ipv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "sourceUeIpv6Prefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_ue_ipv6_prefix: Option<Ipv6Prefix>,
		#[serde(rename = "ssId", default, skip_serializing_if = "Option::is_none")]
		pub ss_id: Option<String>,
		#[serde(rename = "startWlan", default, skip_serializing_if = "Option::is_none")]
		pub start_wlan: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "targetDnai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_dnai: Option<Dnai>,
		#[serde(
			rename = "targetTraRouting",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_tra_routing: Option<RouteToLocation>,
		#[serde(
			rename = "targetUeIpv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_ue_ipv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "targetUeIpv6Prefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_ue_ipv6_prefix: Option<Ipv6Prefix>,
		#[serde(rename = "timeStamp")]
		pub time_stamp: DateTime,
		#[serde(
			rename = "timeWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_window: Option<TimeWindow>,
		/// Transaction Information.
		#[serde(
			rename = "transacInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub transac_infos: Vec<TransactionInfo>,
		#[serde(rename = "ueIpAddr", default, skip_serializing_if = "Option::is_none")]
		pub ue_ip_addr: Option<IpAddr>,
		#[serde(rename = "ueMac", default, skip_serializing_if = "Option::is_none")]
		pub ue_mac: Option<MacAddr48>,
		#[serde(rename = "ulDelays", default, skip_serializing_if = "Vec::is_empty")]
		pub ul_delays: Vec<Uinteger>,
		/// Indicates whether the redundant transmission is setup or terminated.
		/// Set to "true" if  the redundant transmission is setup, otherwise set
		/// to "false" if the redundant  transmission is terminated. Default
		/// value is set to "false".
		#[serde(
			rename = "upRedTrans",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_red_trans: Option<bool>,
		#[serde(rename = "upfInfo", default, skip_serializing_if = "Option::is_none")]
		pub upf_info: Option<UpfInformation>,
	}

	impl From<&EventNotification> for EventNotification {
		fn from(value: &EventNotification) -> Self {
			value.clone()
		}
	}

	/// Represents a subscription to a single event.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a subscription to a single event.",
	///  "type": "object",
	///  "required": [
	///    "event"
	///  ],
	///  "properties": {
	///    "appIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ApplicationId"
	///      },
	///      "minItems": 1
	///    },
	///    "dddStati": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DlDataDeliveryStatus"
	///      },
	///      "minItems": 1
	///    },
	///    "dddTraDescriptors": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DddTrafficDescriptor"
	///      },
	///      "minItems": 1
	///    },
	///    "dnaiChgType": {
	///      "$ref": "#/components/schemas/DnaiChangeType"
	///    },
	///    "event": {
	///      "$ref": "#/components/schemas/SmfEvent"
	///    },
	///    "targetPeriod": {
	///      "$ref": "#/components/schemas/TimeWindow"
	///    },
	///    "transacDispInd": {
	///      "description": "Indicates the subscription for UE transaction
	/// dispersion collectionon, if it is included and set to \"true\". Default
	/// value is \"false\".\n",
	///      "type": "boolean"
	///    },
	///    "transacMetrics": {
	///      "description": "Indicates Session Management Transaction metrics.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TransactionMetric"
	///      },
	///      "minItems": 1
	///    },
	///    "ueIpAddr": {
	///      "$ref": "#/components/schemas/IpAddr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EventSubscription {
		#[serde(rename = "appIds", default, skip_serializing_if = "Vec::is_empty")]
		pub app_ids: Vec<ApplicationId>,
		#[serde(rename = "dddStati", default, skip_serializing_if = "Vec::is_empty")]
		pub ddd_stati: Vec<DlDataDeliveryStatus>,
		#[serde(
			rename = "dddTraDescriptors",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ddd_tra_descriptors: Vec<DddTrafficDescriptor>,
		#[serde(
			rename = "dnaiChgType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dnai_chg_type: Option<DnaiChangeType>,
		pub event: SmfEvent,
		#[serde(
			rename = "targetPeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_period: Option<TimeWindow>,
		/// Indicates the subscription for UE transaction dispersion
		/// collectionon, if it is included and set to "true". Default value is
		/// "false".
		#[serde(
			rename = "transacDispInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub transac_disp_ind: Option<bool>,
		/// Indicates Session Management Transaction metrics.
		#[serde(
			rename = "transacMetrics",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub transac_metrics: Vec<TransactionMetric>,
		#[serde(rename = "ueIpAddr", default, skip_serializing_if = "Option::is_none")]
		pub ue_ip_addr: Option<IpAddr>,
	}

	impl From<&EventSubscription> for EventSubscription {
		fn from(value: &EventSubscription) -> Self {
			value.clone()
		}
	}

	/// Exemption Indication
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Exemption Indication",
	///  "type": "object",
	///  "properties": {
	///    "dnnCongestion": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "snssaiDnnCongestion": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "snssaiOnlyCongestion": {
	///      "default": false,
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ExemptionInd {
		#[serde(rename = "dnnCongestion", default)]
		pub dnn_congestion: bool,
		#[serde(rename = "snssaiDnnCongestion", default)]
		pub snssai_dnn_congestion: bool,
		#[serde(rename = "snssaiOnlyCongestion", default)]
		pub snssai_only_congestion: bool,
	}

	impl From<&ExemptionInd> for ExemptionInd {
		fn from(value: &ExemptionInd) -> Self {
			value.clone()
		}
	}

	/// Unsigned integer indicating Maximum Data Burst Volume (see clauses
	/// 5.7.3.7 and 5.7.4 of 3GPP TS 23.501), expressed in Bytes.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer indicating Maximum Data Burst Volume
	/// (see clauses 5.7.3.7 and 5.7.4 of 3GPP TS 23.501), expressed in Bytes.
	/// \n",
	///  "type": "integer",
	///  "maximum": 2000000.0,
	///  "minimum": 4096.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ExtMaxDataBurstVol(pub i64);

	impl ::std::ops::Deref for ExtMaxDataBurstVol {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ExtMaxDataBurstVol> for i64 {
		fn from(value: ExtMaxDataBurstVol) -> Self {
			value.0
		}
	}

	impl From<&ExtMaxDataBurstVol> for ExtMaxDataBurstVol {
		fn from(value: &ExtMaxDataBurstVol) -> Self {
			value.clone()
		}
	}

	impl From<i64> for ExtMaxDataBurstVol {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ExtMaxDataBurstVol {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ExtMaxDataBurstVol {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ExtMaxDataBurstVol {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ExtMaxDataBurstVol {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ExtMaxDataBurstVol {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Unsigned integer indicating Packet Delay Budget (see clauses 5.7.3.4 and
	/// 5.7.4 of 3GPP TS 23.501 [8])), expressed in 0.01 milliseconds.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer indicating Packet Delay Budget (see
	/// clauses 5.7.3.4 and 5.7.4 of 3GPP TS 23.501 [8])), expressed in 0.01
	/// milliseconds.\n",
	///  "type": "integer",
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ExtPacketDelBudget(
		#[default(_code = "unsafe {std::num::NonZeroU64::new_unchecked(1)}")]
		pub  std::num::NonZeroU64,
	);

	impl ::std::ops::Deref for ExtPacketDelBudget {
		type Target = std::num::NonZeroU64;
		fn deref(&self) -> &std::num::NonZeroU64 {
			&self.0
		}
	}

	impl From<ExtPacketDelBudget> for std::num::NonZeroU64 {
		fn from(value: ExtPacketDelBudget) -> Self {
			value.0
		}
	}

	impl From<&ExtPacketDelBudget> for ExtPacketDelBudget {
		fn from(value: &ExtPacketDelBudget) -> Self {
			value.clone()
		}
	}

	impl From<std::num::NonZeroU64> for ExtPacketDelBudget {
		fn from(value: std::num::NonZeroU64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ExtPacketDelBudget {
		type Err = <std::num::NonZeroU64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ExtPacketDelBudget {
		type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ExtPacketDelBudget {
		type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ExtPacketDelBudget {
		type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ExtPacketDelBudget {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Extended Problem Details
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Extended Problem Details",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    {
	///      "$ref": "#/components/schemas/ProblemDetailsAddInfo"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ExtProblemDetails {
		#[serde(
			rename = "accessTokenError",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_token_error: Option<AccessTokenErr>,
		#[serde(
			rename = "accessTokenRequest",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_token_request: Option<AccessTokenReq>,
		/// A machine-readable application error cause specific to this
		/// occurrence of the problem.  This IE should be present and provide
		/// application-related error information, if available.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<String>,
		/// A human-readable explanation specific to this occurrence of the
		/// problem.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub detail: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub instance: Option<Uri>,
		#[serde(
			rename = "invalidParams",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub invalid_params: Vec<InvalidParam>,
		#[serde(rename = "nrfId", default, skip_serializing_if = "Option::is_none")]
		pub nrf_id: Option<Fqdn>,
		#[serde(
			rename = "remoteError",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remote_error: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub status: Option<i64>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub title: Option<String>,
		#[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
		pub type_: Option<Uri>,
	}

	impl From<&ExtProblemDetails> for ExtProblemDetails {
		fn from(value: &ExtProblemDetails) -> Self {
			value.clone()
		}
	}

	/// Defines a packet filter of an IP flow.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Defines a packet filter of an IP flow.",
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
	pub struct FlowDescription(pub String);

	impl ::std::ops::Deref for FlowDescription {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<FlowDescription> for String {
		fn from(value: FlowDescription) -> Self {
			value.0
		}
	}

	impl From<&FlowDescription> for FlowDescription {
		fn from(value: &FlowDescription) -> Self {
			value.clone()
		}
	}

	impl From<String> for FlowDescription {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for FlowDescription {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for FlowDescription {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Possible values are
	/// - DOWNLINK: The corresponding filter applies for traffic to the UE.
	/// - UPLINK: The corresponding filter applies for traffic from the UE.
	/// - BIDIRECTIONAL: The corresponding filter applies for traffic both to
	///   and from the UE.
	/// - UNSPECIFIED: The corresponding filter applies for traffic to the UE
	///   (downlink), but has no specific direction declared. The service data
	///   flow detection shall apply the filter for uplink traffic as if the
	///   filter was bidirectional. The PCF shall not use the value UNSPECIFIED
	///   in filters created by the network in NW-initiated procedures. The PCF
	///   shall only include the value UNSPECIFIED in filters in UE-initiated
	///   procedures if the same value is received from the SMF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n- DOWNLINK: The corresponding
	/// filter applies for traffic to the UE.\n- UPLINK: The corresponding
	/// filter applies for traffic from the UE.\n- BIDIRECTIONAL: The
	/// corresponding filter applies for traffic both to and from the UE.\n-
	/// UNSPECIFIED: The corresponding filter applies for traffic to the UE
	/// (downlink), but has no specific direction declared. The service data
	/// flow detection shall apply the filter for uplink traffic as if the
	/// filter was bidirectional. The PCF shall not use the value UNSPECIFIED in
	/// filters created by the network in NW-initiated procedures. The PCF shall
	/// only include the value UNSPECIFIED in filters in UE-initiated procedures
	/// if the same value is received from the SMF.\n",
	///  "type": "string",
	///  "enum": [
	///    "DOWNLINK",
	///    "UPLINK",
	///    "BIDIRECTIONAL",
	///    "UNSPECIFIED"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum FlowDirection {
		#[default]
		#[serde(rename = "DOWNLINK")]
		Downlink,
		#[serde(rename = "UPLINK")]
		Uplink,
		#[serde(rename = "BIDIRECTIONAL")]
		Bidirectional,
		#[serde(rename = "UNSPECIFIED")]
		Unspecified,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&FlowDirection> for FlowDirection {
		fn from(value: &FlowDirection) -> Self {
			value.clone()
		}
	}

	impl ToString for FlowDirection {
		fn to_string(&self) -> String {
			match *self {
				Self::Downlink => "DOWNLINK".to_string(),
				Self::Uplink => "UPLINK".to_string(),
				Self::Bidirectional => "BIDIRECTIONAL".to_string(),
				Self::Unspecified => "UNSPECIFIED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for FlowDirection {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DOWNLINK" => Ok(Self::Downlink),
				"UPLINK" => Ok(Self::Uplink),
				"BIDIRECTIONAL" => Ok(Self::Bidirectional),
				"UNSPECIFIED" => Ok(Self::Unspecified),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for FlowDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for FlowDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for FlowDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Forwarding Bearer Container
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Forwarding Bearer Container",
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
	pub struct ForwardingBearerContainer(pub String);

	impl ::std::ops::Deref for ForwardingBearerContainer {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ForwardingBearerContainer> for String {
		fn from(value: ForwardingBearerContainer) -> Self {
			value.0
		}
	}

	impl From<&ForwardingBearerContainer> for ForwardingBearerContainer {
		fn from(value: &ForwardingBearerContainer) -> Self {
			value.clone()
		}
	}

	impl From<String> for ForwardingBearerContainer {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ForwardingBearerContainer {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for ForwardingBearerContainer {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

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
	)]
	pub struct Fqdn(String);

	impl ::std::ops::Deref for Fqdn {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Fqdn> for String {
		fn from(value: Fqdn) -> Self {
			value.0
		}
	}

	impl From<&Fqdn> for Fqdn {
		fn from(value: &Fqdn) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Fqdn {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if value.len() > 253usize {
				return Err("longer than 253 characters".into());
			}
			if value.len() < 4usize {
				return Err("shorter than 4 characters".into());
			}
			if regress::Regex::new(
				"^([0-9A-Za-z]([-0-9A-Za-z]{0,61}[0-9A-Za-z])?\\.)+[A-Za-z]{2,63}\\.?$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^([0-9A-Za-z]([-0-9A-Za-z]{0,61}[0-9A-Za-z])?\\.)+[A-Za-z]{2,63}\\\
				            .?$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Fqdn {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Fqdn {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Fqdn {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Fqdn {
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

	/// Provides the G-NB identifier.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Provides the G-NB identifier.",
	///  "type": "object",
	///  "required": [
	///    "bitLength",
	///    "gNBValue"
	///  ],
	///  "properties": {
	///    "bitLength": {
	///      "description": "Unsigned integer representing the bit length of the
	/// gNB ID as defined in clause 9.3.1.6 of 3GPP TS 38.413 [11], within the
	/// range 22 to 32.\n",
	///      "type": "integer",
	///      "maximum": 32.0,
	///      "minimum": 22.0
	///    },
	///    "gNBValue": {
	///      "description": "This represents the identifier of the gNB. The
	/// value of the gNB ID shall be encoded in hexadecimal representation. Each
	/// character in the string shall take a value of \"0\" to \"9\", \"a\" to
	/// \"f\" or \"A\" to \"F\" and shall represent 4 bits. The padding 0 shall
	/// be added to make multiple nibbles,  the most significant character
	/// representing the padding 0 if required together with the 4 most
	/// significant bits of the gNB ID shall appear first in the string, and the
	/// character representing the 4 least significant bit of the gNB ID shall
	/// appear last in the string.\n",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{6,8}$"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GNbId {
		/// Unsigned integer representing the bit length of the gNB ID as
		/// defined in clause 9.3.1.6 of 3GPP TS 38.413 [11], within the range
		/// 22 to 32.
		#[serde(rename = "bitLength")]
		pub bit_length: i64,
		/// This represents the identifier of the gNB. The value of the gNB ID
		/// shall be encoded in hexadecimal representation. Each character in
		/// the string shall take a value of "0" to "9", "a" to "f" or "A" to
		/// "F" and shall represent 4 bits. The padding 0 shall be added to make
		/// multiple nibbles,  the most significant character representing the
		/// padding 0 if required together with the 4 most significant bits of
		/// the gNB ID shall appear first in the string, and the character
		/// representing the 4 least significant bit of the gNB ID shall appear
		/// last in the string.
		#[serde(rename = "gNBValue")]
		pub g_nb_value: GNbIdGNbValue,
	}

	impl From<&GNbId> for GNbId {
		fn from(value: &GNbId) -> Self {
			value.clone()
		}
	}

	/// This represents the identifier of the gNB. The value of the gNB ID shall
	/// be encoded in hexadecimal representation. Each character in the string
	/// shall take a value of "0" to "9", "a" to "f" or "A" to "F" and shall
	/// represent 4 bits. The padding 0 shall be added to make multiple nibbles,
	/// the most significant character representing the padding 0 if required
	/// together with the 4 most significant bits of the gNB ID shall appear
	/// first in the string, and the character representing the 4 least
	/// significant bit of the gNB ID shall appear last in the string.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This represents the identifier of the gNB. The value of
	/// the gNB ID shall be encoded in hexadecimal representation. Each
	/// character in the string shall take a value of \"0\" to \"9\", \"a\" to
	/// \"f\" or \"A\" to \"F\" and shall represent 4 bits. The padding 0 shall
	/// be added to make multiple nibbles,  the most significant character
	/// representing the padding 0 if required together with the 4 most
	/// significant bits of the gNB ID shall appear first in the string, and the
	/// character representing the 4 least significant bit of the gNB ID shall
	/// appear last in the string.\n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{6,8}$"
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
	)]
	pub struct GNbIdGNbValue(String);

	impl ::std::ops::Deref for GNbIdGNbValue {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<GNbIdGNbValue> for String {
		fn from(value: GNbIdGNbValue) -> Self {
			value.0
		}
	}

	impl From<&GNbIdGNbValue> for GNbIdGNbValue {
		fn from(value: &GNbIdGNbValue) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for GNbIdGNbValue {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{6,8}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{6,8}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for GNbIdGNbValue {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for GNbIdGNbValue {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for GNbIdGNbValue {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GNbIdGNbValue {
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

	/// GBR QoS flow information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "GBR QoS flow information",
	///  "type": "object",
	///  "required": [
	///    "guaFbrDl",
	///    "guaFbrUl",
	///    "maxFbrDl",
	///    "maxFbrUl"
	///  ],
	///  "properties": {
	///    "alternativeQosProfileList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AlternativeQosProfile"
	///      }
	///    },
	///    "guaFbrDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "guaFbrUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "maxFbrDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "maxFbrUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "maxPacketLossRateDl": {
	///      "$ref": "#/components/schemas/PacketLossRate"
	///    },
	///    "maxPacketLossRateUl": {
	///      "$ref": "#/components/schemas/PacketLossRate"
	///    },
	///    "notifControl": {
	///      "$ref": "#/components/schemas/NotificationControl"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GbrQosFlowInformation {
		#[serde(
			rename = "alternativeQosProfileList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alternative_qos_profile_list: Vec<AlternativeQosProfile>,
		#[serde(rename = "guaFbrDl")]
		pub gua_fbr_dl: BitRate,
		#[serde(rename = "guaFbrUl")]
		pub gua_fbr_ul: BitRate,
		#[serde(rename = "maxFbrDl")]
		pub max_fbr_dl: BitRate,
		#[serde(rename = "maxFbrUl")]
		pub max_fbr_ul: BitRate,
		#[serde(
			rename = "maxPacketLossRateDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_packet_loss_rate_dl: Option<PacketLossRate>,
		#[serde(
			rename = "maxPacketLossRateUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_packet_loss_rate_ul: Option<PacketLossRate>,
		#[serde(
			rename = "notifControl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notif_control: Option<NotificationControl>,
	}

	impl From<&GbrQosFlowInformation> for GbrQosFlowInformation {
		fn from(value: &GbrQosFlowInformation) -> Self {
			value.clone()
		}
	}

	/// Global Cable Identifier uniquely identifying the connection between the
	/// 5G-CRG or FN-CRG to the 5GS. See clause 28.15.4 of 3GPP TS 23.003. This
	/// shall be encoded as a string per clause 28.15.4 of 3GPP TS 23.003, and
	/// compliant with the syntax specified  in clause 2.2  of IETF RFC 7542 for
	/// the username part of a NAI. The GCI value is specified in CableLabs
	/// WR-TR-5WWC-ARCH.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Global Cable Identifier uniquely identifying the
	/// connection between the 5G-CRG or FN-CRG to the 5GS. See clause 28.15.4
	/// of 3GPP TS 23.003. This shall be encoded as a string per clause 28.15.4
	/// of 3GPP TS 23.003, and compliant with the syntax specified  in clause
	/// 2.2  of IETF RFC 7542 for the username part of a NAI. The GCI value is
	/// specified in CableLabs WR-TR-5WWC-ARCH.\n",
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
	pub struct Gci(pub String);

	impl ::std::ops::Deref for Gci {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Gci> for String {
		fn from(value: Gci) -> Self {
			value.0
		}
	}

	impl From<&Gci> for Gci {
		fn from(value: &Gci) -> Self {
			value.clone()
		}
	}

	impl From<String> for Gci {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Gci {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for Gci {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Exactly one of cgi, sai or lai shall be present.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Exactly one of cgi, sai or lai shall be present.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "cgi"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "sai"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "lai"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "ageOfLocationInformation": {
	///      "description": "The value represents the elapsed time in minutes since the last network contact of the mobile station. Value \"0\" indicates that the location information was obtained after a successful paging procedure for  Active Location Retrieval when the UE is in idle mode or after a successful location reporting procedure the UE is in connected mode. Any other value than \"0\" indicates that the location information is the last known one. See 3GPP TS 29.002 clause 17.7.8.\n",
	///      "type": "integer",
	///      "maximum": 32767.0,
	///      "minimum": 0.0
	///    },
	///    "cgi": {
	///      "$ref": "#/components/schemas/CellGlobalId"
	///    },
	///    "geodeticInformation": {
	///      "description": "Refers to Calling Geodetic Location.See ITU-T
	/// Recommendation Q.763 (1999) clause 3.88.2.  Only the description of an
	/// ellipsoid point with uncertainty circle is allowed to be used.\n",
	///      "type": "string",
	///      "pattern": "^[0-9A-F]{20}$"
	///    },
	///    "geographicalInformation": {
	///      "description": "Refer to geographical Information.See 3GPP TS
	/// 23.032 clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///      "type": "string",
	///      "pattern": "^[0-9A-F]{16}$"
	///    },
	///    "lai": {
	///      "$ref": "#/components/schemas/LocationAreaId"
	///    },
	///    "locationNumber": {
	///      "description": "Location number within the PLMN. See 3GPP TS
	/// 23.003, clause 4.5.",
	///      "type": "string"
	///    },
	///    "mscNumber": {
	///      "description": "MSC number. See 3GPP TS 23.003 clause 5.1.",
	///      "type": "string"
	///    },
	///    "rai": {
	///      "$ref": "#/components/schemas/RoutingAreaId"
	///    },
	///    "sai": {
	///      "$ref": "#/components/schemas/ServiceAreaId"
	///    },
	///    "ueLocationTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "vlrNumber": {
	///      "description": "VLR number. See 3GPP TS 23.003 clause 5.1.",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum GeraLocation {
		#[default]
		Variant0 {
			/// The value represents the elapsed time in minutes since the last
			/// network contact of the mobile station. Value "0" indicates that
			/// the location information was obtained after a successful paging
			/// procedure for  Active Location Retrieval when the UE is in idle
			/// mode or after a successful location reporting procedure the UE
			/// is in connected mode. Any other value than "0" indicates that
			/// the location information is the last known one. See 3GPP TS
			/// 29.002 clause 17.7.8.
			#[serde(
				rename = "ageOfLocationInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			age_of_location_information: Option<i64>,
			cgi: CellGlobalId,
			/// Refers to Calling Geodetic Location.See ITU-T Recommendation
			/// Q.763 (1999) clause 3.88.2.  Only the description of an
			/// ellipsoid point with uncertainty circle is allowed to be used.
			#[serde(
				rename = "geodeticInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geodetic_information: Option<GeraLocationVariant0GeodeticInformation>,
			/// Refer to geographical Information.See 3GPP TS 23.032 clause
			/// 7.3.2. Only the description of an ellipsoid point with
			/// uncertainty circle is allowed to be used.
			#[serde(
				rename = "geographicalInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geographical_information: Option<GeraLocationVariant0GeographicalInformation>,
			/// Location number within the PLMN. See 3GPP TS 23.003, clause 4.5.
			#[serde(
				rename = "locationNumber",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			location_number: Option<String>,
			/// MSC number. See 3GPP TS 23.003 clause 5.1.
			#[serde(rename = "mscNumber", default, skip_serializing_if = "Option::is_none")]
			msc_number: Option<String>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			rai: Option<RoutingAreaId>,
			#[serde(
				rename = "ueLocationTimestamp",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_location_timestamp: Option<DateTime>,
			/// VLR number. See 3GPP TS 23.003 clause 5.1.
			#[serde(rename = "vlrNumber", default, skip_serializing_if = "Option::is_none")]
			vlr_number: Option<String>,
		},
		Variant1 {
			/// The value represents the elapsed time in minutes since the last
			/// network contact of the mobile station. Value "0" indicates that
			/// the location information was obtained after a successful paging
			/// procedure for  Active Location Retrieval when the UE is in idle
			/// mode or after a successful location reporting procedure the UE
			/// is in connected mode. Any other value than "0" indicates that
			/// the location information is the last known one. See 3GPP TS
			/// 29.002 clause 17.7.8.
			#[serde(
				rename = "ageOfLocationInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			age_of_location_information: Option<i64>,
			/// Refers to Calling Geodetic Location.See ITU-T Recommendation
			/// Q.763 (1999) clause 3.88.2.  Only the description of an
			/// ellipsoid point with uncertainty circle is allowed to be used.
			#[serde(
				rename = "geodeticInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geodetic_information: Option<GeraLocationVariant1GeodeticInformation>,
			/// Refer to geographical Information.See 3GPP TS 23.032 clause
			/// 7.3.2. Only the description of an ellipsoid point with
			/// uncertainty circle is allowed to be used.
			#[serde(
				rename = "geographicalInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geographical_information: Option<GeraLocationVariant1GeographicalInformation>,
			/// Location number within the PLMN. See 3GPP TS 23.003, clause 4.5.
			#[serde(
				rename = "locationNumber",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			location_number: Option<String>,
			/// MSC number. See 3GPP TS 23.003 clause 5.1.
			#[serde(rename = "mscNumber", default, skip_serializing_if = "Option::is_none")]
			msc_number: Option<String>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			rai: Option<RoutingAreaId>,
			sai: ServiceAreaId,
			#[serde(
				rename = "ueLocationTimestamp",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_location_timestamp: Option<DateTime>,
			/// VLR number. See 3GPP TS 23.003 clause 5.1.
			#[serde(rename = "vlrNumber", default, skip_serializing_if = "Option::is_none")]
			vlr_number: Option<String>,
		},
		Variant2 {
			/// The value represents the elapsed time in minutes since the last
			/// network contact of the mobile station. Value "0" indicates that
			/// the location information was obtained after a successful paging
			/// procedure for  Active Location Retrieval when the UE is in idle
			/// mode or after a successful location reporting procedure the UE
			/// is in connected mode. Any other value than "0" indicates that
			/// the location information is the last known one. See 3GPP TS
			/// 29.002 clause 17.7.8.
			#[serde(
				rename = "ageOfLocationInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			age_of_location_information: Option<i64>,
			/// Refers to Calling Geodetic Location.See ITU-T Recommendation
			/// Q.763 (1999) clause 3.88.2.  Only the description of an
			/// ellipsoid point with uncertainty circle is allowed to be used.
			#[serde(
				rename = "geodeticInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geodetic_information: Option<GeraLocationVariant2GeodeticInformation>,
			/// Refer to geographical Information.See 3GPP TS 23.032 clause
			/// 7.3.2. Only the description of an ellipsoid point with
			/// uncertainty circle is allowed to be used.
			#[serde(
				rename = "geographicalInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geographical_information: Option<GeraLocationVariant2GeographicalInformation>,
			lai: LocationAreaId,
			/// Location number within the PLMN. See 3GPP TS 23.003, clause 4.5.
			#[serde(
				rename = "locationNumber",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			location_number: Option<String>,
			/// MSC number. See 3GPP TS 23.003 clause 5.1.
			#[serde(rename = "mscNumber", default, skip_serializing_if = "Option::is_none")]
			msc_number: Option<String>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			rai: Option<RoutingAreaId>,
			#[serde(
				rename = "ueLocationTimestamp",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_location_timestamp: Option<DateTime>,
			/// VLR number. See 3GPP TS 23.003 clause 5.1.
			#[serde(rename = "vlrNumber", default, skip_serializing_if = "Option::is_none")]
			vlr_number: Option<String>,
		},
	}

	impl From<&GeraLocation> for GeraLocation {
		fn from(value: &GeraLocation) -> Self {
			value.clone()
		}
	}

	/// Refers to Calling Geodetic Location.See ITU-T Recommendation Q.763
	/// (1999) clause 3.88.2.  Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refers to Calling Geodetic Location.See ITU-T
	/// Recommendation Q.763 (1999) clause 3.88.2.  Only the description of an
	/// ellipsoid point with uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{20}$"
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
	)]
	pub struct GeraLocationVariant0GeodeticInformation(String);

	impl ::std::ops::Deref for GeraLocationVariant0GeodeticInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<GeraLocationVariant0GeodeticInformation> for String {
		fn from(value: GeraLocationVariant0GeodeticInformation) -> Self {
			value.0
		}
	}

	impl From<&GeraLocationVariant0GeodeticInformation> for GeraLocationVariant0GeodeticInformation {
		fn from(value: &GeraLocationVariant0GeodeticInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for GeraLocationVariant0GeodeticInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{20}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{20}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for GeraLocationVariant0GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for GeraLocationVariant0GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for GeraLocationVariant0GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GeraLocationVariant0GeodeticInformation {
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

	/// Refer to geographical Information.See 3GPP TS 23.032 clause 7.3.2. Only
	/// the description of an ellipsoid point with uncertainty circle is allowed
	/// to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refer to geographical Information.See 3GPP TS 23.032
	/// clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{16}$"
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
	)]
	pub struct GeraLocationVariant0GeographicalInformation(String);

	impl ::std::ops::Deref for GeraLocationVariant0GeographicalInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<GeraLocationVariant0GeographicalInformation> for String {
		fn from(value: GeraLocationVariant0GeographicalInformation) -> Self {
			value.0
		}
	}

	impl From<&GeraLocationVariant0GeographicalInformation>
		for GeraLocationVariant0GeographicalInformation
	{
		fn from(value: &GeraLocationVariant0GeographicalInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for GeraLocationVariant0GeographicalInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{16}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{16}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for GeraLocationVariant0GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for GeraLocationVariant0GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for GeraLocationVariant0GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GeraLocationVariant0GeographicalInformation {
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

	/// Refers to Calling Geodetic Location.See ITU-T Recommendation Q.763
	/// (1999) clause 3.88.2.  Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refers to Calling Geodetic Location.See ITU-T
	/// Recommendation Q.763 (1999) clause 3.88.2.  Only the description of an
	/// ellipsoid point with uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{20}$"
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
	)]
	pub struct GeraLocationVariant1GeodeticInformation(String);

	impl ::std::ops::Deref for GeraLocationVariant1GeodeticInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<GeraLocationVariant1GeodeticInformation> for String {
		fn from(value: GeraLocationVariant1GeodeticInformation) -> Self {
			value.0
		}
	}

	impl From<&GeraLocationVariant1GeodeticInformation> for GeraLocationVariant1GeodeticInformation {
		fn from(value: &GeraLocationVariant1GeodeticInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for GeraLocationVariant1GeodeticInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{20}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{20}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for GeraLocationVariant1GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for GeraLocationVariant1GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for GeraLocationVariant1GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GeraLocationVariant1GeodeticInformation {
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

	/// Refer to geographical Information.See 3GPP TS 23.032 clause 7.3.2. Only
	/// the description of an ellipsoid point with uncertainty circle is allowed
	/// to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refer to geographical Information.See 3GPP TS 23.032
	/// clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{16}$"
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
	)]
	pub struct GeraLocationVariant1GeographicalInformation(String);

	impl ::std::ops::Deref for GeraLocationVariant1GeographicalInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<GeraLocationVariant1GeographicalInformation> for String {
		fn from(value: GeraLocationVariant1GeographicalInformation) -> Self {
			value.0
		}
	}

	impl From<&GeraLocationVariant1GeographicalInformation>
		for GeraLocationVariant1GeographicalInformation
	{
		fn from(value: &GeraLocationVariant1GeographicalInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for GeraLocationVariant1GeographicalInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{16}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{16}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for GeraLocationVariant1GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for GeraLocationVariant1GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for GeraLocationVariant1GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GeraLocationVariant1GeographicalInformation {
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

	/// Refers to Calling Geodetic Location.See ITU-T Recommendation Q.763
	/// (1999) clause 3.88.2.  Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refers to Calling Geodetic Location.See ITU-T
	/// Recommendation Q.763 (1999) clause 3.88.2.  Only the description of an
	/// ellipsoid point with uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{20}$"
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
	)]
	pub struct GeraLocationVariant2GeodeticInformation(String);

	impl ::std::ops::Deref for GeraLocationVariant2GeodeticInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<GeraLocationVariant2GeodeticInformation> for String {
		fn from(value: GeraLocationVariant2GeodeticInformation) -> Self {
			value.0
		}
	}

	impl From<&GeraLocationVariant2GeodeticInformation> for GeraLocationVariant2GeodeticInformation {
		fn from(value: &GeraLocationVariant2GeodeticInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for GeraLocationVariant2GeodeticInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{20}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{20}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for GeraLocationVariant2GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for GeraLocationVariant2GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for GeraLocationVariant2GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GeraLocationVariant2GeodeticInformation {
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

	/// Refer to geographical Information.See 3GPP TS 23.032 clause 7.3.2. Only
	/// the description of an ellipsoid point with uncertainty circle is allowed
	/// to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refer to geographical Information.See 3GPP TS 23.032
	/// clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{16}$"
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
	)]
	pub struct GeraLocationVariant2GeographicalInformation(String);

	impl ::std::ops::Deref for GeraLocationVariant2GeographicalInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<GeraLocationVariant2GeographicalInformation> for String {
		fn from(value: GeraLocationVariant2GeographicalInformation) -> Self {
			value.0
		}
	}

	impl From<&GeraLocationVariant2GeographicalInformation>
		for GeraLocationVariant2GeographicalInformation
	{
		fn from(value: &GeraLocationVariant2GeographicalInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for GeraLocationVariant2GeographicalInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{16}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{16}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for GeraLocationVariant2GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for GeraLocationVariant2GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for GeraLocationVariant2GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GeraLocationVariant2GeographicalInformation {
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

	/// One of the six attributes n3IwfId, gNbIdm, ngeNbId, wagfId, tngfId,
	/// eNbId shall be present.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "One of the six attributes n3IwfId, gNbIdm, ngeNbId,
	/// wagfId, tngfId, eNbId shall be present.\n",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "n3IwfId"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "gNbId"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ngeNbId"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "wagfId"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "tngfId"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "eNbId"
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "plmnId"
	///  ],
	///  "properties": {
	///    "eNbId": {
	///      "$ref": "#/components/schemas/ENbId"
	///    },
	///    "gNbId": {
	///      "$ref": "#/components/schemas/GNbId"
	///    },
	///    "n3IwfId": {
	///      "$ref": "#/components/schemas/N3IwfId"
	///    },
	///    "ngeNbId": {
	///      "$ref": "#/components/schemas/NgeNbId"
	///    },
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "tngfId": {
	///      "$ref": "#/components/schemas/TngfId"
	///    },
	///    "wagfId": {
	///      "$ref": "#/components/schemas/WAgfId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum GlobalRanNodeId {
		#[default]
		Variant0 {
			#[serde(rename = "n3IwfId")]
			n3_iwf_id: N3IwfId,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			nid: Option<Nid>,
			#[serde(rename = "plmnId")]
			plmn_id: PlmnId,
		},
		Variant1 {
			#[serde(rename = "gNbId")]
			g_nb_id: GNbId,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			nid: Option<Nid>,
			#[serde(rename = "plmnId")]
			plmn_id: PlmnId,
		},
		Variant2 {
			#[serde(rename = "ngeNbId")]
			nge_nb_id: NgeNbId,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			nid: Option<Nid>,
			#[serde(rename = "plmnId")]
			plmn_id: PlmnId,
		},
		Variant3 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			nid: Option<Nid>,
			#[serde(rename = "plmnId")]
			plmn_id: PlmnId,
			#[serde(rename = "wagfId")]
			wagf_id: WAgfId,
		},
		Variant4 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			nid: Option<Nid>,
			#[serde(rename = "plmnId")]
			plmn_id: PlmnId,
			#[serde(rename = "tngfId")]
			tngf_id: TngfId,
		},
		Variant5 {
			#[serde(rename = "eNbId")]
			e_nb_id: ENbId,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			nid: Option<Nid>,
			#[serde(rename = "plmnId")]
			plmn_id: PlmnId,
		},
	}

	impl From<&GlobalRanNodeId> for GlobalRanNodeId {
		fn from(value: &GlobalRanNodeId) -> Self {
			value.clone()
		}
	}

	/// String identifying a Gpsi shall contain either an External Id or an
	/// MSISDN.  It shall be formatted as follows -External Identifier=
	/// "extid-'extid', where 'extid'  shall be formatted according to clause
	/// 19.7.2 of 3GPP TS 23.003 that describes an  External Identifier.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a Gpsi shall contain either an
	/// External Id or an MSISDN.  It shall be formatted as follows -External
	/// Identifier= \"extid-'extid', where 'extid'  shall be formatted according
	/// to clause 19.7.2 of 3GPP TS 23.003 that describes an  External
	/// Identifier. \n",
	///  "type": "string",
	///  "pattern": "^(msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|.+)$"
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
	)]
	pub struct Gpsi(String);

	impl ::std::ops::Deref for Gpsi {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Gpsi> for String {
		fn from(value: Gpsi) -> Self {
			value.0
		}
	}

	impl From<&Gpsi> for Gpsi {
		fn from(value: &Gpsi) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Gpsi {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^(msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|.+)$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \
				            \"^(msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|.+)$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Gpsi {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Gpsi {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Gpsi {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Gpsi {
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

	/// String identifying a group of devices network internal globally unique
	/// ID which identifies a set of IMSIs, as specified in clause 19.9 of 3GPP
	/// TS 23.003.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a group of devices network internal
	/// globally unique ID which identifies a set of IMSIs, as specified in
	/// clause 19.9 of 3GPP TS 23.003. \n",
	///  "type": "string",
	///  "pattern":
	/// "^[A-Fa-f0-9]{8}-[0-9]{3}-[0-9]{2,3}-([A-Fa-f0-9][A-Fa-f0-9]){1,10}$"
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
	)]
	pub struct GroupId(String);

	impl ::std::ops::Deref for GroupId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<GroupId> for String {
		fn from(value: GroupId) -> Self {
			value.0
		}
	}

	impl From<&GroupId> for GroupId {
		fn from(value: &GroupId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for GroupId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"^[A-Fa-f0-9]{8}-[0-9]{3}-[0-9]{2,3}-([A-Fa-f0-9][A-Fa-f0-9]){1,10}$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^[A-Fa-f0-9]{8}-[0-9]{3}-[0-9]{2,3}-([A-Fa-f0-9][A-Fa-f0-9]){1,\
				            10}$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for GroupId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for GroupId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for GroupId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GroupId {
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

	/// This IE represents the identifier of the HFC node Id as specified in
	/// CableLabs WR-TR-5WWC-ARCH. It is provisioned by the wireline operator as
	/// part of wireline operations and may contain up to six characters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This IE represents the identifier of the HFC node Id as
	/// specified in CableLabs WR-TR-5WWC-ARCH. It is provisioned by the
	/// wireline operator as part of wireline operations and may contain up to
	/// six characters.\n",
	///  "type": "string",
	///  "maxLength": 6
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
	)]
	pub struct HfcNId(String);

	impl ::std::ops::Deref for HfcNId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<HfcNId> for String {
		fn from(value: HfcNId) -> Self {
			value.0
		}
	}

	impl From<&HfcNId> for HfcNId {
		fn from(value: &HfcNId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for HfcNId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if value.len() > 6usize {
				return Err("longer than 6 characters".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for HfcNId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for HfcNId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for HfcNId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for HfcNId {
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

	/// REpresents the HFC Node Identifer received over NGAP.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "REpresents the HFC Node Identifer received over NGAP.",
	///  "type": "object",
	///  "required": [
	///    "hfcNId"
	///  ],
	///  "properties": {
	///    "hfcNId": {
	///      "$ref": "#/components/schemas/HfcNId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HfcNodeId {
		#[serde(rename = "hfcNId")]
		pub hfc_n_id: HfcNId,
	}

	impl From<&HfcNodeId> for HfcNodeId {
		fn from(value: &HfcNodeId) -> Self {
			value.clone()
		}
	}

	/// Handover state. Possible values are
	/// - NONE
	/// - PREPARING
	/// - PREPARED
	/// - COMPLETED
	/// - CANCELLED
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Handover state. Possible values are\n- NONE\n-
	/// PREPARING\n- PREPARED\n- COMPLETED\n- CANCELLED\n",
	///  "type": "string",
	///  "enum": [
	///    "NONE",
	///    "PREPARING",
	///    "PREPARED",
	///    "COMPLETED",
	///    "CANCELLED"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum HoState {
		#[default]
		#[serde(rename = "NONE")]
		None,
		#[serde(rename = "PREPARING")]
		Preparing,
		#[serde(rename = "PREPARED")]
		Prepared,
		#[serde(rename = "COMPLETED")]
		Completed,
		#[serde(rename = "CANCELLED")]
		Cancelled,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&HoState> for HoState {
		fn from(value: &HoState) -> Self {
			value.clone()
		}
	}

	impl ToString for HoState {
		fn to_string(&self) -> String {
			match *self {
				Self::None => "NONE".to_string(),
				Self::Preparing => "PREPARING".to_string(),
				Self::Prepared => "PREPARED".to_string(),
				Self::Completed => "COMPLETED".to_string(),
				Self::Cancelled => "CANCELLED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for HoState {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NONE" => Ok(Self::None),
				"PREPARING" => Ok(Self::Preparing),
				"PREPARED" => Ok(Self::Prepared),
				"COMPLETED" => Ok(Self::Completed),
				"CANCELLED" => Ok(Self::Cancelled),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for HoState {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for HoState {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for HoState {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within Update Request towards H-SMF, or from I-SMF to SMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Update Request towards H-SMF, or from I-SMF
	/// to SMF",
	///  "type": "object",
	///  "required": [
	///    "requestIndication"
	///  ],
	///  "properties": {
	///    "5gMmCauseValue": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "NotifyList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionNotifyItem"
	///      },
	///      "minItems": 1
	///    },
	///    "addUeLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "additionalAnType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "additionalCnTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "alwaysOnRequested": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "amfNfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "anType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "anTypeCanBeChanged": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "cause": {
	///      "$ref": "#/components/schemas/Cause"
	///    },
	///    "disasterRoamingInd": {
	///      "type": "boolean",
	///      "enum": [
	///        true
	///      ]
	///    },
	///    "dlServingPlmnRateCtl": {
	///      "type": [
	///        "integer",
	///        "null"
	///      ],
	///      "minimum": 10.0
	///    },
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "epsBearerId": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerId"
	///      },
	///      "minItems": 0
	///    },
	///    "epsInterworkingInd": {
	///      "$ref": "#/components/schemas/EpsInterworkingIndication"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "hoPreparationIndication": {
	///      "type": "boolean"
	///    },
	///    "iSmfServiceInstanceId": {
	///      "type": "string"
	///    },
	///    "icnTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "ismfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "ismfPduSessionUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "maNwUpgradeInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "maReleaseInd": {
	///      "$ref": "#/components/schemas/MaReleaseIndication"
	///    },
	///    "maRequestInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "maxIntegrityProtectedDataRateDl": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "maxIntegrityProtectedDataRateUl": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "moExpDataCounter": {
	///      "$ref": "#/components/schemas/MoExpDataCounter"
	///    },
	///    "n1SmInfoFromUe": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n4Info": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt1": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt2": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "ngApCause": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    },
	///    "pauseCharging": {
	///      "type": "boolean"
	///    },
	///    "pcfUeCallbackInfo": {
	///      "$ref": "#/components/schemas/PcfUeCallbackInfo"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "presenceInLadn": {
	///      "$ref": "#/components/schemas/PresenceState"
	///    },
	///    "psaInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PsaInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "pti": {
	///      "$ref": "#/components/schemas/ProcedureTransactionId"
	///    },
	///    "qosFlowsNotifyList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowNotifyItem"
	///      },
	///      "minItems": 1
	///    },
	///    "qosFlowsRelNotifyList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowItem"
	///      },
	///      "minItems": 1
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "requestIndication": {
	///      "$ref": "#/components/schemas/RequestIndication"
	///    },
	///    "revokeEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerId"
	///      },
	///      "minItems": 1
	///    },
	///    "roamingChargingProfile": {
	///      "$ref": "#/components/schemas/RoamingChargingProfile"
	///    },
	///    "satelliteBackhaulCat": {
	///      "$ref": "#/components/schemas/SatelliteBackhaulCategory"
	///    },
	///    "secondaryRatUsageDataReportContainer": {
	///      "type": "array",
	///      "items": {
	///        "$ref":
	/// "#/components/schemas/SecondaryRatUsageDataReportContainer"
	///      },
	///      "minItems": 1
	///    },
	///    "secondaryRatUsageInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SecondaryRatUsageInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "secondaryRatUsageReport": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SecondaryRatUsageReport"
	///      },
	///      "minItems": 1
	///    },
	///    "securityResult": {
	///      "$ref": "#/components/schemas/SecurityResult"
	///    },
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "smPolicyNotifyInd": {
	///      "type": "boolean",
	///      "enum": [
	///        true
	///      ]
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "ueLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "ulclBpInfo": {
	///      "$ref": "#/components/schemas/UlclBpInformation"
	///    },
	///    "unavailableAccessInd": {
	///      "$ref": "#/components/schemas/UnavailableAccessIndication"
	///    },
	///    "unknownN1SmInfo": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "upCnxState": {
	///      "$ref": "#/components/schemas/UpCnxState"
	///    },
	///    "upSecurityInfo": {
	///      "$ref": "#/components/schemas/UpSecurityInfo"
	///    },
	///    "vSmfServiceInstanceId": {
	///      "type": "string"
	///    },
	///    "vcnTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "vplmnQos": {
	///      "$ref": "#/components/schemas/VplmnQos"
	///    },
	///    "vsmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "vsmfPduSessionUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HsmfUpdateData {
		#[serde(
			rename = "addUeLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_ue_location: Option<UserLocation>,
		#[serde(
			rename = "additionalAnType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_an_type: Option<AccessType>,
		#[serde(
			rename = "additionalCnTunnelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_cn_tunnel_info: Option<TunnelInfo>,
		#[serde(rename = "alwaysOnRequested", default)]
		pub always_on_requested: bool,
		#[serde(rename = "amfNfId", default, skip_serializing_if = "Option::is_none")]
		pub amf_nf_id: Option<NfInstanceId>,
		#[serde(rename = "anType", default, skip_serializing_if = "Option::is_none")]
		pub an_type: Option<AccessType>,
		#[serde(rename = "anTypeCanBeChanged", default)]
		pub an_type_can_be_changed: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<Cause>,
		#[serde(
			rename = "disasterRoamingInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub disaster_roaming_ind: Option<bool>,
		#[serde(
			rename = "dlServingPlmnRateCtl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dl_serving_plmn_rate_ctl: Option<i64>,
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<Dnai>,
		#[serde(rename = "epsBearerId", default, skip_serializing_if = "Vec::is_empty")]
		pub eps_bearer_id: Vec<EpsBearerId>,
		#[serde(
			rename = "epsInterworkingInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_interworking_ind: Option<EpsInterworkingIndication>,
		#[serde(
			rename = "5gMmCauseValue",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_g_mm_cause_value: Option<Uinteger>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<crate::common::common_models::Guami>,
		#[serde(
			rename = "hoPreparationIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ho_preparation_indication: Option<bool>,
		#[serde(
			rename = "iSmfServiceInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub i_smf_service_instance_id: Option<String>,
		#[serde(
			rename = "icnTunnelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub icn_tunnel_info: Option<TunnelInfo>,
		#[serde(rename = "ismfId", default, skip_serializing_if = "Option::is_none")]
		pub ismf_id: Option<NfInstanceId>,
		#[serde(
			rename = "ismfPduSessionUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ismf_pdu_session_uri: Option<Uri>,
		#[serde(rename = "maNwUpgradeInd", default)]
		pub ma_nw_upgrade_ind: bool,
		#[serde(
			rename = "maReleaseInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ma_release_ind: Option<MaReleaseIndication>,
		#[serde(rename = "maRequestInd", default)]
		pub ma_request_ind: bool,
		#[serde(
			rename = "maxIntegrityProtectedDataRateDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_integrity_protected_data_rate_dl: Option<MaxIntegrityProtectedDataRate>,
		#[serde(
			rename = "maxIntegrityProtectedDataRateUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_integrity_protected_data_rate_ul: Option<MaxIntegrityProtectedDataRate>,
		#[serde(
			rename = "moExpDataCounter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mo_exp_data_counter: Option<MoExpDataCounter>,
		#[serde(
			rename = "n1SmInfoFromUe",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_sm_info_from_ue: Option<RefToBinaryData>,
		#[serde(rename = "n4Info", default, skip_serializing_if = "Option::is_none")]
		pub n4_info: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt1",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext1: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt2",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext2: Option<N4Information>,
		#[serde(rename = "ngApCause", default, skip_serializing_if = "Option::is_none")]
		pub ng_ap_cause: Option<NgApCause>,
		#[serde(rename = "NotifyList", default, skip_serializing_if = "Vec::is_empty")]
		pub notify_list: Vec<PduSessionNotifyItem>,
		#[serde(
			rename = "pauseCharging",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pause_charging: Option<bool>,
		#[serde(
			rename = "pcfUeCallbackInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_ue_callback_info: Option<PcfUeCallbackInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(
			rename = "presenceInLadn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub presence_in_ladn: Option<PresenceState>,
		#[serde(rename = "psaInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub psa_info: Vec<PsaInformation>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pti: Option<ProcedureTransactionId>,
		#[serde(
			rename = "qosFlowsNotifyList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flows_notify_list: Vec<QosFlowNotifyItem>,
		#[serde(
			rename = "qosFlowsRelNotifyList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flows_rel_notify_list: Vec<QosFlowItem>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(rename = "requestIndication")]
		pub request_indication: RequestIndication,
		#[serde(
			rename = "revokeEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub revoke_ebi_list: Vec<EpsBearerId>,
		#[serde(
			rename = "roamingChargingProfile",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub roaming_charging_profile: Option<RoamingChargingProfile>,
		#[serde(
			rename = "satelliteBackhaulCat",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub satellite_backhaul_cat: Option<SatelliteBackhaulCategory>,
		#[serde(
			rename = "secondaryRatUsageDataReportContainer",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub secondary_rat_usage_data_report_container: Vec<SecondaryRatUsageDataReportContainer>,
		#[serde(
			rename = "secondaryRatUsageInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub secondary_rat_usage_info: Vec<SecondaryRatUsageInfo>,
		#[serde(
			rename = "secondaryRatUsageReport",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub secondary_rat_usage_report: Vec<SecondaryRatUsageReport>,
		#[serde(
			rename = "securityResult",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub security_result: Option<SecurityResult>,
		#[serde(
			rename = "servingNetwork",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network: Option<PlmnIdNid>,
		#[serde(
			rename = "smPolicyNotifyInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_policy_notify_ind: Option<bool>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "ueLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_location: Option<UserLocation>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
		#[serde(
			rename = "ulclBpInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ulcl_bp_info: Option<UlclBpInformation>,
		#[serde(
			rename = "unavailableAccessInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub unavailable_access_ind: Option<UnavailableAccessIndication>,
		#[serde(
			rename = "unknownN1SmInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub unknown_n1_sm_info: Option<RefToBinaryData>,
		#[serde(
			rename = "upCnxState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_cnx_state: Option<UpCnxState>,
		#[serde(
			rename = "upSecurityInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_security_info: Option<UpSecurityInfo>,
		#[serde(
			rename = "vSmfServiceInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub v_smf_service_instance_id: Option<String>,
		#[serde(
			rename = "vcnTunnelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vcn_tunnel_info: Option<TunnelInfo>,
		#[serde(rename = "vplmnQos", default, skip_serializing_if = "Option::is_none")]
		pub vplmn_qos: Option<VplmnQos>,
		#[serde(rename = "vsmfId", default, skip_serializing_if = "Option::is_none")]
		pub vsmf_id: Option<NfInstanceId>,
		#[serde(
			rename = "vsmfPduSessionUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vsmf_pdu_session_uri: Option<Uri>,
	}

	impl From<&HsmfUpdateData> for HsmfUpdateData {
		fn from(value: &HsmfUpdateData) -> Self {
			value.clone()
		}
	}

	/// Error within Update Response from H-SMF, or from SMF to I-SMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Error within Update Response from H-SMF, or from SMF to
	/// I-SMF",
	///  "type": "object",
	///  "required": [
	///    "error"
	///  ],
	///  "properties": {
	///    "backOffTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "error": {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    "n1SmInfoToUe": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n1smCause": {
	///      "type": "string",
	///      "pattern": "^[A-F0-9]{2}$"
	///    },
	///    "pti": {
	///      "$ref": "#/components/schemas/ProcedureTransactionId"
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HsmfUpdateError {
		#[serde(
			rename = "backOffTimer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub back_off_timer: Option<DurationSec>,
		pub error: ProblemDetails,
		#[serde(
			rename = "n1SmInfoToUe",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_sm_info_to_ue: Option<RefToBinaryData>,
		#[serde(rename = "n1smCause", default, skip_serializing_if = "Option::is_none")]
		pub n1sm_cause: Option<HsmfUpdateErrorN1smCause>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pti: Option<ProcedureTransactionId>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
	}

	impl From<&HsmfUpdateError> for HsmfUpdateError {
		fn from(value: &HsmfUpdateError) -> Self {
			value.clone()
		}
	}

	/// HsmfUpdateErrorN1smCause
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-F0-9]{2}$"
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
	)]
	pub struct HsmfUpdateErrorN1smCause(String);

	impl ::std::ops::Deref for HsmfUpdateErrorN1smCause {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<HsmfUpdateErrorN1smCause> for String {
		fn from(value: HsmfUpdateErrorN1smCause) -> Self {
			value.0
		}
	}

	impl From<&HsmfUpdateErrorN1smCause> for HsmfUpdateErrorN1smCause {
		fn from(value: &HsmfUpdateErrorN1smCause) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for HsmfUpdateErrorN1smCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-F0-9]{2}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-F0-9]{2}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for HsmfUpdateErrorN1smCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for HsmfUpdateErrorN1smCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for HsmfUpdateErrorN1smCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for HsmfUpdateErrorN1smCause {
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

	/// Data within Update Response from H-SMF, or from SMF to I-SMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Update Response from H-SMF, or from SMF to
	/// I-SMF",
	///  "type": "object",
	///  "properties": {
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "epsBearerInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "epsPdnCnxInfo": {
	///      "$ref": "#/components/schemas/EpsPdnCnxInfo"
	///    },
	///    "homeProvidedChargingId": {
	///      "type": "string",
	///      "pattern": "^(0|([1-9]{1}[0-9]{0,9}))$"
	///    },
	///    "interPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "intraPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "ipv6MultiHomingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "maxIntegrityProtectedDataRateDl": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "maxIntegrityProtectedDataRateUl": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "n1SmInfoToUe": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n4Info": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt1": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt2": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "pti": {
	///      "$ref": "#/components/schemas/ProcedureTransactionId"
	///    },
	///    "qosFlowsSetupList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowSetupItem"
	///      },
	///      "minItems": 1
	///    },
	///    "roamingChargingProfile": {
	///      "$ref": "#/components/schemas/RoamingChargingProfile"
	///    },
	///    "sessionAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "upSecurity": {
	///      "$ref": "#/components/schemas/UpSecurity"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HsmfUpdatedData {
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<Dnai>,
		#[serde(
			rename = "epsBearerInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub eps_bearer_info: Vec<EpsBearerInfo>,
		#[serde(
			rename = "epsPdnCnxInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_pdn_cnx_info: Option<EpsPdnCnxInfo>,
		#[serde(
			rename = "homeProvidedChargingId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub home_provided_charging_id: Option<HsmfUpdatedDataHomeProvidedChargingId>,
		#[serde(
			rename = "interPlmnApiRoot",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_api_root: Option<Uri>,
		#[serde(
			rename = "intraPlmnApiRoot",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub intra_plmn_api_root: Option<Uri>,
		#[serde(rename = "ipv6MultiHomingInd", default)]
		pub ipv6_multi_homing_ind: bool,
		#[serde(
			rename = "maxIntegrityProtectedDataRateDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_integrity_protected_data_rate_dl: Option<MaxIntegrityProtectedDataRate>,
		#[serde(
			rename = "maxIntegrityProtectedDataRateUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_integrity_protected_data_rate_ul: Option<MaxIntegrityProtectedDataRate>,
		#[serde(
			rename = "n1SmInfoToUe",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_sm_info_to_ue: Option<RefToBinaryData>,
		#[serde(rename = "n4Info", default, skip_serializing_if = "Option::is_none")]
		pub n4_info: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt1",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext1: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt2",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext2: Option<N4Information>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pti: Option<ProcedureTransactionId>,
		#[serde(
			rename = "qosFlowsSetupList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flows_setup_list: Vec<QosFlowSetupItem>,
		#[serde(
			rename = "roamingChargingProfile",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub roaming_charging_profile: Option<RoamingChargingProfile>,
		#[serde(
			rename = "sessionAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub session_ambr: Option<Ambr>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "upSecurity",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_security: Option<UpSecurity>,
	}

	impl From<&HsmfUpdatedData> for HsmfUpdatedData {
		fn from(value: &HsmfUpdatedData) -> Self {
			value.clone()
		}
	}

	/// HsmfUpdatedDataHomeProvidedChargingId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^(0|([1-9]{1}[0-9]{0,9}))$"
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
	)]
	pub struct HsmfUpdatedDataHomeProvidedChargingId(String);

	impl ::std::ops::Deref for HsmfUpdatedDataHomeProvidedChargingId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<HsmfUpdatedDataHomeProvidedChargingId> for String {
		fn from(value: HsmfUpdatedDataHomeProvidedChargingId) -> Self {
			value.0
		}
	}

	impl From<&HsmfUpdatedDataHomeProvidedChargingId> for HsmfUpdatedDataHomeProvidedChargingId {
		fn from(value: &HsmfUpdatedDataHomeProvidedChargingId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for HsmfUpdatedDataHomeProvidedChargingId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^(0|([1-9]{1}[0-9]{0,9}))$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^(0|([1-9]{1}[0-9]{0,9}))$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for HsmfUpdatedDataHomeProvidedChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for HsmfUpdatedDataHomeProvidedChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for HsmfUpdatedDataHomeProvidedChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for HsmfUpdatedDataHomeProvidedChargingId {
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

	/// Indirect Data Forwarding Tunnel Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indirect Data Forwarding Tunnel Information",
	///  "type": "object",
	///  "not": {
	///    "required": [
	///      "additionalTnlNb",
	///      "drbId"
	///    ]
	///  },
	///  "required": [
	///    "gtpTeid"
	///  ],
	///  "properties": {
	///    "additionalTnlNb": {
	///      "$ref": "#/components/schemas/AdditionalTnlNb"
	///    },
	///    "drbId": {
	///      "$ref": "#/components/schemas/DrbId"
	///    },
	///    "gtpTeid": {
	///      "$ref": "#/components/schemas/Teid"
	///    },
	///    "ipv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct IndirectDataForwardingTunnelInfo {
		#[serde(rename = "gtpTeid")]
		pub gtp_teid: Teid,
		#[serde(rename = "ipv4Addr", default, skip_serializing_if = "Option::is_none")]
		pub ipv4_addr: Option<Ipv4Addr>,
		#[serde(rename = "ipv6Addr", default, skip_serializing_if = "Option::is_none")]
		pub ipv6_addr: Option<Ipv6Addr>,
	}

	impl From<&IndirectDataForwardingTunnelInfo> for IndirectDataForwardingTunnelInfo {
		fn from(value: &IndirectDataForwardingTunnelInfo) -> Self {
			value.clone()
		}
	}

	/// string with format 'int64' as defined in OpenAPI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'int64' as defined in OpenAPI.",
	///  "type": "integer",
	///  "format": "int64"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Int64(pub i64);

	impl ::std::ops::Deref for Int64 {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<Int64> for i64 {
		fn from(value: Int64) -> Self {
			value.0
		}
	}

	impl From<&Int64> for Int64 {
		fn from(value: &Int64) -> Self {
			value.clone()
		}
	}

	impl From<i64> for Int64 {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Int64 {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Int64 {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Int64 {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Int64 {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Int64 {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// It contains an invalid parameter and a related description.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It contains an invalid parameter and a related
	/// description.",
	///  "type": "object",
	///  "required": [
	///    "param"
	///  ],
	///  "properties": {
	///    "param": {
	///      "description": "If the invalid parameter is an attribute in a JSON
	/// body, this IE shall contain the  attribute's name and shall be encoded
	/// as a JSON Pointer. If the invalid parameter is  an HTTP header, this IE
	/// shall be formatted as the concatenation of the string \"header \"  plus
	/// the name of such header. If the invalid parameter is a query parameter,
	/// this IE  shall be formatted as the concatenation of the string \"query
	/// \" plus the name of such  query parameter. If the invalid parameter is a
	/// variable part in the path of a resource  URI, this IE shall contain the
	/// name of the variable, including the symbols \"{\" and \"}\"  used in
	/// OpenAPI specification as the notation to represent variable path
	/// segments.\n",
	///      "type": "string"
	///    },
	///    "reason": {
	///      "description": "A human-readable reason, e.g. \"must be a positive
	/// integer\". In cases involving failed  operations in a PATCH request, the
	/// reason string should identify the operation that  failed using the
	/// operation's array index to assist in correlation of the invalid
	/// parameter with the failed operation, e.g.\" Replacement value invalid
	/// for attribute  (failed operation index= 4)\"\n",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct InvalidParam {
		/// If the invalid parameter is an attribute in a JSON body, this IE
		/// shall contain the  attribute's name and shall be encoded as a JSON
		/// Pointer. If the invalid parameter is  an HTTP header, this IE shall
		/// be formatted as the concatenation of the string "header "  plus the
		/// name of such header. If the invalid parameter is a query parameter,
		/// this IE  shall be formatted as the concatenation of the string
		/// "query " plus the name of such  query parameter. If the invalid
		/// parameter is a variable part in the path of a resource  URI, this IE
		/// shall contain the name of the variable, including the symbols "{"
		/// and "}"  used in OpenAPI specification as the notation to represent
		/// variable path segments.
		pub param: String,
		/// A human-readable reason, e.g. "must be a positive integer". In cases
		/// involving failed  operations in a PATCH request, the reason string
		/// should identify the operation that  failed using the operation's
		/// array index to assist in correlation of the invalid  parameter with
		/// the failed operation, e.g." Replacement value invalid for attribute
		/// (failed operation index= 4)"
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub reason: Option<String>,
	}

	impl From<&InvalidParam> for InvalidParam {
		fn from(value: &InvalidParam) -> Self {
			value.clone()
		}
	}

	/// Contains an IP adresse.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains an IP adresse.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "ipv4Addr"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipv6Addr"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipv6Prefix"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "ipv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "ipv6Prefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum IpAddr {
		#[default]
		Variant0 {
			#[serde(rename = "ipv4Addr")]
			ipv4_addr: Ipv4Addr,
		},
		Variant1 {
			#[serde(rename = "ipv6Addr")]
			ipv6_addr: Ipv6Addr,
		},
		Variant2 {
			#[serde(rename = "ipv6Prefix")]
			ipv6_prefix: Ipv6Prefix,
		},
	}

	impl From<&IpAddr> for IpAddr {
		fn from(value: &IpAddr) -> Self {
			value.clone()
		}
	}

	/// IP Address
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "IP Address",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "ipv4Addr"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipv6Addr"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipv6Prefix"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "ipv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "ipv6Prefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum IpAddress {
		#[default]
		Variant0 {
			#[serde(rename = "ipv4Addr")]
			ipv4_addr: Ipv4Addr,
		},
		Variant1 {
			#[serde(rename = "ipv6Addr")]
			ipv6_addr: Ipv6Addr,
		},
		Variant2 {
			#[serde(rename = "ipv6Prefix")]
			ipv6_prefix: Ipv6Prefix,
		},
	}

	impl From<&IpAddress> for IpAddress {
		fn from(value: &IpAddress) -> Self {
			value.clone()
		}
	}

	/// Represents information that identifies which IP pool or external server
	/// is used to allocate the IP address.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents information that identifies which IP pool or
	/// external server is used to allocate the IP address.\n",
	///  "type": "integer"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct IpIndex(pub i64);

	impl ::std::ops::Deref for IpIndex {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<IpIndex> for i64 {
		fn from(value: IpIndex) -> Self {
			value.0
		}
	}

	impl From<&IpIndex> for IpIndex {
		fn from(value: &IpIndex) -> Self {
			value.clone()
		}
	}

	impl From<i64> for IpIndex {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for IpIndex {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for IpIndex {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for IpIndex {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for IpIndex {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for IpIndex {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// String identifying a IPv4 address formatted in the 'dotted decimal'
	/// notation as defined in RFC 1166.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a IPv4 address formatted in the
	/// 'dotted decimal' notation as defined in RFC 1166.\n",
	///  "examples": [
	///    "198.51.100.1"
	///  ],
	///  "type": "string",
	///  "pattern":
	/// "^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.
	/// ){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])$"
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
	)]
	pub struct Ipv4Addr(String);

	impl ::std::ops::Deref for Ipv4Addr {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Ipv4Addr> for String {
		fn from(value: Ipv4Addr) -> Self {
			value.0
		}
	}

	impl From<&Ipv4Addr> for Ipv4Addr {
		fn from(value: &Ipv4Addr) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Ipv4Addr {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.\
				 ){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.\
				            ){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Ipv4Addr {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Ipv4Addr {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Ipv4Addr {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Ipv4Addr {
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
	)]
	pub struct Ipv6Addr(String);

	impl ::std::ops::Deref for Ipv6Addr {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Ipv6Addr> for String {
		fn from(value: Ipv6Addr) -> Self {
			value.0
		}
	}

	impl From<&Ipv6Addr> for Ipv6Addr {
		fn from(value: &Ipv6Addr) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Ipv6Addr {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"(?=.*^((:|(0?|([1-9a-f][0-9a-f]{0,3}))):)((0?|([1-9a-f][0-9a-f]{0,3})):){0,6}(:\
				 |(0?|([1-9a-f][0-9a-f]{0,3})))$)(?=.*^((([^:]+:){7}([^:]+))|((([^:]+:)*[^:]+)?\
				 ::(([^:]+:)*[^:]+)?))$)",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"(?=.*^((:|(0?|([1-9a-f][0-9a-f]{0,3}))):)((0?|([1-9a-f][0-9a-f]{0,\
				            3})):){0,6}(:|(0?|([1-9a-f][0-9a-f]{0,3})))$)(?=.*^((([^:]+:){7}([^:\
				            ]+))|((([^:]+:)*[^:]+)?::(([^:]+:)*[^:]+)?))$)\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Ipv6Addr {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Ipv6Addr {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Ipv6Addr {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Ipv6Addr {
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
	)]
	pub struct Ipv6Prefix(String);

	impl ::std::ops::Deref for Ipv6Prefix {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Ipv6Prefix> for String {
		fn from(value: Ipv6Prefix) -> Self {
			value.0
		}
	}

	impl From<&Ipv6Prefix> for Ipv6Prefix {
		fn from(value: &Ipv6Prefix) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Ipv6Prefix {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"(?=.*^((:|(0?|([1-9a-f][0-9a-f]{0,3}))):)((0?|([1-9a-f][0-9a-f]{0,3})):){0,6}(:\
				 |(0?|([1-9a-f][0-9a-f]{0,3})))(\\/\
				 (([0-9])|([0-9]{2})|(1[0-1][0-9])|(12[0-8])))$)(?=.*^((([^:]+:){7}([^:]+))|((([^:\
				 ]+:)*[^:]+)?::(([^:]+:)*[^:]+)?))(\\/.+)$)",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"(?=.*^((:|(0?|([1-9a-f][0-9a-f]{0,3}))):)((0?|([1-9a-f][0-9a-f]{0,\
				            3})):){0,6}(:|(0?|([1-9a-f][0-9a-f]{0,3})))(\\/\
				            (([0-9])|([0-9]{2})|(1[0-1][0-9])|(12[0-8])))$)(?=.*^((([^:]+:\
				            ){7}([^:]+))|((([^:]+:)*[^:]+)?::(([^:]+:)*[^:]+)?))(\\/.+)$)\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Ipv6Prefix {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Ipv6Prefix {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Ipv6Prefix {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Ipv6Prefix {
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

	/// Possible values are:
	/// - DSL: Identifies a DSL line
	/// - PON: Identifies a PON line
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- DSL: Identifies a DSL line\n-
	/// PON: Identifies a PON line\n",
	///  "type": "string",
	///  "enum": [
	///    "DSL",
	///    "PON"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum LineType {
		#[default]
		#[serde(rename = "DSL")]
		Dsl,
		#[serde(rename = "PON")]
		Pon,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LineType> for LineType {
		fn from(value: &LineType) -> Self {
			value.clone()
		}
	}

	impl ToString for LineType {
		fn to_string(&self) -> String {
			match *self {
				Self::Dsl => "DSL".to_string(),
				Self::Pon => "PON".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LineType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DSL" => Ok(Self::Dsl),
				"PON" => Ok(Self::Pon),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LineType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LineType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LineType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains a Location area identification as defined in 3GPP TS 23.003,
	/// clause 4.1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a Location area identification as defined in
	/// 3GPP TS 23.003, clause 4.1.",
	///  "type": "object",
	///  "required": [
	///    "lac",
	///    "plmnId"
	///  ],
	///  "properties": {
	///    "lac": {
	///      "description": "Location Area Code.",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{4}$"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LocationAreaId {
		/// Location Area Code.
		pub lac: LocationAreaIdLac,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
	}

	impl From<&LocationAreaId> for LocationAreaId {
		fn from(value: &LocationAreaId) -> Self {
			value.clone()
		}
	}

	/// Location Area Code.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Location Area Code.",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{4}$"
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
	)]
	pub struct LocationAreaIdLac(String);

	impl ::std::ops::Deref for LocationAreaIdLac {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<LocationAreaIdLac> for String {
		fn from(value: LocationAreaIdLac) -> Self {
			value.0
		}
	}

	impl From<&LocationAreaIdLac> for LocationAreaIdLac {
		fn from(value: &LocationAreaIdLac) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for LocationAreaIdLac {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{4}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{4}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for LocationAreaIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for LocationAreaIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for LocationAreaIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for LocationAreaIdLac {
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

	/// Multi-Access PDU session release Indication. Possible values are
	///  - REL_MAPDU_OVER_3GPP
	///  - REL_MAPDU_OVER_N3GPP
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Multi-Access PDU session release Indication. Possible
	/// values are\n  - REL_MAPDU_OVER_3GPP\n  - REL_MAPDU_OVER_N3GPP\n",
	///  "type": "string",
	///  "enum": [
	///    "REL_MAPDU_OVER_3GPP",
	///    "REL_MAPDU_OVER_N3GPP"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum MaReleaseIndication {
		#[default]
		#[serde(rename = "REL_MAPDU_OVER_3GPP")]
		RelMapduOver3gpp,
		#[serde(rename = "REL_MAPDU_OVER_N3GPP")]
		RelMapduOverN3gpp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MaReleaseIndication> for MaReleaseIndication {
		fn from(value: &MaReleaseIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for MaReleaseIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::RelMapduOver3gpp => "REL_MAPDU_OVER_3GPP".to_string(),
				Self::RelMapduOverN3gpp => "REL_MAPDU_OVER_N3GPP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MaReleaseIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REL_MAPDU_OVER_3GPP" => Ok(Self::RelMapduOver3gpp),
				"REL_MAPDU_OVER_N3GPP" => Ok(Self::RelMapduOverN3gpp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MaReleaseIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MaReleaseIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MaReleaseIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// String identifying a MAC address formatted in the hexadecimal notation
	/// according to clause 1.1 and clause 2.1 of RFC 7042.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a MAC address formatted in the
	/// hexadecimal notation according to clause 1.1 and clause 2.1 of RFC
	/// 7042.\n",
	///  "type": "string",
	///  "pattern": "^([0-9a-fA-F]{2})((-[0-9a-fA-F]{2}){5})$"
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
	)]
	pub struct MacAddr48(String);

	impl ::std::ops::Deref for MacAddr48 {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<MacAddr48> for String {
		fn from(value: MacAddr48) -> Self {
			value.0
		}
	}

	impl From<&MacAddr48> for MacAddr48 {
		fn from(value: &MacAddr48) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for MacAddr48 {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^([0-9a-fA-F]{2})((-[0-9a-fA-F]{2}){5})$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err(
					"doesn't match pattern \"^([0-9a-fA-F]{2})((-[0-9a-fA-F]{2}){5})$\"".into(),
				);
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for MacAddr48 {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for MacAddr48 {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for MacAddr48 {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for MacAddr48 {
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

	/// Unsigned integer indicating Maximum Data Burst Volume (see clauses
	/// 5.7.3.7 and 5.7.4 of 3GPP TS 23.501), expressed in Bytes.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer indicating Maximum Data Burst Volume
	/// (see clauses 5.7.3.7 and 5.7.4 of 3GPP TS 23.501), expressed in Bytes.
	/// \n",
	///  "type": "integer",
	///  "maximum": 4095.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MaxDataBurstVol(pub i64);

	impl ::std::ops::Deref for MaxDataBurstVol {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<MaxDataBurstVol> for i64 {
		fn from(value: MaxDataBurstVol) -> Self {
			value.0
		}
	}

	impl From<&MaxDataBurstVol> for MaxDataBurstVol {
		fn from(value: &MaxDataBurstVol) -> Self {
			value.clone()
		}
	}

	impl From<i64> for MaxDataBurstVol {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for MaxDataBurstVol {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for MaxDataBurstVol {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MaxDataBurstVol {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MaxDataBurstVol {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for MaxDataBurstVol {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Maximum Integrity Protected Data Rate. Possible values are
	///  - 64_KBPS
	///  - MAX_UE_RATE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Maximum Integrity Protected Data Rate. Possible values
	/// are\n  - 64_KBPS\n  - MAX_UE_RATE\n",
	///  "type": "string",
	///  "enum": [
	///    "64_KBPS",
	///    "MAX_UE_RATE"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum MaxIntegrityProtectedDataRate {
		#[default]
		#[serde(rename = "64_KBPS")]
		SixtyFourKbps,
		#[serde(rename = "MAX_UE_RATE")]
		MaxUeRate,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MaxIntegrityProtectedDataRate> for MaxIntegrityProtectedDataRate {
		fn from(value: &MaxIntegrityProtectedDataRate) -> Self {
			value.clone()
		}
	}

	impl ToString for MaxIntegrityProtectedDataRate {
		fn to_string(&self) -> String {
			match *self {
				Self::SixtyFourKbps => "64_KBPS".to_string(),
				Self::MaxUeRate => "MAX_UE_RATE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MaxIntegrityProtectedDataRate {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"64_KBPS" => Ok(Self::SixtyFourKbps),
				"MAX_UE_RATE" => Ok(Self::MaxUeRate),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MaxIntegrityProtectedDataRate {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MaxIntegrityProtectedDataRate {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MaxIntegrityProtectedDataRate {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

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
	pub struct Mcc(String);

	impl ::std::ops::Deref for Mcc {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Mcc> for String {
		fn from(value: Mcc) -> Self {
			value.0
		}
	}

	impl From<&Mcc> for Mcc {
		fn from(value: &Mcc) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Mcc {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^\\d{3}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^\\d{3}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Mcc {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Mcc {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Mcc {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Mcc {
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

	/// MME capabilities
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MME capabilities",
	///  "type": "object",
	///  "properties": {
	///    "ethernetSupported": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "nonIpSupported": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "upipSupported": {
	///      "default": false,
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MmeCapabilities {
		#[serde(rename = "ethernetSupported", default)]
		pub ethernet_supported: bool,
		#[serde(rename = "nonIpSupported", default)]
		pub non_ip_supported: bool,
		#[serde(rename = "upipSupported", default)]
		pub upip_supported: bool,
	}

	impl From<&MmeCapabilities> for MmeCapabilities {
		fn from(value: &MmeCapabilities) -> Self {
			value.clone()
		}
	}

	/// Mobile Network Code part of the PLMN, comprising 2 or 3 digits, as
	/// defined in clause 9.3.3.5 of 3GPP TS 38.413.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Mobile Network Code part of the PLMN, comprising 2 or 3
	/// digits, as defined in clause 9.3.3.5 of 3GPP TS 38.413.",
	///  "type": "string",
	///  "pattern": "^\\d{2,3}$"
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
	)]
	pub struct Mnc(String);

	impl ::std::ops::Deref for Mnc {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Mnc> for String {
		fn from(value: Mnc) -> Self {
			value.0
		}
	}

	impl From<&Mnc> for Mnc {
		fn from(value: &Mnc) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Mnc {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^\\d{2,3}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^\\d{2,3}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Mnc {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Mnc {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Mnc {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Mnc {
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

	/// Contain the MO Exception Data Counter.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contain the MO Exception Data Counter.",
	///  "type": "object",
	///  "required": [
	///    "counter"
	///  ],
	///  "properties": {
	///    "counter": {
	///      "description": "Unsigned integer identifying the MO Exception Data
	/// Counter, as specified in clause 5.31.14.3 of 3GPP TS 23.501.\n",
	///      "type": "integer"
	///    },
	///    "timeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MoExpDataCounter {
		/// Unsigned integer identifying the MO Exception Data Counter, as
		/// specified in clause 5.31.14.3 of 3GPP TS 23.501.
		pub counter: i64,
		#[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
		pub time_stamp: Option<DateTime>,
	}

	impl From<&MoExpDataCounter> for MoExpDataCounter {
		fn from(value: &MoExpDataCounter) -> Self {
			value.clone()
		}
	}

	/// N2 SM Information Type. Possible values are
	/// - PDU_RES_SETUP_REQ
	/// - PDU_RES_SETUP_RSP
	/// - PDU_RES_SETUP_FAIL
	/// - PDU_RES_REL_CMD
	/// - PDU_RES_REL_RSP
	/// - PDU_RES_MOD_REQ
	/// - PDU_RES_MOD_RSP
	/// - PDU_RES_MOD_FAIL
	/// - PDU_RES_NTY
	/// - PDU_RES_NTY_REL
	/// - PDU_RES_MOD_IND
	/// - PDU_RES_MOD_CFM
	/// - PATH_SWITCH_REQ
	/// - PATH_SWITCH_SETUP_FAIL
	/// - PATH_SWITCH_REQ_ACK
	/// - PATH_SWITCH_REQ_FAIL
	/// - HANDOVER_REQUIRED
	/// - HANDOVER_CMD
	/// - HANDOVER_PREP_FAIL
	/// - HANDOVER_REQ_ACK
	/// - HANDOVER_RES_ALLOC_FAIL
	/// - SECONDARY_RAT_USAGE
	/// - PDU_RES_MOD_IND_FAIL
	/// - UE_CONTEXT_RESUME_REQ
	/// - UE_CONTEXT_RESUME_RSP
	/// - UE_CONTEXT_SUSPEND_REQ
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "N2 SM Information Type. Possible values are\n-
	/// PDU_RES_SETUP_REQ\n- PDU_RES_SETUP_RSP\n- PDU_RES_SETUP_FAIL\n-
	/// PDU_RES_REL_CMD\n- PDU_RES_REL_RSP\n- PDU_RES_MOD_REQ\n-
	/// PDU_RES_MOD_RSP\n- PDU_RES_MOD_FAIL\n- PDU_RES_NTY\n- PDU_RES_NTY_REL\n-
	/// PDU_RES_MOD_IND\n- PDU_RES_MOD_CFM\n- PATH_SWITCH_REQ\n-
	/// PATH_SWITCH_SETUP_FAIL\n- PATH_SWITCH_REQ_ACK\n- PATH_SWITCH_REQ_FAIL\n-
	/// HANDOVER_REQUIRED\n- HANDOVER_CMD\n- HANDOVER_PREP_FAIL\n-
	/// HANDOVER_REQ_ACK\n- HANDOVER_RES_ALLOC_FAIL\n- SECONDARY_RAT_USAGE\n-
	/// PDU_RES_MOD_IND_FAIL\n- UE_CONTEXT_RESUME_REQ\n-
	/// UE_CONTEXT_RESUME_RSP\n- UE_CONTEXT_SUSPEND_REQ\n",
	///  "type": "string",
	///  "enum": [
	///    "PDU_RES_SETUP_REQ",
	///    "PDU_RES_SETUP_RSP",
	///    "PDU_RES_SETUP_FAIL",
	///    "PDU_RES_REL_CMD",
	///    "PDU_RES_REL_RSP",
	///    "PDU_RES_MOD_REQ",
	///    "PDU_RES_MOD_RSP",
	///    "PDU_RES_MOD_FAIL",
	///    "PDU_RES_NTY",
	///    "PDU_RES_NTY_REL",
	///    "PDU_RES_MOD_IND",
	///    "PDU_RES_MOD_CFM",
	///    "PATH_SWITCH_REQ",
	///    "PATH_SWITCH_SETUP_FAIL",
	///    "PATH_SWITCH_REQ_ACK",
	///    "PATH_SWITCH_REQ_FAIL",
	///    "HANDOVER_REQUIRED",
	///    "HANDOVER_CMD",
	///    "HANDOVER_PREP_FAIL",
	///    "HANDOVER_REQ_ACK",
	///    "HANDOVER_RES_ALLOC_FAIL",
	///    "SECONDARY_RAT_USAGE",
	///    "PDU_RES_MOD_IND_FAIL",
	///    "UE_CONTEXT_RESUME_REQ",
	///    "UE_CONTEXT_RESUME_RSP",
	///    "UE_CONTEXT_SUSPEND_REQ"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum N2SmInfoType {
		#[default]
		#[serde(rename = "PDU_RES_SETUP_REQ")]
		PduResSetupReq,
		#[serde(rename = "PDU_RES_SETUP_RSP")]
		PduResSetupRsp,
		#[serde(rename = "PDU_RES_SETUP_FAIL")]
		PduResSetupFail,
		#[serde(rename = "PDU_RES_REL_CMD")]
		PduResRelCmd,
		#[serde(rename = "PDU_RES_REL_RSP")]
		PduResRelRsp,
		#[serde(rename = "PDU_RES_MOD_REQ")]
		PduResModReq,
		#[serde(rename = "PDU_RES_MOD_RSP")]
		PduResModRsp,
		#[serde(rename = "PDU_RES_MOD_FAIL")]
		PduResModFail,
		#[serde(rename = "PDU_RES_NTY")]
		PduResNty,
		#[serde(rename = "PDU_RES_NTY_REL")]
		PduResNtyRel,
		#[serde(rename = "PDU_RES_MOD_IND")]
		PduResModInd,
		#[serde(rename = "PDU_RES_MOD_CFM")]
		PduResModCfm,
		#[serde(rename = "PATH_SWITCH_REQ")]
		PathSwitchReq,
		#[serde(rename = "PATH_SWITCH_SETUP_FAIL")]
		PathSwitchSetupFail,
		#[serde(rename = "PATH_SWITCH_REQ_ACK")]
		PathSwitchReqAck,
		#[serde(rename = "PATH_SWITCH_REQ_FAIL")]
		PathSwitchReqFail,
		#[serde(rename = "HANDOVER_REQUIRED")]
		HandoverRequired,
		#[serde(rename = "HANDOVER_CMD")]
		HandoverCmd,
		#[serde(rename = "HANDOVER_PREP_FAIL")]
		HandoverPrepFail,
		#[serde(rename = "HANDOVER_REQ_ACK")]
		HandoverReqAck,
		#[serde(rename = "HANDOVER_RES_ALLOC_FAIL")]
		HandoverResAllocFail,
		#[serde(rename = "SECONDARY_RAT_USAGE")]
		SecondaryRatUsage,
		#[serde(rename = "PDU_RES_MOD_IND_FAIL")]
		PduResModIndFail,
		#[serde(rename = "UE_CONTEXT_RESUME_REQ")]
		UeContextResumeReq,
		#[serde(rename = "UE_CONTEXT_RESUME_RSP")]
		UeContextResumeRsp,
		#[serde(rename = "UE_CONTEXT_SUSPEND_REQ")]
		UeContextSuspendReq,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&N2SmInfoType> for N2SmInfoType {
		fn from(value: &N2SmInfoType) -> Self {
			value.clone()
		}
	}

	impl ToString for N2SmInfoType {
		fn to_string(&self) -> String {
			match *self {
				Self::PduResSetupReq => "PDU_RES_SETUP_REQ".to_string(),
				Self::PduResSetupRsp => "PDU_RES_SETUP_RSP".to_string(),
				Self::PduResSetupFail => "PDU_RES_SETUP_FAIL".to_string(),
				Self::PduResRelCmd => "PDU_RES_REL_CMD".to_string(),
				Self::PduResRelRsp => "PDU_RES_REL_RSP".to_string(),
				Self::PduResModReq => "PDU_RES_MOD_REQ".to_string(),
				Self::PduResModRsp => "PDU_RES_MOD_RSP".to_string(),
				Self::PduResModFail => "PDU_RES_MOD_FAIL".to_string(),
				Self::PduResNty => "PDU_RES_NTY".to_string(),
				Self::PduResNtyRel => "PDU_RES_NTY_REL".to_string(),
				Self::PduResModInd => "PDU_RES_MOD_IND".to_string(),
				Self::PduResModCfm => "PDU_RES_MOD_CFM".to_string(),
				Self::PathSwitchReq => "PATH_SWITCH_REQ".to_string(),
				Self::PathSwitchSetupFail => "PATH_SWITCH_SETUP_FAIL".to_string(),
				Self::PathSwitchReqAck => "PATH_SWITCH_REQ_ACK".to_string(),
				Self::PathSwitchReqFail => "PATH_SWITCH_REQ_FAIL".to_string(),
				Self::HandoverRequired => "HANDOVER_REQUIRED".to_string(),
				Self::HandoverCmd => "HANDOVER_CMD".to_string(),
				Self::HandoverPrepFail => "HANDOVER_PREP_FAIL".to_string(),
				Self::HandoverReqAck => "HANDOVER_REQ_ACK".to_string(),
				Self::HandoverResAllocFail => "HANDOVER_RES_ALLOC_FAIL".to_string(),
				Self::SecondaryRatUsage => "SECONDARY_RAT_USAGE".to_string(),
				Self::PduResModIndFail => "PDU_RES_MOD_IND_FAIL".to_string(),
				Self::UeContextResumeReq => "UE_CONTEXT_RESUME_REQ".to_string(),
				Self::UeContextResumeRsp => "UE_CONTEXT_RESUME_RSP".to_string(),
				Self::UeContextSuspendReq => "UE_CONTEXT_SUSPEND_REQ".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for N2SmInfoType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PDU_RES_SETUP_REQ" => Ok(Self::PduResSetupReq),
				"PDU_RES_SETUP_RSP" => Ok(Self::PduResSetupRsp),
				"PDU_RES_SETUP_FAIL" => Ok(Self::PduResSetupFail),
				"PDU_RES_REL_CMD" => Ok(Self::PduResRelCmd),
				"PDU_RES_REL_RSP" => Ok(Self::PduResRelRsp),
				"PDU_RES_MOD_REQ" => Ok(Self::PduResModReq),
				"PDU_RES_MOD_RSP" => Ok(Self::PduResModRsp),
				"PDU_RES_MOD_FAIL" => Ok(Self::PduResModFail),
				"PDU_RES_NTY" => Ok(Self::PduResNty),
				"PDU_RES_NTY_REL" => Ok(Self::PduResNtyRel),
				"PDU_RES_MOD_IND" => Ok(Self::PduResModInd),
				"PDU_RES_MOD_CFM" => Ok(Self::PduResModCfm),
				"PATH_SWITCH_REQ" => Ok(Self::PathSwitchReq),
				"PATH_SWITCH_SETUP_FAIL" => Ok(Self::PathSwitchSetupFail),
				"PATH_SWITCH_REQ_ACK" => Ok(Self::PathSwitchReqAck),
				"PATH_SWITCH_REQ_FAIL" => Ok(Self::PathSwitchReqFail),
				"HANDOVER_REQUIRED" => Ok(Self::HandoverRequired),
				"HANDOVER_CMD" => Ok(Self::HandoverCmd),
				"HANDOVER_PREP_FAIL" => Ok(Self::HandoverPrepFail),
				"HANDOVER_REQ_ACK" => Ok(Self::HandoverReqAck),
				"HANDOVER_RES_ALLOC_FAIL" => Ok(Self::HandoverResAllocFail),
				"SECONDARY_RAT_USAGE" => Ok(Self::SecondaryRatUsage),
				"PDU_RES_MOD_IND_FAIL" => Ok(Self::PduResModIndFail),
				"UE_CONTEXT_RESUME_REQ" => Ok(Self::UeContextResumeReq),
				"UE_CONTEXT_RESUME_RSP" => Ok(Self::UeContextResumeRsp),
				"UE_CONTEXT_SUSPEND_REQ" => Ok(Self::UeContextSuspendReq),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for N2SmInfoType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for N2SmInfoType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for N2SmInfoType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// This represents the identifier of the N3IWF ID as specified in clause
	/// 9.3.1.57 of  3GPP TS 38.413 in hexadecimal representation. Each
	/// character in the string shall take a value  of "0" to "9", "a" to "f" or
	/// "A" to "F" and shall represent 4 bits. The most significant  character
	/// representing the 4 most significant bits of the N3IWF ID shall appear
	/// first in the  string, and the character representing the 4 least
	/// significant bit of the N3IWF ID shall  appear last in the string.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This represents the identifier of the N3IWF ID as
	/// specified in clause 9.3.1.57 of  3GPP TS 38.413 in hexadecimal
	/// representation. Each character in the string shall take a value  of
	/// \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4
	/// bits. The most significant  character representing the 4 most
	/// significant bits of the N3IWF ID shall appear first in the  string, and
	/// the character representing the 4 least significant bit of the N3IWF ID
	/// shall  appear last in the string. \n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]+$"
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
	)]
	pub struct N3IwfId(String);

	impl ::std::ops::Deref for N3IwfId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<N3IwfId> for String {
		fn from(value: N3IwfId) -> Self {
			value.0
		}
	}

	impl From<&N3IwfId> for N3IwfId {
		fn from(value: &N3IwfId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for N3IwfId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for N3IwfId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for N3IwfId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for N3IwfId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for N3IwfId {
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

	/// Contains the Non-3GPP access user location.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Non-3GPP access user location.",
	///  "type": "object",
	///  "properties": {
	///    "gci": {
	///      "$ref": "#/components/schemas/Gci"
	///    },
	///    "gli": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "hfcNodeId": {
	///      "$ref": "#/components/schemas/HfcNodeId"
	///    },
	///    "n3IwfId": {
	///      "description": "This IE shall contain the N3IWF identifier received
	/// over NGAP and shall be encoded as a  string of hexadecimal characters.
	/// Each character in the string shall take a value of \"0\"  to \"9\",
	/// \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4 bits. The most
	/// significant  character representing the 4 most significant bits of the
	/// N3IWF ID shall appear first in  the string, and the character
	/// representing the 4 least significant bit of the N3IWF ID  shall appear
	/// last in the string. \n",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]+$"
	///    },
	///    "n3gppTai": {
	///      "$ref": "#/components/schemas/Tai"
	///    },
	///    "portNumber": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "protocol": {
	///      "$ref": "#/components/schemas/TransportProtocol"
	///    },
	///    "tnapId": {
	///      "$ref": "#/components/schemas/TnapId"
	///    },
	///    "twapId": {
	///      "$ref": "#/components/schemas/TwapId"
	///    },
	///    "ueIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ueIpv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "w5gbanLineType": {
	///      "$ref": "#/components/schemas/LineType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N3gaLocation {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gci: Option<Gci>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gli: Option<Bytes>,
		#[serde(rename = "hfcNodeId", default, skip_serializing_if = "Option::is_none")]
		pub hfc_node_id: Option<HfcNodeId>,
		/// This IE shall contain the N3IWF identifier received over NGAP and
		/// shall be encoded as a  string of hexadecimal characters. Each
		/// character in the string shall take a value of "0"  to "9", "a" to
		/// "f" or "A" to "F" and shall represent 4 bits. The most significant
		/// character representing the 4 most significant bits of the N3IWF ID
		/// shall appear first in  the string, and the character representing
		/// the 4 least significant bit of the N3IWF ID  shall appear last in
		/// the string.
		#[serde(rename = "n3IwfId", default, skip_serializing_if = "Option::is_none")]
		pub n3_iwf_id: Option<N3gaLocationN3IwfId>,
		#[serde(rename = "n3gppTai", default, skip_serializing_if = "Option::is_none")]
		pub n3gpp_tai: Option<Tai>,
		#[serde(
			rename = "portNumber",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub port_number: Option<Uinteger>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub protocol: Option<TransportProtocol>,
		#[serde(rename = "tnapId", default, skip_serializing_if = "Option::is_none")]
		pub tnap_id: Option<TnapId>,
		#[serde(rename = "twapId", default, skip_serializing_if = "Option::is_none")]
		pub twap_id: Option<TwapId>,
		#[serde(
			rename = "ueIpv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_ipv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "ueIpv6Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_ipv6_addr: Option<Ipv6Addr>,
		#[serde(
			rename = "w5gbanLineType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub w5gban_line_type: Option<LineType>,
	}

	impl From<&N3gaLocation> for N3gaLocation {
		fn from(value: &N3gaLocation) -> Self {
			value.clone()
		}
	}

	/// This IE shall contain the N3IWF identifier received over NGAP and shall
	/// be encoded as a  string of hexadecimal characters. Each character in the
	/// string shall take a value of "0"  to "9", "a" to "f" or "A" to "F" and
	/// shall represent 4 bits. The most significant  character representing the
	/// 4 most significant bits of the N3IWF ID shall appear first in  the
	/// string, and the character representing the 4 least significant bit of
	/// the N3IWF ID  shall appear last in the string.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This IE shall contain the N3IWF identifier received
	/// over NGAP and shall be encoded as a  string of hexadecimal characters.
	/// Each character in the string shall take a value of \"0\"  to \"9\",
	/// \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4 bits. The most
	/// significant  character representing the 4 most significant bits of the
	/// N3IWF ID shall appear first in  the string, and the character
	/// representing the 4 least significant bit of the N3IWF ID  shall appear
	/// last in the string. \n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]+$"
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
	)]
	pub struct N3gaLocationN3IwfId(String);

	impl ::std::ops::Deref for N3gaLocationN3IwfId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<N3gaLocationN3IwfId> for String {
		fn from(value: N3gaLocationN3IwfId) -> Self {
			value.0
		}
	}

	impl From<&N3gaLocationN3IwfId> for N3gaLocationN3IwfId {
		fn from(value: &N3gaLocationN3IwfId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for N3gaLocationN3IwfId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for N3gaLocationN3IwfId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for N3gaLocationN3IwfId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for N3gaLocationN3IwfId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for N3gaLocationN3IwfId {
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

	/// N4 Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "N4 Information",
	///  "type": "object",
	///  "required": [
	///    "n4MessagePayload",
	///    "n4MessageType"
	///  ],
	///  "properties": {
	///    "n4DnaiInfo": {
	///      "$ref": "#/components/schemas/DnaiInformation"
	///    },
	///    "n4MessagePayload": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n4MessageType": {
	///      "$ref": "#/components/schemas/N4MessageType"
	///    },
	///    "n9UlPdrIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uint16"
	///      },
	///      "minItems": 1
	///    },
	///    "psaUpfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "ulClBpId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N4Information {
		#[serde(
			rename = "n4DnaiInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_dnai_info: Option<DnaiInformation>,
		#[serde(rename = "n4MessagePayload")]
		pub n4_message_payload: RefToBinaryData,
		#[serde(rename = "n4MessageType")]
		pub n4_message_type: N4MessageType,
		#[serde(
			rename = "n9UlPdrIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub n9_ul_pdr_id_list: Vec<Uint16>,
		#[serde(rename = "psaUpfId", default, skip_serializing_if = "Option::is_none")]
		pub psa_upf_id: Option<NfInstanceId>,
		#[serde(rename = "ulClBpId", default, skip_serializing_if = "Option::is_none")]
		pub ul_cl_bp_id: Option<NfInstanceId>,
	}

	impl From<&N4Information> for N4Information {
		fn from(value: &N4Information) -> Self {
			value.clone()
		}
	}

	/// N4 Message Type. Possible values are
	///  - PFCP_SES_EST_REQ
	///  - PFCP_SES_EST_RSP
	///  - PFCP_SES_MOD_REQ
	///  - PFCP_SES_MOD_RSP
	///  - PFCP_SES_DEL_REQ
	///  - PFCP_SES_DEL_RSP
	///  - PFCP_SES_REP_REQ
	///  - PFCP_SES_REP_RSP
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "N4 Message Type. Possible values are\n  -
	/// PFCP_SES_EST_REQ\n  - PFCP_SES_EST_RSP\n  - PFCP_SES_MOD_REQ\n  -
	/// PFCP_SES_MOD_RSP\n  - PFCP_SES_DEL_REQ\n  - PFCP_SES_DEL_RSP\n  -
	/// PFCP_SES_REP_REQ\n  - PFCP_SES_REP_RSP\n",
	///  "type": "string",
	///  "enum": [
	///    "PFCP_SES_EST_REQ",
	///    "PFCP_SES_EST_RSP",
	///    "PFCP_SES_MOD_REQ",
	///    "PFCP_SES_MOD_RSP",
	///    "PFCP_SES_DEL_REQ",
	///    "PFCP_SES_DEL_RSP",
	///    "PFCP_SES_REP_REQ",
	///    "PFCP_SES_REP_RSP"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum N4MessageType {
		#[default]
		#[serde(rename = "PFCP_SES_EST_REQ")]
		PfcpSesEstReq,
		#[serde(rename = "PFCP_SES_EST_RSP")]
		PfcpSesEstRsp,
		#[serde(rename = "PFCP_SES_MOD_REQ")]
		PfcpSesModReq,
		#[serde(rename = "PFCP_SES_MOD_RSP")]
		PfcpSesModRsp,
		#[serde(rename = "PFCP_SES_DEL_REQ")]
		PfcpSesDelReq,
		#[serde(rename = "PFCP_SES_DEL_RSP")]
		PfcpSesDelRsp,
		#[serde(rename = "PFCP_SES_REP_REQ")]
		PfcpSesRepReq,
		#[serde(rename = "PFCP_SES_REP_RSP")]
		PfcpSesRepRsp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&N4MessageType> for N4MessageType {
		fn from(value: &N4MessageType) -> Self {
			value.clone()
		}
	}

	impl ToString for N4MessageType {
		fn to_string(&self) -> String {
			match *self {
				Self::PfcpSesEstReq => "PFCP_SES_EST_REQ".to_string(),
				Self::PfcpSesEstRsp => "PFCP_SES_EST_RSP".to_string(),
				Self::PfcpSesModReq => "PFCP_SES_MOD_REQ".to_string(),
				Self::PfcpSesModRsp => "PFCP_SES_MOD_RSP".to_string(),
				Self::PfcpSesDelReq => "PFCP_SES_DEL_REQ".to_string(),
				Self::PfcpSesDelRsp => "PFCP_SES_DEL_RSP".to_string(),
				Self::PfcpSesRepReq => "PFCP_SES_REP_REQ".to_string(),
				Self::PfcpSesRepRsp => "PFCP_SES_REP_RSP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for N4MessageType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PFCP_SES_EST_REQ" => Ok(Self::PfcpSesEstReq),
				"PFCP_SES_EST_RSP" => Ok(Self::PfcpSesEstRsp),
				"PFCP_SES_MOD_REQ" => Ok(Self::PfcpSesModReq),
				"PFCP_SES_MOD_RSP" => Ok(Self::PfcpSesModRsp),
				"PFCP_SES_DEL_REQ" => Ok(Self::PfcpSesDelReq),
				"PFCP_SES_DEL_RSP" => Ok(Self::PfcpSesDelRsp),
				"PFCP_SES_REP_REQ" => Ok(Self::PfcpSesRepReq),
				"PFCP_SES_REP_RSP" => Ok(Self::PfcpSesRepRsp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for N4MessageType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for N4MessageType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for N4MessageType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the NCGI (NR Cell Global Identity), as described in 3GPP 23.003
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the NCGI (NR Cell Global Identity), as
	/// described in 3GPP 23.003",
	///  "type": "object",
	///  "required": [
	///    "nrCellId",
	///    "plmnId"
	///  ],
	///  "properties": {
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    },
	///    "nrCellId": {
	///      "$ref": "#/components/schemas/NrCellId"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Ncgi {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
		#[serde(rename = "nrCellId")]
		pub nr_cell_id: NrCellId,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
	}

	impl From<&Ncgi> for Ncgi {
		fn from(value: &Ncgi) -> Self {
			value.clone()
		}
	}

	/// Identifier of a group of NFs.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifier of a group of NFs.",
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
	pub struct NfGroupId(pub String);

	impl ::std::ops::Deref for NfGroupId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NfGroupId> for String {
		fn from(value: NfGroupId) -> Self {
			value.0
		}
	}

	impl From<&NfGroupId> for NfGroupId {
		fn from(value: &NfGroupId) -> Self {
			value.clone()
		}
	}

	impl From<String> for NfGroupId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NfGroupId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for NfGroupId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// String uniquely identifying a NF instance. The format of the NF Instance
	/// ID shall be a  Universally Unique Identifier (UUID) version 4, as
	/// described in IETF RFC 4122.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String uniquely identifying a NF instance. The format
	/// of the NF Instance ID shall be a  Universally Unique Identifier (UUID)
	/// version 4, as described in IETF RFC 4122. \n",
	///  "type": "string",
	///  "format": "uuid"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfInstanceId(pub uuid::Uuid);

	impl ::std::ops::Deref for NfInstanceId {
		type Target = uuid::Uuid;
		fn deref(&self) -> &uuid::Uuid {
			&self.0
		}
	}

	impl From<NfInstanceId> for uuid::Uuid {
		fn from(value: NfInstanceId) -> Self {
			value.0
		}
	}

	impl From<&NfInstanceId> for NfInstanceId {
		fn from(value: &NfInstanceId) -> Self {
			value.clone()
		}
	}

	impl From<uuid::Uuid> for NfInstanceId {
		fn from(value: uuid::Uuid) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NfInstanceId {
		type Err = <uuid::Uuid as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for NfInstanceId {
		type Error = <uuid::Uuid as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NfInstanceId {
		type Error = <uuid::Uuid as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NfInstanceId {
		type Error = <uuid::Uuid as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for NfInstanceId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// NF Service Set Identifier (see clause 28.12 of 3GPP TS 23.003) formatted
	/// as the following  string "set<Set ID>.sn<Service Name>.nfi<NF Instance
	/// ID>.5gc.mnc<MNC>.mcc<MCC>", or
	/// "set<SetID>.sn<ServiceName>.nfi<NFInstanceID>.5gc.nid<NID>.mnc<MNC>.
	/// mcc<MCC>" with  <MCC> encoded as defined in clause 5.4.2 ("Mcc" data
	/// type definition)   <MNC> encoding the Mobile Network Code part of the
	/// PLMN, comprising 3 digits.  If there are only 2 significant digits
	/// in the MNC, one "0" digit shall be inserted  at the left side to
	/// fill the 3 digits coding of MNC.  Pattern: '^[0-9]{3}$'
	/// <NID> encoded as defined in clause 5.4.2 ("Nid" data type definition)
	/// <NFInstanceId> encoded as defined in clause 5.3.2  <ServiceName> encoded
	/// as defined in 3GPP TS 29.510  <Set ID> encoded as a string of characters
	/// consisting of alphabetic  characters (A-Z and a-z), digits (0-9)
	/// and/or the hyphen (-) and that shall end  with either an alphabetic
	/// character or a digit.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "NF Service Set Identifier (see clause 28.12 of 3GPP TS
	/// 23.003) formatted as the following  string \"set<Set ID>.sn<Service
	/// Name>.nfi<NF Instance ID>.5gc.mnc<MNC>.mcc<MCC>\", or
	/// \"set<SetID>.sn<ServiceName>.nfi<NFInstanceID>.5gc.nid<NID>.mnc<MNC>.
	/// mcc<MCC>\" with  <MCC> encoded as defined in clause 5.4.2 (\"Mcc\" data
	/// type definition)   <MNC> encoding the Mobile Network Code part of the
	/// PLMN, comprising 3 digits. \n  If there are only 2 significant digits in
	/// the MNC, one \"0\" digit shall be inserted \n  at the left side to fill
	/// the 3 digits coding of MNC.  Pattern: '^[0-9]{3}$'\n<NID> encoded as
	/// defined in clause 5.4.2 (\"Nid\" data type definition)  <NFInstanceId>
	/// encoded as defined in clause 5.3.2  <ServiceName> encoded as defined in
	/// 3GPP TS 29.510  <Set ID> encoded as a string of characters consisting of
	/// alphabetic \n  characters (A-Z and a-z), digits (0-9) and/or the hyphen
	/// (-) and that shall end \n  with either an alphabetic character or a
	/// digit.\n",
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
	pub struct NfServiceSetId(pub String);

	impl ::std::ops::Deref for NfServiceSetId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NfServiceSetId> for String {
		fn from(value: NfServiceSetId) -> Self {
			value.0
		}
	}

	impl From<&NfServiceSetId> for NfServiceSetId {
		fn from(value: &NfServiceSetId) -> Self {
			value.clone()
		}
	}

	impl From<String> for NfServiceSetId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NfServiceSetId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for NfServiceSetId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// NF Set Identifier (see clause 28.12 of 3GPP TS 23.003), formatted as the
	/// following string "set<Set ID>.<nftype>set.5gc.mnc<MNC>.mcc<MCC>", or
	/// "set<SetID>.<NFType>set.5gc.nid<NID>.mnc<MNC>.mcc<MCC>" with  <MCC>
	/// encoded as defined in clause 5.4.2 ("Mcc" data type definition)  <MNC>
	/// encoding the Mobile Network Code part of the PLMN, comprising 3 digits.
	///  If there are only 2 significant digits in the MNC, one "0" digit shall
	/// be inserted  at the left side to fill the 3 digits coding of MNC.
	/// Pattern: '^[0-9]{3}$' <NFType> encoded as a value defined in Table
	/// 6.1.6.3.3-1 of 3GPP TS 29.510 but  with lower case characters <Set
	/// ID> encoded as a string of characters consisting of  alphabetic
	/// characters (A-Z and a-z), digits (0-9) and/or the hyphen (-) and that
	///  shall end with either an alphabetic character or a digit.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "NF Set Identifier (see clause 28.12 of 3GPP TS 23.003),
	/// formatted as the following string \"set<Set
	/// ID>.<nftype>set.5gc.mnc<MNC>.mcc<MCC>\", or
	/// \"set<SetID>.<NFType>set.5gc.nid<NID>.mnc<MNC>.mcc<MCC>\" with  <MCC>
	/// encoded as defined in clause 5.4.2 (\"Mcc\" data type definition)  <MNC>
	/// encoding the Mobile Network Code part of the PLMN, comprising 3 digits.
	/// \n  If there are only 2 significant digits in the MNC, one \"0\" digit
	/// shall be inserted \n  at the left side to fill the 3 digits coding of
	/// MNC.  Pattern: '^[0-9]{3}$'\n<NFType> encoded as a value defined in
	/// Table 6.1.6.3.3-1 of 3GPP TS 29.510 but \n  with lower case characters
	/// <Set ID> encoded as a string of characters consisting of \n  alphabetic
	/// characters (A-Z and a-z), digits (0-9) and/or the hyphen (-) and that \n
	/// shall end with either an alphabetic character or a digit. \n",
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
	pub struct NfSetId(pub String);

	impl ::std::ops::Deref for NfSetId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NfSetId> for String {
		fn from(value: NfSetId) -> Self {
			value.0
		}
	}

	impl From<&NfSetId> for NfSetId {
		fn from(value: &NfSetId) -> Self {
			value.clone()
		}
	}

	impl From<String> for NfSetId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NfSetId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for NfSetId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Represents the NGAP cause.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the NGAP cause.",
	///  "type": "object",
	///  "required": [
	///    "group",
	///    "value"
	///  ],
	///  "properties": {
	///    "group": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "value": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NgApCause {
		pub group: Uinteger,
		pub value: Uinteger,
	}

	impl From<&NgApCause> for NgApCause {
		fn from(value: &NgApCause) -> Self {
			value.clone()
		}
	}

	/// Indicates a NG RAN as target of the handover
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates a NG RAN as target of the handover",
	///  "type": "object",
	///  "required": [
	///    "ranNodeId",
	///    "tai"
	///  ],
	///  "properties": {
	///    "ranNodeId": {
	///      "$ref": "#/components/schemas/GlobalRanNodeId"
	///    },
	///    "tai": {
	///      "$ref": "#/components/schemas/Tai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NgRanTargetId {
		#[serde(rename = "ranNodeId")]
		pub ran_node_id: GlobalRanNodeId,
		pub tai: Tai,
	}

	impl From<&NgRanTargetId> for NgRanTargetId {
		fn from(value: &NgRanTargetId) -> Self {
			value.clone()
		}
	}

	/// This represents the identifier of the ng-eNB ID as specified in clause
	/// 9.3.1.8 of  3GPP TS 38.413. The value of the ng-eNB ID shall be encoded
	/// in hexadecimal representation.  Each character in the string shall take
	/// a value of "0" to "9", "a" to "f" or "A" to "F" and  shall represent 4
	/// bits. The padding 0 shall be added to make multiple nibbles, so the most
	/// significant character representing the padding 0 if required together
	/// with the 4 most  significant bits of the ng-eNB ID shall appear first in
	/// the string, and the character  representing the 4 least significant bit
	/// of the ng-eNB ID (to form a nibble) shall appear last  in the string.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This represents the identifier of the ng-eNB ID as
	/// specified in clause 9.3.1.8 of  3GPP TS 38.413. The value of the ng-eNB
	/// ID shall be encoded in hexadecimal representation.  Each character in
	/// the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or \"A\"
	/// to \"F\" and  shall represent 4 bits. The padding 0 shall be added to
	/// make multiple nibbles, so the most  significant character representing
	/// the padding 0 if required together with the 4 most  significant bits of
	/// the ng-eNB ID shall appear first in the string, and the character
	/// representing the 4 least significant bit of the ng-eNB ID (to form a
	/// nibble) shall appear last  in the string. \n",
	///  "examples": [
	///    "SMacroNGeNB-34B89"
	///  ],
	///  "type": "string",
	///  "pattern":
	/// "^(MacroNGeNB-[A-Fa-f0-9]{5}|LMacroNGeNB-[A-Fa-f0-9]{6}|SMacroNGeNB-[A-Fa-f0-9]{5})$"
	///
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
	)]
	pub struct NgeNbId(String);

	impl ::std::ops::Deref for NgeNbId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NgeNbId> for String {
		fn from(value: NgeNbId) -> Self {
			value.0
		}
	}

	impl From<&NgeNbId> for NgeNbId {
		fn from(value: &NgeNbId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for NgeNbId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^(MacroNGeNB-[A-Fa-f0-9]{5}|LMacroNGeNB-[A-Fa-f0-9]{6}|SMacroNGeNB-[A-Fa-f0-9]{5})$").unwrap().find(value).is_none() { return Err("doesn't match pattern \"^(MacroNGeNB-[A-Fa-f0-9]{5}|LMacroNGeNB-[A-Fa-f0-9]{6}|SMacroNGeNB-[A-Fa-f0-9]{5})$\"".into()); }
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for NgeNbId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for NgeNbId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for NgeNbId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for NgeNbId {
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

	/// This represents the Network Identifier, which together with a PLMN ID is
	/// used to identify an SNPN (see 3GPP TS 23.003 and 3GPP TS 23.501 clause
	/// 5.30.2.1).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This represents the Network Identifier, which together
	/// with a PLMN ID is used to identify an SNPN (see 3GPP TS 23.003 and 3GPP
	/// TS 23.501 clause 5.30.2.1). \n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{11}$"
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
	)]
	pub struct Nid(String);

	impl ::std::ops::Deref for Nid {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Nid> for String {
		fn from(value: Nid) -> Self {
			value.0
		}
	}

	impl From<&Nid> for Nid {
		fn from(value: &Nid) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Nid {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{11}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{11}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Nid {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Nid {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Nid {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Nid {
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

	/// It indicates the QoS Characteristics for a standardized or
	/// pre-configured 5QI for downlink and uplink.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It indicates the QoS Characteristics for a standardized
	/// or pre-configured 5QI for downlink and uplink.\n",
	///  "type": "object",
	///  "minProperties": 0,
	///  "properties": {
	///    "averWindow": {
	///      "$ref": "#/components/schemas/AverWindow"
	///    },
	///    "cnPacketDelayBudgetDl": {
	///      "$ref": "#/components/schemas/ExtPacketDelBudget"
	///    },
	///    "cnPacketDelayBudgetUl": {
	///      "$ref": "#/components/schemas/ExtPacketDelBudget"
	///    },
	///    "extMaxDataBurstVol": {
	///      "$ref": "#/components/schemas/ExtMaxDataBurstVol"
	///    },
	///    "maxDataBurstVol": {
	///      "$ref": "#/components/schemas/MaxDataBurstVol"
	///    },
	///    "priorityLevel": {
	///      "$ref": "#/components/schemas/5QiPriorityLevel"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NonDynamic5Qi {
		#[serde(
			rename = "averWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub aver_window: Option<AverWindow>,
		#[serde(
			rename = "cnPacketDelayBudgetDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cn_packet_delay_budget_dl: Option<ExtPacketDelBudget>,
		#[serde(
			rename = "cnPacketDelayBudgetUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cn_packet_delay_budget_ul: Option<ExtPacketDelBudget>,
		#[serde(
			rename = "extMaxDataBurstVol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ext_max_data_burst_vol: Option<ExtMaxDataBurstVol>,
		#[serde(
			rename = "maxDataBurstVol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_data_burst_vol: Option<MaxDataBurstVol>,
		#[serde(
			rename = "priorityLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub priority_level: Option<_5qiPriorityLevel>,
	}

	impl From<&NonDynamic5Qi> for NonDynamic5Qi {
		fn from(value: &NonDynamic5Qi) -> Self {
			value.clone()
		}
	}

	/// Cause for generating a notification. Possible values are
	/// - QOS_FULFILLED
	/// - QOS_NOT_FULFILLED
	/// - UP_SEC_FULFILLED
	/// - UP_SEC_NOT_FULFILLED
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Cause for generating a notification. Possible values
	/// are\n- QOS_FULFILLED\n- QOS_NOT_FULFILLED\n- UP_SEC_FULFILLED\n-
	/// UP_SEC_NOT_FULFILLED\n",
	///  "type": "string",
	///  "enum": [
	///    "QOS_FULFILLED",
	///    "QOS_NOT_FULFILLED",
	///    "UP_SEC_FULFILLED",
	///    "UP_SEC_NOT_FULFILLED"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum NotificationCause {
		#[default]
		#[serde(rename = "QOS_FULFILLED")]
		QosFulfilled,
		#[serde(rename = "QOS_NOT_FULFILLED")]
		QosNotFulfilled,
		#[serde(rename = "UP_SEC_FULFILLED")]
		UpSecFulfilled,
		#[serde(rename = "UP_SEC_NOT_FULFILLED")]
		UpSecNotFulfilled,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NotificationCause> for NotificationCause {
		fn from(value: &NotificationCause) -> Self {
			value.clone()
		}
	}

	impl ToString for NotificationCause {
		fn to_string(&self) -> String {
			match *self {
				Self::QosFulfilled => "QOS_FULFILLED".to_string(),
				Self::QosNotFulfilled => "QOS_NOT_FULFILLED".to_string(),
				Self::UpSecFulfilled => "UP_SEC_FULFILLED".to_string(),
				Self::UpSecNotFulfilled => "UP_SEC_NOT_FULFILLED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NotificationCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"QOS_FULFILLED" => Ok(Self::QosFulfilled),
				"QOS_NOT_FULFILLED" => Ok(Self::QosNotFulfilled),
				"UP_SEC_FULFILLED" => Ok(Self::UpSecFulfilled),
				"UP_SEC_NOT_FULFILLED" => Ok(Self::UpSecNotFulfilled),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NotificationCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NotificationCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NotificationCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration NotificationControl indicates whether notifications are
	/// requested from the RAN when the GFBR can no longer  (or again) be
	/// fulfilled for a QoS Flow during the lifetime of the QoS Flow (see clause
	/// 5.7.2.4 of 3GPP TS 23.501). It shall comply with the provisions defined
	/// in table 5.5.3.5-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration NotificationControl indicates whether
	/// notifications are requested from the RAN when the GFBR can no longer
	/// (or again) be fulfilled for a QoS Flow during the lifetime of the QoS
	/// Flow (see clause 5.7.2.4 of 3GPP TS 23.501). It shall comply with the
	/// provisions defined in table 5.5.3.5-1. \n",
	///  "type": "string",
	///  "enum": [
	///    "REQUESTED",
	///    "NOT_REQUESTED"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum NotificationControl {
		#[default]
		#[serde(rename = "REQUESTED")]
		Requested,
		#[serde(rename = "NOT_REQUESTED")]
		NotRequested,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NotificationControl> for NotificationControl {
		fn from(value: &NotificationControl) -> Self {
			value.clone()
		}
	}

	impl ToString for NotificationControl {
		fn to_string(&self) -> String {
			match *self {
				Self::Requested => "REQUESTED".to_string(),
				Self::NotRequested => "NOT_REQUESTED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NotificationControl {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REQUESTED" => Ok(Self::Requested),
				"NOT_REQUESTED" => Ok(Self::NotRequested),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NotificationControl {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NotificationControl {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NotificationControl {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - ACTIVATE: The event notification is activated.
	/// - DEACTIVATE: The event notification is deactivated and shall be muted.
	///   The available event(s) shall be stored.
	/// - RETRIEVAL: The event notification shall be sent to the NF service
	///   consumer(s),
	///  after that, is muted again.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- ACTIVATE: The event
	/// notification is activated.\n- DEACTIVATE: The event notification is
	/// deactivated and shall be muted. The available\n   event(s) shall be
	/// stored.\n- RETRIEVAL: The event notification shall be sent to the NF
	/// service consumer(s),\n  after that, is muted again. \n",
	///  "type": "string",
	///  "enum": [
	///    "ACTIVATE",
	///    "DEACTIVATE",
	///    "RETRIEVAL"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum NotificationFlag {
		#[default]
		#[serde(rename = "ACTIVATE")]
		Activate,
		#[serde(rename = "DEACTIVATE")]
		Deactivate,
		#[serde(rename = "RETRIEVAL")]
		Retrieval,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NotificationFlag> for NotificationFlag {
		fn from(value: &NotificationFlag) -> Self {
			value.clone()
		}
	}

	impl ToString for NotificationFlag {
		fn to_string(&self) -> String {
			match *self {
				Self::Activate => "ACTIVATE".to_string(),
				Self::Deactivate => "DEACTIVATE".to_string(),
				Self::Retrieval => "RETRIEVAL".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NotificationFlag {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ACTIVATE" => Ok(Self::Activate),
				"DEACTIVATE" => Ok(Self::Deactivate),
				"RETRIEVAL" => Ok(Self::Retrieval),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NotificationFlag {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NotificationFlag {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NotificationFlag {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Notification Correlation ID and Notification URI provided by the NF
	/// service consumer
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Notification Correlation ID and Notification URI
	/// provided by the NF service consumer\n",
	///  "type": "object",
	///  "required": [
	///    "notifId",
	///    "notifUri"
	///  ],
	///  "properties": {
	///    "notifId": {
	///      "type": "string"
	///    },
	///    "notifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "upBufferInd": {
	///      "default": false,
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NotificationInfo {
		#[serde(rename = "notifId")]
		pub notif_id: String,
		#[serde(rename = "notifUri")]
		pub notif_uri: Uri,
		#[serde(rename = "upBufferInd", default)]
		pub up_buffer_ind: bool,
	}

	impl From<&NotificationInfo> for NotificationInfo {
		fn from(value: &NotificationInfo) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - PERIODIC
	/// - ONE_TIME
	/// - ON_EVENT_DETECTION
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- PERIODIC\n- ONE_TIME\n-
	/// ON_EVENT_DETECTION\n",
	///  "type": "string",
	///  "enum": [
	///    "PERIODIC",
	///    "ONE_TIME",
	///    "ON_EVENT_DETECTION"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum NotificationMethod {
		#[default]
		#[serde(rename = "PERIODIC")]
		Periodic,
		#[serde(rename = "ONE_TIME")]
		OneTime,
		#[serde(rename = "ON_EVENT_DETECTION")]
		OnEventDetection,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NotificationMethod> for NotificationMethod {
		fn from(value: &NotificationMethod) -> Self {
			value.clone()
		}
	}

	impl ToString for NotificationMethod {
		fn to_string(&self) -> String {
			match *self {
				Self::Periodic => "PERIODIC".to_string(),
				Self::OneTime => "ONE_TIME".to_string(),
				Self::OnEventDetection => "ON_EVENT_DETECTION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NotificationMethod {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PERIODIC" => Ok(Self::Periodic),
				"ONE_TIME" => Ok(Self::OneTime),
				"ON_EVENT_DETECTION" => Ok(Self::OnEventDetection),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NotificationMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NotificationMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NotificationMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// 36-bit string identifying an NR Cell Id as specified in clause 9.3.1.7
	/// of 3GPP TS 38.413,  in hexadecimal representation. Each character in the
	/// string shall take a value of "0" to "9",  "a" to "f" or "A" to "F" and
	/// shall represent 4 bits. The most significant character  representing the
	/// 4 most significant bits of the Cell Id shall appear first in the string,
	/// and  the character representing the 4 least significant bit of the Cell
	/// Id shall appear last in the  string.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "36-bit string identifying an NR Cell Id as specified in clause 9.3.1.7 of 3GPP TS 38.413,  in hexadecimal representation. Each character in the string shall take a value of \"0\" to \"9\",  \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4 bits. The most significant character  representing the 4 most significant bits of the Cell Id shall appear first in the string, and  the character representing the 4 least significant bit of the Cell Id shall appear last in the  string. \n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{9}$"
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
	)]
	pub struct NrCellId(String);

	impl ::std::ops::Deref for NrCellId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NrCellId> for String {
		fn from(value: NrCellId) -> Self {
			value.0
		}
	}

	impl From<&NrCellId> for NrCellId {
		fn from(value: &NrCellId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for NrCellId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{9}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{9}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for NrCellId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for NrCellId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for NrCellId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for NrCellId {
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

	/// Contains the NR user location.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the NR user location.",
	///  "type": "object",
	///  "required": [
	///    "ncgi",
	///    "tai"
	///  ],
	///  "properties": {
	///    "ageOfLocationInformation": {
	///      "description": "The value represents the elapsed time in minutes
	/// since the last network contact of the mobile station. Value \"0\"
	/// indicates that the location information was obtained after a successful
	/// paging procedure for Active Location Retrieval when the UE is in idle
	/// mode or after a successful  NG-RAN location reporting procedure with the
	/// eNB when the UE is in connected mode. Any other value than \"0\"
	/// indicates that the location information is the last known one. See 3GPP
	/// TS 29.002 clause 17.7.8.\n",
	///      "type": "integer",
	///      "maximum": 32767.0,
	///      "minimum": 0.0
	///    },
	///    "geodeticInformation": {
	///      "description": "Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763 (1999) [24] clause 3.88.2. Only the description of an ellipsoid point with uncertainty circle is allowed to be used.\n",
	///      "type": "string",
	///      "pattern": "^[0-9A-F]{20}$"
	///    },
	///    "geographicalInformation": {
	///      "description": "Refer to geographical Information. See 3GPP TS
	/// 23.032 clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///      "type": "string",
	///      "pattern": "^[0-9A-F]{16}$"
	///    },
	///    "globalGnbId": {
	///      "$ref": "#/components/schemas/GlobalRanNodeId"
	///    },
	///    "ignoreNcgi": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ncgi": {
	///      "$ref": "#/components/schemas/Ncgi"
	///    },
	///    "tai": {
	///      "$ref": "#/components/schemas/Tai"
	///    },
	///    "ueLocationTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrLocation {
		/// The value represents the elapsed time in minutes since the last
		/// network contact of the mobile station. Value "0" indicates that the
		/// location information was obtained after a successful paging
		/// procedure for Active Location Retrieval when the UE is in idle mode
		/// or after a successful  NG-RAN location reporting procedure with the
		/// eNB when the UE is in connected mode. Any other value than "0"
		/// indicates that the location information is the last known one. See
		/// 3GPP TS 29.002 clause 17.7.8.
		#[serde(
			rename = "ageOfLocationInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub age_of_location_information: Option<i64>,
		/// Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763
		/// (1999) [24] clause 3.88.2. Only the description of an ellipsoid
		/// point with uncertainty circle is allowed to be used.
		#[serde(
			rename = "geodeticInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub geodetic_information: Option<NrLocationGeodeticInformation>,
		/// Refer to geographical Information. See 3GPP TS 23.032 clause 7.3.2.
		/// Only the description of an ellipsoid point with uncertainty circle
		/// is allowed to be used.
		#[serde(
			rename = "geographicalInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub geographical_information: Option<NrLocationGeographicalInformation>,
		#[serde(
			rename = "globalGnbId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub global_gnb_id: Option<GlobalRanNodeId>,
		#[serde(rename = "ignoreNcgi", default)]
		pub ignore_ncgi: bool,
		pub ncgi: Ncgi,
		pub tai: Tai,
		#[serde(
			rename = "ueLocationTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_location_timestamp: Option<DateTime>,
	}

	impl From<&NrLocation> for NrLocation {
		fn from(value: &NrLocation) -> Self {
			value.clone()
		}
	}

	/// Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763
	/// (1999) [24] clause 3.88.2. Only the description of an ellipsoid point
	/// with uncertainty circle is allowed to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763 (1999) [24] clause 3.88.2. Only the description of an ellipsoid point with uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{20}$"
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
	)]
	pub struct NrLocationGeodeticInformation(String);

	impl ::std::ops::Deref for NrLocationGeodeticInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NrLocationGeodeticInformation> for String {
		fn from(value: NrLocationGeodeticInformation) -> Self {
			value.0
		}
	}

	impl From<&NrLocationGeodeticInformation> for NrLocationGeodeticInformation {
		fn from(value: &NrLocationGeodeticInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for NrLocationGeodeticInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{20}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{20}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for NrLocationGeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for NrLocationGeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for NrLocationGeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for NrLocationGeodeticInformation {
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

	/// Refer to geographical Information. See 3GPP TS 23.032 clause 7.3.2. Only
	/// the description of an ellipsoid point with uncertainty circle is allowed
	/// to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refer to geographical Information. See 3GPP TS 23.032
	/// clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{16}$"
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
	)]
	pub struct NrLocationGeographicalInformation(String);

	impl ::std::ops::Deref for NrLocationGeographicalInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NrLocationGeographicalInformation> for String {
		fn from(value: NrLocationGeographicalInformation) -> Self {
			value.0
		}
	}

	impl From<&NrLocationGeographicalInformation> for NrLocationGeographicalInformation {
		fn from(value: &NrLocationGeographicalInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for NrLocationGeographicalInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{16}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{16}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for NrLocationGeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for NrLocationGeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for NrLocationGeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for NrLocationGeographicalInformation {
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

	/// Represents an Individual SMF Notification Subscription resource. The
	/// serviveName property corresponds to the serviceName in the main body of
	/// the specification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an Individual SMF Notification Subscription
	/// resource. The serviveName property corresponds to the serviceName in the
	/// main body of the specification.\n",
	///  "type": "object",
	///  "required": [
	///    "eventSubs",
	///    "notifId",
	///    "notifUri"
	///  ],
	///  "properties": {
	///    "ImmeRep": {
	///      "type": "boolean"
	///    },
	///    "altNotifFqdns": {
	///      "description": "Alternate or backup FQDN(s) where to send
	/// Notifications.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Fqdn"
	///      },
	///      "minItems": 1
	///    },
	///    "altNotifIpv4Addrs": {
	///      "description": "Alternate or backup IPv4 address(es) where to send
	/// Notifications.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "altNotifIpv6Addrs": {
	///      "description": "Alternate or backup IPv6 address(es) where to send
	/// Notifications.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "anyUeInd": {
	///      "description": "Any UE indication. This IE shall be present if the
	/// event subscription is applicable to  any UE. Default value \"false\" is
	/// used, if not present.\n",
	///      "type": "boolean"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "eventNotifs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EventNotification"
	///      },
	///      "minItems": 1
	///    },
	///    "eventSubs": {
	///      "description": "Subscribed events",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EventSubscription"
	///      },
	///      "minItems": 1
	///    },
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "groupId": {
	///      "$ref": "#/components/schemas/GroupId"
	///    },
	///    "grpRepTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "maxReportNbr": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "notifFlag": {
	///      "$ref": "#/components/schemas/NotificationFlag"
	///    },
	///    "notifId": {
	///      "description": "Notification Correlation ID assigned by the NF
	/// service consumer.",
	///      "type": "string"
	///    },
	///    "notifMethod": {
	///      "$ref": "#/components/schemas/NotificationMethod"
	///    },
	///    "notifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "partitionCriteria": {
	///      "description": "Criteria for partitioning the UEs before applying
	/// the sampling ratio.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PartitioningCriteria"
	///      },
	///      "minItems": 1
	///    },
	///    "pduSeId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "repPeriod": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "sampRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "serviveName": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "subId": {
	///      "$ref": "#/components/schemas/SubId"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NsmfEventExposure {
		/// Alternate or backup FQDN(s) where to send Notifications.
		#[serde(
			rename = "altNotifFqdns",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_fqdns: Vec<Fqdn>,
		/// Alternate or backup IPv4 address(es) where to send Notifications.
		#[serde(
			rename = "altNotifIpv4Addrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_ipv4_addrs: Vec<Ipv4Addr>,
		/// Alternate or backup IPv6 address(es) where to send Notifications.
		#[serde(
			rename = "altNotifIpv6Addrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub alt_notif_ipv6_addrs: Vec<Ipv6Addr>,
		/// Any UE indication. This IE shall be present if the event
		/// subscription is applicable to  any UE. Default value "false" is
		/// used, if not present.
		#[serde(rename = "anyUeInd", default, skip_serializing_if = "Option::is_none")]
		pub any_ue_ind: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(rename = "eventNotifs", default, skip_serializing_if = "Vec::is_empty")]
		pub event_notifs: Vec<EventNotification>,
		/// Subscribed events
		#[serde(rename = "eventSubs")]
		pub event_subs: Vec<EventSubscription>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<GroupId>,
		#[serde(
			rename = "grpRepTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub grp_rep_time: Option<DurationSec>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<crate::common::common_models::Guami>,
		#[serde(rename = "ImmeRep", default, skip_serializing_if = "Option::is_none")]
		pub imme_rep: Option<bool>,
		#[serde(
			rename = "maxReportNbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_report_nbr: Option<Uinteger>,
		#[serde(rename = "notifFlag", default, skip_serializing_if = "Option::is_none")]
		pub notif_flag: Option<NotificationFlag>,
		/// Notification Correlation ID assigned by the NF service consumer.
		#[serde(rename = "notifId")]
		pub notif_id: String,
		#[serde(
			rename = "notifMethod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notif_method: Option<NotificationMethod>,
		#[serde(rename = "notifUri")]
		pub notif_uri: Uri,
		/// Criteria for partitioning the UEs before applying the sampling
		/// ratio.
		#[serde(
			rename = "partitionCriteria",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub partition_criteria: Vec<PartitioningCriteria>,
		#[serde(rename = "pduSeId", default, skip_serializing_if = "Option::is_none")]
		pub pdu_se_id: Option<PduSessionId>,
		#[serde(rename = "repPeriod", default, skip_serializing_if = "Option::is_none")]
		pub rep_period: Option<DurationSec>,
		#[serde(rename = "sampRatio", default, skip_serializing_if = "Option::is_none")]
		pub samp_ratio: Option<SamplingRatio>,
		#[serde(
			rename = "serviveName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub servive_name: Option<ServiceName>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(rename = "subId", default, skip_serializing_if = "Option::is_none")]
		pub sub_id: Option<SubId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&NsmfEventExposure> for NsmfEventExposure {
		fn from(value: &NsmfEventExposure) -> Self {
			value.clone()
		}
	}

	/// Represents notifications on events that occurred.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents notifications on events that occurred.",
	///  "type": "object",
	///  "required": [
	///    "eventNotifs",
	///    "notifId"
	///  ],
	///  "properties": {
	///    "ackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "eventNotifs": {
	///      "description": "Notifications about Individual Events",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EventNotification"
	///      },
	///      "minItems": 1
	///    },
	///    "notifId": {
	///      "description": "Notification correlation ID",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NsmfEventExposureNotification {
		#[serde(rename = "ackUri", default, skip_serializing_if = "Option::is_none")]
		pub ack_uri: Option<Uri>,
		/// Notifications about Individual Events
		#[serde(rename = "eventNotifs")]
		pub event_notifs: Vec<EventNotification>,
		/// Notification correlation ID
		#[serde(rename = "notifId")]
		pub notif_id: String,
	}

	impl From<&NsmfEventExposureNotification> for NsmfEventExposureNotification {
		fn from(value: &NsmfEventExposureNotification) -> Self {
			value.clone()
		}
	}

	/// JSON's null value.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "JSON's null value.",
	///  "type": "null"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NullValue(pub ());

	impl ::std::ops::Deref for NullValue {
		type Target = ();
		fn deref(&self) -> &() {
			&self.0
		}
	}

	impl From<NullValue> for () {
		fn from(value: NullValue) -> Self {
			value.0
		}
	}

	impl From<&NullValue> for NullValue {
		fn from(value: &NullValue) -> Self {
			value.clone()
		}
	}

	impl From<()> for NullValue {
		fn from(value: ()) -> Self {
			Self(value)
		}
	}

	/// Unsigned integer indicating Packet Delay Budget (see clauses 5.7.3.4 and
	/// 5.7.4 of 3GPP TS 23.501), expressed in milliseconds.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer indicating Packet Delay Budget (see
	/// clauses 5.7.3.4 and 5.7.4 of 3GPP TS 23.501), expressed in
	/// milliseconds.\n",
	///  "type": "integer",
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PacketDelBudget(
		#[default(_code = "unsafe {std::num::NonZeroU64::new_unchecked(1)}")]
		pub  std::num::NonZeroU64,
	);

	impl ::std::ops::Deref for PacketDelBudget {
		type Target = std::num::NonZeroU64;
		fn deref(&self) -> &std::num::NonZeroU64 {
			&self.0
		}
	}

	impl From<PacketDelBudget> for std::num::NonZeroU64 {
		fn from(value: PacketDelBudget) -> Self {
			value.0
		}
	}

	impl From<&PacketDelBudget> for PacketDelBudget {
		fn from(value: &PacketDelBudget) -> Self {
			value.clone()
		}
	}

	impl From<std::num::NonZeroU64> for PacketDelBudget {
		fn from(value: std::num::NonZeroU64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for PacketDelBudget {
		type Err = <std::num::NonZeroU64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for PacketDelBudget {
		type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PacketDelBudget {
		type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PacketDelBudget {
		type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for PacketDelBudget {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// String representing Packet Error Rate (see clause 5.7.3.5 and 5.7.4 of
	/// 3GPP TS 23.501, expressed as a "scalar x 10-k" where the scalar and the
	/// exponent k are each encoded as one decimal digit.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing Packet Error Rate (see clause
	/// 5.7.3.5 and 5.7.4 of 3GPP TS 23.501, expressed as a \"scalar x 10-k\"
	/// where the scalar and the exponent k are each encoded as one decimal
	/// digit.\n",
	///  "type": "string",
	///  "pattern": "^([0-9]E-[0-9])$"
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
	)]
	pub struct PacketErrRate(String);

	impl ::std::ops::Deref for PacketErrRate {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PacketErrRate> for String {
		fn from(value: PacketErrRate) -> Self {
			value.0
		}
	}

	impl From<&PacketErrRate> for PacketErrRate {
		fn from(value: &PacketErrRate) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PacketErrRate {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^([0-9]E-[0-9])$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^([0-9]E-[0-9])$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PacketErrRate {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PacketErrRate {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PacketErrRate {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PacketErrRate {
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

	/// Unsigned integer indicating Packet Loss Rate (see clauses 5.7.2.8 and
	/// 5.7.4 of 3GPP TS 23.501), expressed in tenth of percent.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer indicating Packet Loss Rate (see
	/// clauses 5.7.2.8 and 5.7.4 of 3GPP TS 23.501), expressed in tenth of
	/// percent.\n",
	///  "type": "integer",
	///  "maximum": 1000.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PacketLossRate(pub i64);

	impl ::std::ops::Deref for PacketLossRate {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<PacketLossRate> for i64 {
		fn from(value: PacketLossRate) -> Self {
			value.0
		}
	}

	impl From<&PacketLossRate> for PacketLossRate {
		fn from(value: &PacketLossRate) -> Self {
			value.clone()
		}
	}

	impl From<i64> for PacketLossRate {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for PacketLossRate {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for PacketLossRate {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PacketLossRate {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PacketLossRate {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for PacketLossRate {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// PartialRecordMethod
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "DEFAULT",
	///    "INDIVIDUAL"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum PartialRecordMethod {
		#[default]
		#[serde(rename = "DEFAULT")]
		Default,
		#[serde(rename = "INDIVIDUAL")]
		Individual,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PartialRecordMethod> for PartialRecordMethod {
		fn from(value: &PartialRecordMethod) -> Self {
			value.clone()
		}
	}

	impl ToString for PartialRecordMethod {
		fn to_string(&self) -> String {
			match *self {
				Self::Default => "DEFAULT".to_string(),
				Self::Individual => "INDIVIDUAL".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PartialRecordMethod {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DEFAULT" => Ok(Self::Default),
				"INDIVIDUAL" => Ok(Self::Individual),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PartialRecordMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PartialRecordMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PartialRecordMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - "TAC": Type Allocation Code
	/// - "SUBPLMN": Subscriber PLMN ID
	/// - "GEOAREA": Geographical area, i.e. list(s) of TAI(s)
	/// - "SNSSAI": S-NSSAI
	/// - "DNN": DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- \"TAC\": Type Allocation Code\n- \"SUBPLMN\": Subscriber PLMN ID\n- \"GEOAREA\": Geographical area, i.e. list(s) of TAI(s)\n- \"SNSSAI\": S-NSSAI\n- \"DNN\": DNN\n",
	///  "type": "string",
	///  "enum": [
	///    "TAC",
	///    "SUBPLMN",
	///    "GEOAREA",
	///    "SNSSAI",
	///    "DNN"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum PartitioningCriteria {
		#[default]
		#[serde(rename = "TAC")]
		Tac,
		#[serde(rename = "SUBPLMN")]
		Subplmn,
		#[serde(rename = "GEOAREA")]
		Geoarea,
		#[serde(rename = "SNSSAI")]
		Snssai,
		#[serde(rename = "DNN")]
		Dnn,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PartitioningCriteria> for PartitioningCriteria {
		fn from(value: &PartitioningCriteria) -> Self {
			value.clone()
		}
	}

	impl ToString for PartitioningCriteria {
		fn to_string(&self) -> String {
			match *self {
				Self::Tac => "TAC".to_string(),
				Self::Subplmn => "SUBPLMN".to_string(),
				Self::Geoarea => "GEOAREA".to_string(),
				Self::Snssai => "SNSSAI".to_string(),
				Self::Dnn => "DNN".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PartitioningCriteria {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TAC" => Ok(Self::Tac),
				"SUBPLMN" => Ok(Self::Subplmn),
				"GEOAREA" => Ok(Self::Geoarea),
				"SNSSAI" => Ok(Self::Snssai),
				"DNN" => Ok(Self::Dnn),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PartitioningCriteria {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PartitioningCriteria {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PartitioningCriteria {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the PCF for the UE information necessary for the PCF for the
	/// PDU session to send  SM Policy Association Establishment and Termination
	/// events.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the PCF for the UE information necessary for
	/// the PCF for the PDU session to send  SM Policy Association Establishment
	/// and Termination events.\n",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "callbackUri"
	///  ],
	///  "properties": {
	///    "bindingInfo": {
	///      "type": "string"
	///    },
	///    "callbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PcfUeCallbackInfo(pub Option<PcfUeCallbackInfoInner>);

	impl ::std::ops::Deref for PcfUeCallbackInfo {
		type Target = Option<PcfUeCallbackInfoInner>;
		fn deref(&self) -> &Option<PcfUeCallbackInfoInner> {
			&self.0
		}
	}

	impl From<PcfUeCallbackInfo> for Option<PcfUeCallbackInfoInner> {
		fn from(value: PcfUeCallbackInfo) -> Self {
			value.0
		}
	}

	impl From<&PcfUeCallbackInfo> for PcfUeCallbackInfo {
		fn from(value: &PcfUeCallbackInfo) -> Self {
			value.clone()
		}
	}

	impl From<Option<PcfUeCallbackInfoInner>> for PcfUeCallbackInfo {
		fn from(value: Option<PcfUeCallbackInfoInner>) -> Self {
			Self(value)
		}
	}

	/// Contains the PCF for the UE information necessary for the PCF for the
	/// PDU session to send  SM Policy Association Establishment and Termination
	/// events.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the PCF for the UE information necessary for
	/// the PCF for the PDU session to send  SM Policy Association Establishment
	/// and Termination events.\n",
	///  "type": "object",
	///  "required": [
	///    "callbackUri"
	///  ],
	///  "properties": {
	///    "bindingInfo": {
	///      "type": "string"
	///    },
	///    "callbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PcfUeCallbackInfoInner {
		#[serde(
			rename = "bindingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub binding_info: Option<String>,
		#[serde(rename = "callbackUri")]
		pub callback_uri: Uri,
	}

	impl From<&PcfUeCallbackInfoInner> for PcfUeCallbackInfoInner {
		fn from(value: &PcfUeCallbackInfoInner) -> Self {
			value.clone()
		}
	}

	/// Type of PDU Session information. Possible values are
	///  - AF_COORDINATION_INFO
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Type of PDU Session information. Possible values are\n
	/// - AF_COORDINATION_INFO\n",
	///  "type": "string",
	///  "enum": [
	///    "AF_COORDINATION_INFO"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum PduSessionContextType {
		#[default]
		#[serde(rename = "AF_COORDINATION_INFO")]
		AfCoordinationInfo,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PduSessionContextType> for PduSessionContextType {
		fn from(value: &PduSessionContextType) -> Self {
			value.clone()
		}
	}

	impl ToString for PduSessionContextType {
		fn to_string(&self) -> String {
			match *self {
				Self::AfCoordinationInfo => "AF_COORDINATION_INFO".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PduSessionContextType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AF_COORDINATION_INFO" => Ok(Self::AfCoordinationInfo),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PduSessionContextType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PduSessionContextType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PduSessionContextType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within Create Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Create Request",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "vsmfId",
	///        "vsmfPduSessionUri"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ismfId",
	///        "ismfPduSessionUri"
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "anType",
	///    "dnn",
	///    "servingNetwork"
	///  ],
	///  "properties": {
	///    "addUeLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "additionalAnType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "additionalCnTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "alwaysOnRequested": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "amfNfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "anType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "apnRateStatus": {
	///      "$ref": "#/components/schemas/ApnRateStatus"
	///    },
	///    "chargingId": {
	///      "type": "string",
	///      "pattern": "^(0|([1-9]{1}[0-9]{0,9}))$"
	///    },
	///    "cpCiotEnabled": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "cpOnlyInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "disasterRoamingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "dlServingPlmnRateCtl": {
	///      "type": "integer",
	///      "minimum": 10.0
	///    },
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "epsBearerCtxStatus": {
	///      "$ref": "#/components/schemas/EpsBearerContextStatus"
	///    },
	///    "epsBearerId": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerId"
	///      },
	///      "minItems": 1
	///    },
	///    "epsInterworkingInd": {
	///      "$ref": "#/components/schemas/EpsInterworkingIndication"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "hNwPubKeyId": {
	///      "type": "integer"
	///    },
	///    "hPcfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "hoPreparationIndication": {
	///      "type": "boolean"
	///    },
	///    "hplmnSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "iSmfServiceInstanceId": {
	///      "type": "string"
	///    },
	///    "icnTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "invokeNef": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ismfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "ismfPduSessionUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "maNwUpgradeInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "maRequestInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "maxIntegrityProtectedDataRateDl": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "maxIntegrityProtectedDataRateUl": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "n1SmInfoFromUe": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n9ForwardingTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "oldPduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "oldPduSessionRef": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "oldSmContextRef": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "pcfGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "pcfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "pcfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "pcfUeCallbackInfo": {
	///      "$ref": "#/components/schemas/PcfUeCallbackInfo"
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "pgwS8cFteid": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "presenceInLadn": {
	///      "$ref": "#/components/schemas/PresenceState"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "redundantPduSessionInfo": {
	///      "$ref": "#/components/schemas/RedundantPduSessionInformation"
	///    },
	///    "requestType": {
	///      "$ref": "#/components/schemas/RequestType"
	///    },
	///    "roamingChargingProfile": {
	///      "$ref": "#/components/schemas/RoamingChargingProfile"
	///    },
	///    "routingIndicator": {
	///      "type": "string"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "satelliteBackhaulCat": {
	///      "$ref": "#/components/schemas/SatelliteBackhaulCategory"
	///    },
	///    "secondaryRatUsageInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SecondaryRatUsageInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "selMode": {
	///      "$ref": "#/components/schemas/DnnSelectionMode"
	///    },
	///    "selectedDnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "smPolicyNotifyInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "smallDataRateStatus": {
	///      "$ref": "#/components/schemas/SmallDataRateStatus"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "udmGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "ueLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "unauthenticatedSupi": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "unknownN1SmInfo": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "upCnxState": {
	///      "$ref": "#/components/schemas/UpCnxState"
	///    },
	///    "upSecurityInfo": {
	///      "$ref": "#/components/schemas/UpSecurityInfo"
	///    },
	///    "upipSupported": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "vSmfServiceInstanceId": {
	///      "type": "string"
	///    },
	///    "vcnTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "vplmnQos": {
	///      "$ref": "#/components/schemas/VplmnQos"
	///    },
	///    "vsmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "vsmfPduSessionUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum PduSessionCreateData {
		#[default]
		Variant0 {
			#[serde(
				rename = "addUeLocation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			add_ue_location: Option<UserLocation>,
			#[serde(
				rename = "additionalAnType",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			additional_an_type: Option<AccessType>,
			#[serde(
				rename = "additionalCnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			additional_cn_tunnel_info: Option<TunnelInfo>,
			#[serde(rename = "alwaysOnRequested", default)]
			always_on_requested: bool,
			#[serde(rename = "amfNfId", default, skip_serializing_if = "Option::is_none")]
			amf_nf_id: Option<NfInstanceId>,
			#[serde(rename = "anType")]
			an_type: AccessType,
			#[serde(
				rename = "apnRateStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			apn_rate_status: Option<ApnRateStatus>,
			#[serde(
				rename = "chargingId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			charging_id: Option<PduSessionCreateDataVariant0ChargingId>,
			#[serde(rename = "cpCiotEnabled", default)]
			cp_ciot_enabled: bool,
			#[serde(rename = "cpOnlyInd", default)]
			cp_only_ind: bool,
			#[serde(rename = "disasterRoamingInd", default)]
			disaster_roaming_ind: bool,
			#[serde(
				rename = "dlServingPlmnRateCtl",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			dl_serving_plmn_rate_ctl: Option<i64>,
			#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
			dnai_list: Vec<Dnai>,
			dnn: Dnn,
			#[serde(
				rename = "epsBearerCtxStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			eps_bearer_ctx_status: Option<EpsBearerContextStatus>,
			#[serde(rename = "epsBearerId", default, skip_serializing_if = "Vec::is_empty")]
			eps_bearer_id: Vec<EpsBearerId>,
			#[serde(
				rename = "epsInterworkingInd",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			eps_interworking_ind: Option<EpsInterworkingIndication>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			guami: Option<crate::common::common_models::Guami>,
			#[serde(
				rename = "hNwPubKeyId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			h_nw_pub_key_id: Option<i64>,
			#[serde(rename = "hPcfId", default, skip_serializing_if = "Option::is_none")]
			h_pcf_id: Option<NfInstanceId>,
			#[serde(
				rename = "hoPreparationIndication",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ho_preparation_indication: Option<bool>,
			#[serde(
				rename = "hplmnSnssai",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			hplmn_snssai: Option<Snssai>,
			#[serde(
				rename = "iSmfServiceInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			i_smf_service_instance_id: Option<String>,
			#[serde(
				rename = "icnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			icn_tunnel_info: Option<TunnelInfo>,
			#[serde(rename = "invokeNef", default)]
			invoke_nef: bool,
			#[serde(rename = "maNwUpgradeInd", default)]
			ma_nw_upgrade_ind: bool,
			#[serde(rename = "maRequestInd", default)]
			ma_request_ind: bool,
			#[serde(
				rename = "maxIntegrityProtectedDataRateDl",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			max_integrity_protected_data_rate_dl: Option<MaxIntegrityProtectedDataRate>,
			#[serde(
				rename = "maxIntegrityProtectedDataRateUl",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			max_integrity_protected_data_rate_ul: Option<MaxIntegrityProtectedDataRate>,
			#[serde(
				rename = "n1SmInfoFromUe",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			n1_sm_info_from_ue: Option<RefToBinaryData>,
			#[serde(
				rename = "n9ForwardingTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			n9_forwarding_tunnel_info: Option<TunnelInfo>,
			#[serde(
				rename = "oldPduSessionId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			old_pdu_session_id: Option<PduSessionId>,
			#[serde(
				rename = "oldPduSessionRef",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			old_pdu_session_ref: Option<Uri>,
			#[serde(
				rename = "oldSmContextRef",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			old_sm_context_ref: Option<Uri>,
			#[serde(
				rename = "pcfGroupId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			pcf_group_id: Option<NfGroupId>,
			#[serde(rename = "pcfId", default, skip_serializing_if = "Option::is_none")]
			pcf_id: Option<NfInstanceId>,
			#[serde(rename = "pcfSetId", default, skip_serializing_if = "Option::is_none")]
			pcf_set_id: Option<NfSetId>,
			#[serde(
				rename = "pcfUeCallbackInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			pcf_ue_callback_info: Option<PcfUeCallbackInfo>,
			#[serde(
				rename = "pduSessionId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			pdu_session_id: Option<PduSessionId>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			pei: Option<Pei>,
			#[serde(
				rename = "pgwS8cFteid",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			pgw_s8c_fteid: Option<Bytes>,
			#[serde(
				rename = "presenceInLadn",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			presence_in_ladn: Option<PresenceState>,
			#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
			rat_type: Option<RatType>,
			#[serde(
				rename = "recoveryTime",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			recovery_time: Option<DateTime>,
			#[serde(
				rename = "redundantPduSessionInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			redundant_pdu_session_info: Option<RedundantPduSessionInformation>,
			#[serde(
				rename = "requestType",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			request_type: Option<RequestType>,
			#[serde(
				rename = "roamingChargingProfile",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			roaming_charging_profile: Option<RoamingChargingProfile>,
			#[serde(
				rename = "routingIndicator",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			routing_indicator: Option<String>,
			#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
			s_nssai: Option<Snssai>,
			#[serde(
				rename = "satelliteBackhaulCat",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			satellite_backhaul_cat: Option<SatelliteBackhaulCategory>,
			#[serde(
				rename = "secondaryRatUsageInfo",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			secondary_rat_usage_info: Vec<SecondaryRatUsageInfo>,
			#[serde(rename = "selMode", default, skip_serializing_if = "Option::is_none")]
			sel_mode: Option<DnnSelectionMode>,
			#[serde(
				rename = "selectedDnn",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			selected_dnn: Option<Dnn>,
			#[serde(rename = "servingNetwork")]
			serving_network: PlmnIdNid,
			#[serde(rename = "smPolicyNotifyInd", default)]
			sm_policy_notify_ind: bool,
			#[serde(
				rename = "smallDataRateStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			small_data_rate_status: Option<SmallDataRateStatus>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			supi: Option<Supi>,
			#[serde(
				rename = "supportedFeatures",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			supported_features: Option<SupportedFeatures>,
			#[serde(
				rename = "udmGroupId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			udm_group_id: Option<NfGroupId>,
			#[serde(
				rename = "ueLocation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_location: Option<UserLocation>,
			#[serde(
				rename = "ueTimeZone",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_time_zone: Option<TimeZone>,
			#[serde(rename = "unauthenticatedSupi", default)]
			unauthenticated_supi: bool,
			#[serde(
				rename = "unknownN1SmInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			unknown_n1_sm_info: Option<RefToBinaryData>,
			#[serde(
				rename = "upCnxState",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			up_cnx_state: Option<UpCnxState>,
			#[serde(
				rename = "upSecurityInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			up_security_info: Option<UpSecurityInfo>,
			#[serde(rename = "upipSupported", default)]
			upip_supported: bool,
			#[serde(
				rename = "vSmfServiceInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			v_smf_service_instance_id: Option<String>,
			#[serde(
				rename = "vcnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			vcn_tunnel_info: Option<TunnelInfo>,
			#[serde(rename = "vplmnQos", default, skip_serializing_if = "Option::is_none")]
			vplmn_qos: Option<VplmnQos>,
			#[serde(rename = "vsmfId")]
			vsmf_id: NfInstanceId,
			#[serde(rename = "vsmfPduSessionUri")]
			vsmf_pdu_session_uri: Uri,
		},
		Variant1 {
			#[serde(
				rename = "addUeLocation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			add_ue_location: Option<UserLocation>,
			#[serde(
				rename = "additionalAnType",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			additional_an_type: Option<AccessType>,
			#[serde(
				rename = "additionalCnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			additional_cn_tunnel_info: Option<TunnelInfo>,
			#[serde(rename = "alwaysOnRequested", default)]
			always_on_requested: bool,
			#[serde(rename = "amfNfId", default, skip_serializing_if = "Option::is_none")]
			amf_nf_id: Option<NfInstanceId>,
			#[serde(rename = "anType")]
			an_type: AccessType,
			#[serde(
				rename = "apnRateStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			apn_rate_status: Option<ApnRateStatus>,
			#[serde(
				rename = "chargingId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			charging_id: Option<PduSessionCreateDataVariant1ChargingId>,
			#[serde(rename = "cpCiotEnabled", default)]
			cp_ciot_enabled: bool,
			#[serde(rename = "cpOnlyInd", default)]
			cp_only_ind: bool,
			#[serde(rename = "disasterRoamingInd", default)]
			disaster_roaming_ind: bool,
			#[serde(
				rename = "dlServingPlmnRateCtl",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			dl_serving_plmn_rate_ctl: Option<i64>,
			#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
			dnai_list: Vec<Dnai>,
			dnn: Dnn,
			#[serde(
				rename = "epsBearerCtxStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			eps_bearer_ctx_status: Option<EpsBearerContextStatus>,
			#[serde(rename = "epsBearerId", default, skip_serializing_if = "Vec::is_empty")]
			eps_bearer_id: Vec<EpsBearerId>,
			#[serde(
				rename = "epsInterworkingInd",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			eps_interworking_ind: Option<EpsInterworkingIndication>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			guami: Option<crate::common::common_models::Guami>,
			#[serde(
				rename = "hNwPubKeyId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			h_nw_pub_key_id: Option<i64>,
			#[serde(rename = "hPcfId", default, skip_serializing_if = "Option::is_none")]
			h_pcf_id: Option<NfInstanceId>,
			#[serde(
				rename = "hoPreparationIndication",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ho_preparation_indication: Option<bool>,
			#[serde(
				rename = "hplmnSnssai",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			hplmn_snssai: Option<Snssai>,
			#[serde(
				rename = "iSmfServiceInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			i_smf_service_instance_id: Option<String>,
			#[serde(
				rename = "icnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			icn_tunnel_info: Option<TunnelInfo>,
			#[serde(rename = "invokeNef", default)]
			invoke_nef: bool,
			#[serde(rename = "ismfId")]
			ismf_id: NfInstanceId,
			#[serde(rename = "ismfPduSessionUri")]
			ismf_pdu_session_uri: Uri,
			#[serde(rename = "maNwUpgradeInd", default)]
			ma_nw_upgrade_ind: bool,
			#[serde(rename = "maRequestInd", default)]
			ma_request_ind: bool,
			#[serde(
				rename = "maxIntegrityProtectedDataRateDl",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			max_integrity_protected_data_rate_dl: Option<MaxIntegrityProtectedDataRate>,
			#[serde(
				rename = "maxIntegrityProtectedDataRateUl",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			max_integrity_protected_data_rate_ul: Option<MaxIntegrityProtectedDataRate>,
			#[serde(
				rename = "n1SmInfoFromUe",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			n1_sm_info_from_ue: Option<RefToBinaryData>,
			#[serde(
				rename = "n9ForwardingTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			n9_forwarding_tunnel_info: Option<TunnelInfo>,
			#[serde(
				rename = "oldPduSessionId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			old_pdu_session_id: Option<PduSessionId>,
			#[serde(
				rename = "oldPduSessionRef",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			old_pdu_session_ref: Option<Uri>,
			#[serde(
				rename = "oldSmContextRef",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			old_sm_context_ref: Option<Uri>,
			#[serde(
				rename = "pcfGroupId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			pcf_group_id: Option<NfGroupId>,
			#[serde(rename = "pcfId", default, skip_serializing_if = "Option::is_none")]
			pcf_id: Option<NfInstanceId>,
			#[serde(rename = "pcfSetId", default, skip_serializing_if = "Option::is_none")]
			pcf_set_id: Option<NfSetId>,
			#[serde(
				rename = "pcfUeCallbackInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			pcf_ue_callback_info: Option<PcfUeCallbackInfo>,
			#[serde(
				rename = "pduSessionId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			pdu_session_id: Option<PduSessionId>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			pei: Option<Pei>,
			#[serde(
				rename = "pgwS8cFteid",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			pgw_s8c_fteid: Option<Bytes>,
			#[serde(
				rename = "presenceInLadn",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			presence_in_ladn: Option<PresenceState>,
			#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
			rat_type: Option<RatType>,
			#[serde(
				rename = "recoveryTime",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			recovery_time: Option<DateTime>,
			#[serde(
				rename = "redundantPduSessionInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			redundant_pdu_session_info: Option<RedundantPduSessionInformation>,
			#[serde(
				rename = "requestType",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			request_type: Option<RequestType>,
			#[serde(
				rename = "roamingChargingProfile",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			roaming_charging_profile: Option<RoamingChargingProfile>,
			#[serde(
				rename = "routingIndicator",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			routing_indicator: Option<String>,
			#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
			s_nssai: Option<Snssai>,
			#[serde(
				rename = "satelliteBackhaulCat",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			satellite_backhaul_cat: Option<SatelliteBackhaulCategory>,
			#[serde(
				rename = "secondaryRatUsageInfo",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			secondary_rat_usage_info: Vec<SecondaryRatUsageInfo>,
			#[serde(rename = "selMode", default, skip_serializing_if = "Option::is_none")]
			sel_mode: Option<DnnSelectionMode>,
			#[serde(
				rename = "selectedDnn",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			selected_dnn: Option<Dnn>,
			#[serde(rename = "servingNetwork")]
			serving_network: PlmnIdNid,
			#[serde(rename = "smPolicyNotifyInd", default)]
			sm_policy_notify_ind: bool,
			#[serde(
				rename = "smallDataRateStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			small_data_rate_status: Option<SmallDataRateStatus>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			supi: Option<Supi>,
			#[serde(
				rename = "supportedFeatures",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			supported_features: Option<SupportedFeatures>,
			#[serde(
				rename = "udmGroupId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			udm_group_id: Option<NfGroupId>,
			#[serde(
				rename = "ueLocation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_location: Option<UserLocation>,
			#[serde(
				rename = "ueTimeZone",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_time_zone: Option<TimeZone>,
			#[serde(rename = "unauthenticatedSupi", default)]
			unauthenticated_supi: bool,
			#[serde(
				rename = "unknownN1SmInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			unknown_n1_sm_info: Option<RefToBinaryData>,
			#[serde(
				rename = "upCnxState",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			up_cnx_state: Option<UpCnxState>,
			#[serde(
				rename = "upSecurityInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			up_security_info: Option<UpSecurityInfo>,
			#[serde(rename = "upipSupported", default)]
			upip_supported: bool,
			#[serde(
				rename = "vSmfServiceInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			v_smf_service_instance_id: Option<String>,
			#[serde(
				rename = "vcnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			vcn_tunnel_info: Option<TunnelInfo>,
			#[serde(rename = "vplmnQos", default, skip_serializing_if = "Option::is_none")]
			vplmn_qos: Option<VplmnQos>,
		},
	}

	impl From<&PduSessionCreateData> for PduSessionCreateData {
		fn from(value: &PduSessionCreateData) -> Self {
			value.clone()
		}
	}

	/// PduSessionCreateDataVariant0ChargingId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^(0|([1-9]{1}[0-9]{0,9}))$"
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
	)]
	pub struct PduSessionCreateDataVariant0ChargingId(String);

	impl ::std::ops::Deref for PduSessionCreateDataVariant0ChargingId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PduSessionCreateDataVariant0ChargingId> for String {
		fn from(value: PduSessionCreateDataVariant0ChargingId) -> Self {
			value.0
		}
	}

	impl From<&PduSessionCreateDataVariant0ChargingId> for PduSessionCreateDataVariant0ChargingId {
		fn from(value: &PduSessionCreateDataVariant0ChargingId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PduSessionCreateDataVariant0ChargingId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^(0|([1-9]{1}[0-9]{0,9}))$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^(0|([1-9]{1}[0-9]{0,9}))$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PduSessionCreateDataVariant0ChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PduSessionCreateDataVariant0ChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PduSessionCreateDataVariant0ChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PduSessionCreateDataVariant0ChargingId {
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

	/// PduSessionCreateDataVariant1ChargingId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^(0|([1-9]{1}[0-9]{0,9}))$"
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
	)]
	pub struct PduSessionCreateDataVariant1ChargingId(String);

	impl ::std::ops::Deref for PduSessionCreateDataVariant1ChargingId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PduSessionCreateDataVariant1ChargingId> for String {
		fn from(value: PduSessionCreateDataVariant1ChargingId) -> Self {
			value.0
		}
	}

	impl From<&PduSessionCreateDataVariant1ChargingId> for PduSessionCreateDataVariant1ChargingId {
		fn from(value: &PduSessionCreateDataVariant1ChargingId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PduSessionCreateDataVariant1ChargingId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^(0|([1-9]{1}[0-9]{0,9}))$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^(0|([1-9]{1}[0-9]{0,9}))$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PduSessionCreateDataVariant1ChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PduSessionCreateDataVariant1ChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PduSessionCreateDataVariant1ChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PduSessionCreateDataVariant1ChargingId {
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

	/// Error within Create Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Error within Create Response",
	///  "type": "object",
	///  "required": [
	///    "error"
	///  ],
	///  "properties": {
	///    "backOffTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "error": {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    "n1SmInfoToUe": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n1smCause": {
	///      "type": "string",
	///      "pattern": "^[A-F0-9]{2}$"
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionCreateError {
		#[serde(
			rename = "backOffTimer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub back_off_timer: Option<DurationSec>,
		pub error: ProblemDetails,
		#[serde(
			rename = "n1SmInfoToUe",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_sm_info_to_ue: Option<RefToBinaryData>,
		#[serde(rename = "n1smCause", default, skip_serializing_if = "Option::is_none")]
		pub n1sm_cause: Option<PduSessionCreateErrorN1smCause>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
	}

	impl From<&PduSessionCreateError> for PduSessionCreateError {
		fn from(value: &PduSessionCreateError) -> Self {
			value.clone()
		}
	}

	/// PduSessionCreateErrorN1smCause
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-F0-9]{2}$"
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
	)]
	pub struct PduSessionCreateErrorN1smCause(String);

	impl ::std::ops::Deref for PduSessionCreateErrorN1smCause {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PduSessionCreateErrorN1smCause> for String {
		fn from(value: PduSessionCreateErrorN1smCause) -> Self {
			value.0
		}
	}

	impl From<&PduSessionCreateErrorN1smCause> for PduSessionCreateErrorN1smCause {
		fn from(value: &PduSessionCreateErrorN1smCause) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PduSessionCreateErrorN1smCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-F0-9]{2}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-F0-9]{2}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PduSessionCreateErrorN1smCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PduSessionCreateErrorN1smCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PduSessionCreateErrorN1smCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PduSessionCreateErrorN1smCause {
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

	/// Data within Create Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Create Response",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "hSmfInstanceId"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "smfInstanceId"
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "pduSessionType",
	///    "sscMode"
	///  ],
	///  "properties": {
	///    "additionalCnTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "additionalSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "alwaysOnGranted": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "cnTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "dnAaaAddress": {
	///      "$ref": "#/components/schemas/IpAddress"
	///    },
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "enablePauseCharging": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "epsBearerInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "epsPdnCnxInfo": {
	///      "$ref": "#/components/schemas/EpsPdnCnxInfo"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "hSmfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "hSmfServiceInstanceId": {
	///      "type": "string"
	///    },
	///    "hcnTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "homeProvidedChargingId": {
	///      "type": "string",
	///      "pattern": "^(0|([1-9]{1}[0-9]{0,9}))$"
	///    },
	///    "interPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "intraPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "ipv6Index": {
	///      "$ref": "#/components/schemas/IpIndex"
	///    },
	///    "ipv6MultiHomingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "maAcceptedInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "maxIntegrityProtectedDataRate": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "maxIntegrityProtectedDataRateDl": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "n1SmInfoToUe": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "nefExtBufSupportInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "nspuSupportInd": {
	///      "type": "boolean"
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "pduSessionType": {
	///      "$ref": "#/components/schemas/PduSessionType"
	///    },
	///    "qosFlowsSetupList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowSetupItem"
	///      },
	///      "minItems": 1
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "redundantPduSessionInfo": {
	///      "$ref": "#/components/schemas/RedundantPduSessionInformation"
	///    },
	///    "roamingChargingProfile": {
	///      "$ref": "#/components/schemas/RoamingChargingProfile"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "sessionAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "smallDataRateControlEnabled": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "smfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smfServiceInstanceId": {
	///      "type": "string"
	///    },
	///    "sscMode": {
	///      "type": "string",
	///      "pattern": "^[0-7]$"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "ueIpv4Address": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ueIpv6InterfaceId": {
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{16}$"
	///    },
	///    "ueIpv6Prefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "upSecurity": {
	///      "$ref": "#/components/schemas/UpSecurity"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum PduSessionCreatedData {
		#[default]
		Variant0 {
			#[serde(
				rename = "additionalCnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			additional_cn_tunnel_info: Option<TunnelInfo>,
			#[serde(
				rename = "additionalSnssai",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			additional_snssai: Option<Snssai>,
			#[serde(rename = "alwaysOnGranted", default)]
			always_on_granted: bool,
			#[serde(
				rename = "cnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			cn_tunnel_info: Option<TunnelInfo>,
			#[serde(
				rename = "dnAaaAddress",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			dn_aaa_address: Option<IpAddress>,
			#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
			dnai_list: Vec<Dnai>,
			#[serde(rename = "enablePauseCharging", default)]
			enable_pause_charging: bool,
			#[serde(
				rename = "epsBearerInfo",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			eps_bearer_info: Vec<EpsBearerInfo>,
			#[serde(
				rename = "epsPdnCnxInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			eps_pdn_cnx_info: Option<EpsPdnCnxInfo>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			#[serde(rename = "hSmfInstanceId")]
			h_smf_instance_id: NfInstanceId,
			#[serde(
				rename = "hSmfServiceInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			h_smf_service_instance_id: Option<String>,
			#[serde(
				rename = "hcnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			hcn_tunnel_info: Option<TunnelInfo>,
			#[serde(
				rename = "homeProvidedChargingId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			home_provided_charging_id: Option<PduSessionCreatedDataVariant0HomeProvidedChargingId>,
			#[serde(
				rename = "interPlmnApiRoot",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			inter_plmn_api_root: Option<Uri>,
			#[serde(
				rename = "intraPlmnApiRoot",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			intra_plmn_api_root: Option<Uri>,
			#[serde(rename = "ipv6Index", default, skip_serializing_if = "Option::is_none")]
			ipv6_index: Option<IpIndex>,
			#[serde(rename = "ipv6MultiHomingInd", default)]
			ipv6_multi_homing_ind: bool,
			#[serde(rename = "maAcceptedInd", default)]
			ma_accepted_ind: bool,
			#[serde(
				rename = "maxIntegrityProtectedDataRate",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			max_integrity_protected_data_rate: Option<MaxIntegrityProtectedDataRate>,
			#[serde(
				rename = "maxIntegrityProtectedDataRateDl",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			max_integrity_protected_data_rate_dl: Option<MaxIntegrityProtectedDataRate>,
			#[serde(
				rename = "n1SmInfoToUe",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			n1_sm_info_to_ue: Option<RefToBinaryData>,
			#[serde(rename = "nefExtBufSupportInd", default)]
			nef_ext_buf_support_ind: bool,
			#[serde(
				rename = "nspuSupportInd",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nspu_support_ind: Option<bool>,
			#[serde(
				rename = "pduSessionId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			pdu_session_id: Option<PduSessionId>,
			#[serde(rename = "pduSessionType")]
			pdu_session_type: PduSessionType,
			#[serde(
				rename = "qosFlowsSetupList",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			qos_flows_setup_list: Vec<QosFlowSetupItem>,
			#[serde(
				rename = "recoveryTime",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			recovery_time: Option<DateTime>,
			#[serde(
				rename = "redundantPduSessionInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			redundant_pdu_session_info: Option<RedundantPduSessionInformation>,
			#[serde(
				rename = "roamingChargingProfile",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			roaming_charging_profile: Option<RoamingChargingProfile>,
			#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
			s_nssai: Option<Snssai>,
			#[serde(
				rename = "sessionAmbr",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			session_ambr: Option<Ambr>,
			#[serde(rename = "smallDataRateControlEnabled", default)]
			small_data_rate_control_enabled: bool,
			#[serde(
				rename = "smfServiceInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			smf_service_instance_id: Option<String>,
			#[serde(rename = "sscMode")]
			ssc_mode: PduSessionCreatedDataVariant0SscMode,
			#[serde(
				rename = "supportedFeatures",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			supported_features: Option<SupportedFeatures>,
			#[serde(
				rename = "ueIpv4Address",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_ipv4_address: Option<Ipv4Addr>,
			#[serde(
				rename = "ueIpv6InterfaceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_ipv6_interface_id: Option<PduSessionCreatedDataVariant0UeIpv6InterfaceId>,
			#[serde(
				rename = "ueIpv6Prefix",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_ipv6_prefix: Option<Ipv6Prefix>,
			#[serde(
				rename = "upSecurity",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			up_security: Option<UpSecurity>,
		},
		Variant1 {
			#[serde(
				rename = "additionalCnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			additional_cn_tunnel_info: Option<TunnelInfo>,
			#[serde(
				rename = "additionalSnssai",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			additional_snssai: Option<Snssai>,
			#[serde(rename = "alwaysOnGranted", default)]
			always_on_granted: bool,
			#[serde(
				rename = "cnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			cn_tunnel_info: Option<TunnelInfo>,
			#[serde(
				rename = "dnAaaAddress",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			dn_aaa_address: Option<IpAddress>,
			#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
			dnai_list: Vec<Dnai>,
			#[serde(rename = "enablePauseCharging", default)]
			enable_pause_charging: bool,
			#[serde(
				rename = "epsBearerInfo",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			eps_bearer_info: Vec<EpsBearerInfo>,
			#[serde(
				rename = "epsPdnCnxInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			eps_pdn_cnx_info: Option<EpsPdnCnxInfo>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			gpsi: Option<Gpsi>,
			#[serde(
				rename = "hSmfServiceInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			h_smf_service_instance_id: Option<String>,
			#[serde(
				rename = "hcnTunnelInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			hcn_tunnel_info: Option<TunnelInfo>,
			#[serde(
				rename = "homeProvidedChargingId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			home_provided_charging_id: Option<PduSessionCreatedDataVariant1HomeProvidedChargingId>,
			#[serde(
				rename = "interPlmnApiRoot",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			inter_plmn_api_root: Option<Uri>,
			#[serde(
				rename = "intraPlmnApiRoot",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			intra_plmn_api_root: Option<Uri>,
			#[serde(rename = "ipv6Index", default, skip_serializing_if = "Option::is_none")]
			ipv6_index: Option<IpIndex>,
			#[serde(rename = "ipv6MultiHomingInd", default)]
			ipv6_multi_homing_ind: bool,
			#[serde(rename = "maAcceptedInd", default)]
			ma_accepted_ind: bool,
			#[serde(
				rename = "maxIntegrityProtectedDataRate",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			max_integrity_protected_data_rate: Option<MaxIntegrityProtectedDataRate>,
			#[serde(
				rename = "maxIntegrityProtectedDataRateDl",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			max_integrity_protected_data_rate_dl: Option<MaxIntegrityProtectedDataRate>,
			#[serde(
				rename = "n1SmInfoToUe",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			n1_sm_info_to_ue: Option<RefToBinaryData>,
			#[serde(rename = "nefExtBufSupportInd", default)]
			nef_ext_buf_support_ind: bool,
			#[serde(
				rename = "nspuSupportInd",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nspu_support_ind: Option<bool>,
			#[serde(
				rename = "pduSessionId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			pdu_session_id: Option<PduSessionId>,
			#[serde(rename = "pduSessionType")]
			pdu_session_type: PduSessionType,
			#[serde(
				rename = "qosFlowsSetupList",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			qos_flows_setup_list: Vec<QosFlowSetupItem>,
			#[serde(
				rename = "recoveryTime",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			recovery_time: Option<DateTime>,
			#[serde(
				rename = "redundantPduSessionInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			redundant_pdu_session_info: Option<RedundantPduSessionInformation>,
			#[serde(
				rename = "roamingChargingProfile",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			roaming_charging_profile: Option<RoamingChargingProfile>,
			#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
			s_nssai: Option<Snssai>,
			#[serde(
				rename = "sessionAmbr",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			session_ambr: Option<Ambr>,
			#[serde(rename = "smallDataRateControlEnabled", default)]
			small_data_rate_control_enabled: bool,
			#[serde(rename = "smfInstanceId")]
			smf_instance_id: NfInstanceId,
			#[serde(
				rename = "smfServiceInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			smf_service_instance_id: Option<String>,
			#[serde(rename = "sscMode")]
			ssc_mode: PduSessionCreatedDataVariant1SscMode,
			#[serde(
				rename = "supportedFeatures",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			supported_features: Option<SupportedFeatures>,
			#[serde(
				rename = "ueIpv4Address",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_ipv4_address: Option<Ipv4Addr>,
			#[serde(
				rename = "ueIpv6InterfaceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_ipv6_interface_id: Option<PduSessionCreatedDataVariant1UeIpv6InterfaceId>,
			#[serde(
				rename = "ueIpv6Prefix",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_ipv6_prefix: Option<Ipv6Prefix>,
			#[serde(
				rename = "upSecurity",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			up_security: Option<UpSecurity>,
		},
	}

	impl From<&PduSessionCreatedData> for PduSessionCreatedData {
		fn from(value: &PduSessionCreatedData) -> Self {
			value.clone()
		}
	}

	/// PduSessionCreatedDataVariant0HomeProvidedChargingId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^(0|([1-9]{1}[0-9]{0,9}))$"
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
	)]
	pub struct PduSessionCreatedDataVariant0HomeProvidedChargingId(String);

	impl ::std::ops::Deref for PduSessionCreatedDataVariant0HomeProvidedChargingId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PduSessionCreatedDataVariant0HomeProvidedChargingId> for String {
		fn from(value: PduSessionCreatedDataVariant0HomeProvidedChargingId) -> Self {
			value.0
		}
	}

	impl From<&PduSessionCreatedDataVariant0HomeProvidedChargingId>
		for PduSessionCreatedDataVariant0HomeProvidedChargingId
	{
		fn from(value: &PduSessionCreatedDataVariant0HomeProvidedChargingId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PduSessionCreatedDataVariant0HomeProvidedChargingId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^(0|([1-9]{1}[0-9]{0,9}))$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^(0|([1-9]{1}[0-9]{0,9}))$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PduSessionCreatedDataVariant0HomeProvidedChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PduSessionCreatedDataVariant0HomeProvidedChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PduSessionCreatedDataVariant0HomeProvidedChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PduSessionCreatedDataVariant0HomeProvidedChargingId {
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

	/// PduSessionCreatedDataVariant0SscMode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-7]$"
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
	)]
	pub struct PduSessionCreatedDataVariant0SscMode(String);

	impl ::std::ops::Deref for PduSessionCreatedDataVariant0SscMode {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PduSessionCreatedDataVariant0SscMode> for String {
		fn from(value: PduSessionCreatedDataVariant0SscMode) -> Self {
			value.0
		}
	}

	impl From<&PduSessionCreatedDataVariant0SscMode> for PduSessionCreatedDataVariant0SscMode {
		fn from(value: &PduSessionCreatedDataVariant0SscMode) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PduSessionCreatedDataVariant0SscMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-7]$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-7]$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PduSessionCreatedDataVariant0SscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PduSessionCreatedDataVariant0SscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PduSessionCreatedDataVariant0SscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PduSessionCreatedDataVariant0SscMode {
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

	/// PduSessionCreatedDataVariant0UeIpv6InterfaceId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{16}$"
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
	)]
	pub struct PduSessionCreatedDataVariant0UeIpv6InterfaceId(String);

	impl ::std::ops::Deref for PduSessionCreatedDataVariant0UeIpv6InterfaceId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PduSessionCreatedDataVariant0UeIpv6InterfaceId> for String {
		fn from(value: PduSessionCreatedDataVariant0UeIpv6InterfaceId) -> Self {
			value.0
		}
	}

	impl From<&PduSessionCreatedDataVariant0UeIpv6InterfaceId>
		for PduSessionCreatedDataVariant0UeIpv6InterfaceId
	{
		fn from(value: &PduSessionCreatedDataVariant0UeIpv6InterfaceId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PduSessionCreatedDataVariant0UeIpv6InterfaceId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{16}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{16}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PduSessionCreatedDataVariant0UeIpv6InterfaceId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PduSessionCreatedDataVariant0UeIpv6InterfaceId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PduSessionCreatedDataVariant0UeIpv6InterfaceId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PduSessionCreatedDataVariant0UeIpv6InterfaceId {
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

	/// PduSessionCreatedDataVariant1HomeProvidedChargingId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^(0|([1-9]{1}[0-9]{0,9}))$"
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
	)]
	pub struct PduSessionCreatedDataVariant1HomeProvidedChargingId(String);

	impl ::std::ops::Deref for PduSessionCreatedDataVariant1HomeProvidedChargingId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PduSessionCreatedDataVariant1HomeProvidedChargingId> for String {
		fn from(value: PduSessionCreatedDataVariant1HomeProvidedChargingId) -> Self {
			value.0
		}
	}

	impl From<&PduSessionCreatedDataVariant1HomeProvidedChargingId>
		for PduSessionCreatedDataVariant1HomeProvidedChargingId
	{
		fn from(value: &PduSessionCreatedDataVariant1HomeProvidedChargingId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PduSessionCreatedDataVariant1HomeProvidedChargingId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^(0|([1-9]{1}[0-9]{0,9}))$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^(0|([1-9]{1}[0-9]{0,9}))$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PduSessionCreatedDataVariant1HomeProvidedChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PduSessionCreatedDataVariant1HomeProvidedChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PduSessionCreatedDataVariant1HomeProvidedChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PduSessionCreatedDataVariant1HomeProvidedChargingId {
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

	/// PduSessionCreatedDataVariant1SscMode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-7]$"
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
	)]
	pub struct PduSessionCreatedDataVariant1SscMode(String);

	impl ::std::ops::Deref for PduSessionCreatedDataVariant1SscMode {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PduSessionCreatedDataVariant1SscMode> for String {
		fn from(value: PduSessionCreatedDataVariant1SscMode) -> Self {
			value.0
		}
	}

	impl From<&PduSessionCreatedDataVariant1SscMode> for PduSessionCreatedDataVariant1SscMode {
		fn from(value: &PduSessionCreatedDataVariant1SscMode) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PduSessionCreatedDataVariant1SscMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-7]$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-7]$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PduSessionCreatedDataVariant1SscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PduSessionCreatedDataVariant1SscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PduSessionCreatedDataVariant1SscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PduSessionCreatedDataVariant1SscMode {
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

	/// PduSessionCreatedDataVariant1UeIpv6InterfaceId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{16}$"
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
	)]
	pub struct PduSessionCreatedDataVariant1UeIpv6InterfaceId(String);

	impl ::std::ops::Deref for PduSessionCreatedDataVariant1UeIpv6InterfaceId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PduSessionCreatedDataVariant1UeIpv6InterfaceId> for String {
		fn from(value: PduSessionCreatedDataVariant1UeIpv6InterfaceId) -> Self {
			value.0
		}
	}

	impl From<&PduSessionCreatedDataVariant1UeIpv6InterfaceId>
		for PduSessionCreatedDataVariant1UeIpv6InterfaceId
	{
		fn from(value: &PduSessionCreatedDataVariant1UeIpv6InterfaceId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PduSessionCreatedDataVariant1UeIpv6InterfaceId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{16}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{16}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PduSessionCreatedDataVariant1UeIpv6InterfaceId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PduSessionCreatedDataVariant1UeIpv6InterfaceId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PduSessionCreatedDataVariant1UeIpv6InterfaceId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PduSessionCreatedDataVariant1UeIpv6InterfaceId {
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

	/// Unsigned integer identifying a PDU session, within the range 0 to 255,
	/// as specified in  clause 11.2.3.1b, bits 1 to 8, of 3GPP TS 24.007. If
	/// the PDU Session ID is allocated by the  Core Network for UEs not
	/// supporting N1 mode, reserved range 64 to 95 is used. PDU Session ID
	/// within the reserved range is only visible in the Core Network.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer identifying a PDU session, within the range 0 to 255, as specified in  clause 11.2.3.1b, bits 1 to 8, of 3GPP TS 24.007. If the PDU Session ID is allocated by the  Core Network for UEs not supporting N1 mode, reserved range 64 to 95 is used. PDU Session ID  within the reserved range is only visible in the Core Network. \n",
	///  "type": "integer",
	///  "maximum": 255.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionId(pub u8);

	impl ::std::ops::Deref for PduSessionId {
		type Target = u8;
		fn deref(&self) -> &u8 {
			&self.0
		}
	}

	impl From<PduSessionId> for u8 {
		fn from(value: PduSessionId) -> Self {
			value.0
		}
	}

	impl From<&PduSessionId> for PduSessionId {
		fn from(value: &PduSessionId) -> Self {
			value.clone()
		}
	}

	impl From<u8> for PduSessionId {
		fn from(value: u8) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for PduSessionId {
		type Err = <u8 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for PduSessionId {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PduSessionId {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PduSessionId {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for PduSessionId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Represents session information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents session information.",
	///  "type": "object",
	///  "properties": {
	///    "n4SessId": {
	///      "description": "The identifier of the N4 session for the reported
	/// PDU Session.",
	///      "type": "string"
	///    },
	///    "pduSessStatus": {
	///      "$ref": "#/components/schemas/PduSessionStatus"
	///    },
	///    "sessInactiveTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionInfo {
		/// The identifier of the N4 session for the reported PDU Session.
		#[serde(rename = "n4SessId", default, skip_serializing_if = "Option::is_none")]
		pub n4_sess_id: Option<String>,
		#[serde(
			rename = "pduSessStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_sess_status: Option<PduSessionStatus>,
		#[serde(
			rename = "sessInactiveTimer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sess_inactive_timer: Option<DurationSec>,
	}

	impl From<&PduSessionInfo> for PduSessionInfo {
		fn from(value: &PduSessionInfo) -> Self {
			value.clone()
		}
	}

	/// Represents the PDU session related information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the PDU session related information.",
	///  "type": "object",
	///  "properties": {
	///    "pduSessId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "sessInfo": {
	///      "$ref": "#/components/schemas/PduSessionInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionInformation {
		#[serde(rename = "pduSessId", default, skip_serializing_if = "Option::is_none")]
		pub pdu_sess_id: Option<PduSessionId>,
		#[serde(rename = "sessInfo", default, skip_serializing_if = "Option::is_none")]
		pub sess_info: Option<PduSessionInfo>,
	}

	impl From<&PduSessionInformation> for PduSessionInformation {
		fn from(value: &PduSessionInformation) -> Self {
			value.clone()
		}
	}

	/// Notification related to a PDU session
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Notification related to a PDU session",
	///  "type": "object",
	///  "required": [
	///    "notificationCause"
	///  ],
	///  "properties": {
	///    "notificationCause": {
	///      "$ref": "#/components/schemas/NotificationCause"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionNotifyItem {
		#[serde(rename = "notificationCause")]
		pub notification_cause: NotificationCause,
	}

	impl From<&PduSessionNotifyItem> for PduSessionNotifyItem {
		fn from(value: &PduSessionNotifyItem) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - ACTIVATED: PDU Session status is activated.
	/// - DEACTIVATED: PDU Session status is deactivated.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- ACTIVATED: PDU Session status
	/// is activated.\n- DEACTIVATED: PDU Session status is deactivated.\n",
	///  "type": "string",
	///  "enum": [
	///    "ACTIVATED",
	///    "DEACTIVATED"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum PduSessionStatus {
		#[default]
		#[serde(rename = "ACTIVATED")]
		Activated,
		#[serde(rename = "DEACTIVATED")]
		Deactivated,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PduSessionStatus> for PduSessionStatus {
		fn from(value: &PduSessionStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for PduSessionStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Activated => "ACTIVATED".to_string(),
				Self::Deactivated => "DEACTIVATED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PduSessionStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ACTIVATED" => Ok(Self::Activated),
				"DEACTIVATED" => Ok(Self::Deactivated),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PduSessionStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PduSessionStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PduSessionStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// PduSessionType indicates the type of a PDU session. It shall comply with
	/// the provisions defined in table 5.4.3.3-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "PduSessionType indicates the type of a PDU session. It
	/// shall comply with the provisions defined in table 5.4.3.3-1. \n",
	///  "type": "string",
	///  "enum": [
	///    "IPV4",
	///    "IPV6",
	///    "IPV4V6",
	///    "UNSTRUCTURED",
	///    "ETHERNET"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum PduSessionType {
		#[default]
		#[serde(rename = "IPV4")]
		Ipv4,
		#[serde(rename = "IPV6")]
		Ipv6,
		#[serde(rename = "IPV4V6")]
		Ipv4v6,
		#[serde(rename = "UNSTRUCTURED")]
		Unstructured,
		#[serde(rename = "ETHERNET")]
		Ethernet,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PduSessionType> for PduSessionType {
		fn from(value: &PduSessionType) -> Self {
			value.clone()
		}
	}

	impl ToString for PduSessionType {
		fn to_string(&self) -> String {
			match *self {
				Self::Ipv4 => "IPV4".to_string(),
				Self::Ipv6 => "IPV6".to_string(),
				Self::Ipv4v6 => "IPV4V6".to_string(),
				Self::Unstructured => "UNSTRUCTURED".to_string(),
				Self::Ethernet => "ETHERNET".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PduSessionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IPV4" => Ok(Self::Ipv4),
				"IPV6" => Ok(Self::Ipv6),
				"IPV4V6" => Ok(Self::Ipv4v6),
				"UNSTRUCTURED" => Ok(Self::Unstructured),
				"ETHERNET" => Ok(Self::Ethernet),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PduSessionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PduSessionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PduSessionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// String representing a Permanent Equipment Identifier that may contain -
	/// an IMEI or IMEISV, as  specified in clause 6.2 of 3GPP TS 23.003; a MAC
	/// address for a 5G-RG or FN-RG via  wireline  access, with an indication
	/// that this address cannot be trusted for regulatory purpose if this
	/// address cannot be used as an Equipment Identifier of the FN-RG, as
	/// specified in clause 4.7.7  of 3GPP TS23.316. Examples are
	/// imei-012345678901234 or imeisv-0123456789012345.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing a Permanent Equipment Identifier
	/// that may contain - an IMEI or IMEISV, as  specified in clause 6.2 of
	/// 3GPP TS 23.003; a MAC address for a 5G-RG or FN-RG via  wireline
	/// access, with an indication that this address cannot be trusted for
	/// regulatory purpose if this  address cannot be used as an Equipment
	/// Identifier of the FN-RG, as specified in clause 4.7.7  of 3GPP TS23.316.
	/// Examples are imei-012345678901234 or imeisv-0123456789012345. \n",
	///  "type": "string",
	///  "pattern":
	/// "^(imei-[0-9]{15}|imeisv-[0-9]{16}|mac((-[0-9a-fA-F]{2}){6})(-untrusted)?
	/// |eui((-[0-9a-fA-F]{2}){8})|.+)$"
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
	)]
	pub struct Pei(String);

	impl ::std::ops::Deref for Pei {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Pei> for String {
		fn from(value: Pei) -> Self {
			value.0
		}
	}

	impl From<&Pei> for Pei {
		fn from(value: &Pei) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Pei {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"^(imei-[0-9]{15}|imeisv-[0-9]{16}|mac((-[0-9a-fA-F]{2}){6})(-untrusted)?\
				 |eui((-[0-9a-fA-F]{2}){8})|.+)$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^(imei-[0-9]{15}|imeisv-[0-9]{16}|mac((-[0-9a-fA-F]{2}){6})(-untrusted)?\
				            |eui((-[0-9a-fA-F]{2}){8})|.+)$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Pei {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Pei {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Pei {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Pei {
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

	/// When PlmnId needs to be converted to string (e.g. when used in maps as
	/// key), the string  shall be composed of three digits "mcc" followed by
	/// "-" and two or three digits "mnc".
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "When PlmnId needs to be converted to string (e.g. when
	/// used in maps as key), the string  shall be composed of three digits
	/// \"mcc\" followed by \"-\" and two or three digits \"mnc\".\n",
	///  "type": "object",
	///  "required": [
	///    "mcc",
	///    "mnc"
	///  ],
	///  "properties": {
	///    "mcc": {
	///      "$ref": "#/components/schemas/Mcc"
	///    },
	///    "mnc": {
	///      "$ref": "#/components/schemas/Mnc"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PlmnId {
		pub mcc: Mcc,
		pub mnc: Mnc,
	}

	impl From<&PlmnId> for PlmnId {
		fn from(value: &PlmnId) -> Self {
			value.clone()
		}
	}

	/// Contains the serving core network operator PLMN ID and, for an SNPN, the
	/// NID that together with the PLMN ID identifies the SNPN.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the serving core network operator PLMN ID and,
	/// for an SNPN, the NID that together with the PLMN ID identifies the
	/// SNPN.\n",
	///  "type": "object",
	///  "required": [
	///    "mcc",
	///    "mnc"
	///  ],
	///  "properties": {
	///    "mcc": {
	///      "$ref": "#/components/schemas/Mcc"
	///    },
	///    "mnc": {
	///      "$ref": "#/components/schemas/Mnc"
	///    },
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PlmnIdNid {
		pub mcc: Mcc,
		pub mnc: Mnc,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
	}

	impl From<&PlmnIdNid> for PlmnIdNid {
		fn from(value: &PlmnIdNid) -> Self {
			value.clone()
		}
	}

	/// The enumeration PreemptionCapability indicates the pre-emption
	/// capability of a request on other QoS flows. See clause 5.7.2.2 of 3GPP
	/// TS 23.501. It shall comply with the provisions defined in table
	/// 5.5.3.1-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration PreemptionCapability indicates the
	/// pre-emption capability of a request on other QoS flows. See clause
	/// 5.7.2.2 of 3GPP TS 23.501. It shall comply with the provisions defined
	/// in table 5.5.3.1-1.\n",
	///  "type": "string",
	///  "enum": [
	///    "NOT_PREEMPT",
	///    "MAY_PREEMPT"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum PreemptionCapability {
		#[default]
		#[serde(rename = "NOT_PREEMPT")]
		NotPreempt,
		#[serde(rename = "MAY_PREEMPT")]
		MayPreempt,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PreemptionCapability> for PreemptionCapability {
		fn from(value: &PreemptionCapability) -> Self {
			value.clone()
		}
	}

	impl ToString for PreemptionCapability {
		fn to_string(&self) -> String {
			match *self {
				Self::NotPreempt => "NOT_PREEMPT".to_string(),
				Self::MayPreempt => "MAY_PREEMPT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PreemptionCapability {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NOT_PREEMPT" => Ok(Self::NotPreempt),
				"MAY_PREEMPT" => Ok(Self::MayPreempt),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PreemptionCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PreemptionCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PreemptionCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration PreemptionVulnerability indicates the pre-emption
	/// vulnerability of the QoS flow to pre-emption from other QoS flows. See
	/// clause 5.7.2.2 of 3GPP TS 23.501. It shall comply with the provisions
	/// defined in table 5.5.3.2-1
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration PreemptionVulnerability indicates the
	/// pre-emption vulnerability of the QoS flow to pre-emption from other QoS
	/// flows. See clause 5.7.2.2 of 3GPP TS 23.501. It shall comply with the
	/// provisions defined in table 5.5.3.2-1\n",
	///  "type": "string",
	///  "enum": [
	///    "NOT_PREEMPTABLE",
	///    "PREEMPTABLE"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum PreemptionVulnerability {
		#[default]
		#[serde(rename = "NOT_PREEMPTABLE")]
		NotPreemptable,
		#[serde(rename = "PREEMPTABLE")]
		Preemptable,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PreemptionVulnerability> for PreemptionVulnerability {
		fn from(value: &PreemptionVulnerability) -> Self {
			value.clone()
		}
	}

	impl ToString for PreemptionVulnerability {
		fn to_string(&self) -> String {
			match *self {
				Self::NotPreemptable => "NOT_PREEMPTABLE".to_string(),
				Self::Preemptable => "PREEMPTABLE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PreemptionVulnerability {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NOT_PREEMPTABLE" => Ok(Self::NotPreemptable),
				"PREEMPTABLE" => Ok(Self::Preemptable),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PreemptionVulnerability {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PreemptionVulnerability {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PreemptionVulnerability {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// -IN_AREA: Indicates that the UE is inside or enters the presence
	/// reporting area. -OUT_OF_AREA: Indicates that the UE is outside or
	/// leaves the presence reporting area -UNKNOW: Indicates it is unknown
	/// whether the UE is in the presence reporting area or not -INACTIVE:
	/// Indicates that the presence reporting area is inactive in the serving
	/// node.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n-IN_AREA: Indicates that the UE
	/// is inside or enters the presence reporting area.\n-OUT_OF_AREA:
	/// Indicates that the UE is outside or leaves the presence reporting
	/// area\n-UNKNOW: Indicates it is unknown whether the UE is in the presence
	/// reporting area or not\n-INACTIVE: Indicates that the presence reporting
	/// area is inactive in the serving node. \n",
	///  "type": "string",
	///  "enum": [
	///    "IN_AREA",
	///    "OUT_OF_AREA",
	///    "UNKNOWN",
	///    "INACTIVE"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum PresenceState {
		#[default]
		#[serde(rename = "IN_AREA")]
		InArea,
		#[serde(rename = "OUT_OF_AREA")]
		OutOfArea,
		#[serde(rename = "UNKNOWN")]
		Unknown,
		#[serde(rename = "INACTIVE")]
		Inactive,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PresenceState> for PresenceState {
		fn from(value: &PresenceState) -> Self {
			value.clone()
		}
	}

	impl ToString for PresenceState {
		fn to_string(&self) -> String {
			match *self {
				Self::InArea => "IN_AREA".to_string(),
				Self::OutOfArea => "OUT_OF_AREA".to_string(),
				Self::Unknown => "UNKNOWN".to_string(),
				Self::Inactive => "INACTIVE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PresenceState {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IN_AREA" => Ok(Self::InArea),
				"OUT_OF_AREA" => Ok(Self::OutOfArea),
				"UNKNOWN" => Ok(Self::Unknown),
				"INACTIVE" => Ok(Self::Inactive),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PresenceState {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PresenceState {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PresenceState {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Provides additional information in an error response.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Provides additional information in an error response.",
	///  "type": "object",
	///  "properties": {
	///    "accessTokenError": {
	///      "$ref": "#/components/schemas/AccessTokenErr"
	///    },
	///    "accessTokenRequest": {
	///      "$ref": "#/components/schemas/AccessTokenReq"
	///    },
	///    "cause": {
	///      "description": "A machine-readable application error cause specific
	/// to this occurrence of the problem.  This IE should be present and
	/// provide application-related error information, if available.\n",
	///      "type": "string"
	///    },
	///    "detail": {
	///      "description": "A human-readable explanation specific to this
	/// occurrence of the problem.",
	///      "type": "string"
	///    },
	///    "instance": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "invalidParams": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/InvalidParam"
	///      },
	///      "minItems": 1
	///    },
	///    "nrfId": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "status": {
	///      "type": "integer"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "title": {
	///      "type": "string"
	///    },
	///    "type": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProblemDetails {
		#[serde(
			rename = "accessTokenError",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_token_error: Option<AccessTokenErr>,
		#[serde(
			rename = "accessTokenRequest",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_token_request: Option<AccessTokenReq>,
		/// A machine-readable application error cause specific to this
		/// occurrence of the problem.  This IE should be present and provide
		/// application-related error information, if available.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<String>,
		/// A human-readable explanation specific to this occurrence of the
		/// problem.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub detail: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub instance: Option<Uri>,
		#[serde(
			rename = "invalidParams",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub invalid_params: Vec<InvalidParam>,
		#[serde(rename = "nrfId", default, skip_serializing_if = "Option::is_none")]
		pub nrf_id: Option<Fqdn>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub status: Option<i64>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub title: Option<String>,
		#[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
		pub type_: Option<Uri>,
	}

	impl From<&ProblemDetails> for ProblemDetails {
		fn from(value: &ProblemDetails) -> Self {
			value.clone()
		}
	}

	/// Problem Details Additional Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Problem Details Additional Information",
	///  "type": "object",
	///  "properties": {
	///    "remoteError": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProblemDetailsAddInfo {
		#[serde(
			rename = "remoteError",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remote_error: Option<bool>,
	}

	impl From<&ProblemDetailsAddInfo> for ProblemDetailsAddInfo {
		fn from(value: &ProblemDetailsAddInfo) -> Self {
			value.clone()
		}
	}

	/// Procedure Transaction Identifier
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Procedure Transaction Identifier",
	///  "type": "integer",
	///  "maximum": 255.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProcedureTransactionId(pub u8);

	impl ::std::ops::Deref for ProcedureTransactionId {
		type Target = u8;
		fn deref(&self) -> &u8 {
			&self.0
		}
	}

	impl From<ProcedureTransactionId> for u8 {
		fn from(value: ProcedureTransactionId) -> Self {
			value.0
		}
	}

	impl From<&ProcedureTransactionId> for ProcedureTransactionId {
		fn from(value: &ProcedureTransactionId) -> Self {
			value.clone()
		}
	}

	impl From<u8> for ProcedureTransactionId {
		fn from(value: u8) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ProcedureTransactionId {
		type Err = <u8 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ProcedureTransactionId {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ProcedureTransactionId {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ProcedureTransactionId {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ProcedureTransactionId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Protection Result of the security policy indicated as "preferred".
	/// Possible values are
	///  - PERFORMED
	///  - NOT_PERFORMED
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Protection Result of the security policy indicated as \"preferred\". Possible values are\n  - PERFORMED\n  - NOT_PERFORMED\n",
	///  "type": "string",
	///  "enum": [
	///    "PERFORMED",
	///    "NOT_PERFORMED"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum ProtectionResult {
		#[default]
		#[serde(rename = "PERFORMED")]
		Performed,
		#[serde(rename = "NOT_PERFORMED")]
		NotPerformed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ProtectionResult> for ProtectionResult {
		fn from(value: &ProtectionResult) -> Self {
			value.clone()
		}
	}

	impl ToString for ProtectionResult {
		fn to_string(&self) -> String {
			match *self {
				Self::Performed => "PERFORMED".to_string(),
				Self::NotPerformed => "NOT_PERFORMED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ProtectionResult {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PERFORMED" => Ok(Self::Performed),
				"NOT_PERFORMED" => Ok(Self::NotPerformed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ProtectionResult {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ProtectionResult {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ProtectionResult {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indication of whether a PSA is inserted or removed. Possible values are
	///  - PSA_INSERTED
	///  - PSA_REMOVED
	///  - PSA_INSERTED_ONLY
	///  - PSA_REMOVED_ONLY
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indication of whether a PSA is inserted or removed.
	/// Possible values are\n  - PSA_INSERTED\n  - PSA_REMOVED\n  -
	/// PSA_INSERTED_ONLY\n  - PSA_REMOVED_ONLY\n",
	///  "type": "string",
	///  "enum": [
	///    "PSA_INSERTED",
	///    "PSA_REMOVED",
	///    "PSA_INSERTED_ONLY",
	///    "PSA_REMOVED_ONLY"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum PsaIndication {
		#[default]
		#[serde(rename = "PSA_INSERTED")]
		PsaInserted,
		#[serde(rename = "PSA_REMOVED")]
		PsaRemoved,
		#[serde(rename = "PSA_INSERTED_ONLY")]
		PsaInsertedOnly,
		#[serde(rename = "PSA_REMOVED_ONLY")]
		PsaRemovedOnly,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PsaIndication> for PsaIndication {
		fn from(value: &PsaIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for PsaIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::PsaInserted => "PSA_INSERTED".to_string(),
				Self::PsaRemoved => "PSA_REMOVED".to_string(),
				Self::PsaInsertedOnly => "PSA_INSERTED_ONLY".to_string(),
				Self::PsaRemovedOnly => "PSA_REMOVED_ONLY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PsaIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PSA_INSERTED" => Ok(Self::PsaInserted),
				"PSA_REMOVED" => Ok(Self::PsaRemoved),
				"PSA_INSERTED_ONLY" => Ok(Self::PsaInsertedOnly),
				"PSA_REMOVED_ONLY" => Ok(Self::PsaRemovedOnly),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PsaIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PsaIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PsaIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// PSA Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "PSA Information",
	///  "type": "object",
	///  "properties": {
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "psaInd": {
	///      "$ref": "#/components/schemas/PsaIndication"
	///    },
	///    "psaUpfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "ueIpv6Prefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PsaInformation {
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<Dnai>,
		#[serde(rename = "psaInd", default, skip_serializing_if = "Option::is_none")]
		pub psa_ind: Option<PsaIndication>,
		#[serde(rename = "psaUpfId", default, skip_serializing_if = "Option::is_none")]
		pub psa_upf_id: Option<NfInstanceId>,
		#[serde(
			rename = "ueIpv6Prefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_ipv6_prefix: Option<Ipv6Prefix>,
	}

	impl From<&PsaInformation> for PsaInformation {
		fn from(value: &PsaInformation) -> Self {
			value.clone()
		}
	}

	/// Unsigned integer identifying a QoS flow, within the range 0 to 63.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer identifying a QoS flow, within the
	/// range 0 to 63.",
	///  "type": "integer",
	///  "maximum": 63.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Qfi(pub i64);

	impl ::std::ops::Deref for Qfi {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<Qfi> for i64 {
		fn from(value: Qfi) -> Self {
			value.0
		}
	}

	impl From<&Qfi> for Qfi {
		fn from(value: &Qfi) -> Self {
			value.clone()
		}
	}

	impl From<i64> for Qfi {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Qfi {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Qfi {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Qfi {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Qfi {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Qfi {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Access type associated with a QoS Flow. Possible values are
	///  - 3GPP
	///  - NON_3GPP
	///  - 3GPP_AND_NON_3GPP
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Access type associated with a QoS Flow. Possible values
	/// are\n  - 3GPP\n  - NON_3GPP\n  - 3GPP_AND_NON_3GPP\n",
	///  "type": "string",
	///  "enum": [
	///    "3GPP",
	///    "NON_3GPP",
	///    "3GPP_AND_NON_3GPP"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum QosFlowAccessType {
		#[default]
		#[serde(rename = "3GPP")]
		ThreeGpp,
		#[serde(rename = "NON_3GPP")]
		Non3gpp,
		#[serde(rename = "3GPP_AND_NON_3GPP")]
		ThreeGppAndNon3gpp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&QosFlowAccessType> for QosFlowAccessType {
		fn from(value: &QosFlowAccessType) -> Self {
			value.clone()
		}
	}

	impl ToString for QosFlowAccessType {
		fn to_string(&self) -> String {
			match *self {
				Self::ThreeGpp => "3GPP".to_string(),
				Self::Non3gpp => "NON_3GPP".to_string(),
				Self::ThreeGppAndNon3gpp => "3GPP_AND_NON_3GPP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for QosFlowAccessType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"3GPP" => Ok(Self::ThreeGpp),
				"NON_3GPP" => Ok(Self::Non3gpp),
				"3GPP_AND_NON_3GPP" => Ok(Self::ThreeGppAndNon3gpp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for QosFlowAccessType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for QosFlowAccessType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for QosFlowAccessType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Individual QoS flow requested to be created or modified
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Individual QoS flow requested to be created or
	/// modified",
	///  "type": "object",
	///  "required": [
	///    "qfi"
	///  ],
	///  "properties": {
	///    "associatedAnType": {
	///      "$ref": "#/components/schemas/QosFlowAccessType"
	///    },
	///    "ebi": {
	///      "$ref": "#/components/schemas/EpsBearerId"
	///    },
	///    "qfi": {
	///      "$ref": "#/components/schemas/Qfi"
	///    },
	///    "qosFlowDescription": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "qosFlowProfile": {
	///      "$ref": "#/components/schemas/QosFlowProfile"
	///    },
	///    "qosRules": {
	///      "$ref": "#/components/schemas/Bytes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosFlowAddModifyRequestItem {
		#[serde(
			rename = "associatedAnType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub associated_an_type: Option<QosFlowAccessType>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ebi: Option<EpsBearerId>,
		pub qfi: Qfi,
		#[serde(
			rename = "qosFlowDescription",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_flow_description: Option<Bytes>,
		#[serde(
			rename = "qosFlowProfile",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_flow_profile: Option<QosFlowProfile>,
		#[serde(rename = "qosRules", default, skip_serializing_if = "Option::is_none")]
		pub qos_rules: Option<Bytes>,
	}

	impl From<&QosFlowAddModifyRequestItem> for QosFlowAddModifyRequestItem {
		fn from(value: &QosFlowAddModifyRequestItem) -> Self {
			value.clone()
		}
	}

	/// Individual QoS flow
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Individual QoS flow",
	///  "type": "object",
	///  "required": [
	///    "qfi"
	///  ],
	///  "properties": {
	///    "cause": {
	///      "$ref": "#/components/schemas/Cause"
	///    },
	///    "currentQosProfileIndex": {
	///      "type": "integer",
	///      "maximum": 8.0,
	///      "minimum": 1.0
	///    },
	///    "ngApCause": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    },
	///    "nullQoSProfileIndex": {
	///      "type": "boolean"
	///    },
	///    "qfi": {
	///      "$ref": "#/components/schemas/Qfi"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosFlowItem {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<Cause>,
		#[serde(
			rename = "currentQosProfileIndex",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub current_qos_profile_index: Option<i64>,
		#[serde(rename = "ngApCause", default, skip_serializing_if = "Option::is_none")]
		pub ng_ap_cause: Option<NgApCause>,
		#[serde(
			rename = "nullQoSProfileIndex",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub null_qo_s_profile_index: Option<bool>,
		pub qfi: Qfi,
	}

	impl From<&QosFlowItem> for QosFlowItem {
		fn from(value: &QosFlowItem) -> Self {
			value.clone()
		}
	}

	/// Notification related to a QoS flow
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Notification related to a QoS flow",
	///  "type": "object",
	///  "required": [
	///    "notificationCause",
	///    "qfi"
	///  ],
	///  "properties": {
	///    "currentQosProfileIndex": {
	///      "type": "integer",
	///      "maximum": 8.0,
	///      "minimum": 1.0
	///    },
	///    "notificationCause": {
	///      "$ref": "#/components/schemas/NotificationCause"
	///    },
	///    "nullQoSProfileIndex": {
	///      "type": "boolean"
	///    },
	///    "qfi": {
	///      "$ref": "#/components/schemas/Qfi"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosFlowNotifyItem {
		#[serde(
			rename = "currentQosProfileIndex",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub current_qos_profile_index: Option<i64>,
		#[serde(rename = "notificationCause")]
		pub notification_cause: NotificationCause,
		#[serde(
			rename = "nullQoSProfileIndex",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub null_qo_s_profile_index: Option<bool>,
		pub qfi: Qfi,
	}

	impl From<&QosFlowNotifyItem> for QosFlowNotifyItem {
		fn from(value: &QosFlowNotifyItem) -> Self {
			value.clone()
		}
	}

	/// QoS flow profile
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "QoS flow profile",
	///  "type": "object",
	///  "required": [
	///    "5qi"
	///  ],
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "additionalQosFlowInfo": {
	///      "$ref": "#/components/schemas/AdditionalQosFlowInfo"
	///    },
	///    "arp": {
	///      "$ref": "#/components/schemas/Arp"
	///    },
	///    "dynamic5Qi": {
	///      "$ref": "#/components/schemas/Dynamic5Qi"
	///    },
	///    "gbrQosFlowInfo": {
	///      "$ref": "#/components/schemas/GbrQosFlowInformation"
	///    },
	///    "nonDynamic5Qi": {
	///      "$ref": "#/components/schemas/NonDynamic5Qi"
	///    },
	///    "qosMonitoringReq": {
	///      "$ref": "#/components/schemas/QosMonitoringReq"
	///    },
	///    "qosRepPeriod": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "rqa": {
	///      "$ref": "#/components/schemas/ReflectiveQoSAttribute"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosFlowProfile {
		#[serde(
			rename = "additionalQosFlowInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_qos_flow_info: Option<AdditionalQosFlowInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub arp: Option<Arp>,
		#[serde(
			rename = "dynamic5Qi",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dynamic5_qi: Option<Dynamic5Qi>,
		#[serde(rename = "5qi")]
		pub five_qi: _5qi,
		#[serde(
			rename = "gbrQosFlowInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub gbr_qos_flow_info: Option<GbrQosFlowInformation>,
		#[serde(
			rename = "nonDynamic5Qi",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub non_dynamic5_qi: Option<NonDynamic5Qi>,
		#[serde(
			rename = "qosMonitoringReq",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_monitoring_req: Option<QosMonitoringReq>,
		#[serde(
			rename = "qosRepPeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_rep_period: Option<DurationSec>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub rqa: Option<ReflectiveQoSAttribute>,
	}

	impl From<&QosFlowProfile> for QosFlowProfile {
		fn from(value: &QosFlowProfile) -> Self {
			value.clone()
		}
	}

	/// Individual QoS flow requested to be released
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Individual QoS flow requested to be released",
	///  "type": "object",
	///  "required": [
	///    "qfi"
	///  ],
	///  "properties": {
	///    "qfi": {
	///      "$ref": "#/components/schemas/Qfi"
	///    },
	///    "qosFlowDescription": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "qosRules": {
	///      "$ref": "#/components/schemas/Bytes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosFlowReleaseRequestItem {
		pub qfi: Qfi,
		#[serde(
			rename = "qosFlowDescription",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_flow_description: Option<Bytes>,
		#[serde(rename = "qosRules", default, skip_serializing_if = "Option::is_none")]
		pub qos_rules: Option<Bytes>,
	}

	impl From<&QosFlowReleaseRequestItem> for QosFlowReleaseRequestItem {
		fn from(value: &QosFlowReleaseRequestItem) -> Self {
			value.clone()
		}
	}

	/// Individual QoS flow to setup
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Individual QoS flow to setup",
	///  "type": "object",
	///  "required": [
	///    "qfi",
	///    "qosRules"
	///  ],
	///  "properties": {
	///    "associatedAnType": {
	///      "$ref": "#/components/schemas/QosFlowAccessType"
	///    },
	///    "defaultQosRuleInd": {
	///      "type": "boolean"
	///    },
	///    "ebi": {
	///      "$ref": "#/components/schemas/EpsBearerId"
	///    },
	///    "qfi": {
	///      "$ref": "#/components/schemas/Qfi"
	///    },
	///    "qosFlowDescription": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "qosFlowProfile": {
	///      "$ref": "#/components/schemas/QosFlowProfile"
	///    },
	///    "qosRules": {
	///      "$ref": "#/components/schemas/Bytes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosFlowSetupItem {
		#[serde(
			rename = "associatedAnType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub associated_an_type: Option<QosFlowAccessType>,
		#[serde(
			rename = "defaultQosRuleInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub default_qos_rule_ind: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ebi: Option<EpsBearerId>,
		pub qfi: Qfi,
		#[serde(
			rename = "qosFlowDescription",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_flow_description: Option<Bytes>,
		#[serde(
			rename = "qosFlowProfile",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_flow_profile: Option<QosFlowProfile>,
		#[serde(rename = "qosRules")]
		pub qos_rules: Bytes,
	}

	impl From<&QosFlowSetupItem> for QosFlowSetupItem {
		fn from(value: &QosFlowSetupItem) -> Self {
			value.clone()
		}
	}

	/// Tunnel Information per QoS Flow
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Tunnel Information per QoS Flow",
	///  "type": "object",
	///  "required": [
	///    "qfiList",
	///    "tunnelInfo"
	///  ],
	///  "properties": {
	///    "qfiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Qfi"
	///      },
	///      "minItems": 1
	///    },
	///    "tunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosFlowTunnel {
		#[serde(rename = "qfiList")]
		pub qfi_list: Vec<Qfi>,
		#[serde(rename = "tunnelInfo")]
		pub tunnel_info: TunnelInfo,
	}

	impl From<&QosFlowTunnel> for QosFlowTunnel {
		fn from(value: &QosFlowTunnel) -> Self {
			value.clone()
		}
	}

	/// Contains QoS flows usage data information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains QoS flows usage data information.",
	///  "type": "object",
	///  "required": [
	///    "downlinkVolume",
	///    "endTimeStamp",
	///    "qfi",
	///    "startTimeStamp",
	///    "uplinkVolume"
	///  ],
	///  "properties": {
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/Int64"
	///    },
	///    "endTimeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "qfi": {
	///      "$ref": "#/components/schemas/Qfi"
	///    },
	///    "startTimeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "uplinkVolume": {
	///      "$ref": "#/components/schemas/Int64"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosFlowUsageReport {
		#[serde(rename = "downlinkVolume")]
		pub downlink_volume: Int64,
		#[serde(rename = "endTimeStamp")]
		pub end_time_stamp: DateTime,
		pub qfi: Qfi,
		#[serde(rename = "startTimeStamp")]
		pub start_time_stamp: DateTime,
		#[serde(rename = "uplinkVolume")]
		pub uplink_volume: Int64,
	}

	impl From<&QosFlowUsageReport> for QosFlowUsageReport {
		fn from(value: &QosFlowUsageReport) -> Self {
			value.clone()
		}
	}

	/// QoS Monitoring Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "QoS Monitoring Information",
	///  "type": "object",
	///  "properties": {
	///    "qosMonitoringInd": {
	///      "default": false,
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosMonitoringInfo {
		#[serde(rename = "qosMonitoringInd", default)]
		pub qos_monitoring_ind: bool,
	}

	impl From<&QosMonitoringInfo> for QosMonitoringInfo {
		fn from(value: &QosMonitoringInfo) -> Self {
			value.clone()
		}
	}

	/// QoS monitoring request. Possible values are
	///  - UL
	///  - DL
	///  - BOTH
	///  - NONE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "QoS monitoring request. Possible values are\n  - UL\n
	/// - DL\n  - BOTH\n  - NONE\n",
	///  "type": "string",
	///  "enum": [
	///    "UL",
	///    "DL",
	///    "BOTH",
	///    "NONE"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum QosMonitoringReq {
		#[default]
		#[serde(rename = "UL")]
		Ul,
		#[serde(rename = "DL")]
		Dl,
		#[serde(rename = "BOTH")]
		Both,
		#[serde(rename = "NONE")]
		None,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&QosMonitoringReq> for QosMonitoringReq {
		fn from(value: &QosMonitoringReq) -> Self {
			value.clone()
		}
	}

	impl ToString for QosMonitoringReq {
		fn to_string(&self) -> String {
			match *self {
				Self::Ul => "UL".to_string(),
				Self::Dl => "DL".to_string(),
				Self::Both => "BOTH".to_string(),
				Self::None => "NONE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for QosMonitoringReq {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UL" => Ok(Self::Ul),
				"DL" => Ok(Self::Dl),
				"BOTH" => Ok(Self::Both),
				"NONE" => Ok(Self::None),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for QosMonitoringReq {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for QosMonitoringReq {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for QosMonitoringReq {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration QosResourceType indicates whether a QoS Flow is non-GBR,
	/// delay critical GBR, or non-delay critical GBR (see clauses 5.7.3.4 and
	/// 5.7.3.5 of 3GPP TS 23.501). It shall comply with the provisions defined
	/// in table 5.5.3.6-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration QosResourceType indicates whether a QoS
	/// Flow is non-GBR, delay critical GBR, or non-delay critical GBR (see
	/// clauses 5.7.3.4 and 5.7.3.5 of 3GPP TS 23.501). It shall comply with the
	/// provisions defined in table 5.5.3.6-1. \n",
	///  "type": "string",
	///  "enum": [
	///    "NON_GBR",
	///    "NON_CRITICAL_GBR",
	///    "CRITICAL_GBR"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum QosResourceType {
		#[default]
		#[serde(rename = "NON_GBR")]
		NonGbr,
		#[serde(rename = "NON_CRITICAL_GBR")]
		NonCriticalGbr,
		#[serde(rename = "CRITICAL_GBR")]
		CriticalGbr,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&QosResourceType> for QosResourceType {
		fn from(value: &QosResourceType) -> Self {
			value.clone()
		}
	}

	impl ToString for QosResourceType {
		fn to_string(&self) -> String {
			match *self {
				Self::NonGbr => "NON_GBR".to_string(),
				Self::NonCriticalGbr => "NON_CRITICAL_GBR".to_string(),
				Self::CriticalGbr => "CRITICAL_GBR".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for QosResourceType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NON_GBR" => Ok(Self::NonGbr),
				"NON_CRITICAL_GBR" => Ok(Self::NonCriticalGbr),
				"CRITICAL_GBR" => Ok(Self::CriticalGbr),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for QosResourceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for QosResourceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for QosResourceType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the radio access used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the radio access used.",
	///  "type": "string",
	///  "enum": [
	///    "NR",
	///    "EUTRA",
	///    "WLAN",
	///    "VIRTUAL",
	///    "NBIOT",
	///    "WIRELINE",
	///    "WIRELINE_CABLE",
	///    "WIRELINE_BBF",
	///    "LTE-M",
	///    "NR_U",
	///    "EUTRA_U",
	///    "TRUSTED_N3GA",
	///    "TRUSTED_WLAN",
	///    "UTRA",
	///    "GERA",
	///    "NR_LEO",
	///    "NR_MEO",
	///    "NR_GEO",
	///    "NR_OTHER_SAT",
	///    "NR_REDCAP",
	///    "WB_E_UTRAN_LEO",
	///    "WB_E_UTRAN_MEO",
	///    "WB_E_UTRAN_GEO",
	///    "WB_E_UTRAN_OTHERSAT",
	///    "NB_IOT_LEO",
	///    "NB_IOT_MEO",
	///    "NB_IOT_GEO",
	///    "NB_IOT_OTHERSAT",
	///    "LTE_M_LEO",
	///    "LTE_M_MEO",
	///    "LTE_M_GEO",
	///    "LTE_M_OTHERSAT"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum RatType {
		#[default]
		#[serde(rename = "NR")]
		Nr,
		#[serde(rename = "EUTRA")]
		Eutra,
		#[serde(rename = "WLAN")]
		Wlan,
		#[serde(rename = "VIRTUAL")]
		Virtual,
		#[serde(rename = "NBIOT")]
		Nbiot,
		#[serde(rename = "WIRELINE")]
		Wireline,
		#[serde(rename = "WIRELINE_CABLE")]
		WirelineCable,
		#[serde(rename = "WIRELINE_BBF")]
		WirelineBbf,
		#[serde(rename = "LTE-M")]
		LteM,
		#[serde(rename = "NR_U")]
		NrU,
		#[serde(rename = "EUTRA_U")]
		EutraU,
		#[serde(rename = "TRUSTED_N3GA")]
		TrustedN3ga,
		#[serde(rename = "TRUSTED_WLAN")]
		TrustedWlan,
		#[serde(rename = "UTRA")]
		Utra,
		#[serde(rename = "GERA")]
		Gera,
		#[serde(rename = "NR_LEO")]
		NrLeo,
		#[serde(rename = "NR_MEO")]
		NrMeo,
		#[serde(rename = "NR_GEO")]
		NrGeo,
		#[serde(rename = "NR_OTHER_SAT")]
		NrOtherSat,
		#[serde(rename = "NR_REDCAP")]
		NrRedcap,
		#[serde(rename = "WB_E_UTRAN_LEO")]
		WbEUtranLeo,
		#[serde(rename = "WB_E_UTRAN_MEO")]
		WbEUtranMeo,
		#[serde(rename = "WB_E_UTRAN_GEO")]
		WbEUtranGeo,
		#[serde(rename = "WB_E_UTRAN_OTHERSAT")]
		WbEUtranOthersat,
		#[serde(rename = "NB_IOT_LEO")]
		NbIotLeo,
		#[serde(rename = "NB_IOT_MEO")]
		NbIotMeo,
		#[serde(rename = "NB_IOT_GEO")]
		NbIotGeo,
		#[serde(rename = "NB_IOT_OTHERSAT")]
		NbIotOthersat,
		#[serde(rename = "LTE_M_LEO")]
		LteMLeo,
		#[serde(rename = "LTE_M_MEO")]
		LteMMeo,
		#[serde(rename = "LTE_M_GEO")]
		LteMGeo,
		#[serde(rename = "LTE_M_OTHERSAT")]
		LteMOthersat,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RatType> for RatType {
		fn from(value: &RatType) -> Self {
			value.clone()
		}
	}

	impl ToString for RatType {
		fn to_string(&self) -> String {
			match *self {
				Self::Nr => "NR".to_string(),
				Self::Eutra => "EUTRA".to_string(),
				Self::Wlan => "WLAN".to_string(),
				Self::Virtual => "VIRTUAL".to_string(),
				Self::Nbiot => "NBIOT".to_string(),
				Self::Wireline => "WIRELINE".to_string(),
				Self::WirelineCable => "WIRELINE_CABLE".to_string(),
				Self::WirelineBbf => "WIRELINE_BBF".to_string(),
				Self::LteM => "LTE-M".to_string(),
				Self::NrU => "NR_U".to_string(),
				Self::EutraU => "EUTRA_U".to_string(),
				Self::TrustedN3ga => "TRUSTED_N3GA".to_string(),
				Self::TrustedWlan => "TRUSTED_WLAN".to_string(),
				Self::Utra => "UTRA".to_string(),
				Self::Gera => "GERA".to_string(),
				Self::NrLeo => "NR_LEO".to_string(),
				Self::NrMeo => "NR_MEO".to_string(),
				Self::NrGeo => "NR_GEO".to_string(),
				Self::NrOtherSat => "NR_OTHER_SAT".to_string(),
				Self::NrRedcap => "NR_REDCAP".to_string(),
				Self::WbEUtranLeo => "WB_E_UTRAN_LEO".to_string(),
				Self::WbEUtranMeo => "WB_E_UTRAN_MEO".to_string(),
				Self::WbEUtranGeo => "WB_E_UTRAN_GEO".to_string(),
				Self::WbEUtranOthersat => "WB_E_UTRAN_OTHERSAT".to_string(),
				Self::NbIotLeo => "NB_IOT_LEO".to_string(),
				Self::NbIotMeo => "NB_IOT_MEO".to_string(),
				Self::NbIotGeo => "NB_IOT_GEO".to_string(),
				Self::NbIotOthersat => "NB_IOT_OTHERSAT".to_string(),
				Self::LteMLeo => "LTE_M_LEO".to_string(),
				Self::LteMMeo => "LTE_M_MEO".to_string(),
				Self::LteMGeo => "LTE_M_GEO".to_string(),
				Self::LteMOthersat => "LTE_M_OTHERSAT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RatType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NR" => Ok(Self::Nr),
				"EUTRA" => Ok(Self::Eutra),
				"WLAN" => Ok(Self::Wlan),
				"VIRTUAL" => Ok(Self::Virtual),
				"NBIOT" => Ok(Self::Nbiot),
				"WIRELINE" => Ok(Self::Wireline),
				"WIRELINE_CABLE" => Ok(Self::WirelineCable),
				"WIRELINE_BBF" => Ok(Self::WirelineBbf),
				"LTE-M" => Ok(Self::LteM),
				"NR_U" => Ok(Self::NrU),
				"EUTRA_U" => Ok(Self::EutraU),
				"TRUSTED_N3GA" => Ok(Self::TrustedN3ga),
				"TRUSTED_WLAN" => Ok(Self::TrustedWlan),
				"UTRA" => Ok(Self::Utra),
				"GERA" => Ok(Self::Gera),
				"NR_LEO" => Ok(Self::NrLeo),
				"NR_MEO" => Ok(Self::NrMeo),
				"NR_GEO" => Ok(Self::NrGeo),
				"NR_OTHER_SAT" => Ok(Self::NrOtherSat),
				"NR_REDCAP" => Ok(Self::NrRedcap),
				"WB_E_UTRAN_LEO" => Ok(Self::WbEUtranLeo),
				"WB_E_UTRAN_MEO" => Ok(Self::WbEUtranMeo),
				"WB_E_UTRAN_GEO" => Ok(Self::WbEUtranGeo),
				"WB_E_UTRAN_OTHERSAT" => Ok(Self::WbEUtranOthersat),
				"NB_IOT_LEO" => Ok(Self::NbIotLeo),
				"NB_IOT_MEO" => Ok(Self::NbIotMeo),
				"NB_IOT_GEO" => Ok(Self::NbIotGeo),
				"NB_IOT_OTHERSAT" => Ok(Self::NbIotOthersat),
				"LTE_M_LEO" => Ok(Self::LteMLeo),
				"LTE_M_MEO" => Ok(Self::LteMMeo),
				"LTE_M_GEO" => Ok(Self::LteMGeo),
				"LTE_M_OTHERSAT" => Ok(Self::LteMOthersat),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RatType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RatType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RatType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The response shall include a Location header field containing a
	/// different URI  (pointing to a different URI of an other service
	/// instance), or the same URI if a request  is redirected to the same
	/// target resource via a different SCP.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The response shall include a Location header field
	/// containing a different URI  (pointing to a different URI of an other
	/// service instance), or the same URI if a request  is redirected to the
	/// same target resource via a different SCP.\n",
	///  "type": "object",
	///  "properties": {
	///    "cause": {
	///      "type": "string"
	///    },
	///    "targetScp": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "targetSepp": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RedirectResponse {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<String>,
		#[serde(rename = "targetScp", default, skip_serializing_if = "Option::is_none")]
		pub target_scp: Option<Uri>,
		#[serde(
			rename = "targetSepp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_sepp: Option<Uri>,
	}

	impl From<&RedirectResponse> for RedirectResponse {
		fn from(value: &RedirectResponse) -> Self {
			value.clone()
		}
	}

	/// Redundant PDU Session Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Redundant PDU Session Information",
	///  "type": "object",
	///  "required": [
	///    "rsn"
	///  ],
	///  "properties": {
	///    "pduSessionPairId": {
	///      "type": "integer",
	///      "maximum": 255.0,
	///      "minimum": 0.0
	///    },
	///    "rsn": {
	///      "$ref": "#/components/schemas/Rsn"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RedundantPduSessionInformation {
		#[serde(
			rename = "pduSessionPairId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_pair_id: Option<u8>,
		pub rsn: Rsn,
	}

	impl From<&RedundantPduSessionInformation> for RedundantPduSessionInformation {
		fn from(value: &RedundantPduSessionInformation) -> Self {
			value.clone()
		}
	}

	/// This parameter provides information about the referenced binary body
	/// data.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This parameter provides information about the
	/// referenced binary body data.",
	///  "type": "object",
	///  "required": [
	///    "contentId"
	///  ],
	///  "properties": {
	///    "contentId": {
	///      "description": "This IE shall contain the value of the Content-ID
	/// header of the referenced binary body part.\n",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RefToBinaryData {
		/// This IE shall contain the value of the Content-ID header of the
		/// referenced binary body part.
		#[serde(rename = "contentId")]
		pub content_id: String,
	}

	impl From<&RefToBinaryData> for RefToBinaryData {
		fn from(value: &RefToBinaryData) -> Self {
			value.clone()
		}
	}

	/// The enumeration ReflectiveQosAttribute indicates whether certain traffic
	/// of the QoS flow may be subject to Reflective QoS (see clause 5.7.2.3 of
	/// 3GPP TS 23.501). It shall comply with the provisions defined in table
	/// 5.5.3.3-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration ReflectiveQosAttribute indicates
	/// whether certain traffic of the QoS flow may be subject to Reflective QoS
	/// (see clause 5.7.2.3 of 3GPP TS 23.501). It shall comply with the
	/// provisions defined in table 5.5.3.3-1. \n",
	///  "type": "string",
	///  "enum": [
	///    "RQOS",
	///    "NO_RQOS"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum ReflectiveQoSAttribute {
		#[default]
		#[serde(rename = "RQOS")]
		Rqos,
		#[serde(rename = "NO_RQOS")]
		NoRqos,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReflectiveQoSAttribute> for ReflectiveQoSAttribute {
		fn from(value: &ReflectiveQoSAttribute) -> Self {
			value.clone()
		}
	}

	impl ToString for ReflectiveQoSAttribute {
		fn to_string(&self) -> String {
			match *self {
				Self::Rqos => "RQOS".to_string(),
				Self::NoRqos => "NO_RQOS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReflectiveQoSAttribute {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"RQOS" => Ok(Self::Rqos),
				"NO_RQOS" => Ok(Self::NoRqos),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReflectiveQoSAttribute {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReflectiveQoSAttribute {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReflectiveQoSAttribute {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within Release Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Release Request",
	///  "type": "object",
	///  "properties": {
	///    "5gMmCauseValue": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "addUeLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "cause": {
	///      "$ref": "#/components/schemas/Cause"
	///    },
	///    "n4Info": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt1": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt2": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "ngApCause": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    },
	///    "secondaryRatUsageInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SecondaryRatUsageInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "secondaryRatUsageReport": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SecondaryRatUsageReport"
	///      },
	///      "minItems": 1
	///    },
	///    "ueLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReleaseData {
		#[serde(
			rename = "addUeLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_ue_location: Option<UserLocation>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<Cause>,
		#[serde(
			rename = "5gMmCauseValue",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_g_mm_cause_value: Option<Uinteger>,
		#[serde(rename = "n4Info", default, skip_serializing_if = "Option::is_none")]
		pub n4_info: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt1",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext1: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt2",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext2: Option<N4Information>,
		#[serde(rename = "ngApCause", default, skip_serializing_if = "Option::is_none")]
		pub ng_ap_cause: Option<NgApCause>,
		#[serde(
			rename = "secondaryRatUsageInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub secondary_rat_usage_info: Vec<SecondaryRatUsageInfo>,
		#[serde(
			rename = "secondaryRatUsageReport",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub secondary_rat_usage_report: Vec<SecondaryRatUsageReport>,
		#[serde(
			rename = "ueLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_location: Option<UserLocation>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
	}

	impl From<&ReleaseData> for ReleaseData {
		fn from(value: &ReleaseData) -> Self {
			value.clone()
		}
	}

	/// Data within Release Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Release Response",
	///  "type": "object",
	///  "properties": {
	///    "apnRateStatus": {
	///      "$ref": "#/components/schemas/ApnRateStatus"
	///    },
	///    "n4Info": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt1": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt2": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "smallDataRateStatus": {
	///      "$ref": "#/components/schemas/SmallDataRateStatus"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReleasedData {
		#[serde(
			rename = "apnRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub apn_rate_status: Option<ApnRateStatus>,
		#[serde(rename = "n4Info", default, skip_serializing_if = "Option::is_none")]
		pub n4_info: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt1",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext1: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt2",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext2: Option<N4Information>,
		#[serde(
			rename = "smallDataRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub small_data_rate_status: Option<SmallDataRateStatus>,
	}

	impl From<&ReleasedData> for ReleasedData {
		fn from(value: &ReleasedData) -> Self {
			value.clone()
		}
	}

	/// Request Indication in Update (SM context) service operation. Possible
	/// values are
	/// - UE_REQ_PDU_SES_MOD
	/// - UE_REQ_PDU_SES_REL
	/// - PDU_SES_MOB
	/// - NW_REQ_PDU_SES_AUTH
	/// - NW_REQ_PDU_SES_MOD
	/// - NW_REQ_PDU_SES_REL
	/// - EBI_ASSIGNMENT_REQ
	/// - REL_DUE_TO_5G_AN_REQUEST
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Request Indication in Update (SM context) service
	/// operation. Possible values are\n- UE_REQ_PDU_SES_MOD\n-
	/// UE_REQ_PDU_SES_REL\n- PDU_SES_MOB\n- NW_REQ_PDU_SES_AUTH\n-
	/// NW_REQ_PDU_SES_MOD\n- NW_REQ_PDU_SES_REL\n- EBI_ASSIGNMENT_REQ\n-
	/// REL_DUE_TO_5G_AN_REQUEST\n",
	///  "type": "string",
	///  "enum": [
	///    "UE_REQ_PDU_SES_MOD",
	///    "UE_REQ_PDU_SES_REL",
	///    "PDU_SES_MOB",
	///    "NW_REQ_PDU_SES_AUTH",
	///    "NW_REQ_PDU_SES_MOD",
	///    "NW_REQ_PDU_SES_REL",
	///    "EBI_ASSIGNMENT_REQ",
	///    "REL_DUE_TO_5G_AN_REQUEST"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum RequestIndication {
		#[default]
		#[serde(rename = "UE_REQ_PDU_SES_MOD")]
		UeReqPduSesMod,
		#[serde(rename = "UE_REQ_PDU_SES_REL")]
		UeReqPduSesRel,
		#[serde(rename = "PDU_SES_MOB")]
		PduSesMob,
		#[serde(rename = "NW_REQ_PDU_SES_AUTH")]
		NwReqPduSesAuth,
		#[serde(rename = "NW_REQ_PDU_SES_MOD")]
		NwReqPduSesMod,
		#[serde(rename = "NW_REQ_PDU_SES_REL")]
		NwReqPduSesRel,
		#[serde(rename = "EBI_ASSIGNMENT_REQ")]
		EbiAssignmentReq,
		#[serde(rename = "REL_DUE_TO_5G_AN_REQUEST")]
		RelDueTo5gAnRequest,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RequestIndication> for RequestIndication {
		fn from(value: &RequestIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for RequestIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::UeReqPduSesMod => "UE_REQ_PDU_SES_MOD".to_string(),
				Self::UeReqPduSesRel => "UE_REQ_PDU_SES_REL".to_string(),
				Self::PduSesMob => "PDU_SES_MOB".to_string(),
				Self::NwReqPduSesAuth => "NW_REQ_PDU_SES_AUTH".to_string(),
				Self::NwReqPduSesMod => "NW_REQ_PDU_SES_MOD".to_string(),
				Self::NwReqPduSesRel => "NW_REQ_PDU_SES_REL".to_string(),
				Self::EbiAssignmentReq => "EBI_ASSIGNMENT_REQ".to_string(),
				Self::RelDueTo5gAnRequest => "REL_DUE_TO_5G_AN_REQUEST".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RequestIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UE_REQ_PDU_SES_MOD" => Ok(Self::UeReqPduSesMod),
				"UE_REQ_PDU_SES_REL" => Ok(Self::UeReqPduSesRel),
				"PDU_SES_MOB" => Ok(Self::PduSesMob),
				"NW_REQ_PDU_SES_AUTH" => Ok(Self::NwReqPduSesAuth),
				"NW_REQ_PDU_SES_MOD" => Ok(Self::NwReqPduSesMod),
				"NW_REQ_PDU_SES_REL" => Ok(Self::NwReqPduSesRel),
				"EBI_ASSIGNMENT_REQ" => Ok(Self::EbiAssignmentReq),
				"REL_DUE_TO_5G_AN_REQUEST" => Ok(Self::RelDueTo5gAnRequest),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RequestIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RequestIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RequestIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Request Type in Create (SM context) service operation. Possible values
	/// are
	/// - INITIAL_REQUEST
	/// - EXISTING_PDU_SESSION
	/// - INITIAL_EMERGENCY_REQUEST
	/// - EXISTING_EMERGENCY_PDU_SESSION
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Request Type in Create (SM context) service operation.
	/// Possible values are\n- INITIAL_REQUEST\n- EXISTING_PDU_SESSION\n-
	/// INITIAL_EMERGENCY_REQUEST\n- EXISTING_EMERGENCY_PDU_SESSION\n",
	///  "type": "string",
	///  "enum": [
	///    "INITIAL_REQUEST",
	///    "EXISTING_PDU_SESSION",
	///    "INITIAL_EMERGENCY_REQUEST",
	///    "EXISTING_EMERGENCY_PDU_SESSION"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum RequestType {
		#[default]
		#[serde(rename = "INITIAL_REQUEST")]
		InitialRequest,
		#[serde(rename = "EXISTING_PDU_SESSION")]
		ExistingPduSession,
		#[serde(rename = "INITIAL_EMERGENCY_REQUEST")]
		InitialEmergencyRequest,
		#[serde(rename = "EXISTING_EMERGENCY_PDU_SESSION")]
		ExistingEmergencyPduSession,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RequestType> for RequestType {
		fn from(value: &RequestType) -> Self {
			value.clone()
		}
	}

	impl ToString for RequestType {
		fn to_string(&self) -> String {
			match *self {
				Self::InitialRequest => "INITIAL_REQUEST".to_string(),
				Self::ExistingPduSession => "EXISTING_PDU_SESSION".to_string(),
				Self::InitialEmergencyRequest => "INITIAL_EMERGENCY_REQUEST".to_string(),
				Self::ExistingEmergencyPduSession => "EXISTING_EMERGENCY_PDU_SESSION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RequestType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"INITIAL_REQUEST" => Ok(Self::InitialRequest),
				"EXISTING_PDU_SESSION" => Ok(Self::ExistingPduSession),
				"INITIAL_EMERGENCY_REQUEST" => Ok(Self::InitialEmergencyRequest),
				"EXISTING_EMERGENCY_PDU_SESSION" => Ok(Self::ExistingEmergencyPduSession),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RequestType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RequestType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RequestType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Status of SM context or PDU session resource. Possible values are
	/// - RELEASED
	/// - UNCHANGED
	/// - TRANSFERRED
	/// - UPDATED
	/// - ALT_ANCHOR_SMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Status of SM context or PDU session resource. Possible
	/// values are\n- RELEASED\n- UNCHANGED\n- TRANSFERRED\n- UPDATED\n-
	/// ALT_ANCHOR_SMF\n",
	///  "type": "string",
	///  "enum": [
	///    "RELEASED",
	///    "UNCHANGED",
	///    "TRANSFERRED",
	///    "UPDATED",
	///    "ALT_ANCHOR_SMF"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum ResourceStatus {
		#[default]
		#[serde(rename = "RELEASED")]
		Released,
		#[serde(rename = "UNCHANGED")]
		Unchanged,
		#[serde(rename = "TRANSFERRED")]
		Transferred,
		#[serde(rename = "UPDATED")]
		Updated,
		#[serde(rename = "ALT_ANCHOR_SMF")]
		AltAnchorSmf,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ResourceStatus> for ResourceStatus {
		fn from(value: &ResourceStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for ResourceStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Released => "RELEASED".to_string(),
				Self::Unchanged => "UNCHANGED".to_string(),
				Self::Transferred => "TRANSFERRED".to_string(),
				Self::Updated => "UPDATED".to_string(),
				Self::AltAnchorSmf => "ALT_ANCHOR_SMF".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ResourceStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"RELEASED" => Ok(Self::Released),
				"UNCHANGED" => Ok(Self::Unchanged),
				"TRANSFERRED" => Ok(Self::Transferred),
				"UPDATED" => Ok(Self::Updated),
				"ALT_ANCHOR_SMF" => Ok(Self::AltAnchorSmf),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ResourceStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ResourceStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ResourceStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within Retrieve Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Retrieve Request",
	///  "type": "object",
	///  "properties": {
	///    "pduSessionContextType": {
	///      "$ref": "#/components/schemas/PduSessionContextType"
	///    },
	///    "smallDataRateStatusReq": {
	///      "default": false,
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RetrieveData {
		#[serde(
			rename = "pduSessionContextType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_context_type: Option<PduSessionContextType>,
		#[serde(rename = "smallDataRateStatusReq", default)]
		pub small_data_rate_status_req: bool,
	}

	impl From<&RetrieveData> for RetrieveData {
		fn from(value: &RetrieveData) -> Self {
			value.clone()
		}
	}

	/// Data within Retrieve Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Retrieve Response",
	///  "type": "object",
	///  "properties": {
	///    "afCoordinationInfo": {
	///      "$ref": "#/components/schemas/AfCoordinationInfo"
	///    },
	///    "smallDataRateStatus": {
	///      "$ref": "#/components/schemas/SmallDataRateStatus"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RetrievedData {
		#[serde(
			rename = "afCoordinationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub af_coordination_info: Option<AfCoordinationInfo>,
		#[serde(
			rename = "smallDataRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub small_data_rate_status: Option<SmallDataRateStatus>,
	}

	impl From<&RetrievedData> for RetrievedData {
		fn from(value: &RetrievedData) -> Self {
			value.clone()
		}
	}

	/// RoamingChargingProfile
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "partialRecordMethod": {
	///      "$ref": "#/components/schemas/PartialRecordMethod"
	///    },
	///    "triggers": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Trigger"
	///      },
	///      "minItems": 0
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RoamingChargingProfile {
		#[serde(
			rename = "partialRecordMethod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub partial_record_method: Option<PartialRecordMethod>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub triggers: Vec<Trigger>,
	}

	impl From<&RoamingChargingProfile> for RoamingChargingProfile {
		fn from(value: &RoamingChargingProfile) -> Self {
			value.clone()
		}
	}

	/// At least one of the "ipv4Addr" attribute and the "ipv6Addr" attribute
	/// shall be included in the "RouteInformation" data type.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "At least one of the \"ipv4Addr\" attribute and the
	/// \"ipv6Addr\" attribute shall be included in the \"RouteInformation\"
	/// data type. \n",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "portNumber"
	///  ],
	///  "properties": {
	///    "ipv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "portNumber": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RouteInformation(pub Option<RouteInformationInner>);

	impl ::std::ops::Deref for RouteInformation {
		type Target = Option<RouteInformationInner>;
		fn deref(&self) -> &Option<RouteInformationInner> {
			&self.0
		}
	}

	impl From<RouteInformation> for Option<RouteInformationInner> {
		fn from(value: RouteInformation) -> Self {
			value.0
		}
	}

	impl From<&RouteInformation> for RouteInformation {
		fn from(value: &RouteInformation) -> Self {
			value.clone()
		}
	}

	impl From<Option<RouteInformationInner>> for RouteInformation {
		fn from(value: Option<RouteInformationInner>) -> Self {
			Self(value)
		}
	}

	/// At least one of the "ipv4Addr" attribute and the "ipv6Addr" attribute
	/// shall be included in the "RouteInformation" data type.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "At least one of the \"ipv4Addr\" attribute and the
	/// \"ipv6Addr\" attribute shall be included in the \"RouteInformation\"
	/// data type. \n",
	///  "type": "object",
	///  "required": [
	///    "portNumber"
	///  ],
	///  "properties": {
	///    "ipv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "portNumber": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RouteInformationInner {
		#[serde(rename = "ipv4Addr", default, skip_serializing_if = "Option::is_none")]
		pub ipv4_addr: Option<Ipv4Addr>,
		#[serde(rename = "ipv6Addr", default, skip_serializing_if = "Option::is_none")]
		pub ipv6_addr: Option<Ipv6Addr>,
		#[serde(rename = "portNumber")]
		pub port_number: Uinteger,
	}

	impl From<&RouteInformationInner> for RouteInformationInner {
		fn from(value: &RouteInformationInner) -> Self {
			value.clone()
		}
	}

	/// At least one of the "routeInfo" attribute and the "routeProfId"
	/// attribute shall be included in the "RouteToLocation" data type.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "At least one of the \"routeInfo\" attribute and the
	/// \"routeProfId\" attribute shall be included in the \"RouteToLocation\"
	/// data type.\n",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "anyOf": [
	///    {
	///      "required": [
	///        "routeInfo"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "routeProfId"
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "dnai"
	///  ],
	///  "properties": {
	///    "dnai": {
	///      "$ref": "#/components/schemas/Dnai"
	///    },
	///    "routeInfo": {
	///      "$ref": "#/components/schemas/RouteInformation"
	///    },
	///    "routeProfId": {
	///      "description": "Identifies the routing profile Id.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RouteToLocation(pub Option<RouteToLocationInner>);

	impl ::std::ops::Deref for RouteToLocation {
		type Target = Option<RouteToLocationInner>;
		fn deref(&self) -> &Option<RouteToLocationInner> {
			&self.0
		}
	}

	impl From<RouteToLocation> for Option<RouteToLocationInner> {
		fn from(value: RouteToLocation) -> Self {
			value.0
		}
	}

	impl From<&RouteToLocation> for RouteToLocation {
		fn from(value: &RouteToLocation) -> Self {
			value.clone()
		}
	}

	impl From<Option<RouteToLocationInner>> for RouteToLocation {
		fn from(value: Option<RouteToLocationInner>) -> Self {
			Self(value)
		}
	}

	/// At least one of the "routeInfo" attribute and the "routeProfId"
	/// attribute shall be included in the "RouteToLocation" data type.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "At least one of the \"routeInfo\" attribute and the
	/// \"routeProfId\" attribute shall be included in the \"RouteToLocation\"
	/// data type.\n",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "routeInfo"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "routeProfId"
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "dnai"
	///  ],
	///  "properties": {
	///    "dnai": {
	///      "$ref": "#/components/schemas/Dnai"
	///    },
	///    "routeInfo": {
	///      "$ref": "#/components/schemas/RouteInformation"
	///    },
	///    "routeProfId": {
	///      "description": "Identifies the routing profile Id.",
	///      "type": [
	///        "string",
	///        "null"
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum RouteToLocationInner {
		#[default]
		Variant0 {
			dnai: Dnai,
			#[serde(rename = "routeInfo")]
			route_info: RouteInformation,
		},
		Variant1 {
			dnai: Dnai,
			/// Identifies the routing profile Id.
			#[serde(rename = "routeProfId")]
			route_prof_id: Option<String>,
		},
	}

	impl From<&RouteToLocationInner> for RouteToLocationInner {
		fn from(value: &RouteToLocationInner) -> Self {
			value.clone()
		}
	}

	/// Contains a Routing Area Identification as defined in 3GPP TS 23.003,
	/// clause 4.2.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a Routing Area Identification as defined in
	/// 3GPP TS 23.003, clause 4.2.",
	///  "type": "object",
	///  "required": [
	///    "lac",
	///    "plmnId",
	///    "rac"
	///  ],
	///  "properties": {
	///    "lac": {
	///      "description": "Location Area Code",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{4}$"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "rac": {
	///      "description": "Routing Area Code",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{2}$"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RoutingAreaId {
		/// Location Area Code
		pub lac: RoutingAreaIdLac,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		/// Routing Area Code
		pub rac: RoutingAreaIdRac,
	}

	impl From<&RoutingAreaId> for RoutingAreaId {
		fn from(value: &RoutingAreaId) -> Self {
			value.clone()
		}
	}

	/// Location Area Code
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Location Area Code",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{4}$"
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
	)]
	pub struct RoutingAreaIdLac(String);

	impl ::std::ops::Deref for RoutingAreaIdLac {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<RoutingAreaIdLac> for String {
		fn from(value: RoutingAreaIdLac) -> Self {
			value.0
		}
	}

	impl From<&RoutingAreaIdLac> for RoutingAreaIdLac {
		fn from(value: &RoutingAreaIdLac) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for RoutingAreaIdLac {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{4}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{4}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for RoutingAreaIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for RoutingAreaIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for RoutingAreaIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for RoutingAreaIdLac {
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

	/// Routing Area Code
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Routing Area Code",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{2}$"
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
	)]
	pub struct RoutingAreaIdRac(String);

	impl ::std::ops::Deref for RoutingAreaIdRac {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<RoutingAreaIdRac> for String {
		fn from(value: RoutingAreaIdRac) -> Self {
			value.0
		}
	}

	impl From<&RoutingAreaIdRac> for RoutingAreaIdRac {
		fn from(value: &RoutingAreaIdRac) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for RoutingAreaIdRac {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{2}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{2}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for RoutingAreaIdRac {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for RoutingAreaIdRac {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for RoutingAreaIdRac {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for RoutingAreaIdRac {
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

	/// Redundancy Sequence Number. Possible values are
	///  - V1
	///  - V2
	///  - NONE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Redundancy Sequence Number. Possible values are\n  -
	/// V1\n  - V2\n  - NONE\n",
	///  "type": "string",
	///  "enum": [
	///    "V1",
	///    "V2",
	///    "NONE"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum Rsn {
		#[default]
		V1,
		V2,
		#[serde(rename = "NONE")]
		None,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&Rsn> for Rsn {
		fn from(value: &Rsn) -> Self {
			value.clone()
		}
	}

	impl ToString for Rsn {
		fn to_string(&self) -> String {
			match *self {
				Self::V1 => "V1".to_string(),
				Self::V2 => "V2".to_string(),
				Self::None => "NONE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for Rsn {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"V1" => Ok(Self::V1),
				"V2" => Ok(Self::V2),
				"NONE" => Ok(Self::None),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for Rsn {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Rsn {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Rsn {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Unsigned integer indicating Sampling Ratio (see clauses 4.15.1 of 3GPP
	/// TS 23.502), expressed in percent.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer indicating Sampling Ratio (see clauses
	/// 4.15.1 of 3GPP TS 23.502), expressed in percent. \n",
	///  "type": "integer",
	///  "maximum": 100.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SamplingRatio(pub i64);

	impl ::std::ops::Deref for SamplingRatio {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<SamplingRatio> for i64 {
		fn from(value: SamplingRatio) -> Self {
			value.0
		}
	}

	impl From<&SamplingRatio> for SamplingRatio {
		fn from(value: &SamplingRatio) -> Self {
			value.clone()
		}
	}

	impl From<i64> for SamplingRatio {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SamplingRatio {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SamplingRatio {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SamplingRatio {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SamplingRatio {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SamplingRatio {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Indicates the satellite backhaul used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the satellite backhaul used.",
	///  "type": "string",
	///  "enum": [
	///    "GEO",
	///    "MEO",
	///    "LEO",
	///    "OTHER_SAT",
	///    "NON_SATELLITE"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum SatelliteBackhaulCategory {
		#[default]
		#[serde(rename = "GEO")]
		Geo,
		#[serde(rename = "MEO")]
		Meo,
		#[serde(rename = "LEO")]
		Leo,
		#[serde(rename = "OTHER_SAT")]
		OtherSat,
		#[serde(rename = "NON_SATELLITE")]
		NonSatellite,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SatelliteBackhaulCategory> for SatelliteBackhaulCategory {
		fn from(value: &SatelliteBackhaulCategory) -> Self {
			value.clone()
		}
	}

	impl ToString for SatelliteBackhaulCategory {
		fn to_string(&self) -> String {
			match *self {
				Self::Geo => "GEO".to_string(),
				Self::Meo => "MEO".to_string(),
				Self::Leo => "LEO".to_string(),
				Self::OtherSat => "OTHER_SAT".to_string(),
				Self::NonSatellite => "NON_SATELLITE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SatelliteBackhaulCategory {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"GEO" => Ok(Self::Geo),
				"MEO" => Ok(Self::Meo),
				"LEO" => Ok(Self::Leo),
				"OTHER_SAT" => Ok(Self::OtherSat),
				"NON_SATELLITE" => Ok(Self::NonSatellite),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SatelliteBackhaulCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SatelliteBackhaulCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SatelliteBackhaulCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SBI Binding Level
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "SBI Binding Level",
	///  "type": "string",
	///  "enum": [
	///    "NF_INSTANCE_BINDING",
	///    "NF_SET_BINDING",
	///    "NF_SERVICE_SET_BINDING",
	///    "NF_SERVICE_INSTANCE_BINDING"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum SbiBindingLevel {
		#[default]
		#[serde(rename = "NF_INSTANCE_BINDING")]
		NfInstanceBinding,
		#[serde(rename = "NF_SET_BINDING")]
		NfSetBinding,
		#[serde(rename = "NF_SERVICE_SET_BINDING")]
		NfServiceSetBinding,
		#[serde(rename = "NF_SERVICE_INSTANCE_BINDING")]
		NfServiceInstanceBinding,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SbiBindingLevel> for SbiBindingLevel {
		fn from(value: &SbiBindingLevel) -> Self {
			value.clone()
		}
	}

	impl ToString for SbiBindingLevel {
		fn to_string(&self) -> String {
			match *self {
				Self::NfInstanceBinding => "NF_INSTANCE_BINDING".to_string(),
				Self::NfSetBinding => "NF_SET_BINDING".to_string(),
				Self::NfServiceSetBinding => "NF_SERVICE_SET_BINDING".to_string(),
				Self::NfServiceInstanceBinding => "NF_SERVICE_INSTANCE_BINDING".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SbiBindingLevel {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NF_INSTANCE_BINDING" => Ok(Self::NfInstanceBinding),
				"NF_SET_BINDING" => Ok(Self::NfSetBinding),
				"NF_SERVICE_SET_BINDING" => Ok(Self::NfServiceSetBinding),
				"NF_SERVICE_INSTANCE_BINDING" => Ok(Self::NfServiceInstanceBinding),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SbiBindingLevel {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SbiBindingLevel {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SbiBindingLevel {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Identifies time and day of the week when the UE is available for
	/// communication.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies time and day of the week when the UE is
	/// available for communication.",
	///  "type": "object",
	///  "properties": {
	///    "daysOfWeek": {
	///      "description": "Identifies the day(s) of the week. If absent, it
	/// indicates every day of the week.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DayOfWeek"
	///      },
	///      "maxItems": 6,
	///      "minItems": 1
	///    },
	///    "timeOfDayEnd": {
	///      "$ref": "#/components/schemas/TimeOfDay"
	///    },
	///    "timeOfDayStart": {
	///      "$ref": "#/components/schemas/TimeOfDay"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ScheduledCommunicationTime {
		/// Identifies the day(s) of the week. If absent, it indicates every day
		/// of the week.
		#[serde(rename = "daysOfWeek", default, skip_serializing_if = "Vec::is_empty")]
		pub days_of_week: Vec<DayOfWeek>,
		#[serde(
			rename = "timeOfDayEnd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_of_day_end: Option<TimeOfDay>,
		#[serde(
			rename = "timeOfDayStart",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_of_day_start: Option<TimeOfDay>,
	}

	impl From<&ScheduledCommunicationTime> for ScheduledCommunicationTime {
		fn from(value: &ScheduledCommunicationTime) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// -DOWNLINK_ONLY: Downlink only
	/// -UPLINK_ONLY: Uplink only
	/// -BIDIRECTIONA: Bi-directional
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n-DOWNLINK_ONLY: Downlink
	/// only\n-UPLINK_ONLY: Uplink only\n-BIDIRECTIONA: Bi-directional\n",
	///  "type": "string",
	///  "enum": [
	///    "DOWNLINK_ONLY",
	///    "UPLINK_ONLY",
	///    "BIDIRECTIONAL"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum ScheduledCommunicationType {
		#[default]
		#[serde(rename = "DOWNLINK_ONLY")]
		DownlinkOnly,
		#[serde(rename = "UPLINK_ONLY")]
		UplinkOnly,
		#[serde(rename = "BIDIRECTIONAL")]
		Bidirectional,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ScheduledCommunicationType> for ScheduledCommunicationType {
		fn from(value: &ScheduledCommunicationType) -> Self {
			value.clone()
		}
	}

	impl ToString for ScheduledCommunicationType {
		fn to_string(&self) -> String {
			match *self {
				Self::DownlinkOnly => "DOWNLINK_ONLY".to_string(),
				Self::UplinkOnly => "UPLINK_ONLY".to_string(),
				Self::Bidirectional => "BIDIRECTIONAL".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ScheduledCommunicationType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DOWNLINK_ONLY" => Ok(Self::DownlinkOnly),
				"UPLINK_ONLY" => Ok(Self::UplinkOnly),
				"BIDIRECTIONAL" => Ok(Self::Bidirectional),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ScheduledCommunicationType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ScheduledCommunicationType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ScheduledCommunicationType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// string with format "date-time" as defined in OpenAPI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format \"date-time\" as defined in
	/// OpenAPI.",
	///  "type": "string",
	///  "format": "date-time"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDateTime(pub chrono::DateTime<chrono::offset::Utc>);

	impl ::std::ops::Deref for SchemasDateTime {
		type Target = chrono::DateTime<chrono::offset::Utc>;
		fn deref(&self) -> &chrono::DateTime<chrono::offset::Utc> {
			&self.0
		}
	}

	impl From<SchemasDateTime> for chrono::DateTime<chrono::offset::Utc> {
		fn from(value: SchemasDateTime) -> Self {
			value.0
		}
	}

	impl From<&SchemasDateTime> for SchemasDateTime {
		fn from(value: &SchemasDateTime) -> Self {
			value.clone()
		}
	}

	impl From<chrono::DateTime<chrono::offset::Utc>> for SchemasDateTime {
		fn from(value: chrono::DateTime<chrono::offset::Utc>) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SchemasDateTime {
		type Err = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SchemasDateTime {
		type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SchemasDateTime {
		type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SchemasDateTime {
		type Error = <chrono::DateTime<chrono::offset::Utc> as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SchemasDateTime {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Secondary Rat Usage Data Report Container
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Secondary Rat Usage Data Report Container",
	///  "type": "string",
	///  "format": "byte"
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
	pub struct SecondaryRatUsageDataReportContainer(pub String);

	impl ::std::ops::Deref for SecondaryRatUsageDataReportContainer {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SecondaryRatUsageDataReportContainer> for String {
		fn from(value: SecondaryRatUsageDataReportContainer) -> Self {
			value.0
		}
	}

	impl From<&SecondaryRatUsageDataReportContainer> for SecondaryRatUsageDataReportContainer {
		fn from(value: &SecondaryRatUsageDataReportContainer) -> Self {
			value.clone()
		}
	}

	impl From<String> for SecondaryRatUsageDataReportContainer {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SecondaryRatUsageDataReportContainer {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for SecondaryRatUsageDataReportContainer {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Secondary RAT Usage Information to report usage data for a secondary RAT
	/// for QoS flows and/or the whole PDU session.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Secondary RAT Usage Information to report usage data
	/// for a secondary RAT for QoS flows and/or the whole PDU session.\n",
	///  "type": "object",
	///  "required": [
	///    "secondaryRatType"
	///  ],
	///  "properties": {
	///    "pduSessionUsageData": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/VolumeTimedReport"
	///      },
	///      "minItems": 1
	///    },
	///    "qosFlowsUsageData": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowUsageReport"
	///      },
	///      "minItems": 1
	///    },
	///    "secondaryRatType": {
	///      "$ref": "#/components/schemas/RatType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SecondaryRatUsageInfo {
		#[serde(
			rename = "pduSessionUsageData",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pdu_session_usage_data: Vec<VolumeTimedReport>,
		#[serde(
			rename = "qosFlowsUsageData",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flows_usage_data: Vec<QosFlowUsageReport>,
		#[serde(rename = "secondaryRatType")]
		pub secondary_rat_type: RatType,
	}

	impl From<&SecondaryRatUsageInfo> for SecondaryRatUsageInfo {
		fn from(value: &SecondaryRatUsageInfo) -> Self {
			value.clone()
		}
	}

	/// Secondary RAT Usage Report to report usage data for a secondary RAT for
	/// QoS flows.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Secondary RAT Usage Report to report usage data for a
	/// secondary RAT for QoS flows.",
	///  "type": "object",
	///  "required": [
	///    "qosFlowsUsageData",
	///    "secondaryRatType"
	///  ],
	///  "properties": {
	///    "qosFlowsUsageData": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowUsageReport"
	///      },
	///      "minItems": 1
	///    },
	///    "secondaryRatType": {
	///      "$ref": "#/components/schemas/RatType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SecondaryRatUsageReport {
		#[serde(rename = "qosFlowsUsageData")]
		pub qos_flows_usage_data: Vec<QosFlowUsageReport>,
		#[serde(rename = "secondaryRatType")]
		pub secondary_rat_type: RatType,
	}

	impl From<&SecondaryRatUsageReport> for SecondaryRatUsageReport {
		fn from(value: &SecondaryRatUsageReport) -> Self {
			value.clone()
		}
	}

	/// Security Result
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Security Result",
	///  "type": "object",
	///  "properties": {
	///    "confidentialityProtectionResult": {
	///      "$ref": "#/components/schemas/ProtectionResult"
	///    },
	///    "integrityProtectionResult": {
	///      "$ref": "#/components/schemas/ProtectionResult"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SecurityResult {
		#[serde(
			rename = "confidentialityProtectionResult",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub confidentiality_protection_result: Option<ProtectionResult>,
		#[serde(
			rename = "integrityProtectionResult",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub integrity_protection_result: Option<ProtectionResult>,
	}

	impl From<&SecurityResult> for SecurityResult {
		fn from(value: &SecurityResult) -> Self {
			value.clone()
		}
	}

	/// Data within Send MO Data Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Send MO Data Request",
	///  "type": "object",
	///  "required": [
	///    "moData"
	///  ],
	///  "properties": {
	///    "moData": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "moExpDataCounter": {
	///      "$ref": "#/components/schemas/MoExpDataCounter"
	///    },
	///    "ueLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SendMoDataReqData {
		#[serde(rename = "moData")]
		pub mo_data: RefToBinaryData,
		#[serde(
			rename = "moExpDataCounter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mo_exp_data_counter: Option<MoExpDataCounter>,
		#[serde(
			rename = "ueLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_location: Option<UserLocation>,
	}

	impl From<&SendMoDataReqData> for SendMoDataReqData {
		fn from(value: &SendMoDataReqData) -> Self {
			value.clone()
		}
	}

	/// Contains addressing information (IP addresses and/or FQDNs) of a server.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains addressing information (IP addresses and/or
	/// FQDNs) of a server.",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "ipv4Addresses"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipv6Addresses"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "fqdnList"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "fqdnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Fqdn"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv4Addresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6Addresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum ServerAddressingInfo {
		#[default]
		Variant0 {
			#[serde(rename = "ipv4Addresses")]
			ipv4_addresses: Vec<Ipv4Addr>,
		},
		Variant1 {
			#[serde(rename = "ipv6Addresses")]
			ipv6_addresses: Vec<Ipv6Addr>,
		},
		Variant2 {
			#[serde(rename = "fqdnList")]
			fqdn_list: Vec<Fqdn>,
		},
	}

	impl From<&ServerAddressingInfo> for ServerAddressingInfo {
		fn from(value: &ServerAddressingInfo) -> Self {
			value.clone()
		}
	}

	/// Contains a Service Area Identifier as defined in 3GPP TS 23.003, clause
	/// 12.5.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a Service Area Identifier as defined in 3GPP
	/// TS 23.003, clause 12.5.",
	///  "type": "object",
	///  "required": [
	///    "lac",
	///    "plmnId",
	///    "sac"
	///  ],
	///  "properties": {
	///    "lac": {
	///      "description": "Location Area Code.",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{4}$"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "sac": {
	///      "description": "Service Area Code.",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{4}$"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ServiceAreaId {
		/// Location Area Code.
		pub lac: ServiceAreaIdLac,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		/// Service Area Code.
		pub sac: ServiceAreaIdSac,
	}

	impl From<&ServiceAreaId> for ServiceAreaId {
		fn from(value: &ServiceAreaId) -> Self {
			value.clone()
		}
	}

	/// Location Area Code.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Location Area Code.",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{4}$"
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
	)]
	pub struct ServiceAreaIdLac(String);

	impl ::std::ops::Deref for ServiceAreaIdLac {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ServiceAreaIdLac> for String {
		fn from(value: ServiceAreaIdLac) -> Self {
			value.0
		}
	}

	impl From<&ServiceAreaIdLac> for ServiceAreaIdLac {
		fn from(value: &ServiceAreaIdLac) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ServiceAreaIdLac {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{4}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{4}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for ServiceAreaIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ServiceAreaIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ServiceAreaIdLac {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ServiceAreaIdLac {
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

	/// Service Area Code.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Service Area Code.",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{4}$"
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
	)]
	pub struct ServiceAreaIdSac(String);

	impl ::std::ops::Deref for ServiceAreaIdSac {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ServiceAreaIdSac> for String {
		fn from(value: ServiceAreaIdSac) -> Self {
			value.0
		}
	}

	impl From<&ServiceAreaIdSac> for ServiceAreaIdSac {
		fn from(value: &ServiceAreaIdSac) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ServiceAreaIdSac {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{4}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{4}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for ServiceAreaIdSac {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ServiceAreaIdSac {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ServiceAreaIdSac {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ServiceAreaIdSac {
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

	/// Service names known to NRF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Service names known to NRF",
	///  "type": "string",
	///  "enum": [
	///    "nnrf-nfm",
	///    "nnrf-disc",
	///    "nnrf-oauth2",
	///    "nudm-sdm",
	///    "nudm-uecm",
	///    "nudm-ueau",
	///    "nudm-ee",
	///    "nudm-pp",
	///    "nudm-niddau",
	///    "nudm-mt",
	///    "nudm-ssau",
	///    "nudm-rsds",
	///    "nudm-ueid",
	///    "namf-comm",
	///    "namf-evts",
	///    "namf-mt",
	///    "namf-loc",
	///    "namf-mbs-comm",
	///    "namf-mbs-bc",
	///    "nsmf-pdusession",
	///    "nsmf-event-exposure",
	///    "nsmf-nidd",
	///    "nausf-auth",
	///    "nausf-sorprotection",
	///    "nausf-upuprotection",
	///    "nnef-pfdmanagement",
	///    "nnef-smcontext",
	///    "nnef-eventexposure",
	///    "nnef-eas-deployment",
	///    "3gpp-cp-parameter-provisioning",
	///    "3gpp-device-triggering",
	///    "3gpp-bdt",
	///    "3gpp-traffic-influence",
	///    "3gpp-chargeable-party",
	///    "3gpp-as-session-with-qos",
	///    "3gpp-msisdn-less-mo-sms",
	///    "3gpp-service-parameter",
	///    "3gpp-monitoring-event",
	///    "3gpp-nidd-configuration-trigger",
	///    "3gpp-nidd",
	///    "3gpp-analyticsexposure",
	///    "3gpp-racs-parameter-provisioning",
	///    "3gpp-ecr-control",
	///    "3gpp-applying-bdt-policy",
	///    "3gpp-mo-lcs-notify",
	///    "3gpp-time-sync",
	///    "3gpp-am-influence",
	///    "3gpp-am-policyauthorization",
	///    "3gpp-akma",
	///    "3gpp-eas-deployment",
	///    "3gpp-iptvconfiguration",
	///    "3gpp-mbs-tmgi",
	///    "3gpp-mbs-session",
	///    "3gpp-authentication",
	///    "3gpp-asti",
	///    "npcf-am-policy-control",
	///    "npcf-smpolicycontrol",
	///    "npcf-policyauthorization",
	///    "npcf-bdtpolicycontrol",
	///    "npcf-eventexposure",
	///    "npcf-ue-policy-control",
	///    "npcf-am-policyauthorization",
	///    "npcf-mbspolicycontrol",
	///    "npcf-mbspolicyauth",
	///    "nsmsf-sms",
	///    "nnssf-nsselection",
	///    "nnssf-nssaiavailability",
	///    "nudr-dr",
	///    "nudr-group-id-map",
	///    "nlmf-loc",
	///    "n5g-eir-eic",
	///    "nbsf-management",
	///    "nchf-spendinglimitcontrol",
	///    "nchf-convergedcharging",
	///    "nchf-offlineonlycharging",
	///    "nnwdaf-eventssubscription",
	///    "nnwdaf-analyticsinfo",
	///    "nnwdaf-datamanagement",
	///    "nnwdaf-mlmodelprovision",
	///    "ngmlc-loc",
	///    "nucmf-provisioning",
	///    "nucmf-uecapabilitymanagement",
	///    "nhss-sdm",
	///    "nhss-uecm",
	///    "nhss-ueau",
	///    "nhss-ee",
	///    "nhss-ims-sdm",
	///    "nhss-ims-uecm",
	///    "nhss-ims-ueau",
	///    "nhss-gba-sdm",
	///    "nhss-gba-ueau",
	///    "nsepp-telescopic",
	///    "nsoraf-sor",
	///    "nspaf-secured-packet",
	///    "nudsf-dr",
	///    "nudsf-timer",
	///    "nnssaaf-nssaa",
	///    "nnssaaf-aiw",
	///    "naanf-akma",
	///    "n5gddnmf-discovery",
	///    "nmfaf-3dadatamanagement",
	///    "nmfaf-3cadatamanagement",
	///    "neasdf-dnscontext",
	///    "neasdf-baselinednspattern",
	///    "ndccf-datamanagement",
	///    "ndccf-contextmanagement",
	///    "nnsacf-nsac",
	///    "nnsacf-slice-ee",
	///    "nmbsmf-tmgi",
	///    "nmbsmf-mbssession",
	///    "nadrf-datamanagement",
	///    "nbsp-gba",
	///    "ntsctsf-time-sync",
	///    "ntsctsf-qos-tscai",
	///    "ntsctsf-asti",
	///    "npkmf-keyreq",
	///    "npkmf-userid",
	///    "npkmf-discovery",
	///    "nmnpf-npstatus",
	///    "niwmsc-smservice",
	///    "nmbsf-mbs-us",
	///    "nmbsf-mbs-ud-ingest",
	///    "nmbstf-distsession",
	///    "npanf-prosekey",
	///    "npanf-userid"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum ServiceName {
		#[default]
		#[serde(rename = "nnrf-nfm")]
		NnrfNfm,
		#[serde(rename = "nnrf-disc")]
		NnrfDisc,
		#[serde(rename = "nnrf-oauth2")]
		NnrfOauth2,
		#[serde(rename = "nudm-sdm")]
		NudmSdm,
		#[serde(rename = "nudm-uecm")]
		NudmUecm,
		#[serde(rename = "nudm-ueau")]
		NudmUeau,
		#[serde(rename = "nudm-ee")]
		NudmEe,
		#[serde(rename = "nudm-pp")]
		NudmPp,
		#[serde(rename = "nudm-niddau")]
		NudmNiddau,
		#[serde(rename = "nudm-mt")]
		NudmMt,
		#[serde(rename = "nudm-ssau")]
		NudmSsau,
		#[serde(rename = "nudm-rsds")]
		NudmRsds,
		#[serde(rename = "nudm-ueid")]
		NudmUeid,
		#[serde(rename = "namf-comm")]
		NamfComm,
		#[serde(rename = "namf-evts")]
		NamfEvts,
		#[serde(rename = "namf-mt")]
		NamfMt,
		#[serde(rename = "namf-loc")]
		NamfLoc,
		#[serde(rename = "namf-mbs-comm")]
		NamfMbsComm,
		#[serde(rename = "namf-mbs-bc")]
		NamfMbsBc,
		#[serde(rename = "nsmf-pdusession")]
		NsmfPdusession,
		#[serde(rename = "nsmf-event-exposure")]
		NsmfEventExposure,
		#[serde(rename = "nsmf-nidd")]
		NsmfNidd,
		#[serde(rename = "nausf-auth")]
		NausfAuth,
		#[serde(rename = "nausf-sorprotection")]
		NausfSorprotection,
		#[serde(rename = "nausf-upuprotection")]
		NausfUpuprotection,
		#[serde(rename = "nnef-pfdmanagement")]
		NnefPfdmanagement,
		#[serde(rename = "nnef-smcontext")]
		NnefSmcontext,
		#[serde(rename = "nnef-eventexposure")]
		NnefEventexposure,
		#[serde(rename = "nnef-eas-deployment")]
		NnefEasDeployment,
		#[serde(rename = "3gpp-cp-parameter-provisioning")]
		ThreeGppCpParameterProvisioning,
		#[serde(rename = "3gpp-device-triggering")]
		ThreeGppDeviceTriggering,
		#[serde(rename = "3gpp-bdt")]
		ThreeGppBdt,
		#[serde(rename = "3gpp-traffic-influence")]
		ThreeGppTrafficInfluence,
		#[serde(rename = "3gpp-chargeable-party")]
		ThreeGppChargeableParty,
		#[serde(rename = "3gpp-as-session-with-qos")]
		ThreeGppAsSessionWithQos,
		#[serde(rename = "3gpp-msisdn-less-mo-sms")]
		ThreeGppMsisdnLessMoSms,
		#[serde(rename = "3gpp-service-parameter")]
		ThreeGppServiceParameter,
		#[serde(rename = "3gpp-monitoring-event")]
		ThreeGppMonitoringEvent,
		#[serde(rename = "3gpp-nidd-configuration-trigger")]
		ThreeGppNiddConfigurationTrigger,
		#[serde(rename = "3gpp-nidd")]
		ThreeGppNidd,
		#[serde(rename = "3gpp-analyticsexposure")]
		ThreeGppAnalyticsexposure,
		#[serde(rename = "3gpp-racs-parameter-provisioning")]
		ThreeGppRacsParameterProvisioning,
		#[serde(rename = "3gpp-ecr-control")]
		ThreeGppEcrControl,
		#[serde(rename = "3gpp-applying-bdt-policy")]
		ThreeGppApplyingBdtPolicy,
		#[serde(rename = "3gpp-mo-lcs-notify")]
		ThreeGppMoLcsNotify,
		#[serde(rename = "3gpp-time-sync")]
		ThreeGppTimeSync,
		#[serde(rename = "3gpp-am-influence")]
		ThreeGppAmInfluence,
		#[serde(rename = "3gpp-am-policyauthorization")]
		ThreeGppAmPolicyauthorization,
		#[serde(rename = "3gpp-akma")]
		ThreeGppAkma,
		#[serde(rename = "3gpp-eas-deployment")]
		ThreeGppEasDeployment,
		#[serde(rename = "3gpp-iptvconfiguration")]
		ThreeGppIptvconfiguration,
		#[serde(rename = "3gpp-mbs-tmgi")]
		ThreeGppMbsTmgi,
		#[serde(rename = "3gpp-mbs-session")]
		ThreeGppMbsSession,
		#[serde(rename = "3gpp-authentication")]
		ThreeGppAuthentication,
		#[serde(rename = "3gpp-asti")]
		ThreeGppAsti,
		#[serde(rename = "npcf-am-policy-control")]
		NpcfAmPolicyControl,
		#[serde(rename = "npcf-smpolicycontrol")]
		NpcfSmpolicycontrol,
		#[serde(rename = "npcf-policyauthorization")]
		NpcfPolicyauthorization,
		#[serde(rename = "npcf-bdtpolicycontrol")]
		NpcfBdtpolicycontrol,
		#[serde(rename = "npcf-eventexposure")]
		NpcfEventexposure,
		#[serde(rename = "npcf-ue-policy-control")]
		NpcfUePolicyControl,
		#[serde(rename = "npcf-am-policyauthorization")]
		NpcfAmPolicyauthorization,
		#[serde(rename = "npcf-mbspolicycontrol")]
		NpcfMbspolicycontrol,
		#[serde(rename = "npcf-mbspolicyauth")]
		NpcfMbspolicyauth,
		#[serde(rename = "nsmsf-sms")]
		NsmsfSms,
		#[serde(rename = "nnssf-nsselection")]
		NnssfNsselection,
		#[serde(rename = "nnssf-nssaiavailability")]
		NnssfNssaiavailability,
		#[serde(rename = "nudr-dr")]
		NudrDr,
		#[serde(rename = "nudr-group-id-map")]
		NudrGroupIdMap,
		#[serde(rename = "nlmf-loc")]
		NlmfLoc,
		#[serde(rename = "n5g-eir-eic")]
		N5gEirEic,
		#[serde(rename = "nbsf-management")]
		NbsfManagement,
		#[serde(rename = "nchf-spendinglimitcontrol")]
		NchfSpendinglimitcontrol,
		#[serde(rename = "nchf-convergedcharging")]
		NchfConvergedcharging,
		#[serde(rename = "nchf-offlineonlycharging")]
		NchfOfflineonlycharging,
		#[serde(rename = "nnwdaf-eventssubscription")]
		NnwdafEventssubscription,
		#[serde(rename = "nnwdaf-analyticsinfo")]
		NnwdafAnalyticsinfo,
		#[serde(rename = "nnwdaf-datamanagement")]
		NnwdafDatamanagement,
		#[serde(rename = "nnwdaf-mlmodelprovision")]
		NnwdafMlmodelprovision,
		#[serde(rename = "ngmlc-loc")]
		NgmlcLoc,
		#[serde(rename = "nucmf-provisioning")]
		NucmfProvisioning,
		#[serde(rename = "nucmf-uecapabilitymanagement")]
		NucmfUecapabilitymanagement,
		#[serde(rename = "nhss-sdm")]
		NhssSdm,
		#[serde(rename = "nhss-uecm")]
		NhssUecm,
		#[serde(rename = "nhss-ueau")]
		NhssUeau,
		#[serde(rename = "nhss-ee")]
		NhssEe,
		#[serde(rename = "nhss-ims-sdm")]
		NhssImsSdm,
		#[serde(rename = "nhss-ims-uecm")]
		NhssImsUecm,
		#[serde(rename = "nhss-ims-ueau")]
		NhssImsUeau,
		#[serde(rename = "nhss-gba-sdm")]
		NhssGbaSdm,
		#[serde(rename = "nhss-gba-ueau")]
		NhssGbaUeau,
		#[serde(rename = "nsepp-telescopic")]
		NseppTelescopic,
		#[serde(rename = "nsoraf-sor")]
		NsorafSor,
		#[serde(rename = "nspaf-secured-packet")]
		NspafSecuredPacket,
		#[serde(rename = "nudsf-dr")]
		NudsfDr,
		#[serde(rename = "nudsf-timer")]
		NudsfTimer,
		#[serde(rename = "nnssaaf-nssaa")]
		NnssaafNssaa,
		#[serde(rename = "nnssaaf-aiw")]
		NnssaafAiw,
		#[serde(rename = "naanf-akma")]
		NaanfAkma,
		#[serde(rename = "n5gddnmf-discovery")]
		N5gddnmfDiscovery,
		#[serde(rename = "nmfaf-3dadatamanagement")]
		Nmfaf3dadatamanagement,
		#[serde(rename = "nmfaf-3cadatamanagement")]
		Nmfaf3cadatamanagement,
		#[serde(rename = "neasdf-dnscontext")]
		NeasdfDnscontext,
		#[serde(rename = "neasdf-baselinednspattern")]
		NeasdfBaselinednspattern,
		#[serde(rename = "ndccf-datamanagement")]
		NdccfDatamanagement,
		#[serde(rename = "ndccf-contextmanagement")]
		NdccfContextmanagement,
		#[serde(rename = "nnsacf-nsac")]
		NnsacfNsac,
		#[serde(rename = "nnsacf-slice-ee")]
		NnsacfSliceEe,
		#[serde(rename = "nmbsmf-tmgi")]
		NmbsmfTmgi,
		#[serde(rename = "nmbsmf-mbssession")]
		NmbsmfMbssession,
		#[serde(rename = "nadrf-datamanagement")]
		NadrfDatamanagement,
		#[serde(rename = "nbsp-gba")]
		NbspGba,
		#[serde(rename = "ntsctsf-time-sync")]
		NtsctsfTimeSync,
		#[serde(rename = "ntsctsf-qos-tscai")]
		NtsctsfQosTscai,
		#[serde(rename = "ntsctsf-asti")]
		NtsctsfAsti,
		#[serde(rename = "npkmf-keyreq")]
		NpkmfKeyreq,
		#[serde(rename = "npkmf-userid")]
		NpkmfUserid,
		#[serde(rename = "npkmf-discovery")]
		NpkmfDiscovery,
		#[serde(rename = "nmnpf-npstatus")]
		NmnpfNpstatus,
		#[serde(rename = "niwmsc-smservice")]
		NiwmscSmservice,
		#[serde(rename = "nmbsf-mbs-us")]
		NmbsfMbsUs,
		#[serde(rename = "nmbsf-mbs-ud-ingest")]
		NmbsfMbsUdIngest,
		#[serde(rename = "nmbstf-distsession")]
		NmbstfDistsession,
		#[serde(rename = "npanf-prosekey")]
		NpanfProsekey,
		#[serde(rename = "npanf-userid")]
		NpanfUserid,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ServiceName> for ServiceName {
		fn from(value: &ServiceName) -> Self {
			value.clone()
		}
	}

	impl ToString for ServiceName {
		fn to_string(&self) -> String {
			match *self {
				Self::NnrfNfm => "nnrf-nfm".to_string(),
				Self::NnrfDisc => "nnrf-disc".to_string(),
				Self::NnrfOauth2 => "nnrf-oauth2".to_string(),
				Self::NudmSdm => "nudm-sdm".to_string(),
				Self::NudmUecm => "nudm-uecm".to_string(),
				Self::NudmUeau => "nudm-ueau".to_string(),
				Self::NudmEe => "nudm-ee".to_string(),
				Self::NudmPp => "nudm-pp".to_string(),
				Self::NudmNiddau => "nudm-niddau".to_string(),
				Self::NudmMt => "nudm-mt".to_string(),
				Self::NudmSsau => "nudm-ssau".to_string(),
				Self::NudmRsds => "nudm-rsds".to_string(),
				Self::NudmUeid => "nudm-ueid".to_string(),
				Self::NamfComm => "namf-comm".to_string(),
				Self::NamfEvts => "namf-evts".to_string(),
				Self::NamfMt => "namf-mt".to_string(),
				Self::NamfLoc => "namf-loc".to_string(),
				Self::NamfMbsComm => "namf-mbs-comm".to_string(),
				Self::NamfMbsBc => "namf-mbs-bc".to_string(),
				Self::NsmfPdusession => "nsmf-pdusession".to_string(),
				Self::NsmfEventExposure => "nsmf-event-exposure".to_string(),
				Self::NsmfNidd => "nsmf-nidd".to_string(),
				Self::NausfAuth => "nausf-auth".to_string(),
				Self::NausfSorprotection => "nausf-sorprotection".to_string(),
				Self::NausfUpuprotection => "nausf-upuprotection".to_string(),
				Self::NnefPfdmanagement => "nnef-pfdmanagement".to_string(),
				Self::NnefSmcontext => "nnef-smcontext".to_string(),
				Self::NnefEventexposure => "nnef-eventexposure".to_string(),
				Self::NnefEasDeployment => "nnef-eas-deployment".to_string(),
				Self::ThreeGppCpParameterProvisioning => {
					"3gpp-cp-parameter-provisioning".to_string()
				}
				Self::ThreeGppDeviceTriggering => "3gpp-device-triggering".to_string(),
				Self::ThreeGppBdt => "3gpp-bdt".to_string(),
				Self::ThreeGppTrafficInfluence => "3gpp-traffic-influence".to_string(),
				Self::ThreeGppChargeableParty => "3gpp-chargeable-party".to_string(),
				Self::ThreeGppAsSessionWithQos => "3gpp-as-session-with-qos".to_string(),
				Self::ThreeGppMsisdnLessMoSms => "3gpp-msisdn-less-mo-sms".to_string(),
				Self::ThreeGppServiceParameter => "3gpp-service-parameter".to_string(),
				Self::ThreeGppMonitoringEvent => "3gpp-monitoring-event".to_string(),
				Self::ThreeGppNiddConfigurationTrigger => {
					"3gpp-nidd-configuration-trigger".to_string()
				}
				Self::ThreeGppNidd => "3gpp-nidd".to_string(),
				Self::ThreeGppAnalyticsexposure => "3gpp-analyticsexposure".to_string(),
				Self::ThreeGppRacsParameterProvisioning => {
					"3gpp-racs-parameter-provisioning".to_string()
				}
				Self::ThreeGppEcrControl => "3gpp-ecr-control".to_string(),
				Self::ThreeGppApplyingBdtPolicy => "3gpp-applying-bdt-policy".to_string(),
				Self::ThreeGppMoLcsNotify => "3gpp-mo-lcs-notify".to_string(),
				Self::ThreeGppTimeSync => "3gpp-time-sync".to_string(),
				Self::ThreeGppAmInfluence => "3gpp-am-influence".to_string(),
				Self::ThreeGppAmPolicyauthorization => "3gpp-am-policyauthorization".to_string(),
				Self::ThreeGppAkma => "3gpp-akma".to_string(),
				Self::ThreeGppEasDeployment => "3gpp-eas-deployment".to_string(),
				Self::ThreeGppIptvconfiguration => "3gpp-iptvconfiguration".to_string(),
				Self::ThreeGppMbsTmgi => "3gpp-mbs-tmgi".to_string(),
				Self::ThreeGppMbsSession => "3gpp-mbs-session".to_string(),
				Self::ThreeGppAuthentication => "3gpp-authentication".to_string(),
				Self::ThreeGppAsti => "3gpp-asti".to_string(),
				Self::NpcfAmPolicyControl => "npcf-am-policy-control".to_string(),
				Self::NpcfSmpolicycontrol => "npcf-smpolicycontrol".to_string(),
				Self::NpcfPolicyauthorization => "npcf-policyauthorization".to_string(),
				Self::NpcfBdtpolicycontrol => "npcf-bdtpolicycontrol".to_string(),
				Self::NpcfEventexposure => "npcf-eventexposure".to_string(),
				Self::NpcfUePolicyControl => "npcf-ue-policy-control".to_string(),
				Self::NpcfAmPolicyauthorization => "npcf-am-policyauthorization".to_string(),
				Self::NpcfMbspolicycontrol => "npcf-mbspolicycontrol".to_string(),
				Self::NpcfMbspolicyauth => "npcf-mbspolicyauth".to_string(),
				Self::NsmsfSms => "nsmsf-sms".to_string(),
				Self::NnssfNsselection => "nnssf-nsselection".to_string(),
				Self::NnssfNssaiavailability => "nnssf-nssaiavailability".to_string(),
				Self::NudrDr => "nudr-dr".to_string(),
				Self::NudrGroupIdMap => "nudr-group-id-map".to_string(),
				Self::NlmfLoc => "nlmf-loc".to_string(),
				Self::N5gEirEic => "n5g-eir-eic".to_string(),
				Self::NbsfManagement => "nbsf-management".to_string(),
				Self::NchfSpendinglimitcontrol => "nchf-spendinglimitcontrol".to_string(),
				Self::NchfConvergedcharging => "nchf-convergedcharging".to_string(),
				Self::NchfOfflineonlycharging => "nchf-offlineonlycharging".to_string(),
				Self::NnwdafEventssubscription => "nnwdaf-eventssubscription".to_string(),
				Self::NnwdafAnalyticsinfo => "nnwdaf-analyticsinfo".to_string(),
				Self::NnwdafDatamanagement => "nnwdaf-datamanagement".to_string(),
				Self::NnwdafMlmodelprovision => "nnwdaf-mlmodelprovision".to_string(),
				Self::NgmlcLoc => "ngmlc-loc".to_string(),
				Self::NucmfProvisioning => "nucmf-provisioning".to_string(),
				Self::NucmfUecapabilitymanagement => "nucmf-uecapabilitymanagement".to_string(),
				Self::NhssSdm => "nhss-sdm".to_string(),
				Self::NhssUecm => "nhss-uecm".to_string(),
				Self::NhssUeau => "nhss-ueau".to_string(),
				Self::NhssEe => "nhss-ee".to_string(),
				Self::NhssImsSdm => "nhss-ims-sdm".to_string(),
				Self::NhssImsUecm => "nhss-ims-uecm".to_string(),
				Self::NhssImsUeau => "nhss-ims-ueau".to_string(),
				Self::NhssGbaSdm => "nhss-gba-sdm".to_string(),
				Self::NhssGbaUeau => "nhss-gba-ueau".to_string(),
				Self::NseppTelescopic => "nsepp-telescopic".to_string(),
				Self::NsorafSor => "nsoraf-sor".to_string(),
				Self::NspafSecuredPacket => "nspaf-secured-packet".to_string(),
				Self::NudsfDr => "nudsf-dr".to_string(),
				Self::NudsfTimer => "nudsf-timer".to_string(),
				Self::NnssaafNssaa => "nnssaaf-nssaa".to_string(),
				Self::NnssaafAiw => "nnssaaf-aiw".to_string(),
				Self::NaanfAkma => "naanf-akma".to_string(),
				Self::N5gddnmfDiscovery => "n5gddnmf-discovery".to_string(),
				Self::Nmfaf3dadatamanagement => "nmfaf-3dadatamanagement".to_string(),
				Self::Nmfaf3cadatamanagement => "nmfaf-3cadatamanagement".to_string(),
				Self::NeasdfDnscontext => "neasdf-dnscontext".to_string(),
				Self::NeasdfBaselinednspattern => "neasdf-baselinednspattern".to_string(),
				Self::NdccfDatamanagement => "ndccf-datamanagement".to_string(),
				Self::NdccfContextmanagement => "ndccf-contextmanagement".to_string(),
				Self::NnsacfNsac => "nnsacf-nsac".to_string(),
				Self::NnsacfSliceEe => "nnsacf-slice-ee".to_string(),
				Self::NmbsmfTmgi => "nmbsmf-tmgi".to_string(),
				Self::NmbsmfMbssession => "nmbsmf-mbssession".to_string(),
				Self::NadrfDatamanagement => "nadrf-datamanagement".to_string(),
				Self::NbspGba => "nbsp-gba".to_string(),
				Self::NtsctsfTimeSync => "ntsctsf-time-sync".to_string(),
				Self::NtsctsfQosTscai => "ntsctsf-qos-tscai".to_string(),
				Self::NtsctsfAsti => "ntsctsf-asti".to_string(),
				Self::NpkmfKeyreq => "npkmf-keyreq".to_string(),
				Self::NpkmfUserid => "npkmf-userid".to_string(),
				Self::NpkmfDiscovery => "npkmf-discovery".to_string(),
				Self::NmnpfNpstatus => "nmnpf-npstatus".to_string(),
				Self::NiwmscSmservice => "niwmsc-smservice".to_string(),
				Self::NmbsfMbsUs => "nmbsf-mbs-us".to_string(),
				Self::NmbsfMbsUdIngest => "nmbsf-mbs-ud-ingest".to_string(),
				Self::NmbstfDistsession => "nmbstf-distsession".to_string(),
				Self::NpanfProsekey => "npanf-prosekey".to_string(),
				Self::NpanfUserid => "npanf-userid".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ServiceName {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"nnrf-nfm" => Ok(Self::NnrfNfm),
				"nnrf-disc" => Ok(Self::NnrfDisc),
				"nnrf-oauth2" => Ok(Self::NnrfOauth2),
				"nudm-sdm" => Ok(Self::NudmSdm),
				"nudm-uecm" => Ok(Self::NudmUecm),
				"nudm-ueau" => Ok(Self::NudmUeau),
				"nudm-ee" => Ok(Self::NudmEe),
				"nudm-pp" => Ok(Self::NudmPp),
				"nudm-niddau" => Ok(Self::NudmNiddau),
				"nudm-mt" => Ok(Self::NudmMt),
				"nudm-ssau" => Ok(Self::NudmSsau),
				"nudm-rsds" => Ok(Self::NudmRsds),
				"nudm-ueid" => Ok(Self::NudmUeid),
				"namf-comm" => Ok(Self::NamfComm),
				"namf-evts" => Ok(Self::NamfEvts),
				"namf-mt" => Ok(Self::NamfMt),
				"namf-loc" => Ok(Self::NamfLoc),
				"namf-mbs-comm" => Ok(Self::NamfMbsComm),
				"namf-mbs-bc" => Ok(Self::NamfMbsBc),
				"nsmf-pdusession" => Ok(Self::NsmfPdusession),
				"nsmf-event-exposure" => Ok(Self::NsmfEventExposure),
				"nsmf-nidd" => Ok(Self::NsmfNidd),
				"nausf-auth" => Ok(Self::NausfAuth),
				"nausf-sorprotection" => Ok(Self::NausfSorprotection),
				"nausf-upuprotection" => Ok(Self::NausfUpuprotection),
				"nnef-pfdmanagement" => Ok(Self::NnefPfdmanagement),
				"nnef-smcontext" => Ok(Self::NnefSmcontext),
				"nnef-eventexposure" => Ok(Self::NnefEventexposure),
				"nnef-eas-deployment" => Ok(Self::NnefEasDeployment),
				"3gpp-cp-parameter-provisioning" => Ok(Self::ThreeGppCpParameterProvisioning),
				"3gpp-device-triggering" => Ok(Self::ThreeGppDeviceTriggering),
				"3gpp-bdt" => Ok(Self::ThreeGppBdt),
				"3gpp-traffic-influence" => Ok(Self::ThreeGppTrafficInfluence),
				"3gpp-chargeable-party" => Ok(Self::ThreeGppChargeableParty),
				"3gpp-as-session-with-qos" => Ok(Self::ThreeGppAsSessionWithQos),
				"3gpp-msisdn-less-mo-sms" => Ok(Self::ThreeGppMsisdnLessMoSms),
				"3gpp-service-parameter" => Ok(Self::ThreeGppServiceParameter),
				"3gpp-monitoring-event" => Ok(Self::ThreeGppMonitoringEvent),
				"3gpp-nidd-configuration-trigger" => Ok(Self::ThreeGppNiddConfigurationTrigger),
				"3gpp-nidd" => Ok(Self::ThreeGppNidd),
				"3gpp-analyticsexposure" => Ok(Self::ThreeGppAnalyticsexposure),
				"3gpp-racs-parameter-provisioning" => Ok(Self::ThreeGppRacsParameterProvisioning),
				"3gpp-ecr-control" => Ok(Self::ThreeGppEcrControl),
				"3gpp-applying-bdt-policy" => Ok(Self::ThreeGppApplyingBdtPolicy),
				"3gpp-mo-lcs-notify" => Ok(Self::ThreeGppMoLcsNotify),
				"3gpp-time-sync" => Ok(Self::ThreeGppTimeSync),
				"3gpp-am-influence" => Ok(Self::ThreeGppAmInfluence),
				"3gpp-am-policyauthorization" => Ok(Self::ThreeGppAmPolicyauthorization),
				"3gpp-akma" => Ok(Self::ThreeGppAkma),
				"3gpp-eas-deployment" => Ok(Self::ThreeGppEasDeployment),
				"3gpp-iptvconfiguration" => Ok(Self::ThreeGppIptvconfiguration),
				"3gpp-mbs-tmgi" => Ok(Self::ThreeGppMbsTmgi),
				"3gpp-mbs-session" => Ok(Self::ThreeGppMbsSession),
				"3gpp-authentication" => Ok(Self::ThreeGppAuthentication),
				"3gpp-asti" => Ok(Self::ThreeGppAsti),
				"npcf-am-policy-control" => Ok(Self::NpcfAmPolicyControl),
				"npcf-smpolicycontrol" => Ok(Self::NpcfSmpolicycontrol),
				"npcf-policyauthorization" => Ok(Self::NpcfPolicyauthorization),
				"npcf-bdtpolicycontrol" => Ok(Self::NpcfBdtpolicycontrol),
				"npcf-eventexposure" => Ok(Self::NpcfEventexposure),
				"npcf-ue-policy-control" => Ok(Self::NpcfUePolicyControl),
				"npcf-am-policyauthorization" => Ok(Self::NpcfAmPolicyauthorization),
				"npcf-mbspolicycontrol" => Ok(Self::NpcfMbspolicycontrol),
				"npcf-mbspolicyauth" => Ok(Self::NpcfMbspolicyauth),
				"nsmsf-sms" => Ok(Self::NsmsfSms),
				"nnssf-nsselection" => Ok(Self::NnssfNsselection),
				"nnssf-nssaiavailability" => Ok(Self::NnssfNssaiavailability),
				"nudr-dr" => Ok(Self::NudrDr),
				"nudr-group-id-map" => Ok(Self::NudrGroupIdMap),
				"nlmf-loc" => Ok(Self::NlmfLoc),
				"n5g-eir-eic" => Ok(Self::N5gEirEic),
				"nbsf-management" => Ok(Self::NbsfManagement),
				"nchf-spendinglimitcontrol" => Ok(Self::NchfSpendinglimitcontrol),
				"nchf-convergedcharging" => Ok(Self::NchfConvergedcharging),
				"nchf-offlineonlycharging" => Ok(Self::NchfOfflineonlycharging),
				"nnwdaf-eventssubscription" => Ok(Self::NnwdafEventssubscription),
				"nnwdaf-analyticsinfo" => Ok(Self::NnwdafAnalyticsinfo),
				"nnwdaf-datamanagement" => Ok(Self::NnwdafDatamanagement),
				"nnwdaf-mlmodelprovision" => Ok(Self::NnwdafMlmodelprovision),
				"ngmlc-loc" => Ok(Self::NgmlcLoc),
				"nucmf-provisioning" => Ok(Self::NucmfProvisioning),
				"nucmf-uecapabilitymanagement" => Ok(Self::NucmfUecapabilitymanagement),
				"nhss-sdm" => Ok(Self::NhssSdm),
				"nhss-uecm" => Ok(Self::NhssUecm),
				"nhss-ueau" => Ok(Self::NhssUeau),
				"nhss-ee" => Ok(Self::NhssEe),
				"nhss-ims-sdm" => Ok(Self::NhssImsSdm),
				"nhss-ims-uecm" => Ok(Self::NhssImsUecm),
				"nhss-ims-ueau" => Ok(Self::NhssImsUeau),
				"nhss-gba-sdm" => Ok(Self::NhssGbaSdm),
				"nhss-gba-ueau" => Ok(Self::NhssGbaUeau),
				"nsepp-telescopic" => Ok(Self::NseppTelescopic),
				"nsoraf-sor" => Ok(Self::NsorafSor),
				"nspaf-secured-packet" => Ok(Self::NspafSecuredPacket),
				"nudsf-dr" => Ok(Self::NudsfDr),
				"nudsf-timer" => Ok(Self::NudsfTimer),
				"nnssaaf-nssaa" => Ok(Self::NnssaafNssaa),
				"nnssaaf-aiw" => Ok(Self::NnssaafAiw),
				"naanf-akma" => Ok(Self::NaanfAkma),
				"n5gddnmf-discovery" => Ok(Self::N5gddnmfDiscovery),
				"nmfaf-3dadatamanagement" => Ok(Self::Nmfaf3dadatamanagement),
				"nmfaf-3cadatamanagement" => Ok(Self::Nmfaf3cadatamanagement),
				"neasdf-dnscontext" => Ok(Self::NeasdfDnscontext),
				"neasdf-baselinednspattern" => Ok(Self::NeasdfBaselinednspattern),
				"ndccf-datamanagement" => Ok(Self::NdccfDatamanagement),
				"ndccf-contextmanagement" => Ok(Self::NdccfContextmanagement),
				"nnsacf-nsac" => Ok(Self::NnsacfNsac),
				"nnsacf-slice-ee" => Ok(Self::NnsacfSliceEe),
				"nmbsmf-tmgi" => Ok(Self::NmbsmfTmgi),
				"nmbsmf-mbssession" => Ok(Self::NmbsmfMbssession),
				"nadrf-datamanagement" => Ok(Self::NadrfDatamanagement),
				"nbsp-gba" => Ok(Self::NbspGba),
				"ntsctsf-time-sync" => Ok(Self::NtsctsfTimeSync),
				"ntsctsf-qos-tscai" => Ok(Self::NtsctsfQosTscai),
				"ntsctsf-asti" => Ok(Self::NtsctsfAsti),
				"npkmf-keyreq" => Ok(Self::NpkmfKeyreq),
				"npkmf-userid" => Ok(Self::NpkmfUserid),
				"npkmf-discovery" => Ok(Self::NpkmfDiscovery),
				"nmnpf-npstatus" => Ok(Self::NmnpfNpstatus),
				"niwmsc-smservice" => Ok(Self::NiwmscSmservice),
				"nmbsf-mbs-us" => Ok(Self::NmbsfMbsUs),
				"nmbsf-mbs-ud-ingest" => Ok(Self::NmbsfMbsUdIngest),
				"nmbstf-distsession" => Ok(Self::NmbstfDistsession),
				"npanf-prosekey" => Ok(Self::NpanfProsekey),
				"npanf-userid" => Ok(Self::NpanfUserid),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ServiceName {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ServiceName {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ServiceName {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Complete SM Context
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Complete SM Context",
	///  "type": "object",
	///  "required": [
	///    "dnn",
	///    "pduSessionId",
	///    "pduSessionType",
	///    "qosFlowsList",
	///    "sNssai",
	///    "sessionAmbr"
	///  ],
	///  "properties": {
	///    "addRanTunnelInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowTunnel"
	///      },
	///      "minItems": 1
	///    },
	///    "addRedRanTunnelInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowTunnel"
	///      },
	///      "minItems": 1
	///    },
	///    "alwaysOnGranted": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "anchorSmfOauth2Required": {
	///      "type": "boolean"
	///    },
	///    "chargingId": {
	///      "type": "string",
	///      "pattern": "^(0|([1-9]{1}[0-9]{0,9}))$"
	///    },
	///    "chargingInfo": {
	///      "$ref": "#/components/schemas/ChargingInformation"
	///    },
	///    "dlsetSupportInd": {
	///      "type": "boolean"
	///    },
	///    "dnAaaAddress": {
	///      "$ref": "#/components/schemas/IpAddress"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "enablePauseCharging": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "epsBearerInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "epsPdnCnxInfo": {
	///      "$ref": "#/components/schemas/EpsPdnCnxInfo"
	///    },
	///    "forwardingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "hNwPubKeyId": {
	///      "type": "integer"
	///    },
	///    "hSmfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "hSmfServiceInstanceId": {
	///      "type": "string"
	///    },
	///    "hSmfUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "hplmnSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "interPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "intraPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "ipv6Index": {
	///      "$ref": "#/components/schemas/IpIndex"
	///    },
	///    "maxIntegrityProtectedDataRate": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "maxIntegrityProtectedDataRateDl": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "n9fscSupportInd": {
	///      "type": "boolean"
	///    },
	///    "nefExtBufSupportInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "nspuSupportInd": {
	///      "type": "boolean"
	///    },
	///    "pcfGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "pcfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "pcfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "pduSessionRef": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "pduSessionSmfBinding": {
	///      "$ref": "#/components/schemas/SbiBindingLevel"
	///    },
	///    "pduSessionSmfServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "pduSessionSmfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "pduSessionType": {
	///      "$ref": "#/components/schemas/PduSessionType"
	///    },
	///    "psaTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "qosFlowsList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowSetupItem"
	///      },
	///      "minItems": 1
	///    },
	///    "ranTunnelInfo": {
	///      "$ref": "#/components/schemas/QosFlowTunnel"
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "redRanTunnelInfo": {
	///      "$ref": "#/components/schemas/QosFlowTunnel"
	///    },
	///    "redundantPduSessionInfo": {
	///      "$ref": "#/components/schemas/RedundantPduSessionInformation"
	///    },
	///    "roamingChargingProfile": {
	///      "$ref": "#/components/schemas/RoamingChargingProfile"
	///    },
	///    "routingIndicator": {
	///      "type": "string"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "satelliteBackhaulCat": {
	///      "$ref": "#/components/schemas/SatelliteBackhaulCategory"
	///    },
	///    "selMode": {
	///      "$ref": "#/components/schemas/DnnSelectionMode"
	///    },
	///    "selectedDnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "sessionAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "smfBindingInfo": {
	///      "type": "string"
	///    },
	///    "smfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smfServiceInstanceId": {
	///      "type": "string"
	///    },
	///    "smfUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "sscMode": {
	///      "type": "string",
	///      "pattern": "^[0-7]$"
	///    },
	///    "udmGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "ueIpv4Address": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ueIpv6Prefix": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "upSecurity": {
	///      "$ref": "#/components/schemas/UpSecurity"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContext {
		#[serde(
			rename = "addRanTunnelInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub add_ran_tunnel_info: Vec<QosFlowTunnel>,
		#[serde(
			rename = "addRedRanTunnelInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub add_red_ran_tunnel_info: Vec<QosFlowTunnel>,
		#[serde(rename = "alwaysOnGranted", default)]
		pub always_on_granted: bool,
		#[serde(
			rename = "anchorSmfOauth2Required",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub anchor_smf_oauth2_required: Option<bool>,
		#[serde(
			rename = "chargingId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub charging_id: Option<SmContextChargingId>,
		#[serde(
			rename = "chargingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub charging_info: Option<ChargingInformation>,
		#[serde(
			rename = "dlsetSupportInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dlset_support_ind: Option<bool>,
		#[serde(
			rename = "dnAaaAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dn_aaa_address: Option<IpAddress>,
		pub dnn: Dnn,
		#[serde(rename = "enablePauseCharging", default)]
		pub enable_pause_charging: bool,
		#[serde(
			rename = "epsBearerInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub eps_bearer_info: Vec<EpsBearerInfo>,
		#[serde(
			rename = "epsPdnCnxInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_pdn_cnx_info: Option<EpsPdnCnxInfo>,
		#[serde(rename = "forwardingInd", default)]
		pub forwarding_ind: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(
			rename = "hNwPubKeyId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub h_nw_pub_key_id: Option<i64>,
		#[serde(
			rename = "hSmfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub h_smf_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "hSmfServiceInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub h_smf_service_instance_id: Option<String>,
		#[serde(rename = "hSmfUri", default, skip_serializing_if = "Option::is_none")]
		pub h_smf_uri: Option<Uri>,
		#[serde(
			rename = "hplmnSnssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hplmn_snssai: Option<Snssai>,
		#[serde(
			rename = "interPlmnApiRoot",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_api_root: Option<Uri>,
		#[serde(
			rename = "intraPlmnApiRoot",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub intra_plmn_api_root: Option<Uri>,
		#[serde(rename = "ipv6Index", default, skip_serializing_if = "Option::is_none")]
		pub ipv6_index: Option<IpIndex>,
		#[serde(
			rename = "maxIntegrityProtectedDataRate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_integrity_protected_data_rate: Option<MaxIntegrityProtectedDataRate>,
		#[serde(
			rename = "maxIntegrityProtectedDataRateDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_integrity_protected_data_rate_dl: Option<MaxIntegrityProtectedDataRate>,
		#[serde(
			rename = "n9fscSupportInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n9fsc_support_ind: Option<bool>,
		#[serde(rename = "nefExtBufSupportInd", default)]
		pub nef_ext_buf_support_ind: bool,
		#[serde(
			rename = "nspuSupportInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nspu_support_ind: Option<bool>,
		#[serde(
			rename = "pcfGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_group_id: Option<NfGroupId>,
		#[serde(rename = "pcfId", default, skip_serializing_if = "Option::is_none")]
		pub pcf_id: Option<NfInstanceId>,
		#[serde(rename = "pcfSetId", default, skip_serializing_if = "Option::is_none")]
		pub pcf_set_id: Option<NfSetId>,
		#[serde(rename = "pduSessionId")]
		pub pdu_session_id: PduSessionId,
		#[serde(
			rename = "pduSessionRef",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_ref: Option<Uri>,
		#[serde(
			rename = "pduSessionSmfBinding",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_smf_binding: Option<SbiBindingLevel>,
		#[serde(
			rename = "pduSessionSmfServiceSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_smf_service_set_id: Option<NfServiceSetId>,
		#[serde(
			rename = "pduSessionSmfSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_smf_set_id: Option<NfSetId>,
		#[serde(rename = "pduSessionType")]
		pub pdu_session_type: PduSessionType,
		#[serde(
			rename = "psaTunnelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub psa_tunnel_info: Option<TunnelInfo>,
		#[serde(rename = "qosFlowsList")]
		pub qos_flows_list: Vec<QosFlowSetupItem>,
		#[serde(
			rename = "ranTunnelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ran_tunnel_info: Option<QosFlowTunnel>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
		#[serde(
			rename = "redRanTunnelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub red_ran_tunnel_info: Option<QosFlowTunnel>,
		#[serde(
			rename = "redundantPduSessionInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub redundant_pdu_session_info: Option<RedundantPduSessionInformation>,
		#[serde(
			rename = "roamingChargingProfile",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub roaming_charging_profile: Option<RoamingChargingProfile>,
		#[serde(
			rename = "routingIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub routing_indicator: Option<String>,
		#[serde(rename = "sNssai")]
		pub s_nssai: Snssai,
		#[serde(
			rename = "satelliteBackhaulCat",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub satellite_backhaul_cat: Option<SatelliteBackhaulCategory>,
		#[serde(rename = "selMode", default, skip_serializing_if = "Option::is_none")]
		pub sel_mode: Option<DnnSelectionMode>,
		#[serde(
			rename = "selectedDnn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub selected_dnn: Option<Dnn>,
		#[serde(rename = "sessionAmbr")]
		pub session_ambr: Ambr,
		#[serde(
			rename = "smfBindingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_binding_info: Option<String>,
		#[serde(
			rename = "smfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "smfServiceInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_service_instance_id: Option<String>,
		#[serde(rename = "smfUri", default, skip_serializing_if = "Option::is_none")]
		pub smf_uri: Option<Uri>,
		#[serde(rename = "sscMode", default, skip_serializing_if = "Option::is_none")]
		pub ssc_mode: Option<SmContextSscMode>,
		#[serde(
			rename = "udmGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub udm_group_id: Option<NfGroupId>,
		#[serde(
			rename = "ueIpv4Address",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_ipv4_address: Option<Ipv4Addr>,
		#[serde(
			rename = "ueIpv6Prefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_ipv6_prefix: Option<Ipv6Prefix>,
		#[serde(
			rename = "upSecurity",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_security: Option<UpSecurity>,
	}

	impl From<&SmContext> for SmContext {
		fn from(value: &SmContext) -> Self {
			value.clone()
		}
	}

	/// SmContextChargingId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^(0|([1-9]{1}[0-9]{0,9}))$"
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
	)]
	pub struct SmContextChargingId(String);

	impl ::std::ops::Deref for SmContextChargingId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SmContextChargingId> for String {
		fn from(value: SmContextChargingId) -> Self {
			value.0
		}
	}

	impl From<&SmContextChargingId> for SmContextChargingId {
		fn from(value: &SmContextChargingId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SmContextChargingId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^(0|([1-9]{1}[0-9]{0,9}))$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^(0|([1-9]{1}[0-9]{0,9}))$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SmContextChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SmContextChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SmContextChargingId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SmContextChargingId {
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

	/// Data within Create SM Context Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Create SM Context Request",
	///  "type": "object",
	///  "required": [
	///    "anType",
	///    "servingNetwork",
	///    "servingNfId",
	///    "smContextStatusUri"
	///  ],
	///  "properties": {
	///    "addUeLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "additionalAnType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "additionalHsmfId": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfInstanceId"
	///      },
	///      "minItems": 1
	///    },
	///    "additionalHsmfUri": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uri"
	///      },
	///      "minItems": 1
	///    },
	///    "additionalSmfId": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfInstanceId"
	///      },
	///      "minItems": 1
	///    },
	///    "additionalSmfUri": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uri"
	///      },
	///      "minItems": 1
	///    },
	///    "anType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "anchorSmfOauth2Required": {
	///      "type": "boolean"
	///    },
	///    "apnRateStatus": {
	///      "$ref": "#/components/schemas/ApnRateStatus"
	///    },
	///    "backupAmfInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/BackupAmfInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "cpCiotEnabled": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "cpOnlyInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ddnFailureSubs": {
	///      "$ref": "#/components/schemas/DdnFailureSubs"
	///    },
	///    "directForwardingFlag": {
	///      "type": "boolean"
	///    },
	///    "disasterRoamingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "dlDataWaitingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "epsBearerCtxStatus": {
	///      "$ref": "#/components/schemas/EpsBearerContextStatus"
	///    },
	///    "epsInterworkingInd": {
	///      "$ref": "#/components/schemas/EpsInterworkingIndication"
	///    },
	///    "extendedNasSmTimerInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "hNwPubKeyId": {
	///      "type": "integer"
	///    },
	///    "hSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "hSmfUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "hoState": {
	///      "$ref": "#/components/schemas/HoState"
	///    },
	///    "hplmnSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "indirectForwardingFlag": {
	///      "type": "boolean"
	///    },
	///    "invokeNef": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "maNwUpgradeInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "maRequestInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "n1SmMsg": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfo": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfoExt1": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfoType": {
	///      "$ref": "#/components/schemas/N2SmInfoType"
	///    },
	///    "n2SmInfoTypeExt1": {
	///      "$ref": "#/components/schemas/N2SmInfoType"
	///    },
	///    "nrfAccessTokenUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfDiscoveryUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfManagementUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfOauth2Required": {
	///      "description": "Map indicating whether the NRF requires
	/// Oauth2-based authorization for accessing its services. The key of the
	/// map shall be the name of an NRF service, e.g. \"nnrf-nfm\" or
	/// \"nnrf-disc\"",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "boolean"
	///      }
	///    },
	///    "nrfUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "oldPduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "oldPduSessionRef": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "oldSmContextRef": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "oldSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "onboardingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "pcfGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "pcfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "pcfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "pcfUeCallbackInfo": {
	///      "$ref": "#/components/schemas/PcfUeCallbackInfo"
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "pduSessionsActivateList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionId"
	///      },
	///      "minItems": 1
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "presenceInLadn": {
	///      "$ref": "#/components/schemas/PresenceState"
	///    },
	///    "pvsInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServerAddressingInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "ranUnchangedInd": {
	///      "type": "boolean"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "requestType": {
	///      "$ref": "#/components/schemas/RequestType"
	///    },
	///    "routingIndicator": {
	///      "type": "string"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "samePcfSelectionInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "satelliteBackhaulCat": {
	///      "$ref": "#/components/schemas/SatelliteBackhaulCategory"
	///    },
	///    "selMode": {
	///      "$ref": "#/components/schemas/DnnSelectionMode"
	///    },
	///    "selectedDnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "serviceName": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    },
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "servingNfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smContextRef": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "smContextSmfBinding": {
	///      "$ref": "#/components/schemas/SbiBindingLevel"
	///    },
	///    "smContextSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smContextSmfOauth2Required": {
	///      "type": "boolean"
	///    },
	///    "smContextSmfPlmnId": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "smContextSmfServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "smContextSmfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "smContextStatusUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "smPolicyNotifyInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "smallDataRateStatus": {
	///      "$ref": "#/components/schemas/SmallDataRateStatus"
	///    },
	///    "smfBindingInfo": {
	///      "type": "string"
	///    },
	///    "smfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smfTransferInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "smfUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "targetDnai": {
	///      "$ref": "#/components/schemas/Dnai"
	///    },
	///    "targetId": {
	///      "$ref": "#/components/schemas/NgRanTargetId"
	///    },
	///    "tngfInfo": {
	///      "$ref": "#/components/schemas/TngfInfo"
	///    },
	///    "traceData": {
	///      "$ref": "#/components/schemas/TraceData"
	///    },
	///    "twifInfo": {
	///      "$ref": "#/components/schemas/TwifInfo"
	///    },
	///    "uavAuthenticated": {
	///      "type": "boolean"
	///    },
	///    "udmGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "ueEpsPdnConnection": {
	///      "$ref": "#/components/schemas/EpsPdnCnxContainer"
	///    },
	///    "ueLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "unauthenticatedSupi": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "upCnxState": {
	///      "$ref": "#/components/schemas/UpCnxState"
	///    },
	///    "upipSupported": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "wAgfInfo": {
	///      "$ref": "#/components/schemas/WAgfInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContextCreateData {
		#[serde(
			rename = "addUeLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_ue_location: Option<UserLocation>,
		#[serde(
			rename = "additionalAnType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_an_type: Option<AccessType>,
		#[serde(
			rename = "additionalHsmfId",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_hsmf_id: Vec<NfInstanceId>,
		#[serde(
			rename = "additionalHsmfUri",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_hsmf_uri: Vec<Uri>,
		#[serde(
			rename = "additionalSmfId",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_smf_id: Vec<NfInstanceId>,
		#[serde(
			rename = "additionalSmfUri",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_smf_uri: Vec<Uri>,
		#[serde(rename = "anType")]
		pub an_type: AccessType,
		#[serde(
			rename = "anchorSmfOauth2Required",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub anchor_smf_oauth2_required: Option<bool>,
		#[serde(
			rename = "apnRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub apn_rate_status: Option<ApnRateStatus>,
		#[serde(
			rename = "backupAmfInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub backup_amf_info: Vec<BackupAmfInfo>,
		#[serde(rename = "cpCiotEnabled", default)]
		pub cp_ciot_enabled: bool,
		#[serde(rename = "cpOnlyInd", default)]
		pub cp_only_ind: bool,
		#[serde(
			rename = "ddnFailureSubs",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ddn_failure_subs: Option<DdnFailureSubs>,
		#[serde(
			rename = "directForwardingFlag",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub direct_forwarding_flag: Option<bool>,
		#[serde(rename = "disasterRoamingInd", default)]
		pub disaster_roaming_ind: bool,
		#[serde(rename = "dlDataWaitingInd", default)]
		pub dl_data_waiting_ind: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "epsBearerCtxStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_bearer_ctx_status: Option<EpsBearerContextStatus>,
		#[serde(
			rename = "epsInterworkingInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_interworking_ind: Option<EpsInterworkingIndication>,
		#[serde(rename = "extendedNasSmTimerInd", default)]
		pub extended_nas_sm_timer_ind: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<crate::common::common_models::Guami>,
		#[serde(
			rename = "hNwPubKeyId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub h_nw_pub_key_id: Option<i64>,
		#[serde(rename = "hSmfId", default, skip_serializing_if = "Option::is_none")]
		pub h_smf_id: Option<NfInstanceId>,
		#[serde(rename = "hSmfUri", default, skip_serializing_if = "Option::is_none")]
		pub h_smf_uri: Option<Uri>,
		#[serde(rename = "hoState", default, skip_serializing_if = "Option::is_none")]
		pub ho_state: Option<HoState>,
		#[serde(
			rename = "hplmnSnssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hplmn_snssai: Option<Snssai>,
		#[serde(
			rename = "indirectForwardingFlag",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub indirect_forwarding_flag: Option<bool>,
		#[serde(rename = "invokeNef", default)]
		pub invoke_nef: bool,
		#[serde(rename = "maNwUpgradeInd", default)]
		pub ma_nw_upgrade_ind: bool,
		#[serde(rename = "maRequestInd", default)]
		pub ma_request_ind: bool,
		#[serde(rename = "n1SmMsg", default, skip_serializing_if = "Option::is_none")]
		pub n1_sm_msg: Option<RefToBinaryData>,
		#[serde(rename = "n2SmInfo", default, skip_serializing_if = "Option::is_none")]
		pub n2_sm_info: Option<RefToBinaryData>,
		#[serde(
			rename = "n2SmInfoExt1",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_sm_info_ext1: Option<RefToBinaryData>,
		#[serde(
			rename = "n2SmInfoType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_sm_info_type: Option<N2SmInfoType>,
		#[serde(
			rename = "n2SmInfoTypeExt1",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_sm_info_type_ext1: Option<N2SmInfoType>,
		#[serde(
			rename = "nrfAccessTokenUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_access_token_uri: Option<Uri>,
		#[serde(
			rename = "nrfDiscoveryUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_discovery_uri: Option<Uri>,
		#[serde(
			rename = "nrfManagementUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_management_uri: Option<Uri>,
		/// Map indicating whether the NRF requires Oauth2-based authorization
		/// for accessing its services. The key of the map shall be the name of
		/// an NRF service, e.g. "nnrf-nfm" or "nnrf-disc"
		#[serde(
			rename = "nrfOauth2Required",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub nrf_oauth2_required: ::std::collections::HashMap<String, bool>,
		#[serde(rename = "nrfUri", default, skip_serializing_if = "Option::is_none")]
		pub nrf_uri: Option<Uri>,
		#[serde(
			rename = "oldPduSessionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub old_pdu_session_id: Option<PduSessionId>,
		#[serde(
			rename = "oldPduSessionRef",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub old_pdu_session_ref: Option<Uri>,
		#[serde(
			rename = "oldSmContextRef",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub old_sm_context_ref: Option<Uri>,
		#[serde(rename = "oldSmfId", default, skip_serializing_if = "Option::is_none")]
		pub old_smf_id: Option<NfInstanceId>,
		#[serde(rename = "onboardingInd", default)]
		pub onboarding_ind: bool,
		#[serde(
			rename = "pcfGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_group_id: Option<NfGroupId>,
		#[serde(rename = "pcfId", default, skip_serializing_if = "Option::is_none")]
		pub pcf_id: Option<NfInstanceId>,
		#[serde(rename = "pcfSetId", default, skip_serializing_if = "Option::is_none")]
		pub pcf_set_id: Option<NfSetId>,
		#[serde(
			rename = "pcfUeCallbackInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_ue_callback_info: Option<PcfUeCallbackInfo>,
		#[serde(
			rename = "pduSessionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_id: Option<PduSessionId>,
		#[serde(
			rename = "pduSessionsActivateList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pdu_sessions_activate_list: Vec<PduSessionId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(
			rename = "presenceInLadn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub presence_in_ladn: Option<PresenceState>,
		#[serde(rename = "pvsInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub pvs_info: Vec<ServerAddressingInfo>,
		#[serde(
			rename = "ranUnchangedInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ran_unchanged_ind: Option<bool>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "requestType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub request_type: Option<RequestType>,
		#[serde(
			rename = "routingIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub routing_indicator: Option<String>,
		#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
		pub s_nssai: Option<Snssai>,
		#[serde(rename = "samePcfSelectionInd", default)]
		pub same_pcf_selection_ind: bool,
		#[serde(
			rename = "satelliteBackhaulCat",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub satellite_backhaul_cat: Option<SatelliteBackhaulCategory>,
		#[serde(rename = "selMode", default, skip_serializing_if = "Option::is_none")]
		pub sel_mode: Option<DnnSelectionMode>,
		#[serde(
			rename = "selectedDnn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub selected_dnn: Option<Dnn>,
		#[serde(
			rename = "serviceName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_name: Option<ServiceName>,
		#[serde(rename = "servingNetwork")]
		pub serving_network: PlmnIdNid,
		#[serde(rename = "servingNfId")]
		pub serving_nf_id: NfInstanceId,
		#[serde(
			rename = "smContextRef",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_context_ref: Option<Uri>,
		#[serde(
			rename = "smContextSmfBinding",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_context_smf_binding: Option<SbiBindingLevel>,
		#[serde(
			rename = "smContextSmfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_context_smf_id: Option<NfInstanceId>,
		#[serde(
			rename = "smContextSmfOauth2Required",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_context_smf_oauth2_required: Option<bool>,
		#[serde(
			rename = "smContextSmfPlmnId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_context_smf_plmn_id: Option<PlmnIdNid>,
		#[serde(
			rename = "smContextSmfServiceSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_context_smf_service_set_id: Option<NfServiceSetId>,
		#[serde(
			rename = "smContextSmfSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_context_smf_set_id: Option<NfSetId>,
		#[serde(rename = "smContextStatusUri")]
		pub sm_context_status_uri: Uri,
		#[serde(rename = "smPolicyNotifyInd", default)]
		pub sm_policy_notify_ind: bool,
		#[serde(
			rename = "smallDataRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub small_data_rate_status: Option<SmallDataRateStatus>,
		#[serde(
			rename = "smfBindingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_binding_info: Option<String>,
		#[serde(rename = "smfId", default, skip_serializing_if = "Option::is_none")]
		pub smf_id: Option<NfInstanceId>,
		#[serde(rename = "smfTransferInd", default)]
		pub smf_transfer_ind: bool,
		#[serde(rename = "smfUri", default, skip_serializing_if = "Option::is_none")]
		pub smf_uri: Option<Uri>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "targetDnai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_dnai: Option<Dnai>,
		#[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
		pub target_id: Option<NgRanTargetId>,
		#[serde(rename = "tngfInfo", default, skip_serializing_if = "Option::is_none")]
		pub tngf_info: Option<TngfInfo>,
		#[serde(rename = "traceData", default, skip_serializing_if = "Option::is_none")]
		pub trace_data: Option<TraceData>,
		#[serde(rename = "twifInfo", default, skip_serializing_if = "Option::is_none")]
		pub twif_info: Option<TwifInfo>,
		#[serde(
			rename = "uavAuthenticated",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uav_authenticated: Option<bool>,
		#[serde(
			rename = "udmGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub udm_group_id: Option<NfGroupId>,
		#[serde(
			rename = "ueEpsPdnConnection",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_eps_pdn_connection: Option<EpsPdnCnxContainer>,
		#[serde(
			rename = "ueLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_location: Option<UserLocation>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
		#[serde(rename = "unauthenticatedSupi", default)]
		pub unauthenticated_supi: bool,
		#[serde(
			rename = "upCnxState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_cnx_state: Option<UpCnxState>,
		#[serde(rename = "upipSupported", default)]
		pub upip_supported: bool,
		#[serde(rename = "wAgfInfo", default, skip_serializing_if = "Option::is_none")]
		pub w_agf_info: Option<WAgfInfo>,
	}

	impl From<&SmContextCreateData> for SmContextCreateData {
		fn from(value: &SmContextCreateData) -> Self {
			value.clone()
		}
	}

	/// Error within Create SM Context Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Error within Create SM Context Response",
	///  "type": "object",
	///  "required": [
	///    "error"
	///  ],
	///  "properties": {
	///    "error": {
	///      "$ref": "#/components/schemas/ExtProblemDetails"
	///    },
	///    "n1SmMsg": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfo": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfoType": {
	///      "$ref": "#/components/schemas/N2SmInfoType"
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContextCreateError {
		pub error: ExtProblemDetails,
		#[serde(rename = "n1SmMsg", default, skip_serializing_if = "Option::is_none")]
		pub n1_sm_msg: Option<RefToBinaryData>,
		#[serde(rename = "n2SmInfo", default, skip_serializing_if = "Option::is_none")]
		pub n2_sm_info: Option<RefToBinaryData>,
		#[serde(
			rename = "n2SmInfoType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_sm_info_type: Option<N2SmInfoType>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
	}

	impl From<&SmContextCreateError> for SmContextCreateError {
		fn from(value: &SmContextCreateError) -> Self {
			value.clone()
		}
	}

	/// Data within Create SM Context Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Create SM Context Response",
	///  "type": "object",
	///  "properties": {
	///    "additionalSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "allocatedEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EbiArpMapping"
	///      },
	///      "minItems": 1
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "hSmfUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "hoState": {
	///      "$ref": "#/components/schemas/HoState"
	///    },
	///    "interPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "n2SmInfo": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfoType": {
	///      "$ref": "#/components/schemas/N2SmInfoType"
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "selectedOldSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "selectedSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smfServiceInstanceId": {
	///      "type": "string"
	///    },
	///    "smfUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "upCnxState": {
	///      "$ref": "#/components/schemas/UpCnxState"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContextCreatedData {
		#[serde(
			rename = "additionalSnssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_snssai: Option<Snssai>,
		#[serde(
			rename = "allocatedEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allocated_ebi_list: Vec<EbiArpMapping>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "hSmfUri", default, skip_serializing_if = "Option::is_none")]
		pub h_smf_uri: Option<Uri>,
		#[serde(rename = "hoState", default, skip_serializing_if = "Option::is_none")]
		pub ho_state: Option<HoState>,
		#[serde(
			rename = "interPlmnApiRoot",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_api_root: Option<Uri>,
		#[serde(rename = "n2SmInfo", default, skip_serializing_if = "Option::is_none")]
		pub n2_sm_info: Option<RefToBinaryData>,
		#[serde(
			rename = "n2SmInfoType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_sm_info_type: Option<N2SmInfoType>,
		#[serde(
			rename = "pduSessionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_id: Option<PduSessionId>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
		#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
		pub s_nssai: Option<Snssai>,
		#[serde(
			rename = "selectedOldSmfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub selected_old_smf_id: Option<NfInstanceId>,
		#[serde(
			rename = "selectedSmfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub selected_smf_id: Option<NfInstanceId>,
		#[serde(
			rename = "smfServiceInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_service_instance_id: Option<String>,
		#[serde(rename = "smfUri", default, skip_serializing_if = "Option::is_none")]
		pub smf_uri: Option<Uri>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "upCnxState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_cnx_state: Option<UpCnxState>,
	}

	impl From<&SmContextCreatedData> for SmContextCreatedData {
		fn from(value: &SmContextCreatedData) -> Self {
			value.clone()
		}
	}

	/// Data within Release SM Context Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Release SM Context Request",
	///  "type": "object",
	///  "properties": {
	///    "5gMmCauseValue": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "addUeLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "cause": {
	///      "$ref": "#/components/schemas/Cause"
	///    },
	///    "ismfReleaseOnly": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "n2SmInfo": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfoType": {
	///      "$ref": "#/components/schemas/N2SmInfoType"
	///    },
	///    "ngApCause": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    },
	///    "ueLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "vsmfReleaseOnly": {
	///      "default": false,
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContextReleaseData {
		#[serde(
			rename = "addUeLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_ue_location: Option<UserLocation>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<Cause>,
		#[serde(
			rename = "5gMmCauseValue",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_g_mm_cause_value: Option<Uinteger>,
		#[serde(rename = "ismfReleaseOnly", default)]
		pub ismf_release_only: bool,
		#[serde(rename = "n2SmInfo", default, skip_serializing_if = "Option::is_none")]
		pub n2_sm_info: Option<RefToBinaryData>,
		#[serde(
			rename = "n2SmInfoType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_sm_info_type: Option<N2SmInfoType>,
		#[serde(rename = "ngApCause", default, skip_serializing_if = "Option::is_none")]
		pub ng_ap_cause: Option<NgApCause>,
		#[serde(
			rename = "ueLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_location: Option<UserLocation>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
		#[serde(rename = "vsmfReleaseOnly", default)]
		pub vsmf_release_only: bool,
	}

	impl From<&SmContextReleaseData> for SmContextReleaseData {
		fn from(value: &SmContextReleaseData) -> Self {
			value.clone()
		}
	}

	/// Data within Release SM Context Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Release SM Context Response",
	///  "type": "object",
	///  "properties": {
	///    "apnRateStatus": {
	///      "$ref": "#/components/schemas/ApnRateStatus"
	///    },
	///    "smallDataRateStatus": {
	///      "$ref": "#/components/schemas/SmallDataRateStatus"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContextReleasedData {
		#[serde(
			rename = "apnRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub apn_rate_status: Option<ApnRateStatus>,
		#[serde(
			rename = "smallDataRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub small_data_rate_status: Option<SmallDataRateStatus>,
	}

	impl From<&SmContextReleasedData> for SmContextReleasedData {
		fn from(value: &SmContextReleasedData) -> Self {
			value.clone()
		}
	}

	/// Data within Retrieve SM Context Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Retrieve SM Context Request",
	///  "type": "object",
	///  "properties": {
	///    "notToTransferEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerId"
	///      },
	///      "minItems": 1
	///    },
	///    "ranUnchangedInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "smContextType": {
	///      "$ref": "#/components/schemas/SmContextType"
	///    },
	///    "targetMmeCap": {
	///      "$ref": "#/components/schemas/MmeCapabilities"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContextRetrieveData {
		#[serde(
			rename = "notToTransferEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub not_to_transfer_ebi_list: Vec<EpsBearerId>,
		#[serde(rename = "ranUnchangedInd", default)]
		pub ran_unchanged_ind: bool,
		#[serde(
			rename = "servingNetwork",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network: Option<PlmnId>,
		#[serde(
			rename = "smContextType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_context_type: Option<SmContextType>,
		#[serde(
			rename = "targetMmeCap",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_mme_cap: Option<MmeCapabilities>,
	}

	impl From<&SmContextRetrieveData> for SmContextRetrieveData {
		fn from(value: &SmContextRetrieveData) -> Self {
			value.clone()
		}
	}

	/// Data within Retrieve SM Context Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Retrieve SM Context Response",
	///  "type": "object",
	///  "required": [
	///    "ueEpsPdnConnection"
	///  ],
	///  "properties": {
	///    "afCoordinationInfo": {
	///      "$ref": "#/components/schemas/AfCoordinationInfo"
	///    },
	///    "apnRateStatus": {
	///      "$ref": "#/components/schemas/ApnRateStatus"
	///    },
	///    "dlDataWaitingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "smContext": {
	///      "$ref": "#/components/schemas/SmContext"
	///    },
	///    "smallDataRateStatus": {
	///      "$ref": "#/components/schemas/SmallDataRateStatus"
	///    },
	///    "ueEpsPdnConnection": {
	///      "$ref": "#/components/schemas/EpsPdnCnxContainer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContextRetrievedData {
		#[serde(
			rename = "afCoordinationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub af_coordination_info: Option<AfCoordinationInfo>,
		#[serde(
			rename = "apnRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub apn_rate_status: Option<ApnRateStatus>,
		#[serde(rename = "dlDataWaitingInd", default)]
		pub dl_data_waiting_ind: bool,
		#[serde(rename = "smContext", default, skip_serializing_if = "Option::is_none")]
		pub sm_context: Option<SmContext>,
		#[serde(
			rename = "smallDataRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub small_data_rate_status: Option<SmallDataRateStatus>,
		#[serde(rename = "ueEpsPdnConnection")]
		pub ue_eps_pdn_connection: EpsPdnCnxContainer,
	}

	impl From<&SmContextRetrievedData> for SmContextRetrievedData {
		fn from(value: &SmContextRetrievedData) -> Self {
			value.clone()
		}
	}

	/// SmContextSscMode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-7]$"
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
	)]
	pub struct SmContextSscMode(String);

	impl ::std::ops::Deref for SmContextSscMode {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SmContextSscMode> for String {
		fn from(value: SmContextSscMode) -> Self {
			value.0
		}
	}

	impl From<&SmContextSscMode> for SmContextSscMode {
		fn from(value: &SmContextSscMode) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SmContextSscMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-7]$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-7]$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SmContextSscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SmContextSscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SmContextSscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SmContextSscMode {
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

	/// Data within Notify SM Context Status Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Notify SM Context Status Request",
	///  "type": "object",
	///  "required": [
	///    "statusInfo"
	///  ],
	///  "properties": {
	///    "altAnchorSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "altAnchorSmfUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "apnRateStatus": {
	///      "$ref": "#/components/schemas/ApnRateStatus"
	///    },
	///    "ddnFailureStatus": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "interPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "newIntermediateSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "newSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "newSmfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "notifyCorrelationIdsForddnFailure": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "oldPduSessionRef": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "oldSmContextRef": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "oldSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smallDataRateStatus": {
	///      "$ref": "#/components/schemas/SmallDataRateStatus"
	///    },
	///    "statusInfo": {
	///      "$ref": "#/components/schemas/StatusInfo"
	///    },
	///    "targetDnaiInfo": {
	///      "$ref": "#/components/schemas/TargetDnaiInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContextStatusNotification {
		#[serde(
			rename = "altAnchorSmfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub alt_anchor_smf_id: Option<NfInstanceId>,
		#[serde(
			rename = "altAnchorSmfUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub alt_anchor_smf_uri: Option<Uri>,
		#[serde(
			rename = "apnRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub apn_rate_status: Option<ApnRateStatus>,
		#[serde(rename = "ddnFailureStatus", default)]
		pub ddn_failure_status: bool,
		#[serde(
			rename = "interPlmnApiRoot",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_api_root: Option<Uri>,
		#[serde(
			rename = "newIntermediateSmfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub new_intermediate_smf_id: Option<NfInstanceId>,
		#[serde(rename = "newSmfId", default, skip_serializing_if = "Option::is_none")]
		pub new_smf_id: Option<NfInstanceId>,
		#[serde(
			rename = "newSmfSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub new_smf_set_id: Option<NfSetId>,
		#[serde(
			rename = "notifyCorrelationIdsForddnFailure",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub notify_correlation_ids_forddn_failure: Vec<String>,
		#[serde(
			rename = "oldPduSessionRef",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub old_pdu_session_ref: Option<Uri>,
		#[serde(
			rename = "oldSmContextRef",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub old_sm_context_ref: Option<Uri>,
		#[serde(rename = "oldSmfId", default, skip_serializing_if = "Option::is_none")]
		pub old_smf_id: Option<NfInstanceId>,
		#[serde(
			rename = "smallDataRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub small_data_rate_status: Option<SmallDataRateStatus>,
		#[serde(rename = "statusInfo")]
		pub status_info: StatusInfo,
		#[serde(
			rename = "targetDnaiInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_dnai_info: Option<TargetDnaiInfo>,
	}

	impl From<&SmContextStatusNotification> for SmContextStatusNotification {
		fn from(value: &SmContextStatusNotification) -> Self {
			value.clone()
		}
	}

	/// Type of SM Context information. Possible values are
	///  - EPS_PDN_CONNECTION
	///  - SM_CONTEXT
	///  - AF_COORDINATION_INFO
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Type of SM Context information. Possible values are\n
	/// - EPS_PDN_CONNECTION\n  - SM_CONTEXT\n  - AF_COORDINATION_INFO\n",
	///  "type": "string",
	///  "enum": [
	///    "EPS_PDN_CONNECTION",
	///    "SM_CONTEXT",
	///    "AF_COORDINATION_INFO"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum SmContextType {
		#[default]
		#[serde(rename = "EPS_PDN_CONNECTION")]
		EpsPdnConnection,
		#[serde(rename = "SM_CONTEXT")]
		SmContext,
		#[serde(rename = "AF_COORDINATION_INFO")]
		AfCoordinationInfo,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmContextType> for SmContextType {
		fn from(value: &SmContextType) -> Self {
			value.clone()
		}
	}

	impl ToString for SmContextType {
		fn to_string(&self) -> String {
			match *self {
				Self::EpsPdnConnection => "EPS_PDN_CONNECTION".to_string(),
				Self::SmContext => "SM_CONTEXT".to_string(),
				Self::AfCoordinationInfo => "AF_COORDINATION_INFO".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmContextType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EPS_PDN_CONNECTION" => Ok(Self::EpsPdnConnection),
				"SM_CONTEXT" => Ok(Self::SmContext),
				"AF_COORDINATION_INFO" => Ok(Self::AfCoordinationInfo),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmContextType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmContextType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmContextType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within Update SM Context Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Update SM Context Request",
	///  "type": "object",
	///  "properties": {
	///    "5gMmCauseValue": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "addUeLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "additionalAnType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "anType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "anTypeCanBeChanged": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "anTypeToReactivate": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "backupAmfInfo": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/BackupAmfInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "cause": {
	///      "$ref": "#/components/schemas/Cause"
	///    },
	///    "dataForwarding": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ddnFailureSubs": {
	///      "$ref": "#/components/schemas/DdnFailureSubs"
	///    },
	///    "epsBearerSetup": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerContainer"
	///      },
	///      "minItems": 0
	///    },
	///    "epsInterworkingInd": {
	///      "$ref": "#/components/schemas/EpsInterworkingIndication"
	///    },
	///    "exemptionInd": {
	///      "$ref": "#/components/schemas/ExemptionInd"
	///    },
	///    "extendedNasSmTimerInd": {
	///      "type": "boolean"
	///    },
	///    "failedToBeSwitched": {
	///      "type": "boolean"
	///    },
	///    "forwardingBearerContexts": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ForwardingBearerContainer"
	///      },
	///      "minItems": 1
	///    },
	///    "forwardingFTeid": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "hoState": {
	///      "$ref": "#/components/schemas/HoState"
	///    },
	///    "maNwUpgradeInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "maReleaseInd": {
	///      "$ref": "#/components/schemas/MaReleaseIndication"
	///    },
	///    "maRequestInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "moExpDataCounter": {
	///      "$ref": "#/components/schemas/MoExpDataCounter"
	///    },
	///    "n1SmMsg": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfo": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfoExt1": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfoType": {
	///      "$ref": "#/components/schemas/N2SmInfoType"
	///    },
	///    "n2SmInfoTypeExt1": {
	///      "$ref": "#/components/schemas/N2SmInfoType"
	///    },
	///    "n9DlForwardingTnlList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IndirectDataForwardingTunnelInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "n9DlForwardingTunnel": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "n9ForwardingTunnel": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "n9InactivityTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "n9UlForwardingTnlList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IndirectDataForwardingTunnelInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "ngApCause": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    },
	///    "pcfUeCallbackInfo": {
	///      "$ref": "#/components/schemas/PcfUeCallbackInfo"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "presenceInLadn": {
	///      "$ref": "#/components/schemas/PresenceState"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "release": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "revokeEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerId"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "satelliteBackhaulCat": {
	///      "$ref": "#/components/schemas/SatelliteBackhaulCategory"
	///    },
	///    "secondaryRatUsageDataReportContainer": {
	///      "type": "array",
	///      "items": {
	///        "$ref":
	/// "#/components/schemas/SecondaryRatUsageDataReportContainer"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "servingNfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "skipN2PduSessionResRelInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "smContextStatusUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "smPolicyNotifyInd": {
	///      "type": "boolean",
	///      "enum": [
	///        true
	///      ]
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "targetId": {
	///      "$ref": "#/components/schemas/NgRanTargetId"
	///    },
	///    "targetServingNfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "toBeSwitched": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "traceData": {
	///      "$ref": "#/components/schemas/TraceData"
	///    },
	///    "ueLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "upCnxState": {
	///      "$ref": "#/components/schemas/UpCnxState"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContextUpdateData {
		#[serde(
			rename = "addUeLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_ue_location: Option<UserLocation>,
		#[serde(
			rename = "additionalAnType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_an_type: Option<AccessType>,
		#[serde(rename = "anType", default, skip_serializing_if = "Option::is_none")]
		pub an_type: Option<AccessType>,
		#[serde(rename = "anTypeCanBeChanged", default)]
		pub an_type_can_be_changed: bool,
		#[serde(
			rename = "anTypeToReactivate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub an_type_to_reactivate: Option<AccessType>,
		#[serde(
			rename = "backupAmfInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub backup_amf_info: Option<Vec<BackupAmfInfo>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<Cause>,
		#[serde(rename = "dataForwarding", default)]
		pub data_forwarding: bool,
		#[serde(
			rename = "ddnFailureSubs",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ddn_failure_subs: Option<DdnFailureSubs>,
		#[serde(
			rename = "epsBearerSetup",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub eps_bearer_setup: Vec<EpsBearerContainer>,
		#[serde(
			rename = "epsInterworkingInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_interworking_ind: Option<EpsInterworkingIndication>,
		#[serde(
			rename = "exemptionInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub exemption_ind: Option<ExemptionInd>,
		#[serde(
			rename = "extendedNasSmTimerInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub extended_nas_sm_timer_ind: Option<bool>,
		#[serde(
			rename = "failedToBeSwitched",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub failed_to_be_switched: Option<bool>,
		#[serde(
			rename = "5gMmCauseValue",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_g_mm_cause_value: Option<Uinteger>,
		#[serde(
			rename = "forwardingBearerContexts",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub forwarding_bearer_contexts: Vec<ForwardingBearerContainer>,
		#[serde(
			rename = "forwardingFTeid",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub forwarding_f_teid: Option<Bytes>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<crate::common::common_models::Guami>,
		#[serde(rename = "hoState", default, skip_serializing_if = "Option::is_none")]
		pub ho_state: Option<HoState>,
		#[serde(rename = "maNwUpgradeInd", default)]
		pub ma_nw_upgrade_ind: bool,
		#[serde(
			rename = "maReleaseInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ma_release_ind: Option<MaReleaseIndication>,
		#[serde(rename = "maRequestInd", default)]
		pub ma_request_ind: bool,
		#[serde(
			rename = "moExpDataCounter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mo_exp_data_counter: Option<MoExpDataCounter>,
		#[serde(rename = "n1SmMsg", default, skip_serializing_if = "Option::is_none")]
		pub n1_sm_msg: Option<RefToBinaryData>,
		#[serde(rename = "n2SmInfo", default, skip_serializing_if = "Option::is_none")]
		pub n2_sm_info: Option<RefToBinaryData>,
		#[serde(
			rename = "n2SmInfoExt1",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_sm_info_ext1: Option<RefToBinaryData>,
		#[serde(
			rename = "n2SmInfoType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_sm_info_type: Option<N2SmInfoType>,
		#[serde(
			rename = "n2SmInfoTypeExt1",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_sm_info_type_ext1: Option<N2SmInfoType>,
		#[serde(
			rename = "n9DlForwardingTnlList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub n9_dl_forwarding_tnl_list: Vec<IndirectDataForwardingTunnelInfo>,
		#[serde(
			rename = "n9DlForwardingTunnel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n9_dl_forwarding_tunnel: Option<TunnelInfo>,
		#[serde(
			rename = "n9ForwardingTunnel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n9_forwarding_tunnel: Option<TunnelInfo>,
		#[serde(
			rename = "n9InactivityTimer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n9_inactivity_timer: Option<DurationSec>,
		#[serde(
			rename = "n9UlForwardingTnlList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub n9_ul_forwarding_tnl_list: Vec<IndirectDataForwardingTunnelInfo>,
		#[serde(rename = "ngApCause", default, skip_serializing_if = "Option::is_none")]
		pub ng_ap_cause: Option<NgApCause>,
		#[serde(
			rename = "pcfUeCallbackInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_ue_callback_info: Option<PcfUeCallbackInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(
			rename = "presenceInLadn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub presence_in_ladn: Option<PresenceState>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(default)]
		pub release: bool,
		#[serde(
			rename = "revokeEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub revoke_ebi_list: Vec<EpsBearerId>,
		#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
		pub s_nssai: Option<Snssai>,
		#[serde(
			rename = "satelliteBackhaulCat",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub satellite_backhaul_cat: Option<SatelliteBackhaulCategory>,
		#[serde(
			rename = "secondaryRatUsageDataReportContainer",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub secondary_rat_usage_data_report_container: Vec<SecondaryRatUsageDataReportContainer>,
		#[serde(
			rename = "servingNetwork",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network: Option<PlmnIdNid>,
		#[serde(
			rename = "servingNfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_nf_id: Option<NfInstanceId>,
		#[serde(rename = "skipN2PduSessionResRelInd", default)]
		pub skip_n2_pdu_session_res_rel_ind: bool,
		#[serde(
			rename = "smContextStatusUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_context_status_uri: Option<Uri>,
		#[serde(
			rename = "smPolicyNotifyInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sm_policy_notify_ind: Option<bool>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
		pub target_id: Option<NgRanTargetId>,
		#[serde(
			rename = "targetServingNfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_serving_nf_id: Option<NfInstanceId>,
		#[serde(rename = "toBeSwitched", default)]
		pub to_be_switched: bool,
		#[serde(rename = "traceData", default, skip_serializing_if = "Option::is_none")]
		pub trace_data: Option<TraceData>,
		#[serde(
			rename = "ueLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_location: Option<UserLocation>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
		#[serde(
			rename = "upCnxState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_cnx_state: Option<UpCnxState>,
	}

	impl From<&SmContextUpdateData> for SmContextUpdateData {
		fn from(value: &SmContextUpdateData) -> Self {
			value.clone()
		}
	}

	/// Error within Update SM Context Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Error within Update SM Context Response",
	///  "type": "object",
	///  "required": [
	///    "error"
	///  ],
	///  "properties": {
	///    "error": {
	///      "$ref": "#/components/schemas/ExtProblemDetails"
	///    },
	///    "n1SmMsg": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfo": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfoType": {
	///      "$ref": "#/components/schemas/N2SmInfoType"
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "upCnxState": {
	///      "$ref": "#/components/schemas/UpCnxState"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContextUpdateError {
		pub error: ExtProblemDetails,
		#[serde(rename = "n1SmMsg", default, skip_serializing_if = "Option::is_none")]
		pub n1_sm_msg: Option<RefToBinaryData>,
		#[serde(rename = "n2SmInfo", default, skip_serializing_if = "Option::is_none")]
		pub n2_sm_info: Option<RefToBinaryData>,
		#[serde(
			rename = "n2SmInfoType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_sm_info_type: Option<N2SmInfoType>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
		#[serde(
			rename = "upCnxState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_cnx_state: Option<UpCnxState>,
	}

	impl From<&SmContextUpdateError> for SmContextUpdateError {
		fn from(value: &SmContextUpdateError) -> Self {
			value.clone()
		}
	}

	/// Data within Update SM Context Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Update SM Context Response",
	///  "type": "object",
	///  "properties": {
	///    "allocatedEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EbiArpMapping"
	///      },
	///      "minItems": 1
	///    },
	///    "anchorSmfFeatures": {
	///      "$ref": "#/components/schemas/AnchorSmfFeatures"
	///    },
	///    "cause": {
	///      "$ref": "#/components/schemas/Cause"
	///    },
	///    "dataForwarding": {
	///      "type": "boolean"
	///    },
	///    "epsBearerSetup": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerContainer"
	///      },
	///      "minItems": 1
	///    },
	///    "forwardingBearerContexts": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ForwardingBearerContainer"
	///      },
	///      "minItems": 1
	///    },
	///    "forwardingFTeid": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "hoState": {
	///      "$ref": "#/components/schemas/HoState"
	///    },
	///    "interPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "maAcceptedInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "modifiedEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EbiArpMapping"
	///      },
	///      "minItems": 1
	///    },
	///    "n1SmMsg": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfo": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n2SmInfoType": {
	///      "$ref": "#/components/schemas/N2SmInfoType"
	///    },
	///    "n3DlForwardingTnlList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IndirectDataForwardingTunnelInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "n3UlForwardingTnlList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IndirectDataForwardingTunnelInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "n9UlForwardingTunnel": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "releaseEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerId"
	///      },
	///      "minItems": 1
	///    },
	///    "selectedOldSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "selectedSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "upCnxState": {
	///      "$ref": "#/components/schemas/UpCnxState"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmContextUpdatedData {
		#[serde(
			rename = "allocatedEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allocated_ebi_list: Vec<EbiArpMapping>,
		#[serde(
			rename = "anchorSmfFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub anchor_smf_features: Option<AnchorSmfFeatures>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<Cause>,
		#[serde(
			rename = "dataForwarding",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub data_forwarding: Option<bool>,
		#[serde(
			rename = "epsBearerSetup",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub eps_bearer_setup: Vec<EpsBearerContainer>,
		#[serde(
			rename = "forwardingBearerContexts",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub forwarding_bearer_contexts: Vec<ForwardingBearerContainer>,
		#[serde(
			rename = "forwardingFTeid",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub forwarding_f_teid: Option<Bytes>,
		#[serde(rename = "hoState", default, skip_serializing_if = "Option::is_none")]
		pub ho_state: Option<HoState>,
		#[serde(
			rename = "interPlmnApiRoot",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_api_root: Option<Uri>,
		#[serde(rename = "maAcceptedInd", default)]
		pub ma_accepted_ind: bool,
		#[serde(
			rename = "modifiedEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub modified_ebi_list: Vec<EbiArpMapping>,
		#[serde(rename = "n1SmMsg", default, skip_serializing_if = "Option::is_none")]
		pub n1_sm_msg: Option<RefToBinaryData>,
		#[serde(rename = "n2SmInfo", default, skip_serializing_if = "Option::is_none")]
		pub n2_sm_info: Option<RefToBinaryData>,
		#[serde(
			rename = "n2SmInfoType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_sm_info_type: Option<N2SmInfoType>,
		#[serde(
			rename = "n3DlForwardingTnlList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub n3_dl_forwarding_tnl_list: Vec<IndirectDataForwardingTunnelInfo>,
		#[serde(
			rename = "n3UlForwardingTnlList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub n3_ul_forwarding_tnl_list: Vec<IndirectDataForwardingTunnelInfo>,
		#[serde(
			rename = "n9UlForwardingTunnel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n9_ul_forwarding_tunnel: Option<TunnelInfo>,
		#[serde(
			rename = "releaseEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub release_ebi_list: Vec<EpsBearerId>,
		#[serde(
			rename = "selectedOldSmfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub selected_old_smf_id: Option<NfInstanceId>,
		#[serde(
			rename = "selectedSmfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub selected_smf_id: Option<NfInstanceId>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "upCnxState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub up_cnx_state: Option<UpCnxState>,
	}

	impl From<&SmContextUpdatedData> for SmContextUpdatedData {
		fn from(value: &SmContextUpdatedData) -> Self {
			value.clone()
		}
	}

	/// Represents information on the SM congestion control applied SM NAS
	/// messages that SMF sends  to UE for PDU Session.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents information on the SM congestion control
	/// applied SM NAS messages that SMF sends  to UE for PDU Session.\n",
	///  "type": "object",
	///  "required": [
	///    "appliedSmccType",
	///    "backoffTimer",
	///    "smNasType",
	///    "timeStamp"
	///  ],
	///  "properties": {
	///    "appliedSmccType": {
	///      "$ref": "#/components/schemas/AppliedSmccType"
	///    },
	///    "backoffTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "smNasType": {
	///      "type": "string"
	///    },
	///    "timeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmNasFromSmf {
		#[serde(rename = "appliedSmccType")]
		pub applied_smcc_type: AppliedSmccType,
		#[serde(rename = "backoffTimer")]
		pub backoff_timer: DurationSec,
		#[serde(rename = "smNasType")]
		pub sm_nas_type: String,
		#[serde(rename = "timeStamp")]
		pub time_stamp: DateTime,
	}

	impl From<&SmNasFromSmf> for SmNasFromSmf {
		fn from(value: &SmNasFromSmf) -> Self {
			value.clone()
		}
	}

	/// Represents information on the SM NAS messages that SMF receives from UE
	/// for PDU Session.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents information on the SM NAS messages that SMF
	/// receives from UE for PDU Session.\n",
	///  "type": "object",
	///  "required": [
	///    "smNasType",
	///    "timeStamp"
	///  ],
	///  "properties": {
	///    "smNasType": {
	///      "type": "string"
	///    },
	///    "timeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmNasFromUe {
		#[serde(rename = "smNasType")]
		pub sm_nas_type: String,
		#[serde(rename = "timeStamp")]
		pub time_stamp: DateTime,
	}

	impl From<&SmNasFromUe> for SmNasFromUe {
		fn from(value: &SmNasFromUe) -> Self {
			value.clone()
		}
	}

	/// It indicates theSmall Data Rate Control Status
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It indicates theSmall Data Rate Control Status",
	///  "type": "object",
	///  "properties": {
	///    "remainExReportsDl": {
	///      "description": "When present, it shall indicate number of
	/// additional exception reports the AF is allowed to send downlink  in the
	/// given time unit for the given PDU session (see clause 5.31.14.3 in 3GPP
	/// TS 23.501\n",
	///      "type": "integer",
	///      "minimum": 0.0
	///    },
	///    "remainExReportsUl": {
	///      "description": "When present, it shall indicate number of
	/// additional exception reports the UE is allowed to send uplink in the
	/// given time  unit for  the given PDU session (see clause 5.31.14.3 of
	/// 3GPP TS 23.501.\n",
	///      "type": "integer",
	///      "minimum": 0.0
	///    },
	///    "remainPacketsDl": {
	///      "description": "When present it shall contain the number of packets
	/// the AF is allowed to send downlink in the given time unit for the given
	/// PDU session (see clause 5.31.14.3 of 3GPP TS 23.501.\n",
	///      "type": "integer",
	///      "minimum": 0.0
	///    },
	///    "remainPacketsUl": {
	///      "description": "When present, it shall contain the number of
	/// packets the UE is allowed to send uplink in the given time unit for the
	/// given PDU session (see clause 5.31.14.3 of 3GPP TS 23.501.\n",
	///      "type": "integer",
	///      "minimum": 0.0
	///    },
	///    "validityTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmallDataRateStatus {
		/// When present, it shall indicate number of additional exception
		/// reports the AF is allowed to send downlink  in the given time unit
		/// for the given PDU session (see clause 5.31.14.3 in 3GPP TS 23.501
		#[serde(
			rename = "remainExReportsDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remain_ex_reports_dl: Option<u64>,
		/// When present, it shall indicate number of additional exception
		/// reports the UE is allowed to send uplink in the given time  unit for
		/// the given PDU session (see clause 5.31.14.3 of 3GPP TS 23.501.
		#[serde(
			rename = "remainExReportsUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remain_ex_reports_ul: Option<u64>,
		/// When present it shall contain the number of packets the AF is
		/// allowed to send downlink in the given time unit for the given PDU
		/// session (see clause 5.31.14.3 of 3GPP TS 23.501.
		#[serde(
			rename = "remainPacketsDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remain_packets_dl: Option<u64>,
		/// When present, it shall contain the number of packets the UE is
		/// allowed to send uplink in the given time unit for the given PDU
		/// session (see clause 5.31.14.3 of 3GPP TS 23.501.
		#[serde(
			rename = "remainPacketsUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remain_packets_ul: Option<u64>,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&SmallDataRateStatus> for SmallDataRateStatus {
		fn from(value: &SmallDataRateStatus) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - AC_TY_CH: Access Type Change
	/// - UP_PATH_CH: UP Path Change
	/// - PDU_SES_REL: PDU Session Release
	/// - PLMN_CH: PLMN Change
	/// - UE_IP_CH: UE IP address change
	/// - RAT_TY_CH: RAT Type Change
	/// - DDDS: Downlink data delivery status
	/// - COMM_FAIL: Communication Failure
	/// - PDU_SES_EST: PDU Session Establishment
	/// - QFI_ALLOC: QFI allocation
	/// - QOS_MON: QoS Monitoring
	/// - SMCC_EXP: SM congestion control experience for PDU Session
	/// - DISPERSION: Session Management transaction dispersion
	/// - RED_TRANS_EXP: Redundant transmission experience for PDU Session
	/// - WLAN_INFO: WLAN information on PDU session for which Access Type is
	///   NON_3GPP_ACCESS and
	///  RAT Type is TRUSTED_WLAN
	/// - UPF_INFO: The UPF information, including the UPF ID/address/FQDN
	///   information.
	/// - UP_STATUS_INFO: The User Plane status information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- AC_TY_CH: Access Type Change\n-
	/// UP_PATH_CH: UP Path Change\n- PDU_SES_REL: PDU Session Release\n-
	/// PLMN_CH: PLMN Change\n- UE_IP_CH: UE IP address change\n- RAT_TY_CH: RAT
	/// Type Change\n- DDDS: Downlink data delivery status\n- COMM_FAIL:
	/// Communication Failure\n- PDU_SES_EST: PDU Session Establishment\n-
	/// QFI_ALLOC: QFI allocation\n- QOS_MON: QoS Monitoring\n- SMCC_EXP: SM
	/// congestion control experience for PDU Session\n- DISPERSION: Session
	/// Management transaction dispersion\n- RED_TRANS_EXP: Redundant
	/// transmission experience for PDU Session\n- WLAN_INFO: WLAN information
	/// on PDU session for which Access Type is NON_3GPP_ACCESS and\n  RAT Type
	/// is TRUSTED_WLAN\n- UPF_INFO: The UPF information, including the UPF
	/// ID/address/FQDN information.\n- UP_STATUS_INFO: The User Plane status
	/// information.\n",
	///  "type": "string",
	///  "enum": [
	///    "AC_TY_CH",
	///    "UP_PATH_CH",
	///    "PDU_SES_REL",
	///    "PLMN_CH",
	///    "UE_IP_CH",
	///    "RAT_TY_CH",
	///    "DDDS",
	///    "COMM_FAIL",
	///    "PDU_SES_EST",
	///    "QFI_ALLOC",
	///    "QOS_MON",
	///    "SMCC_EXP",
	///    "DISPERSION",
	///    "RED_TRANS_EXP",
	///    "WLAN_INFO",
	///    "UPF_INFO",
	///    "UP_STATUS_INFO"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum SmfEvent {
		#[default]
		#[serde(rename = "AC_TY_CH")]
		AcTyCh,
		#[serde(rename = "UP_PATH_CH")]
		UpPathCh,
		#[serde(rename = "PDU_SES_REL")]
		PduSesRel,
		#[serde(rename = "PLMN_CH")]
		PlmnCh,
		#[serde(rename = "UE_IP_CH")]
		UeIpCh,
		#[serde(rename = "RAT_TY_CH")]
		RatTyCh,
		#[serde(rename = "DDDS")]
		Ddds,
		#[serde(rename = "COMM_FAIL")]
		CommFail,
		#[serde(rename = "PDU_SES_EST")]
		PduSesEst,
		#[serde(rename = "QFI_ALLOC")]
		QfiAlloc,
		#[serde(rename = "QOS_MON")]
		QosMon,
		#[serde(rename = "SMCC_EXP")]
		SmccExp,
		#[serde(rename = "DISPERSION")]
		Dispersion,
		#[serde(rename = "RED_TRANS_EXP")]
		RedTransExp,
		#[serde(rename = "WLAN_INFO")]
		WlanInfo,
		#[serde(rename = "UPF_INFO")]
		UpfInfo,
		#[serde(rename = "UP_STATUS_INFO")]
		UpStatusInfo,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmfEvent> for SmfEvent {
		fn from(value: &SmfEvent) -> Self {
			value.clone()
		}
	}

	impl ToString for SmfEvent {
		fn to_string(&self) -> String {
			match *self {
				Self::AcTyCh => "AC_TY_CH".to_string(),
				Self::UpPathCh => "UP_PATH_CH".to_string(),
				Self::PduSesRel => "PDU_SES_REL".to_string(),
				Self::PlmnCh => "PLMN_CH".to_string(),
				Self::UeIpCh => "UE_IP_CH".to_string(),
				Self::RatTyCh => "RAT_TY_CH".to_string(),
				Self::Ddds => "DDDS".to_string(),
				Self::CommFail => "COMM_FAIL".to_string(),
				Self::PduSesEst => "PDU_SES_EST".to_string(),
				Self::QfiAlloc => "QFI_ALLOC".to_string(),
				Self::QosMon => "QOS_MON".to_string(),
				Self::SmccExp => "SMCC_EXP".to_string(),
				Self::Dispersion => "DISPERSION".to_string(),
				Self::RedTransExp => "RED_TRANS_EXP".to_string(),
				Self::WlanInfo => "WLAN_INFO".to_string(),
				Self::UpfInfo => "UPF_INFO".to_string(),
				Self::UpStatusInfo => "UP_STATUS_INFO".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmfEvent {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AC_TY_CH" => Ok(Self::AcTyCh),
				"UP_PATH_CH" => Ok(Self::UpPathCh),
				"PDU_SES_REL" => Ok(Self::PduSesRel),
				"PLMN_CH" => Ok(Self::PlmnCh),
				"UE_IP_CH" => Ok(Self::UeIpCh),
				"RAT_TY_CH" => Ok(Self::RatTyCh),
				"DDDS" => Ok(Self::Ddds),
				"COMM_FAIL" => Ok(Self::CommFail),
				"PDU_SES_EST" => Ok(Self::PduSesEst),
				"QFI_ALLOC" => Ok(Self::QfiAlloc),
				"QOS_MON" => Ok(Self::QosMon),
				"SMCC_EXP" => Ok(Self::SmccExp),
				"DISPERSION" => Ok(Self::Dispersion),
				"RED_TRANS_EXP" => Ok(Self::RedTransExp),
				"WLAN_INFO" => Ok(Self::WlanInfo),
				"UPF_INFO" => Ok(Self::UpfInfo),
				"UP_STATUS_INFO" => Ok(Self::UpStatusInfo),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmfEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmfEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmfEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Smf Selection Type. Possible values are
	///  - CURRENT_PDU_SESSION
	///  - NEXT_PDU_SESSION
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Smf Selection Type. Possible values are\n  -
	/// CURRENT_PDU_SESSION\n  - NEXT_PDU_SESSION\n",
	///  "type": "string",
	///  "enum": [
	///    "CURRENT_PDU_SESSION",
	///    "NEXT_PDU_SESSION"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum SmfSelectionType {
		#[default]
		#[serde(rename = "CURRENT_PDU_SESSION")]
		CurrentPduSession,
		#[serde(rename = "NEXT_PDU_SESSION")]
		NextPduSession,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmfSelectionType> for SmfSelectionType {
		fn from(value: &SmfSelectionType) -> Self {
			value.clone()
		}
	}

	impl ToString for SmfSelectionType {
		fn to_string(&self) -> String {
			match *self {
				Self::CurrentPduSession => "CURRENT_PDU_SESSION".to_string(),
				Self::NextPduSession => "NEXT_PDU_SESSION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmfSelectionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CURRENT_PDU_SESSION" => Ok(Self::CurrentPduSession),
				"NEXT_PDU_SESSION" => Ok(Self::NextPduSession),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmfSelectionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmfSelectionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmfSelectionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// When Snssai needs to be converted to string (e.g. when used in maps as
	/// key), the string shall be composed of one to three digits "sst"
	/// optionally followed by "-" and 6 hexadecimal digits "sd".
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "When Snssai needs to be converted to string (e.g. when
	/// used in maps as key), the string shall be composed of one to three
	/// digits \"sst\" optionally followed by \"-\" and 6 hexadecimal digits
	/// \"sd\".\n",
	///  "type": "object",
	///  "required": [
	///    "sst"
	///  ],
	///  "properties": {
	///    "sd": {
	///      "description": "3-octet string, representing the Slice
	/// Differentiator, in hexadecimal representation. Each character in the
	/// string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to
	/// \"F\" and shall represent 4 bits. The most significant character
	/// representing the 4 most significant bits of the SD shall appear first in
	/// the string, and the character representing the 4 least significant bit
	/// of the SD shall appear last in the string. This is an optional parameter
	/// that complements the Slice/Service type(s) to allow to  differentiate
	/// amongst multiple Network Slices of the same Slice/Service type. This IE
	/// shall be absent if no SD value is associated with the SST.\n",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{6}$"
	///    },
	///    "sst": {
	///      "description": "Unsigned integer, within the range 0 to 255,
	/// representing the Slice/Service Type.  It indicates the expected Network
	/// Slice behaviour in terms of features and services. Values 0 to 127
	/// correspond to the standardized SST range. Values 128 to 255 correspond
	/// to the Operator-specific range. See clause 28.4.2 of 3GPP TS 23.003.
	/// Standardized values are defined in clause 5.15.2.2 of 3GPP TS 23.501.
	/// \n",
	///      "type": "integer",
	///      "maximum": 255.0,
	///      "minimum": 0.0
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Snssai {
		/// 3-octet string, representing the Slice Differentiator, in
		/// hexadecimal representation. Each character in the string shall take
		/// a value of "0" to "9", "a" to "f" or "A" to "F" and shall represent
		/// 4 bits. The most significant character representing the 4 most
		/// significant bits of the SD shall appear first in the string, and the
		/// character representing the 4 least significant bit of the SD shall
		/// appear last in the string. This is an optional parameter that
		/// complements the Slice/Service type(s) to allow to  differentiate
		/// amongst multiple Network Slices of the same Slice/Service type. This
		/// IE shall be absent if no SD value is associated with the SST.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub sd: Option<SnssaiSd>,
		/// Unsigned integer, within the range 0 to 255, representing the
		/// Slice/Service Type.  It indicates the expected Network Slice
		/// behaviour in terms of features and services. Values 0 to 127
		/// correspond to the standardized SST range. Values 128 to 255
		/// correspond  to the Operator-specific range. See clause 28.4.2 of
		/// 3GPP TS 23.003. Standardized values are defined in clause 5.15.2.2
		/// of 3GPP TS 23.501.
		pub sst: u8,
	}

	impl From<&Snssai> for Snssai {
		fn from(value: &Snssai) -> Self {
			value.clone()
		}
	}

	/// 3-octet string, representing the Slice Differentiator, in hexadecimal
	/// representation. Each character in the string shall take a value of "0"
	/// to "9", "a" to "f" or "A" to "F" and shall represent 4 bits. The most
	/// significant character representing the 4 most significant bits of the SD
	/// shall appear first in the string, and the character representing the 4
	/// least significant bit of the SD shall appear last in the string. This is
	/// an optional parameter that complements the Slice/Service type(s) to
	/// allow to  differentiate amongst multiple Network Slices of the same
	/// Slice/Service type. This IE shall be absent if no SD value is associated
	/// with the SST.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "3-octet string, representing the Slice Differentiator,
	/// in hexadecimal representation. Each character in the string shall take a
	/// value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall
	/// represent 4 bits. The most significant character representing the 4 most
	/// significant bits of the SD shall appear first in the string, and the
	/// character representing the 4 least significant bit of the SD shall
	/// appear last in the string. This is an optional parameter that
	/// complements the Slice/Service type(s) to allow to  differentiate amongst
	/// multiple Network Slices of the same Slice/Service type. This IE shall be
	/// absent if no SD value is associated with the SST.\n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{6}$"
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
	)]
	pub struct SnssaiSd(String);

	impl ::std::ops::Deref for SnssaiSd {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SnssaiSd> for String {
		fn from(value: SnssaiSd) -> Self {
			value.0
		}
	}

	impl From<&SnssaiSd> for SnssaiSd {
		fn from(value: &SnssaiSd) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SnssaiSd {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{6}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{6}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SnssaiSd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SnssaiSd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SnssaiSd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SnssaiSd {
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

	/// Possible values are:
	/// - STATIONARY: Identifies the UE is stationary
	/// - MOBILE: Identifies the UE is mobile
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- STATIONARY: Identifies the UE
	/// is stationary\n- MOBILE: Identifies the UE is mobile\n",
	///  "type": "string",
	///  "enum": [
	///    "STATIONARY",
	///    "MOBILE"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum StationaryIndication {
		#[default]
		#[serde(rename = "STATIONARY")]
		Stationary,
		#[serde(rename = "MOBILE")]
		Mobile,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&StationaryIndication> for StationaryIndication {
		fn from(value: &StationaryIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for StationaryIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::Stationary => "STATIONARY".to_string(),
				Self::Mobile => "MOBILE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for StationaryIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"STATIONARY" => Ok(Self::Stationary),
				"MOBILE" => Ok(Self::Mobile),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for StationaryIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for StationaryIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for StationaryIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Status of SM context or of PDU session
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Status of SM context or of PDU session",
	///  "type": "object",
	///  "required": [
	///    "resourceStatus"
	///  ],
	///  "properties": {
	///    "anType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "cause": {
	///      "$ref": "#/components/schemas/Cause"
	///    },
	///    "cnAssistedRanPara": {
	///      "$ref": "#/components/schemas/CnAssistedRanPara"
	///    },
	///    "resourceStatus": {
	///      "$ref": "#/components/schemas/ResourceStatus"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct StatusInfo {
		#[serde(rename = "anType", default, skip_serializing_if = "Option::is_none")]
		pub an_type: Option<AccessType>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<Cause>,
		#[serde(
			rename = "cnAssistedRanPara",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cn_assisted_ran_para: Option<CnAssistedRanPara>,
		#[serde(rename = "resourceStatus")]
		pub resource_status: ResourceStatus,
	}

	impl From<&StatusInfo> for StatusInfo {
		fn from(value: &StatusInfo) -> Self {
			value.clone()
		}
	}

	/// Data within Notify Status Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Notify Status Request",
	///  "type": "object",
	///  "required": [
	///    "statusInfo"
	///  ],
	///  "properties": {
	///    "apnRateStatus": {
	///      "$ref": "#/components/schemas/ApnRateStatus"
	///    },
	///    "epsPdnCnxInfo": {
	///      "$ref": "#/components/schemas/EpsPdnCnxInfo"
	///    },
	///    "interPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "intraPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "newSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "oldPduSessionRef": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "smallDataRateStatus": {
	///      "$ref": "#/components/schemas/SmallDataRateStatus"
	///    },
	///    "statusInfo": {
	///      "$ref": "#/components/schemas/StatusInfo"
	///    },
	///    "targetDnaiInfo": {
	///      "$ref": "#/components/schemas/TargetDnaiInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct StatusNotification {
		#[serde(
			rename = "apnRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub apn_rate_status: Option<ApnRateStatus>,
		#[serde(
			rename = "epsPdnCnxInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_pdn_cnx_info: Option<EpsPdnCnxInfo>,
		#[serde(
			rename = "interPlmnApiRoot",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_api_root: Option<Uri>,
		#[serde(
			rename = "intraPlmnApiRoot",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub intra_plmn_api_root: Option<Uri>,
		#[serde(rename = "newSmfId", default, skip_serializing_if = "Option::is_none")]
		pub new_smf_id: Option<NfInstanceId>,
		#[serde(
			rename = "oldPduSessionRef",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub old_pdu_session_ref: Option<Uri>,
		#[serde(
			rename = "smallDataRateStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub small_data_rate_status: Option<SmallDataRateStatus>,
		#[serde(rename = "statusInfo")]
		pub status_info: StatusInfo,
		#[serde(
			rename = "targetDnaiInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_dnai_info: Option<TargetDnaiInfo>,
	}

	impl From<&StatusNotification> for StatusNotification {
		fn from(value: &StatusNotification) -> Self {
			value.clone()
		}
	}

	/// Identifies an Individual SMF Notification Subscription. To enable that
	/// the value is used as part of a URI, the string shall only contain
	/// characters allowed according to the "lower-with-hyphen" naming
	/// convention defined in 3GPP TS 29.501. In an OpenAPI schema, the format
	/// shall be designated as "SubId".
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies an Individual SMF Notification Subscription.
	/// To enable that the value is used as part of a URI, the string shall only
	/// contain characters allowed according to the \"lower-with-hyphen\" naming
	/// convention defined in 3GPP TS 29.501. In an OpenAPI schema, the format
	/// shall be designated as \"SubId\".\n",
	///  "type": "string",
	///  "format": "SubId"
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
	pub struct SubId(pub String);

	impl ::std::ops::Deref for SubId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SubId> for String {
		fn from(value: SubId) -> Self {
			value.0
		}
	}

	impl From<&SubId> for SubId {
		fn from(value: &SubId) -> Self {
			value.clone()
		}
	}

	impl From<String> for SubId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SubId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for SubId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// String identifying a Supi that shall contain either an IMSI, a network
	/// specific identifier, a Global Cable Identifier (GCI) or a Global
	/// Line Identifier (GLI) as specified in clause 2.2A of 3GPP TS 23.003.
	/// It shall be formatted as follows
	/// - for an IMSI "imsi-<imsi>", where <imsi> shall be formatted according
	///   to clause 2.2 of 3GPP TS 23.003 that describes an IMSI.
	/// - for a network specific identifier "nai-<nai>, where <nai> shall be
	///   formatted according to clause 28.7.2 of 3GPP TS 23.003 that describes
	///   an NAI.
	/// - for a GCI "gci-<gci>", where <gci> shall be formatted according to
	///   clause 28.15.2 of 3GPP TS 23.003.
	/// - for a GLI "gli-<gli>", where <gli> shall be formatted according to
	///   clause 28.16.2 of 3GPP TS 23.003.To enable that the value is used as
	///   part of an URI, the string shall only contain characters allowed
	///   according to the "lower-with-hyphen" naming convention defined in 3GPP
	///   TS 29.501.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a Supi that shall contain either an
	/// IMSI, a network specific identifier,\na Global Cable Identifier (GCI) or
	/// a Global Line Identifier (GLI) as specified in clause \n2.2A of 3GPP TS
	/// 23.003. It shall be formatted as follows\n - for an IMSI
	/// \"imsi-<imsi>\", where <imsi> shall be formatted according to clause
	/// 2.2\n   of 3GPP TS 23.003 that describes an IMSI.\n - for a network
	/// specific identifier \"nai-<nai>, where <nai> shall be formatted\n
	/// according to clause 28.7.2 of 3GPP TS 23.003 that describes an NAI.\n -
	/// for a GCI \"gci-<gci>\", where <gci> shall be formatted according to
	/// clause 28.15.2\n   of 3GPP TS 23.003.\n - for a GLI \"gli-<gli>\", where
	/// <gli> shall be formatted according to clause 28.16.2 of\n   3GPP TS
	/// 23.003.To enable that the value is used as part of an URI, the string
	/// shall\n   only contain characters allowed according to the
	/// \"lower-with-hyphen\" naming convention\n   defined in 3GPP TS
	/// 29.501.\n",
	///  "type": "string",
	///  "pattern": "^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$"
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
	)]
	pub struct Supi(String);

	impl ::std::ops::Deref for Supi {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Supi> for String {
		fn from(value: Supi) -> Self {
			value.0
		}
	}

	impl From<&Supi> for Supi {
		fn from(value: &Supi) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Supi {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \
				            \"^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Supi {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Supi {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Supi {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Supi {
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

	/// A string used to indicate the features supported by an API that is used
	/// as defined in clause  6.6 in 3GPP TS 29.500. The string shall contain a
	/// bitmask indicating supported features in  hexadecimal representation
	/// Each character in the string shall take a value of "0" to "9",  "a" to
	/// "f" or "A" to "F" and shall represent the support of 4 features as
	/// described in  table 5.2.2-3. The most significant character representing
	/// the highest-numbered features shall  appear first in the string, and the
	/// character representing features 1 to 4 shall appear last  in the string.
	/// The list of features and their numbering (starting with 1) are defined
	/// separately for each API. If the string contains a lower number of
	/// characters than there are  defined features for an API, all features
	/// that would be represented by characters that are not  present in the
	/// string are not supported.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A string used to indicate the features supported by an API that is used as defined in clause  6.6 in 3GPP TS 29.500. The string shall contain a bitmask indicating supported features in  hexadecimal representation Each character in the string shall take a value of \"0\" to \"9\",  \"a\" to \"f\" or \"A\" to \"F\" and shall represent the support of 4 features as described in  table 5.2.2-3. The most significant character representing the highest-numbered features shall  appear first in the string, and the character representing features 1 to 4 shall appear last  in the string. The list of features and their numbering (starting with 1) are defined  separately for each API. If the string contains a lower number of characters than there are  defined features for an API, all features that would be represented by characters that are not  present in the string are not supported.\n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]*$"
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
	)]
	pub struct SupportedFeatures(String);

	impl ::std::ops::Deref for SupportedFeatures {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SupportedFeatures> for String {
		fn from(value: SupportedFeatures) -> Self {
			value.0
		}
	}

	impl From<&SupportedFeatures> for SupportedFeatures {
		fn from(value: &SupportedFeatures) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SupportedFeatures {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]*$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]*$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SupportedFeatures {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SupportedFeatures {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SupportedFeatures {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SupportedFeatures {
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

	/// 2 or 3-octet string identifying a tracking area code as specified in
	/// clause 9.3.3.10  of 3GPP TS 38.413, in hexadecimal representation. Each
	/// character in the string shall  take a value of "0" to "9", "a" to "f" or
	/// "A" to "F" and shall represent 4 bits. The most significant character
	/// representing the 4 most significant bits of the TAC shall  appear first
	/// in the string, and the character representing the 4 least significant
	/// bit  of the TAC shall appear last in the string.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "2 or 3-octet string identifying a tracking area code as
	/// specified in clause 9.3.3.10  of 3GPP TS 38.413, in hexadecimal
	/// representation. Each character in the string shall  take a value of
	/// \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4
	/// bits. The most significant character representing the 4 most significant
	/// bits of the TAC shall  appear first in the string, and the character
	/// representing the 4 least significant bit  of the TAC shall appear last
	/// in the string. \n",
	///  "type": "string",
	///  "pattern": "(^[A-Fa-f0-9]{4}$)|(^[A-Fa-f0-9]{6}$)"
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
	)]
	pub struct Tac(String);

	impl ::std::ops::Deref for Tac {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Tac> for String {
		fn from(value: Tac) -> Self {
			value.0
		}
	}

	impl From<&Tac> for Tac {
		fn from(value: &Tac) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Tac {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("(^[A-Fa-f0-9]{4}$)|(^[A-Fa-f0-9]{6}$)")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err(
					"doesn't match pattern \"(^[A-Fa-f0-9]{4}$)|(^[A-Fa-f0-9]{6}$)\"".into(),
				);
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Tac {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Tac {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Tac {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Tac {
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

	/// Contains the tracking area identity as described in 3GPP 23.003
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the tracking area identity as described in
	/// 3GPP 23.003",
	///  "type": "object",
	///  "required": [
	///    "plmnId",
	///    "tac"
	///  ],
	///  "properties": {
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "tac": {
	///      "$ref": "#/components/schemas/Tac"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Tai {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		pub tac: Tac,
	}

	impl From<&Tai> for Tai {
		fn from(value: &Tai) -> Self {
			value.clone()
		}
	}

	/// Target DNAI Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Target DNAI Information",
	///  "type": "object",
	///  "required": [
	///    "smfSelectionType"
	///  ],
	///  "properties": {
	///    "smfSelectionType": {
	///      "$ref": "#/components/schemas/SmfSelectionType"
	///    },
	///    "targetDnai": {
	///      "$ref": "#/components/schemas/Dnai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TargetDnaiInfo {
		#[serde(rename = "smfSelectionType")]
		pub smf_selection_type: SmfSelectionType,
		#[serde(
			rename = "targetDnai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_dnai: Option<Dnai>,
	}

	impl From<&TargetDnaiInfo> for TargetDnaiInfo {
		fn from(value: &TargetDnaiInfo) -> Self {
			value.clone()
		}
	}

	/// GTP Tunnel Endpoint Identifier
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "GTP Tunnel Endpoint Identifier",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{8}$"
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
	)]
	pub struct Teid(String);

	impl ::std::ops::Deref for Teid {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Teid> for String {
		fn from(value: Teid) -> Self {
			value.0
		}
	}

	impl From<&Teid> for Teid {
		fn from(value: &Teid) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Teid {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{8}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{8}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Teid {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Teid {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Teid {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Teid {
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

	/// String with format partial-time or full-time as defined in clause 5.6 of
	/// IETF RFC 3339. Examples, 20:15:00, 20:15:00-08:00 (for 8 hours behind
	/// UTC).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String with format partial-time or full-time as defined
	/// in clause 5.6 of IETF RFC 3339. Examples, 20:15:00, 20:15:00-08:00 (for
	/// 8 hours behind UTC). \n",
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
	pub struct TimeOfDay(pub String);

	impl ::std::ops::Deref for TimeOfDay {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TimeOfDay> for String {
		fn from(value: TimeOfDay) -> Self {
			value.0
		}
	}

	impl From<&TimeOfDay> for TimeOfDay {
		fn from(value: &TimeOfDay) -> Self {
			value.clone()
		}
	}

	impl From<String> for TimeOfDay {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for TimeOfDay {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for TimeOfDay {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Represents a time window identified by a start time and a stop time.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a time window identified by a start time and
	/// a stop time.",
	///  "type": "object",
	///  "required": [
	///    "startTime",
	///    "stopTime"
	///  ],
	///  "properties": {
	///    "startTime": {
	///      "$ref": "#/components/schemas/schemas-DateTime"
	///    },
	///    "stopTime": {
	///      "$ref": "#/components/schemas/schemas-DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TimeWindow {
		#[serde(rename = "startTime")]
		pub start_time: SchemasDateTime,
		#[serde(rename = "stopTime")]
		pub stop_time: SchemasDateTime,
	}

	impl From<&TimeWindow> for TimeWindow {
		fn from(value: &TimeWindow) -> Self {
			value.clone()
		}
	}

	/// String with format "time-numoffset" optionally appended by
	/// "daylightSavingTime", where
	/// - "time-numoffset" shall represent the time zone adjusted for daylight
	///   saving time and be encoded as time-numoffset as defined in clause 5.6
	///   of IETF RFC 3339;
	/// - "daylightSavingTime" shall represent the adjustment that has been made
	///   and shall be encoded as "+1" or "+2" for a +1 or +2 hours adjustment.
	///
	/// The example is for 8 hours behind UTC, +1 hour adjustment for Daylight
	/// Saving Time.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String with format \"time-numoffset\" optionally
	/// appended by \"daylightSavingTime\", where \n- \"time-numoffset\" shall
	/// represent the time zone adjusted for daylight saving time and be\n
	/// encoded as time-numoffset as defined in clause 5.6 of IETF RFC 3339; \n-
	/// \"daylightSavingTime\" shall represent the adjustment that has been made
	/// and shall be\n   encoded as \"+1\" or \"+2\" for a +1 or +2 hours
	/// adjustment. \n\nThe example is for 8 hours behind UTC, +1 hour
	/// adjustment for Daylight Saving Time.\n",
	///  "examples": [
	///    "-08:00+1"
	///  ],
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
	pub struct TimeZone(pub String);

	impl ::std::ops::Deref for TimeZone {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TimeZone> for String {
		fn from(value: TimeZone) -> Self {
			value.0
		}
	}

	impl From<&TimeZone> for TimeZone {
		fn from(value: &TimeZone) -> Self {
			value.clone()
		}
	}

	impl From<String> for TimeZone {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for TimeZone {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for TimeZone {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contain the TNAP Identifier see clause5.6.2 of 3GPP TS 23.501.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contain the TNAP Identifier see clause5.6.2 of 3GPP TS
	/// 23.501.",
	///  "type": "object",
	///  "properties": {
	///    "bssId": {
	///      "description": "When present, it shall contain the BSSID of the
	/// access point to which the UE is attached, that is received over NGAP,
	/// see IEEE Std 802.11-2012. \n",
	///      "type": "string"
	///    },
	///    "civicAddress": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "ssId": {
	///      "description": "This IE shall be present if the UE is accessing the
	/// 5GC via a trusted WLAN access network.When present, it shall contain the
	/// SSID of the access point to which the UE is attached, that is received
	/// over NGAP,  see IEEE Std 802.11-2012. \n",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TnapId {
		/// When present, it shall contain the BSSID of the access point to
		/// which the UE is attached, that is received over NGAP, see IEEE Std
		/// 802.11-2012.
		#[serde(rename = "bssId", default, skip_serializing_if = "Option::is_none")]
		pub bss_id: Option<String>,
		#[serde(
			rename = "civicAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub civic_address: Option<Bytes>,
		/// This IE shall be present if the UE is accessing the 5GC via a
		/// trusted WLAN access network.When present, it shall contain the SSID
		/// of the access point to which the UE is attached, that is received
		/// over NGAP,  see IEEE Std 802.11-2012.
		#[serde(rename = "ssId", default, skip_serializing_if = "Option::is_none")]
		pub ss_id: Option<String>,
	}

	impl From<&TnapId> for TnapId {
		fn from(value: &TnapId) -> Self {
			value.clone()
		}
	}

	/// This represents the identifier of the TNGF ID as specified in clause
	/// 9.3.1.161 of  3GPP TS 38.413  in hexadecimal representation. Each
	/// character in the string shall take a value of "0" to "9", "a"  to "f" or
	/// "A" to "F" and shall represent 4 bits. The most significant character
	/// representing the  4 most significant bits of the TNGF ID shall appear
	/// first in the string, and the character  representing the 4 least
	/// significant bit of the TNGF ID shall appear last in the string.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This represents the identifier of the TNGF ID as
	/// specified in clause 9.3.1.161 of  3GPP TS 38.413  in hexadecimal
	/// representation. Each character in the string shall take a value of \"0\"
	/// to \"9\", \"a\"  to \"f\" or \"A\" to \"F\" and shall represent 4 bits.
	/// The most significant character representing the  4 most significant bits
	/// of the TNGF ID shall appear first in the string, and the character
	/// representing the 4 least significant bit of the TNGF ID shall appear
	/// last in the string. \n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]+$"
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
	)]
	pub struct TngfId(String);

	impl ::std::ops::Deref for TngfId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TngfId> for String {
		fn from(value: TngfId) -> Self {
			value.0
		}
	}

	impl From<&TngfId> for TngfId {
		fn from(value: &TngfId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TngfId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for TngfId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TngfId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TngfId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TngfId {
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

	/// Infomation of the TNGF endpoints
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Infomation of the TNGF endpoints",
	///  "type": "object",
	///  "properties": {
	///    "endpointFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "ipv4EndpointAddresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6EndpointAddresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TngfInfo {
		#[serde(
			rename = "endpointFqdn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub endpoint_fqdn: Option<Fqdn>,
		#[serde(
			rename = "ipv4EndpointAddresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_endpoint_addresses: Vec<Ipv4Addr>,
		#[serde(
			rename = "ipv6EndpointAddresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_endpoint_addresses: Vec<Ipv6Addr>,
	}

	impl From<&TngfInfo> for TngfInfo {
		fn from(value: &TngfInfo) -> Self {
			value.clone()
		}
	}

	/// contains Trace control and configuration parameters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "contains Trace control and configuration parameters.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "eventList",
	///    "neTypeList",
	///    "traceDepth",
	///    "traceRef"
	///  ],
	///  "properties": {
	///    "collectionEntityIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "collectionEntityIpv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "eventList": {
	///      "description": "Triggering events (see 3GPP TS 32.422).It shall be
	/// encoded as an octet string in hexadecimal representation. Each character
	/// in the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or
	/// \"A\" to \"F\" and shall represent 4 bits. The most significant
	/// character representing the 4 most significant bits shall appear first in
	/// the string, and the character representing the 4 least significant bit
	/// shall appear last in the string. Octets shall be coded according to 3GPP
	/// TS 32.422.\n",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]+$"
	///    },
	///    "interfaceList": {
	///      "description": "List of Interfaces (see 3GPP TS 32.422).It shall be
	/// encoded as an octet string in hexadecimal representation. Each character
	/// in the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or
	/// \"A\" to \"F\" and shall represent 4 bits. The most significant
	/// character representing the 4 most significant bits shall appear first in
	/// the string, and the character representing the  4 least significant bit
	/// shall appear last in the string. Octets shall be coded according to 3GPP
	/// TS 32.422. If this attribute is not present, all the interfaces
	/// applicable to the list of NE types indicated in the neTypeList attribute
	/// should be traced.\n",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]+$"
	///    },
	///    "neTypeList": {
	///      "description": "List of NE Types (see 3GPP TS 32.422).It shall be
	/// encoded as an octet string in hexadecimal representation. Each character
	/// in the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or
	/// \"A\" to \"F\" and shall represent 4 bits. The most significant
	/// character representing the 4 most significant bits shall Appear first in
	/// the string, and the character representing the 4 least significant bit
	/// shall appear last in the string.Octets shall be coded according to 3GPP
	/// TS 32.422.\n",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]+$"
	///    },
	///    "traceDepth": {
	///      "$ref": "#/components/schemas/TraceDepth"
	///    },
	///    "traceRef": {
	///      "description": "Trace Reference (see 3GPP TS 32.422).It shall be
	/// encoded as the concatenation of MCC, MNC and Trace ID as follows:
	/// <MCC><MNC>-<Trace ID> The Trace ID shall be encoded as a 3 octet string
	/// in hexadecimal representation. Each character in the Trace ID string
	/// shall take a value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\"
	/// and shall represent 4 bits. The most significant character representing
	/// the 4 most significant bits of the Trace ID shall appear first  in the
	/// string, and the character representing the 4 least significant bit of
	/// the Trace ID shall appear last in the string.\n",
	///      "type": "string",
	///      "pattern": "^[0-9]{3}[0-9]{2,3}-[A-Fa-f0-9]{6}$"
	///    },
	///    "traceReportingConsumerUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TraceData(pub Option<TraceDataInner>);

	impl ::std::ops::Deref for TraceData {
		type Target = Option<TraceDataInner>;
		fn deref(&self) -> &Option<TraceDataInner> {
			&self.0
		}
	}

	impl From<TraceData> for Option<TraceDataInner> {
		fn from(value: TraceData) -> Self {
			value.0
		}
	}

	impl From<&TraceData> for TraceData {
		fn from(value: &TraceData) -> Self {
			value.clone()
		}
	}

	impl From<Option<TraceDataInner>> for TraceData {
		fn from(value: Option<TraceDataInner>) -> Self {
			Self(value)
		}
	}

	/// contains Trace control and configuration parameters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "contains Trace control and configuration parameters.",
	///  "type": "object",
	///  "required": [
	///    "eventList",
	///    "neTypeList",
	///    "traceDepth",
	///    "traceRef"
	///  ],
	///  "properties": {
	///    "collectionEntityIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "collectionEntityIpv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "eventList": {
	///      "description": "Triggering events (see 3GPP TS 32.422).It shall be
	/// encoded as an octet string in hexadecimal representation. Each character
	/// in the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or
	/// \"A\" to \"F\" and shall represent 4 bits. The most significant
	/// character representing the 4 most significant bits shall appear first in
	/// the string, and the character representing the 4 least significant bit
	/// shall appear last in the string. Octets shall be coded according to 3GPP
	/// TS 32.422.\n",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]+$"
	///    },
	///    "interfaceList": {
	///      "description": "List of Interfaces (see 3GPP TS 32.422).It shall be
	/// encoded as an octet string in hexadecimal representation. Each character
	/// in the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or
	/// \"A\" to \"F\" and shall represent 4 bits. The most significant
	/// character representing the 4 most significant bits shall appear first in
	/// the string, and the character representing the  4 least significant bit
	/// shall appear last in the string. Octets shall be coded according to 3GPP
	/// TS 32.422. If this attribute is not present, all the interfaces
	/// applicable to the list of NE types indicated in the neTypeList attribute
	/// should be traced.\n",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]+$"
	///    },
	///    "neTypeList": {
	///      "description": "List of NE Types (see 3GPP TS 32.422).It shall be
	/// encoded as an octet string in hexadecimal representation. Each character
	/// in the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or
	/// \"A\" to \"F\" and shall represent 4 bits. The most significant
	/// character representing the 4 most significant bits shall Appear first in
	/// the string, and the character representing the 4 least significant bit
	/// shall appear last in the string.Octets shall be coded according to 3GPP
	/// TS 32.422.\n",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]+$"
	///    },
	///    "traceDepth": {
	///      "$ref": "#/components/schemas/TraceDepth"
	///    },
	///    "traceRef": {
	///      "description": "Trace Reference (see 3GPP TS 32.422).It shall be
	/// encoded as the concatenation of MCC, MNC and Trace ID as follows:
	/// <MCC><MNC>-<Trace ID> The Trace ID shall be encoded as a 3 octet string
	/// in hexadecimal representation. Each character in the Trace ID string
	/// shall take a value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\"
	/// and shall represent 4 bits. The most significant character representing
	/// the 4 most significant bits of the Trace ID shall appear first  in the
	/// string, and the character representing the 4 least significant bit of
	/// the Trace ID shall appear last in the string.\n",
	///      "type": "string",
	///      "pattern": "^[0-9]{3}[0-9]{2,3}-[A-Fa-f0-9]{6}$"
	///    },
	///    "traceReportingConsumerUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TraceDataInner {
		#[serde(
			rename = "collectionEntityIpv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub collection_entity_ipv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "collectionEntityIpv6Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub collection_entity_ipv6_addr: Option<Ipv6Addr>,
		/// Triggering events (see 3GPP TS 32.422).It shall be encoded as an
		/// octet string in hexadecimal representation. Each character in the
		/// string shall take a value of "0" to "9", "a" to "f" or "A" to "F"
		/// and shall represent 4 bits. The most significant character
		/// representing the 4 most significant bits shall appear first in the
		/// string, and the character representing the 4 least significant bit
		/// shall appear last in the string. Octets shall be coded according to
		/// 3GPP TS 32.422.
		#[serde(rename = "eventList")]
		pub event_list: TraceDataInnerEventList,
		/// List of Interfaces (see 3GPP TS 32.422).It shall be encoded as an
		/// octet string in hexadecimal representation. Each character in the
		/// string shall take a value of "0" to "9", "a" to "f" or "A" to "F"
		/// and shall represent 4 bits. The most significant character
		/// representing the 4 most significant bits shall appear first in the
		/// string, and the character representing the  4 least significant bit
		/// shall appear last in the string. Octets shall be coded according to
		/// 3GPP TS 32.422. If this attribute is not present, all the interfaces
		/// applicable to the list of NE types indicated in the neTypeList
		/// attribute should be traced.
		#[serde(
			rename = "interfaceList",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub interface_list: Option<TraceDataInnerInterfaceList>,
		/// List of NE Types (see 3GPP TS 32.422).It shall be encoded as an
		/// octet string in hexadecimal representation. Each character in the
		/// string shall take a value of "0" to "9", "a" to "f" or "A" to "F"
		/// and shall represent 4 bits. The most significant character
		/// representing the 4 most significant bits shall Appear first in the
		/// string, and the character representing the 4 least significant bit
		/// shall appear last in the string.Octets shall be coded according to
		/// 3GPP TS 32.422.
		#[serde(rename = "neTypeList")]
		pub ne_type_list: TraceDataInnerNeTypeList,
		#[serde(rename = "traceDepth")]
		pub trace_depth: TraceDepth,
		/// Trace Reference (see 3GPP TS 32.422).It shall be encoded as the
		/// concatenation of MCC, MNC and Trace ID as follows: <MCC><MNC>-<Trace
		/// ID> The Trace ID shall be encoded as a 3 octet string in hexadecimal
		/// representation. Each character in the Trace ID string shall take a
		/// value of "0" to "9", "a" to "f" or "A" to "F" and shall represent 4
		/// bits. The most significant character representing the 4 most
		/// significant bits of the Trace ID shall appear first  in the string,
		/// and the character representing the 4 least significant bit of the
		/// Trace ID shall appear last in the string.
		#[serde(rename = "traceRef")]
		pub trace_ref: TraceDataInnerTraceRef,
		#[serde(
			rename = "traceReportingConsumerUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub trace_reporting_consumer_uri: Option<Uri>,
	}

	impl From<&TraceDataInner> for TraceDataInner {
		fn from(value: &TraceDataInner) -> Self {
			value.clone()
		}
	}

	/// Triggering events (see 3GPP TS 32.422).It shall be encoded as an octet
	/// string in hexadecimal representation. Each character in the string shall
	/// take a value of "0" to "9", "a" to "f" or "A" to "F" and shall represent
	/// 4 bits. The most significant character representing the 4 most
	/// significant bits shall appear first in the string, and the character
	/// representing the 4 least significant bit shall appear last in the
	/// string. Octets shall be coded according to 3GPP TS 32.422.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Triggering events (see 3GPP TS 32.422).It shall be
	/// encoded as an octet string in hexadecimal representation. Each character
	/// in the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or
	/// \"A\" to \"F\" and shall represent 4 bits. The most significant
	/// character representing the 4 most significant bits shall appear first in
	/// the string, and the character representing the 4 least significant bit
	/// shall appear last in the string. Octets shall be coded according to 3GPP
	/// TS 32.422.\n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]+$"
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
	)]
	pub struct TraceDataInnerEventList(String);

	impl ::std::ops::Deref for TraceDataInnerEventList {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TraceDataInnerEventList> for String {
		fn from(value: TraceDataInnerEventList) -> Self {
			value.0
		}
	}

	impl From<&TraceDataInnerEventList> for TraceDataInnerEventList {
		fn from(value: &TraceDataInnerEventList) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TraceDataInnerEventList {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for TraceDataInnerEventList {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TraceDataInnerEventList {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TraceDataInnerEventList {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TraceDataInnerEventList {
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

	/// List of Interfaces (see 3GPP TS 32.422).It shall be encoded as an octet
	/// string in hexadecimal representation. Each character in the string shall
	/// take a value of "0" to "9", "a" to "f" or "A" to "F" and shall represent
	/// 4 bits. The most significant character representing the 4 most
	/// significant bits shall appear first in the string, and the character
	/// representing the  4 least significant bit shall appear last in the
	/// string. Octets shall be coded according to 3GPP TS 32.422. If this
	/// attribute is not present, all the interfaces applicable to the list of
	/// NE types indicated in the neTypeList attribute should be traced.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of Interfaces (see 3GPP TS 32.422).It shall be
	/// encoded as an octet string in hexadecimal representation. Each character
	/// in the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or
	/// \"A\" to \"F\" and shall represent 4 bits. The most significant
	/// character representing the 4 most significant bits shall appear first in
	/// the string, and the character representing the  4 least significant bit
	/// shall appear last in the string. Octets shall be coded according to 3GPP
	/// TS 32.422. If this attribute is not present, all the interfaces
	/// applicable to the list of NE types indicated in the neTypeList attribute
	/// should be traced.\n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]+$"
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
	)]
	pub struct TraceDataInnerInterfaceList(String);

	impl ::std::ops::Deref for TraceDataInnerInterfaceList {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TraceDataInnerInterfaceList> for String {
		fn from(value: TraceDataInnerInterfaceList) -> Self {
			value.0
		}
	}

	impl From<&TraceDataInnerInterfaceList> for TraceDataInnerInterfaceList {
		fn from(value: &TraceDataInnerInterfaceList) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TraceDataInnerInterfaceList {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for TraceDataInnerInterfaceList {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TraceDataInnerInterfaceList {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TraceDataInnerInterfaceList {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TraceDataInnerInterfaceList {
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

	/// List of NE Types (see 3GPP TS 32.422).It shall be encoded as an octet
	/// string in hexadecimal representation. Each character in the string shall
	/// take a value of "0" to "9", "a" to "f" or "A" to "F" and shall represent
	/// 4 bits. The most significant character representing the 4 most
	/// significant bits shall Appear first in the string, and the character
	/// representing the 4 least significant bit shall appear last in the
	/// string.Octets shall be coded according to 3GPP TS 32.422.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of NE Types (see 3GPP TS 32.422).It shall be
	/// encoded as an octet string in hexadecimal representation. Each character
	/// in the string shall take a value of \"0\" to \"9\", \"a\" to \"f\" or
	/// \"A\" to \"F\" and shall represent 4 bits. The most significant
	/// character representing the 4 most significant bits shall Appear first in
	/// the string, and the character representing the 4 least significant bit
	/// shall appear last in the string.Octets shall be coded according to 3GPP
	/// TS 32.422.\n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]+$"
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
	)]
	pub struct TraceDataInnerNeTypeList(String);

	impl ::std::ops::Deref for TraceDataInnerNeTypeList {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TraceDataInnerNeTypeList> for String {
		fn from(value: TraceDataInnerNeTypeList) -> Self {
			value.0
		}
	}

	impl From<&TraceDataInnerNeTypeList> for TraceDataInnerNeTypeList {
		fn from(value: &TraceDataInnerNeTypeList) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TraceDataInnerNeTypeList {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for TraceDataInnerNeTypeList {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TraceDataInnerNeTypeList {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TraceDataInnerNeTypeList {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TraceDataInnerNeTypeList {
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

	/// Trace Reference (see 3GPP TS 32.422).It shall be encoded as the
	/// concatenation of MCC, MNC and Trace ID as follows: <MCC><MNC>-<Trace ID>
	/// The Trace ID shall be encoded as a 3 octet string in hexadecimal
	/// representation. Each character in the Trace ID string shall take a value
	/// of "0" to "9", "a" to "f" or "A" to "F" and shall represent 4 bits. The
	/// most significant character representing the 4 most significant bits of
	/// the Trace ID shall appear first  in the string, and the character
	/// representing the 4 least significant bit of the Trace ID shall appear
	/// last in the string.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Trace Reference (see 3GPP TS 32.422).It shall be
	/// encoded as the concatenation of MCC, MNC and Trace ID as follows:
	/// <MCC><MNC>-<Trace ID> The Trace ID shall be encoded as a 3 octet string
	/// in hexadecimal representation. Each character in the Trace ID string
	/// shall take a value of \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\"
	/// and shall represent 4 bits. The most significant character representing
	/// the 4 most significant bits of the Trace ID shall appear first  in the
	/// string, and the character representing the 4 least significant bit of
	/// the Trace ID shall appear last in the string.\n",
	///  "type": "string",
	///  "pattern": "^[0-9]{3}[0-9]{2,3}-[A-Fa-f0-9]{6}$"
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
	)]
	pub struct TraceDataInnerTraceRef(String);

	impl ::std::ops::Deref for TraceDataInnerTraceRef {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TraceDataInnerTraceRef> for String {
		fn from(value: TraceDataInnerTraceRef) -> Self {
			value.0
		}
	}

	impl From<&TraceDataInnerTraceRef> for TraceDataInnerTraceRef {
		fn from(value: &TraceDataInnerTraceRef) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TraceDataInnerTraceRef {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{3}[0-9]{2,3}-[A-Fa-f0-9]{6}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{3}[0-9]{2,3}-[A-Fa-f0-9]{6}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for TraceDataInnerTraceRef {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TraceDataInnerTraceRef {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TraceDataInnerTraceRef {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TraceDataInnerTraceRef {
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

	/// The enumeration TraceDepth defines how detailed information should be
	/// recorded in the trace. See 3GPP TS 32.422 for further description of the
	/// values. It shall comply with the provisions defined in table 5.6.3.1-1
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration TraceDepth defines how detailed
	/// information should be recorded in the trace. See 3GPP TS 32.422 for
	/// further description of the values. It shall comply with the provisions
	/// defined in table 5.6.3.1-1\n",
	///  "type": "string",
	///  "enum": [
	///    "MINIMUM",
	///    "MEDIUM",
	///    "MAXIMUM",
	///    "MINIMUM_WO_VENDOR_EXTENSION",
	///    "MEDIUM_WO_VENDOR_EXTENSION",
	///    "MAXIMUM_WO_VENDOR_EXTENSION"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum TraceDepth {
		#[default]
		#[serde(rename = "MINIMUM")]
		Minimum,
		#[serde(rename = "MEDIUM")]
		Medium,
		#[serde(rename = "MAXIMUM")]
		Maximum,
		#[serde(rename = "MINIMUM_WO_VENDOR_EXTENSION")]
		MinimumWoVendorExtension,
		#[serde(rename = "MEDIUM_WO_VENDOR_EXTENSION")]
		MediumWoVendorExtension,
		#[serde(rename = "MAXIMUM_WO_VENDOR_EXTENSION")]
		MaximumWoVendorExtension,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&TraceDepth> for TraceDepth {
		fn from(value: &TraceDepth) -> Self {
			value.clone()
		}
	}

	impl ToString for TraceDepth {
		fn to_string(&self) -> String {
			match *self {
				Self::Minimum => "MINIMUM".to_string(),
				Self::Medium => "MEDIUM".to_string(),
				Self::Maximum => "MAXIMUM".to_string(),
				Self::MinimumWoVendorExtension => "MINIMUM_WO_VENDOR_EXTENSION".to_string(),
				Self::MediumWoVendorExtension => "MEDIUM_WO_VENDOR_EXTENSION".to_string(),
				Self::MaximumWoVendorExtension => "MAXIMUM_WO_VENDOR_EXTENSION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TraceDepth {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MINIMUM" => Ok(Self::Minimum),
				"MEDIUM" => Ok(Self::Medium),
				"MAXIMUM" => Ok(Self::Maximum),
				"MINIMUM_WO_VENDOR_EXTENSION" => Ok(Self::MinimumWoVendorExtension),
				"MEDIUM_WO_VENDOR_EXTENSION" => Ok(Self::MediumWoVendorExtension),
				"MAXIMUM_WO_VENDOR_EXTENSION" => Ok(Self::MaximumWoVendorExtension),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for TraceDepth {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TraceDepth {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TraceDepth {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - SINGLE_TRANS_UL: Uplink single packet transmission.
	/// - SINGLE_TRANS_DL: Downlink single packet transmission.
	/// - DUAL_TRANS_UL_FIRST: Dual packet transmission, firstly uplink packet
	///   transmission
	///  with subsequent downlink packet transmission.
	/// - DUAL_TRANS_DL_FIRST: Dual packet transmission, firstly downlink packet
	///   transmission
	///  with subsequent uplink packet transmission.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- SINGLE_TRANS_UL: Uplink single
	/// packet transmission.\n- SINGLE_TRANS_DL: Downlink single packet
	/// transmission.\n- DUAL_TRANS_UL_FIRST: Dual packet transmission, firstly
	/// uplink packet transmission\n  with subsequent downlink packet
	/// transmission.\n- DUAL_TRANS_DL_FIRST: Dual packet transmission, firstly
	/// downlink packet transmission\n  with subsequent uplink packet
	/// transmission. \n",
	///  "type": "string",
	///  "enum": [
	///    "SINGLE_TRANS_UL",
	///    "SINGLE_TRANS_DL",
	///    "DUAL_TRANS_UL_FIRST",
	///    "DUAL_TRANS_DL_FIRST",
	///    "MULTI_TRANS"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum TrafficProfile {
		#[default]
		#[serde(rename = "SINGLE_TRANS_UL")]
		SingleTransUl,
		#[serde(rename = "SINGLE_TRANS_DL")]
		SingleTransDl,
		#[serde(rename = "DUAL_TRANS_UL_FIRST")]
		DualTransUlFirst,
		#[serde(rename = "DUAL_TRANS_DL_FIRST")]
		DualTransDlFirst,
		#[serde(rename = "MULTI_TRANS")]
		MultiTrans,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&TrafficProfile> for TrafficProfile {
		fn from(value: &TrafficProfile) -> Self {
			value.clone()
		}
	}

	impl ToString for TrafficProfile {
		fn to_string(&self) -> String {
			match *self {
				Self::SingleTransUl => "SINGLE_TRANS_UL".to_string(),
				Self::SingleTransDl => "SINGLE_TRANS_DL".to_string(),
				Self::DualTransUlFirst => "DUAL_TRANS_UL_FIRST".to_string(),
				Self::DualTransDlFirst => "DUAL_TRANS_DL_FIRST".to_string(),
				Self::MultiTrans => "MULTI_TRANS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TrafficProfile {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SINGLE_TRANS_UL" => Ok(Self::SingleTransUl),
				"SINGLE_TRANS_DL" => Ok(Self::SingleTransDl),
				"DUAL_TRANS_UL_FIRST" => Ok(Self::DualTransUlFirst),
				"DUAL_TRANS_DL_FIRST" => Ok(Self::DualTransDlFirst),
				"MULTI_TRANS" => Ok(Self::MultiTrans),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for TrafficProfile {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TrafficProfile {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TrafficProfile {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents SMF Transaction Information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents SMF Transaction Information.",
	///  "type": "object",
	///  "required": [
	///    "transaction"
	///  ],
	///  "properties": {
	///    "appIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ApplicationId"
	///      },
	///      "minItems": 1
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "transacMetrics": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TransactionMetric"
	///      },
	///      "minItems": 1
	///    },
	///    "transaction": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TransactionInfo {
		#[serde(rename = "appIds", default, skip_serializing_if = "Vec::is_empty")]
		pub app_ids: Vec<ApplicationId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(
			rename = "transacMetrics",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub transac_metrics: Vec<TransactionMetric>,
		pub transaction: Uinteger,
	}

	impl From<&TransactionInfo> for TransactionInfo {
		fn from(value: &TransactionInfo) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - PDU_SES_EST: PDU Session Establishment
	/// - PDU_SES_AUTH: PDU Session Authentication
	/// - PDU_SES_MODIF: PDU Session Modification
	/// - PDU_SES_REL: PDU Session Release
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- PDU_SES_EST: PDU Session
	/// Establishment\n- PDU_SES_AUTH: PDU Session Authentication\n-
	/// PDU_SES_MODIF: PDU Session Modification\n- PDU_SES_REL: PDU Session
	/// Release\n",
	///  "type": "string",
	///  "enum": [
	///    "PDU_SES_EST",
	///    "PDU_SES_AUTH",
	///    "PDU_SES_MODIF",
	///    "PDU_SES_REL"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum TransactionMetric {
		#[default]
		#[serde(rename = "PDU_SES_EST")]
		PduSesEst,
		#[serde(rename = "PDU_SES_AUTH")]
		PduSesAuth,
		#[serde(rename = "PDU_SES_MODIF")]
		PduSesModif,
		#[serde(rename = "PDU_SES_REL")]
		PduSesRel,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&TransactionMetric> for TransactionMetric {
		fn from(value: &TransactionMetric) -> Self {
			value.clone()
		}
	}

	impl ToString for TransactionMetric {
		fn to_string(&self) -> String {
			match *self {
				Self::PduSesEst => "PDU_SES_EST".to_string(),
				Self::PduSesAuth => "PDU_SES_AUTH".to_string(),
				Self::PduSesModif => "PDU_SES_MODIF".to_string(),
				Self::PduSesRel => "PDU_SES_REL".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TransactionMetric {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PDU_SES_EST" => Ok(Self::PduSesEst),
				"PDU_SES_AUTH" => Ok(Self::PduSesAuth),
				"PDU_SES_MODIF" => Ok(Self::PduSesModif),
				"PDU_SES_REL" => Ok(Self::PduSesRel),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for TransactionMetric {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TransactionMetric {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TransactionMetric {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within Transfer MO Data Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Transfer MO Data Request",
	///  "type": "object",
	///  "required": [
	///    "moData"
	///  ],
	///  "properties": {
	///    "moData": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "moExpDataCounter": {
	///      "$ref": "#/components/schemas/MoExpDataCounter"
	///    },
	///    "ueLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TransferMoDataReqData {
		#[serde(rename = "moData")]
		pub mo_data: RefToBinaryData,
		#[serde(
			rename = "moExpDataCounter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mo_exp_data_counter: Option<MoExpDataCounter>,
		#[serde(
			rename = "ueLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_location: Option<UserLocation>,
	}

	impl From<&TransferMoDataReqData> for TransferMoDataReqData {
		fn from(value: &TransferMoDataReqData) -> Self {
			value.clone()
		}
	}

	/// Transfer MT Data Error Response Additional Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Transfer MT Data Error Response Additional
	/// Information",
	///  "type": "object",
	///  "properties": {
	///    "maxWaitingTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TransferMtDataAddInfo {
		#[serde(
			rename = "maxWaitingTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_waiting_time: Option<DurationSec>,
	}

	impl From<&TransferMtDataAddInfo> for TransferMtDataAddInfo {
		fn from(value: &TransferMtDataAddInfo) -> Self {
			value.clone()
		}
	}

	/// Transfer MT Data Error Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Transfer MT Data Error Response",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/ExtProblemDetails"
	///    },
	///    {
	///      "$ref": "#/components/schemas/TransferMtDataAddInfo"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TransferMtDataError {
		#[serde(
			rename = "accessTokenError",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_token_error: Option<AccessTokenErr>,
		#[serde(
			rename = "accessTokenRequest",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_token_request: Option<AccessTokenReq>,
		/// A machine-readable application error cause specific to this
		/// occurrence of the problem.  This IE should be present and provide
		/// application-related error information, if available.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<String>,
		/// A human-readable explanation specific to this occurrence of the
		/// problem.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub detail: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub instance: Option<Uri>,
		#[serde(
			rename = "invalidParams",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub invalid_params: Vec<InvalidParam>,
		#[serde(
			rename = "maxWaitingTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_waiting_time: Option<DurationSec>,
		#[serde(rename = "nrfId", default, skip_serializing_if = "Option::is_none")]
		pub nrf_id: Option<Fqdn>,
		#[serde(
			rename = "remoteError",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remote_error: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub status: Option<i64>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub title: Option<String>,
		#[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
		pub type_: Option<Uri>,
	}

	impl From<&TransferMtDataError> for TransferMtDataError {
		fn from(value: &TransferMtDataError) -> Self {
			value.clone()
		}
	}

	/// Data within Transfer MT Data Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Transfer MT Data Request",
	///  "type": "object",
	///  "required": [
	///    "mtData"
	///  ],
	///  "properties": {
	///    "mtData": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TransferMtDataReqData {
		#[serde(rename = "mtData")]
		pub mt_data: RefToBinaryData,
	}

	impl From<&TransferMtDataReqData> for TransferMtDataReqData {
		fn from(value: &TransferMtDataReqData) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - UDP: User Datagram Protocol.
	/// - TCP: Transmission Control Protocol.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- UDP: User Datagram Protocol.\n-
	/// TCP: Transmission Control Protocol. \n",
	///  "type": "string",
	///  "enum": [
	///    "UDP",
	///    "TCP"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum TransportProtocol {
		#[default]
		#[serde(rename = "UDP")]
		Udp,
		#[serde(rename = "TCP")]
		Tcp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&TransportProtocol> for TransportProtocol {
		fn from(value: &TransportProtocol) -> Self {
			value.clone()
		}
	}

	impl ToString for TransportProtocol {
		fn to_string(&self) -> String {
			match *self {
				Self::Udp => "UDP".to_string(),
				Self::Tcp => "TCP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TransportProtocol {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UDP" => Ok(Self::Udp),
				"TCP" => Ok(Self::Tcp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for TransportProtocol {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TransportProtocol {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TransportProtocol {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Trigger
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "triggerCategory"
	///  ],
	///  "properties": {
	///    "eventLimit": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "maxNumberOfccc": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "tariffTimeChange": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "timeLimit": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "triggerCategory": {
	///      "$ref": "#/components/schemas/TriggerCategory"
	///    },
	///    "triggerType": {
	///      "$ref": "#/components/schemas/TriggerType"
	///    },
	///    "volumeLimit": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "volumeLimit64": {
	///      "$ref": "#/components/schemas/Uint64"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Trigger {
		#[serde(
			rename = "eventLimit",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_limit: Option<Uint32>,
		#[serde(
			rename = "maxNumberOfccc",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_number_ofccc: Option<Uint32>,
		#[serde(
			rename = "tariffTimeChange",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tariff_time_change: Option<DateTime>,
		#[serde(rename = "timeLimit", default, skip_serializing_if = "Option::is_none")]
		pub time_limit: Option<DurationSec>,
		#[serde(rename = "triggerCategory")]
		pub trigger_category: TriggerCategory,
		#[serde(
			rename = "triggerType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub trigger_type: Option<TriggerType>,
		#[serde(
			rename = "volumeLimit",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub volume_limit: Option<Uint32>,
		#[serde(
			rename = "volumeLimit64",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub volume_limit64: Option<Uint64>,
	}

	impl From<&Trigger> for Trigger {
		fn from(value: &Trigger) -> Self {
			value.clone()
		}
	}

	/// TriggerCategory
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "IMMEDIATE_REPORT",
	///    "DEFERRED_REPORT"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum TriggerCategory {
		#[default]
		#[serde(rename = "IMMEDIATE_REPORT")]
		ImmediateReport,
		#[serde(rename = "DEFERRED_REPORT")]
		DeferredReport,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&TriggerCategory> for TriggerCategory {
		fn from(value: &TriggerCategory) -> Self {
			value.clone()
		}
	}

	impl ToString for TriggerCategory {
		fn to_string(&self) -> String {
			match *self {
				Self::ImmediateReport => "IMMEDIATE_REPORT".to_string(),
				Self::DeferredReport => "DEFERRED_REPORT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TriggerCategory {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IMMEDIATE_REPORT" => Ok(Self::ImmediateReport),
				"DEFERRED_REPORT" => Ok(Self::DeferredReport),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for TriggerCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TriggerCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TriggerCategory {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// TriggerType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "QUOTA_THRESHOLD",
	///    "QHT",
	///    "FINAL",
	///    "QUOTA_EXHAUSTED",
	///    "VALIDITY_TIME",
	///    "OTHER_QUOTA_TYPE",
	///    "FORCED_REAUTHORISATION",
	///    "UNUSED_QUOTA_TIMER",
	///    "UNIT_COUNT_INACTIVITY_TIMER",
	///    "ABNORMAL_RELEASE",
	///    "QOS_CHANGE",
	///    "VOLUME_LIMIT",
	///    "TIME_LIMIT",
	///    "EVENT_LIMIT",
	///    "PLMN_CHANGE",
	///    "USER_LOCATION_CHANGE",
	///    "RAT_CHANGE",
	///    "SESSION_AMBR_CHANGE",
	///    "UE_TIMEZONE_CHANGE",
	///    "TARIFF_TIME_CHANGE",
	///    "MAX_NUMBER_OF_CHANGES_IN_CHARGING_CONDITIONS",
	///    "MANAGEMENT_INTERVENTION",
	///    "CHANGE_OF_UE_PRESENCE_IN_PRESENCE_REPORTING_AREA",
	///    "CHANGE_OF_3GPP_PS_DATA_OFF_STATUS",
	///    "SERVING_NODE_CHANGE",
	///    "REMOVAL_OF_UPF",
	///    "ADDITION_OF_UPF",
	///    "INSERTION_OF_ISMF",
	///    "REMOVAL_OF_ISMF",
	///    "CHANGE_OF_ISMF",
	///    "START_OF_SERVICE_DATA_FLOW",
	///    "ECGI_CHANGE",
	///    "TAI_CHANGE",
	///    "HANDOVER_CANCEL",
	///    "HANDOVER_START",
	///    "HANDOVER_COMPLETE",
	///    "GFBR_GUARANTEED_STATUS_CHANGE",
	///    "ADDITION_OF_ACCESS",
	///    "REMOVAL_OF_ACCESS",
	///    "START_OF_SDF_ADDITIONAL_ACCESS",
	///    "REDUNDANT_TRANSMISSION_CHANGE",
	///    "CGI_SAI_CHANGE",
	///    "RAI_CHANGE",
	///    "VSMF_CHANGE"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum TriggerType {
		#[default]
		#[serde(rename = "QUOTA_THRESHOLD")]
		QuotaThreshold,
		#[serde(rename = "QHT")]
		Qht,
		#[serde(rename = "FINAL")]
		Final,
		#[serde(rename = "QUOTA_EXHAUSTED")]
		QuotaExhausted,
		#[serde(rename = "VALIDITY_TIME")]
		ValidityTime,
		#[serde(rename = "OTHER_QUOTA_TYPE")]
		OtherQuotaType,
		#[serde(rename = "FORCED_REAUTHORISATION")]
		ForcedReauthorisation,
		#[serde(rename = "UNUSED_QUOTA_TIMER")]
		UnusedQuotaTimer,
		#[serde(rename = "UNIT_COUNT_INACTIVITY_TIMER")]
		UnitCountInactivityTimer,
		#[serde(rename = "ABNORMAL_RELEASE")]
		AbnormalRelease,
		#[serde(rename = "QOS_CHANGE")]
		QosChange,
		#[serde(rename = "VOLUME_LIMIT")]
		VolumeLimit,
		#[serde(rename = "TIME_LIMIT")]
		TimeLimit,
		#[serde(rename = "EVENT_LIMIT")]
		EventLimit,
		#[serde(rename = "PLMN_CHANGE")]
		PlmnChange,
		#[serde(rename = "USER_LOCATION_CHANGE")]
		UserLocationChange,
		#[serde(rename = "RAT_CHANGE")]
		RatChange,
		#[serde(rename = "SESSION_AMBR_CHANGE")]
		SessionAmbrChange,
		#[serde(rename = "UE_TIMEZONE_CHANGE")]
		UeTimezoneChange,
		#[serde(rename = "TARIFF_TIME_CHANGE")]
		TariffTimeChange,
		#[serde(rename = "MAX_NUMBER_OF_CHANGES_IN_CHARGING_CONDITIONS")]
		MaxNumberOfChangesInChargingConditions,
		#[serde(rename = "MANAGEMENT_INTERVENTION")]
		ManagementIntervention,
		#[serde(rename = "CHANGE_OF_UE_PRESENCE_IN_PRESENCE_REPORTING_AREA")]
		ChangeOfUePresenceInPresenceReportingArea,
		#[serde(rename = "CHANGE_OF_3GPP_PS_DATA_OFF_STATUS")]
		ChangeOf3gppPsDataOffStatus,
		#[serde(rename = "SERVING_NODE_CHANGE")]
		ServingNodeChange,
		#[serde(rename = "REMOVAL_OF_UPF")]
		RemovalOfUpf,
		#[serde(rename = "ADDITION_OF_UPF")]
		AdditionOfUpf,
		#[serde(rename = "INSERTION_OF_ISMF")]
		InsertionOfIsmf,
		#[serde(rename = "REMOVAL_OF_ISMF")]
		RemovalOfIsmf,
		#[serde(rename = "CHANGE_OF_ISMF")]
		ChangeOfIsmf,
		#[serde(rename = "START_OF_SERVICE_DATA_FLOW")]
		StartOfServiceDataFlow,
		#[serde(rename = "ECGI_CHANGE")]
		EcgiChange,
		#[serde(rename = "TAI_CHANGE")]
		TaiChange,
		#[serde(rename = "HANDOVER_CANCEL")]
		HandoverCancel,
		#[serde(rename = "HANDOVER_START")]
		HandoverStart,
		#[serde(rename = "HANDOVER_COMPLETE")]
		HandoverComplete,
		#[serde(rename = "GFBR_GUARANTEED_STATUS_CHANGE")]
		GfbrGuaranteedStatusChange,
		#[serde(rename = "ADDITION_OF_ACCESS")]
		AdditionOfAccess,
		#[serde(rename = "REMOVAL_OF_ACCESS")]
		RemovalOfAccess,
		#[serde(rename = "START_OF_SDF_ADDITIONAL_ACCESS")]
		StartOfSdfAdditionalAccess,
		#[serde(rename = "REDUNDANT_TRANSMISSION_CHANGE")]
		RedundantTransmissionChange,
		#[serde(rename = "CGI_SAI_CHANGE")]
		CgiSaiChange,
		#[serde(rename = "RAI_CHANGE")]
		RaiChange,
		#[serde(rename = "VSMF_CHANGE")]
		VsmfChange,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&TriggerType> for TriggerType {
		fn from(value: &TriggerType) -> Self {
			value.clone()
		}
	}

	impl ToString for TriggerType {
		fn to_string(&self) -> String {
			match *self {
				Self::QuotaThreshold => "QUOTA_THRESHOLD".to_string(),
				Self::Qht => "QHT".to_string(),
				Self::Final => "FINAL".to_string(),
				Self::QuotaExhausted => "QUOTA_EXHAUSTED".to_string(),
				Self::ValidityTime => "VALIDITY_TIME".to_string(),
				Self::OtherQuotaType => "OTHER_QUOTA_TYPE".to_string(),
				Self::ForcedReauthorisation => "FORCED_REAUTHORISATION".to_string(),
				Self::UnusedQuotaTimer => "UNUSED_QUOTA_TIMER".to_string(),
				Self::UnitCountInactivityTimer => "UNIT_COUNT_INACTIVITY_TIMER".to_string(),
				Self::AbnormalRelease => "ABNORMAL_RELEASE".to_string(),
				Self::QosChange => "QOS_CHANGE".to_string(),
				Self::VolumeLimit => "VOLUME_LIMIT".to_string(),
				Self::TimeLimit => "TIME_LIMIT".to_string(),
				Self::EventLimit => "EVENT_LIMIT".to_string(),
				Self::PlmnChange => "PLMN_CHANGE".to_string(),
				Self::UserLocationChange => "USER_LOCATION_CHANGE".to_string(),
				Self::RatChange => "RAT_CHANGE".to_string(),
				Self::SessionAmbrChange => "SESSION_AMBR_CHANGE".to_string(),
				Self::UeTimezoneChange => "UE_TIMEZONE_CHANGE".to_string(),
				Self::TariffTimeChange => "TARIFF_TIME_CHANGE".to_string(),
				Self::MaxNumberOfChangesInChargingConditions => {
					"MAX_NUMBER_OF_CHANGES_IN_CHARGING_CONDITIONS".to_string()
				}
				Self::ManagementIntervention => "MANAGEMENT_INTERVENTION".to_string(),
				Self::ChangeOfUePresenceInPresenceReportingArea => {
					"CHANGE_OF_UE_PRESENCE_IN_PRESENCE_REPORTING_AREA".to_string()
				}
				Self::ChangeOf3gppPsDataOffStatus => {
					"CHANGE_OF_3GPP_PS_DATA_OFF_STATUS".to_string()
				}
				Self::ServingNodeChange => "SERVING_NODE_CHANGE".to_string(),
				Self::RemovalOfUpf => "REMOVAL_OF_UPF".to_string(),
				Self::AdditionOfUpf => "ADDITION_OF_UPF".to_string(),
				Self::InsertionOfIsmf => "INSERTION_OF_ISMF".to_string(),
				Self::RemovalOfIsmf => "REMOVAL_OF_ISMF".to_string(),
				Self::ChangeOfIsmf => "CHANGE_OF_ISMF".to_string(),
				Self::StartOfServiceDataFlow => "START_OF_SERVICE_DATA_FLOW".to_string(),
				Self::EcgiChange => "ECGI_CHANGE".to_string(),
				Self::TaiChange => "TAI_CHANGE".to_string(),
				Self::HandoverCancel => "HANDOVER_CANCEL".to_string(),
				Self::HandoverStart => "HANDOVER_START".to_string(),
				Self::HandoverComplete => "HANDOVER_COMPLETE".to_string(),
				Self::GfbrGuaranteedStatusChange => "GFBR_GUARANTEED_STATUS_CHANGE".to_string(),
				Self::AdditionOfAccess => "ADDITION_OF_ACCESS".to_string(),
				Self::RemovalOfAccess => "REMOVAL_OF_ACCESS".to_string(),
				Self::StartOfSdfAdditionalAccess => "START_OF_SDF_ADDITIONAL_ACCESS".to_string(),
				Self::RedundantTransmissionChange => "REDUNDANT_TRANSMISSION_CHANGE".to_string(),
				Self::CgiSaiChange => "CGI_SAI_CHANGE".to_string(),
				Self::RaiChange => "RAI_CHANGE".to_string(),
				Self::VsmfChange => "VSMF_CHANGE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TriggerType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"QUOTA_THRESHOLD" => Ok(Self::QuotaThreshold),
				"QHT" => Ok(Self::Qht),
				"FINAL" => Ok(Self::Final),
				"QUOTA_EXHAUSTED" => Ok(Self::QuotaExhausted),
				"VALIDITY_TIME" => Ok(Self::ValidityTime),
				"OTHER_QUOTA_TYPE" => Ok(Self::OtherQuotaType),
				"FORCED_REAUTHORISATION" => Ok(Self::ForcedReauthorisation),
				"UNUSED_QUOTA_TIMER" => Ok(Self::UnusedQuotaTimer),
				"UNIT_COUNT_INACTIVITY_TIMER" => Ok(Self::UnitCountInactivityTimer),
				"ABNORMAL_RELEASE" => Ok(Self::AbnormalRelease),
				"QOS_CHANGE" => Ok(Self::QosChange),
				"VOLUME_LIMIT" => Ok(Self::VolumeLimit),
				"TIME_LIMIT" => Ok(Self::TimeLimit),
				"EVENT_LIMIT" => Ok(Self::EventLimit),
				"PLMN_CHANGE" => Ok(Self::PlmnChange),
				"USER_LOCATION_CHANGE" => Ok(Self::UserLocationChange),
				"RAT_CHANGE" => Ok(Self::RatChange),
				"SESSION_AMBR_CHANGE" => Ok(Self::SessionAmbrChange),
				"UE_TIMEZONE_CHANGE" => Ok(Self::UeTimezoneChange),
				"TARIFF_TIME_CHANGE" => Ok(Self::TariffTimeChange),
				"MAX_NUMBER_OF_CHANGES_IN_CHARGING_CONDITIONS" => {
					Ok(Self::MaxNumberOfChangesInChargingConditions)
				}
				"MANAGEMENT_INTERVENTION" => Ok(Self::ManagementIntervention),
				"CHANGE_OF_UE_PRESENCE_IN_PRESENCE_REPORTING_AREA" => {
					Ok(Self::ChangeOfUePresenceInPresenceReportingArea)
				}
				"CHANGE_OF_3GPP_PS_DATA_OFF_STATUS" => Ok(Self::ChangeOf3gppPsDataOffStatus),
				"SERVING_NODE_CHANGE" => Ok(Self::ServingNodeChange),
				"REMOVAL_OF_UPF" => Ok(Self::RemovalOfUpf),
				"ADDITION_OF_UPF" => Ok(Self::AdditionOfUpf),
				"INSERTION_OF_ISMF" => Ok(Self::InsertionOfIsmf),
				"REMOVAL_OF_ISMF" => Ok(Self::RemovalOfIsmf),
				"CHANGE_OF_ISMF" => Ok(Self::ChangeOfIsmf),
				"START_OF_SERVICE_DATA_FLOW" => Ok(Self::StartOfServiceDataFlow),
				"ECGI_CHANGE" => Ok(Self::EcgiChange),
				"TAI_CHANGE" => Ok(Self::TaiChange),
				"HANDOVER_CANCEL" => Ok(Self::HandoverCancel),
				"HANDOVER_START" => Ok(Self::HandoverStart),
				"HANDOVER_COMPLETE" => Ok(Self::HandoverComplete),
				"GFBR_GUARANTEED_STATUS_CHANGE" => Ok(Self::GfbrGuaranteedStatusChange),
				"ADDITION_OF_ACCESS" => Ok(Self::AdditionOfAccess),
				"REMOVAL_OF_ACCESS" => Ok(Self::RemovalOfAccess),
				"START_OF_SDF_ADDITIONAL_ACCESS" => Ok(Self::StartOfSdfAdditionalAccess),
				"REDUNDANT_TRANSMISSION_CHANGE" => Ok(Self::RedundantTransmissionChange),
				"CGI_SAI_CHANGE" => Ok(Self::CgiSaiChange),
				"RAI_CHANGE" => Ok(Self::RaiChange),
				"VSMF_CHANGE" => Ok(Self::VsmfChange),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for TriggerType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TriggerType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TriggerType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Tunnel Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Tunnel Information",
	///  "type": "object",
	///  "required": [
	///    "gtpTeid"
	///  ],
	///  "properties": {
	///    "anType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "gtpTeid": {
	///      "$ref": "#/components/schemas/Teid"
	///    },
	///    "ipv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TunnelInfo {
		#[serde(rename = "anType", default, skip_serializing_if = "Option::is_none")]
		pub an_type: Option<AccessType>,
		#[serde(rename = "gtpTeid")]
		pub gtp_teid: Teid,
		#[serde(rename = "ipv4Addr", default, skip_serializing_if = "Option::is_none")]
		pub ipv4_addr: Option<Ipv4Addr>,
		#[serde(rename = "ipv6Addr", default, skip_serializing_if = "Option::is_none")]
		pub ipv6_addr: Option<Ipv6Addr>,
	}

	impl From<&TunnelInfo> for TunnelInfo {
		fn from(value: &TunnelInfo) -> Self {
			value.clone()
		}
	}

	/// Contain the TWAP Identifier as defined in clause 4.2.8.5.3 of 3GPP TS
	/// 23.501 or the WLAN location information as defined in clause 4.5.7.2.8
	/// of 3GPP TS 23.402.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contain the TWAP Identifier as defined in clause
	/// 4.2.8.5.3 of 3GPP TS 23.501 or the WLAN location information as defined
	/// in clause 4.5.7.2.8 of 3GPP TS 23.402.\n",
	///  "type": "object",
	///  "required": [
	///    "ssId"
	///  ],
	///  "properties": {
	///    "bssId": {
	///      "description": "When present, it shall contain the BSSID of the
	/// access point to which the UE is attached, for trusted WLAN access, see
	/// IEEE Std 802.11-2012. \n",
	///      "type": "string"
	///    },
	///    "civicAddress": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "ssId": {
	///      "description": "This IE shall contain the SSID of the access point
	/// to which the UE is attached, that is received over NGAP, see IEEE Std
	/// 802.11-2012. \n",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TwapId {
		/// When present, it shall contain the BSSID of the access point to
		/// which the UE is attached, for trusted WLAN access, see IEEE Std
		/// 802.11-2012.
		#[serde(rename = "bssId", default, skip_serializing_if = "Option::is_none")]
		pub bss_id: Option<String>,
		#[serde(
			rename = "civicAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub civic_address: Option<Bytes>,
		/// This IE shall contain the SSID of the access point to which the UE
		/// is attached, that is received over NGAP, see IEEE Std 802.11-2012.
		#[serde(rename = "ssId")]
		pub ss_id: String,
	}

	impl From<&TwapId> for TwapId {
		fn from(value: &TwapId) -> Self {
			value.clone()
		}
	}

	/// Addressing information (IP addresses, FQDN) of the TWIF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Addressing information (IP addresses, FQDN) of the
	/// TWIF",
	///  "type": "object",
	///  "properties": {
	///    "endpointFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "ipv4EndpointAddresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6EndpointAddresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TwifInfo {
		#[serde(
			rename = "endpointFqdn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub endpoint_fqdn: Option<Fqdn>,
		#[serde(
			rename = "ipv4EndpointAddresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_endpoint_addresses: Vec<Ipv4Addr>,
		#[serde(
			rename = "ipv6EndpointAddresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_endpoint_addresses: Vec<Ipv6Addr>,
	}

	impl From<&TwifInfo> for TwifInfo {
		fn from(value: &TwifInfo) -> Self {
			value.clone()
		}
	}

	/// Integer where the allowed values correspond to the value range of an
	/// unsigned 16-bit integer.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Integer where the allowed values correspond to the
	/// value range of an unsigned 16-bit integer.",
	///  "type": "integer",
	///  "maximum": 65535.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Uint16(pub u16);

	impl ::std::ops::Deref for Uint16 {
		type Target = u16;
		fn deref(&self) -> &u16 {
			&self.0
		}
	}

	impl From<Uint16> for u16 {
		fn from(value: Uint16) -> Self {
			value.0
		}
	}

	impl From<&Uint16> for Uint16 {
		fn from(value: &Uint16) -> Self {
			value.clone()
		}
	}

	impl From<u16> for Uint16 {
		fn from(value: u16) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Uint16 {
		type Err = <u16 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Uint16 {
		type Error = <u16 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Uint16 {
		type Error = <u16 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Uint16 {
		type Error = <u16 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Uint16 {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Integer where the allowed values correspond to the value range of an
	/// unsigned 32-bit integer.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Integer where the allowed values correspond to the
	/// value range of an unsigned 32-bit integer.\n",
	///  "type": "integer",
	///  "maximum": 4294967295.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Uint32(pub u32);

	impl ::std::ops::Deref for Uint32 {
		type Target = u32;
		fn deref(&self) -> &u32 {
			&self.0
		}
	}

	impl From<Uint32> for u32 {
		fn from(value: Uint32) -> Self {
			value.0
		}
	}

	impl From<&Uint32> for Uint32 {
		fn from(value: &Uint32) -> Self {
			value.clone()
		}
	}

	impl From<u32> for Uint32 {
		fn from(value: u32) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Uint32 {
		type Err = <u32 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Uint32 {
		type Error = <u32 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Uint32 {
		type Error = <u32 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Uint32 {
		type Error = <u32 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Uint32 {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Integer where the allowed values correspond to the value range of an
	/// unsigned 64-bit integer.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Integer where the allowed values correspond to the
	/// value range of an unsigned 64-bit integer.\n",
	///  "type": "integer",
	///  "maximum": 1.8446744073709552e19,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Uint64(pub u64);

	impl ::std::ops::Deref for Uint64 {
		type Target = u64;
		fn deref(&self) -> &u64 {
			&self.0
		}
	}

	impl From<Uint64> for u64 {
		fn from(value: Uint64) -> Self {
			value.0
		}
	}

	impl From<&Uint64> for Uint64 {
		fn from(value: &Uint64) -> Self {
			value.clone()
		}
	}

	impl From<u64> for Uint64 {
		fn from(value: u64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Uint64 {
		type Err = <u64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Uint64 {
		type Error = <u64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Uint64 {
		type Error = <u64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Uint64 {
		type Error = <u64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Uint64 {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Unsigned Integer, i.e. only value 0 and integers above 0 are
	/// permissible.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned Integer, i.e. only value 0 and integers above
	/// 0 are permissible.",
	///  "type": "integer",
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Uinteger(pub u64);

	impl ::std::ops::Deref for Uinteger {
		type Target = u64;
		fn deref(&self) -> &u64 {
			&self.0
		}
	}

	impl From<Uinteger> for u64 {
		fn from(value: Uinteger) -> Self {
			value.0
		}
	}

	impl From<&Uinteger> for Uinteger {
		fn from(value: &Uinteger) -> Self {
			value.clone()
		}
	}

	impl From<u64> for Uinteger {
		fn from(value: u64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Uinteger {
		type Err = <u64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Uinteger {
		type Error = <u64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Uinteger {
		type Error = <u64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Uinteger {
		type Error = <u64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Uinteger {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// UL CL or BP Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "UL CL or BP Information",
	///  "type": "object",
	///  "properties": {
	///    "ulclBpUpfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UlclBpInformation {
		#[serde(
			rename = "ulclBpUpfId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ulcl_bp_upf_id: Option<NfInstanceId>,
	}

	impl From<&UlclBpInformation> for UlclBpInformation {
		fn from(value: &UlclBpInformation) -> Self {
			value.clone()
		}
	}

	/// Indicates the access type of a MA PDU session that is unavailable.
	/// Possible values are
	///  - 3GA_UNAVAILABLE
	///  - N3GA_UNAVAILABLE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the access type of a MA PDU session that is
	/// unavailable. Possible values are\n  - 3GA_UNAVAILABLE\n  -
	/// N3GA_UNAVAILABLE\n",
	///  "type": "string",
	///  "enum": [
	///    "3GA_UNAVAILABLE",
	///    "N3GA_UNAVAILABLE"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum UnavailableAccessIndication {
		#[default]
		#[serde(rename = "3GA_UNAVAILABLE")]
		ThreeGaUnavailable,
		#[serde(rename = "N3GA_UNAVAILABLE")]
		N3gaUnavailable,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UnavailableAccessIndication> for UnavailableAccessIndication {
		fn from(value: &UnavailableAccessIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for UnavailableAccessIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::ThreeGaUnavailable => "3GA_UNAVAILABLE".to_string(),
				Self::N3gaUnavailable => "N3GA_UNAVAILABLE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UnavailableAccessIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"3GA_UNAVAILABLE" => Ok(Self::ThreeGaUnavailable),
				"N3GA_UNAVAILABLE" => Ok(Self::N3gaUnavailable),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UnavailableAccessIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UnavailableAccessIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UnavailableAccessIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// User Plane Connection State. Possible values are
	/// - ACTIVATED
	/// - DEACTIVATED
	/// - ACTIVATING
	/// - SUSPENDED
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "User Plane Connection State. Possible values are\n-
	/// ACTIVATED\n- DEACTIVATED\n- ACTIVATING\n- SUSPENDED\n",
	///  "type": "string",
	///  "enum": [
	///    "ACTIVATED",
	///    "DEACTIVATED",
	///    "ACTIVATING",
	///    "SUSPENDED"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum UpCnxState {
		#[default]
		#[serde(rename = "ACTIVATED")]
		Activated,
		#[serde(rename = "DEACTIVATED")]
		Deactivated,
		#[serde(rename = "ACTIVATING")]
		Activating,
		#[serde(rename = "SUSPENDED")]
		Suspended,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UpCnxState> for UpCnxState {
		fn from(value: &UpCnxState) -> Self {
			value.clone()
		}
	}

	impl ToString for UpCnxState {
		fn to_string(&self) -> String {
			match *self {
				Self::Activated => "ACTIVATED".to_string(),
				Self::Deactivated => "DEACTIVATED".to_string(),
				Self::Activating => "ACTIVATING".to_string(),
				Self::Suspended => "SUSPENDED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UpCnxState {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ACTIVATED" => Ok(Self::Activated),
				"DEACTIVATED" => Ok(Self::Deactivated),
				"ACTIVATING" => Ok(Self::Activating),
				"SUSPENDED" => Ok(Self::Suspended),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UpCnxState {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UpCnxState {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UpCnxState {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// indicates whether UP confidentiality protection is required, preferred
	/// or not needed for all the traffic on the PDU Session. It shall comply
	/// with the provisions defined in table 5.4.3.5-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "indicates whether UP confidentiality protection is
	/// required, preferred or not needed for all the traffic on the PDU
	/// Session. It shall comply with the provisions defined in table
	/// 5.4.3.5-1.\n",
	///  "type": "string",
	///  "enum": [
	///    "REQUIRED",
	///    "PREFERRED",
	///    "NOT_NEEDED"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum UpConfidentiality {
		#[default]
		#[serde(rename = "REQUIRED")]
		Required,
		#[serde(rename = "PREFERRED")]
		Preferred,
		#[serde(rename = "NOT_NEEDED")]
		NotNeeded,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UpConfidentiality> for UpConfidentiality {
		fn from(value: &UpConfidentiality) -> Self {
			value.clone()
		}
	}

	impl ToString for UpConfidentiality {
		fn to_string(&self) -> String {
			match *self {
				Self::Required => "REQUIRED".to_string(),
				Self::Preferred => "PREFERRED".to_string(),
				Self::NotNeeded => "NOT_NEEDED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UpConfidentiality {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REQUIRED" => Ok(Self::Required),
				"PREFERRED" => Ok(Self::Preferred),
				"NOT_NEEDED" => Ok(Self::NotNeeded),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UpConfidentiality {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UpConfidentiality {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UpConfidentiality {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// indicates whether UP integrity protection is required, preferred or not
	/// needed for all the traffic on the PDU Session. It shall comply with the
	/// provisions defined in  table 5.4.3.4-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "indicates whether UP integrity protection is required,
	/// preferred or not needed for all the traffic on the PDU Session. It shall
	/// comply with the provisions defined in  table 5.4.3.4-1. \n",
	///  "type": "string",
	///  "enum": [
	///    "REQUIRED",
	///    "PREFERRED",
	///    "NOT_NEEDED"
	///  ],
	///  "x-allow-unknown": true
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
	pub enum UpIntegrity {
		#[default]
		#[serde(rename = "REQUIRED")]
		Required,
		#[serde(rename = "PREFERRED")]
		Preferred,
		#[serde(rename = "NOT_NEEDED")]
		NotNeeded,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UpIntegrity> for UpIntegrity {
		fn from(value: &UpIntegrity) -> Self {
			value.clone()
		}
	}

	impl ToString for UpIntegrity {
		fn to_string(&self) -> String {
			match *self {
				Self::Required => "REQUIRED".to_string(),
				Self::Preferred => "PREFERRED".to_string(),
				Self::NotNeeded => "NOT_NEEDED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UpIntegrity {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REQUIRED" => Ok(Self::Required),
				"PREFERRED" => Ok(Self::Preferred),
				"NOT_NEEDED" => Ok(Self::NotNeeded),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UpIntegrity {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UpIntegrity {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UpIntegrity {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains Userplain security information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains Userplain security information.",
	///  "type": "object",
	///  "required": [
	///    "upConfid",
	///    "upIntegr"
	///  ],
	///  "properties": {
	///    "upConfid": {
	///      "$ref": "#/components/schemas/UpConfidentiality"
	///    },
	///    "upIntegr": {
	///      "$ref": "#/components/schemas/UpIntegrity"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UpSecurity {
		#[serde(rename = "upConfid")]
		pub up_confid: UpConfidentiality,
		#[serde(rename = "upIntegr")]
		pub up_integr: UpIntegrity,
	}

	impl From<&UpSecurity> for UpSecurity {
		fn from(value: &UpSecurity) -> Self {
			value.clone()
		}
	}

	/// User Plane Security Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "User Plane Security Information",
	///  "type": "object",
	///  "required": [
	///    "upSecurity"
	///  ],
	///  "properties": {
	///    "maxIntegrityProtectedDataRateDl": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "maxIntegrityProtectedDataRateUl": {
	///      "$ref": "#/components/schemas/MaxIntegrityProtectedDataRate"
	///    },
	///    "securityResult": {
	///      "$ref": "#/components/schemas/SecurityResult"
	///    },
	///    "upSecurity": {
	///      "$ref": "#/components/schemas/UpSecurity"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UpSecurityInfo {
		#[serde(
			rename = "maxIntegrityProtectedDataRateDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_integrity_protected_data_rate_dl: Option<MaxIntegrityProtectedDataRate>,
		#[serde(
			rename = "maxIntegrityProtectedDataRateUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_integrity_protected_data_rate_ul: Option<MaxIntegrityProtectedDataRate>,
		#[serde(
			rename = "securityResult",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub security_result: Option<SecurityResult>,
		#[serde(rename = "upSecurity")]
		pub up_security: UpSecurity,
	}

	impl From<&UpSecurityInfo> for UpSecurityInfo {
		fn from(value: &UpSecurityInfo) -> Self {
			value.clone()
		}
	}

	/// Represents the ID/address/FQDN of the UPF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the ID/address/FQDN of the UPF.",
	///  "type": "object",
	///  "properties": {
	///    "upfAddr": {
	///      "$ref": "#/components/schemas/AddrFqdn"
	///    },
	///    "upfId": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UpfInformation {
		#[serde(rename = "upfAddr", default, skip_serializing_if = "Option::is_none")]
		pub upf_addr: Option<AddrFqdn>,
		#[serde(rename = "upfId", default, skip_serializing_if = "Option::is_none")]
		pub upf_id: Option<String>,
	}

	impl From<&UpfInformation> for UpfInformation {
		fn from(value: &UpfInformation) -> Self {
			value.clone()
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

	/// At least one of eutraLocation, nrLocation and n3gaLocation shall be
	/// present. Several of them may be present.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "At least one of eutraLocation, nrLocation and
	/// n3gaLocation shall be present. Several of them may be present.\n",
	///  "type": "object",
	///  "properties": {
	///    "eutraLocation": {
	///      "$ref": "#/components/schemas/EutraLocation"
	///    },
	///    "geraLocation": {
	///      "$ref": "#/components/schemas/GeraLocation"
	///    },
	///    "n3gaLocation": {
	///      "$ref": "#/components/schemas/N3gaLocation"
	///    },
	///    "nrLocation": {
	///      "$ref": "#/components/schemas/NrLocation"
	///    },
	///    "utraLocation": {
	///      "$ref": "#/components/schemas/UtraLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UserLocation {
		#[serde(
			rename = "eutraLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eutra_location: Option<EutraLocation>,
		#[serde(
			rename = "geraLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub gera_location: Option<GeraLocation>,
		#[serde(
			rename = "n3gaLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n3ga_location: Option<N3gaLocation>,
		#[serde(
			rename = "nrLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nr_location: Option<NrLocation>,
		#[serde(
			rename = "utraLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub utra_location: Option<UtraLocation>,
	}

	impl From<&UserLocation> for UserLocation {
		fn from(value: &UserLocation) -> Self {
			value.clone()
		}
	}

	/// Exactly one of cgi, sai or lai shall be present.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Exactly one of cgi, sai or lai shall be present.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "cgi"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "sai"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "lai"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "ageOfLocationInformation": {
	///      "description": "The value represents the elapsed time in minutes
	/// since the last network contact of the mobile station.  Value \"0\"
	/// indicates that the location information was obtained after a successful
	/// paging procedure for  Active Location Retrieval when the UE is in idle
	/// mode\n or after a successful location reporting procedure  the UE is in
	/// connected mode. Any\nother value than \"0\" indicates that the location
	/// information is the last known one.  See 3GPP TS 29.002 clause
	/// 17.7.8.\n",
	///      "type": "integer",
	///      "maximum": 32767.0,
	///      "minimum": 0.0
	///    },
	///    "cgi": {
	///      "$ref": "#/components/schemas/CellGlobalId"
	///    },
	///    "geodeticInformation": {
	///      "description": "Refers to Calling Geodetic Location. See ITU-T
	/// Recommendation Q.763 (1999) clause 3.88.2. Only the description of an
	/// ellipsoid point with uncertainty circle is allowed to be used.\n",
	///      "type": "string",
	///      "pattern": "^[0-9A-F]{20}$"
	///    },
	///    "geographicalInformation": {
	///      "description": "Refer to geographical Information.See 3GPP TS
	/// 23.032 clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///      "type": "string",
	///      "pattern": "^[0-9A-F]{16}$"
	///    },
	///    "lai": {
	///      "$ref": "#/components/schemas/LocationAreaId"
	///    },
	///    "rai": {
	///      "$ref": "#/components/schemas/RoutingAreaId"
	///    },
	///    "sai": {
	///      "$ref": "#/components/schemas/ServiceAreaId"
	///    },
	///    "ueLocationTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum UtraLocation {
		#[default]
		Variant0 {
			/// The value represents the elapsed time in minutes since the last
			/// network contact of the mobile station.  Value "0" indicates that
			/// the location information was obtained after a successful paging
			/// procedure for  Active Location Retrieval when the UE is in idle
			/// mode or after a successful location reporting
			/// procedure  the UE is in connected mode. Any
			/// other value than "0" indicates that the location information is
			/// the last known one.  See 3GPP TS 29.002 clause 17.7.8.
			#[serde(
				rename = "ageOfLocationInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			age_of_location_information: Option<i64>,
			cgi: CellGlobalId,
			/// Refers to Calling Geodetic Location. See ITU-T Recommendation
			/// Q.763 (1999) clause 3.88.2. Only the description of an ellipsoid
			/// point with uncertainty circle is allowed to be used.
			#[serde(
				rename = "geodeticInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geodetic_information: Option<UtraLocationVariant0GeodeticInformation>,
			/// Refer to geographical Information.See 3GPP TS 23.032 clause
			/// 7.3.2. Only the description of an ellipsoid point with
			/// uncertainty circle is allowed to be used.
			#[serde(
				rename = "geographicalInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geographical_information: Option<UtraLocationVariant0GeographicalInformation>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			rai: Option<RoutingAreaId>,
			#[serde(
				rename = "ueLocationTimestamp",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_location_timestamp: Option<DateTime>,
		},
		Variant1 {
			/// The value represents the elapsed time in minutes since the last
			/// network contact of the mobile station.  Value "0" indicates that
			/// the location information was obtained after a successful paging
			/// procedure for  Active Location Retrieval when the UE is in idle
			/// mode or after a successful location reporting
			/// procedure  the UE is in connected mode. Any
			/// other value than "0" indicates that the location information is
			/// the last known one.  See 3GPP TS 29.002 clause 17.7.8.
			#[serde(
				rename = "ageOfLocationInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			age_of_location_information: Option<i64>,
			/// Refers to Calling Geodetic Location. See ITU-T Recommendation
			/// Q.763 (1999) clause 3.88.2. Only the description of an ellipsoid
			/// point with uncertainty circle is allowed to be used.
			#[serde(
				rename = "geodeticInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geodetic_information: Option<UtraLocationVariant1GeodeticInformation>,
			/// Refer to geographical Information.See 3GPP TS 23.032 clause
			/// 7.3.2. Only the description of an ellipsoid point with
			/// uncertainty circle is allowed to be used.
			#[serde(
				rename = "geographicalInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geographical_information: Option<UtraLocationVariant1GeographicalInformation>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			rai: Option<RoutingAreaId>,
			sai: ServiceAreaId,
			#[serde(
				rename = "ueLocationTimestamp",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_location_timestamp: Option<DateTime>,
		},
		Variant2 {
			/// The value represents the elapsed time in minutes since the last
			/// network contact of the mobile station.  Value "0" indicates that
			/// the location information was obtained after a successful paging
			/// procedure for  Active Location Retrieval when the UE is in idle
			/// mode or after a successful location reporting
			/// procedure  the UE is in connected mode. Any
			/// other value than "0" indicates that the location information is
			/// the last known one.  See 3GPP TS 29.002 clause 17.7.8.
			#[serde(
				rename = "ageOfLocationInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			age_of_location_information: Option<i64>,
			/// Refers to Calling Geodetic Location. See ITU-T Recommendation
			/// Q.763 (1999) clause 3.88.2. Only the description of an ellipsoid
			/// point with uncertainty circle is allowed to be used.
			#[serde(
				rename = "geodeticInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geodetic_information: Option<UtraLocationVariant2GeodeticInformation>,
			/// Refer to geographical Information.See 3GPP TS 23.032 clause
			/// 7.3.2. Only the description of an ellipsoid point with
			/// uncertainty circle is allowed to be used.
			#[serde(
				rename = "geographicalInformation",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			geographical_information: Option<UtraLocationVariant2GeographicalInformation>,
			lai: LocationAreaId,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			rai: Option<RoutingAreaId>,
			#[serde(
				rename = "ueLocationTimestamp",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ue_location_timestamp: Option<DateTime>,
		},
	}

	impl From<&UtraLocation> for UtraLocation {
		fn from(value: &UtraLocation) -> Self {
			value.clone()
		}
	}

	/// Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763
	/// (1999) clause 3.88.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refers to Calling Geodetic Location. See ITU-T
	/// Recommendation Q.763 (1999) clause 3.88.2. Only the description of an
	/// ellipsoid point with uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{20}$"
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
	)]
	pub struct UtraLocationVariant0GeodeticInformation(String);

	impl ::std::ops::Deref for UtraLocationVariant0GeodeticInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<UtraLocationVariant0GeodeticInformation> for String {
		fn from(value: UtraLocationVariant0GeodeticInformation) -> Self {
			value.0
		}
	}

	impl From<&UtraLocationVariant0GeodeticInformation> for UtraLocationVariant0GeodeticInformation {
		fn from(value: &UtraLocationVariant0GeodeticInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for UtraLocationVariant0GeodeticInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{20}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{20}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for UtraLocationVariant0GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for UtraLocationVariant0GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for UtraLocationVariant0GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for UtraLocationVariant0GeodeticInformation {
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

	/// Refer to geographical Information.See 3GPP TS 23.032 clause 7.3.2. Only
	/// the description of an ellipsoid point with uncertainty circle is allowed
	/// to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refer to geographical Information.See 3GPP TS 23.032
	/// clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{16}$"
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
	)]
	pub struct UtraLocationVariant0GeographicalInformation(String);

	impl ::std::ops::Deref for UtraLocationVariant0GeographicalInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<UtraLocationVariant0GeographicalInformation> for String {
		fn from(value: UtraLocationVariant0GeographicalInformation) -> Self {
			value.0
		}
	}

	impl From<&UtraLocationVariant0GeographicalInformation>
		for UtraLocationVariant0GeographicalInformation
	{
		fn from(value: &UtraLocationVariant0GeographicalInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for UtraLocationVariant0GeographicalInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{16}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{16}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for UtraLocationVariant0GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for UtraLocationVariant0GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for UtraLocationVariant0GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for UtraLocationVariant0GeographicalInformation {
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

	/// Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763
	/// (1999) clause 3.88.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refers to Calling Geodetic Location. See ITU-T
	/// Recommendation Q.763 (1999) clause 3.88.2. Only the description of an
	/// ellipsoid point with uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{20}$"
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
	)]
	pub struct UtraLocationVariant1GeodeticInformation(String);

	impl ::std::ops::Deref for UtraLocationVariant1GeodeticInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<UtraLocationVariant1GeodeticInformation> for String {
		fn from(value: UtraLocationVariant1GeodeticInformation) -> Self {
			value.0
		}
	}

	impl From<&UtraLocationVariant1GeodeticInformation> for UtraLocationVariant1GeodeticInformation {
		fn from(value: &UtraLocationVariant1GeodeticInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for UtraLocationVariant1GeodeticInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{20}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{20}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for UtraLocationVariant1GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for UtraLocationVariant1GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for UtraLocationVariant1GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for UtraLocationVariant1GeodeticInformation {
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

	/// Refer to geographical Information.See 3GPP TS 23.032 clause 7.3.2. Only
	/// the description of an ellipsoid point with uncertainty circle is allowed
	/// to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refer to geographical Information.See 3GPP TS 23.032
	/// clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{16}$"
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
	)]
	pub struct UtraLocationVariant1GeographicalInformation(String);

	impl ::std::ops::Deref for UtraLocationVariant1GeographicalInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<UtraLocationVariant1GeographicalInformation> for String {
		fn from(value: UtraLocationVariant1GeographicalInformation) -> Self {
			value.0
		}
	}

	impl From<&UtraLocationVariant1GeographicalInformation>
		for UtraLocationVariant1GeographicalInformation
	{
		fn from(value: &UtraLocationVariant1GeographicalInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for UtraLocationVariant1GeographicalInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{16}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{16}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for UtraLocationVariant1GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for UtraLocationVariant1GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for UtraLocationVariant1GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for UtraLocationVariant1GeographicalInformation {
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

	/// Refers to Calling Geodetic Location. See ITU-T Recommendation Q.763
	/// (1999) clause 3.88.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refers to Calling Geodetic Location. See ITU-T
	/// Recommendation Q.763 (1999) clause 3.88.2. Only the description of an
	/// ellipsoid point with uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{20}$"
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
	)]
	pub struct UtraLocationVariant2GeodeticInformation(String);

	impl ::std::ops::Deref for UtraLocationVariant2GeodeticInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<UtraLocationVariant2GeodeticInformation> for String {
		fn from(value: UtraLocationVariant2GeodeticInformation) -> Self {
			value.0
		}
	}

	impl From<&UtraLocationVariant2GeodeticInformation> for UtraLocationVariant2GeodeticInformation {
		fn from(value: &UtraLocationVariant2GeodeticInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for UtraLocationVariant2GeodeticInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{20}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{20}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for UtraLocationVariant2GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for UtraLocationVariant2GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for UtraLocationVariant2GeodeticInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for UtraLocationVariant2GeodeticInformation {
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

	/// Refer to geographical Information.See 3GPP TS 23.032 clause 7.3.2. Only
	/// the description of an ellipsoid point with uncertainty circle is allowed
	/// to be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Refer to geographical Information.See 3GPP TS 23.032
	/// clause 7.3.2. Only the description of an ellipsoid point with
	/// uncertainty circle is allowed to be used.\n",
	///  "type": "string",
	///  "pattern": "^[0-9A-F]{16}$"
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
	)]
	pub struct UtraLocationVariant2GeographicalInformation(String);

	impl ::std::ops::Deref for UtraLocationVariant2GeographicalInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<UtraLocationVariant2GeographicalInformation> for String {
		fn from(value: UtraLocationVariant2GeographicalInformation) -> Self {
			value.0
		}
	}

	impl From<&UtraLocationVariant2GeographicalInformation>
		for UtraLocationVariant2GeographicalInformation
	{
		fn from(value: &UtraLocationVariant2GeographicalInformation) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for UtraLocationVariant2GeographicalInformation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9A-F]{16}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9A-F]{16}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for UtraLocationVariant2GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for UtraLocationVariant2GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for UtraLocationVariant2GeographicalInformation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for UtraLocationVariant2GeographicalInformation {
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

	/// Contains Usage data information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains Usage data information.",
	///  "type": "object",
	///  "required": [
	///    "downlinkVolume",
	///    "endTimeStamp",
	///    "startTimeStamp",
	///    "uplinkVolume"
	///  ],
	///  "properties": {
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/Int64"
	///    },
	///    "endTimeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "startTimeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "uplinkVolume": {
	///      "$ref": "#/components/schemas/Int64"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VolumeTimedReport {
		#[serde(rename = "downlinkVolume")]
		pub downlink_volume: Int64,
		#[serde(rename = "endTimeStamp")]
		pub end_time_stamp: DateTime,
		#[serde(rename = "startTimeStamp")]
		pub start_time_stamp: DateTime,
		#[serde(rename = "uplinkVolume")]
		pub uplink_volume: Int64,
	}

	impl From<&VolumeTimedReport> for VolumeTimedReport {
		fn from(value: &VolumeTimedReport) -> Self {
			value.clone()
		}
	}

	/// VPLMN QoS
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "VPLMN QoS",
	///  "type": "object",
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "arp": {
	///      "$ref": "#/components/schemas/Arp"
	///    },
	///    "guaFbrDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "guaFbrUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "maxFbrDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "maxFbrUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "sessionAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VplmnQos {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub arp: Option<Arp>,
		#[serde(rename = "5qi", default, skip_serializing_if = "Option::is_none")]
		pub five_qi: Option<_5qi>,
		#[serde(rename = "guaFbrDl", default, skip_serializing_if = "Option::is_none")]
		pub gua_fbr_dl: Option<BitRate>,
		#[serde(rename = "guaFbrUl", default, skip_serializing_if = "Option::is_none")]
		pub gua_fbr_ul: Option<BitRate>,
		#[serde(rename = "maxFbrDl", default, skip_serializing_if = "Option::is_none")]
		pub max_fbr_dl: Option<BitRate>,
		#[serde(rename = "maxFbrUl", default, skip_serializing_if = "Option::is_none")]
		pub max_fbr_ul: Option<BitRate>,
		#[serde(
			rename = "sessionAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub session_ambr: Option<Ambr>,
	}

	impl From<&VplmnQos> for VplmnQos {
		fn from(value: &VplmnQos) -> Self {
			value.clone()
		}
	}

	/// Data within Update Request towards V-SMF, or from SMF to I-SMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Update Request towards V-SMF, or from SMF
	/// to I-SMF",
	///  "type": "object",
	///  "required": [
	///    "requestIndication"
	///  ],
	///  "properties": {
	///    "additionalCnTunnelInfo": {
	///      "$ref": "#/components/schemas/TunnelInfo"
	///    },
	///    "alwaysOnGranted": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "assignEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Arp"
	///      },
	///      "minItems": 1
	///    },
	///    "backOffTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "cause": {
	///      "$ref": "#/components/schemas/Cause"
	///    },
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      }
	///    },
	///    "epsBearerInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "epsPdnCnxInfo": {
	///      "$ref": "#/components/schemas/EpsPdnCnxInfo"
	///    },
	///    "hsmfPduSessionUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "maAcceptedInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "maReleaseInd": {
	///      "$ref": "#/components/schemas/MaReleaseIndication"
	///    },
	///    "modifiedEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EbiArpMapping"
	///      },
	///      "minItems": 1
	///    },
	///    "n1SmInfoToUe": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n1smCause": {
	///      "type": "string"
	///    },
	///    "n4Info": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt1": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt2": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt3": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n9DataForwardingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "n9InactivityTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "newSmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "newSmfPduSessionUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "pti": {
	///      "$ref": "#/components/schemas/ProcedureTransactionId"
	///    },
	///    "qosFlowsAddModRequestList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowAddModifyRequestItem"
	///      },
	///      "minItems": 1
	///    },
	///    "qosFlowsRelRequestList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowReleaseRequestItem"
	///      },
	///      "minItems": 1
	///    },
	///    "qosMonitoringInfo": {
	///      "$ref": "#/components/schemas/QosMonitoringInfo"
	///    },
	///    "requestIndication": {
	///      "$ref": "#/components/schemas/RequestIndication"
	///    },
	///    "revokeEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerId"
	///      },
	///      "minItems": 1
	///    },
	///    "sessionAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "smallDataRateControlEnabled": {
	///      "type": "boolean"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VsmfUpdateData {
		#[serde(
			rename = "additionalCnTunnelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_cn_tunnel_info: Option<TunnelInfo>,
		#[serde(rename = "alwaysOnGranted", default)]
		pub always_on_granted: bool,
		#[serde(
			rename = "assignEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub assign_ebi_list: Vec<Arp>,
		#[serde(
			rename = "backOffTimer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub back_off_timer: Option<DurationSec>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub cause: Option<Cause>,
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<Dnai>,
		#[serde(
			rename = "epsBearerInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub eps_bearer_info: Vec<EpsBearerInfo>,
		#[serde(
			rename = "epsPdnCnxInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_pdn_cnx_info: Option<EpsPdnCnxInfo>,
		#[serde(
			rename = "hsmfPduSessionUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hsmf_pdu_session_uri: Option<Uri>,
		#[serde(rename = "maAcceptedInd", default)]
		pub ma_accepted_ind: bool,
		#[serde(
			rename = "maReleaseInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ma_release_ind: Option<MaReleaseIndication>,
		#[serde(
			rename = "modifiedEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub modified_ebi_list: Vec<EbiArpMapping>,
		#[serde(
			rename = "n1SmInfoToUe",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_sm_info_to_ue: Option<RefToBinaryData>,
		#[serde(rename = "n1smCause", default, skip_serializing_if = "Option::is_none")]
		pub n1sm_cause: Option<String>,
		#[serde(rename = "n4Info", default, skip_serializing_if = "Option::is_none")]
		pub n4_info: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt1",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext1: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt2",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext2: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt3",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext3: Option<N4Information>,
		#[serde(rename = "n9DataForwardingInd", default)]
		pub n9_data_forwarding_ind: bool,
		#[serde(
			rename = "n9InactivityTimer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n9_inactivity_timer: Option<DurationSec>,
		#[serde(rename = "newSmfId", default, skip_serializing_if = "Option::is_none")]
		pub new_smf_id: Option<NfInstanceId>,
		#[serde(
			rename = "newSmfPduSessionUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub new_smf_pdu_session_uri: Option<Uri>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pti: Option<ProcedureTransactionId>,
		#[serde(
			rename = "qosFlowsAddModRequestList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flows_add_mod_request_list: Vec<QosFlowAddModifyRequestItem>,
		#[serde(
			rename = "qosFlowsRelRequestList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flows_rel_request_list: Vec<QosFlowReleaseRequestItem>,
		#[serde(
			rename = "qosMonitoringInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qos_monitoring_info: Option<QosMonitoringInfo>,
		#[serde(rename = "requestIndication")]
		pub request_indication: RequestIndication,
		#[serde(
			rename = "revokeEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub revoke_ebi_list: Vec<EpsBearerId>,
		#[serde(
			rename = "sessionAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub session_ambr: Option<Ambr>,
		#[serde(
			rename = "smallDataRateControlEnabled",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub small_data_rate_control_enabled: Option<bool>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&VsmfUpdateData> for VsmfUpdateData {
		fn from(value: &VsmfUpdateData) -> Self {
			value.clone()
		}
	}

	/// Error within Update Response from V-SMF, or from I-SMF to SMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Error within Update Response from V-SMF, or from I-SMF
	/// to SMF",
	///  "type": "object",
	///  "required": [
	///    "error"
	///  ],
	///  "properties": {
	///    "5gMmCauseValue": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "error": {
	///      "$ref": "#/components/schemas/ExtProblemDetails"
	///    },
	///    "failedToAssignEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Arp"
	///      },
	///      "minItems": 1
	///    },
	///    "n1SmInfoFromUe": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n1smCause": {
	///      "type": "string",
	///      "pattern": "^[A-F0-9]{2}$"
	///    },
	///    "n4Info": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt1": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt2": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt3": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "ngApCause": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    },
	///    "pti": {
	///      "$ref": "#/components/schemas/ProcedureTransactionId"
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "unknownN1SmInfo": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VsmfUpdateError {
		pub error: ExtProblemDetails,
		#[serde(
			rename = "failedToAssignEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub failed_to_assign_ebi_list: Vec<Arp>,
		#[serde(
			rename = "5gMmCauseValue",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_g_mm_cause_value: Option<Uinteger>,
		#[serde(
			rename = "n1SmInfoFromUe",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_sm_info_from_ue: Option<RefToBinaryData>,
		#[serde(rename = "n1smCause", default, skip_serializing_if = "Option::is_none")]
		pub n1sm_cause: Option<VsmfUpdateErrorN1smCause>,
		#[serde(rename = "n4Info", default, skip_serializing_if = "Option::is_none")]
		pub n4_info: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt1",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext1: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt2",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext2: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt3",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext3: Option<N4Information>,
		#[serde(rename = "ngApCause", default, skip_serializing_if = "Option::is_none")]
		pub ng_ap_cause: Option<NgApCause>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pti: Option<ProcedureTransactionId>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
		#[serde(
			rename = "unknownN1SmInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub unknown_n1_sm_info: Option<RefToBinaryData>,
	}

	impl From<&VsmfUpdateError> for VsmfUpdateError {
		fn from(value: &VsmfUpdateError) -> Self {
			value.clone()
		}
	}

	/// VsmfUpdateErrorN1smCause
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-F0-9]{2}$"
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
	)]
	pub struct VsmfUpdateErrorN1smCause(String);

	impl ::std::ops::Deref for VsmfUpdateErrorN1smCause {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<VsmfUpdateErrorN1smCause> for String {
		fn from(value: VsmfUpdateErrorN1smCause) -> Self {
			value.0
		}
	}

	impl From<&VsmfUpdateErrorN1smCause> for VsmfUpdateErrorN1smCause {
		fn from(value: &VsmfUpdateErrorN1smCause) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for VsmfUpdateErrorN1smCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-F0-9]{2}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-F0-9]{2}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for VsmfUpdateErrorN1smCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for VsmfUpdateErrorN1smCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for VsmfUpdateErrorN1smCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for VsmfUpdateErrorN1smCause {
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

	/// Data within Update Response from V-SMF, or from I-SMF to SMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Update Response from V-SMF, or from I-SMF
	/// to SMF",
	///  "type": "object",
	///  "properties": {
	///    "addUeLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "assignedEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EbiArpMapping"
	///      },
	///      "minItems": 1
	///    },
	///    "failedToAssignEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Arp"
	///      },
	///      "minItems": 1
	///    },
	///    "modifiedEbiListNotDelivered": {
	///      "type": "boolean"
	///    },
	///    "n1SmInfoFromUe": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n4Info": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt1": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt2": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "n4InfoExt3": {
	///      "$ref": "#/components/schemas/N4Information"
	///    },
	///    "qosFlowsAddModList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowItem"
	///      },
	///      "minItems": 1
	///    },
	///    "qosFlowsFailedtoAddModList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowItem"
	///      },
	///      "minItems": 1
	///    },
	///    "qosFlowsFailedtoRelList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowItem"
	///      },
	///      "minItems": 1
	///    },
	///    "qosFlowsRelList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowItem"
	///      },
	///      "minItems": 1
	///    },
	///    "releasedEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerId"
	///      },
	///      "minItems": 1
	///    },
	///    "secondaryRatUsageInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SecondaryRatUsageInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "secondaryRatUsageReport": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SecondaryRatUsageReport"
	///      },
	///      "minItems": 1
	///    },
	///    "ueLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "unknownN1SmInfo": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VsmfUpdatedData {
		#[serde(
			rename = "addUeLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_ue_location: Option<UserLocation>,
		#[serde(
			rename = "assignedEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub assigned_ebi_list: Vec<EbiArpMapping>,
		#[serde(
			rename = "failedToAssignEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub failed_to_assign_ebi_list: Vec<Arp>,
		#[serde(
			rename = "modifiedEbiListNotDelivered",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub modified_ebi_list_not_delivered: Option<bool>,
		#[serde(
			rename = "n1SmInfoFromUe",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_sm_info_from_ue: Option<RefToBinaryData>,
		#[serde(rename = "n4Info", default, skip_serializing_if = "Option::is_none")]
		pub n4_info: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt1",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext1: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt2",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext2: Option<N4Information>,
		#[serde(
			rename = "n4InfoExt3",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n4_info_ext3: Option<N4Information>,
		#[serde(
			rename = "qosFlowsAddModList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flows_add_mod_list: Vec<QosFlowItem>,
		#[serde(
			rename = "qosFlowsFailedtoAddModList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flows_failedto_add_mod_list: Vec<QosFlowItem>,
		#[serde(
			rename = "qosFlowsFailedtoRelList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flows_failedto_rel_list: Vec<QosFlowItem>,
		#[serde(
			rename = "qosFlowsRelList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flows_rel_list: Vec<QosFlowItem>,
		#[serde(
			rename = "releasedEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub released_ebi_list: Vec<EpsBearerId>,
		#[serde(
			rename = "secondaryRatUsageInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub secondary_rat_usage_info: Vec<SecondaryRatUsageInfo>,
		#[serde(
			rename = "secondaryRatUsageReport",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub secondary_rat_usage_report: Vec<SecondaryRatUsageReport>,
		#[serde(
			rename = "ueLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_location: Option<UserLocation>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
		#[serde(
			rename = "unknownN1SmInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub unknown_n1_sm_info: Option<RefToBinaryData>,
	}

	impl From<&VsmfUpdatedData> for VsmfUpdatedData {
		fn from(value: &VsmfUpdatedData) -> Self {
			value.clone()
		}
	}

	/// This represents the identifier of the W-AGF ID as specified in clause
	/// 9.3.1.162 of  3GPP TS 38.413 in hexadecimal representation. Each
	/// character in the string shall take a value  of "0" to "9", "a" to "f" or
	/// "A" to "F" and shall represent 4 bits. The most significant  character
	/// representing the 4 most significant bits of the W-AGF ID shall appear
	/// first in the  string, and the character representing the 4 least
	/// significant bit of the W-AGF ID shall  appear last in the string.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This represents the identifier of the W-AGF ID as
	/// specified in clause 9.3.1.162 of  3GPP TS 38.413 in hexadecimal
	/// representation. Each character in the string shall take a value  of
	/// \"0\" to \"9\", \"a\" to \"f\" or \"A\" to \"F\" and shall represent 4
	/// bits. The most significant  character representing the 4 most
	/// significant bits of the W-AGF ID shall appear first in the  string, and
	/// the character representing the 4 least significant bit of the W-AGF ID
	/// shall  appear last in the string. \n",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]+$"
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
	)]
	pub struct WAgfId(String);

	impl ::std::ops::Deref for WAgfId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<WAgfId> for String {
		fn from(value: WAgfId) -> Self {
			value.0
		}
	}

	impl From<&WAgfId> for WAgfId {
		fn from(value: &WAgfId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for WAgfId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for WAgfId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for WAgfId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for WAgfId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for WAgfId {
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

	/// Information of the W-AGF end-points
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of the W-AGF end-points",
	///  "type": "object",
	///  "properties": {
	///    "endpointFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "ipv4EndpointAddresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6EndpointAddresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct WAgfInfo {
		#[serde(
			rename = "endpointFqdn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub endpoint_fqdn: Option<Fqdn>,
		#[serde(
			rename = "ipv4EndpointAddresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_endpoint_addresses: Vec<Ipv4Addr>,
		#[serde(
			rename = "ipv6EndpointAddresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_endpoint_addresses: Vec<Ipv6Addr>,
	}

	impl From<&WAgfInfo> for WAgfInfo {
		fn from(value: &WAgfInfo) -> Self {
			value.clone()
		}
	}

	/// Unsigned integer representing a 5G QoS Identifier (see clause 5.7.2.1 of
	/// 3GPP TS 23.501, within the range 0 to 255.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer representing a 5G QoS Identifier (see
	/// clause 5.7.2.1 of 3GPP TS 23.501, within the range 0 to 255.\n",
	///  "type": "integer",
	///  "maximum": 255.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct _5qi(pub u8);

	impl ::std::ops::Deref for _5qi {
		type Target = u8;
		fn deref(&self) -> &u8 {
			&self.0
		}
	}

	impl From<_5qi> for u8 {
		fn from(value: _5qi) -> Self {
			value.0
		}
	}

	impl From<&_5qi> for _5qi {
		fn from(value: &_5qi) -> Self {
			value.clone()
		}
	}

	impl From<u8> for _5qi {
		fn from(value: u8) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for _5qi {
		type Err = <u8 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for _5qi {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for _5qi {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for _5qi {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for _5qi {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Unsigned integer indicating the 5QI Priority Level (see clauses 5.7.3.3
	/// and 5.7.4 of 3GPP TS 23.501, within the range 1 to 127.Values are
	/// ordered in decreasing order of priority,  i.e. with 1 as the highest
	/// priority and 127 as the lowest priority.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer indicating the 5QI Priority Level (see
	/// clauses 5.7.3.3 and 5.7.4 of 3GPP TS 23.501, within the range 1 to
	/// 127.Values are ordered in decreasing order of priority,  i.e. with 1 as
	/// the highest priority and 127 as the lowest priority. \n",
	///  "type": "integer",
	///  "maximum": 127.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct _5qiPriorityLevel(pub i64);

	impl ::std::ops::Deref for _5qiPriorityLevel {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<_5qiPriorityLevel> for i64 {
		fn from(value: _5qiPriorityLevel) -> Self {
			value.0
		}
	}

	impl From<&_5qiPriorityLevel> for _5qiPriorityLevel {
		fn from(value: &_5qiPriorityLevel) -> Self {
			value.clone()
		}
	}

	impl From<i64> for _5qiPriorityLevel {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for _5qiPriorityLevel {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for _5qiPriorityLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for _5qiPriorityLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for _5qiPriorityLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for _5qiPriorityLevel {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}
}

#[derive(Clone, Debug)]
/// Client for Openapi-5GC
///
/// Merged Apis. © 2024, 3GPP Organizational Partners (ARIB, ATIS, CCSA, ETSI,
/// TSDSI, TTA, TTC). All rights reserved.
///
/// Version: 1.2.6
pub struct Client {
	pub(crate) baseurl: String,
	pub(crate) client: reqwest::Client,
}

impl Client {
	/// Create a new client.
	///
	/// `baseurl` is the base URL provided to the internal
	/// `reqwest::Client`, and should include a scheme and hostname,
	/// as well as port and a path stem if applicable.
	pub fn new(baseurl: &str) -> Self {
		#[cfg(not(target_arch = "wasm32"))]
		let client = {
			let dur = std::time::Duration::from_secs(15);
			reqwest::ClientBuilder::new()
				.connect_timeout(dur)
				.timeout(dur)
		};
		#[cfg(target_arch = "wasm32")]
		let client = reqwest::ClientBuilder::new();
		Self::new_with_client(baseurl, client.build().unwrap())
	}

	/// Construct a new client with an existing `reqwest::Client`,
	/// allowing more control over its configuration.
	///
	/// `baseurl` is the base URL provided to the internal
	/// `reqwest::Client`, and should include a scheme and hostname,
	/// as well as port and a path stem if applicable.
	pub fn new_with_client(
		baseurl: &str,
		client: reqwest::Client,
	) -> Self {
		Self {
			baseurl: baseurl.to_string(),
			client,
		}
	}

	/// Get the base URL to which requests are made.
	pub fn baseurl(&self) -> &String {
		&self.baseurl
	}

	/// Get the internal `reqwest::Client` used to make requests.
	pub fn client(&self) -> &reqwest::Client {
		&self.client
	}

	/// Get the version of this API.
	///
	/// This string is pulled directly from the source OpenAPI
	/// document and may be in any format the API selects.
	pub fn api_version(&self) -> &'static str {
		"1.2.6"
	}
}

#[allow(clippy::all)]
impl Client {}

/// Items consumers will typically use such as the Client.
pub mod prelude {
	#[allow(unused_imports)]
	pub use super::Client;
}
