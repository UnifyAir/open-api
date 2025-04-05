#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};

#[allow(unused_imports)]
pub use crate::progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use crate::progenitor_client::{RequestBuilderExt, encode_path};

/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
	use macros::NewUnchecked;

	use crate::common::{
		AmfId,
		Fqdn,
		Ipv4Addr,
		Ipv4AddrMask,
		Ipv6Addr,
		Ipv6Prefix,
		Mcc,
		Mnc,
		NfType,
		Uri,
		error,
	};

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
		NewUnchecked,
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

	/// Indicates wether the access is via 3GPP or via non-3GPP but with the
	/// OpenAPI  'nullable: true' property."
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates wether the access is via 3GPP or via non-3GPP
	/// but with the OpenAPI  'nullable: true' property.\"\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/AccessType"
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
	pub enum AccessTypeRm {
		#[default]
		AccessType(AccessType),
		NullValue(NullValue),
	}

	impl From<&AccessTypeRm> for AccessTypeRm {
		fn from(value: &AccessTypeRm) -> Self {
			value.clone()
		}
	}

	impl From<AccessType> for AccessTypeRm {
		fn from(value: AccessType) -> Self {
			Self::AccessType(value)
		}
	}

	impl From<NullValue> for AccessTypeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// The ACS information for the 5G-RG is defined in BBF TR-069 [42] or in
	/// BBF TR-369
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The ACS information for the 5G-RG is defined in BBF
	/// TR-069 [42] or in BBF TR-369",
	///  "type": "object",
	///  "properties": {
	///    "acsIpv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "acsIpv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "acsUrl": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AcsInfo {
		#[serde(
			rename = "acsIpv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub acs_ipv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "acsIpv6Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub acs_ipv6_addr: Option<Ipv6Addr>,
		#[serde(rename = "acsUrl", default, skip_serializing_if = "Option::is_none")]
		pub acs_url: Option<Uri>,
	}

	impl From<&AcsInfo> for AcsInfo {
		fn from(value: &AcsInfo) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the 'AcsInfo' data type,
	/// but with the  OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'AcsInfo' data type, but with the  OpenAPI 'nullable: true'
	/// property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/AcsInfo"
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
	pub enum AcsInfoRm {
		#[default]
		AcsInfo(AcsInfo),
		NullValue(NullValue),
	}

	impl From<&AcsInfoRm> for AcsInfoRm {
		fn from(value: &AcsInfoRm) -> Self {
			value.clone()
		}
	}

	impl From<AcsInfo> for AcsInfoRm {
		fn from(value: AcsInfo) -> Self {
			Self::AcsInfo(value)
		}
	}

	impl From<NullValue> for AcsInfoRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// Contains an AF application identifier.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains an AF application identifier.",
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
	pub struct AfAppId(pub String);

	impl ::std::ops::Deref for AfAppId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AfAppId> for String {
		fn from(value: AfAppId) -> Self {
			value.0
		}
	}

	impl From<&AfAppId> for AfAppId {
		fn from(value: &AfAppId) -> Self {
			value.clone()
		}
	}

	impl From<String> for AfAppId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AfAppId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for AfAppId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Altitude
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates value of altitude.",
	///  "type": "number",
	///  "format": "double",
	///  "maximum": 32767.0,
	///  "minimum": -32767.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Altitude(pub f64);

	impl ::std::ops::Deref for Altitude {
		type Target = f64;
		fn deref(&self) -> &f64 {
			&self.0
		}
	}

	impl From<Altitude> for f64 {
		fn from(value: Altitude) -> Self {
			value.0
		}
	}

	impl From<&Altitude> for Altitude {
		fn from(value: &Altitude) -> Self {
			value.clone()
		}
	}

	impl From<f64> for Altitude {
		fn from(value: f64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Altitude {
		type Err = <f64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Altitude {
		type Error = <f64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Altitude {
		type Error = <f64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Altitude {
		type Error = <f64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Altitude {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// This data type is defined in the same way as the 'Ambr' data type, but
	/// with the OpenAPI 'nullable: true' property."
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Ambr'
	/// data type, but with the OpenAPI 'nullable: true' property.\"\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Ambr"
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
	pub enum AmbrRm {
		#[default]
		Ambr(Ambr),
		NullValue(NullValue),
	}

	impl From<&AmbrRm> for AmbrRm {
		fn from(value: &AmbrRm) -> Self {
			value.clone()
		}
	}

	impl From<Ambr> for AmbrRm {
		fn from(value: Ambr) -> Self {
			Self::Ambr(value)
		}
	}

	impl From<NullValue> for AmbrRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// AmfName
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/Fqdn"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfName(pub Fqdn);

	impl ::std::ops::Deref for AmfName {
		type Target = Fqdn;
		fn deref(&self) -> &Fqdn {
			&self.0
		}
	}

	impl From<AmfName> for Fqdn {
		fn from(value: AmfName) -> Self {
			value.0
		}
	}

	impl From<&AmfName> for AmfName {
		fn from(value: &AmfName) -> Self {
			value.clone()
		}
	}

	impl From<Fqdn> for AmfName {
		fn from(value: Fqdn) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AmfName {
		type Err = <Fqdn as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for AmfName {
		type Error = <Fqdn as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AmfName {
		type Error = <Fqdn as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AmfName {
		type Error = <Fqdn as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for AmfName {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Indicates value of angle.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates value of angle.",
	///  "type": "integer",
	///  "maximum": 360.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Angle(pub i64);

	impl ::std::ops::Deref for Angle {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<Angle> for i64 {
		fn from(value: Angle) -> Self {
			value.0
		}
	}

	impl From<&Angle> for Angle {
		fn from(value: &Angle) -> Self {
			value.clone()
		}
	}

	impl From<i64> for Angle {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Angle {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Angle {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Angle {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Angle {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Angle {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// Application provided charging identifier allowing correlation of
	/// charging information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Application provided charging identifier allowing
	/// correlation of charging information.",
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
	pub struct ApplicationChargingId(pub String);

	impl ::std::ops::Deref for ApplicationChargingId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ApplicationChargingId> for String {
		fn from(value: ApplicationChargingId) -> Self {
			value.0
		}
	}

	impl From<&ApplicationChargingId> for ApplicationChargingId {
		fn from(value: &ApplicationChargingId) -> Self {
			value.clone()
		}
	}

	impl From<String> for ApplicationChargingId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ApplicationChargingId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for ApplicationChargingId {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// String providing an application identifier with the OpenAPI 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String providing an application identifier with the
	/// OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ApplicationIdRm(pub Option<String>);

	impl ::std::ops::Deref for ApplicationIdRm {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<ApplicationIdRm> for Option<String> {
		fn from(value: ApplicationIdRm) -> Self {
			value.0
		}
	}

	impl From<&ApplicationIdRm> for ApplicationIdRm {
		fn from(value: &ApplicationIdRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for ApplicationIdRm {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// Provides area information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Provides area information.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "tacs"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "areaCode"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "areaCode": {
	///      "$ref": "#/components/schemas/AreaCode"
	///    },
	///    "tacs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tac"
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
	pub enum Area {
		#[default]
		Variant0 { tacs: Vec<Tac> },
		Variant1 {
			#[serde(rename = "areaCode")]
			area_code: AreaCode,
		},
	}

	impl From<&Area> for Area {
		fn from(value: &Area) -> Self {
			value.clone()
		}
	}

	/// Values are operator specific.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Values are operator specific.",
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
	pub struct AreaCode(pub String);

	impl ::std::ops::Deref for AreaCode {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AreaCode> for String {
		fn from(value: AreaCode) -> Self {
			value.0
		}
	}

	impl From<&AreaCode> for AreaCode {
		fn from(value: &AreaCode) -> Self {
			value.clone()
		}
	}

	impl From<String> for AreaCode {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AreaCode {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for AreaCode {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// This data type is defined in the same way as the 'AreaCode' data type,
	/// but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'AreaCode' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AreaCodeRm(pub Option<String>);

	impl ::std::ops::Deref for AreaCodeRm {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<AreaCodeRm> for Option<String> {
		fn from(value: AreaCodeRm) -> Self {
			value.0
		}
	}

	impl From<&AreaCodeRm> for AreaCodeRm {
		fn from(value: &AreaCodeRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for AreaCodeRm {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// Contain the area based on Cells or Tracking Areas.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contain the area based on Cells or Tracking Areas.",
	///  "type": "object",
	///  "properties": {
	///    "eutraCellIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EutraCellId"
	///      },
	///      "minItems": 1
	///    },
	///    "nrCellIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NrCellId"
	///      },
	///      "minItems": 1
	///    },
	///    "tacInfoPerPlmn": {
	///      "description": "A map (list of key-value pairs) where PlmnId
	/// converted to a string serves as key\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/TacInfo"
	///      }
	///    },
	///    "tacList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tac"
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
	pub struct AreaScope {
		#[serde(
			rename = "eutraCellIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub eutra_cell_id_list: Vec<EutraCellId>,
		#[serde(
			rename = "nrCellIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nr_cell_id_list: Vec<NrCellId>,
		/// A map (list of key-value pairs) where PlmnId converted to a string
		/// serves as key
		#[serde(
			rename = "tacInfoPerPlmn",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub tac_info_per_plmn: ::std::collections::HashMap<String, TacInfo>,
		#[serde(rename = "tacList", default, skip_serializing_if = "Vec::is_empty")]
		pub tac_list: Vec<Tac>,
	}

	impl From<&AreaScope> for AreaScope {
		fn from(value: &AreaScope) -> Self {
			value.clone()
		}
	}

	/// AreaSessionId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/Uint16"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AreaSessionId(pub Uint16);

	impl ::std::ops::Deref for AreaSessionId {
		type Target = Uint16;
		fn deref(&self) -> &Uint16 {
			&self.0
		}
	}

	impl From<AreaSessionId> for Uint16 {
		fn from(value: AreaSessionId) -> Self {
			value.0
		}
	}

	impl From<&AreaSessionId> for AreaSessionId {
		fn from(value: &AreaSessionId) -> Self {
			value.clone()
		}
	}

	impl From<Uint16> for AreaSessionId {
		fn from(value: Uint16) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AreaSessionId {
		type Err = <Uint16 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for AreaSessionId {
		type Error = <Uint16 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AreaSessionId {
		type Error = <Uint16 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AreaSessionId {
		type Error = <Uint16 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for AreaSessionId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// AreaSessionPolicyId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/Uint16"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AreaSessionPolicyId(pub Uint16);

	impl ::std::ops::Deref for AreaSessionPolicyId {
		type Target = Uint16;
		fn deref(&self) -> &Uint16 {
			&self.0
		}
	}

	impl From<AreaSessionPolicyId> for Uint16 {
		fn from(value: AreaSessionPolicyId) -> Self {
			value.0
		}
	}

	impl From<&AreaSessionPolicyId> for AreaSessionPolicyId {
		fn from(value: &AreaSessionPolicyId) -> Self {
			value.clone()
		}
	}

	impl From<Uint16> for AreaSessionPolicyId {
		fn from(value: Uint16) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AreaSessionPolicyId {
		type Err = <Uint16 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for AreaSessionPolicyId {
		type Error = <Uint16 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AreaSessionPolicyId {
		type Error = <Uint16 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AreaSessionPolicyId {
		type Error = <Uint16 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for AreaSessionPolicyId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Integer value indicating the ARFCN applicable for a downlink, uplink or
	/// bi-directional (TDD) NR global frequency raster, as definition of
	/// "ARFCN-ValueNR" IE in clause 6.3.2 of 3GPP TS 38.331.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Integer value indicating the ARFCN applicable for a
	/// downlink, uplink or bi-directional (TDD) NR global frequency raster, as
	/// definition of \"ARFCN-ValueNR\" IE in clause 6.3.2 of 3GPP TS
	/// 38.331.\n",
	///  "type": "integer",
	///  "maximum": 3279165.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ArfcnValueNr(pub i64);

	impl ::std::ops::Deref for ArfcnValueNr {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ArfcnValueNr> for i64 {
		fn from(value: ArfcnValueNr) -> Self {
			value.0
		}
	}

	impl From<&ArfcnValueNr> for ArfcnValueNr {
		fn from(value: &ArfcnValueNr) -> Self {
			value.clone()
		}
	}

	impl From<i64> for ArfcnValueNr {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ArfcnValueNr {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ArfcnValueNr {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ArfcnValueNr {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ArfcnValueNr {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ArfcnValueNr {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// This data type is defined in the same way as the 'ArpPriorityLevel' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'ArpPriorityLevel' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
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
	pub struct ArpPriorityLevelRm(pub Option<i64>);

	impl ::std::ops::Deref for ArpPriorityLevelRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<ArpPriorityLevelRm> for Option<i64> {
		fn from(value: ArpPriorityLevelRm) -> Self {
			value.0
		}
	}

	impl From<&ArpPriorityLevelRm> for ArpPriorityLevelRm {
		fn from(value: &ArpPriorityLevelRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for ArpPriorityLevelRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the 'Arp' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Arp'
	/// data type, but with the OpenAPI 'nullable: true' property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Arp"
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
	pub enum ArpRm {
		#[default]
		Arp(Arp),
		NullValue(NullValue),
	}

	impl From<&ArpRm> for ArpRm {
		fn from(value: &ArpRm) -> Self {
			value.clone()
		}
	}

	impl From<Arp> for ArpRm {
		fn from(value: Arp) -> Self {
			Self::Arp(value)
		}
	}

	impl From<NullValue> for ArpRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// contains a search parameter and its positive or negative content.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "contains a search parameter and its positive or
	/// negative content.",
	///  "type": "object",
	///  "required": [
	///    "attr",
	///    "value"
	///  ],
	///  "properties": {
	///    "attr": {
	///      "description": "contains the name of a defined query parameter.",
	///      "type": "string"
	///    },
	///    "negative": {
	///      "description": "indicates whether the negative condition applies
	/// for the query condition.",
	///      "type": "boolean"
	///    },
	///    "value": {}
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Atom {
		/// contains the name of a defined query parameter.
		pub attr: String,
		/// indicates whether the negative condition applies for the query
		/// condition.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub negative: Option<bool>,
		pub value: ::serde_json::Value,
	}

	impl From<&Atom> for Atom {
		fn from(value: &Atom) -> Self {
			value.clone()
		}
	}

	/// Containes Capability to support procedures related to Access Traffic
	/// Steering, Switching, Splitting.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Containes Capability to support procedures related to
	/// Access Traffic Steering, Switching, Splitting.\n",
	///  "type": "object",
	///  "properties": {
	///    "atsssLL": {
	///      "description": "Indicates the ATSSS-LL capability to support
	/// procedures related to Access Traffic Steering, Switching, Splitting (see
	/// clauses 4.2.10, 5.32 of 3GPP TS 23.501). true: Supported false
	/// (default): Not Supported\n",
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "mptcp": {
	///      "description": "Indicates the MPTCP capability to support
	/// procedures related to Access Traffic Steering, Switching, Splitting (see
	/// clauses 4.2.10, 5.32 of 3GPP TS 23.501 true: Supported false (default):
	/// Not Supported\n",
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "rttWithoutPmf": {
	///      "description": "This IE is only used by the UPF to indicate whether
	/// the UPF supports RTT measurement without PMF (see clauses 5.32.2,
	/// 6.3.3.3 of 3GPP TS 23.501 true: Supported false (default): Not
	/// Supported\n",
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
	pub struct AtsssCapability {
		/// Indicates the ATSSS-LL capability to support procedures related to
		/// Access Traffic Steering, Switching, Splitting (see clauses 4.2.10,
		/// 5.32 of 3GPP TS 23.501). true: Supported false (default): Not
		/// Supported
		#[serde(rename = "atsssLL", default)]
		pub atsss_ll: bool,
		/// Indicates the MPTCP capability to support procedures related to
		/// Access Traffic Steering, Switching, Splitting (see clauses 4.2.10,
		/// 5.32 of 3GPP TS 23.501 true: Supported false (default): Not
		/// Supported
		#[serde(default)]
		pub mptcp: bool,
		/// This IE is only used by the UPF to indicate whether the UPF supports
		/// RTT measurement without PMF (see clauses 5.32.2, 6.3.3.3 of 3GPP TS
		/// 23.501 true: Supported false (default): Not Supported
		#[serde(rename = "rttWithoutPmf", default)]
		pub rtt_without_pmf: bool,
	}

	impl From<&AtsssCapability> for AtsssCapability {
		fn from(value: &AtsssCapability) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - "EAP_SUCCESS": The NSSAA status is EAP-Success.
	/// - "EAP_FAILURE": The NSSAA status is EAP-Failure.
	/// - "PENDING": The NSSAA status is Pending.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- \"EAP_SUCCESS\": The NSSAA
	/// status is EAP-Success.\n- \"EAP_FAILURE\": The NSSAA status is
	/// EAP-Failure.\n- \"PENDING\": The NSSAA status is Pending. \n",
	///  "type": "string",
	///  "enum": [
	///    "EAP_SUCCESS",
	///    "EAP_FAILURE",
	///    "PENDING"
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
	pub enum AuthStatus {
		#[default]
		#[serde(rename = "EAP_SUCCESS")]
		EapSuccess,
		#[serde(rename = "EAP_FAILURE")]
		EapFailure,
		#[serde(rename = "PENDING")]
		Pending,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AuthStatus> for AuthStatus {
		fn from(value: &AuthStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for AuthStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::EapSuccess => "EAP_SUCCESS".to_string(),
				Self::EapFailure => "EAP_FAILURE".to_string(),
				Self::Pending => "PENDING".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AuthStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EAP_SUCCESS" => Ok(Self::EapSuccess),
				"EAP_FAILURE" => Ok(Self::EapFailure),
				"PENDING" => Ok(Self::Pending),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AuthStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AuthStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AuthStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// This data type is defined in the same way as the 'AverWindow' data type,
	/// but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'AverWindow' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "default": 2000,
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 4095.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AverWindowRm(pub Option<i64>);

	impl ::std::ops::Deref for AverWindowRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<AverWindowRm> for Option<i64> {
		fn from(value: AverWindowRm) -> Self {
			value.0
		}
	}

	impl From<&AverWindowRm> for AverWindowRm {
		fn from(value: &AverWindowRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for AverWindowRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
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
	///      "$ref": "#/components/schemas/AmfName"
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
		pub backup_amf: AmfName,
		/// If present, this IE shall contain the list of GUAMI(s) (supported by
		/// the AMF) for which the backupAmf IE applies.
		#[serde(rename = "guamiList", default, skip_serializing_if = "Vec::is_empty")]
		pub guami_list: Vec<Guami>,
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

	/// This data type is defined in the same way as the 'BatteryIndication'
	/// data type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'BatteryIndication' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/BatteryIndication"
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
	pub enum BatteryIndicationRm {
		#[default]
		BatteryIndication(BatteryIndication),
		NullValue(NullValue),
	}

	impl From<&BatteryIndicationRm> for BatteryIndicationRm {
		fn from(value: &BatteryIndicationRm) -> Self {
			value.clone()
		}
	}

	impl From<BatteryIndication> for BatteryIndicationRm {
		fn from(value: BatteryIndication) -> Self {
			Self::BatteryIndication(value)
		}
	}

	impl From<NullValue> for BatteryIndicationRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// string with format 'binary' as defined in OpenAPI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'binary' as defined in OpenAPI.",
	///  "type": "string",
	///  "format": "binary"
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
	pub struct Binary(pub String);

	impl ::std::ops::Deref for Binary {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Binary> for String {
		fn from(value: Binary) -> Self {
			value.0
		}
	}

	impl From<&Binary> for Binary {
		fn from(value: &Binary) -> Self {
			value.clone()
		}
	}

	impl From<String> for Binary {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Binary {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for Binary {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// string with format 'binary' as defined in OpenAPI OpenAPI with
	/// 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'binary' as defined in OpenAPI
	/// OpenAPI with 'nullable: true' property.",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "format": "binary"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BinaryRm(pub Option<String>);

	impl ::std::ops::Deref for BinaryRm {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<BinaryRm> for Option<String> {
		fn from(value: BinaryRm) -> Self {
			value.0
		}
	}

	impl From<&BinaryRm> for BinaryRm {
		fn from(value: &BinaryRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for BinaryRm {
		fn from(value: Option<String>) -> Self {
			Self(value)
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
		NewUnchecked,
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

	/// This data type is defined in the same way as the 'BitRate' data type,
	/// but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'BitRate' data type, but with the OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^\\d+(\\.\\d+)? (bps|Kbps|Mbps|Gbps|Tbps)$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BitRateRm(pub Option<BitRateRmInner>);

	impl ::std::ops::Deref for BitRateRm {
		type Target = Option<BitRateRmInner>;
		fn deref(&self) -> &Option<BitRateRmInner> {
			&self.0
		}
	}

	impl From<BitRateRm> for Option<BitRateRmInner> {
		fn from(value: BitRateRm) -> Self {
			value.0
		}
	}

	impl From<&BitRateRm> for BitRateRm {
		fn from(value: &BitRateRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<BitRateRmInner>> for BitRateRm {
		fn from(value: Option<BitRateRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the 'BitRate' data type,
	/// but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'BitRate' data type, but with the OpenAPI 'nullable: true' property.\n",
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
		NewUnchecked,
	)]
	pub struct BitRateRmInner(String);

	impl ::std::ops::Deref for BitRateRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<BitRateRmInner> for String {
		fn from(value: BitRateRmInner) -> Self {
			value.0
		}
	}

	impl From<&BitRateRmInner> for BitRateRmInner {
		fn from(value: &BitRateRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for BitRateRmInner {
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

	impl ::std::convert::TryFrom<&str> for BitRateRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for BitRateRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for BitRateRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for BitRateRmInner {
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

	/// Broadcast MBS Session's Delivery Status
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Broadcast MBS Session's Delivery Status",
	///  "type": "string",
	///  "enum": [
	///    "STARTED",
	///    "TERMINATED"
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
	pub enum BroadcastDeliveryStatus {
		#[default]
		#[serde(rename = "STARTED")]
		Started,
		#[serde(rename = "TERMINATED")]
		Terminated,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&BroadcastDeliveryStatus> for BroadcastDeliveryStatus {
		fn from(value: &BroadcastDeliveryStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for BroadcastDeliveryStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Started => "STARTED".to_string(),
				Self::Terminated => "TERMINATED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for BroadcastDeliveryStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"STARTED" => Ok(Self::Started),
				"TERMINATED" => Ok(Self::Terminated),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for BroadcastDeliveryStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for BroadcastDeliveryStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for BroadcastDeliveryStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// string with format 'bytes' as defined in OpenAPI OpenAPI with 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'bytes' as defined in OpenAPI
	/// OpenAPI with 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "format": "byte"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BytesRm(pub Option<String>);

	impl ::std::ops::Deref for BytesRm {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<BytesRm> for Option<String> {
		fn from(value: BytesRm) -> Self {
			value.0
		}
	}

	impl From<&BytesRm> for BytesRm {
		fn from(value: &BytesRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for BytesRm {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// String representing the C-MSISDN as defined in clause 18.7 of 3GPP TS
	/// 23.003.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing the C-MSISDN as defined in clause
	/// 18.7 of 3GPP TS 23.003.",
	///  "type": "string",
	///  "pattern": "^[0-9]{5,15}$"
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
	pub struct CMsisdn(String);

	impl ::std::ops::Deref for CMsisdn {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<CMsisdn> for String {
		fn from(value: CMsisdn) -> Self {
			value.0
		}
	}

	impl From<&CMsisdn> for CMsisdn {
		fn from(value: &CMsisdn) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for CMsisdn {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{5,15}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{5,15}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for CMsisdn {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for CMsisdn {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for CMsisdn {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CMsisdn {
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

	/// String representing the C-MSISDN as defined in clause 18.7 of 3GPP TS
	/// 23.003 with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing the C-MSISDN as defined in clause 18.7 of 3GPP TS 23.003 with the OpenAPI 'nullable: true' property. \n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^[0-9]{5,15}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CMsisdnRm(pub Option<CMsisdnRmInner>);

	impl ::std::ops::Deref for CMsisdnRm {
		type Target = Option<CMsisdnRmInner>;
		fn deref(&self) -> &Option<CMsisdnRmInner> {
			&self.0
		}
	}

	impl From<CMsisdnRm> for Option<CMsisdnRmInner> {
		fn from(value: CMsisdnRm) -> Self {
			value.0
		}
	}

	impl From<&CMsisdnRm> for CMsisdnRm {
		fn from(value: &CMsisdnRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<CMsisdnRmInner>> for CMsisdnRm {
		fn from(value: Option<CMsisdnRmInner>) -> Self {
			Self(value)
		}
	}

	/// String representing the C-MSISDN as defined in clause 18.7 of 3GPP TS
	/// 23.003 with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing the C-MSISDN as defined in clause 18.7 of 3GPP TS 23.003 with the OpenAPI 'nullable: true' property. \n",
	///  "type": "string",
	///  "pattern": "^[0-9]{5,15}$"
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
	pub struct CMsisdnRmInner(String);

	impl ::std::ops::Deref for CMsisdnRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<CMsisdnRmInner> for String {
		fn from(value: CMsisdnRmInner) -> Self {
			value.0
		}
	}

	impl From<&CMsisdnRmInner> for CMsisdnRmInner {
		fn from(value: &CMsisdnRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for CMsisdnRmInner {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{5,15}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{5,15}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for CMsisdnRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for CMsisdnRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for CMsisdnRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CMsisdnRmInner {
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

	/// String containing a Closed Access Group Identifier.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String containing a Closed Access Group Identifier.",
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
		NewUnchecked,
	)]
	pub struct CagId(String);

	impl ::std::ops::Deref for CagId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<CagId> for String {
		fn from(value: CagId) -> Self {
			value.0
		}
	}

	impl From<&CagId> for CagId {
		fn from(value: &CagId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for CagId {
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

	impl ::std::convert::TryFrom<&str> for CagId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for CagId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for CagId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CagId {
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
		NewUnchecked,
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
		NewUnchecked,
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

	/// It contains data which need to be changed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It contains data which need to be changed.",
	///  "type": "object",
	///  "required": [
	///    "op",
	///    "path"
	///  ],
	///  "properties": {
	///    "from": {
	///      "description": "indicates the path of the source JSON element
	/// (according to JSON Pointer syntax)  being moved or copied to the
	/// location indicated by the \"path\" attribute. It shall  be present if
	/// the \"op\" attribute is of value \"MOVE\".\n",
	///      "type": "string"
	///    },
	///    "newValue": {},
	///    "op": {
	///      "$ref": "#/components/schemas/ChangeType"
	///    },
	///    "origValue": {},
	///    "path": {
	///      "description": "contains a JSON pointer value (as defined in IETF
	/// RFC 6901) that references a target  location within the resource on
	/// which the change has been applied.\n",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ChangeItem {
		/// indicates the path of the source JSON element (according to JSON
		/// Pointer syntax)  being moved or copied to the location indicated by
		/// the "path" attribute. It shall  be present if the "op" attribute is
		/// of value "MOVE".
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub from: Option<String>,
		#[serde(rename = "newValue", default, skip_serializing_if = "Option::is_none")]
		pub new_value: Option<::serde_json::Value>,
		pub op: ChangeType,
		#[serde(rename = "origValue", default, skip_serializing_if = "Option::is_none")]
		pub orig_value: Option<::serde_json::Value>,
		/// contains a JSON pointer value (as defined in IETF RFC 6901) that
		/// references a target  location within the resource on which the
		/// change has been applied.
		pub path: String,
	}

	impl From<&ChangeItem> for ChangeItem {
		fn from(value: &ChangeItem) -> Self {
			value.clone()
		}
	}

	/// Indicates the type of change to be performed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the type of change to be performed.",
	///  "type": "string",
	///  "enum": [
	///    "ADD",
	///    "MOVE",
	///    "REMOVE",
	///    "REPLACE"
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
	pub enum ChangeType {
		#[default]
		#[serde(rename = "ADD")]
		Add,
		#[serde(rename = "MOVE")]
		Move,
		#[serde(rename = "REMOVE")]
		Remove,
		#[serde(rename = "REPLACE")]
		Replace,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ChangeType> for ChangeType {
		fn from(value: &ChangeType) -> Self {
			value.clone()
		}
	}

	impl ToString for ChangeType {
		fn to_string(&self) -> String {
			match *self {
				Self::Add => "ADD".to_string(),
				Self::Move => "MOVE".to_string(),
				Self::Remove => "REMOVE".to_string(),
				Self::Replace => "REPLACE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ChangeType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ADD" => Ok(Self::Add),
				"MOVE" => Ok(Self::Move),
				"REMOVE" => Ok(Self::Remove),
				"REPLACE" => Ok(Self::Replace),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ChangeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ChangeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ChangeType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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
	///  "deprecated": true,
	///  "type": "integer",
	///  "maximum": 4294967295.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ChargingId(pub u32);

	impl ::std::ops::Deref for ChargingId {
		type Target = u32;
		fn deref(&self) -> &u32 {
			&self.0
		}
	}

	impl From<ChargingId> for u32 {
		fn from(value: ChargingId) -> Self {
			value.0
		}
	}

	impl From<&ChargingId> for ChargingId {
		fn from(value: &ChargingId) -> Self {
			value.clone()
		}
	}

	impl From<u32> for ChargingId {
		fn from(value: u32) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ChargingId {
		type Err = <u32 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ChargingId {
		type Error = <u32 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ChargingId {
		type Error = <u32 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ChargingId {
		type Error = <u32 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ChargingId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Indicates a Civic address.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates a Civic address.",
	///  "type": "object",
	///  "properties": {
	///    "A1": {
	///      "type": "string"
	///    },
	///    "A2": {
	///      "type": "string"
	///    },
	///    "A3": {
	///      "type": "string"
	///    },
	///    "A4": {
	///      "type": "string"
	///    },
	///    "A5": {
	///      "type": "string"
	///    },
	///    "A6": {
	///      "type": "string"
	///    },
	///    "ADDCODE": {
	///      "type": "string"
	///    },
	///    "BLD": {
	///      "type": "string"
	///    },
	///    "FLR": {
	///      "type": "string"
	///    },
	///    "HNO": {
	///      "type": "string"
	///    },
	///    "HNS": {
	///      "type": "string"
	///    },
	///    "LMK": {
	///      "type": "string"
	///    },
	///    "LOC": {
	///      "type": "string"
	///    },
	///    "NAM": {
	///      "type": "string"
	///    },
	///    "PC": {
	///      "type": "string"
	///    },
	///    "PCN": {
	///      "type": "string"
	///    },
	///    "PLC": {
	///      "type": "string"
	///    },
	///    "POBOX": {
	///      "type": "string"
	///    },
	///    "POD": {
	///      "type": "string"
	///    },
	///    "POM": {
	///      "type": "string"
	///    },
	///    "PRD": {
	///      "type": "string"
	///    },
	///    "PRM": {
	///      "type": "string"
	///    },
	///    "RD": {
	///      "type": "string"
	///    },
	///    "RDBR": {
	///      "type": "string"
	///    },
	///    "RDSEC": {
	///      "type": "string"
	///    },
	///    "RDSUBBR": {
	///      "type": "string"
	///    },
	///    "ROOM": {
	///      "type": "string"
	///    },
	///    "SEAT": {
	///      "type": "string"
	///    },
	///    "STS": {
	///      "type": "string"
	///    },
	///    "UNIT": {
	///      "type": "string"
	///    },
	///    "country": {
	///      "type": "string"
	///    },
	///    "method": {
	///      "type": "string"
	///    },
	///    "providedBy": {
	///      "type": "string"
	///    },
	///    "usageRules": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CivicAddress {
		#[serde(rename = "A1", default, skip_serializing_if = "Option::is_none")]
		pub a1: Option<String>,
		#[serde(rename = "A2", default, skip_serializing_if = "Option::is_none")]
		pub a2: Option<String>,
		#[serde(rename = "A3", default, skip_serializing_if = "Option::is_none")]
		pub a3: Option<String>,
		#[serde(rename = "A4", default, skip_serializing_if = "Option::is_none")]
		pub a4: Option<String>,
		#[serde(rename = "A5", default, skip_serializing_if = "Option::is_none")]
		pub a5: Option<String>,
		#[serde(rename = "A6", default, skip_serializing_if = "Option::is_none")]
		pub a6: Option<String>,
		#[serde(rename = "ADDCODE", default, skip_serializing_if = "Option::is_none")]
		pub addcode: Option<String>,
		#[serde(rename = "BLD", default, skip_serializing_if = "Option::is_none")]
		pub bld: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub country: Option<String>,
		#[serde(rename = "FLR", default, skip_serializing_if = "Option::is_none")]
		pub flr: Option<String>,
		#[serde(rename = "HNO", default, skip_serializing_if = "Option::is_none")]
		pub hno: Option<String>,
		#[serde(rename = "HNS", default, skip_serializing_if = "Option::is_none")]
		pub hns: Option<String>,
		#[serde(rename = "LMK", default, skip_serializing_if = "Option::is_none")]
		pub lmk: Option<String>,
		#[serde(rename = "LOC", default, skip_serializing_if = "Option::is_none")]
		pub loc: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub method: Option<String>,
		#[serde(rename = "NAM", default, skip_serializing_if = "Option::is_none")]
		pub nam: Option<String>,
		#[serde(rename = "PC", default, skip_serializing_if = "Option::is_none")]
		pub pc: Option<String>,
		#[serde(rename = "PCN", default, skip_serializing_if = "Option::is_none")]
		pub pcn: Option<String>,
		#[serde(rename = "PLC", default, skip_serializing_if = "Option::is_none")]
		pub plc: Option<String>,
		#[serde(rename = "POBOX", default, skip_serializing_if = "Option::is_none")]
		pub pobox: Option<String>,
		#[serde(rename = "POD", default, skip_serializing_if = "Option::is_none")]
		pub pod: Option<String>,
		#[serde(rename = "POM", default, skip_serializing_if = "Option::is_none")]
		pub pom: Option<String>,
		#[serde(rename = "PRD", default, skip_serializing_if = "Option::is_none")]
		pub prd: Option<String>,
		#[serde(rename = "PRM", default, skip_serializing_if = "Option::is_none")]
		pub prm: Option<String>,
		#[serde(
			rename = "providedBy",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub provided_by: Option<String>,
		#[serde(rename = "RD", default, skip_serializing_if = "Option::is_none")]
		pub rd: Option<String>,
		#[serde(rename = "RDBR", default, skip_serializing_if = "Option::is_none")]
		pub rdbr: Option<String>,
		#[serde(rename = "RDSEC", default, skip_serializing_if = "Option::is_none")]
		pub rdsec: Option<String>,
		#[serde(rename = "RDSUBBR", default, skip_serializing_if = "Option::is_none")]
		pub rdsubbr: Option<String>,
		#[serde(rename = "ROOM", default, skip_serializing_if = "Option::is_none")]
		pub room: Option<String>,
		#[serde(rename = "SEAT", default, skip_serializing_if = "Option::is_none")]
		pub seat: Option<String>,
		#[serde(rename = "STS", default, skip_serializing_if = "Option::is_none")]
		pub sts: Option<String>,
		#[serde(rename = "UNIT", default, skip_serializing_if = "Option::is_none")]
		pub unit: Option<String>,
		#[serde(
			rename = "usageRules",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub usage_rules: Option<String>,
	}

	impl From<&CivicAddress> for CivicAddress {
		fn from(value: &CivicAddress) -> Self {
			value.clone()
		}
	}

	/// A conjunctive normal form
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A conjunctive normal form",
	///  "type": "object",
	///  "required": [
	///    "cnfUnits"
	///  ],
	///  "properties": {
	///    "cnfUnits": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CnfUnit"
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
	pub struct Cnf {
		#[serde(rename = "cnfUnits")]
		pub cnf_units: Vec<CnfUnit>,
	}

	impl From<&Cnf> for Cnf {
		fn from(value: &Cnf) -> Self {
			value.clone()
		}
	}

	/// During the processing of cnfUnits attribute, all the members in the
	/// array shall be  interpreted as logically concatenated with logical
	/// "AND".
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "During the processing of cnfUnits attribute, all the
	/// members in the array shall be  interpreted as logically concatenated
	/// with logical \"AND\".\n",
	///  "type": "object",
	///  "required": [
	///    "cnfUnit"
	///  ],
	///  "properties": {
	///    "cnfUnit": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Atom"
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
	pub struct CnfUnit {
		#[serde(rename = "cnfUnit")]
		pub cnf_unit: Vec<Atom>,
	}

	impl From<&CnfUnit> for CnfUnit {
		fn from(value: &CnfUnit) -> Self {
			value.clone()
		}
	}

	/// Contains codec related information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains codec related information.",
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
	pub struct CodecData(pub String);

	impl ::std::ops::Deref for CodecData {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<CodecData> for String {
		fn from(value: CodecData) -> Self {
			value.0
		}
	}

	impl From<&CodecData> for CodecData {
		fn from(value: &CodecData) -> Self {
			value.clone()
		}
	}

	impl From<String> for CodecData {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for CodecData {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for CodecData {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// The enumeration CollectionPeriodRmmLteMdt defines Collection period for
	/// RRM measurements LTE for MDT in the trace. See 3GPP TS 32.422 for
	/// further description of the values. It shall comply with the provisions
	/// defined in table 5.6.3.15-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration CollectionPeriodRmmLteMdt defines
	/// Collection period for RRM measurements LTE for MDT in the trace. See
	/// 3GPP TS 32.422 for further description of the values. It shall comply
	/// with the provisions defined in table 5.6.3.15-1.\n",
	///  "type": "string",
	///  "enum": [
	///    1024,
	///    1280,
	///    2048,
	///    2560,
	///    5120,
	///    10240,
	///    60000
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
	pub enum CollectionPeriodRmmLteMdt {
		#[default]
		#[serde(rename = "1024")]
		NUM1024,
		#[serde(rename = "1280")]
		NUM1280,
		#[serde(rename = "2048")]
		NUM2048,
		#[serde(rename = "2560")]
		NUM2560,
		#[serde(rename = "5120")]
		NUM5120,
		#[serde(rename = "10240")]
		NUM10240,
		#[serde(rename = "60000")]
		NUM60000,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&CollectionPeriodRmmLteMdt> for CollectionPeriodRmmLteMdt {
		fn from(value: &CollectionPeriodRmmLteMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for CollectionPeriodRmmLteMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::NUM1024 => "1024".to_string(),
				Self::NUM1280 => "1280".to_string(),
				Self::NUM2048 => "2048".to_string(),
				Self::NUM2560 => "2560".to_string(),
				Self::NUM5120 => "5120".to_string(),
				Self::NUM10240 => "10240".to_string(),
				Self::NUM60000 => "60000".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for CollectionPeriodRmmLteMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"1024" => Ok(Self::NUM1024),
				"1280" => Ok(Self::NUM1280),
				"2048" => Ok(Self::NUM2048),
				"2560" => Ok(Self::NUM2560),
				"5120" => Ok(Self::NUM5120),
				"10240" => Ok(Self::NUM10240),
				"60000" => Ok(Self::NUM60000),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for CollectionPeriodRmmLteMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for CollectionPeriodRmmLteMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for CollectionPeriodRmmLteMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration CollectionPeriodRmmNrMdt defines Collection period for
	/// RRM measurements NR for MDT in the trace. See 3GPP TS 32.422 for further
	/// description of the values. It shall comply with the provisions defined
	/// in table 5.6.3.19-1
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration CollectionPeriodRmmNrMdt defines
	/// Collection period for RRM measurements NR for MDT in the trace. See 3GPP
	/// TS 32.422 for further description of the values. It shall comply with
	/// the provisions defined in table 5.6.3.19-1\n",
	///  "type": "string",
	///  "enum": [
	///    1024,
	///    2048,
	///    5120,
	///    10240,
	///    60000
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
	pub enum CollectionPeriodRmmNrMdt {
		#[default]
		#[serde(rename = "1024")]
		NUM1024,
		#[serde(rename = "2048")]
		NUM2048,
		#[serde(rename = "5120")]
		NUM5120,
		#[serde(rename = "10240")]
		NUM10240,
		#[serde(rename = "60000")]
		NUM60000,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&CollectionPeriodRmmNrMdt> for CollectionPeriodRmmNrMdt {
		fn from(value: &CollectionPeriodRmmNrMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for CollectionPeriodRmmNrMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::NUM1024 => "1024".to_string(),
				Self::NUM2048 => "2048".to_string(),
				Self::NUM5120 => "5120".to_string(),
				Self::NUM10240 => "10240".to_string(),
				Self::NUM60000 => "60000".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for CollectionPeriodRmmNrMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"1024" => Ok(Self::NUM1024),
				"2048" => Ok(Self::NUM2048),
				"5120" => Ok(Self::NUM5120),
				"10240" => Ok(Self::NUM10240),
				"60000" => Ok(Self::NUM60000),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for CollectionPeriodRmmNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for CollectionPeriodRmmNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for CollectionPeriodRmmNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The ComplexQuery data type is either a conjunctive normal form or a
	/// disjunctive normal form.  The attribute names "cnfUnits" and "dnfUnits"
	/// (see clause 5.2.4.11 and clause 5.2.4.12)  serve as discriminator.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The ComplexQuery data type is either a conjunctive
	/// normal form or a disjunctive normal form.  The attribute names
	/// \"cnfUnits\" and \"dnfUnits\" (see clause 5.2.4.11 and clause 5.2.4.12)
	/// serve as discriminator.\n",
	///  "oneOf": [
	///    {
	///      "$ref": "#/components/schemas/Cnf"
	///    },
	///    {
	///      "$ref": "#/components/schemas/Dnf"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum ComplexQuery {
		#[default]
		Cnf(Cnf),
		Dnf(Dnf),
	}

	impl From<&ComplexQuery> for ComplexQuery {
		fn from(value: &ComplexQuery) -> Self {
			value.clone()
		}
	}

	impl From<Cnf> for ComplexQuery {
		fn from(value: Cnf) -> Self {
			Self::Cnf(value)
		}
	}

	impl From<Dnf> for ComplexQuery {
		fn from(value: Dnf) -> Self {
			Self::Dnf(value)
		}
	}

	/// Indicates value of confidence.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates value of confidence.",
	///  "type": "integer",
	///  "maximum": 100.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Confidence(pub i64);

	impl ::std::ops::Deref for Confidence {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<Confidence> for i64 {
		fn from(value: Confidence) -> Self {
			value.0
		}
	}

	impl From<&Confidence> for Confidence {
		fn from(value: &Confidence) -> Self {
			value.clone()
		}
	}

	impl From<i64> for Confidence {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Confidence {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Confidence {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Confidence {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Confidence {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Confidence {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// It contains the Core Network type 5GC or EPC.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It contains the Core Network type 5GC or EPC.",
	///  "type": "string",
	///  "enum": [
	///    "5GC",
	///    "EPC"
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
	pub enum CoreNetworkType {
		#[default]
		#[serde(rename = "5GC")]
		FiveGc,
		#[serde(rename = "EPC")]
		Epc,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&CoreNetworkType> for CoreNetworkType {
		fn from(value: &CoreNetworkType) -> Self {
			value.clone()
		}
	}

	impl ToString for CoreNetworkType {
		fn to_string(&self) -> String {
			match *self {
				Self::FiveGc => "5GC".to_string(),
				Self::Epc => "EPC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for CoreNetworkType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"5GC" => Ok(Self::FiveGc),
				"EPC" => Ok(Self::Epc),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for CoreNetworkType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for CoreNetworkType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for CoreNetworkType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// It contains the Core Network type 5GC or EPC but with the OpenAPI
	/// 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It contains the Core Network type 5GC or EPC but with
	/// the OpenAPI 'nullable: true' property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/CoreNetworkType"
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
	pub enum CoreNetworkTypeRm {
		#[default]
		CoreNetworkType(CoreNetworkType),
		NullValue(NullValue),
	}

	impl From<&CoreNetworkTypeRm> for CoreNetworkTypeRm {
		fn from(value: &CoreNetworkTypeRm) -> Self {
			value.clone()
		}
	}

	impl From<CoreNetworkType> for CoreNetworkTypeRm {
		fn from(value: CoreNetworkType) -> Self {
			Self::CoreNetworkType(value)
		}
	}

	impl From<NullValue> for CoreNetworkTypeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// string with format 'date' as defined in OpenAPI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'date' as defined in OpenAPI.",
	///  "type": "string",
	///  "format": "date"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Date(pub chrono::naive::NaiveDate);

	impl ::std::ops::Deref for Date {
		type Target = chrono::naive::NaiveDate;
		fn deref(&self) -> &chrono::naive::NaiveDate {
			&self.0
		}
	}

	impl From<Date> for chrono::naive::NaiveDate {
		fn from(value: Date) -> Self {
			value.0
		}
	}

	impl From<&Date> for Date {
		fn from(value: &Date) -> Self {
			value.clone()
		}
	}

	impl From<chrono::naive::NaiveDate> for Date {
		fn from(value: chrono::naive::NaiveDate) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Date {
		type Err = <chrono::naive::NaiveDate as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Date {
		type Error = <chrono::naive::NaiveDate as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Date {
		type Error = <chrono::naive::NaiveDate as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Date {
		type Error = <chrono::naive::NaiveDate as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Date {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// string with format 'date' as defined in OpenAPI OpenAPI with 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'date' as defined in OpenAPI OpenAPI
	/// with 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "format": "date"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DateRm(pub Option<chrono::naive::NaiveDate>);

	impl ::std::ops::Deref for DateRm {
		type Target = Option<chrono::naive::NaiveDate>;
		fn deref(&self) -> &Option<chrono::naive::NaiveDate> {
			&self.0
		}
	}

	impl From<DateRm> for Option<chrono::naive::NaiveDate> {
		fn from(value: DateRm) -> Self {
			value.0
		}
	}

	impl From<&DateRm> for DateRm {
		fn from(value: &DateRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<chrono::naive::NaiveDate>> for DateRm {
		fn from(value: Option<chrono::naive::NaiveDate>) -> Self {
			Self(value)
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

	/// string with format 'date-time' as defined in OpenAPI with
	/// 'nullable:true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'date-time' as defined in OpenAPI
	/// with 'nullable:true' property. \n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "format": "date-time"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DateTimeRm(pub Option<chrono::DateTime<chrono::offset::Utc>>);

	impl ::std::ops::Deref for DateTimeRm {
		type Target = Option<chrono::DateTime<chrono::offset::Utc>>;
		fn deref(&self) -> &Option<chrono::DateTime<chrono::offset::Utc>> {
			&self.0
		}
	}

	impl From<DateTimeRm> for Option<chrono::DateTime<chrono::offset::Utc>> {
		fn from(value: DateTimeRm) -> Self {
			value.0
		}
	}

	impl From<&DateTimeRm> for DateTimeRm {
		fn from(value: &DateTimeRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<chrono::DateTime<chrono::offset::Utc>>> for DateTimeRm {
		fn from(value: Option<chrono::DateTime<chrono::offset::Utc>>) -> Self {
			Self(value)
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

	/// DiameterIdentity
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/Fqdn"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DiameterIdentity(pub Fqdn);

	impl ::std::ops::Deref for DiameterIdentity {
		type Target = Fqdn;
		fn deref(&self) -> &Fqdn {
			&self.0
		}
	}

	impl From<DiameterIdentity> for Fqdn {
		fn from(value: DiameterIdentity) -> Self {
			value.0
		}
	}

	impl From<&DiameterIdentity> for DiameterIdentity {
		fn from(value: &DiameterIdentity) -> Self {
			value.clone()
		}
	}

	impl From<Fqdn> for DiameterIdentity {
		fn from(value: Fqdn) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for DiameterIdentity {
		type Err = <Fqdn as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for DiameterIdentity {
		type Error = <Fqdn as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DiameterIdentity {
		type Error = <Fqdn as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DiameterIdentity {
		type Error = <Fqdn as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for DiameterIdentity {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// DiameterIdentityRm
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/FqdnRm"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DiameterIdentityRm(pub FqdnRm);

	impl ::std::ops::Deref for DiameterIdentityRm {
		type Target = FqdnRm;
		fn deref(&self) -> &FqdnRm {
			&self.0
		}
	}

	impl From<DiameterIdentityRm> for FqdnRm {
		fn from(value: DiameterIdentityRm) -> Self {
			value.0
		}
	}

	impl From<&DiameterIdentityRm> for DiameterIdentityRm {
		fn from(value: &DiameterIdentityRm) -> Self {
			value.clone()
		}
	}

	impl From<FqdnRm> for DiameterIdentityRm {
		fn from(value: FqdnRm) -> Self {
			Self(value)
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

	/// This data type is defined in the same way as the ' DlDataDeliveryStatus
	/// ' data type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the '
	/// DlDataDeliveryStatus ' data type, but with the OpenAPI 'nullable: true'
	/// property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/DlDataDeliveryStatus"
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
	pub enum DlDataDeliveryStatusRm {
		#[default]
		DlDataDeliveryStatus(DlDataDeliveryStatus),
		NullValue(NullValue),
	}

	impl From<&DlDataDeliveryStatusRm> for DlDataDeliveryStatusRm {
		fn from(value: &DlDataDeliveryStatusRm) -> Self {
			value.clone()
		}
	}

	impl From<DlDataDeliveryStatus> for DlDataDeliveryStatusRm {
		fn from(value: DlDataDeliveryStatus) -> Self {
			Self::DlDataDeliveryStatus(value)
		}
	}

	impl From<NullValue> for DlDataDeliveryStatusRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// It can take the values  as specified for DnaiChangeType but with the
	/// OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It can take the values  as specified for DnaiChangeType
	/// but with the OpenAPI 'nullable: true' property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/DnaiChangeType"
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
	pub enum DnaiChangeTypeRm {
		#[default]
		DnaiChangeType(DnaiChangeType),
		NullValue(NullValue),
	}

	impl From<&DnaiChangeTypeRm> for DnaiChangeTypeRm {
		fn from(value: &DnaiChangeTypeRm) -> Self {
			value.clone()
		}
	}

	impl From<DnaiChangeType> for DnaiChangeTypeRm {
		fn from(value: DnaiChangeType) -> Self {
			Self::DnaiChangeType(value)
		}
	}

	impl From<NullValue> for DnaiChangeTypeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// This data type is defined in the same way as the 'Dnai' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Dnai'
	/// data type, but with the OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnaiRm(pub Option<String>);

	impl ::std::ops::Deref for DnaiRm {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<DnaiRm> for Option<String> {
		fn from(value: DnaiRm) -> Self {
			value.0
		}
	}

	impl From<&DnaiRm> for DnaiRm {
		fn from(value: &DnaiRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for DnaiRm {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// A disjunctive normal form.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A disjunctive normal form.",
	///  "type": "object",
	///  "required": [
	///    "dnfUnits"
	///  ],
	///  "properties": {
	///    "dnfUnits": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DnfUnit"
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
	pub struct Dnf {
		#[serde(rename = "dnfUnits")]
		pub dnf_units: Vec<DnfUnit>,
	}

	impl From<&Dnf> for Dnf {
		fn from(value: &Dnf) -> Self {
			value.clone()
		}
	}

	/// During the processing of dnfUnits attribute, all the members in the
	/// array shall be  interpreted as logically concatenated with logical "OR".
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "During the processing of dnfUnits attribute, all the
	/// members in the array shall be  interpreted as logically concatenated
	/// with logical \"OR\".\n",
	///  "type": "object",
	///  "required": [
	///    "dnfUnit"
	///  ],
	///  "properties": {
	///    "dnfUnit": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Atom"
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
	pub struct DnfUnit {
		#[serde(rename = "dnfUnit")]
		pub dnf_unit: Vec<Atom>,
	}

	impl From<&DnfUnit> for DnfUnit {
		fn from(value: &DnfUnit) -> Self {
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

	/// String representing a Data Network as defined in clause 9A of 3GPP TS
	/// 23.003;  it shall contain either a DNN Network Identifier, or a full DNN
	/// with both the  Network Identifier and Operator Identifier, as specified
	/// in 3GPP TS 23.003 clause 9.1.1  and 9.1.2. It shall be coded as string
	/// in which the labels are separated by dots  (e.g. 'Label1.Label2.Label3')
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing a Data Network as defined in clause 9A of 3GPP TS 23.003;  it shall contain either a DNN Network Identifier, or a full DNN with both the  Network Identifier and Operator Identifier, as specified in 3GPP TS 23.003 clause 9.1.1  and 9.1.2. It shall be coded as string in which the labels are separated by dots  (e.g. 'Label1.Label2.Label3') with the OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnRm(pub Option<String>);

	impl ::std::ops::Deref for DnnRm {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<DnnRm> for Option<String> {
		fn from(value: DnnRm) -> Self {
			value.0
		}
	}

	impl From<&DnnRm> for DnnRm {
		fn from(value: &DnnRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for DnnRm {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// Double
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'double' as defined in OpenAPI",
	///  "type": "number",
	///  "format": "double"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Double(pub f64);

	impl ::std::ops::Deref for Double {
		type Target = f64;
		fn deref(&self) -> &f64 {
			&self.0
		}
	}

	impl From<Double> for f64 {
		fn from(value: Double) -> Self {
			value.0
		}
	}

	impl From<&Double> for Double {
		fn from(value: &Double) -> Self {
			value.clone()
		}
	}

	impl From<f64> for Double {
		fn from(value: f64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Double {
		type Err = <f64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Double {
		type Error = <f64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Double {
		type Error = <f64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Double {
		type Error = <f64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Double {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// string with format 'double' as defined in OpenAPI with 'nullable: true'
	/// property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'double' as defined in OpenAPI with
	/// 'nullable: true' property.\n",
	///  "type": [
	///    "number",
	///    "null"
	///  ],
	///  "format": "double"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DoubleRm(pub Option<f64>);

	impl ::std::ops::Deref for DoubleRm {
		type Target = Option<f64>;
		fn deref(&self) -> &Option<f64> {
			&self.0
		}
	}

	impl From<DoubleRm> for Option<f64> {
		fn from(value: DoubleRm) -> Self {
			value.0
		}
	}

	impl From<&DoubleRm> for DoubleRm {
		fn from(value: &DoubleRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<f64>> for DoubleRm {
		fn from(value: Option<f64>) -> Self {
			Self(value)
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

	/// indicating a time in seconds with OpenAPI defined 'nullable: true'
	/// property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "indicating a time in seconds with OpenAPI defined
	/// 'nullable: true' property.",
	///  "type": [
	///    "integer",
	///    "null"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DurationSecRm(pub Option<i64>);

	impl ::std::ops::Deref for DurationSecRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<DurationSecRm> for Option<i64> {
		fn from(value: DurationSecRm) -> Self {
			value.0
		}
	}

	impl From<&DurationSecRm> for DurationSecRm {
		fn from(value: &DurationSecRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for DurationSecRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
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
		NewUnchecked,
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

	/// This data type is defined in the same way as the 'Ecgi' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Ecgi'
	/// data type, but with the OpenAPI 'nullable: true' property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Ecgi"
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
	pub enum EcgiRm {
		#[default]
		Ecgi(Ecgi),
		NullValue(NullValue),
	}

	impl From<&EcgiRm> for EcgiRm {
		fn from(value: &EcgiRm) -> Self {
			value.clone()
		}
	}

	impl From<Ecgi> for EcgiRm {
		fn from(value: Ecgi) -> Self {
			Self::Ecgi(value)
		}
	}

	impl From<NullValue> for EcgiRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Contains the Edge Configuration Server Address Configuration Information
	/// as defined in clause 5.2.3.6.1 of 3GPP TS 23.502.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Edge Configuration Server Address
	/// Configuration Information as defined in clause 5.2.3.6.1 of 3GPP TS
	/// 23.502.\n",
	///  "type": "object",
	///  "properties": {
	///    "ecsFqdnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Fqdn"
	///      },
	///      "minItems": 1
	///    },
	///    "ecsIpAddressList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpAddr"
	///      },
	///      "minItems": 1
	///    },
	///    "ecsProviderId": {
	///      "type": "string"
	///    },
	///    "ecsUriList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uri"
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
	pub struct EcsServerAddr {
		#[serde(rename = "ecsFqdnList", default, skip_serializing_if = "Vec::is_empty")]
		pub ecs_fqdn_list: Vec<Fqdn>,
		#[serde(
			rename = "ecsIpAddressList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ecs_ip_address_list: Vec<IpAddr>,
		#[serde(
			rename = "ecsProviderId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ecs_provider_id: Option<String>,
		#[serde(rename = "ecsUriList", default, skip_serializing_if = "Vec::is_empty")]
		pub ecs_uri_list: Vec<Uri>,
	}

	impl From<&EcsServerAddr> for EcsServerAddr {
		fn from(value: &EcsServerAddr) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the ' EcsServerAddr ' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the '
	/// EcsServerAddr ' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/EcsServerAddr"
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
	pub enum EcsServerAddrRm {
		#[default]
		EcsServerAddr(EcsServerAddr),
		NullValue(NullValue),
	}

	impl From<&EcsServerAddrRm> for EcsServerAddrRm {
		fn from(value: &EcsServerAddrRm) -> Self {
			value.clone()
		}
	}

	impl From<EcsServerAddr> for EcsServerAddrRm {
		fn from(value: EcsServerAddr) -> Self {
			Self::EcsServerAddr(value)
		}
	}

	impl From<NullValue> for EcsServerAddrRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Ellipsoid Arc.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Ellipsoid Arc.",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/GADShape"
	///    },
	///    {
	///      "type": "object",
	///      "required": [
	///        "confidence",
	///        "includedAngle",
	///        "innerRadius",
	///        "offsetAngle",
	///        "point",
	///        "uncertaintyRadius"
	///      ],
	///      "properties": {
	///        "confidence": {
	///          "$ref": "#/components/schemas/Confidence"
	///        },
	///        "includedAngle": {
	///          "$ref": "#/components/schemas/Angle"
	///        },
	///        "innerRadius": {
	///          "$ref": "#/components/schemas/InnerRadius"
	///        },
	///        "offsetAngle": {
	///          "$ref": "#/components/schemas/Angle"
	///        },
	///        "point": {
	///          "$ref": "#/components/schemas/GeographicalCoordinates"
	///        },
	///        "uncertaintyRadius": {
	///          "$ref": "#/components/schemas/Uncertainty"
	///        }
	///      }
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EllipsoidArc {
		pub confidence: Confidence,
		#[serde(rename = "includedAngle")]
		pub included_angle: Angle,
		#[serde(rename = "innerRadius")]
		pub inner_radius: InnerRadius,
		#[serde(rename = "offsetAngle")]
		pub offset_angle: Angle,
		pub point: GeographicalCoordinates,
		pub shape: SupportedGadShapes,
		#[serde(rename = "uncertaintyRadius")]
		pub uncertainty_radius: Uncertainty,
	}

	impl From<&EllipsoidArc> for EllipsoidArc {
		fn from(value: &EllipsoidArc) -> Self {
			value.clone()
		}
	}

	/// Empty JSON object { }, it is defined with the keyword
	/// additionalProperties false
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Empty JSON object { }, it is defined with the keyword
	/// additionalProperties false",
	///  "type": "object",
	///  "additionalProperties": false
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(deny_unknown_fields)]
	pub struct EmptyObject {}

	impl From<&EmptyObject> for EmptyObject {
		fn from(value: &EmptyObject) -> Self {
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
		NewUnchecked,
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

	/// This data type is defined in the same way as the 'EutraCellId' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'EutraCellId' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^[A-Fa-f0-9]{7}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EutraCellIdRm(pub Option<EutraCellIdRmInner>);

	impl ::std::ops::Deref for EutraCellIdRm {
		type Target = Option<EutraCellIdRmInner>;
		fn deref(&self) -> &Option<EutraCellIdRmInner> {
			&self.0
		}
	}

	impl From<EutraCellIdRm> for Option<EutraCellIdRmInner> {
		fn from(value: EutraCellIdRm) -> Self {
			value.0
		}
	}

	impl From<&EutraCellIdRm> for EutraCellIdRm {
		fn from(value: &EutraCellIdRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<EutraCellIdRmInner>> for EutraCellIdRm {
		fn from(value: Option<EutraCellIdRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the 'EutraCellId' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'EutraCellId' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
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
		NewUnchecked,
	)]
	pub struct EutraCellIdRmInner(String);

	impl ::std::ops::Deref for EutraCellIdRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<EutraCellIdRmInner> for String {
		fn from(value: EutraCellIdRmInner) -> Self {
			value.0
		}
	}

	impl From<&EutraCellIdRmInner> for EutraCellIdRmInner {
		fn from(value: &EutraCellIdRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for EutraCellIdRmInner {
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

	impl ::std::convert::TryFrom<&str> for EutraCellIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for EutraCellIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for EutraCellIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for EutraCellIdRmInner {
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
		NewUnchecked,
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
		NewUnchecked,
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

	/// This data type is defined in the same way as the 'EutraLocation' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'EutraLocation' data type, but with the OpenAPI 'nullable: true'
	/// property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/EutraLocation"
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
	pub enum EutraLocationRm {
		#[default]
		EutraLocation(EutraLocation),
		NullValue(NullValue),
	}

	impl From<&EutraLocationRm> for EutraLocationRm {
		fn from(value: &EutraLocationRm) -> Self {
			value.clone()
		}
	}

	impl From<EutraLocation> for EutraLocationRm {
		fn from(value: EutraLocation) -> Self {
			Self::EutraLocation(value)
		}
	}

	impl From<NullValue> for EutraLocationRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// The enumeration EventForMdt defines events triggered measurement for
	/// logged MDT in the trace. See 3GPP TS 32.422 for further description of
	/// the values. It shall comply with the provisions defined in table
	/// 5.6.3.11-1
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration EventForMdt defines events triggered
	/// measurement for logged MDT in the trace. See 3GPP TS 32.422 for further
	/// description of the values. It shall comply with the provisions defined
	/// in table 5.6.3.11-1\n",
	///  "type": "string",
	///  "enum": [
	///    "OUT_OF_COVERAG",
	///    "A2_EVENT"
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
	pub enum EventForMdt {
		#[default]
		#[serde(rename = "OUT_OF_COVERAG")]
		OutOfCoverag,
		#[serde(rename = "A2_EVENT")]
		A2Event,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&EventForMdt> for EventForMdt {
		fn from(value: &EventForMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for EventForMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::OutOfCoverag => "OUT_OF_COVERAG".to_string(),
				Self::A2Event => "A2_EVENT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for EventForMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"OUT_OF_COVERAG" => Ok(Self::OutOfCoverag),
				"A2_EVENT" => Ok(Self::A2Event),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for EventForMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for EventForMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for EventForMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// This data type is defined in the same way as the 'ExtMaxDataBurstVol'
	/// data type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'ExtMaxDataBurstVol' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 2000000.0,
	///  "minimum": 4096.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ExtMaxDataBurstVolRm(pub Option<i64>);

	impl ::std::ops::Deref for ExtMaxDataBurstVolRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<ExtMaxDataBurstVolRm> for Option<i64> {
		fn from(value: ExtMaxDataBurstVolRm) -> Self {
			value.0
		}
	}

	impl From<&ExtMaxDataBurstVolRm> for ExtMaxDataBurstVolRm {
		fn from(value: &ExtMaxDataBurstVolRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for ExtMaxDataBurstVolRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
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

	/// This data type is defined in the same way as the 'ExtPacketDelBudget'
	/// data type, but with the OpenAPI 'nullable: true' property. "
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'ExtPacketDelBudget' data type, but with the OpenAPI 'nullable: true'
	/// property. \"\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ExtPacketDelBudgetRm(pub Option<std::num::NonZeroU64>);

	impl ::std::ops::Deref for ExtPacketDelBudgetRm {
		type Target = Option<std::num::NonZeroU64>;
		fn deref(&self) -> &Option<std::num::NonZeroU64> {
			&self.0
		}
	}

	impl From<ExtPacketDelBudgetRm> for Option<std::num::NonZeroU64> {
		fn from(value: ExtPacketDelBudgetRm) -> Self {
			value.0
		}
	}

	impl From<&ExtPacketDelBudgetRm> for ExtPacketDelBudgetRm {
		fn from(value: &ExtPacketDelBudgetRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<std::num::NonZeroU64>> for ExtPacketDelBudgetRm {
		fn from(value: Option<std::num::NonZeroU64>) -> Self {
			Self(value)
		}
	}

	/// The sdRanges and wildcardSd attributes shall be exclusive from each
	/// other. If one of these attributes is present,  the sd attribute shall
	/// also be present and it shall contain one Slice Differentiator value
	/// within the range of SD  (if the sdRanges attribute is present) or with
	/// any value (if the wildcardSd attribute is present).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The sdRanges and wildcardSd attributes shall be
	/// exclusive from each other. If one of these attributes is present,  the
	/// sd attribute shall also be present and it shall contain one Slice
	/// Differentiator value within the range of SD  (if the sdRanges attribute
	/// is present) or with any value (if the wildcardSd attribute is
	/// present).\n",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    {
	///      "$ref": "#/components/schemas/SnssaiExtension"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ExtSnssai {
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
		pub sd: Option<ExtSnssaiSd>,
		/// Unsigned integer, within the range 0 to 255, representing the
		/// Slice/Service Type.  It indicates the expected Network Slice
		/// behaviour in terms of features and services. Values 0 to 127
		/// correspond to the standardized SST range. Values 128 to 255
		/// correspond  to the Operator-specific range. See clause 28.4.2 of
		/// 3GPP TS 23.003. Standardized values are defined in clause 5.15.2.2
		/// of 3GPP TS 23.501.
		pub sst: u8,
	}

	impl From<&ExtSnssai> for ExtSnssai {
		fn from(value: &ExtSnssai) -> Self {
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
		NewUnchecked,
	)]
	pub struct ExtSnssaiSd(String);

	impl ::std::ops::Deref for ExtSnssaiSd {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ExtSnssaiSd> for String {
		fn from(value: ExtSnssaiSd) -> Self {
			value.0
		}
	}

	impl From<&ExtSnssaiSd> for ExtSnssaiSd {
		fn from(value: &ExtSnssaiSd) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ExtSnssaiSd {
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

	impl ::std::convert::TryFrom<&str> for ExtSnssaiSd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ExtSnssaiSd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ExtSnssaiSd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ExtSnssaiSd {
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

	/// String identifying External Group Identifier that identifies a group
	/// made up of one or more  subscriptions associated to a group of IMSIs, as
	/// specified in clause 19.7.3 of 3GPP TS 23.003.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying External Group Identifier that identifies a group made up of one or more  subscriptions associated to a group of IMSIs, as specified in clause 19.7.3 of 3GPP TS 23.003. \n",
	///  "type": "string",
	///  "pattern": "^extgroupid-[^@]+@[^@]+$"
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
	pub struct ExternalGroupId(String);

	impl ::std::ops::Deref for ExternalGroupId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ExternalGroupId> for String {
		fn from(value: ExternalGroupId) -> Self {
			value.0
		}
	}

	impl From<&ExternalGroupId> for ExternalGroupId {
		fn from(value: &ExternalGroupId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ExternalGroupId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^extgroupid-[^@]+@[^@]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^extgroupid-[^@]+@[^@]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for ExternalGroupId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ExternalGroupId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ExternalGroupId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ExternalGroupId {
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

	/// String identifying External Group Identifier that identifies a group
	/// made up of one or more  subscriptions associated to a group of IMSIs, as
	/// specified in clause 19.7.3 of  3GPP TS 23.003  with the OpenAPI
	/// 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying External Group Identifier that
	/// identifies a group made up of one or more  subscriptions associated to a
	/// group of IMSIs, as specified in clause 19.7.3 of  3GPP TS 23.003  with
	/// the OpenAPI 'nullable: true' property. \n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^extgroupid-[^@]+@[^@]+$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ExternalGroupIdRm(pub Option<ExternalGroupIdRmInner>);

	impl ::std::ops::Deref for ExternalGroupIdRm {
		type Target = Option<ExternalGroupIdRmInner>;
		fn deref(&self) -> &Option<ExternalGroupIdRmInner> {
			&self.0
		}
	}

	impl From<ExternalGroupIdRm> for Option<ExternalGroupIdRmInner> {
		fn from(value: ExternalGroupIdRm) -> Self {
			value.0
		}
	}

	impl From<&ExternalGroupIdRm> for ExternalGroupIdRm {
		fn from(value: &ExternalGroupIdRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<ExternalGroupIdRmInner>> for ExternalGroupIdRm {
		fn from(value: Option<ExternalGroupIdRmInner>) -> Self {
			Self(value)
		}
	}

	/// String identifying External Group Identifier that identifies a group
	/// made up of one or more  subscriptions associated to a group of IMSIs, as
	/// specified in clause 19.7.3 of  3GPP TS 23.003  with the OpenAPI
	/// 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying External Group Identifier that
	/// identifies a group made up of one or more  subscriptions associated to a
	/// group of IMSIs, as specified in clause 19.7.3 of  3GPP TS 23.003  with
	/// the OpenAPI 'nullable: true' property. \n",
	///  "type": "string",
	///  "pattern": "^extgroupid-[^@]+@[^@]+$"
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
	pub struct ExternalGroupIdRmInner(String);

	impl ::std::ops::Deref for ExternalGroupIdRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ExternalGroupIdRmInner> for String {
		fn from(value: ExternalGroupIdRmInner) -> Self {
			value.0
		}
	}

	impl From<&ExternalGroupIdRmInner> for ExternalGroupIdRmInner {
		fn from(value: &ExternalGroupIdRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ExternalGroupIdRmInner {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^extgroupid-[^@]+@[^@]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^extgroupid-[^@]+@[^@]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for ExternalGroupIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ExternalGroupIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ExternalGroupIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ExternalGroupIdRmInner {
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

	/// List of geographic area or list of civic address info for MBS Service
	/// Area
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of geographic area or list of civic address info
	/// for MBS Service Area",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "geographicAreaList"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "civicAddressList"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "civicAddressList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CivicAddress"
	///      },
	///      "minItems": 1
	///    },
	///    "geographicAreaList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GeographicArea"
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
	pub enum ExternalMbsServiceArea {
		#[default]
		Variant0 {
			#[serde(rename = "geographicAreaList")]
			geographic_area_list: Vec<GeographicArea>,
		},
		Variant1 {
			#[serde(rename = "civicAddressList")]
			civic_address_list: Vec<CivicAddress>,
		},
	}

	impl From<&ExternalMbsServiceArea> for ExternalMbsServiceArea {
		fn from(value: &ExternalMbsServiceArea) -> Self {
			value.clone()
		}
	}

	/// Float
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'float' as defined in OpenAPI.",
	///  "type": "number",
	///  "format": "float"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Float(pub f32);

	impl ::std::ops::Deref for Float {
		type Target = f32;
		fn deref(&self) -> &f32 {
			&self.0
		}
	}

	impl From<Float> for f32 {
		fn from(value: Float) -> Self {
			value.0
		}
	}

	impl From<&Float> for Float {
		fn from(value: &Float) -> Self {
			value.clone()
		}
	}

	impl From<f32> for Float {
		fn from(value: f32) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Float {
		type Err = <f32 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Float {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Float {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Float {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Float {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// string with format 'float' as defined in OpenAPI with the OpenAPI
	/// defined 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'float' as defined in OpenAPI with
	/// the OpenAPI defined 'nullable: true' property.\n",
	///  "type": [
	///    "number",
	///    "null"
	///  ],
	///  "format": "float"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct FloatRm(pub Option<f32>);

	impl ::std::ops::Deref for FloatRm {
		type Target = Option<f32>;
		fn deref(&self) -> &Option<f32> {
			&self.0
		}
	}

	impl From<FloatRm> for Option<f32> {
		fn from(value: FloatRm) -> Self {
			value.0
		}
	}

	impl From<&FloatRm> for FloatRm {
		fn from(value: &FloatRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<f32>> for FloatRm {
		fn from(value: Option<f32>) -> Self {
			Self(value)
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

	/// a matching rule for a FQDN pattern
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "a matching rule for a FQDN pattern",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "regex"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "stringMatchingRule"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "regex": {
	///      "type": "string"
	///    },
	///    "stringMatchingRule": {
	///      "$ref": "#/components/schemas/StringMatchingRule"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum FqdnPatternMatchingRule {
		#[default]
		Variant0 { regex: String },
		Variant1 {
			#[serde(rename = "stringMatchingRule")]
			string_matching_rule: StringMatchingRule,
		},
	}

	impl From<&FqdnPatternMatchingRule> for FqdnPatternMatchingRule {
		fn from(value: &FqdnPatternMatchingRule) -> Self {
			value.clone()
		}
	}

	/// Fully Qualified Domain Name, but it also allows the null value
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Fully Qualified Domain Name, but it also allows the
	/// null value",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Fqdn"
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
	pub enum FqdnRm {
		Fqdn(Fqdn),
		#[default]
		NullValue(NullValue),
	}

	impl From<&FqdnRm> for FqdnRm {
		fn from(value: &FqdnRm) -> Self {
			value.clone()
		}
	}

	impl From<Fqdn> for FqdnRm {
		fn from(value: Fqdn) -> Self {
			Self::Fqdn(value)
		}
	}

	impl From<NullValue> for FqdnRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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
		NewUnchecked,
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

	/// Common base type for GAD shapes.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Common base type for GAD shapes.",
	///  "type": "object",
	///  "required": [
	///    "shape"
	///  ],
	///  "properties": {
	///    "shape": {
	///      "$ref": "#/components/schemas/SupportedGADShapes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GadShape {
		pub shape: SupportedGadShapes,
	}

	impl From<&GadShape> for GadShape {
		fn from(value: &GadShape) -> Self {
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

	/// List of geographic area or list of civic address info
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of geographic area or list of civic address info",
	///  "type": "object",
	///  "properties": {
	///    "civicAddressList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CivicAddress"
	///      },
	///      "minItems": 1
	///    },
	///    "geographicAreaList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GeographicArea"
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
	pub struct GeoServiceArea {
		#[serde(
			rename = "civicAddressList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub civic_address_list: Vec<CivicAddress>,
		#[serde(
			rename = "geographicAreaList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub geographic_area_list: Vec<GeographicArea>,
	}

	impl From<&GeoServiceArea> for GeoServiceArea {
		fn from(value: &GeoServiceArea) -> Self {
			value.clone()
		}
	}

	/// Geographic area specified by different shape.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Geographic area specified by different shape.",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Point"
	///    },
	///    {
	///      "$ref": "#/components/schemas/PointUncertaintyCircle"
	///    },
	///    {
	///      "$ref": "#/components/schemas/PointUncertaintyEllipse"
	///    },
	///    {
	///      "$ref": "#/components/schemas/Polygon"
	///    },
	///    {
	///      "$ref": "#/components/schemas/PointAltitude"
	///    },
	///    {
	///      "$ref": "#/components/schemas/PointAltitudeUncertainty"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EllipsoidArc"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GeographicArea {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Point>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<PointUncertaintyCircle>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_2: Option<PointUncertaintyEllipse>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_3: Option<Polygon>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_4: Option<PointAltitude>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_5: Option<PointAltitudeUncertainty>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_6: Option<EllipsoidArc>,
	}

	impl From<&GeographicArea> for GeographicArea {
		fn from(value: &GeographicArea) -> Self {
			value.clone()
		}
	}

	/// Geographical coordinates.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Geographical coordinates.",
	///  "type": "object",
	///  "required": [
	///    "lat",
	///    "lon"
	///  ],
	///  "properties": {
	///    "lat": {
	///      "type": "number",
	///      "format": "double",
	///      "maximum": 90.0,
	///      "minimum": -90.0
	///    },
	///    "lon": {
	///      "type": "number",
	///      "format": "double",
	///      "maximum": 180.0,
	///      "minimum": -180.0
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GeographicalCoordinates {
		pub lat: f64,
		pub lon: f64,
	}

	impl From<&GeographicalCoordinates> for GeographicalCoordinates {
		fn from(value: &GeographicalCoordinates) -> Self {
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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

	/// Gli
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/Bytes"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Gli(pub Bytes);

	impl ::std::ops::Deref for Gli {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<Gli> for Bytes {
		fn from(value: Gli) -> Self {
			value.0
		}
	}

	impl From<&Gli> for Gli {
		fn from(value: &Gli) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for Gli {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Gli {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Gli {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Gli {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Gli {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Gli {
		fn to_string(&self) -> String {
			self.0.to_string()
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
		NewUnchecked,
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

	/// String identifying a Gpsi shall contain either an External Id or an
	/// MSISDN. It shall be  formatted as follows -External Identifier=
	/// 'extid-'extid', where 'extid' shall be formatted  according to clause
	/// 19.7.2 of 3GPP TS 23.003 that describes an External Identifier with the
	/// OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a Gpsi shall contain either an
	/// External Id or an MSISDN. It shall be  formatted as follows -External
	/// Identifier= 'extid-'extid', where 'extid' shall be formatted  according
	/// to clause 19.7.2 of 3GPP TS 23.003 that describes an External Identifier
	/// with the  OpenAPI 'nullable: true' property. \n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^(msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|.+)$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GpsiRm(pub Option<GpsiRmInner>);

	impl ::std::ops::Deref for GpsiRm {
		type Target = Option<GpsiRmInner>;
		fn deref(&self) -> &Option<GpsiRmInner> {
			&self.0
		}
	}

	impl From<GpsiRm> for Option<GpsiRmInner> {
		fn from(value: GpsiRm) -> Self {
			value.0
		}
	}

	impl From<&GpsiRm> for GpsiRm {
		fn from(value: &GpsiRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<GpsiRmInner>> for GpsiRm {
		fn from(value: Option<GpsiRmInner>) -> Self {
			Self(value)
		}
	}

	/// String identifying a Gpsi shall contain either an External Id or an
	/// MSISDN. It shall be  formatted as follows -External Identifier=
	/// 'extid-'extid', where 'extid' shall be formatted  according to clause
	/// 19.7.2 of 3GPP TS 23.003 that describes an External Identifier with the
	/// OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a Gpsi shall contain either an
	/// External Id or an MSISDN. It shall be  formatted as follows -External
	/// Identifier= 'extid-'extid', where 'extid' shall be formatted  according
	/// to clause 19.7.2 of 3GPP TS 23.003 that describes an External Identifier
	/// with the  OpenAPI 'nullable: true' property. \n",
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
		NewUnchecked,
	)]
	pub struct GpsiRmInner(String);

	impl ::std::ops::Deref for GpsiRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<GpsiRmInner> for String {
		fn from(value: GpsiRmInner) -> Self {
			value.0
		}
	}

	impl From<&GpsiRmInner> for GpsiRmInner {
		fn from(value: &GpsiRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for GpsiRmInner {
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

	impl ::std::convert::TryFrom<&str> for GpsiRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for GpsiRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for GpsiRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GpsiRmInner {
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
		NewUnchecked,
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

	/// String identifying a group of devices network internal globally unique
	/// ID which identifies a set of IMSIs, as specified in clause 19.9 of 3GPP
	/// TS 23.003 with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a group of devices network internal
	/// globally unique ID which identifies a set of IMSIs, as specified in
	/// clause 19.9 of 3GPP TS 23.003 with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern":
	/// "^[A-Fa-f0-9]{8}-[0-9]{3}-[0-9]{2,3}-([A-Fa-f0-9][A-Fa-f0-9]){1,10}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GroupIdRm(pub Option<GroupIdRmInner>);

	impl ::std::ops::Deref for GroupIdRm {
		type Target = Option<GroupIdRmInner>;
		fn deref(&self) -> &Option<GroupIdRmInner> {
			&self.0
		}
	}

	impl From<GroupIdRm> for Option<GroupIdRmInner> {
		fn from(value: GroupIdRm) -> Self {
			value.0
		}
	}

	impl From<&GroupIdRm> for GroupIdRm {
		fn from(value: &GroupIdRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<GroupIdRmInner>> for GroupIdRm {
		fn from(value: Option<GroupIdRmInner>) -> Self {
			Self(value)
		}
	}

	/// String identifying a group of devices network internal globally unique
	/// ID which identifies a set of IMSIs, as specified in clause 19.9 of 3GPP
	/// TS 23.003 with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a group of devices network internal
	/// globally unique ID which identifies a set of IMSIs, as specified in
	/// clause 19.9 of 3GPP TS 23.003 with the OpenAPI 'nullable: true'
	/// property.\n",
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
		NewUnchecked,
	)]
	pub struct GroupIdRmInner(String);

	impl ::std::ops::Deref for GroupIdRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<GroupIdRmInner> for String {
		fn from(value: GroupIdRmInner) -> Self {
			value.0
		}
	}

	impl From<&GroupIdRmInner> for GroupIdRmInner {
		fn from(value: &GroupIdRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for GroupIdRmInner {
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

	impl ::std::convert::TryFrom<&str> for GroupIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for GroupIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for GroupIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GroupIdRmInner {
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

	/// Globally Unique AMF Identifier constructed out of PLMN, Network and AMF
	/// identity.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Globally Unique AMF Identifier constructed out of PLMN,
	/// Network and AMF identity.",
	///  "type": "object",
	///  "required": [
	///    "amfId",
	///    "plmnId"
	///  ],
	///  "properties": {
	///    "amfId": {
	///      "$ref": "#/components/schemas/AmfId"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Guami {
		#[serde(rename = "amfId")]
		pub amf_id: AmfId,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnIdNid,
	}

	impl From<&Guami> for Guami {
		fn from(value: &Guami) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the 'Guami' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Guami' data type, but with the OpenAPI 'nullable: true' property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Guami"
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
	pub enum GuamiRm {
		#[default]
		Guami(Guami),
		NullValue(NullValue),
	}

	impl From<&GuamiRm> for GuamiRm {
		fn from(value: &GuamiRm) -> Self {
			value.clone()
		}
	}

	impl From<Guami> for GuamiRm {
		fn from(value: Guami) -> Self {
			Self::Guami(value)
		}
	}

	impl From<NullValue> for GuamiRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Hypertext Application Language (HAL) template contains the extended 3GPP
	/// hypermedia format.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Hypertext Application Language (HAL) template contains
	/// the extended 3GPP hypermedia format.\n",
	///  "type": "object",
	///  "required": [
	///    "method"
	///  ],
	///  "properties": {
	///    "contentType": {
	///      "description": "The media type that should be used for the
	/// corresponding request. If the attribute is missing, or contains an
	/// unrecognized value, the client should act as if the  contentType is set
	/// to \"application/json\".\n",
	///      "type": "string"
	///    },
	///    "method": {
	///      "$ref": "#/components/schemas/HttpMethod"
	///    },
	///    "properties": {
	///      "description": "The properties that should be included in the body
	/// of the corresponding request.  If the contentType attribute is set to
	/// \"application/json\", then this attribute  describes the attributes of
	/// the JSON object of the body.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Property"
	///      },
	///      "minItems": 1
	///    },
	///    "title": {
	///      "description": "A human-readable string that can be used to
	/// identify this template",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HalTemplate {
		/// The media type that should be used for the corresponding request. If
		/// the attribute is missing, or contains an unrecognized value, the
		/// client should act as if the  contentType is set to
		/// "application/json".
		#[serde(
			rename = "contentType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub content_type: Option<String>,
		pub method: HttpMethod,
		/// The properties that should be included in the body of the
		/// corresponding request.  If the contentType attribute is set to
		/// "application/json", then this attribute  describes the attributes of
		/// the JSON object of the body.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub properties: Vec<Property>,
		/// A human-readable string that can be used to identify this template
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub title: Option<String>,
	}

	impl From<&HalTemplate> for HalTemplate {
		fn from(value: &HalTemplate) -> Self {
			value.clone()
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
		NewUnchecked,
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

	/// This data type is defined in the same way as the 'HfcNId' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'HfcNId' data type, but with the OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "maxLength": 6
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HfcNIdRm(pub Option<HfcNIdRmInner>);

	impl ::std::ops::Deref for HfcNIdRm {
		type Target = Option<HfcNIdRmInner>;
		fn deref(&self) -> &Option<HfcNIdRmInner> {
			&self.0
		}
	}

	impl From<HfcNIdRm> for Option<HfcNIdRmInner> {
		fn from(value: HfcNIdRm) -> Self {
			value.0
		}
	}

	impl From<&HfcNIdRm> for HfcNIdRm {
		fn from(value: &HfcNIdRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<HfcNIdRmInner>> for HfcNIdRm {
		fn from(value: Option<HfcNIdRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the 'HfcNId' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'HfcNId' data type, but with the OpenAPI 'nullable: true' property.\n",
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
		NewUnchecked,
	)]
	pub struct HfcNIdRmInner(String);

	impl ::std::ops::Deref for HfcNIdRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<HfcNIdRmInner> for String {
		fn from(value: HfcNIdRmInner) -> Self {
			value.0
		}
	}

	impl From<&HfcNIdRmInner> for HfcNIdRmInner {
		fn from(value: &HfcNIdRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for HfcNIdRmInner {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if value.len() > 6usize {
				return Err("longer than 6 characters".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for HfcNIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for HfcNIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for HfcNIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for HfcNIdRmInner {
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

	/// This data type is defined in the same way as the 'HfcNodeId' data type,
	/// but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'HfcNodeId' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/HfcNodeId"
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
	pub enum HfcNodeIdRm {
		#[default]
		HfcNodeId(HfcNodeId),
		NullValue(NullValue),
	}

	impl From<&HfcNodeIdRm> for HfcNodeIdRm {
		fn from(value: &HfcNodeIdRm) -> Self {
			value.clone()
		}
	}

	impl From<HfcNodeId> for HfcNodeIdRm {
		fn from(value: HfcNodeId) -> Self {
			Self::HfcNodeId(value)
		}
	}

	impl From<NullValue> for HfcNodeIdRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// HTTP methodes.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "HTTP methodes.",
	///  "type": "string",
	///  "enum": [
	///    "GET",
	///    "POST",
	///    "PUT",
	///    "DELETE",
	///    "PATCH",
	///    "OPTIONS",
	///    "HEAD",
	///    "CONNECT",
	///    "TRACE"
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
	pub enum HttpMethod {
		#[default]
		#[serde(rename = "GET")]
		Get,
		#[serde(rename = "POST")]
		Post,
		#[serde(rename = "PUT")]
		Put,
		#[serde(rename = "DELETE")]
		Delete,
		#[serde(rename = "PATCH")]
		Patch,
		#[serde(rename = "OPTIONS")]
		Options,
		#[serde(rename = "HEAD")]
		Head,
		#[serde(rename = "CONNECT")]
		Connect,
		#[serde(rename = "TRACE")]
		Trace,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&HttpMethod> for HttpMethod {
		fn from(value: &HttpMethod) -> Self {
			value.clone()
		}
	}

	impl ToString for HttpMethod {
		fn to_string(&self) -> String {
			match *self {
				Self::Get => "GET".to_string(),
				Self::Post => "POST".to_string(),
				Self::Put => "PUT".to_string(),
				Self::Delete => "DELETE".to_string(),
				Self::Patch => "PATCH".to_string(),
				Self::Options => "OPTIONS".to_string(),
				Self::Head => "HEAD".to_string(),
				Self::Connect => "CONNECT".to_string(),
				Self::Trace => "TRACE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for HttpMethod {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"GET" => Ok(Self::Get),
				"POST" => Ok(Self::Post),
				"PUT" => Ok(Self::Put),
				"DELETE" => Ok(Self::Delete),
				"PATCH" => Ok(Self::Patch),
				"OPTIONS" => Ok(Self::Options),
				"HEAD" => Ok(Self::Head),
				"CONNECT" => Ok(Self::Connect),
				"TRACE" => Ok(Self::Trace),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for HttpMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for HttpMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for HttpMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Ingress Tunnel Address Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Ingress Tunnel Address Information",
	///  "type": "object",
	///  "required": [
	///    "ingressTunAddr"
	///  ],
	///  "properties": {
	///    "ingressTunAddr": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TunnelAddress"
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
	pub struct IngressTunAddrInfo {
		#[serde(rename = "ingressTunAddr")]
		pub ingress_tun_addr: Vec<TunnelAddress>,
	}

	impl From<&IngressTunAddrInfo> for IngressTunAddrInfo {
		fn from(value: &IngressTunAddrInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates value of the inner radius.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates value of the inner radius.",
	///  "type": "integer",
	///  "format": "int32",
	///  "maximum": 327675.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct InnerRadius(pub i64);

	impl ::std::ops::Deref for InnerRadius {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<InnerRadius> for i64 {
		fn from(value: InnerRadius) -> Self {
			value.0
		}
	}

	impl From<&InnerRadius> for InnerRadius {
		fn from(value: &InnerRadius) -> Self {
			value.clone()
		}
	}

	impl From<i64> for InnerRadius {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for InnerRadius {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for InnerRadius {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for InnerRadius {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for InnerRadius {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for InnerRadius {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// string with format 'int32' as defined in OpenAPI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'int32' as defined in OpenAPI.",
	///  "type": "integer",
	///  "format": "int32"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Int32(pub i32);

	impl ::std::ops::Deref for Int32 {
		type Target = i32;
		fn deref(&self) -> &i32 {
			&self.0
		}
	}

	impl From<Int32> for i32 {
		fn from(value: Int32) -> Self {
			value.0
		}
	}

	impl From<&Int32> for Int32 {
		fn from(value: &Int32) -> Self {
			value.clone()
		}
	}

	impl From<i32> for Int32 {
		fn from(value: i32) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Int32 {
		type Err = <i32 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Int32 {
		type Error = <i32 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Int32 {
		type Error = <i32 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Int32 {
		type Error = <i32 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Int32 {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// string with format 'int32' as defined in OpenAPI with the OpenAPI
	/// defined 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'int32' as defined in OpenAPI with
	/// the OpenAPI defined 'nullable: true' property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "format": "int32"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Int32Rm(pub Option<i32>);

	impl ::std::ops::Deref for Int32Rm {
		type Target = Option<i32>;
		fn deref(&self) -> &Option<i32> {
			&self.0
		}
	}

	impl From<Int32Rm> for Option<i32> {
		fn from(value: Int32Rm) -> Self {
			value.0
		}
	}

	impl From<&Int32Rm> for Int32Rm {
		fn from(value: &Int32Rm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i32>> for Int32Rm {
		fn from(value: Option<i32>) -> Self {
			Self(value)
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

	/// string with format 'int64' as defined in OpenAPI with the OpenAPI
	/// defined 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "string with format 'int64' as defined in OpenAPI with
	/// the OpenAPI defined 'nullable: true' property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "format": "int64"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Int64Rm(pub Option<i64>);

	impl ::std::ops::Deref for Int64Rm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<Int64Rm> for Option<i64> {
		fn from(value: Int64Rm) -> Self {
			value.0
		}
	}

	impl From<&Int64Rm> for Int64Rm {
		fn from(value: &Int64Rm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for Int64Rm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
		}
	}

	/// Indicates the Inter Frequency Target information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the Inter Frequency Target information.",
	///  "type": "object",
	///  "required": [
	///    "dlCarrierFreq"
	///  ],
	///  "properties": {
	///    "cellIdList": {
	///      "description": "When present, this IE shall contain a list of the
	/// physical cell identities where the UE is requested to perform
	/// measurement logging for the indicated frequency.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PhysCellId"
	///      },
	///      "maxItems": 32,
	///      "minItems": 1
	///    },
	///    "dlCarrierFreq": {
	///      "$ref": "#/components/schemas/ArfcnValueNR"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct InterFreqTargetInfo {
		/// When present, this IE shall contain a list of the physical cell
		/// identities where the UE is requested to perform measurement logging
		/// for the indicated frequency.
		#[serde(rename = "cellIdList", default, skip_serializing_if = "Vec::is_empty")]
		pub cell_id_list: Vec<PhysCellId>,
		#[serde(rename = "dlCarrierFreq")]
		pub dl_carrier_freq: ArfcnValueNr,
	}

	impl From<&InterFreqTargetInfo> for InterFreqTargetInfo {
		fn from(value: &InterFreqTargetInfo) -> Self {
			value.clone()
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

	/// String identifying a IPv4 address mask formatted in the 'dotted decimal'
	/// notation as defined in RFC 1166 with the OpenAPI defined 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a IPv4 address mask formatted in the
	/// 'dotted decimal' notation as defined in RFC 1166 with the OpenAPI
	/// defined 'nullable: true' property.\n",
	///  "examples": [
	///    "198.51.0.0/16"
	///  ],
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern":
	/// "^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.
	/// ){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])(\\/
	/// ([0-9]|[1-2][0-9]|3[0-2]))$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Ipv4AddrMaskRm(pub Option<Ipv4AddrMask>);

	impl ::std::ops::Deref for Ipv4AddrMaskRm {
		type Target = Option<Ipv4AddrMask>;
		fn deref(&self) -> &Option<Ipv4AddrMask> {
			&self.0
		}
	}

	impl From<Ipv4AddrMaskRm> for Option<Ipv4AddrMask> {
		fn from(value: Ipv4AddrMaskRm) -> Self {
			value.0
		}
	}

	impl From<&Ipv4AddrMaskRm> for Ipv4AddrMaskRm {
		fn from(value: &Ipv4AddrMaskRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<Ipv4AddrMask>> for Ipv4AddrMaskRm {
		fn from(value: Option<Ipv4AddrMask>) -> Self {
			Self(value)
		}
	}

	/// String identifying a IPv4 address formatted in the 'dotted decimal'
	/// notation as defined in RFC 1166 with the OpenAPI defined 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a IPv4 address formatted in the
	/// 'dotted decimal' notation as defined in RFC 1166 with the OpenAPI
	/// defined 'nullable: true' property.\n",
	///  "examples": [
	///    "198.51.100.1"
	///  ],
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern":
	/// "^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.
	/// ){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Ipv4AddrRm(pub Option<Ipv4Addr>);

	impl ::std::ops::Deref for Ipv4AddrRm {
		type Target = Option<Ipv4Addr>;
		fn deref(&self) -> &Option<Ipv4Addr> {
			&self.0
		}
	}

	impl From<Ipv4AddrRm> for Option<Ipv4Addr> {
		fn from(value: Ipv4AddrRm) -> Self {
			value.0
		}
	}

	impl From<&Ipv4AddrRm> for Ipv4AddrRm {
		fn from(value: &Ipv4AddrRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<Ipv4Addr>> for Ipv4AddrRm {
		fn from(value: Option<Ipv4Addr>) -> Self {
			Self(value)
		}
	}

	/// String identifying an IPv6 address formatted according to clause 4 of
	/// RFC5952 with the OpenAPI 'nullable: true' property. The mixed IPv4 IPv6
	/// notation according to clause 5 of RFC5952 shall not be used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying an IPv6 address formatted according
	/// to clause 4 of RFC5952 with the OpenAPI 'nullable: true' property. The
	/// mixed IPv4 IPv6 notation according to clause 5 of RFC5952 shall not be
	/// used.\n",
	///  "examples": [
	///    "2001:db8:85a3::8a2e:370:7334"
	///  ],
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern":
	/// "(?=.*^((:|(0?|([1-9a-f][0-9a-f]{0,3}))):)((0?|([1-9a-f][0-9a-f]{0,3})):
	/// ){0,6}(:|(0?|([1-9a-f][0-9a-f]{0,3})))$)(?=.*^((([^:]+:){7}([^:
	/// ]+))|((([^:]+:)*[^:]+)?::(([^:]+:)*[^:]+)?))$)"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Ipv6AddrRm(pub Option<Ipv6Addr>);

	impl ::std::ops::Deref for Ipv6AddrRm {
		type Target = Option<Ipv6Addr>;
		fn deref(&self) -> &Option<Ipv6Addr> {
			&self.0
		}
	}

	impl From<Ipv6AddrRm> for Option<Ipv6Addr> {
		fn from(value: Ipv6AddrRm) -> Self {
			value.0
		}
	}

	impl From<&Ipv6AddrRm> for Ipv6AddrRm {
		fn from(value: &Ipv6AddrRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<Ipv6Addr>> for Ipv6AddrRm {
		fn from(value: Option<Ipv6Addr>) -> Self {
			Self(value)
		}
	}

	/// String identifying an IPv6 address prefix formatted according to clause
	/// 4 of RFC 5952 with the OpenAPI 'nullable: true' property. IPv6Prefix
	/// data type may contain an individual /128 IPv6 address.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying an IPv6 address prefix formatted
	/// according to clause 4 of RFC 5952 with the OpenAPI 'nullable: true'
	/// property. IPv6Prefix data type may contain an individual /128 IPv6
	/// address.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern":
	/// "(?=.*^((:|(0?|([1-9a-f][0-9a-f]{0,3}))):)((0?|([1-9a-f][0-9a-f]{0,3})):
	/// ){0,6}(:|(0?|([1-9a-f][0-9a-f]{0,3})))(\\/
	/// (([0-9])|([0-9]{2})|(1[0-1][0-9])|(12[0-8])))$)(?=.*^((([^:]+:){7}([^:
	/// ]+))|((([^:]+:)*[^:]+)?::(([^:]+:)*[^:]+)?))(\\/.+)$)"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Ipv6PrefixRm(pub Option<Ipv6Prefix>);

	impl ::std::ops::Deref for Ipv6PrefixRm {
		type Target = Option<Ipv6Prefix>;
		fn deref(&self) -> &Option<Ipv6Prefix> {
			&self.0
		}
	}

	impl From<Ipv6PrefixRm> for Option<Ipv6Prefix> {
		fn from(value: Ipv6PrefixRm) -> Self {
			value.0
		}
	}

	impl From<&Ipv6PrefixRm> for Ipv6PrefixRm {
		fn from(value: &Ipv6PrefixRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<Ipv6Prefix>> for Ipv6PrefixRm {
		fn from(value: Option<Ipv6Prefix>) -> Self {
			Self(value)
		}
	}

	/// The enumeration JobType defines Job Type in the trace. See 3GPP TS
	/// 32.422 for further  description of the values. It shall comply with the
	/// provisions defined in table 5.6.3.3-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration JobType defines Job Type in the trace.
	/// See 3GPP TS 32.422 for further  description of the values. It shall
	/// comply with the provisions defined in table 5.6.3.3-1.\n",
	///  "type": "string",
	///  "enum": [
	///    "IMMEDIATE_MDT_ONLY",
	///    "LOGGED_MDT_ONLY",
	///    "TRACE_ONLY",
	///    "IMMEDIATE_MDT_AND_TRACE",
	///    "RLF_REPORTS_ONLY",
	///    "RCEF_REPORTS_ONLY",
	///    "LOGGED_MBSFN_MDT"
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
	pub enum JobType {
		#[default]
		#[serde(rename = "IMMEDIATE_MDT_ONLY")]
		ImmediateMdtOnly,
		#[serde(rename = "LOGGED_MDT_ONLY")]
		LoggedMdtOnly,
		#[serde(rename = "TRACE_ONLY")]
		TraceOnly,
		#[serde(rename = "IMMEDIATE_MDT_AND_TRACE")]
		ImmediateMdtAndTrace,
		#[serde(rename = "RLF_REPORTS_ONLY")]
		RlfReportsOnly,
		#[serde(rename = "RCEF_REPORTS_ONLY")]
		RcefReportsOnly,
		#[serde(rename = "LOGGED_MBSFN_MDT")]
		LoggedMbsfnMdt,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&JobType> for JobType {
		fn from(value: &JobType) -> Self {
			value.clone()
		}
	}

	impl ToString for JobType {
		fn to_string(&self) -> String {
			match *self {
				Self::ImmediateMdtOnly => "IMMEDIATE_MDT_ONLY".to_string(),
				Self::LoggedMdtOnly => "LOGGED_MDT_ONLY".to_string(),
				Self::TraceOnly => "TRACE_ONLY".to_string(),
				Self::ImmediateMdtAndTrace => "IMMEDIATE_MDT_AND_TRACE".to_string(),
				Self::RlfReportsOnly => "RLF_REPORTS_ONLY".to_string(),
				Self::RcefReportsOnly => "RCEF_REPORTS_ONLY".to_string(),
				Self::LoggedMbsfnMdt => "LOGGED_MBSFN_MDT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for JobType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IMMEDIATE_MDT_ONLY" => Ok(Self::ImmediateMdtOnly),
				"LOGGED_MDT_ONLY" => Ok(Self::LoggedMdtOnly),
				"TRACE_ONLY" => Ok(Self::TraceOnly),
				"IMMEDIATE_MDT_AND_TRACE" => Ok(Self::ImmediateMdtAndTrace),
				"RLF_REPORTS_ONLY" => Ok(Self::RlfReportsOnly),
				"RCEF_REPORTS_ONLY" => Ok(Self::RcefReportsOnly),
				"LOGGED_MBSFN_MDT" => Ok(Self::LoggedMbsfnMdt),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for JobType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for JobType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for JobType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - "LOCATION_ALLOWED_WITH_NOTIFICATION": Location allowed with
	///   notification
	/// - "LOCATION_ALLOWED_WITHOUT_NOTIFICATION": Location allowed without
	///   notification
	/// - "LOCATION_ALLOWED_WITHOUT_RESPONSE": Location with notification and
	///   privacy verification; location allowed if no response
	/// - "LOCATION_RESTRICTED_WITHOUT_RESPONSE": Location with notification and
	///   privacy
	///  verification; location restricted if no response
	/// - "NOTIFICATION_ONLY": Notification only
	/// - "NOTIFICATION_AND_VERIFICATION_ONLY": Notification and privacy
	///   verification only
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n-
	/// \"LOCATION_ALLOWED_WITH_NOTIFICATION\": Location allowed with
	/// notification\n- \"LOCATION_ALLOWED_WITHOUT_NOTIFICATION\": Location
	/// allowed without notification\n- \"LOCATION_ALLOWED_WITHOUT_RESPONSE\":
	/// Location with notification and privacy\n   verification; location
	/// allowed if no response\n- \"LOCATION_RESTRICTED_WITHOUT_RESPONSE\":
	/// Location with notification and privacy\n  verification; location
	/// restricted if no response\n- \"NOTIFICATION_ONLY\": Notification only\n-
	/// \"NOTIFICATION_AND_VERIFICATION_ONLY\": Notification and privacy
	/// verification only\n",
	///  "type": "string",
	///  "enum": [
	///    "LOCATION_ALLOWED_WITH_NOTIFICATION",
	///    "LOCATION_ALLOWED_WITHOUT_NOTIFICATION",
	///    "LOCATION_ALLOWED_WITHOUT_RESPONSE",
	///    "LOCATION_RESTRICTED_WITHOUT_RESPONSE",
	///    "NOTIFICATION_ONLY",
	///    "NOTIFICATION_AND_VERIFICATION_ONLY"
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
	pub enum LcsServiceAuth {
		#[default]
		#[serde(rename = "LOCATION_ALLOWED_WITH_NOTIFICATION")]
		LocationAllowedWithNotification,
		#[serde(rename = "LOCATION_ALLOWED_WITHOUT_NOTIFICATION")]
		LocationAllowedWithoutNotification,
		#[serde(rename = "LOCATION_ALLOWED_WITHOUT_RESPONSE")]
		LocationAllowedWithoutResponse,
		#[serde(rename = "LOCATION_RESTRICTED_WITHOUT_RESPONSE")]
		LocationRestrictedWithoutResponse,
		#[serde(rename = "NOTIFICATION_ONLY")]
		NotificationOnly,
		#[serde(rename = "NOTIFICATION_AND_VERIFICATION_ONLY")]
		NotificationAndVerificationOnly,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LcsServiceAuth> for LcsServiceAuth {
		fn from(value: &LcsServiceAuth) -> Self {
			value.clone()
		}
	}

	impl ToString for LcsServiceAuth {
		fn to_string(&self) -> String {
			match *self {
				Self::LocationAllowedWithNotification => {
					"LOCATION_ALLOWED_WITH_NOTIFICATION".to_string()
				}
				Self::LocationAllowedWithoutNotification => {
					"LOCATION_ALLOWED_WITHOUT_NOTIFICATION".to_string()
				}
				Self::LocationAllowedWithoutResponse => {
					"LOCATION_ALLOWED_WITHOUT_RESPONSE".to_string()
				}
				Self::LocationRestrictedWithoutResponse => {
					"LOCATION_RESTRICTED_WITHOUT_RESPONSE".to_string()
				}
				Self::NotificationOnly => "NOTIFICATION_ONLY".to_string(),
				Self::NotificationAndVerificationOnly => {
					"NOTIFICATION_AND_VERIFICATION_ONLY".to_string()
				}
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LcsServiceAuth {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOCATION_ALLOWED_WITH_NOTIFICATION" => Ok(Self::LocationAllowedWithNotification),
				"LOCATION_ALLOWED_WITHOUT_NOTIFICATION" => {
					Ok(Self::LocationAllowedWithoutNotification)
				}
				"LOCATION_ALLOWED_WITHOUT_RESPONSE" => Ok(Self::LocationAllowedWithoutResponse),
				"LOCATION_RESTRICTED_WITHOUT_RESPONSE" => {
					Ok(Self::LocationRestrictedWithoutResponse)
				}
				"NOTIFICATION_ONLY" => Ok(Self::NotificationOnly),
				"NOTIFICATION_AND_VERIFICATION_ONLY" => Ok(Self::NotificationAndVerificationOnly),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LcsServiceAuth {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LcsServiceAuth {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LcsServiceAuth {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// This data type is defined in the same way as the 'LineType' data type,
	/// but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'LineType' data type, but with the OpenAPI 'nullable: true' property.
	/// \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/LineType"
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
	pub enum LineTypeRm {
		#[default]
		LineType(LineType),
		NullValue(NullValue),
	}

	impl From<&LineTypeRm> for LineTypeRm {
		fn from(value: &LineTypeRm) -> Self {
			value.clone()
		}
	}

	impl From<LineType> for LineTypeRm {
		fn from(value: LineType) -> Self {
			Self::LineType(value)
		}
	}

	impl From<NullValue> for LineTypeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// It contains the URI of the linked resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It contains the URI of the linked resource.",
	///  "type": "object",
	///  "properties": {
	///    "href": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Link {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub href: Option<Uri>,
	}

	impl From<&Link> for Link {
		fn from(value: &Link) -> Self {
			value.clone()
		}
	}

	/// It contains the URI of the linked resource with the OpenAPI 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It contains the URI of the linked resource with the
	/// OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "href": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LinkRm(pub Option<LinkRmInner>);

	impl ::std::ops::Deref for LinkRm {
		type Target = Option<LinkRmInner>;
		fn deref(&self) -> &Option<LinkRmInner> {
			&self.0
		}
	}

	impl From<LinkRm> for Option<LinkRmInner> {
		fn from(value: LinkRm) -> Self {
			value.0
		}
	}

	impl From<&LinkRm> for LinkRm {
		fn from(value: &LinkRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<LinkRmInner>> for LinkRm {
		fn from(value: Option<LinkRmInner>) -> Self {
			Self(value)
		}
	}

	/// It contains the URI of the linked resource with the OpenAPI 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It contains the URI of the linked resource with the
	/// OpenAPI 'nullable: true' property.\n",
	///  "type": "object",
	///  "properties": {
	///    "href": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LinkRmInner {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub href: Option<Uri>,
	}

	impl From<&LinkRmInner> for LinkRmInner {
		fn from(value: &LinkRmInner) -> Self {
			value.clone()
		}
	}

	/// A list of mutually exclusive alternatives of 1 or more links.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A list of mutually exclusive alternatives of 1 or more
	/// links.",
	///  "oneOf": [
	///    {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Link"
	///      },
	///      "minItems": 1
	///    },
	///    {
	///      "$ref": "#/components/schemas/Link"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum LinksValueSchema {
		#[default]
		Variant0(Vec<Link>),
		Variant1(Link),
	}

	impl From<&LinksValueSchema> for LinksValueSchema {
		fn from(value: &LinksValueSchema) -> Self {
			value.clone()
		}
	}

	impl From<Vec<Link>> for LinksValueSchema {
		fn from(value: Vec<Link>) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<Link> for LinksValueSchema {
		fn from(value: Link) -> Self {
			Self::Variant1(value)
		}
	}

	/// Local 2D point with uncertainty ellipse
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Local 2D point with uncertainty ellipse",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/GADShape"
	///    },
	///    {
	///      "type": "object",
	///      "required": [
	///        "confidence",
	///        "localOrigin",
	///        "point",
	///        "uncertaintyEllipse"
	///      ],
	///      "properties": {
	///        "confidence": {
	///          "$ref": "#/components/schemas/Confidence"
	///        },
	///        "localOrigin": {
	///          "$ref": "#/components/schemas/LocalOrigin"
	///        },
	///        "point": {
	///          "$ref": "#/components/schemas/RelativeCartesianLocation"
	///        },
	///        "uncertaintyEllipse": {
	///          "$ref": "#/components/schemas/UncertaintyEllipse"
	///        }
	///      }
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Local2dPointUncertaintyEllipse {
		pub confidence: Confidence,
		#[serde(rename = "localOrigin")]
		pub local_origin: LocalOrigin,
		pub point: RelativeCartesianLocation,
		pub shape: SupportedGadShapes,
		#[serde(rename = "uncertaintyEllipse")]
		pub uncertainty_ellipse: UncertaintyEllipse,
	}

	impl From<&Local2dPointUncertaintyEllipse> for Local2dPointUncertaintyEllipse {
		fn from(value: &Local2dPointUncertaintyEllipse) -> Self {
			value.clone()
		}
	}

	/// Local 3D point with uncertainty ellipsoid
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Local 3D point with uncertainty ellipsoid",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/GADShape"
	///    },
	///    {
	///      "type": "object",
	///      "required": [
	///        "confidence",
	///        "localOrigin",
	///        "point",
	///        "uncertaintyEllipsoid"
	///      ],
	///      "properties": {
	///        "confidence": {
	///          "$ref": "#/components/schemas/Confidence"
	///        },
	///        "localOrigin": {
	///          "$ref": "#/components/schemas/LocalOrigin"
	///        },
	///        "point": {
	///          "$ref": "#/components/schemas/RelativeCartesianLocation"
	///        },
	///        "uncertaintyEllipsoid": {
	///          "$ref": "#/components/schemas/UncertaintyEllipsoid"
	///        }
	///      }
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Local3dPointUncertaintyEllipsoid {
		pub confidence: Confidence,
		#[serde(rename = "localOrigin")]
		pub local_origin: LocalOrigin,
		pub point: RelativeCartesianLocation,
		pub shape: SupportedGadShapes,
		#[serde(rename = "uncertaintyEllipsoid")]
		pub uncertainty_ellipsoid: UncertaintyEllipsoid,
	}

	impl From<&Local3dPointUncertaintyEllipsoid> for Local3dPointUncertaintyEllipsoid {
		fn from(value: &Local3dPointUncertaintyEllipsoid) -> Self {
			value.clone()
		}
	}

	/// Indicates a Local origin in a reference system
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates a Local origin in a reference system",
	///  "type": "object",
	///  "properties": {
	///    "coordinateId": {
	///      "type": "string"
	///    },
	///    "point": {
	///      "$ref": "#/components/schemas/GeographicalCoordinates"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LocalOrigin {
		#[serde(
			rename = "coordinateId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub coordinate_id: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub point: Option<GeographicalCoordinates>,
	}

	impl From<&LocalOrigin> for LocalOrigin {
		fn from(value: &LocalOrigin) -> Self {
			value.clone()
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
		NewUnchecked,
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

	/// The enumeration LoggingIntervalMdt defines Logging Interval for MDT in
	/// the trace. See 3GPP TS 32.422 for further description of the values. It
	/// shall comply with the provisions defined in table 5.6.3.12-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration LoggingIntervalMdt defines Logging
	/// Interval for MDT in the trace. See 3GPP TS 32.422 for further
	/// description of the values. It shall comply with the provisions defined
	/// in table 5.6.3.12-1.\n",
	///  "type": "string",
	///  "enum": [
	///    600,
	///    1200,
	///    2400,
	///    3600,
	///    5400,
	///    7200
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
	pub enum LoggingDurationMdt {
		#[default]
		#[serde(rename = "600")]
		NUM600,
		#[serde(rename = "1200")]
		NUM1200,
		#[serde(rename = "2400")]
		NUM2400,
		#[serde(rename = "3600")]
		NUM3600,
		#[serde(rename = "5400")]
		NUM5400,
		#[serde(rename = "7200")]
		NUM7200,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LoggingDurationMdt> for LoggingDurationMdt {
		fn from(value: &LoggingDurationMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for LoggingDurationMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::NUM600 => "600".to_string(),
				Self::NUM1200 => "1200".to_string(),
				Self::NUM2400 => "2400".to_string(),
				Self::NUM3600 => "3600".to_string(),
				Self::NUM5400 => "5400".to_string(),
				Self::NUM7200 => "7200".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LoggingDurationMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"600" => Ok(Self::NUM600),
				"1200" => Ok(Self::NUM1200),
				"2400" => Ok(Self::NUM2400),
				"3600" => Ok(Self::NUM3600),
				"5400" => Ok(Self::NUM5400),
				"7200" => Ok(Self::NUM7200),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LoggingDurationMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LoggingDurationMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LoggingDurationMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration LoggingDurationMdt defines Logging Duration in NR for
	/// MDT in the trace. See 3GPP TS 32.422 for further description of the
	/// values. It shall comply with the provisions defined in table 5.6.3.20-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration LoggingDurationMdt defines Logging
	/// Duration in NR for MDT in the trace. See 3GPP TS 32.422 for further
	/// description of the values. It shall comply with the provisions defined
	/// in table 5.6.3.20-1.\n",
	///  "type": "string",
	///  "enum": [
	///    600,
	///    1200,
	///    2400,
	///    3600,
	///    5400,
	///    7200
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
	pub enum LoggingDurationNrMdt {
		#[default]
		#[serde(rename = "600")]
		NUM600,
		#[serde(rename = "1200")]
		NUM1200,
		#[serde(rename = "2400")]
		NUM2400,
		#[serde(rename = "3600")]
		NUM3600,
		#[serde(rename = "5400")]
		NUM5400,
		#[serde(rename = "7200")]
		NUM7200,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LoggingDurationNrMdt> for LoggingDurationNrMdt {
		fn from(value: &LoggingDurationNrMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for LoggingDurationNrMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::NUM600 => "600".to_string(),
				Self::NUM1200 => "1200".to_string(),
				Self::NUM2400 => "2400".to_string(),
				Self::NUM3600 => "3600".to_string(),
				Self::NUM5400 => "5400".to_string(),
				Self::NUM7200 => "7200".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LoggingDurationNrMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"600" => Ok(Self::NUM600),
				"1200" => Ok(Self::NUM1200),
				"2400" => Ok(Self::NUM2400),
				"3600" => Ok(Self::NUM3600),
				"5400" => Ok(Self::NUM5400),
				"7200" => Ok(Self::NUM7200),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LoggingDurationNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LoggingDurationNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LoggingDurationNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration LoggingIntervalMdt defines Logging Interval for MDT in
	/// the trace. See 3GPP TS 32.422 for further description of the values. It
	/// shall comply with the provisions defined in table 5.6.3.12-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration LoggingIntervalMdt defines Logging
	/// Interval for MDT in the trace. See 3GPP TS 32.422 for further
	/// description of the values. It shall comply with the provisions defined
	/// in table 5.6.3.12-1.\n",
	///  "type": "string",
	///  "enum": [
	///    128,
	///    256,
	///    512,
	///    1024,
	///    2048,
	///    3072,
	///    4096,
	///    6144
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
	pub enum LoggingIntervalMdt {
		#[default]
		#[serde(rename = "128")]
		NUM128,
		#[serde(rename = "256")]
		NUM256,
		#[serde(rename = "512")]
		NUM512,
		#[serde(rename = "1024")]
		NUM1024,
		#[serde(rename = "2048")]
		NUM2048,
		#[serde(rename = "3072")]
		NUM3072,
		#[serde(rename = "4096")]
		NUM4096,
		#[serde(rename = "6144")]
		NUM6144,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LoggingIntervalMdt> for LoggingIntervalMdt {
		fn from(value: &LoggingIntervalMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for LoggingIntervalMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::NUM128 => "128".to_string(),
				Self::NUM256 => "256".to_string(),
				Self::NUM512 => "512".to_string(),
				Self::NUM1024 => "1024".to_string(),
				Self::NUM2048 => "2048".to_string(),
				Self::NUM3072 => "3072".to_string(),
				Self::NUM4096 => "4096".to_string(),
				Self::NUM6144 => "6144".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LoggingIntervalMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"128" => Ok(Self::NUM128),
				"256" => Ok(Self::NUM256),
				"512" => Ok(Self::NUM512),
				"1024" => Ok(Self::NUM1024),
				"2048" => Ok(Self::NUM2048),
				"3072" => Ok(Self::NUM3072),
				"4096" => Ok(Self::NUM4096),
				"6144" => Ok(Self::NUM6144),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LoggingIntervalMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LoggingIntervalMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LoggingIntervalMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration LoggingIntervalNrMdt defines Logging Interval in NR for
	/// MDT in the trace. See 3GPP TS 32.422 for further description of the
	/// values. It shall comply with the provisions defined in table 5.6.3.18-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration LoggingIntervalNrMdt defines Logging
	/// Interval in NR for MDT in the trace. See 3GPP TS 32.422 for further
	/// description of the values. It shall comply with the provisions defined
	/// in table 5.6.3.18-1.\n",
	///  "type": "string",
	///  "enum": [
	///    128,
	///    256,
	///    512,
	///    1024,
	///    2048,
	///    3072,
	///    4096,
	///    6144,
	///    320,
	///    640,
	///    "infinity"
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
	pub enum LoggingIntervalNrMdt {
		#[default]
		#[serde(rename = "128")]
		NUM128,
		#[serde(rename = "256")]
		NUM256,
		#[serde(rename = "512")]
		NUM512,
		#[serde(rename = "1024")]
		NUM1024,
		#[serde(rename = "2048")]
		NUM2048,
		#[serde(rename = "3072")]
		NUM3072,
		#[serde(rename = "4096")]
		NUM4096,
		#[serde(rename = "6144")]
		NUM6144,
		#[serde(rename = "320")]
		NUM320,
		#[serde(rename = "640")]
		NUM640,
		#[serde(rename = "infinity")]
		Infinity,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LoggingIntervalNrMdt> for LoggingIntervalNrMdt {
		fn from(value: &LoggingIntervalNrMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for LoggingIntervalNrMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::NUM128 => "128".to_string(),
				Self::NUM256 => "256".to_string(),
				Self::NUM512 => "512".to_string(),
				Self::NUM1024 => "1024".to_string(),
				Self::NUM2048 => "2048".to_string(),
				Self::NUM3072 => "3072".to_string(),
				Self::NUM4096 => "4096".to_string(),
				Self::NUM6144 => "6144".to_string(),
				Self::NUM320 => "320".to_string(),
				Self::NUM640 => "640".to_string(),
				Self::Infinity => "infinity".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LoggingIntervalNrMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"128" => Ok(Self::NUM128),
				"256" => Ok(Self::NUM256),
				"512" => Ok(Self::NUM512),
				"1024" => Ok(Self::NUM1024),
				"2048" => Ok(Self::NUM2048),
				"3072" => Ok(Self::NUM3072),
				"4096" => Ok(Self::NUM4096),
				"6144" => Ok(Self::NUM6144),
				"320" => Ok(Self::NUM320),
				"640" => Ok(Self::NUM640),
				"infinity" => Ok(Self::Infinity),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LoggingIntervalNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LoggingIntervalNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LoggingIntervalNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains LTE V2X services authorized information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains LTE V2X services authorized information.",
	///  "type": "object",
	///  "properties": {
	///    "pedestrianUeAuth": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "vehicleUeAuth": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LteV2xAuth {
		#[serde(
			rename = "pedestrianUeAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pedestrian_ue_auth: Option<UeAuth>,
		#[serde(
			rename = "vehicleUeAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vehicle_ue_auth: Option<UeAuth>,
	}

	impl From<&LteV2xAuth> for LteV2xAuth {
		fn from(value: &LteV2xAuth) -> Self {
			value.clone()
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
		NewUnchecked,
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

	/// "String identifying a MAC address formatted in the hexadecimal notation
	/// according to clause 1.1 and clause 2.1 of RFC 7042 with the OpenAPI
	/// 'nullable: true' property."
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "\"String identifying a MAC address formatted in the
	/// hexadecimal notation according to clause 1.1 and clause 2.1 of RFC 7042
	/// with the OpenAPI 'nullable: true' property.\"\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^([0-9a-fA-F]{2})((-[0-9a-fA-F]{2}){5})$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MacAddr48Rm(pub Option<MacAddr48RmInner>);

	impl ::std::ops::Deref for MacAddr48Rm {
		type Target = Option<MacAddr48RmInner>;
		fn deref(&self) -> &Option<MacAddr48RmInner> {
			&self.0
		}
	}

	impl From<MacAddr48Rm> for Option<MacAddr48RmInner> {
		fn from(value: MacAddr48Rm) -> Self {
			value.0
		}
	}

	impl From<&MacAddr48Rm> for MacAddr48Rm {
		fn from(value: &MacAddr48Rm) -> Self {
			value.clone()
		}
	}

	impl From<Option<MacAddr48RmInner>> for MacAddr48Rm {
		fn from(value: Option<MacAddr48RmInner>) -> Self {
			Self(value)
		}
	}

	/// "String identifying a MAC address formatted in the hexadecimal notation
	/// according to clause 1.1 and clause 2.1 of RFC 7042 with the OpenAPI
	/// 'nullable: true' property."
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "\"String identifying a MAC address formatted in the
	/// hexadecimal notation according to clause 1.1 and clause 2.1 of RFC 7042
	/// with the OpenAPI 'nullable: true' property.\"\n",
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
		NewUnchecked,
	)]
	pub struct MacAddr48RmInner(String);

	impl ::std::ops::Deref for MacAddr48RmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<MacAddr48RmInner> for String {
		fn from(value: MacAddr48RmInner) -> Self {
			value.0
		}
	}

	impl From<&MacAddr48RmInner> for MacAddr48RmInner {
		fn from(value: &MacAddr48RmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for MacAddr48RmInner {
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

	impl ::std::convert::TryFrom<&str> for MacAddr48RmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for MacAddr48RmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for MacAddr48RmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for MacAddr48RmInner {
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

	/// ManAssiUeRadioCapId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/Bytes"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ManAssiUeRadioCapId(pub Bytes);

	impl ::std::ops::Deref for ManAssiUeRadioCapId {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<ManAssiUeRadioCapId> for Bytes {
		fn from(value: ManAssiUeRadioCapId) -> Self {
			value.0
		}
	}

	impl From<&ManAssiUeRadioCapId> for ManAssiUeRadioCapId {
		fn from(value: &ManAssiUeRadioCapId) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for ManAssiUeRadioCapId {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ManAssiUeRadioCapId {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ManAssiUeRadioCapId {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ManAssiUeRadioCapId {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ManAssiUeRadioCapId {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ManAssiUeRadioCapId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// the matching operation.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "the matching operation.",
	///  "type": "string",
	///  "enum": [
	///    "FULL_MATCH",
	///    "MATCH_ALL",
	///    "STARTS_WITH",
	///    "NOT_START_WITH",
	///    "ENDS_WITH",
	///    "NOT_END_WITH",
	///    "CONTAINS",
	///    "NOT_CONTAIN"
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
	pub enum MatchingOperator {
		#[default]
		#[serde(rename = "FULL_MATCH")]
		FullMatch,
		#[serde(rename = "MATCH_ALL")]
		MatchAll,
		#[serde(rename = "STARTS_WITH")]
		StartsWith,
		#[serde(rename = "NOT_START_WITH")]
		NotStartWith,
		#[serde(rename = "ENDS_WITH")]
		EndsWith,
		#[serde(rename = "NOT_END_WITH")]
		NotEndWith,
		#[serde(rename = "CONTAINS")]
		Contains,
		#[serde(rename = "NOT_CONTAIN")]
		NotContain,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MatchingOperator> for MatchingOperator {
		fn from(value: &MatchingOperator) -> Self {
			value.clone()
		}
	}

	impl ToString for MatchingOperator {
		fn to_string(&self) -> String {
			match *self {
				Self::FullMatch => "FULL_MATCH".to_string(),
				Self::MatchAll => "MATCH_ALL".to_string(),
				Self::StartsWith => "STARTS_WITH".to_string(),
				Self::NotStartWith => "NOT_START_WITH".to_string(),
				Self::EndsWith => "ENDS_WITH".to_string(),
				Self::NotEndWith => "NOT_END_WITH".to_string(),
				Self::Contains => "CONTAINS".to_string(),
				Self::NotContain => "NOT_CONTAIN".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MatchingOperator {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"FULL_MATCH" => Ok(Self::FullMatch),
				"MATCH_ALL" => Ok(Self::MatchAll),
				"STARTS_WITH" => Ok(Self::StartsWith),
				"NOT_START_WITH" => Ok(Self::NotStartWith),
				"ENDS_WITH" => Ok(Self::EndsWith),
				"NOT_END_WITH" => Ok(Self::NotEndWith),
				"CONTAINS" => Ok(Self::Contains),
				"NOT_CONTAIN" => Ok(Self::NotContain),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MatchingOperator {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MatchingOperator {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MatchingOperator {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// This data type is defined in the same way as the 'MaxDataBurstVol' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'MaxDataBurstVol' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 4095.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MaxDataBurstVolRm(pub Option<i64>);

	impl ::std::ops::Deref for MaxDataBurstVolRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<MaxDataBurstVolRm> for Option<i64> {
		fn from(value: MaxDataBurstVolRm) -> Self {
			value.0
		}
	}

	impl From<&MaxDataBurstVolRm> for MaxDataBurstVolRm {
		fn from(value: &MaxDataBurstVolRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for MaxDataBurstVolRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
		}
	}

	/// MBS Frequency Selection Area Identifier
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS Frequency Selection Area Identifier",
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
		NewUnchecked,
	)]
	pub struct MbsFsaId(String);

	impl ::std::ops::Deref for MbsFsaId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<MbsFsaId> for String {
		fn from(value: MbsFsaId) -> Self {
			value.0
		}
	}

	impl From<&MbsFsaId> for MbsFsaId {
		fn from(value: &MbsFsaId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for MbsFsaId {
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

	impl ::std::convert::TryFrom<&str> for MbsFsaId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for MbsFsaId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for MbsFsaId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for MbsFsaId {
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

	/// MBS Security Key Data Structure
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS Security Key Data Structure",
	///  "type": "object",
	///  "required": [
	///    "keyDomainId",
	///    "mskId"
	///  ],
	///  "properties": {
	///    "keyDomainId": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "msk": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "mskId": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "mskLifetime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "mtk": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "mtkId": {
	///      "$ref": "#/components/schemas/Bytes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsKeyInfo {
		#[serde(rename = "keyDomainId")]
		pub key_domain_id: Bytes,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub msk: Option<Bytes>,
		#[serde(rename = "mskId")]
		pub msk_id: Bytes,
		#[serde(
			rename = "mskLifetime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub msk_lifetime: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub mtk: Option<Bytes>,
		#[serde(rename = "mtkId", default, skip_serializing_if = "Option::is_none")]
		pub mtk_id: Option<Bytes>,
	}

	impl From<&MbsKeyInfo> for MbsKeyInfo {
		fn from(value: &MbsKeyInfo) -> Self {
			value.clone()
		}
	}

	/// Represents an MBS Media Component.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an MBS Media Component.",
	///  "type": "object",
	///  "required": [
	///    "mbsMedCompNum"
	///  ],
	///  "properties": {
	///    "mbsFlowDescs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/FlowDescription"
	///      },
	///      "minItems": 1
	///    },
	///    "mbsMedCompNum": {
	///      "type": "integer"
	///    },
	///    "mbsMediaInfo": {
	///      "$ref": "#/components/schemas/MbsMediaInfo"
	///    },
	///    "mbsQoSReq": {
	///      "$ref": "#/components/schemas/MbsQoSReq"
	///    },
	///    "mbsSdfResPrio": {
	///      "$ref": "#/components/schemas/ReservPriority"
	///    },
	///    "qosRef": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsMediaComp {
		#[serde(
			rename = "mbsFlowDescs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mbs_flow_descs: Vec<FlowDescription>,
		#[serde(rename = "mbsMedCompNum")]
		pub mbs_med_comp_num: i64,
		#[serde(
			rename = "mbsMediaInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_media_info: Option<MbsMediaInfo>,
		#[serde(rename = "mbsQoSReq", default, skip_serializing_if = "Option::is_none")]
		pub mbs_qo_s_req: Option<MbsQoSReq>,
		#[serde(
			rename = "mbsSdfResPrio",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_sdf_res_prio: Option<ReservPriority>,
		#[serde(rename = "qosRef", default, skip_serializing_if = "Option::is_none")]
		pub qos_ref: Option<String>,
	}

	impl From<&MbsMediaComp> for MbsMediaComp {
		fn from(value: &MbsMediaComp) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the MbsMediaComp data type,
	/// but with the OpenAPI nullable property set to true.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// MbsMediaComp data type, but with the OpenAPI nullable property set to
	/// true.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/MbsMediaComp"
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
	pub enum MbsMediaCompRm {
		#[default]
		MbsMediaComp(MbsMediaComp),
		NullValue(NullValue),
	}

	impl From<&MbsMediaCompRm> for MbsMediaCompRm {
		fn from(value: &MbsMediaCompRm) -> Self {
			value.clone()
		}
	}

	impl From<MbsMediaComp> for MbsMediaCompRm {
		fn from(value: MbsMediaComp) -> Self {
			Self::MbsMediaComp(value)
		}
	}

	impl From<NullValue> for MbsMediaCompRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Represent MBS Media Information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represent MBS Media Information.",
	///  "type": "object",
	///  "properties": {
	///    "codecs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CodecData"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "maxReqMbsBwDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "mbsMedType": {
	///      "$ref": "#/components/schemas/MediaType"
	///    },
	///    "minReqMbsBwDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsMediaInfo {
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub codecs: Vec<CodecData>,
		#[serde(
			rename = "maxReqMbsBwDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_req_mbs_bw_dl: Option<BitRate>,
		#[serde(
			rename = "mbsMedType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_med_type: Option<MediaType>,
		#[serde(
			rename = "minReqMbsBwDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub min_req_mbs_bw_dl: Option<BitRate>,
	}

	impl From<&MbsMediaInfo> for MbsMediaInfo {
		fn from(value: &MbsMediaInfo) -> Self {
			value.clone()
		}
	}

	/// Represent MBS QoS requirements.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represent MBS QoS requirements.",
	///  "type": "object",
	///  "required": [
	///    "5qi"
	///  ],
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "averWindow": {
	///      "$ref": "#/components/schemas/AverWindow"
	///    },
	///    "guarBitRate": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "maxBitRate": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "reqMbsArp": {
	///      "$ref": "#/components/schemas/Arp"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsQoSReq {
		#[serde(
			rename = "averWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub aver_window: Option<AverWindow>,
		#[serde(rename = "5qi")]
		pub five_qi: _5qi,
		#[serde(
			rename = "guarBitRate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub guar_bit_rate: Option<BitRate>,
		#[serde(
			rename = "maxBitRate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_bit_rate: Option<BitRate>,
		#[serde(rename = "reqMbsArp", default, skip_serializing_if = "Option::is_none")]
		pub req_mbs_arp: Option<Arp>,
	}

	impl From<&MbsQoSReq> for MbsQoSReq {
		fn from(value: &MbsQoSReq) -> Self {
			value.clone()
		}
	}

	/// MbsSecurityContext
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "keyList"
	///  ],
	///  "properties": {
	///    "keyList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of MbsSecurityContext",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MbsKeyInfo"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsSecurityContext {
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of MbsSecurityContext
		#[serde(rename = "keyList")]
		pub key_list: ::std::collections::HashMap<String, MbsKeyInfo>,
	}

	impl From<&MbsSecurityContext> for MbsSecurityContext {
		fn from(value: &MbsSecurityContext) -> Self {
			value.clone()
		}
	}

	/// MBS Service Area
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS Service Area",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "ncgiList"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "taiList"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "ncgiList": {
	///      "description": "List of NR cell Ids",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NcgiTai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "description": "List of tracking area Ids",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
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
	pub enum MbsServiceArea {
		#[default]
		Variant0 {
			/// List of NR cell Ids
			#[serde(rename = "ncgiList")]
			ncgi_list: Vec<NcgiTai>,
		},
		Variant1 {
			/// List of tracking area Ids
			#[serde(rename = "taiList")]
			tai_list: Vec<Tai>,
		},
	}

	impl From<&MbsServiceArea> for MbsServiceArea {
		fn from(value: &MbsServiceArea) -> Self {
			value.clone()
		}
	}

	/// MBS Service Area Information for location dependent MBS session
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS Service Area Information for location dependent MBS
	/// session",
	///  "type": "object",
	///  "required": [
	///    "areaSessionId",
	///    "mbsServiceArea"
	///  ],
	///  "properties": {
	///    "areaSessionId": {
	///      "$ref": "#/components/schemas/AreaSessionId"
	///    },
	///    "mbsServiceArea": {
	///      "$ref": "#/components/schemas/MbsServiceArea"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsServiceAreaInfo {
		#[serde(rename = "areaSessionId")]
		pub area_session_id: AreaSessionId,
		#[serde(rename = "mbsServiceArea")]
		pub mbs_service_area: MbsServiceArea,
	}

	impl From<&MbsServiceAreaInfo> for MbsServiceAreaInfo {
		fn from(value: &MbsServiceAreaInfo) -> Self {
			value.clone()
		}
	}

	/// Represent MBS Service Information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represent MBS Service Information.",
	///  "type": "object",
	///  "required": [
	///    "mbsMediaComps"
	///  ],
	///  "properties": {
	///    "afAppId": {
	///      "$ref": "#/components/schemas/AfAppId"
	///    },
	///    "mbsMediaComps": {
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MbsMediaCompRm"
	///      }
	///    },
	///    "mbsSdfResPrio": {
	///      "$ref": "#/components/schemas/ReservPriority"
	///    },
	///    "mbsSessionAmbr": {
	///      "$ref": "#/components/schemas/BitRate"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsServiceInfo {
		#[serde(rename = "afAppId", default, skip_serializing_if = "Option::is_none")]
		pub af_app_id: Option<AfAppId>,
		#[serde(rename = "mbsMediaComps")]
		pub mbs_media_comps: ::std::collections::HashMap<String, MbsMediaCompRm>,
		#[serde(
			rename = "mbsSdfResPrio",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_sdf_res_prio: Option<ReservPriority>,
		#[serde(
			rename = "mbsSessionAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_session_ambr: Option<BitRate>,
	}

	impl From<&MbsServiceInfo> for MbsServiceInfo {
		fn from(value: &MbsServiceInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates the MBS service type of an MBS session
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the MBS service type of an MBS session",
	///  "type": "string",
	///  "enum": [
	///    "MULTICAST",
	///    "BROADCAST"
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
	pub enum MbsServiceType {
		#[default]
		#[serde(rename = "MULTICAST")]
		Multicast,
		#[serde(rename = "BROADCAST")]
		Broadcast,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MbsServiceType> for MbsServiceType {
		fn from(value: &MbsServiceType) -> Self {
			value.clone()
		}
	}

	impl ToString for MbsServiceType {
		fn to_string(&self) -> String {
			match *self {
				Self::Multicast => "MULTICAST".to_string(),
				Self::Broadcast => "BROADCAST".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MbsServiceType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MULTICAST" => Ok(Self::Multicast),
				"BROADCAST" => Ok(Self::Broadcast),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MbsServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MbsServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MbsServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Individual MBS session
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Individual MBS session",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "mbsSessionId"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "tmgiAllocReq"
	///      ]
	///    }
	///  ],
	///  "not": {
	///    "required": [
	///      "extRedMbsServArea",
	///      "redMbsServArea"
	///    ]
	///  },
	///  "required": [
	///    "serviceType"
	///  ],
	///  "properties": {
	///    "activationTime": {
	///      "deprecated": true,
	///      "type": "string",
	///      "format": "date-time"
	///    },
	///    "activityStatus": {
	///      "$ref": "#/components/schemas/MbsSessionActivityStatus"
	///    },
	///    "anyUeInd": {
	///      "default": false,
	///      "writeOnly": true,
	///      "type": "boolean"
	///    },
	///    "areaSessionId": {
	///      "$ref": "#/components/schemas/AreaSessionId"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "expirationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "extMbsServiceArea": {
	///      "$ref": "#/components/schemas/ExternalMbsServiceArea"
	///    },
	///    "extRedMbsServArea": {
	///      "$ref": "#/components/schemas/ExternalMbsServiceArea"
	///    },
	///    "ingressTunAddr": {
	///      "readOnly": true,
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TunnelAddress"
	///      },
	///      "minItems": 1
	///    },
	///    "ingressTunAddrReq": {
	///      "default": false,
	///      "writeOnly": true,
	///      "type": "boolean"
	///    },
	///    "locationDependent": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "mbsFsaIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MbsFsaId"
	///      },
	///      "minItems": 1
	///    },
	///    "mbsServInfo": {
	///      "$ref": "#/components/schemas/MbsServiceInfo"
	///    },
	///    "mbsServiceArea": {
	///      "$ref": "#/components/schemas/MbsServiceArea"
	///    },
	///    "mbsSessionId": {
	///      "$ref": "#/components/schemas/MbsSessionId"
	///    },
	///    "mbsSessionSubsc": {
	///      "$ref": "#/components/schemas/MbsSessionSubscription"
	///    },
	///    "redMbsServArea": {
	///      "$ref": "#/components/schemas/MbsServiceArea"
	///    },
	///    "serviceType": {
	///      "$ref": "#/components/schemas/MbsServiceType"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "ssm": {
	///      "$ref": "#/components/schemas/Ssm"
	///    },
	///    "startTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "terminationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "tmgi": {
	///      "$ref": "#/components/schemas/Tmgi"
	///    },
	///    "tmgiAllocReq": {
	///      "default": false,
	///      "writeOnly": true,
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum MbsSession {
		#[default]
		Variant0 {
			#[serde(
				rename = "activationTime",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			activation_time: Option<chrono::DateTime<chrono::offset::Utc>>,
			#[serde(
				rename = "activityStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			activity_status: Option<MbsSessionActivityStatus>,
			#[serde(rename = "anyUeInd", default)]
			any_ue_ind: bool,
			#[serde(
				rename = "areaSessionId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			area_session_id: Option<AreaSessionId>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(
				rename = "expirationTime",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			expiration_time: Option<DateTime>,
			#[serde(
				rename = "extMbsServiceArea",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ext_mbs_service_area: Option<ExternalMbsServiceArea>,
			#[serde(
				rename = "ingressTunAddr",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			ingress_tun_addr: Vec<TunnelAddress>,
			#[serde(rename = "ingressTunAddrReq", default)]
			ingress_tun_addr_req: bool,
			#[serde(rename = "locationDependent", default)]
			location_dependent: bool,
			#[serde(
				rename = "mbsFsaIdList",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			mbs_fsa_id_list: Vec<MbsFsaId>,
			#[serde(
				rename = "mbsServInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			mbs_serv_info: Option<MbsServiceInfo>,
			#[serde(
				rename = "mbsServiceArea",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			mbs_service_area: Option<MbsServiceArea>,
			#[serde(rename = "mbsSessionId")]
			mbs_session_id: MbsSessionId,
			#[serde(
				rename = "mbsSessionSubsc",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			mbs_session_subsc: Option<MbsSessionSubscription>,
			#[serde(rename = "serviceType")]
			service_type: MbsServiceType,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			ssm: Option<Ssm>,
			#[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
			start_time: Option<DateTime>,
			#[serde(
				rename = "terminationTime",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			termination_time: Option<DateTime>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			tmgi: Option<Tmgi>,
		},
		Variant1 {
			#[serde(
				rename = "activationTime",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			activation_time: Option<chrono::DateTime<chrono::offset::Utc>>,
			#[serde(
				rename = "activityStatus",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			activity_status: Option<MbsSessionActivityStatus>,
			#[serde(rename = "anyUeInd", default)]
			any_ue_ind: bool,
			#[serde(
				rename = "areaSessionId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			area_session_id: Option<AreaSessionId>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(
				rename = "expirationTime",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			expiration_time: Option<DateTime>,
			#[serde(
				rename = "extMbsServiceArea",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ext_mbs_service_area: Option<ExternalMbsServiceArea>,
			#[serde(
				rename = "ingressTunAddr",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			ingress_tun_addr: Vec<TunnelAddress>,
			#[serde(rename = "ingressTunAddrReq", default)]
			ingress_tun_addr_req: bool,
			#[serde(rename = "locationDependent", default)]
			location_dependent: bool,
			#[serde(
				rename = "mbsFsaIdList",
				default,
				skip_serializing_if = "Vec::is_empty"
			)]
			mbs_fsa_id_list: Vec<MbsFsaId>,
			#[serde(
				rename = "mbsServInfo",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			mbs_serv_info: Option<MbsServiceInfo>,
			#[serde(
				rename = "mbsServiceArea",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			mbs_service_area: Option<MbsServiceArea>,
			#[serde(
				rename = "mbsSessionSubsc",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			mbs_session_subsc: Option<MbsSessionSubscription>,
			#[serde(rename = "serviceType")]
			service_type: MbsServiceType,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			ssm: Option<Ssm>,
			#[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
			start_time: Option<DateTime>,
			#[serde(
				rename = "terminationTime",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			termination_time: Option<DateTime>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			tmgi: Option<Tmgi>,
			#[serde(rename = "tmgiAllocReq")]
			tmgi_alloc_req: bool,
		},
	}

	impl From<&MbsSession> for MbsSession {
		fn from(value: &MbsSession) -> Self {
			value.clone()
		}
	}

	/// Indicates the MBS session's activity status
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the MBS session's activity status",
	///  "type": "string",
	///  "enum": [
	///    "ACTIVE",
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
	pub enum MbsSessionActivityStatus {
		#[default]
		#[serde(rename = "ACTIVE")]
		Active,
		#[serde(rename = "INACTIVE")]
		Inactive,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MbsSessionActivityStatus> for MbsSessionActivityStatus {
		fn from(value: &MbsSessionActivityStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for MbsSessionActivityStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Active => "ACTIVE".to_string(),
				Self::Inactive => "INACTIVE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MbsSessionActivityStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ACTIVE" => Ok(Self::Active),
				"INACTIVE" => Ok(Self::Inactive),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MbsSessionActivityStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MbsSessionActivityStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MbsSessionActivityStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// MBS session event
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS session event",
	///  "type": "object",
	///  "required": [
	///    "eventType"
	///  ],
	///  "properties": {
	///    "eventType": {
	///      "$ref": "#/components/schemas/MbsSessionEventType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsSessionEvent {
		#[serde(rename = "eventType")]
		pub event_type: MbsSessionEventType,
	}

	impl From<&MbsSessionEvent> for MbsSessionEvent {
		fn from(value: &MbsSessionEvent) -> Self {
			value.clone()
		}
	}

	/// MBS session event report
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS session event report",
	///  "type": "object",
	///  "required": [
	///    "eventType"
	///  ],
	///  "properties": {
	///    "broadcastDelStatus": {
	///      "$ref": "#/components/schemas/BroadcastDeliveryStatus"
	///    },
	///    "eventType": {
	///      "$ref": "#/components/schemas/MbsSessionEventType"
	///    },
	///    "ingressTunAddrInfo": {
	///      "$ref": "#/components/schemas/IngressTunAddrInfo"
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
	pub struct MbsSessionEventReport {
		#[serde(
			rename = "broadcastDelStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub broadcast_del_status: Option<BroadcastDeliveryStatus>,
		#[serde(rename = "eventType")]
		pub event_type: MbsSessionEventType,
		#[serde(
			rename = "ingressTunAddrInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ingress_tun_addr_info: Option<IngressTunAddrInfo>,
		#[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
		pub time_stamp: Option<DateTime>,
	}

	impl From<&MbsSessionEventReport> for MbsSessionEventReport {
		fn from(value: &MbsSessionEventReport) -> Self {
			value.clone()
		}
	}

	/// MBS session event report list
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS session event report list",
	///  "type": "object",
	///  "required": [
	///    "eventReportList"
	///  ],
	///  "properties": {
	///    "eventReportList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MbsSessionEventReport"
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
	pub struct MbsSessionEventReportList {
		#[serde(rename = "eventReportList")]
		pub event_report_list: Vec<MbsSessionEventReport>,
		#[serde(
			rename = "notifyCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notify_correlation_id: Option<String>,
	}

	impl From<&MbsSessionEventReportList> for MbsSessionEventReportList {
		fn from(value: &MbsSessionEventReportList) -> Self {
			value.clone()
		}
	}

	/// MBS Session Event Type
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS Session Event Type",
	///  "type": "string",
	///  "enum": [
	///    "MBS_REL_TMGI_EXPIRY",
	///    "BROADCAST_DELIVERY_STATUS",
	///    "INGRESS_TUNNEL_ADD_CHANGE"
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
	pub enum MbsSessionEventType {
		#[default]
		#[serde(rename = "MBS_REL_TMGI_EXPIRY")]
		MbsRelTmgiExpiry,
		#[serde(rename = "BROADCAST_DELIVERY_STATUS")]
		BroadcastDeliveryStatus,
		#[serde(rename = "INGRESS_TUNNEL_ADD_CHANGE")]
		IngressTunnelAddChange,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MbsSessionEventType> for MbsSessionEventType {
		fn from(value: &MbsSessionEventType) -> Self {
			value.clone()
		}
	}

	impl ToString for MbsSessionEventType {
		fn to_string(&self) -> String {
			match *self {
				Self::MbsRelTmgiExpiry => "MBS_REL_TMGI_EXPIRY".to_string(),
				Self::BroadcastDeliveryStatus => "BROADCAST_DELIVERY_STATUS".to_string(),
				Self::IngressTunnelAddChange => "INGRESS_TUNNEL_ADD_CHANGE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MbsSessionEventType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MBS_REL_TMGI_EXPIRY" => Ok(Self::MbsRelTmgiExpiry),
				"BROADCAST_DELIVERY_STATUS" => Ok(Self::BroadcastDeliveryStatus),
				"INGRESS_TUNNEL_ADD_CHANGE" => Ok(Self::IngressTunnelAddChange),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MbsSessionEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MbsSessionEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MbsSessionEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// MBS Session Identifier
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS Session Identifier",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "tmgi"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ssm"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    },
	///    "ssm": {
	///      "$ref": "#/components/schemas/Ssm"
	///    },
	///    "tmgi": {
	///      "$ref": "#/components/schemas/Tmgi"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum MbsSessionId {
		#[default]
		Variant0 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			nid: Option<Nid>,
			tmgi: Tmgi,
		},
		Variant1 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			nid: Option<Nid>,
			ssm: Ssm,
		},
	}

	impl From<&MbsSessionId> for MbsSessionId {
		fn from(value: &MbsSessionId) -> Self {
			value.clone()
		}
	}

	/// MBS session subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS session subscription",
	///  "type": "object",
	///  "required": [
	///    "eventList",
	///    "notifyUri"
	///  ],
	///  "properties": {
	///    "areaSessionId": {
	///      "$ref": "#/components/schemas/AreaSessionId"
	///    },
	///    "eventList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MbsSessionEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "expiryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "mbsSessionId": {
	///      "$ref": "#/components/schemas/MbsSessionId"
	///    },
	///    "mbsSessionSubscUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nfcInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "notifyCorrelationId": {
	///      "type": "string"
	///    },
	///    "notifyUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsSessionSubscription {
		#[serde(
			rename = "areaSessionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub area_session_id: Option<AreaSessionId>,
		#[serde(rename = "eventList")]
		pub event_list: Vec<MbsSessionEvent>,
		#[serde(
			rename = "expiryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub expiry_time: Option<DateTime>,
		#[serde(
			rename = "mbsSessionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_session_id: Option<MbsSessionId>,
		#[serde(
			rename = "mbsSessionSubscUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbs_session_subsc_uri: Option<Uri>,
		#[serde(
			rename = "nfcInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nfc_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "notifyCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notify_correlation_id: Option<String>,
		#[serde(rename = "notifyUri")]
		pub notify_uri: Uri,
	}

	impl From<&MbsSessionSubscription> for MbsSessionSubscription {
		fn from(value: &MbsSessionSubscription) -> Self {
			value.clone()
		}
	}

	/// Contains an MBSFN area information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains an MBSFN area information.",
	///  "type": "object",
	///  "properties": {
	///    "carrierFrequency": {
	///      "description": "When present, this IE shall contain the Carrier
	/// Frequency (EARFCN).",
	///      "type": "integer",
	///      "maximum": 262143.0,
	///      "minimum": 0.0
	///    },
	///    "mbsfnAreaId": {
	///      "description": "This IE shall contain the MBSFN Area ID.",
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
	pub struct MbsfnArea {
		/// When present, this IE shall contain the Carrier Frequency (EARFCN).
		#[serde(
			rename = "carrierFrequency",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub carrier_frequency: Option<i64>,
		/// This IE shall contain the MBSFN Area ID.
		#[serde(
			rename = "mbsfnAreaId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mbsfn_area_id: Option<u8>,
	}

	impl From<&MbsfnArea> for MbsfnArea {
		fn from(value: &MbsfnArea) -> Self {
			value.clone()
		}
	}

	/// Mobile Country Code part of the PLMN, comprising 3 digits, as defined in
	/// clause 9.3.3.5 of  3GPP TS 38.413 with the OpenAPI 'nullable: true'
	/// property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Mobile Country Code part of the PLMN, comprising 3
	/// digits, as defined in clause 9.3.3.5 of  3GPP TS 38.413 with the OpenAPI
	/// 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^\\d{3}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MccRm(pub Option<Mcc>);

	impl ::std::ops::Deref for MccRm {
		type Target = Option<Mcc>;
		fn deref(&self) -> &Option<Mcc> {
			&self.0
		}
	}

	impl From<MccRm> for Option<Mcc> {
		fn from(value: MccRm) -> Self {
			value.0
		}
	}

	impl From<&MccRm> for MccRm {
		fn from(value: &MccRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<Mcc>> for MccRm {
		fn from(value: Option<Mcc>) -> Self {
			Self(value)
		}
	}

	/// contains contain MDT configuration data.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "contains contain MDT configuration data.",
	///  "type": "object",
	///  "required": [
	///    "jobType"
	///  ],
	///  "properties": {
	///    "addPositioningMethodList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PositioningMethodMdt"
	///      },
	///      "minItems": 1
	///    },
	///    "areaScope": {
	///      "$ref": "#/components/schemas/AreaScope"
	///    },
	///    "collectionPeriodRmmLte": {
	///      "$ref": "#/components/schemas/CollectionPeriodRmmLteMdt"
	///    },
	///    "collectionPeriodRmmNr": {
	///      "$ref": "#/components/schemas/CollectionPeriodRmmNrMdt"
	///    },
	///    "eventList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EventForMdt"
	///      },
	///      "minItems": 1
	///    },
	///    "eventThresholdRsrp": {
	///      "description": "This IE shall be present if the report trigger
	/// parameter is configured for A2 event reporting or A2 event triggered
	/// periodic reporting and the job type parameter is configured for
	/// Immediate MDT or combined Immediate MDT and Trace in LTE. When present,
	/// this IE shall indicate the Event Threshold for RSRP, and the value shall
	/// be between 0-97.\n",
	///      "type": "integer",
	///      "maximum": 97.0,
	///      "minimum": 0.0
	///    },
	///    "eventThresholdRsrpNr": {
	///      "description": "This IE shall be present if the report trigger
	/// parameter is configured for A2 event reporting or A2 event triggered
	/// periodic reporting and the job type parameter is configured for
	/// Immediate MDT or combined Immediate MDT and Trace in NR. When present,
	/// this IE shall indicate the Event Threshold for RSRP, and the value shall
	/// be between 0-127.\n",
	///      "type": "integer",
	///      "maximum": 127.0,
	///      "minimum": 0.0
	///    },
	///    "eventThresholdRsrq": {
	///      "description": "This IE shall be present if the report trigger
	/// parameter is configured for A2 event reporting or A2 event triggered
	/// periodic reporting and the job type parameter is configured for
	/// Immediate MDT or combined Immediate MDT and Trace in LTE.When present,
	/// this IE shall indicate the Event Threshold for RSRQ, and the value shall
	/// be between 0-34.\n",
	///      "type": "integer",
	///      "maximum": 34.0,
	///      "minimum": 0.0
	///    },
	///    "eventThresholdRsrqNr": {
	///      "description": "This IE shall be present if the report trigger
	/// parameter is configured for A2 event reporting or A2 event triggered
	/// periodic reporting and the job type parameter is configured for
	/// Immediate MDT or combined Immediate MDT and Trace in NR.When present,
	/// this IE shall indicate the Event Threshold for RSRQ, and the value shall
	/// be between 0-127.\n",
	///      "type": "integer",
	///      "maximum": 127.0,
	///      "minimum": 0.0
	///    },
	///    "interFreqTargetList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/InterFreqTargetInfo"
	///      },
	///      "maxItems": 8,
	///      "minItems": 1
	///    },
	///    "jobType": {
	///      "$ref": "#/components/schemas/JobType"
	///    },
	///    "loggingDuration": {
	///      "$ref": "#/components/schemas/LoggingDurationMdt"
	///    },
	///    "loggingDurationNr": {
	///      "$ref": "#/components/schemas/LoggingDurationNrMdt"
	///    },
	///    "loggingInterval": {
	///      "$ref": "#/components/schemas/LoggingIntervalMdt"
	///    },
	///    "loggingIntervalNr": {
	///      "$ref": "#/components/schemas/LoggingIntervalNrMdt"
	///    },
	///    "mbsfnAreaList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MbsfnArea"
	///      },
	///      "maxItems": 8,
	///      "minItems": 1
	///    },
	///    "mdtAllowedPlmnIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
	///      },
	///      "maxItems": 16,
	///      "minItems": 1
	///    },
	///    "measurementLteList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MeasurementLteForMdt"
	///      }
	///    },
	///    "measurementNrList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MeasurementNrForMdt"
	///      },
	///      "minItems": 1
	///    },
	///    "measurementPeriodLte": {
	///      "$ref": "#/components/schemas/MeasurementPeriodLteMdt"
	///    },
	///    "positioningMethod": {
	///      "$ref": "#/components/schemas/PositioningMethodMdt"
	///    },
	///    "reportAmount": {
	///      "$ref": "#/components/schemas/ReportAmountMdt"
	///    },
	///    "reportInterval": {
	///      "$ref": "#/components/schemas/ReportIntervalMdt"
	///    },
	///    "reportIntervalNr": {
	///      "$ref": "#/components/schemas/ReportIntervalNrMdt"
	///    },
	///    "reportType": {
	///      "$ref": "#/components/schemas/ReportTypeMdt"
	///    },
	///    "reportingTriggerList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ReportingTrigger"
	///      },
	///      "minItems": 1
	///    },
	///    "sensorMeasurementList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SensorMeasurement"
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
	pub struct MdtConfiguration {
		#[serde(
			rename = "addPositioningMethodList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub add_positioning_method_list: Vec<PositioningMethodMdt>,
		#[serde(rename = "areaScope", default, skip_serializing_if = "Option::is_none")]
		pub area_scope: Option<AreaScope>,
		#[serde(
			rename = "collectionPeriodRmmLte",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub collection_period_rmm_lte: Option<CollectionPeriodRmmLteMdt>,
		#[serde(
			rename = "collectionPeriodRmmNr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub collection_period_rmm_nr: Option<CollectionPeriodRmmNrMdt>,
		#[serde(rename = "eventList", default, skip_serializing_if = "Vec::is_empty")]
		pub event_list: Vec<EventForMdt>,
		/// This IE shall be present if the report trigger parameter is
		/// configured for A2 event reporting or A2 event triggered periodic
		/// reporting and the job type parameter is configured for Immediate MDT
		/// or combined Immediate MDT and Trace in LTE. When present, this IE
		/// shall indicate the Event Threshold for RSRP, and the value shall be
		/// between 0-97.
		#[serde(
			rename = "eventThresholdRsrp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_threshold_rsrp: Option<i64>,
		/// This IE shall be present if the report trigger parameter is
		/// configured for A2 event reporting or A2 event triggered periodic
		/// reporting and the job type parameter is configured for Immediate MDT
		/// or combined Immediate MDT and Trace in NR. When present, this IE
		/// shall indicate the Event Threshold for RSRP, and the value shall be
		/// between 0-127.
		#[serde(
			rename = "eventThresholdRsrpNr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_threshold_rsrp_nr: Option<i64>,
		/// This IE shall be present if the report trigger parameter is
		/// configured for A2 event reporting or A2 event triggered periodic
		/// reporting and the job type parameter is configured for Immediate MDT
		/// or combined Immediate MDT and Trace in LTE.When present, this IE
		/// shall indicate the Event Threshold for RSRQ, and the value shall be
		/// between 0-34.
		#[serde(
			rename = "eventThresholdRsrq",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_threshold_rsrq: Option<i64>,
		/// This IE shall be present if the report trigger parameter is
		/// configured for A2 event reporting or A2 event triggered periodic
		/// reporting and the job type parameter is configured for Immediate MDT
		/// or combined Immediate MDT and Trace in NR.When present, this IE
		/// shall indicate the Event Threshold for RSRQ, and the value shall be
		/// between 0-127.
		#[serde(
			rename = "eventThresholdRsrqNr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_threshold_rsrq_nr: Option<i64>,
		#[serde(
			rename = "interFreqTargetList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub inter_freq_target_list: Vec<InterFreqTargetInfo>,
		#[serde(rename = "jobType")]
		pub job_type: JobType,
		#[serde(
			rename = "loggingDuration",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub logging_duration: Option<LoggingDurationMdt>,
		#[serde(
			rename = "loggingDurationNr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub logging_duration_nr: Option<LoggingDurationNrMdt>,
		#[serde(
			rename = "loggingInterval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub logging_interval: Option<LoggingIntervalMdt>,
		#[serde(
			rename = "loggingIntervalNr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub logging_interval_nr: Option<LoggingIntervalNrMdt>,
		#[serde(
			rename = "mbsfnAreaList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mbsfn_area_list: Vec<MbsfnArea>,
		#[serde(
			rename = "mdtAllowedPlmnIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mdt_allowed_plmn_id_list: Vec<PlmnId>,
		#[serde(
			rename = "measurementLteList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub measurement_lte_list: Vec<MeasurementLteForMdt>,
		#[serde(
			rename = "measurementNrList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub measurement_nr_list: Vec<MeasurementNrForMdt>,
		#[serde(
			rename = "measurementPeriodLte",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub measurement_period_lte: Option<MeasurementPeriodLteMdt>,
		#[serde(
			rename = "positioningMethod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub positioning_method: Option<PositioningMethodMdt>,
		#[serde(
			rename = "reportAmount",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub report_amount: Option<ReportAmountMdt>,
		#[serde(
			rename = "reportInterval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub report_interval: Option<ReportIntervalMdt>,
		#[serde(
			rename = "reportIntervalNr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub report_interval_nr: Option<ReportIntervalNrMdt>,
		#[serde(
			rename = "reportType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub report_type: Option<ReportTypeMdt>,
		#[serde(
			rename = "reportingTriggerList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub reporting_trigger_list: Vec<ReportingTrigger>,
		#[serde(
			rename = "sensorMeasurementList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub sensor_measurement_list: Vec<SensorMeasurement>,
	}

	impl From<&MdtConfiguration> for MdtConfiguration {
		fn from(value: &MdtConfiguration) -> Self {
			value.clone()
		}
	}

	/// The enumeration MeasurementLteForMdt defines Measurements used for MDT
	/// in LTE in the trace. See 3GPP TS 32.422 for further description of the
	/// values. It shall comply with the provisions defined in table 5.6.3.5-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration MeasurementLteForMdt defines
	/// Measurements used for MDT in LTE in the trace. See 3GPP TS 32.422 for
	/// further description of the values. It shall comply with the provisions
	/// defined in table 5.6.3.5-1.\n",
	///  "type": "string",
	///  "enum": [
	///    "M1",
	///    "M2",
	///    "M3",
	///    "M4_DL",
	///    "M4_UL",
	///    "M5_DL",
	///    "M5_UL",
	///    "M6_DL",
	///    "M6_UL",
	///    "M7_DL",
	///    "M7_UL",
	///    "M8",
	///    "M9"
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
	pub enum MeasurementLteForMdt {
		#[default]
		M1,
		M2,
		M3,
		#[serde(rename = "M4_DL")]
		M4Dl,
		#[serde(rename = "M4_UL")]
		M4Ul,
		#[serde(rename = "M5_DL")]
		M5Dl,
		#[serde(rename = "M5_UL")]
		M5Ul,
		#[serde(rename = "M6_DL")]
		M6Dl,
		#[serde(rename = "M6_UL")]
		M6Ul,
		#[serde(rename = "M7_DL")]
		M7Dl,
		#[serde(rename = "M7_UL")]
		M7Ul,
		M8,
		M9,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MeasurementLteForMdt> for MeasurementLteForMdt {
		fn from(value: &MeasurementLteForMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for MeasurementLteForMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::M1 => "M1".to_string(),
				Self::M2 => "M2".to_string(),
				Self::M3 => "M3".to_string(),
				Self::M4Dl => "M4_DL".to_string(),
				Self::M4Ul => "M4_UL".to_string(),
				Self::M5Dl => "M5_DL".to_string(),
				Self::M5Ul => "M5_UL".to_string(),
				Self::M6Dl => "M6_DL".to_string(),
				Self::M6Ul => "M6_UL".to_string(),
				Self::M7Dl => "M7_DL".to_string(),
				Self::M7Ul => "M7_UL".to_string(),
				Self::M8 => "M8".to_string(),
				Self::M9 => "M9".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MeasurementLteForMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"M1" => Ok(Self::M1),
				"M2" => Ok(Self::M2),
				"M3" => Ok(Self::M3),
				"M4_DL" => Ok(Self::M4Dl),
				"M4_UL" => Ok(Self::M4Ul),
				"M5_DL" => Ok(Self::M5Dl),
				"M5_UL" => Ok(Self::M5Ul),
				"M6_DL" => Ok(Self::M6Dl),
				"M6_UL" => Ok(Self::M6Ul),
				"M7_DL" => Ok(Self::M7Dl),
				"M7_UL" => Ok(Self::M7Ul),
				"M8" => Ok(Self::M8),
				"M9" => Ok(Self::M9),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MeasurementLteForMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MeasurementLteForMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MeasurementLteForMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration MeasurementNrForMdt defines Measurements used for MDT in
	/// NR in the trace. See 3GPP TS 32.422 for further description of the
	/// values. It shall comply with the provisions defined in table 5.6.3.6-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration MeasurementNrForMdt defines
	/// Measurements used for MDT in NR in the trace. See 3GPP TS 32.422 for
	/// further description of the values. It shall comply with the provisions
	/// defined in table 5.6.3.6-1.\n",
	///  "type": "string",
	///  "enum": [
	///    "M1",
	///    "M2",
	///    "M3",
	///    "M4_DL",
	///    "M4_UL",
	///    "M5_DL",
	///    "M5_UL",
	///    "M6_DL",
	///    "M6_UL",
	///    "M7_DL",
	///    "M7_UL",
	///    "M8",
	///    "M9"
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
	pub enum MeasurementNrForMdt {
		#[default]
		M1,
		M2,
		M3,
		#[serde(rename = "M4_DL")]
		M4Dl,
		#[serde(rename = "M4_UL")]
		M4Ul,
		#[serde(rename = "M5_DL")]
		M5Dl,
		#[serde(rename = "M5_UL")]
		M5Ul,
		#[serde(rename = "M6_DL")]
		M6Dl,
		#[serde(rename = "M6_UL")]
		M6Ul,
		#[serde(rename = "M7_DL")]
		M7Dl,
		#[serde(rename = "M7_UL")]
		M7Ul,
		M8,
		M9,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MeasurementNrForMdt> for MeasurementNrForMdt {
		fn from(value: &MeasurementNrForMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for MeasurementNrForMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::M1 => "M1".to_string(),
				Self::M2 => "M2".to_string(),
				Self::M3 => "M3".to_string(),
				Self::M4Dl => "M4_DL".to_string(),
				Self::M4Ul => "M4_UL".to_string(),
				Self::M5Dl => "M5_DL".to_string(),
				Self::M5Ul => "M5_UL".to_string(),
				Self::M6Dl => "M6_DL".to_string(),
				Self::M6Ul => "M6_UL".to_string(),
				Self::M7Dl => "M7_DL".to_string(),
				Self::M7Ul => "M7_UL".to_string(),
				Self::M8 => "M8".to_string(),
				Self::M9 => "M9".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MeasurementNrForMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"M1" => Ok(Self::M1),
				"M2" => Ok(Self::M2),
				"M3" => Ok(Self::M3),
				"M4_DL" => Ok(Self::M4Dl),
				"M4_UL" => Ok(Self::M4Ul),
				"M5_DL" => Ok(Self::M5Dl),
				"M5_UL" => Ok(Self::M5Ul),
				"M6_DL" => Ok(Self::M6Dl),
				"M6_UL" => Ok(Self::M6Ul),
				"M7_DL" => Ok(Self::M7Dl),
				"M7_UL" => Ok(Self::M7Ul),
				"M8" => Ok(Self::M8),
				"M9" => Ok(Self::M9),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MeasurementNrForMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MeasurementNrForMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MeasurementNrForMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration MeasurementPeriodLteMdt defines Measurement period LTE
	/// for MDT in the trace.  See 3GPP TS 32.422 for further description of the
	/// values. It shall comply with the provisions defined in table 5.6.3.16-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration MeasurementPeriodLteMdt defines
	/// Measurement period LTE for MDT in the trace.  See 3GPP TS 32.422 for
	/// further description of the values. It shall comply with the provisions
	/// defined in table 5.6.3.16-1.\n",
	///  "type": "string",
	///  "enum": [
	///    1024,
	///    1280,
	///    2048,
	///    2560,
	///    5120,
	///    10240,
	///    60000
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
	pub enum MeasurementPeriodLteMdt {
		#[default]
		#[serde(rename = "1024")]
		NUM1024,
		#[serde(rename = "1280")]
		NUM1280,
		#[serde(rename = "2048")]
		NUM2048,
		#[serde(rename = "2560")]
		NUM2560,
		#[serde(rename = "5120")]
		NUM5120,
		#[serde(rename = "10240")]
		NUM10240,
		#[serde(rename = "60000")]
		NUM60000,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MeasurementPeriodLteMdt> for MeasurementPeriodLteMdt {
		fn from(value: &MeasurementPeriodLteMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for MeasurementPeriodLteMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::NUM1024 => "1024".to_string(),
				Self::NUM1280 => "1280".to_string(),
				Self::NUM2048 => "2048".to_string(),
				Self::NUM2560 => "2560".to_string(),
				Self::NUM5120 => "5120".to_string(),
				Self::NUM10240 => "10240".to_string(),
				Self::NUM60000 => "60000".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MeasurementPeriodLteMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"1024" => Ok(Self::NUM1024),
				"1280" => Ok(Self::NUM1280),
				"2048" => Ok(Self::NUM2048),
				"2560" => Ok(Self::NUM2560),
				"5120" => Ok(Self::NUM5120),
				"10240" => Ok(Self::NUM10240),
				"60000" => Ok(Self::NUM60000),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MeasurementPeriodLteMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MeasurementPeriodLteMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MeasurementPeriodLteMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the media type of a media component.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the media type of a media component.",
	///  "type": "string",
	///  "enum": [
	///    "AUDIO",
	///    "VIDEO",
	///    "DATA",
	///    "APPLICATION",
	///    "CONTROL",
	///    "TEXT",
	///    "MESSAGE",
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
	pub enum MediaType {
		#[default]
		#[serde(rename = "AUDIO")]
		Audio,
		#[serde(rename = "VIDEO")]
		Video,
		#[serde(rename = "DATA")]
		Data,
		#[serde(rename = "APPLICATION")]
		Application,
		#[serde(rename = "CONTROL")]
		Control,
		#[serde(rename = "TEXT")]
		Text,
		#[serde(rename = "MESSAGE")]
		Message,
		#[serde(rename = "OTHER")]
		Other,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MediaType> for MediaType {
		fn from(value: &MediaType) -> Self {
			value.clone()
		}
	}

	impl ToString for MediaType {
		fn to_string(&self) -> String {
			match *self {
				Self::Audio => "AUDIO".to_string(),
				Self::Video => "VIDEO".to_string(),
				Self::Data => "DATA".to_string(),
				Self::Application => "APPLICATION".to_string(),
				Self::Control => "CONTROL".to_string(),
				Self::Text => "TEXT".to_string(),
				Self::Message => "MESSAGE".to_string(),
				Self::Other => "OTHER".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MediaType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AUDIO" => Ok(Self::Audio),
				"VIDEO" => Ok(Self::Video),
				"DATA" => Ok(Self::Data),
				"APPLICATION" => Ok(Self::Application),
				"CONTROL" => Ok(Self::Control),
				"TEXT" => Ok(Self::Text),
				"MESSAGE" => Ok(Self::Message),
				"OTHER" => Ok(Self::Other),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MediaType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MediaType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MediaType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Mobile Network Code part of the PLMN, comprising 2 or 3 digits, as
	/// defined in clause 9.3.3.5 of 3GPP TS 38.413 with the OpenAPI 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Mobile Network Code part of the PLMN, comprising 2 or 3
	/// digits, as defined in clause 9.3.3.5 of 3GPP TS 38.413 with the OpenAPI
	/// 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^\\d{2,3}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MncRm(pub Option<Mnc>);

	impl ::std::ops::Deref for MncRm {
		type Target = Option<Mnc>;
		fn deref(&self) -> &Option<Mnc> {
			&self.0
		}
	}

	impl From<MncRm> for Option<Mnc> {
		fn from(value: MncRm) -> Self {
			value.0
		}
	}

	impl From<&MncRm> for MncRm {
		fn from(value: &MncRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<Mnc>> for MncRm {
		fn from(value: Option<Mnc>) -> Self {
			Self(value)
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

	/// String uniquely identifying MTC provider information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String uniquely identifying MTC provider information.",
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
	pub struct MtcProviderInformation(pub String);

	impl ::std::ops::Deref for MtcProviderInformation {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<MtcProviderInformation> for String {
		fn from(value: MtcProviderInformation) -> Self {
			value.0
		}
	}

	impl From<&MtcProviderInformation> for MtcProviderInformation {
		fn from(value: &MtcProviderInformation) -> Self {
			value.clone()
		}
	}

	impl From<String> for MtcProviderInformation {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for MtcProviderInformation {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for MtcProviderInformation {
		fn to_string(&self) -> String {
			self.0.to_string()
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
		NewUnchecked,
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
	///      "$ref": "#/components/schemas/Gli"
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
		pub gli: Option<Gli>,
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
		NewUnchecked,
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

	/// This data type is defined in the same way as the 'Ncgi' data type, but
	/// with the  OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Ncgi'
	/// data type, but with the  OpenAPI 'nullable: true' property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Ncgi"
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
	pub enum NcgiRm {
		#[default]
		Ncgi(Ncgi),
		NullValue(NullValue),
	}

	impl From<&NcgiRm> for NcgiRm {
		fn from(value: &NcgiRm) -> Self {
			value.clone()
		}
	}

	impl From<Ncgi> for NcgiRm {
		fn from(value: Ncgi) -> Self {
			Self::Ncgi(value)
		}
	}

	impl From<NullValue> for NcgiRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// List of NR cell ids, with their pertaining TAIs
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of NR cell ids, with their pertaining TAIs",
	///  "type": "object",
	///  "required": [
	///    "cellList",
	///    "tai"
	///  ],
	///  "properties": {
	///    "cellList": {
	///      "description": "List of List of NR cell ids",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ncgi"
	///      },
	///      "minItems": 1
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
	pub struct NcgiTai {
		/// List of List of NR cell ids
		#[serde(rename = "cellList")]
		pub cell_list: Vec<Ncgi>,
		pub tai: Tai,
	}

	impl From<&NcgiTai> for NcgiTai {
		fn from(value: &NcgiTai) -> Self {
			value.clone()
		}
	}

	/// contains PLMN and Network identity.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "contains PLMN and Network identity.",
	///  "type": "object",
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
	pub struct NetworkId {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub mcc: Option<Mcc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub mnc: Option<Mnc>,
	}

	impl From<&NetworkId> for NetworkId {
		fn from(value: &NetworkId) -> Self {
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
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		smart_default::SmartDefault,
		Copy,
		PartialEq,
		Eq,
		Hash,
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
		NewUnchecked,
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
		NewUnchecked,
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

	/// This data type is defined in the same way as the 'Nid' data type, but
	/// with the OpenAPI 'nullable: true' property."
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Nid'
	/// data type, but with the OpenAPI 'nullable: true' property.\"\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^[A-Fa-f0-9]{11}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NidRm(pub Option<NidRmInner>);

	impl ::std::ops::Deref for NidRm {
		type Target = Option<NidRmInner>;
		fn deref(&self) -> &Option<NidRmInner> {
			&self.0
		}
	}

	impl From<NidRm> for Option<NidRmInner> {
		fn from(value: NidRm) -> Self {
			value.0
		}
	}

	impl From<&NidRm> for NidRm {
		fn from(value: &NidRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<NidRmInner>> for NidRm {
		fn from(value: Option<NidRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the 'Nid' data type, but
	/// with the OpenAPI 'nullable: true' property."
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Nid'
	/// data type, but with the OpenAPI 'nullable: true' property.\"\n",
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
		NewUnchecked,
	)]
	pub struct NidRmInner(String);

	impl ::std::ops::Deref for NidRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NidRmInner> for String {
		fn from(value: NidRmInner) -> Self {
			value.0
		}
	}

	impl From<&NidRmInner> for NidRmInner {
		fn from(value: &NidRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for NidRmInner {
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

	impl ::std::convert::TryFrom<&str> for NidRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for NidRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for NidRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for NidRmInner {
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

	/// This enumeration is defined in the same way as the 'NotificationControl'
	/// enumeration, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This enumeration is defined in the same way as the
	/// 'NotificationControl' enumeration, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/NotificationControl"
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
	pub enum NotificationControlRm {
		#[default]
		NotificationControl(NotificationControl),
		NullValue(NullValue),
	}

	impl From<&NotificationControlRm> for NotificationControlRm {
		fn from(value: &NotificationControlRm) -> Self {
			value.clone()
		}
	}

	impl From<NotificationControl> for NotificationControlRm {
		fn from(value: NotificationControl) -> Self {
			Self::NotificationControl(value)
		}
	}

	impl From<NullValue> for NotificationControlRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// Indicates changes on a resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates changes on a resource.",
	///  "type": "object",
	///  "required": [
	///    "changes",
	///    "resourceId"
	///  ],
	///  "properties": {
	///    "changes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ChangeItem"
	///      },
	///      "minItems": 1
	///    },
	///    "resourceId": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NotifyItem {
		pub changes: Vec<ChangeItem>,
		#[serde(rename = "resourceId")]
		pub resource_id: Uri,
	}

	impl From<&NotifyItem> for NotifyItem {
		fn from(value: &NotifyItem) -> Self {
			value.clone()
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
		NewUnchecked,
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

	/// This data type is defined in the same way as the 'NrCellId' data type,
	/// but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'NrCellId' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^[A-Fa-f0-9]{9}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrCellIdRm(pub Option<NrCellIdRmInner>);

	impl ::std::ops::Deref for NrCellIdRm {
		type Target = Option<NrCellIdRmInner>;
		fn deref(&self) -> &Option<NrCellIdRmInner> {
			&self.0
		}
	}

	impl From<NrCellIdRm> for Option<NrCellIdRmInner> {
		fn from(value: NrCellIdRm) -> Self {
			value.0
		}
	}

	impl From<&NrCellIdRm> for NrCellIdRm {
		fn from(value: &NrCellIdRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<NrCellIdRmInner>> for NrCellIdRm {
		fn from(value: Option<NrCellIdRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the 'NrCellId' data type,
	/// but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'NrCellId' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
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
		NewUnchecked,
	)]
	pub struct NrCellIdRmInner(String);

	impl ::std::ops::Deref for NrCellIdRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NrCellIdRmInner> for String {
		fn from(value: NrCellIdRmInner) -> Self {
			value.0
		}
	}

	impl From<&NrCellIdRmInner> for NrCellIdRmInner {
		fn from(value: &NrCellIdRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for NrCellIdRmInner {
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

	impl ::std::convert::TryFrom<&str> for NrCellIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for NrCellIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for NrCellIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for NrCellIdRmInner {
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
		NewUnchecked,
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
		NewUnchecked,
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

	/// This data type is defined in the same way as the 'NrLocation' data type,
	/// but with the OpenAPI 'nullable: true' property."
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'NrLocation' data type, but with the OpenAPI 'nullable: true'
	/// property.\" \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/NrLocation"
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
	pub enum NrLocationRm {
		#[default]
		NrLocation(NrLocation),
		NullValue(NullValue),
	}

	impl From<&NrLocationRm> for NrLocationRm {
		fn from(value: &NrLocationRm) -> Self {
			value.clone()
		}
	}

	impl From<NrLocation> for NrLocationRm {
		fn from(value: NrLocation) -> Self {
			Self::NrLocation(value)
		}
	}

	impl From<NullValue> for NrLocationRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Contains NR V2X services authorized information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains NR V2X services authorized information.",
	///  "type": "object",
	///  "properties": {
	///    "pedestrianUeAuth": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "vehicleUeAuth": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrV2xAuth {
		#[serde(
			rename = "pedestrianUeAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pedestrian_ue_auth: Option<UeAuth>,
		#[serde(
			rename = "vehicleUeAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vehicle_ue_auth: Option<UeAuth>,
	}

	impl From<&NrV2xAuth> for NrV2xAuth {
		fn from(value: &NrV2xAuth) -> Self {
			value.clone()
		}
	}

	/// String providing a Network Slice Simultaneous Registration Group. See
	/// clause 5.15.12 of  3GPP TS 23.501
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String providing a Network Slice Simultaneous
	/// Registration Group. See clause 5.15.12 of  3GPP TS 23.501\n",
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
	pub struct NsSrg(pub String);

	impl ::std::ops::Deref for NsSrg {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NsSrg> for String {
		fn from(value: NsSrg) -> Self {
			value.0
		}
	}

	impl From<&NsSrg> for NsSrg {
		fn from(value: &NsSrg) -> Self {
			value.clone()
		}
	}

	impl From<String> for NsSrg {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NsSrg {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for NsSrg {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// String providing a Network Slice Simultaneous Registration Group with
	/// the OpenAPI "nullable: true" property. See clause 5.15.12 of 3GPP TS
	/// 23.501
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String providing a Network Slice Simultaneous
	/// Registration Group with the OpenAPI \"nullable: true\" property. See
	/// clause 5.15.12 of 3GPP TS 23.501\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NsSrgRm(pub Option<String>);

	impl ::std::ops::Deref for NsSrgRm {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<NsSrgRm> for Option<String> {
		fn from(value: NsSrgRm) -> Self {
			value.0
		}
	}

	impl From<&NsSrgRm> for NsSrgRm {
		fn from(value: &NsSrgRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for NsSrgRm {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// String identifying the Network Slice Admission Control Service Area
	/// Identifier.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying the Network Slice Admission Control
	/// Service Area Identifier.\n",
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
	pub struct NsacSai(pub String);

	impl ::std::ops::Deref for NsacSai {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NsacSai> for String {
		fn from(value: NsacSai) -> Self {
			value.0
		}
	}

	impl From<&NsacSai> for NsacSai {
		fn from(value: &NsacSai) -> Self {
			value.clone()
		}
	}

	impl From<String> for NsacSai {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NsacSai {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for NsacSai {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// The Network Slice AS Group ID, see 3GPP TS 38.413
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The Network Slice AS Group ID, see 3GPP TS 38.413\n",
	///  "type": "integer"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NsagId(pub i64);

	impl ::std::ops::Deref for NsagId {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<NsagId> for i64 {
		fn from(value: NsagId) -> Self {
			value.0
		}
	}

	impl From<&NsagId> for NsagId {
		fn from(value: &NsagId) -> Self {
			value.clone()
		}
	}

	impl From<i64> for NsagId {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NsagId {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for NsagId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NsagId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NsagId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for NsagId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// This data type is defined in the same way as the "NsagId" data type, but
	/// with the OpenAPI "nullable: true" property
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// \"NsagId\" data type, but with the OpenAPI \"nullable: true\"
	/// property\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NsagIdRm(pub Option<i64>);

	impl ::std::ops::Deref for NsagIdRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<NsagIdRm> for Option<i64> {
		fn from(value: NsagIdRm) -> Self {
			value.0
		}
	}

	impl From<&NsagIdRm> for NsagIdRm {
		fn from(value: &NsagIdRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for NsagIdRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
		}
	}

	/// contains the Subscribed S-NSSAI subject to NSSAA procedure and the
	/// status.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "contains the Subscribed S-NSSAI subject to NSSAA
	/// procedure and the status.",
	///  "type": "object",
	///  "required": [
	///    "snssai",
	///    "status"
	///  ],
	///  "properties": {
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "status": {
	///      "$ref": "#/components/schemas/AuthStatus"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NssaaStatus {
		pub snssai: Snssai,
		pub status: AuthStatus,
	}

	impl From<&NssaaStatus> for NssaaStatus {
		fn from(value: &NssaaStatus) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the 'NssaaStatus' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'NssaaStatus' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/NssaaStatus"
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
	pub enum NssaaStatusRm {
		#[default]
		NssaaStatus(NssaaStatus),
		NullValue(NullValue),
	}

	impl From<&NssaaStatusRm> for NssaaStatusRm {
		fn from(value: &NssaaStatusRm) -> Self {
			value.clone()
		}
	}

	impl From<NssaaStatus> for NssaaStatusRm {
		fn from(value: NssaaStatus) -> Self {
			Self::NssaaStatus(value)
		}
	}

	impl From<NullValue> for NssaaStatusRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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
		::serde::Deserialize,
		::serde::Serialize,
		Clone,
		Debug,
		smart_default::SmartDefault,
		NewUnchecked,
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

	/// Contains information regarding operater  determined  barring.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains information regarding operater  determined
	/// barring.",
	///  "type": "object",
	///  "properties": {
	///    "roamingOdb": {
	///      "$ref": "#/components/schemas/RoamingOdb"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct OdbData {
		#[serde(
			rename = "roamingOdb",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub roaming_odb: Option<RoamingOdb>,
	}

	impl From<&OdbData> for OdbData {
		fn from(value: &OdbData) -> Self {
			value.clone()
		}
	}

	/// The enumeration OdbPacketServices defines the Barring of Packet Oriented
	/// Services. See 3GPP TS 23.015 for further description. It shall comply
	/// with the provisions defined in table 5.7.3.2-1
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration OdbPacketServices defines the Barring
	/// of Packet Oriented Services. See 3GPP TS 23.015 for further description.
	/// It shall comply with the provisions defined in table 5.7.3.2-1\n",
	///  "anyOf": [
	///    {
	///      "type": "string",
	///      "enum": [
	///        "ALL_PACKET_SERVICES",
	///        "ROAMER_ACCESS_HPLMN_AP",
	///        "ROAMER_ACCESS_VPLMN_AP"
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
	pub enum OdbPacketServices {
		#[default]
		Variant0(OdbPacketServicesVariant0),
		Variant1(NullValue),
	}

	impl From<&OdbPacketServices> for OdbPacketServices {
		fn from(value: &OdbPacketServices) -> Self {
			value.clone()
		}
	}

	impl From<OdbPacketServicesVariant0> for OdbPacketServices {
		fn from(value: OdbPacketServicesVariant0) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<NullValue> for OdbPacketServices {
		fn from(value: NullValue) -> Self {
			Self::Variant1(value)
		}
	}

	/// OdbPacketServicesVariant0
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "ALL_PACKET_SERVICES",
	///    "ROAMER_ACCESS_HPLMN_AP",
	///    "ROAMER_ACCESS_VPLMN_AP"
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
	pub enum OdbPacketServicesVariant0 {
		#[default]
		#[serde(rename = "ALL_PACKET_SERVICES")]
		AllPacketServices,
		#[serde(rename = "ROAMER_ACCESS_HPLMN_AP")]
		RoamerAccessHplmnAp,
		#[serde(rename = "ROAMER_ACCESS_VPLMN_AP")]
		RoamerAccessVplmnAp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&OdbPacketServicesVariant0> for OdbPacketServicesVariant0 {
		fn from(value: &OdbPacketServicesVariant0) -> Self {
			value.clone()
		}
	}

	impl ToString for OdbPacketServicesVariant0 {
		fn to_string(&self) -> String {
			match *self {
				Self::AllPacketServices => "ALL_PACKET_SERVICES".to_string(),
				Self::RoamerAccessHplmnAp => "ROAMER_ACCESS_HPLMN_AP".to_string(),
				Self::RoamerAccessVplmnAp => "ROAMER_ACCESS_VPLMN_AP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for OdbPacketServicesVariant0 {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ALL_PACKET_SERVICES" => Ok(Self::AllPacketServices),
				"ROAMER_ACCESS_HPLMN_AP" => Ok(Self::RoamerAccessHplmnAp),
				"ROAMER_ACCESS_VPLMN_AP" => Ok(Self::RoamerAccessVplmnAp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for OdbPacketServicesVariant0 {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for OdbPacketServicesVariant0 {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for OdbPacketServicesVariant0 {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates value of orientation angle.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates value of orientation angle.",
	///  "type": "integer",
	///  "maximum": 180.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Orientation(pub i64);

	impl ::std::ops::Deref for Orientation {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<Orientation> for i64 {
		fn from(value: Orientation) -> Self {
			value.0
		}
	}

	impl From<&Orientation> for Orientation {
		fn from(value: &Orientation) -> Self {
			value.clone()
		}
	}

	impl From<i64> for Orientation {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Orientation {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Orientation {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Orientation {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Orientation {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Orientation {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// This data type is defined in the same way as the 'PacketDelBudget' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'PacketDelBudget' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PacketDelBudgetRm(pub Option<std::num::NonZeroU64>);

	impl ::std::ops::Deref for PacketDelBudgetRm {
		type Target = Option<std::num::NonZeroU64>;
		fn deref(&self) -> &Option<std::num::NonZeroU64> {
			&self.0
		}
	}

	impl From<PacketDelBudgetRm> for Option<std::num::NonZeroU64> {
		fn from(value: PacketDelBudgetRm) -> Self {
			value.0
		}
	}

	impl From<&PacketDelBudgetRm> for PacketDelBudgetRm {
		fn from(value: &PacketDelBudgetRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<std::num::NonZeroU64>> for PacketDelBudgetRm {
		fn from(value: Option<std::num::NonZeroU64>) -> Self {
			Self(value)
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
		NewUnchecked,
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

	/// This data type is defined in the same way as the 'PacketErrRate' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'PacketErrRate' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^([0-9]E-[0-9])$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PacketErrRateRm(pub Option<PacketErrRateRmInner>);

	impl ::std::ops::Deref for PacketErrRateRm {
		type Target = Option<PacketErrRateRmInner>;
		fn deref(&self) -> &Option<PacketErrRateRmInner> {
			&self.0
		}
	}

	impl From<PacketErrRateRm> for Option<PacketErrRateRmInner> {
		fn from(value: PacketErrRateRm) -> Self {
			value.0
		}
	}

	impl From<&PacketErrRateRm> for PacketErrRateRm {
		fn from(value: &PacketErrRateRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<PacketErrRateRmInner>> for PacketErrRateRm {
		fn from(value: Option<PacketErrRateRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the 'PacketErrRate' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'PacketErrRate' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
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
		NewUnchecked,
	)]
	pub struct PacketErrRateRmInner(String);

	impl ::std::ops::Deref for PacketErrRateRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PacketErrRateRmInner> for String {
		fn from(value: PacketErrRateRmInner) -> Self {
			value.0
		}
	}

	impl From<&PacketErrRateRmInner> for PacketErrRateRmInner {
		fn from(value: &PacketErrRateRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PacketErrRateRmInner {
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

	impl ::std::convert::TryFrom<&str> for PacketErrRateRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PacketErrRateRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PacketErrRateRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PacketErrRateRmInner {
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

	/// This data type is defined in the same way as the 'PacketLossRate' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'PacketLossRate' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 1000.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PacketLossRateRm(pub Option<i64>);

	impl ::std::ops::Deref for PacketLossRateRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<PacketLossRateRm> for Option<i64> {
		fn from(value: PacketLossRateRm) -> Self {
			value.0
		}
	}

	impl From<&PacketLossRateRm> for PacketLossRateRm {
		fn from(value: &PacketLossRateRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for PacketLossRateRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
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

	/// This data type is defined in the same way as the ' PartitioningCriteria
	/// ' data type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the '
	/// PartitioningCriteria ' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/PartitioningCriteria"
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
	pub enum PartitioningCriteriaRm {
		#[default]
		PartitioningCriteria(PartitioningCriteria),
		NullValue(NullValue),
	}

	impl From<&PartitioningCriteriaRm> for PartitioningCriteriaRm {
		fn from(value: &PartitioningCriteriaRm) -> Self {
			value.clone()
		}
	}

	impl From<PartitioningCriteria> for PartitioningCriteriaRm {
		fn from(value: PartitioningCriteria) -> Self {
			Self::PartitioningCriteria(value)
		}
	}

	impl From<NullValue> for PartitioningCriteriaRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// it contains information on data to be changed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "it contains information on data to be changed.",
	///  "type": "object",
	///  "required": [
	///    "op",
	///    "path"
	///  ],
	///  "properties": {
	///    "from": {
	///      "description": "indicates the path of the source JSON element
	/// (according to JSON Pointer syntax) being moved or copied to the location
	/// indicated by the \"path\" attribute.\n",
	///      "type": "string"
	///    },
	///    "op": {
	///      "$ref": "#/components/schemas/PatchOperation"
	///    },
	///    "path": {
	///      "description": "contains a JSON pointer value (as defined in IETF
	/// RFC 6901) that references a location of a resource on which the patch
	/// operation shall be performed.\n",
	///      "type": "string"
	///    },
	///    "value": {}
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PatchItem {
		/// indicates the path of the source JSON element (according to JSON
		/// Pointer syntax) being moved or copied to the location indicated by
		/// the "path" attribute.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub from: Option<String>,
		pub op: PatchOperation,
		/// contains a JSON pointer value (as defined in IETF RFC 6901) that
		/// references a location of a resource on which the patch operation
		/// shall be performed.
		pub path: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub value: Option<::serde_json::Value>,
	}

	impl From<&PatchItem> for PatchItem {
		fn from(value: &PatchItem) -> Self {
			value.clone()
		}
	}

	/// Operations as defined in IETF RFC 6902.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Operations as defined in IETF RFC 6902.",
	///  "type": "string",
	///  "enum": [
	///    "add",
	///    "copy",
	///    "move",
	///    "remove",
	///    "replace",
	///    "test"
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
	pub enum PatchOperation {
		#[default]
		#[serde(rename = "add")]
		Add,
		#[serde(rename = "copy")]
		Copy,
		#[serde(rename = "move")]
		Move,
		#[serde(rename = "remove")]
		Remove,
		#[serde(rename = "replace")]
		Replace,
		#[serde(rename = "test")]
		Test,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PatchOperation> for PatchOperation {
		fn from(value: &PatchOperation) -> Self {
			value.clone()
		}
	}

	impl ToString for PatchOperation {
		fn to_string(&self) -> String {
			match *self {
				Self::Add => "add".to_string(),
				Self::Copy => "copy".to_string(),
				Self::Move => "move".to_string(),
				Self::Remove => "remove".to_string(),
				Self::Replace => "replace".to_string(),
				Self::Test => "test".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PatchOperation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"add" => Ok(Self::Add),
				"copy" => Ok(Self::Copy),
				"move" => Ok(Self::Move),
				"remove" => Ok(Self::Remove),
				"replace" => Ok(Self::Replace),
				"test" => Ok(Self::Test),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PatchOperation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PatchOperation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PatchOperation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The execution report result on failed modification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The execution report result on failed modification.",
	///  "type": "object",
	///  "required": [
	///    "report"
	///  ],
	///  "properties": {
	///    "report": {
	///      "description": "The execution report contains an array of report
	/// items. Each report item indicates one  failed modification.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ReportItem"
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
	pub struct PatchResult {
		/// The execution report contains an array of report items. Each report
		/// item indicates one  failed modification.
		pub report: Vec<ReportItem>,
	}

	impl From<&PatchResult> for PatchResult {
		fn from(value: &PatchResult) -> Self {
			value.clone()
		}
	}

	/// it shall represent the PC5 Flow Bit Rates
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "it shall represent the PC5 Flow Bit Rates",
	///  "type": "object",
	///  "properties": {
	///    "guaFbr": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "maxFbr": {
	///      "$ref": "#/components/schemas/BitRate"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Pc5FlowBitRates {
		#[serde(rename = "guaFbr", default, skip_serializing_if = "Option::is_none")]
		pub gua_fbr: Option<BitRate>,
		#[serde(rename = "maxFbr", default, skip_serializing_if = "Option::is_none")]
		pub max_fbr: Option<BitRate>,
	}

	impl From<&Pc5FlowBitRates> for Pc5FlowBitRates {
		fn from(value: &Pc5FlowBitRates) -> Self {
			value.clone()
		}
	}

	/// Contains policy data on the PC5 QoS parameters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains policy data on the PC5 QoS parameters.",
	///  "type": "object",
	///  "required": [
	///    "pc5QosFlowList"
	///  ],
	///  "properties": {
	///    "pc5LinkAmbr": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "pc5QosFlowList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Pc5QosFlowItem"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Pc5QoSPara {
		#[serde(
			rename = "pc5LinkAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pc5_link_ambr: Option<BitRate>,
		#[serde(rename = "pc5QosFlowList")]
		pub pc5_qos_flow_list: Vec<Pc5QosFlowItem>,
	}

	impl From<&Pc5QoSPara> for Pc5QoSPara {
		fn from(value: &Pc5QoSPara) -> Self {
			value.clone()
		}
	}

	/// Contains a PC5 QOS flow.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a PC5 QOS flow.",
	///  "type": "object",
	///  "required": [
	///    "pqi"
	///  ],
	///  "properties": {
	///    "pc5FlowBitRates": {
	///      "$ref": "#/components/schemas/Pc5FlowBitRates"
	///    },
	///    "pqi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "range": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Pc5QosFlowItem {
		#[serde(
			rename = "pc5FlowBitRates",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pc5_flow_bit_rates: Option<Pc5FlowBitRates>,
		pub pqi: _5qi,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub range: Option<Uinteger>,
	}

	impl From<&Pc5QosFlowItem> for Pc5QosFlowItem {
		fn from(value: &Pc5QosFlowItem) -> Self {
			value.clone()
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

	/// indicates the DNN and S-NSSAI combination of a PDU session.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "indicates the DNN and S-NSSAI combination of a PDU
	/// session.",
	///  "type": "object",
	///  "required": [
	///    "dnn",
	///    "snssai"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionInfo {
		pub dnn: Dnn,
		pub snssai: Snssai,
	}

	impl From<&PduSessionInfo> for PduSessionInfo {
		fn from(value: &PduSessionInfo) -> Self {
			value.clone()
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

	/// PduSessionType indicates the type of a PDU session. It shall comply with
	/// the provisions defined in table 5.4.3.3-1 but with the OpenAPI
	/// "nullable: true" property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "PduSessionType indicates the type of a PDU session. It
	/// shall comply with the provisions defined in table 5.4.3.3-1 but with the
	/// OpenAPI \"nullable: true\" property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/PduSessionType"
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
	pub enum PduSessionTypeRm {
		#[default]
		PduSessionType(PduSessionType),
		NullValue(NullValue),
	}

	impl From<&PduSessionTypeRm> for PduSessionTypeRm {
		fn from(value: &PduSessionTypeRm) -> Self {
			value.clone()
		}
	}

	impl From<PduSessionType> for PduSessionTypeRm {
		fn from(value: PduSessionType) -> Self {
			Self::PduSessionType(value)
		}
	}

	impl From<NullValue> for PduSessionTypeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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
		NewUnchecked,
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

	/// This data type is defined in the same way as the 'Pei' data type but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Pei'
	/// data type but with the OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern":
	/// "^(imei-[0-9]{15}|imeisv-[0-9]{16}|mac((-[0-9a-fA-F]{2}){6})(-untrusted)?
	/// |eui((-[0-9a-fA-F]{2}){8})|.+)$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PeiRm(pub Option<PeiRmInner>);

	impl ::std::ops::Deref for PeiRm {
		type Target = Option<PeiRmInner>;
		fn deref(&self) -> &Option<PeiRmInner> {
			&self.0
		}
	}

	impl From<PeiRm> for Option<PeiRmInner> {
		fn from(value: PeiRm) -> Self {
			value.0
		}
	}

	impl From<&PeiRm> for PeiRm {
		fn from(value: &PeiRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<PeiRmInner>> for PeiRm {
		fn from(value: Option<PeiRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the 'Pei' data type but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Pei'
	/// data type but with the OpenAPI 'nullable: true' property.\n",
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
		NewUnchecked,
	)]
	pub struct PeiRmInner(String);

	impl ::std::ops::Deref for PeiRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PeiRmInner> for String {
		fn from(value: PeiRmInner) -> Self {
			value.0
		}
	}

	impl From<&PeiRmInner> for PeiRmInner {
		fn from(value: &PeiRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PeiRmInner {
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

	impl ::std::convert::TryFrom<&str> for PeiRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PeiRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PeiRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PeiRmInner {
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

	/// Integer value identifying the physical cell identity (PCI), as
	/// definition of "PhysCellId" IE  in clause 6.3.2 of 3GPP TS 38.331.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Integer value identifying the physical cell identity
	/// (PCI), as definition of \"PhysCellId\" IE  in clause 6.3.2 of 3GPP TS
	/// 38.331.\n",
	///  "type": "integer",
	///  "maximum": 1007.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PhysCellId(pub i64);

	impl ::std::ops::Deref for PhysCellId {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<PhysCellId> for i64 {
		fn from(value: PhysCellId) -> Self {
			value.0
		}
	}

	impl From<&PhysCellId> for PhysCellId {
		fn from(value: &PhysCellId) -> Self {
			value.clone()
		}
	}

	impl From<i64> for PhysCellId {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for PhysCellId {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for PhysCellId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PhysCellId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PhysCellId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for PhysCellId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// PlmnAssiUeRadioCapId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/Bytes"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PlmnAssiUeRadioCapId(pub Bytes);

	impl ::std::ops::Deref for PlmnAssiUeRadioCapId {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<PlmnAssiUeRadioCapId> for Bytes {
		fn from(value: PlmnAssiUeRadioCapId) -> Self {
			value.0
		}
	}

	impl From<&PlmnAssiUeRadioCapId> for PlmnAssiUeRadioCapId {
		fn from(value: &PlmnAssiUeRadioCapId) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for PlmnAssiUeRadioCapId {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for PlmnAssiUeRadioCapId {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for PlmnAssiUeRadioCapId {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PlmnAssiUeRadioCapId {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PlmnAssiUeRadioCapId {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for PlmnAssiUeRadioCapId {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// This data type is defined in the same way as the 'PlmnIdNid' data type,
	/// but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'PlmnIdNid' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/PlmnIdNid"
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
	pub enum PlmnIdNidRm {
		#[default]
		PlmnIdNid(PlmnIdNid),
		NullValue(NullValue),
	}

	impl From<&PlmnIdNidRm> for PlmnIdNidRm {
		fn from(value: &PlmnIdNidRm) -> Self {
			value.clone()
		}
	}

	impl From<PlmnIdNid> for PlmnIdNidRm {
		fn from(value: PlmnIdNid) -> Self {
			Self::PlmnIdNid(value)
		}
	}

	impl From<NullValue> for PlmnIdNidRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// This data type is defined in the same way as the 'PlmnId' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'PlmnId' data type, but with the OpenAPI 'nullable: true' property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/PlmnId"
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
	pub enum PlmnIdRm {
		#[default]
		PlmnId(PlmnId),
		NullValue(NullValue),
	}

	impl From<&PlmnIdRm> for PlmnIdRm {
		fn from(value: &PlmnIdRm) -> Self {
			value.clone()
		}
	}

	impl From<PlmnId> for PlmnIdRm {
		fn from(value: PlmnId) -> Self {
			Self::PlmnId(value)
		}
	}

	impl From<NullValue> for PlmnIdRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Ellipsoid Point.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Ellipsoid Point.",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/GADShape"
	///    },
	///    {
	///      "type": "object",
	///      "required": [
	///        "point"
	///      ],
	///      "properties": {
	///        "point": {
	///          "$ref": "#/components/schemas/GeographicalCoordinates"
	///        }
	///      }
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Point {
		pub point: GeographicalCoordinates,
		pub shape: SupportedGadShapes,
	}

	impl From<&Point> for Point {
		fn from(value: &Point) -> Self {
			value.clone()
		}
	}

	/// Ellipsoid point with altitude.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Ellipsoid point with altitude.",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/GADShape"
	///    },
	///    {
	///      "type": "object",
	///      "required": [
	///        "altitude",
	///        "point"
	///      ],
	///      "properties": {
	///        "altitude": {
	///          "$ref": "#/components/schemas/Altitude"
	///        },
	///        "point": {
	///          "$ref": "#/components/schemas/GeographicalCoordinates"
	///        }
	///      }
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PointAltitude {
		pub altitude: Altitude,
		pub point: GeographicalCoordinates,
		pub shape: SupportedGadShapes,
	}

	impl From<&PointAltitude> for PointAltitude {
		fn from(value: &PointAltitude) -> Self {
			value.clone()
		}
	}

	/// Ellipsoid point with altitude and uncertainty ellipsoid.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Ellipsoid point with altitude and uncertainty
	/// ellipsoid.",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/GADShape"
	///    },
	///    {
	///      "type": "object",
	///      "required": [
	///        "altitude",
	///        "confidence",
	///        "point",
	///        "uncertaintyAltitude",
	///        "uncertaintyEllipse"
	///      ],
	///      "properties": {
	///        "altitude": {
	///          "$ref": "#/components/schemas/Altitude"
	///        },
	///        "confidence": {
	///          "$ref": "#/components/schemas/Confidence"
	///        },
	///        "point": {
	///          "$ref": "#/components/schemas/GeographicalCoordinates"
	///        },
	///        "uncertaintyAltitude": {
	///          "$ref": "#/components/schemas/Uncertainty"
	///        },
	///        "uncertaintyEllipse": {
	///          "$ref": "#/components/schemas/UncertaintyEllipse"
	///        },
	///        "vConfidence": {
	///          "$ref": "#/components/schemas/Confidence"
	///        }
	///      }
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PointAltitudeUncertainty {
		pub altitude: Altitude,
		pub confidence: Confidence,
		pub point: GeographicalCoordinates,
		pub shape: SupportedGadShapes,
		#[serde(rename = "uncertaintyAltitude")]
		pub uncertainty_altitude: Uncertainty,
		#[serde(rename = "uncertaintyEllipse")]
		pub uncertainty_ellipse: UncertaintyEllipse,
		#[serde(
			rename = "vConfidence",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub v_confidence: Option<Confidence>,
	}

	impl From<&PointAltitudeUncertainty> for PointAltitudeUncertainty {
		fn from(value: &PointAltitudeUncertainty) -> Self {
			value.clone()
		}
	}

	/// List of points.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of points.",
	///  "type": "array",
	///  "items": {
	///    "$ref": "#/components/schemas/GeographicalCoordinates"
	///  },
	///  "maxItems": 15,
	///  "minItems": 3
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PointList(pub Vec<GeographicalCoordinates>);

	impl ::std::ops::Deref for PointList {
		type Target = Vec<GeographicalCoordinates>;
		fn deref(&self) -> &Vec<GeographicalCoordinates> {
			&self.0
		}
	}

	impl From<PointList> for Vec<GeographicalCoordinates> {
		fn from(value: PointList) -> Self {
			value.0
		}
	}

	impl From<&PointList> for PointList {
		fn from(value: &PointList) -> Self {
			value.clone()
		}
	}

	impl From<Vec<GeographicalCoordinates>> for PointList {
		fn from(value: Vec<GeographicalCoordinates>) -> Self {
			Self(value)
		}
	}

	/// Ellipsoid point with uncertainty circle.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Ellipsoid point with uncertainty circle.",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/GADShape"
	///    },
	///    {
	///      "type": "object",
	///      "required": [
	///        "point",
	///        "uncertainty"
	///      ],
	///      "properties": {
	///        "point": {
	///          "$ref": "#/components/schemas/GeographicalCoordinates"
	///        },
	///        "uncertainty": {
	///          "$ref": "#/components/schemas/Uncertainty"
	///        }
	///      }
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PointUncertaintyCircle {
		pub point: GeographicalCoordinates,
		pub shape: SupportedGadShapes,
		pub uncertainty: Uncertainty,
	}

	impl From<&PointUncertaintyCircle> for PointUncertaintyCircle {
		fn from(value: &PointUncertaintyCircle) -> Self {
			value.clone()
		}
	}

	/// Ellipsoid point with uncertainty ellipse.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Ellipsoid point with uncertainty ellipse.",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/GADShape"
	///    },
	///    {
	///      "type": "object",
	///      "required": [
	///        "confidence",
	///        "point",
	///        "uncertaintyEllipse"
	///      ],
	///      "properties": {
	///        "confidence": {
	///          "$ref": "#/components/schemas/Confidence"
	///        },
	///        "point": {
	///          "$ref": "#/components/schemas/GeographicalCoordinates"
	///        },
	///        "uncertaintyEllipse": {
	///          "$ref": "#/components/schemas/UncertaintyEllipse"
	///        }
	///      }
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PointUncertaintyEllipse {
		pub confidence: Confidence,
		pub point: GeographicalCoordinates,
		pub shape: SupportedGadShapes,
		#[serde(rename = "uncertaintyEllipse")]
		pub uncertainty_ellipse: UncertaintyEllipse,
	}

	impl From<&PointUncertaintyEllipse> for PointUncertaintyEllipse {
		fn from(value: &PointUncertaintyEllipse) -> Self {
			value.clone()
		}
	}

	/// Polygon.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Polygon.",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/GADShape"
	///    },
	///    {
	///      "type": "object",
	///      "required": [
	///        "pointList"
	///      ],
	///      "properties": {
	///        "pointList": {
	///          "$ref": "#/components/schemas/PointList"
	///        }
	///      }
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Polygon {
		#[serde(rename = "pointList")]
		pub point_list: PointList,
		pub shape: SupportedGadShapes,
	}

	impl From<&Polygon> for Polygon {
		fn from(value: &Polygon) -> Self {
			value.clone()
		}
	}

	/// The enumeration LoggingDurationMdt defines Logging Duration for MDT in
	/// the trace. See 3GPP TS 32.422 for further description of the values. It
	/// shall comply with the provisions defined in table 5.6.3.13-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration LoggingDurationMdt defines Logging
	/// Duration for MDT in the trace. See 3GPP TS 32.422 for further
	/// description of the values. It shall comply with the provisions defined
	/// in table 5.6.3.13-1.\n",
	///  "type": "string",
	///  "enum": [
	///    "GNSS",
	///    "E_CELL_ID"
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
	pub enum PositioningMethodMdt {
		#[default]
		#[serde(rename = "GNSS")]
		Gnss,
		#[serde(rename = "E_CELL_ID")]
		ECellId,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PositioningMethodMdt> for PositioningMethodMdt {
		fn from(value: &PositioningMethodMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for PositioningMethodMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::Gnss => "GNSS".to_string(),
				Self::ECellId => "E_CELL_ID".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PositioningMethodMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"GNSS" => Ok(Self::Gnss),
				"E_CELL_ID" => Ok(Self::ECellId),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PositioningMethodMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PositioningMethodMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PositioningMethodMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// This enumeration is defined in the same way as the
	/// 'PreemptionCapability' enumeration, but with the OpenAPI 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This enumeration is defined in the same way as the
	/// 'PreemptionCapability' enumeration, but with the OpenAPI 'nullable:
	/// true' property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/PreemptionCapability"
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
	pub enum PreemptionCapabilityRm {
		#[default]
		PreemptionCapability(PreemptionCapability),
		NullValue(NullValue),
	}

	impl From<&PreemptionCapabilityRm> for PreemptionCapabilityRm {
		fn from(value: &PreemptionCapabilityRm) -> Self {
			value.clone()
		}
	}

	impl From<PreemptionCapability> for PreemptionCapabilityRm {
		fn from(value: PreemptionCapability) -> Self {
			Self::PreemptionCapability(value)
		}
	}

	impl From<NullValue> for PreemptionCapabilityRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// This enumeration is defined in the same way as the
	/// 'PreemptionVulnerability' enumeration, but with the OpenAPI 'nullable:
	/// true' property."
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This enumeration is defined in the same way as the
	/// 'PreemptionVulnerability' enumeration, but with the OpenAPI 'nullable:
	/// true' property.\" \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/PreemptionVulnerability"
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
	pub enum PreemptionVulnerabilityRm {
		#[default]
		PreemptionVulnerability(PreemptionVulnerability),
		NullValue(NullValue),
	}

	impl From<&PreemptionVulnerabilityRm> for PreemptionVulnerabilityRm {
		fn from(value: &PreemptionVulnerabilityRm) -> Self {
			value.clone()
		}
	}

	impl From<PreemptionVulnerability> for PreemptionVulnerabilityRm {
		fn from(value: PreemptionVulnerability) -> Self {
			Self::PreemptionVulnerability(value)
		}
	}

	impl From<NullValue> for PreemptionVulnerabilityRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// If the additionalPraId IE is present, this IE shall state the presence
	/// information of the UE for the individual PRA identified by the
	/// additionalPraId IE;  If the additionalPraId IE is not present, this IE
	/// shall state the presence information of the UE for the PRA identified by
	/// the praId IE.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "If the additionalPraId IE is present, this IE shall
	/// state the presence information of the UE for the individual PRA
	/// identified by the additionalPraId IE;  If the additionalPraId IE is not
	/// present, this IE shall state the presence information of the UE for the
	/// PRA identified by the praId IE.\n",
	///  "type": "object",
	///  "properties": {
	///    "additionalPraId": {
	///      "description": "This IE may be present if the praId IE is present
	/// and if it contains a PRA identifier referring to a set of Core Network
	/// predefined Presence Reporting Areas. When present, this IE shall contain
	/// a PRA Identifier of an individual PRA within the Set of Core Network
	/// predefined Presence Reporting Areas indicated by the praId IE. \n",
	///      "type": "string"
	///    },
	///    "ecgiList": {
	///      "description": "Represents the list of EUTRAN cell Ids that
	/// constitutes the area. This IE shall be present if the Area of Interest
	/// subscribed is a list of EUTRAN cell Ids. \n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ecgi"
	///      },
	///      "minItems": 1
	///    },
	///    "globalRanNodeIdList": {
	///      "description": "Represents the list of NG RAN node identifiers that
	/// constitutes the area. This IE shall be present if the Area of Interest
	/// subscribed is a list of NG RAN node identifiers. \n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GlobalRanNodeId"
	///      },
	///      "minItems": 1
	///    },
	///    "globaleNbIdList": {
	///      "description": "Represents the list of eNodeB identifiers that
	/// constitutes the area. This IE shall be  present if the Area of Interest
	/// subscribed is a list of eNodeB identifiers.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GlobalRanNodeId"
	///      },
	///      "minItems": 1
	///    },
	///    "ncgiList": {
	///      "description": "Represents the list of NR cell Ids that constitutes
	/// the area. This IE shall be present if the Area of Interest subscribed is
	/// a list of NR cell Ids. \n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ncgi"
	///      },
	///      "minItems": 1
	///    },
	///    "praId": {
	///      "description": "Represents an identifier of the Presence Reporting
	/// Area (see clause 28.10 of 3GPP  TS 23.003.  This IE shall be present  if
	/// the Area of Interest subscribed or reported is a Presence Reporting Area
	/// or a Set of Core Network predefined Presence Reporting Areas. When
	/// present, it shall be encoded as a string representing an integer in the
	/// following ranges: 0 to 8 388 607 for UE-dedicated PRA 8 388 608 to 16
	/// 777 215 for Core Network predefined PRA Examples: PRA ID 123 is encoded
	/// as \"123\" PRA ID 11 238 660 is encoded as \"11238660\"\n",
	///      "type": "string"
	///    },
	///    "presenceState": {
	///      "$ref": "#/components/schemas/PresenceState"
	///    },
	///    "trackingAreaList": {
	///      "description": "Represents the list of tracking areas that
	/// constitutes the area. This IE shall be present if the subscription or
	/// the event report is for tracking UE presence in the tracking areas. For
	/// non 3GPP access the TAI shall be the N3GPP TAI. \n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
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
	pub struct PresenceInfo {
		/// This IE may be present if the praId IE is present and if it contains
		/// a PRA identifier referring to a set of Core Network predefined
		/// Presence Reporting Areas. When present, this IE shall contain a PRA
		/// Identifier of an individual PRA within the Set of Core Network
		/// predefined Presence Reporting Areas indicated by the praId IE.
		#[serde(
			rename = "additionalPraId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_pra_id: Option<String>,
		/// Represents the list of EUTRAN cell Ids that constitutes the area.
		/// This IE shall be present if the Area of Interest subscribed is a
		/// list of EUTRAN cell Ids.
		#[serde(rename = "ecgiList", default, skip_serializing_if = "Vec::is_empty")]
		pub ecgi_list: Vec<Ecgi>,
		/// Represents the list of NG RAN node identifiers that constitutes the
		/// area. This IE shall be present if the Area of Interest subscribed is
		/// a list of NG RAN node identifiers.
		#[serde(
			rename = "globalRanNodeIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub global_ran_node_id_list: Vec<GlobalRanNodeId>,
		/// Represents the list of eNodeB identifiers that constitutes the area.
		/// This IE shall be  present if the Area of Interest subscribed is a
		/// list of eNodeB identifiers.
		#[serde(
			rename = "globaleNbIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub globale_nb_id_list: Vec<GlobalRanNodeId>,
		/// Represents the list of NR cell Ids that constitutes the area. This
		/// IE shall be present if the Area of Interest subscribed is a list of
		/// NR cell Ids.
		#[serde(rename = "ncgiList", default, skip_serializing_if = "Vec::is_empty")]
		pub ncgi_list: Vec<Ncgi>,
		/// Represents an identifier of the Presence Reporting Area (see clause
		/// 28.10 of 3GPP  TS 23.003.  This IE shall be present  if the Area of
		/// Interest subscribed or reported is a Presence Reporting Area or a
		/// Set of Core Network predefined Presence Reporting Areas. When
		/// present, it shall be encoded as a string representing an integer in
		/// the following ranges: 0 to 8 388 607 for UE-dedicated PRA 8 388 608
		/// to 16 777 215 for Core Network predefined PRA Examples: PRA ID 123
		/// is encoded as "123" PRA ID 11 238 660 is encoded as "11238660"
		#[serde(rename = "praId", default, skip_serializing_if = "Option::is_none")]
		pub pra_id: Option<String>,
		#[serde(
			rename = "presenceState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub presence_state: Option<PresenceState>,
		/// Represents the list of tracking areas that constitutes the area.
		/// This IE shall be present if the subscription or  the event report is
		/// for tracking UE presence in the tracking areas. For non 3GPP access
		/// the TAI shall be the N3GPP TAI.
		#[serde(
			rename = "trackingAreaList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tracking_area_list: Vec<Tai>,
	}

	impl From<&PresenceInfo> for PresenceInfo {
		fn from(value: &PresenceInfo) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the 'PresenceInfo' data
	/// type, but with the OpenAPI 'nullable: true' property.  If the
	/// additionalPraId IE is present, this IE shall state the presence
	/// information of the UE for the individual PRA identified by the
	/// additionalPraId IE;  If the additionalPraId IE is not present, this IE
	/// shall state the presence information of the UE for the PRA identified by
	/// the praId IE.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'PresenceInfo' data type, but with the OpenAPI 'nullable: true'
	/// property.  If the additionalPraId IE is present, this IE shall state the
	/// presence information of the UE for the individual PRA identified by the
	/// additionalPraId IE;  If the additionalPraId IE is not present, this IE
	/// shall state the presence information of the UE for the PRA identified by
	/// the praId IE. \n",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "additionalPraId": {
	///      "description": "This IE may be present if the praId IE is present
	/// and if it contains a PRA identifier referring to a set of Core Network
	/// predefined Presence Reporting Areas. When present, this IE shall contain
	/// a PRA Identifier of an individual PRA within the Set of Core Network
	/// predefined Presence Reporting Areas indicated by the praId IE.\n",
	///      "type": "string"
	///    },
	///    "ecgiList": {
	///      "description": "Represents the list of EUTRAN cell Ids that
	/// constitutes the area. This IE shall be present if the Area of Interest
	/// subscribed is a list of EUTRAN cell Ids.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ecgi"
	///      },
	///      "minItems": 0
	///    },
	///    "globalRanNodeIdList": {
	///      "description": "Represents the list of NG RAN node identifiers that
	/// constitutes the area. This IE shall be present if the Area of Interest
	/// subscribed is a list of NG RAN node identifiers.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GlobalRanNodeId"
	///      }
	///    },
	///    "globaleNbIdList": {
	///      "description": "Represents the list of eNodeB identifiers that
	/// constitutes the area. This IE shall be present if the Area of Interest
	/// subscribed is a list of eNodeB identifiers.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GlobalRanNodeId"
	///      },
	///      "minItems": 1
	///    },
	///    "ncgiList": {
	///      "description": "Represents the list of NR cell Ids that constitutes
	/// the area. This IE shall be present if the Area of Interest subscribed is
	/// a list of NR cell Ids.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ncgi"
	///      },
	///      "minItems": 0
	///    },
	///    "praId": {
	///      "description": "Represents an identifier of the Presence Reporting
	/// Area (see clause 28.10 of \n3GPP TS 23.003. This IE shall be present  if
	/// the Area of Interest subscribed or\nreported is a Presence Reporting
	/// Area or a Set of Core Network predefined Presence\nReporting Areas. When
	/// present, it shall be encoded as a string representing an integer\nin the
	/// following ranges:\n- 0 to 8 388 607 for UE-dedicated PRA\n- 8 388 608 to
	/// 16 777 215 for Core Network predefined PRA\nExamples:\nPRA ID 123 is
	/// encoded as \"123\"\nPRA ID 11 238 660 is encoded as \"11238660\"\n",
	///      "type": "string"
	///    },
	///    "presenceState": {
	///      "$ref": "#/components/schemas/PresenceState"
	///    },
	///    "trackingAreaList": {
	///      "description": "Represents the list of tracking areas that
	/// constitutes the area. This IE shall be present if the subscription or
	/// the event report  is for tracking UE presence in the tracking areas. For
	/// non 3GPP access the TAI shall be the N3GPP TAI.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
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
	pub struct PresenceInfoRm(pub Option<PresenceInfoRmInner>);

	impl ::std::ops::Deref for PresenceInfoRm {
		type Target = Option<PresenceInfoRmInner>;
		fn deref(&self) -> &Option<PresenceInfoRmInner> {
			&self.0
		}
	}

	impl From<PresenceInfoRm> for Option<PresenceInfoRmInner> {
		fn from(value: PresenceInfoRm) -> Self {
			value.0
		}
	}

	impl From<&PresenceInfoRm> for PresenceInfoRm {
		fn from(value: &PresenceInfoRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<PresenceInfoRmInner>> for PresenceInfoRm {
		fn from(value: Option<PresenceInfoRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the 'PresenceInfo' data
	/// type, but with the OpenAPI 'nullable: true' property.  If the
	/// additionalPraId IE is present, this IE shall state the presence
	/// information of the UE for the individual PRA identified by the
	/// additionalPraId IE;  If the additionalPraId IE is not present, this IE
	/// shall state the presence information of the UE for the PRA identified by
	/// the praId IE.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'PresenceInfo' data type, but with the OpenAPI 'nullable: true'
	/// property.  If the additionalPraId IE is present, this IE shall state the
	/// presence information of the UE for the individual PRA identified by the
	/// additionalPraId IE;  If the additionalPraId IE is not present, this IE
	/// shall state the presence information of the UE for the PRA identified by
	/// the praId IE. \n",
	///  "type": "object",
	///  "properties": {
	///    "additionalPraId": {
	///      "description": "This IE may be present if the praId IE is present
	/// and if it contains a PRA identifier referring to a set of Core Network
	/// predefined Presence Reporting Areas. When present, this IE shall contain
	/// a PRA Identifier of an individual PRA within the Set of Core Network
	/// predefined Presence Reporting Areas indicated by the praId IE.\n",
	///      "type": "string"
	///    },
	///    "ecgiList": {
	///      "description": "Represents the list of EUTRAN cell Ids that
	/// constitutes the area. This IE shall be present if the Area of Interest
	/// subscribed is a list of EUTRAN cell Ids.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ecgi"
	///      },
	///      "minItems": 0
	///    },
	///    "globalRanNodeIdList": {
	///      "description": "Represents the list of NG RAN node identifiers that
	/// constitutes the area. This IE shall be present if the Area of Interest
	/// subscribed is a list of NG RAN node identifiers.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GlobalRanNodeId"
	///      }
	///    },
	///    "globaleNbIdList": {
	///      "description": "Represents the list of eNodeB identifiers that
	/// constitutes the area. This IE shall be present if the Area of Interest
	/// subscribed is a list of eNodeB identifiers.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GlobalRanNodeId"
	///      },
	///      "minItems": 1
	///    },
	///    "ncgiList": {
	///      "description": "Represents the list of NR cell Ids that constitutes
	/// the area. This IE shall be present if the Area of Interest subscribed is
	/// a list of NR cell Ids.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ncgi"
	///      },
	///      "minItems": 0
	///    },
	///    "praId": {
	///      "description": "Represents an identifier of the Presence Reporting
	/// Area (see clause 28.10 of \n3GPP TS 23.003. This IE shall be present  if
	/// the Area of Interest subscribed or\nreported is a Presence Reporting
	/// Area or a Set of Core Network predefined Presence\nReporting Areas. When
	/// present, it shall be encoded as a string representing an integer\nin the
	/// following ranges:\n- 0 to 8 388 607 for UE-dedicated PRA\n- 8 388 608 to
	/// 16 777 215 for Core Network predefined PRA\nExamples:\nPRA ID 123 is
	/// encoded as \"123\"\nPRA ID 11 238 660 is encoded as \"11238660\"\n",
	///      "type": "string"
	///    },
	///    "presenceState": {
	///      "$ref": "#/components/schemas/PresenceState"
	///    },
	///    "trackingAreaList": {
	///      "description": "Represents the list of tracking areas that
	/// constitutes the area. This IE shall be present if the subscription or
	/// the event report  is for tracking UE presence in the tracking areas. For
	/// non 3GPP access the TAI shall be the N3GPP TAI.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
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
	pub struct PresenceInfoRmInner {
		/// This IE may be present if the praId IE is present and if it contains
		/// a PRA identifier referring to a set of Core Network predefined
		/// Presence Reporting Areas. When present, this IE shall contain a PRA
		/// Identifier of an individual PRA within the Set of Core Network
		/// predefined Presence Reporting Areas indicated by the praId IE.
		#[serde(
			rename = "additionalPraId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_pra_id: Option<String>,
		/// Represents the list of EUTRAN cell Ids that constitutes the area.
		/// This IE shall be present if the Area of Interest subscribed is a
		/// list of EUTRAN cell Ids.
		#[serde(rename = "ecgiList", default, skip_serializing_if = "Vec::is_empty")]
		pub ecgi_list: Vec<Ecgi>,
		/// Represents the list of NG RAN node identifiers that constitutes the
		/// area. This IE shall be present if the Area of Interest subscribed is
		/// a list of NG RAN node identifiers.
		#[serde(
			rename = "globalRanNodeIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub global_ran_node_id_list: Vec<GlobalRanNodeId>,
		/// Represents the list of eNodeB identifiers that constitutes the area.
		/// This IE shall be present if the Area of Interest subscribed is a
		/// list of eNodeB identifiers.
		#[serde(
			rename = "globaleNbIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub globale_nb_id_list: Vec<GlobalRanNodeId>,
		/// Represents the list of NR cell Ids that constitutes the area. This
		/// IE shall be present if the Area of Interest subscribed is a list of
		/// NR cell Ids.
		#[serde(rename = "ncgiList", default, skip_serializing_if = "Vec::is_empty")]
		pub ncgi_list: Vec<Ncgi>,
		/// Represents an identifier of the Presence Reporting Area (see clause
		/// 28.10 of 3GPP TS 23.003. This IE shall be present  if the
		/// Area of Interest subscribed or reported is a Presence
		/// Reporting Area or a Set of Core Network predefined Presence
		/// Reporting Areas. When present, it shall be encoded as a string
		/// representing an integer in the following ranges:
		/// - 0 to 8 388 607 for UE-dedicated PRA
		/// - 8 388 608 to 16 777 215 for Core Network predefined PRA
		/// Examples:
		/// PRA ID 123 is encoded as "123"
		/// PRA ID 11 238 660 is encoded as "11238660"
		#[serde(rename = "praId", default, skip_serializing_if = "Option::is_none")]
		pub pra_id: Option<String>,
		#[serde(
			rename = "presenceState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub presence_state: Option<PresenceState>,
		/// Represents the list of tracking areas that constitutes the area.
		/// This IE shall be present if the subscription or the event report  is
		/// for tracking UE presence in the tracking areas. For non 3GPP access
		/// the TAI shall be the N3GPP TAI.
		#[serde(
			rename = "trackingAreaList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tracking_area_list: Vec<Tai>,
	}

	impl From<&PresenceInfoRmInner> for PresenceInfoRmInner {
		fn from(value: &PresenceInfoRmInner) -> Self {
			value.clone()
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

	/// If the contentType attribute is set to "application/json", then this
	/// attribute describes  the attributes of the JSON object of the body.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "If the contentType attribute is set to
	/// \"application/json\", then this attribute describes  the attributes of
	/// the JSON object of the body.\n",
	///  "type": "object",
	///  "required": [
	///    "name"
	///  ],
	///  "properties": {
	///    "name": {
	///      "description": "The name of the property",
	///      "type": "string"
	///    },
	///    "regex": {
	///      "description": "A regular expression string to be applied to the
	/// value of the property.",
	///      "type": "string"
	///    },
	///    "required": {
	///      "description": "Indicates whether the property is required – true=
	/// required –  false(default)= not required.\n",
	///      "type": "boolean"
	///    },
	///    "value": {
	///      "description": "The property value. When present, it shall be a
	/// valid JSON string.",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Property {
		/// The name of the property
		pub name: String,
		/// A regular expression string to be applied to the value of the
		/// property.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub regex: Option<String>,
		/// Indicates whether the property is required – true= required –
		/// false(default)= not required.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub required: Option<bool>,
		/// The property value. When present, it shall be a valid JSON string.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub value: Option<String>,
	}

	impl From<&Property> for Property {
		fn from(value: &Property) -> Self {
			value.clone()
		}
	}

	/// Indicates whether the UE is authorized to use ProSe related services.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates whether the UE is authorized to use ProSe
	/// related services.\n",
	///  "type": "object",
	///  "properties": {
	///    "proseDirectCommunicationAuth": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "proseDirectDiscoveryAuth": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "proseL2RelayAuth": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "proseL2RemoteAuth": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "proseL3RelayAuth": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "proseL3RemoteAuth": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProseServiceAuth {
		#[serde(
			rename = "proseDirectCommunicationAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_direct_communication_auth: Option<UeAuth>,
		#[serde(
			rename = "proseDirectDiscoveryAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_direct_discovery_auth: Option<UeAuth>,
		#[serde(
			rename = "proseL2RelayAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_l2_relay_auth: Option<UeAuth>,
		#[serde(
			rename = "proseL2RemoteAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_l2_remote_auth: Option<UeAuth>,
		#[serde(
			rename = "proseL3RelayAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_l3_relay_auth: Option<UeAuth>,
		#[serde(
			rename = "proseL3RemoteAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_l3_remote_auth: Option<UeAuth>,
	}

	impl From<&ProseServiceAuth> for ProseServiceAuth {
		fn from(value: &ProseServiceAuth) -> Self {
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

	/// This data type is defined in the same way as the 'Qfi' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Qfi'
	/// data type, but with the OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 63.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QfiRm(pub Option<i64>);

	impl ::std::ops::Deref for QfiRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<QfiRm> for Option<i64> {
		fn from(value: QfiRm) -> Self {
			value.0
		}
	}

	impl From<&QfiRm> for QfiRm {
		fn from(value: &QfiRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for QfiRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
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

	/// This enumeration is defined in the same way as the 'QosResourceType'
	/// enumeration, but with the OpenAPI 'nullable: true' property. "
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This enumeration is defined in the same way as the
	/// 'QosResourceType' enumeration, but with the OpenAPI 'nullable: true'
	/// property. \"\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/QosResourceType"
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
	pub enum QosResourceTypeRm {
		#[default]
		QosResourceType(QosResourceType),
		NullValue(NullValue),
	}

	impl From<&QosResourceTypeRm> for QosResourceTypeRm {
		fn from(value: &QosResourceTypeRm) -> Self {
			value.clone()
		}
	}

	impl From<QosResourceType> for QosResourceTypeRm {
		fn from(value: QosResourceType) -> Self {
			Self::QosResourceType(value)
		}
	}

	impl From<NullValue> for QosResourceTypeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// Provides information about the radio access but with the OpenAPI
	/// 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Provides information about the radio access but with
	/// the OpenAPI 'nullable: true' property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/RatType"
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
	pub enum RatTypeRm {
		#[default]
		RatType(RatType),
		NullValue(NullValue),
	}

	impl From<&RatTypeRm> for RatTypeRm {
		fn from(value: &RatTypeRm) -> Self {
			value.clone()
		}
	}

	impl From<RatType> for RatTypeRm {
		fn from(value: RatType) -> Self {
			Self::RatType(value)
		}
	}

	impl From<NullValue> for RatTypeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// RatingGroup
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/Uint32"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RatingGroup(pub Uint32);

	impl ::std::ops::Deref for RatingGroup {
		type Target = Uint32;
		fn deref(&self) -> &Uint32 {
			&self.0
		}
	}

	impl From<RatingGroup> for Uint32 {
		fn from(value: RatingGroup) -> Self {
			value.0
		}
	}

	impl From<&RatingGroup> for RatingGroup {
		fn from(value: &RatingGroup) -> Self {
			value.clone()
		}
	}

	impl From<Uint32> for RatingGroup {
		fn from(value: Uint32) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for RatingGroup {
		type Err = <Uint32 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for RatingGroup {
		type Error = <Uint32 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RatingGroup {
		type Error = <Uint32 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RatingGroup {
		type Error = <Uint32 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for RatingGroup {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// This data type is defined in the same way as the ' RefToBinaryData '
	/// data type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the '
	/// RefToBinaryData ' data type, but with the OpenAPI 'nullable: true'
	/// property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/RefToBinaryData"
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
	pub enum RefToBinaryDataRm {
		#[default]
		RefToBinaryData(RefToBinaryData),
		NullValue(NullValue),
	}

	impl From<&RefToBinaryDataRm> for RefToBinaryDataRm {
		fn from(value: &RefToBinaryDataRm) -> Self {
			value.clone()
		}
	}

	impl From<RefToBinaryData> for RefToBinaryDataRm {
		fn from(value: RefToBinaryData) -> Self {
			Self::RefToBinaryData(value)
		}
	}

	impl From<NullValue> for RefToBinaryDataRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// This enumeration is defined in the same way as the
	/// 'ReflectiveQosAttribute' enumeration, but with the OpenAPI 'nullable:
	/// true' property. "
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This enumeration is defined in the same way as the
	/// 'ReflectiveQosAttribute' enumeration, but with the OpenAPI 'nullable:
	/// true' property. \"\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/ReflectiveQoSAttribute"
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
	pub enum ReflectiveQoSAttributeRm {
		#[default]
		ReflectiveQoSAttribute(ReflectiveQoSAttribute),
		NullValue(NullValue),
	}

	impl From<&ReflectiveQoSAttributeRm> for ReflectiveQoSAttributeRm {
		fn from(value: &ReflectiveQoSAttributeRm) -> Self {
			value.clone()
		}
	}

	impl From<ReflectiveQoSAttribute> for ReflectiveQoSAttributeRm {
		fn from(value: ReflectiveQoSAttribute) -> Self {
			Self::ReflectiveQoSAttribute(value)
		}
	}

	impl From<NullValue> for ReflectiveQoSAttributeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Relative Cartesian Location
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Relative Cartesian Location",
	///  "type": "object",
	///  "required": [
	///    "x",
	///    "y"
	///  ],
	///  "properties": {
	///    "x": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "y": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "z": {
	///      "$ref": "#/components/schemas/Float"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RelativeCartesianLocation {
		pub x: Float,
		pub y: Float,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub z: Option<Float>,
	}

	impl From<&RelativeCartesianLocation> for RelativeCartesianLocation {
		fn from(value: &RelativeCartesianLocation) -> Self {
			value.clone()
		}
	}

	/// Relay Service Code to identify a connectivity service provided by the
	/// UE-to-Network relay.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Relay Service Code to identify a connectivity service
	/// provided by the UE-to-Network relay.\n",
	///  "type": "integer",
	///  "maximum": 16777215.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RelayServiceCode(pub i64);

	impl ::std::ops::Deref for RelayServiceCode {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<RelayServiceCode> for i64 {
		fn from(value: RelayServiceCode) -> Self {
			value.0
		}
	}

	impl From<&RelayServiceCode> for RelayServiceCode {
		fn from(value: &RelayServiceCode) -> Self {
			value.clone()
		}
	}

	impl From<i64> for RelayServiceCode {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for RelayServiceCode {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for RelayServiceCode {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RelayServiceCode {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RelayServiceCode {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for RelayServiceCode {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// The enumeration ReportAmountMdt defines Report Amount for MDT in the
	/// trace. See 3GPP TS 32.422 for further description of the values. It
	/// shall comply with the provisions defined in table 5.6.3.10-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration ReportAmountMdt defines Report Amount
	/// for MDT in the trace. See 3GPP TS 32.422 for further description of the
	/// values. It shall comply with the provisions defined in table
	/// 5.6.3.10-1.\n",
	///  "type": "string",
	///  "enum": [
	///    1,
	///    2,
	///    4,
	///    8,
	///    16,
	///    32,
	///    64,
	///    "infinity"
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
	pub enum ReportAmountMdt {
		#[default]
		#[serde(rename = "1")]
		NUM1,
		#[serde(rename = "2")]
		NUM2,
		#[serde(rename = "4")]
		NUM4,
		#[serde(rename = "8")]
		NUM8,
		#[serde(rename = "16")]
		NUM16,
		#[serde(rename = "32")]
		NUM32,
		#[serde(rename = "64")]
		NUM64,
		#[serde(rename = "infinity")]
		Infinity,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReportAmountMdt> for ReportAmountMdt {
		fn from(value: &ReportAmountMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for ReportAmountMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::NUM1 => "1".to_string(),
				Self::NUM2 => "2".to_string(),
				Self::NUM4 => "4".to_string(),
				Self::NUM8 => "8".to_string(),
				Self::NUM16 => "16".to_string(),
				Self::NUM32 => "32".to_string(),
				Self::NUM64 => "64".to_string(),
				Self::Infinity => "infinity".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReportAmountMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"1" => Ok(Self::NUM1),
				"2" => Ok(Self::NUM2),
				"4" => Ok(Self::NUM4),
				"8" => Ok(Self::NUM8),
				"16" => Ok(Self::NUM16),
				"32" => Ok(Self::NUM32),
				"64" => Ok(Self::NUM64),
				"infinity" => Ok(Self::Infinity),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReportAmountMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportAmountMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportAmountMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration ReportIntervalMdt defines Report Interval for MDT in the
	/// trace. See 3GPP TS 32.422 for further description of the values. It
	/// shall comply with the provisions defined in table 5.6.3.9-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration ReportIntervalMdt defines Report
	/// Interval for MDT in the trace. See 3GPP TS 32.422 for further
	/// description of the values. It shall comply with the provisions defined
	/// in table 5.6.3.9-1.\n",
	///  "type": "string",
	///  "enum": [
	///    120,
	///    240,
	///    480,
	///    640,
	///    1024,
	///    2048,
	///    5120,
	///    10240,
	///    60000,
	///    360000,
	///    720000,
	///    1800000,
	///    3600000
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
	pub enum ReportIntervalMdt {
		#[default]
		#[serde(rename = "120")]
		NUM120,
		#[serde(rename = "240")]
		NUM240,
		#[serde(rename = "480")]
		NUM480,
		#[serde(rename = "640")]
		NUM640,
		#[serde(rename = "1024")]
		NUM1024,
		#[serde(rename = "2048")]
		NUM2048,
		#[serde(rename = "5120")]
		NUM5120,
		#[serde(rename = "10240")]
		NUM10240,
		#[serde(rename = "60000")]
		NUM60000,
		#[serde(rename = "360000")]
		NUM360000,
		#[serde(rename = "720000")]
		NUM720000,
		#[serde(rename = "1800000")]
		NUM1800000,
		#[serde(rename = "3600000")]
		NUM3600000,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReportIntervalMdt> for ReportIntervalMdt {
		fn from(value: &ReportIntervalMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for ReportIntervalMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::NUM120 => "120".to_string(),
				Self::NUM240 => "240".to_string(),
				Self::NUM480 => "480".to_string(),
				Self::NUM640 => "640".to_string(),
				Self::NUM1024 => "1024".to_string(),
				Self::NUM2048 => "2048".to_string(),
				Self::NUM5120 => "5120".to_string(),
				Self::NUM10240 => "10240".to_string(),
				Self::NUM60000 => "60000".to_string(),
				Self::NUM360000 => "360000".to_string(),
				Self::NUM720000 => "720000".to_string(),
				Self::NUM1800000 => "1800000".to_string(),
				Self::NUM3600000 => "3600000".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReportIntervalMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"120" => Ok(Self::NUM120),
				"240" => Ok(Self::NUM240),
				"480" => Ok(Self::NUM480),
				"640" => Ok(Self::NUM640),
				"1024" => Ok(Self::NUM1024),
				"2048" => Ok(Self::NUM2048),
				"5120" => Ok(Self::NUM5120),
				"10240" => Ok(Self::NUM10240),
				"60000" => Ok(Self::NUM60000),
				"360000" => Ok(Self::NUM360000),
				"720000" => Ok(Self::NUM720000),
				"1800000" => Ok(Self::NUM1800000),
				"3600000" => Ok(Self::NUM3600000),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReportIntervalMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportIntervalMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportIntervalMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration ReportIntervalNrMdt defines Report Interval in NR for
	/// MDT in the trace. See 3GPP TS 32.422 for further description of the
	/// values. It shall comply with the provisions defined in table 5.6.3.17-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration ReportIntervalNrMdt defines Report
	/// Interval in NR for MDT in the trace. See 3GPP TS 32.422 for further
	/// description of the values. It shall comply with the provisions defined
	/// in table 5.6.3.17-1.\n",
	///  "type": "string",
	///  "enum": [
	///    120,
	///    240,
	///    480,
	///    640,
	///    1024,
	///    2048,
	///    5120,
	///    10240,
	///    20480,
	///    40960,
	///    60000,
	///    360000,
	///    720000,
	///    1800000,
	///    3600000
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
	pub enum ReportIntervalNrMdt {
		#[default]
		#[serde(rename = "120")]
		NUM120,
		#[serde(rename = "240")]
		NUM240,
		#[serde(rename = "480")]
		NUM480,
		#[serde(rename = "640")]
		NUM640,
		#[serde(rename = "1024")]
		NUM1024,
		#[serde(rename = "2048")]
		NUM2048,
		#[serde(rename = "5120")]
		NUM5120,
		#[serde(rename = "10240")]
		NUM10240,
		#[serde(rename = "20480")]
		NUM20480,
		#[serde(rename = "40960")]
		NUM40960,
		#[serde(rename = "60000")]
		NUM60000,
		#[serde(rename = "360000")]
		NUM360000,
		#[serde(rename = "720000")]
		NUM720000,
		#[serde(rename = "1800000")]
		NUM1800000,
		#[serde(rename = "3600000")]
		NUM3600000,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReportIntervalNrMdt> for ReportIntervalNrMdt {
		fn from(value: &ReportIntervalNrMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for ReportIntervalNrMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::NUM120 => "120".to_string(),
				Self::NUM240 => "240".to_string(),
				Self::NUM480 => "480".to_string(),
				Self::NUM640 => "640".to_string(),
				Self::NUM1024 => "1024".to_string(),
				Self::NUM2048 => "2048".to_string(),
				Self::NUM5120 => "5120".to_string(),
				Self::NUM10240 => "10240".to_string(),
				Self::NUM20480 => "20480".to_string(),
				Self::NUM40960 => "40960".to_string(),
				Self::NUM60000 => "60000".to_string(),
				Self::NUM360000 => "360000".to_string(),
				Self::NUM720000 => "720000".to_string(),
				Self::NUM1800000 => "1800000".to_string(),
				Self::NUM3600000 => "3600000".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReportIntervalNrMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"120" => Ok(Self::NUM120),
				"240" => Ok(Self::NUM240),
				"480" => Ok(Self::NUM480),
				"640" => Ok(Self::NUM640),
				"1024" => Ok(Self::NUM1024),
				"2048" => Ok(Self::NUM2048),
				"5120" => Ok(Self::NUM5120),
				"10240" => Ok(Self::NUM10240),
				"20480" => Ok(Self::NUM20480),
				"40960" => Ok(Self::NUM40960),
				"60000" => Ok(Self::NUM60000),
				"360000" => Ok(Self::NUM360000),
				"720000" => Ok(Self::NUM720000),
				"1800000" => Ok(Self::NUM1800000),
				"3600000" => Ok(Self::NUM3600000),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReportIntervalNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportIntervalNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportIntervalNrMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// indicates performed modivications.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "indicates performed modivications.",
	///  "type": "object",
	///  "required": [
	///    "path"
	///  ],
	///  "properties": {
	///    "path": {
	///      "description": "Contains a JSON pointer value (as defined in IETF
	/// RFC 6901) that references a  location of a resource to which the
	/// modification is subject.\n",
	///      "type": "string"
	///    },
	///    "reason": {
	///      "description": "A human-readable reason providing details on the
	/// reported modification failure.  The reason string should identify the
	/// operation that failed using the operation's  array index to assist in
	/// correlation of the invalid parameter with the failed  operation, e.g.
	/// \"Replacement value invalid for attribute (failed operation index=
	/// 4)\".\n",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReportItem {
		/// Contains a JSON pointer value (as defined in IETF RFC 6901) that
		/// references a  location of a resource to which the modification is
		/// subject.
		pub path: String,
		/// A human-readable reason providing details on the reported
		/// modification failure.  The reason string should identify the
		/// operation that failed using the operation's  array index to assist
		/// in correlation of the invalid parameter with the failed  operation,
		/// e.g. "Replacement value invalid for attribute (failed operation
		/// index= 4)".
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub reason: Option<String>,
	}

	impl From<&ReportItem> for ReportItem {
		fn from(value: &ReportItem) -> Self {
			value.clone()
		}
	}

	/// The enumeration ReportTypeMdt defines Report Type for logged MDT in the
	/// trace. See 3GPP TS 32.422 for further description of the values. It
	/// shall comply with the provisions defined in table 5.6.3.4-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration ReportTypeMdt defines Report Type for
	/// logged MDT in the trace. See 3GPP TS 32.422 for further description of
	/// the values. It shall comply with the provisions defined in table
	/// 5.6.3.4-1.\n",
	///  "type": "string",
	///  "enum": [
	///    "PERIODICAL",
	///    "EVENT_TRIGGED"
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
	pub enum ReportTypeMdt {
		#[default]
		#[serde(rename = "PERIODICAL")]
		Periodical,
		#[serde(rename = "EVENT_TRIGGED")]
		EventTrigged,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReportTypeMdt> for ReportTypeMdt {
		fn from(value: &ReportTypeMdt) -> Self {
			value.clone()
		}
	}

	impl ToString for ReportTypeMdt {
		fn to_string(&self) -> String {
			match *self {
				Self::Periodical => "PERIODICAL".to_string(),
				Self::EventTrigged => "EVENT_TRIGGED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReportTypeMdt {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PERIODICAL" => Ok(Self::Periodical),
				"EVENT_TRIGGED" => Ok(Self::EventTrigged),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReportTypeMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportTypeMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportTypeMdt {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The enumeration ReportingTrigger defines Reporting Triggers for MDT in
	/// the trace. See 3GPP TS 32.42] for further  description of the values. It
	/// shall comply with the provisions defined in table 5.6.3.8-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration ReportingTrigger defines Reporting
	/// Triggers for MDT in the trace. See 3GPP TS 32.42] for further
	/// description of the values. It shall comply with the provisions defined
	/// in table 5.6.3.8-1.\n",
	///  "type": "string",
	///  "enum": [
	///    "PERIODICAL",
	///    "EVENT_A2",
	///    "EVENT_A2_PERIODIC",
	///    "ALL_RRM_EVENT_TRIGGERS"
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
	pub enum ReportingTrigger {
		#[default]
		#[serde(rename = "PERIODICAL")]
		Periodical,
		#[serde(rename = "EVENT_A2")]
		EventA2,
		#[serde(rename = "EVENT_A2_PERIODIC")]
		EventA2Periodic,
		#[serde(rename = "ALL_RRM_EVENT_TRIGGERS")]
		AllRrmEventTriggers,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReportingTrigger> for ReportingTrigger {
		fn from(value: &ReportingTrigger) -> Self {
			value.clone()
		}
	}

	impl ToString for ReportingTrigger {
		fn to_string(&self) -> String {
			match *self {
				Self::Periodical => "PERIODICAL".to_string(),
				Self::EventA2 => "EVENT_A2".to_string(),
				Self::EventA2Periodic => "EVENT_A2_PERIODIC".to_string(),
				Self::AllRrmEventTriggers => "ALL_RRM_EVENT_TRIGGERS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReportingTrigger {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PERIODICAL" => Ok(Self::Periodical),
				"EVENT_A2" => Ok(Self::EventA2),
				"EVENT_A2_PERIODIC" => Ok(Self::EventA2Periodic),
				"ALL_RRM_EVENT_TRIGGERS" => Ok(Self::AllRrmEventTriggers),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReportingTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportingTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportingTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the reservation priority.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the reservation priority.",
	///  "type": "string",
	///  "enum": [
	///    "PRIO_1",
	///    "PRIO_2",
	///    "PRIO_3",
	///    "PRIO_4",
	///    "PRIO_5",
	///    "PRIO_6",
	///    "PRIO_7",
	///    "PRIO_8",
	///    "PRIO_9",
	///    "PRIO_10",
	///    "PRIO_11",
	///    "PRIO_12",
	///    "PRIO_13",
	///    "PRIO_14",
	///    "PRIO_15",
	///    "PRIO_16"
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
	pub enum ReservPriority {
		#[default]
		#[serde(rename = "PRIO_1")]
		Prio1,
		#[serde(rename = "PRIO_2")]
		Prio2,
		#[serde(rename = "PRIO_3")]
		Prio3,
		#[serde(rename = "PRIO_4")]
		Prio4,
		#[serde(rename = "PRIO_5")]
		Prio5,
		#[serde(rename = "PRIO_6")]
		Prio6,
		#[serde(rename = "PRIO_7")]
		Prio7,
		#[serde(rename = "PRIO_8")]
		Prio8,
		#[serde(rename = "PRIO_9")]
		Prio9,
		#[serde(rename = "PRIO_10")]
		Prio10,
		#[serde(rename = "PRIO_11")]
		Prio11,
		#[serde(rename = "PRIO_12")]
		Prio12,
		#[serde(rename = "PRIO_13")]
		Prio13,
		#[serde(rename = "PRIO_14")]
		Prio14,
		#[serde(rename = "PRIO_15")]
		Prio15,
		#[serde(rename = "PRIO_16")]
		Prio16,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReservPriority> for ReservPriority {
		fn from(value: &ReservPriority) -> Self {
			value.clone()
		}
	}

	impl ToString for ReservPriority {
		fn to_string(&self) -> String {
			match *self {
				Self::Prio1 => "PRIO_1".to_string(),
				Self::Prio2 => "PRIO_2".to_string(),
				Self::Prio3 => "PRIO_3".to_string(),
				Self::Prio4 => "PRIO_4".to_string(),
				Self::Prio5 => "PRIO_5".to_string(),
				Self::Prio6 => "PRIO_6".to_string(),
				Self::Prio7 => "PRIO_7".to_string(),
				Self::Prio8 => "PRIO_8".to_string(),
				Self::Prio9 => "PRIO_9".to_string(),
				Self::Prio10 => "PRIO_10".to_string(),
				Self::Prio11 => "PRIO_11".to_string(),
				Self::Prio12 => "PRIO_12".to_string(),
				Self::Prio13 => "PRIO_13".to_string(),
				Self::Prio14 => "PRIO_14".to_string(),
				Self::Prio15 => "PRIO_15".to_string(),
				Self::Prio16 => "PRIO_16".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReservPriority {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PRIO_1" => Ok(Self::Prio1),
				"PRIO_2" => Ok(Self::Prio2),
				"PRIO_3" => Ok(Self::Prio3),
				"PRIO_4" => Ok(Self::Prio4),
				"PRIO_5" => Ok(Self::Prio5),
				"PRIO_6" => Ok(Self::Prio6),
				"PRIO_7" => Ok(Self::Prio7),
				"PRIO_8" => Ok(Self::Prio8),
				"PRIO_9" => Ok(Self::Prio9),
				"PRIO_10" => Ok(Self::Prio10),
				"PRIO_11" => Ok(Self::Prio11),
				"PRIO_12" => Ok(Self::Prio12),
				"PRIO_13" => Ok(Self::Prio13),
				"PRIO_14" => Ok(Self::Prio14),
				"PRIO_15" => Ok(Self::Prio15),
				"PRIO_16" => Ok(Self::Prio16),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReservPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReservPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReservPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// It contains the restriction type ALLOWED_AREAS or NOT_ALLOWED_AREAS.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It contains the restriction type ALLOWED_AREAS or
	/// NOT_ALLOWED_AREAS.",
	///  "type": "string",
	///  "enum": [
	///    "ALLOWED_AREAS",
	///    "NOT_ALLOWED_AREAS"
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
	pub enum RestrictionType {
		#[default]
		#[serde(rename = "ALLOWED_AREAS")]
		AllowedAreas,
		#[serde(rename = "NOT_ALLOWED_AREAS")]
		NotAllowedAreas,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RestrictionType> for RestrictionType {
		fn from(value: &RestrictionType) -> Self {
			value.clone()
		}
	}

	impl ToString for RestrictionType {
		fn to_string(&self) -> String {
			match *self {
				Self::AllowedAreas => "ALLOWED_AREAS".to_string(),
				Self::NotAllowedAreas => "NOT_ALLOWED_AREAS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RestrictionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ALLOWED_AREAS" => Ok(Self::AllowedAreas),
				"NOT_ALLOWED_AREAS" => Ok(Self::NotAllowedAreas),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RestrictionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RestrictionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RestrictionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// It contains the restriction type ALLOWED_AREAS or NOT_ALLOWED_AREAS but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It contains the restriction type ALLOWED_AREAS or NOT_ALLOWED_AREAS but with the OpenAPI 'nullable: true' property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/RestrictionType"
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
	pub enum RestrictionTypeRm {
		#[default]
		RestrictionType(RestrictionType),
		NullValue(NullValue),
	}

	impl From<&RestrictionTypeRm> for RestrictionTypeRm {
		fn from(value: &RestrictionTypeRm) -> Self {
			value.clone()
		}
	}

	impl From<RestrictionType> for RestrictionTypeRm {
		fn from(value: RestrictionType) -> Self {
			Self::RestrictionType(value)
		}
	}

	impl From<NullValue> for RestrictionTypeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Unsigned integer representing the "Subscriber Profile ID for
	/// RAT/Frequency Priority"  as specified in 3GPP TS 36.413.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer representing the \"Subscriber Profile
	/// ID for RAT/Frequency Priority\"  as specified in 3GPP TS 36.413.\n",
	///  "type": "integer",
	///  "maximum": 256.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RfspIndex(pub i64);

	impl ::std::ops::Deref for RfspIndex {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<RfspIndex> for i64 {
		fn from(value: RfspIndex) -> Self {
			value.0
		}
	}

	impl From<&RfspIndex> for RfspIndex {
		fn from(value: &RfspIndex) -> Self {
			value.clone()
		}
	}

	impl From<i64> for RfspIndex {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for RfspIndex {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for RfspIndex {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RfspIndex {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RfspIndex {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for RfspIndex {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Unsigned integer representing the 'Subscriber Profile ID for
	/// RAT/Frequency Priority'  as specified in 3GPP TS 36.413 with the OpenAPI
	/// 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer representing the 'Subscriber Profile
	/// ID for RAT/Frequency Priority'  as specified in 3GPP TS 36.413 with the
	/// OpenAPI 'nullable: true' property. \n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 256.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RfspIndexRm(pub Option<i64>);

	impl ::std::ops::Deref for RfspIndexRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<RfspIndexRm> for Option<i64> {
		fn from(value: RfspIndexRm) -> Self {
			value.0
		}
	}

	impl From<&RfspIndexRm> for RfspIndexRm {
		fn from(value: &RfspIndexRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for RfspIndexRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
		}
	}

	/// RgWirelineCharacteristics
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/Bytes"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RgWirelineCharacteristics(pub Bytes);

	impl ::std::ops::Deref for RgWirelineCharacteristics {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<RgWirelineCharacteristics> for Bytes {
		fn from(value: RgWirelineCharacteristics) -> Self {
			value.0
		}
	}

	impl From<&RgWirelineCharacteristics> for RgWirelineCharacteristics {
		fn from(value: &RgWirelineCharacteristics) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for RgWirelineCharacteristics {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for RgWirelineCharacteristics {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for RgWirelineCharacteristics {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RgWirelineCharacteristics {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RgWirelineCharacteristics {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for RgWirelineCharacteristics {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// This data type is defined in the same way as the
	/// 'RgWirelineCharacteristics' data type, but with the OpenAPI 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'RgWirelineCharacteristics' data type, but with the OpenAPI 'nullable:
	/// true' property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/RgWirelineCharacteristics"
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
	pub struct RgWirelineCharacteristicsRm {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<RgWirelineCharacteristics>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<NullValue>,
	}

	impl From<&RgWirelineCharacteristicsRm> for RgWirelineCharacteristicsRm {
		fn from(value: &RgWirelineCharacteristicsRm) -> Self {
			value.clone()
		}
	}

	/// The enumeration RoamingOdb defines the Barring of Roaming as. See 3GPP
	/// TS 23.015 for further description. It shall comply with the provisions
	/// defined in table 5.7.3.1-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration RoamingOdb defines the Barring of
	/// Roaming as. See 3GPP TS 23.015 for further description. It shall comply
	/// with the provisions defined in table 5.7.3.1-1.\n",
	///  "type": "string",
	///  "enum": [
	///    "OUTSIDE_HOME_PLMN",
	///    "OUTSIDE_HOME_PLMN_COUNTRY"
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
	pub enum RoamingOdb {
		#[default]
		#[serde(rename = "OUTSIDE_HOME_PLMN")]
		OutsideHomePlmn,
		#[serde(rename = "OUTSIDE_HOME_PLMN_COUNTRY")]
		OutsideHomePlmnCountry,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RoamingOdb> for RoamingOdb {
		fn from(value: &RoamingOdb) -> Self {
			value.clone()
		}
	}

	impl ToString for RoamingOdb {
		fn to_string(&self) -> String {
			match *self {
				Self::OutsideHomePlmn => "OUTSIDE_HOME_PLMN".to_string(),
				Self::OutsideHomePlmnCountry => "OUTSIDE_HOME_PLMN_COUNTRY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RoamingOdb {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"OUTSIDE_HOME_PLMN" => Ok(Self::OutsideHomePlmn),
				"OUTSIDE_HOME_PLMN_COUNTRY" => Ok(Self::OutsideHomePlmnCountry),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RoamingOdb {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RoamingOdb {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RoamingOdb {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates if access is allowed to a given serving network, e.g. a PLMN
	/// (MCC, MNC) or an  SNPN (MCC, MNC, NID).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates if access is allowed to a given serving
	/// network, e.g. a PLMN (MCC, MNC) or an  SNPN (MCC, MNC, NID).\n",
	///  "type": "object",
	///  "properties": {
	///    "accessAllowed": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RoamingRestrictions {
		#[serde(
			rename = "accessAllowed",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_allowed: Option<bool>,
	}

	impl From<&RoamingRestrictions> for RoamingRestrictions {
		fn from(value: &RoamingRestrictions) -> Self {
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
		NewUnchecked,
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
		NewUnchecked,
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

	/// Contains the network slice status information in terms of the current
	/// number of UEs registered  with a network slice, the current number of
	/// PDU Sessions established on a network slice or both.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the network slice status information in terms
	/// of the current number of UEs registered  with a network slice, the
	/// current number of PDU Sessions established on a network slice or
	/// both.\n",
	///  "type": "object",
	///  "properties": {
	///    "reachedNumPduSess": {
	///      "$ref": "#/components/schemas/SACInfo"
	///    },
	///    "reachedNumUes": {
	///      "$ref": "#/components/schemas/SACInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SacEventStatus {
		#[serde(
			rename = "reachedNumPduSess",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reached_num_pdu_sess: Option<SacInfo>,
		#[serde(
			rename = "reachedNumUes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reached_num_ues: Option<SacInfo>,
	}

	impl From<&SacEventStatus> for SacEventStatus {
		fn from(value: &SacEventStatus) -> Self {
			value.clone()
		}
	}

	/// Represents threshold(s) to control the triggering of network slice
	/// reporting notifications or the information contained in the network
	/// slice reporting notification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents threshold(s) to control the triggering of
	/// network slice reporting notifications or the information contained in
	/// the network slice reporting notification.\n",
	///  "type": "object",
	///  "properties": {
	///    "numericValNumPduSess": {
	///      "type": "integer"
	///    },
	///    "numericValNumUes": {
	///      "type": "integer"
	///    },
	///    "percValueNumPduSess": {
	///      "type": "integer",
	///      "maximum": 100.0,
	///      "minimum": 0.0
	///    },
	///    "percValueNumUes": {
	///      "type": "integer",
	///      "maximum": 100.0,
	///      "minimum": 0.0
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SacInfo {
		#[serde(
			rename = "numericValNumPduSess",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub numeric_val_num_pdu_sess: Option<i64>,
		#[serde(
			rename = "numericValNumUes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub numeric_val_num_ues: Option<i64>,
		#[serde(
			rename = "percValueNumPduSess",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub perc_value_num_pdu_sess: Option<i64>,
		#[serde(
			rename = "percValueNumUes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub perc_value_num_ues: Option<i64>,
	}

	impl From<&SacInfo> for SacInfo {
		fn from(value: &SacInfo) -> Self {
			value.clone()
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

	/// This data type is defined in the same way as the 'SamplingRatio' data
	/// type, but with the  OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'SamplingRatio' data type, but with the  OpenAPI 'nullable: true'
	/// property. \n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 100.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SamplingRatioRm(pub Option<i64>);

	impl ::std::ops::Deref for SamplingRatioRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<SamplingRatioRm> for Option<i64> {
		fn from(value: SamplingRatioRm) -> Self {
			value.0
		}
	}

	impl From<&SamplingRatioRm> for SamplingRatioRm {
		fn from(value: &SamplingRatioRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for SamplingRatioRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
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

	/// Provides information about the satellite backhaul but with the OpenAPI
	/// 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Provides information about the satellite backhaul but
	/// with the OpenAPI 'nullable: true' property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/SatelliteBackhaulCategory"
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
	pub enum SatelliteBackhaulCategoryRm {
		#[default]
		SatelliteBackhaulCategory(SatelliteBackhaulCategory),
		NullValue(NullValue),
	}

	impl From<&SatelliteBackhaulCategoryRm> for SatelliteBackhaulCategoryRm {
		fn from(value: &SatelliteBackhaulCategoryRm) -> Self {
			value.clone()
		}
	}

	impl From<SatelliteBackhaulCategory> for SatelliteBackhaulCategoryRm {
		fn from(value: SatelliteBackhaulCategory) -> Self {
			Self::SatelliteBackhaulCategory(value)
		}
	}

	impl From<NullValue> for SatelliteBackhaulCategoryRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// This data type is defined in the same way as the
	/// 'ScheduledCommunicationTime' data type, but with the OpenAPI 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'ScheduledCommunicationTime' data type, but with the OpenAPI 'nullable:
	/// true' property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/ScheduledCommunicationTime"
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
	pub enum ScheduledCommunicationTimeRm {
		#[default]
		ScheduledCommunicationTime(ScheduledCommunicationTime),
		NullValue(NullValue),
	}

	impl From<&ScheduledCommunicationTimeRm> for ScheduledCommunicationTimeRm {
		fn from(value: &ScheduledCommunicationTimeRm) -> Self {
			value.clone()
		}
	}

	impl From<ScheduledCommunicationTime> for ScheduledCommunicationTimeRm {
		fn from(value: ScheduledCommunicationTime) -> Self {
			Self::ScheduledCommunicationTime(value)
		}
	}

	impl From<NullValue> for ScheduledCommunicationTimeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// This enumeration is defined in the same way as the
	/// 'ScheduledCommunicationTypen' enumeration, but with the OpenAPI
	/// 'nullable: true' property."
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This enumeration is defined in the same way as the
	/// 'ScheduledCommunicationTypen' enumeration, but with the OpenAPI
	/// 'nullable: true' property.\" \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/ScheduledCommunicationType"
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
	pub enum ScheduledCommunicationTypeRm {
		#[default]
		ScheduledCommunicationType(ScheduledCommunicationType),
		NullValue(NullValue),
	}

	impl From<&ScheduledCommunicationTypeRm> for ScheduledCommunicationTypeRm {
		fn from(value: &ScheduledCommunicationTypeRm) -> Self {
			value.clone()
		}
	}

	impl From<ScheduledCommunicationType> for ScheduledCommunicationTypeRm {
		fn from(value: ScheduledCommunicationType) -> Self {
			Self::ScheduledCommunicationType(value)
		}
	}

	impl From<NullValue> for ScheduledCommunicationTypeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// A range of SDs (Slice Differentiators)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A range of SDs (Slice Differentiators)",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "description": "Last value identifying the end of an SD range. This
	/// string shall be formatted as specified for the sd attribute of the
	/// Snssai data type in clause 5.4.4.2.\n",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{6}$"
	///    },
	///    "start": {
	///      "description": "First value identifying the start of an SD range.
	/// This string shall be formatted as specified for the sd attribute of the
	/// Snssai data type in clause 5.4.4.2.\n",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{6}$"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SdRange {
		/// Last value identifying the end of an SD range. This string shall be
		/// formatted as specified for the sd attribute of the Snssai data type
		/// in clause 5.4.4.2.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<SdRangeEnd>,
		/// First value identifying the start of an SD range. This string shall
		/// be formatted as specified for the sd attribute of the Snssai data
		/// type in clause 5.4.4.2.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<SdRangeStart>,
	}

	impl From<&SdRange> for SdRange {
		fn from(value: &SdRange) -> Self {
			value.clone()
		}
	}

	/// Last value identifying the end of an SD range. This string shall be
	/// formatted as specified for the sd attribute of the Snssai data type in
	/// clause 5.4.4.2.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Last value identifying the end of an SD range. This
	/// string shall be formatted as specified for the sd attribute of the
	/// Snssai data type in clause 5.4.4.2.\n",
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
		NewUnchecked,
	)]
	pub struct SdRangeEnd(String);

	impl ::std::ops::Deref for SdRangeEnd {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SdRangeEnd> for String {
		fn from(value: SdRangeEnd) -> Self {
			value.0
		}
	}

	impl From<&SdRangeEnd> for SdRangeEnd {
		fn from(value: &SdRangeEnd) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SdRangeEnd {
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

	impl ::std::convert::TryFrom<&str> for SdRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SdRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SdRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SdRangeEnd {
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

	/// First value identifying the start of an SD range. This string shall be
	/// formatted as specified for the sd attribute of the Snssai data type in
	/// clause 5.4.4.2.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "First value identifying the start of an SD range. This
	/// string shall be formatted as specified for the sd attribute of the
	/// Snssai data type in clause 5.4.4.2.\n",
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
		NewUnchecked,
	)]
	pub struct SdRangeStart(String);

	impl ::std::ops::Deref for SdRangeStart {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SdRangeStart> for String {
		fn from(value: SdRangeStart) -> Self {
			value.0
		}
	}

	impl From<&SdRangeStart> for SdRangeStart {
		fn from(value: &SdRangeStart) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SdRangeStart {
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

	impl ::std::convert::TryFrom<&str> for SdRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SdRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SdRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SdRangeStart {
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

	/// It contains the URI of the linked resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "It contains the URI of the linked resource.",
	///  "type": "object",
	///  "required": [
	///    "self"
	///  ],
	///  "properties": {
	///    "self": {
	///      "$ref": "#/components/schemas/Link"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SelfLink {
		#[serde(rename = "self")]
		pub self_: Link,
	}

	impl From<&SelfLink> for SelfLink {
		fn from(value: &SelfLink) -> Self {
			value.clone()
		}
	}

	/// The enumeration SensorMeasurement defines sensor measurement type for
	/// MDT in the trace. See 3GPP TS 32.422 for further description of the
	/// values. It shall comply with the provisions defined in table 5.6.3.7-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The enumeration SensorMeasurement defines sensor
	/// measurement type for MDT in the trace. See 3GPP TS 32.422 for further
	/// description of the values. It shall comply with the provisions defined
	/// in table 5.6.3.7-1.\n",
	///  "type": "string",
	///  "enum": [
	///    "BAROMETRIC_PRESSURE",
	///    "UE_SPEED",
	///    "UE_ORIENTATION"
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
	pub enum SensorMeasurement {
		#[default]
		#[serde(rename = "BAROMETRIC_PRESSURE")]
		BarometricPressure,
		#[serde(rename = "UE_SPEED")]
		UeSpeed,
		#[serde(rename = "UE_ORIENTATION")]
		UeOrientation,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SensorMeasurement> for SensorMeasurement {
		fn from(value: &SensorMeasurement) -> Self {
			value.clone()
		}
	}

	impl ToString for SensorMeasurement {
		fn to_string(&self) -> String {
			match *self {
				Self::BarometricPressure => "BAROMETRIC_PRESSURE".to_string(),
				Self::UeSpeed => "UE_SPEED".to_string(),
				Self::UeOrientation => "UE_ORIENTATION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SensorMeasurement {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"BAROMETRIC_PRESSURE" => Ok(Self::BarometricPressure),
				"UE_SPEED" => Ok(Self::UeSpeed),
				"UE_ORIENTATION" => Ok(Self::UeOrientation),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SensorMeasurement {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SensorMeasurement {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SensorMeasurement {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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
		NewUnchecked,
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
		NewUnchecked,
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

	/// Provides information about allowed or not allowed areas.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Provides information about allowed or not allowed
	/// areas.",
	///  "type": "object",
	///  "allOf": [
	///    {
	///      "oneOf": [
	///        {
	///          "not": {
	///            "required": [
	///              "restrictionType"
	///            ]
	///          }
	///        },
	///        {
	///          "required": [
	///            "areas"
	///          ]
	///        }
	///      ]
	///    },
	///    {
	///      "anyOf": [
	///        {
	///          "not": {
	///            "required": [
	///              "restrictionType"
	///            ],
	///            "properties": {
	///              "restrictionType": {
	///                "type": "string",
	///                "enum": [
	///                  "NOT_ALLOWED_AREAS"
	///                ]
	///              }
	///            }
	///          }
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "maxNumOfTAs"
	///            ]
	///          }
	///        }
	///      ]
	///    },
	///    {
	///      "anyOf": [
	///        {
	///          "not": {
	///            "required": [
	///              "restrictionType"
	///            ],
	///            "properties": {
	///              "restrictionType": {
	///                "type": "string",
	///                "enum": [
	///                  "ALLOWED_AREAS"
	///                ]
	///              }
	///            }
	///          }
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "maxNumOfTAsForNotAllowedAreas"
	///            ]
	///          }
	///        }
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "areas": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Area"
	///      }
	///    },
	///    "maxNumOfTAs": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "maxNumOfTAsForNotAllowedAreas": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "restrictionType": {
	///      "$ref": "#/components/schemas/RestrictionType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum ServiceAreaRestriction {
		#[default]
		Variant0(ServiceAreaRestrictionVariant0),
		Variant1(ServiceAreaRestrictionVariant1),
	}

	impl From<&ServiceAreaRestriction> for ServiceAreaRestriction {
		fn from(value: &ServiceAreaRestriction) -> Self {
			value.clone()
		}
	}

	impl From<ServiceAreaRestrictionVariant0> for ServiceAreaRestriction {
		fn from(value: ServiceAreaRestrictionVariant0) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<ServiceAreaRestrictionVariant1> for ServiceAreaRestriction {
		fn from(value: ServiceAreaRestrictionVariant1) -> Self {
			Self::Variant1(value)
		}
	}

	/// ServiceAreaRestrictionVariant0
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "anyOf": [
	///        {
	///          "allOf": [
	///            {},
	///            {
	///              "allOf": [
	///                {
	///                  "oneOf": [
	///                    {
	///                      "allOf": [
	///                        {},
	///                        {
	///                          "allOf": [
	///                            {
	///                              "type": "object",
	///                              "properties": {
	///                                "areas": {
	///                                  "type": "array",
	///                                  "items": {
	///                                    "$ref": "#/components/schemas/Area"
	///                                  }
	///                                },
	///                                "maxNumOfTAs": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "maxNumOfTAsForNotAllowedAreas": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "restrictionType": {
	///                                  "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                }
	///                              }
	///                            },
	///                            {
	///                              "not": {
	///                                "required": [
	///                                  "restrictionType"
	///                                ]
	///                              }
	///                            },
	///                            {
	///                              "not": {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              }
	///                            }
	///                          ]
	///                        },
	///                        {
	///                          "not": {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              },
	///                              {
	///                                "not": {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                }
	///                              }
	///                            ]
	///                          }
	///                        }
	///                      ]
	///                    },
	///                    {
	///                      "allOf": [
	///                        {},
	///                        {
	///                          "allOf": [
	///                            {
	///                              "type": "object",
	///                              "properties": {
	///                                "areas": {
	///                                  "type": "array",
	///                                  "items": {
	///                                    "$ref": "#/components/schemas/Area"
	///                                  }
	///                                },
	///                                "maxNumOfTAs": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "maxNumOfTAsForNotAllowedAreas": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "restrictionType": {
	///                                  "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                }
	///                              }
	///                            },
	///                            {
	///                              "required": [
	///                                "areas"
	///                              ]
	///                            },
	///                            {
	///                              "not": {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              }
	///                            }
	///                          ]
	///                        },
	///                        {
	///                          "not": {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                }
	///                              }
	///                            ]
	///                          }
	///                        }
	///                      ]
	///                    }
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "restrictionType"
	///                    ],
	///                    "properties": {
	///                      "restrictionType": {
	///                        "type": "string",
	///                        "enum": [
	///                          "NOT_ALLOWED_AREAS"
	///                        ]
	///                      }
	///                    }
	///                  }
	///                },
	///                {
	///                  "not": {
	///                    "not": {
	///                      "required": [
	///                        "maxNumOfTAs"
	///                      ]
	///                    }
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "oneOf": [
	///                      {
	///                        "allOf": [
	///                          {},
	///                          {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                }
	///                              }
	///                            ]
	///                          },
	///                          {
	///                            "not": {
	///                              "allOf": [
	///                                {
	///                                  "type": "object",
	///                                  "properties": {
	///                                    "areas": {
	///                                      "type": "array",
	///                                      "items": {
	///                                        "$ref":
	/// "#/components/schemas/Area"
	///                                      }
	///                                    },
	///                                    "maxNumOfTAs": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "maxNumOfTAsForNotAllowedAreas": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "restrictionType": {
	///                                      "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                    }
	///                                  }
	///                                },
	///                                {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                },
	///                                {
	///                                  "not": {
	///                                    "not": {
	///                                      "required": [
	///                                        "restrictionType"
	///                                      ]
	///                                    }
	///                                  }
	///                                }
	///                              ]
	///                            }
	///                          }
	///                        ]
	///                      },
	///                      {
	///                        "allOf": [
	///                          {},
	///                          {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              },
	///                              {
	///                                "not": {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                }
	///                              }
	///                            ]
	///                          },
	///                          {
	///                            "not": {
	///                              "allOf": [
	///                                {
	///                                  "type": "object",
	///                                  "properties": {
	///                                    "areas": {
	///                                      "type": "array",
	///                                      "items": {
	///                                        "$ref":
	/// "#/components/schemas/Area"
	///                                      }
	///                                    },
	///                                    "maxNumOfTAs": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "maxNumOfTAsForNotAllowedAreas": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "restrictionType": {
	///                                      "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                    }
	///                                  }
	///                                },
	///                                {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                },
	///                                {
	///                                  "not": {
	///                                    "required": [
	///                                      "areas"
	///                                    ]
	///                                  }
	///                                }
	///                              ]
	///                            }
	///                          }
	///                        ]
	///                      }
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "maxNumOfTAs"
	///                      ]
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "not": {
	///                        "required": [
	///                          "restrictionType"
	///                        ],
	///                        "properties": {
	///                          "restrictionType": {
	///                            "type": "string",
	///                            "enum": [
	///                              "NOT_ALLOWED_AREAS"
	///                            ]
	///                          }
	///                        }
	///                      }
	///                    }
	///                  }
	///                ]
	///              }
	///            }
	///          ]
	///        },
	///        {
	///          "allOf": [
	///            {},
	///            {
	///              "allOf": [
	///                {
	///                  "oneOf": [
	///                    {
	///                      "allOf": [
	///                        {},
	///                        {
	///                          "allOf": [
	///                            {
	///                              "type": "object",
	///                              "properties": {
	///                                "areas": {
	///                                  "type": "array",
	///                                  "items": {
	///                                    "$ref": "#/components/schemas/Area"
	///                                  }
	///                                },
	///                                "maxNumOfTAs": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "maxNumOfTAsForNotAllowedAreas": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "restrictionType": {
	///                                  "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                }
	///                              }
	///                            },
	///                            {
	///                              "not": {
	///                                "required": [
	///                                  "restrictionType"
	///                                ]
	///                              }
	///                            },
	///                            {
	///                              "not": {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              }
	///                            }
	///                          ]
	///                        },
	///                        {
	///                          "not": {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              },
	///                              {
	///                                "not": {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                }
	///                              }
	///                            ]
	///                          }
	///                        }
	///                      ]
	///                    },
	///                    {
	///                      "allOf": [
	///                        {},
	///                        {
	///                          "allOf": [
	///                            {
	///                              "type": "object",
	///                              "properties": {
	///                                "areas": {
	///                                  "type": "array",
	///                                  "items": {
	///                                    "$ref": "#/components/schemas/Area"
	///                                  }
	///                                },
	///                                "maxNumOfTAs": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "maxNumOfTAsForNotAllowedAreas": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "restrictionType": {
	///                                  "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                }
	///                              }
	///                            },
	///                            {
	///                              "required": [
	///                                "areas"
	///                              ]
	///                            },
	///                            {
	///                              "not": {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              }
	///                            }
	///                          ]
	///                        },
	///                        {
	///                          "not": {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                }
	///                              }
	///                            ]
	///                          }
	///                        }
	///                      ]
	///                    }
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "maxNumOfTAs"
	///                    ]
	///                  }
	///                },
	///                {
	///                  "not": {
	///                    "not": {
	///                      "required": [
	///                        "restrictionType"
	///                      ],
	///                      "properties": {
	///                        "restrictionType": {
	///                          "type": "string",
	///                          "enum": [
	///                            "NOT_ALLOWED_AREAS"
	///                          ]
	///                        }
	///                      }
	///                    }
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "oneOf": [
	///                      {
	///                        "allOf": [
	///                          {},
	///                          {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                }
	///                              }
	///                            ]
	///                          },
	///                          {
	///                            "not": {
	///                              "allOf": [
	///                                {
	///                                  "type": "object",
	///                                  "properties": {
	///                                    "areas": {
	///                                      "type": "array",
	///                                      "items": {
	///                                        "$ref":
	/// "#/components/schemas/Area"
	///                                      }
	///                                    },
	///                                    "maxNumOfTAs": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "maxNumOfTAsForNotAllowedAreas": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "restrictionType": {
	///                                      "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                    }
	///                                  }
	///                                },
	///                                {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                },
	///                                {
	///                                  "not": {
	///                                    "not": {
	///                                      "required": [
	///                                        "restrictionType"
	///                                      ]
	///                                    }
	///                                  }
	///                                }
	///                              ]
	///                            }
	///                          }
	///                        ]
	///                      },
	///                      {
	///                        "allOf": [
	///                          {},
	///                          {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              },
	///                              {
	///                                "not": {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                }
	///                              }
	///                            ]
	///                          },
	///                          {
	///                            "not": {
	///                              "allOf": [
	///                                {
	///                                  "type": "object",
	///                                  "properties": {
	///                                    "areas": {
	///                                      "type": "array",
	///                                      "items": {
	///                                        "$ref":
	/// "#/components/schemas/Area"
	///                                      }
	///                                    },
	///                                    "maxNumOfTAs": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "maxNumOfTAsForNotAllowedAreas": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "restrictionType": {
	///                                      "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                    }
	///                                  }
	///                                },
	///                                {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                },
	///                                {
	///                                  "not": {
	///                                    "required": [
	///                                      "areas"
	///                                    ]
	///                                  }
	///                                }
	///                              ]
	///                            }
	///                          }
	///                        ]
	///                      }
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "restrictionType"
	///                      ],
	///                      "properties": {
	///                        "restrictionType": {
	///                          "type": "string",
	///                          "enum": [
	///                            "NOT_ALLOWED_AREAS"
	///                          ]
	///                        }
	///                      }
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "not": {
	///                        "required": [
	///                          "maxNumOfTAs"
	///                        ]
	///                      }
	///                    }
	///                  }
	///                ]
	///              }
	///            }
	///          ]
	///        }
	///      ]
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "restrictionType"
	///        ],
	///        "properties": {
	///          "restrictionType": {
	///            "type": "string",
	///            "enum": [
	///              "ALLOWED_AREAS"
	///            ]
	///          }
	///        }
	///      }
	///    },
	///    {
	///      "not": {
	///        "not": {
	///          "required": [
	///            "maxNumOfTAsForNotAllowedAreas"
	///          ]
	///        }
	///      }
	///    }
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
	#[serde(deny_unknown_fields)]
	pub enum ServiceAreaRestrictionVariant0 {
		#[default]
		None,
	}

	impl From<&ServiceAreaRestrictionVariant0> for ServiceAreaRestrictionVariant0 {
		fn from(value: &ServiceAreaRestrictionVariant0) -> Self {
			value.clone()
		}
	}

	/// ServiceAreaRestrictionVariant1
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "anyOf": [
	///        {
	///          "allOf": [
	///            {},
	///            {
	///              "allOf": [
	///                {
	///                  "oneOf": [
	///                    {
	///                      "allOf": [
	///                        {},
	///                        {
	///                          "allOf": [
	///                            {
	///                              "type": "object",
	///                              "properties": {
	///                                "areas": {
	///                                  "type": "array",
	///                                  "items": {
	///                                    "$ref": "#/components/schemas/Area"
	///                                  }
	///                                },
	///                                "maxNumOfTAs": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "maxNumOfTAsForNotAllowedAreas": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "restrictionType": {
	///                                  "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                }
	///                              }
	///                            },
	///                            {
	///                              "not": {
	///                                "required": [
	///                                  "restrictionType"
	///                                ]
	///                              }
	///                            },
	///                            {
	///                              "not": {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              }
	///                            }
	///                          ]
	///                        },
	///                        {
	///                          "not": {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              },
	///                              {
	///                                "not": {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                }
	///                              }
	///                            ]
	///                          }
	///                        }
	///                      ]
	///                    },
	///                    {
	///                      "allOf": [
	///                        {},
	///                        {
	///                          "allOf": [
	///                            {
	///                              "type": "object",
	///                              "properties": {
	///                                "areas": {
	///                                  "type": "array",
	///                                  "items": {
	///                                    "$ref": "#/components/schemas/Area"
	///                                  }
	///                                },
	///                                "maxNumOfTAs": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "maxNumOfTAsForNotAllowedAreas": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "restrictionType": {
	///                                  "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                }
	///                              }
	///                            },
	///                            {
	///                              "required": [
	///                                "areas"
	///                              ]
	///                            },
	///                            {
	///                              "not": {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              }
	///                            }
	///                          ]
	///                        },
	///                        {
	///                          "not": {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                }
	///                              }
	///                            ]
	///                          }
	///                        }
	///                      ]
	///                    }
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "restrictionType"
	///                    ],
	///                    "properties": {
	///                      "restrictionType": {
	///                        "type": "string",
	///                        "enum": [
	///                          "NOT_ALLOWED_AREAS"
	///                        ]
	///                      }
	///                    }
	///                  }
	///                },
	///                {
	///                  "not": {
	///                    "not": {
	///                      "required": [
	///                        "maxNumOfTAs"
	///                      ]
	///                    }
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "oneOf": [
	///                      {
	///                        "allOf": [
	///                          {},
	///                          {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                }
	///                              }
	///                            ]
	///                          },
	///                          {
	///                            "not": {
	///                              "allOf": [
	///                                {
	///                                  "type": "object",
	///                                  "properties": {
	///                                    "areas": {
	///                                      "type": "array",
	///                                      "items": {
	///                                        "$ref":
	/// "#/components/schemas/Area"
	///                                      }
	///                                    },
	///                                    "maxNumOfTAs": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "maxNumOfTAsForNotAllowedAreas": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "restrictionType": {
	///                                      "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                    }
	///                                  }
	///                                },
	///                                {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                },
	///                                {
	///                                  "not": {
	///                                    "not": {
	///                                      "required": [
	///                                        "restrictionType"
	///                                      ]
	///                                    }
	///                                  }
	///                                }
	///                              ]
	///                            }
	///                          }
	///                        ]
	///                      },
	///                      {
	///                        "allOf": [
	///                          {},
	///                          {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              },
	///                              {
	///                                "not": {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                }
	///                              }
	///                            ]
	///                          },
	///                          {
	///                            "not": {
	///                              "allOf": [
	///                                {
	///                                  "type": "object",
	///                                  "properties": {
	///                                    "areas": {
	///                                      "type": "array",
	///                                      "items": {
	///                                        "$ref":
	/// "#/components/schemas/Area"
	///                                      }
	///                                    },
	///                                    "maxNumOfTAs": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "maxNumOfTAsForNotAllowedAreas": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "restrictionType": {
	///                                      "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                    }
	///                                  }
	///                                },
	///                                {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                },
	///                                {
	///                                  "not": {
	///                                    "required": [
	///                                      "areas"
	///                                    ]
	///                                  }
	///                                }
	///                              ]
	///                            }
	///                          }
	///                        ]
	///                      }
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "maxNumOfTAs"
	///                      ]
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "not": {
	///                        "required": [
	///                          "restrictionType"
	///                        ],
	///                        "properties": {
	///                          "restrictionType": {
	///                            "type": "string",
	///                            "enum": [
	///                              "NOT_ALLOWED_AREAS"
	///                            ]
	///                          }
	///                        }
	///                      }
	///                    }
	///                  }
	///                ]
	///              }
	///            }
	///          ]
	///        },
	///        {
	///          "allOf": [
	///            {},
	///            {
	///              "allOf": [
	///                {
	///                  "oneOf": [
	///                    {
	///                      "allOf": [
	///                        {},
	///                        {
	///                          "allOf": [
	///                            {
	///                              "type": "object",
	///                              "properties": {
	///                                "areas": {
	///                                  "type": "array",
	///                                  "items": {
	///                                    "$ref": "#/components/schemas/Area"
	///                                  }
	///                                },
	///                                "maxNumOfTAs": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "maxNumOfTAsForNotAllowedAreas": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "restrictionType": {
	///                                  "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                }
	///                              }
	///                            },
	///                            {
	///                              "not": {
	///                                "required": [
	///                                  "restrictionType"
	///                                ]
	///                              }
	///                            },
	///                            {
	///                              "not": {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              }
	///                            }
	///                          ]
	///                        },
	///                        {
	///                          "not": {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              },
	///                              {
	///                                "not": {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                }
	///                              }
	///                            ]
	///                          }
	///                        }
	///                      ]
	///                    },
	///                    {
	///                      "allOf": [
	///                        {},
	///                        {
	///                          "allOf": [
	///                            {
	///                              "type": "object",
	///                              "properties": {
	///                                "areas": {
	///                                  "type": "array",
	///                                  "items": {
	///                                    "$ref": "#/components/schemas/Area"
	///                                  }
	///                                },
	///                                "maxNumOfTAs": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "maxNumOfTAsForNotAllowedAreas": {
	///                                  "$ref": "#/components/schemas/Uinteger"
	///                                },
	///                                "restrictionType": {
	///                                  "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                }
	///                              }
	///                            },
	///                            {
	///                              "required": [
	///                                "areas"
	///                              ]
	///                            },
	///                            {
	///                              "not": {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              }
	///                            }
	///                          ]
	///                        },
	///                        {
	///                          "not": {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                }
	///                              }
	///                            ]
	///                          }
	///                        }
	///                      ]
	///                    }
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "maxNumOfTAs"
	///                    ]
	///                  }
	///                },
	///                {
	///                  "not": {
	///                    "not": {
	///                      "required": [
	///                        "restrictionType"
	///                      ],
	///                      "properties": {
	///                        "restrictionType": {
	///                          "type": "string",
	///                          "enum": [
	///                            "NOT_ALLOWED_AREAS"
	///                          ]
	///                        }
	///                      }
	///                    }
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "oneOf": [
	///                      {
	///                        "allOf": [
	///                          {},
	///                          {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "restrictionType"
	///                                  ]
	///                                }
	///                              },
	///                              {
	///                                "not": {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                }
	///                              }
	///                            ]
	///                          },
	///                          {
	///                            "not": {
	///                              "allOf": [
	///                                {
	///                                  "type": "object",
	///                                  "properties": {
	///                                    "areas": {
	///                                      "type": "array",
	///                                      "items": {
	///                                        "$ref":
	/// "#/components/schemas/Area"
	///                                      }
	///                                    },
	///                                    "maxNumOfTAs": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "maxNumOfTAsForNotAllowedAreas": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "restrictionType": {
	///                                      "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                    }
	///                                  }
	///                                },
	///                                {
	///                                  "required": [
	///                                    "areas"
	///                                  ]
	///                                },
	///                                {
	///                                  "not": {
	///                                    "not": {
	///                                      "required": [
	///                                        "restrictionType"
	///                                      ]
	///                                    }
	///                                  }
	///                                }
	///                              ]
	///                            }
	///                          }
	///                        ]
	///                      },
	///                      {
	///                        "allOf": [
	///                          {},
	///                          {
	///                            "allOf": [
	///                              {
	///                                "type": "object",
	///                                "properties": {
	///                                  "areas": {
	///                                    "type": "array",
	///                                    "items": {
	///                                      "$ref": "#/components/schemas/Area"
	///                                    }
	///                                  },
	///                                  "maxNumOfTAs": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "maxNumOfTAsForNotAllowedAreas": {
	///                                    "$ref":
	/// "#/components/schemas/Uinteger"
	///                                  },
	///                                  "restrictionType": {
	///                                    "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                  }
	///                                }
	///                              },
	///                              {
	///                                "required": [
	///                                  "areas"
	///                                ]
	///                              },
	///                              {
	///                                "not": {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                }
	///                              }
	///                            ]
	///                          },
	///                          {
	///                            "not": {
	///                              "allOf": [
	///                                {
	///                                  "type": "object",
	///                                  "properties": {
	///                                    "areas": {
	///                                      "type": "array",
	///                                      "items": {
	///                                        "$ref":
	/// "#/components/schemas/Area"
	///                                      }
	///                                    },
	///                                    "maxNumOfTAs": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "maxNumOfTAsForNotAllowedAreas": {
	///                                      "$ref":
	/// "#/components/schemas/Uinteger"
	///                                    },
	///                                    "restrictionType": {
	///                                      "$ref":
	/// "#/components/schemas/RestrictionType"
	///                                    }
	///                                  }
	///                                },
	///                                {
	///                                  "not": {
	///                                    "required": [
	///                                      "restrictionType"
	///                                    ]
	///                                  }
	///                                },
	///                                {
	///                                  "not": {
	///                                    "required": [
	///                                      "areas"
	///                                    ]
	///                                  }
	///                                }
	///                              ]
	///                            }
	///                          }
	///                        ]
	///                      }
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "restrictionType"
	///                      ],
	///                      "properties": {
	///                        "restrictionType": {
	///                          "type": "string",
	///                          "enum": [
	///                            "NOT_ALLOWED_AREAS"
	///                          ]
	///                        }
	///                      }
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "not": {
	///                        "required": [
	///                          "maxNumOfTAs"
	///                        ]
	///                      }
	///                    }
	///                  }
	///                ]
	///              }
	///            }
	///          ]
	///        }
	///      ]
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "maxNumOfTAsForNotAllowedAreas"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "not": {
	///          "required": [
	///            "restrictionType"
	///          ],
	///          "properties": {
	///            "restrictionType": {
	///              "type": "string",
	///              "enum": [
	///                "ALLOWED_AREAS"
	///              ]
	///            }
	///          }
	///        }
	///      }
	///    }
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
	#[serde(deny_unknown_fields)]
	pub enum ServiceAreaRestrictionVariant1 {
		#[default]
		None,
	}

	impl From<&ServiceAreaRestrictionVariant1> for ServiceAreaRestrictionVariant1 {
		fn from(value: &ServiceAreaRestrictionVariant1) -> Self {
			value.clone()
		}
	}

	/// ServiceId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/Uint32"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ServiceId(pub Uint32);

	impl ::std::ops::Deref for ServiceId {
		type Target = Uint32;
		fn deref(&self) -> &Uint32 {
			&self.0
		}
	}

	impl From<ServiceId> for Uint32 {
		fn from(value: ServiceId) -> Self {
			value.0
		}
	}

	impl From<&ServiceId> for ServiceId {
		fn from(value: &ServiceId) -> Self {
			value.clone()
		}
	}

	impl From<Uint32> for ServiceId {
		fn from(value: Uint32) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ServiceId {
		type Err = <Uint32 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ServiceId {
		type Error = <Uint32 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ServiceId {
		type Error = <Uint32 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ServiceId {
		type Error = <Uint32 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ServiceId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// MBR related to slice
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBR related to slice",
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
	pub struct SliceMbr {
		pub downlink: BitRate,
		pub uplink: BitRate,
	}

	impl From<&SliceMbr> for SliceMbr {
		fn from(value: &SliceMbr) -> Self {
			value.clone()
		}
	}

	/// SliceMbr with nullable: true
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "SliceMbr with nullable: true",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/SliceMbr"
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
	pub enum SliceMbrRm {
		#[default]
		SliceMbr(SliceMbr),
		NullValue(NullValue),
	}

	impl From<&SliceMbrRm> for SliceMbrRm {
		fn from(value: &SliceMbrRm) -> Self {
			value.clone()
		}
	}

	impl From<SliceMbr> for SliceMbrRm {
		fn from(value: SliceMbr) -> Self {
			Self::SliceMbr(value)
		}
	}

	impl From<NullValue> for SliceMbrRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// Extensions to the Snssai data type, sdRanges and wildcardSd shall not be
	/// present simultaneously
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Extensions to the Snssai data type, sdRanges and
	/// wildcardSd shall not be present simultaneously\n",
	///  "type": "object",
	///  "not": {
	///    "required": [
	///      "sdRanges",
	///      "wildcardSd"
	///    ]
	///  },
	///  "properties": {
	///    "sdRanges": {
	///      "description": "When present, it shall contain the range(s) of
	/// Slice Differentiator values supported for the Slice/Service Type value
	/// indicated in the sst attribute of the Snssai data type\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SdRange"
	///      },
	///      "minItems": 1
	///    },
	///    "wildcardSd": {
	///      "description": "When present, it shall be set to true, to indicate
	/// that all SD values are supported for the Slice/Service Type value
	/// indicated in the sst attribute of the Snssai data type.\n",
	///      "type": "boolean",
	///      "enum": [
	///        true
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SnssaiExtension {}

	impl From<&SnssaiExtension> for SnssaiExtension {
		fn from(value: &SnssaiExtension) -> Self {
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
		NewUnchecked,
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

	/// Contains the Spatial Validity Condition.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Spatial Validity Condition.",
	///  "type": "object",
	///  "properties": {
	///    "countries": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Mcc"
	///      },
	///      "minItems": 1
	///    },
	///    "geographicalServiceArea": {
	///      "$ref": "#/components/schemas/GeoServiceArea"
	///    },
	///    "trackingAreaList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
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
	pub struct SpatialValidityCond {
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub countries: Vec<Mcc>,
		#[serde(
			rename = "geographicalServiceArea",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub geographical_service_area: Option<GeoServiceArea>,
		#[serde(
			rename = "trackingAreaList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tracking_area_list: Vec<Tai>,
	}

	impl From<&SpatialValidityCond> for SpatialValidityCond {
		fn from(value: &SpatialValidityCond) -> Self {
			value.clone()
		}
	}

	/// Contains the Spatial Validity Condition or the null value.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Spatial Validity Condition or the null
	/// value.",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/SpatialValidityCond"
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
	pub enum SpatialValidityCondRm {
		#[default]
		SpatialValidityCond(SpatialValidityCond),
		NullValue(NullValue),
	}

	impl From<&SpatialValidityCondRm> for SpatialValidityCondRm {
		fn from(value: &SpatialValidityCondRm) -> Self {
			value.clone()
		}
	}

	impl From<SpatialValidityCond> for SpatialValidityCondRm {
		fn from(value: SpatialValidityCond) -> Self {
			Self::SpatialValidityCond(value)
		}
	}

	impl From<NullValue> for SpatialValidityCondRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// represents the service and session continuity mode It shall comply with
	/// the provisions defined in table 5.4.3.6-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "represents the service and session continuity mode It
	/// shall comply with the provisions defined in table 5.4.3.6-1. \n",
	///  "type": "string",
	///  "enum": [
	///    "SSC_MODE_1",
	///    "SSC_MODE_2",
	///    "SSC_MODE_3"
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
	pub enum SscMode {
		#[default]
		#[serde(rename = "SSC_MODE_1")]
		SscMode1,
		#[serde(rename = "SSC_MODE_2")]
		SscMode2,
		#[serde(rename = "SSC_MODE_3")]
		SscMode3,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SscMode> for SscMode {
		fn from(value: &SscMode) -> Self {
			value.clone()
		}
	}

	impl ToString for SscMode {
		fn to_string(&self) -> String {
			match *self {
				Self::SscMode1 => "SSC_MODE_1".to_string(),
				Self::SscMode2 => "SSC_MODE_2".to_string(),
				Self::SscMode3 => "SSC_MODE_3".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SscMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SSC_MODE_1" => Ok(Self::SscMode1),
				"SSC_MODE_2" => Ok(Self::SscMode2),
				"SSC_MODE_3" => Ok(Self::SscMode3),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SscMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// represents the service and session continuity mode It shall comply with
	/// the provisions defined in table 5.4.3.6-1 but with the OpenAPI
	/// 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "represents the service and session continuity mode It
	/// shall comply with the provisions defined in table 5.4.3.6-1 but with the
	/// OpenAPI 'nullable: true' property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/SscMode"
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
	pub enum SscModeRm {
		#[default]
		SscMode(SscMode),
		NullValue(NullValue),
	}

	impl From<&SscModeRm> for SscModeRm {
		fn from(value: &SscModeRm) -> Self {
			value.clone()
		}
	}

	impl From<SscMode> for SscModeRm {
		fn from(value: SscMode) -> Self {
			Self::SscMode(value)
		}
	}

	impl From<NullValue> for SscModeRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Source specific IP multicast address
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Source specific IP multicast address",
	///  "type": "object",
	///  "required": [
	///    "destIpAddr",
	///    "sourceIpAddr"
	///  ],
	///  "properties": {
	///    "destIpAddr": {
	///      "$ref": "#/components/schemas/IpAddr"
	///    },
	///    "sourceIpAddr": {
	///      "$ref": "#/components/schemas/IpAddr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Ssm {
		#[serde(rename = "destIpAddr")]
		pub dest_ip_addr: IpAddr,
		#[serde(rename = "sourceIpAddr")]
		pub source_ip_addr: IpAddr,
	}

	impl From<&Ssm> for Ssm {
		fn from(value: &Ssm) -> Self {
			value.clone()
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

	/// This enumeration is defined in the same way as the
	/// 'StationaryIndication' enumeration, but with the OpenAPI 'nullable:
	/// true' property."
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This enumeration is defined in the same way as the
	/// 'StationaryIndication' enumeration, but with the OpenAPI 'nullable:
	/// true' property.\"\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/StationaryIndication"
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
	pub enum StationaryIndicationRm {
		#[default]
		StationaryIndication(StationaryIndication),
		NullValue(NullValue),
	}

	impl From<&StationaryIndicationRm> for StationaryIndicationRm {
		fn from(value: &StationaryIndicationRm) -> Self {
			value.clone()
		}
	}

	impl From<StationaryIndication> for StationaryIndicationRm {
		fn from(value: StationaryIndication) -> Self {
			Self::StationaryIndication(value)
		}
	}

	impl From<NullValue> for StationaryIndicationRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// String representing the STN-SR as defined in clause 18.6 of 3GPP TS
	/// 23.003.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing the STN-SR as defined in clause
	/// 18.6 of 3GPP TS 23.003.",
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
	pub struct StnSr(pub String);

	impl ::std::ops::Deref for StnSr {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<StnSr> for String {
		fn from(value: StnSr) -> Self {
			value.0
		}
	}

	impl From<&StnSr> for StnSr {
		fn from(value: &StnSr) -> Self {
			value.clone()
		}
	}

	impl From<String> for StnSr {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for StnSr {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for StnSr {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// String representing the STN-SR as defined in clause 18.6 of 3GPP TS
	/// 23.003 with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing the STN-SR as defined in clause 18.6 of 3GPP TS 23.003 with the OpenAPI 'nullable: true' property. \n",
	///  "type": [
	///    "string",
	///    "null"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct StnSrRm(pub Option<String>);

	impl ::std::ops::Deref for StnSrRm {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<StnSrRm> for Option<String> {
		fn from(value: StnSrRm) -> Self {
			value.0
		}
	}

	impl From<&StnSrRm> for StnSrRm {
		fn from(value: &StnSrRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for StnSrRm {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// A String with Matching Operator
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A String with Matching Operator",
	///  "type": "object",
	///  "required": [
	///    "matchingOperator"
	///  ],
	///  "properties": {
	///    "matchingOperator": {
	///      "$ref": "#/components/schemas/MatchingOperator"
	///    },
	///    "matchingString": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct StringMatchingCondition {
		#[serde(rename = "matchingOperator")]
		pub matching_operator: MatchingOperator,
		#[serde(
			rename = "matchingString",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub matching_string: Option<String>,
	}

	impl From<&StringMatchingCondition> for StringMatchingCondition {
		fn from(value: &StringMatchingCondition) -> Self {
			value.clone()
		}
	}

	/// A list of conditions for string matching
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A list of conditions for string matching",
	///  "type": "object",
	///  "properties": {
	///    "stringMatchingConditions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/StringMatchingCondition"
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
	pub struct StringMatchingRule {
		#[serde(
			rename = "stringMatchingConditions",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub string_matching_conditions: Vec<StringMatchingCondition>,
	}

	impl From<&StringMatchingRule> for StringMatchingRule {
		fn from(value: &StringMatchingRule) -> Self {
			value.clone()
		}
	}

	/// Provides the subsribed 5QI and the ARP, it may contain the priority
	/// level.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Provides the subsribed 5QI and the ARP, it may contain
	/// the priority level.",
	///  "type": "object",
	///  "required": [
	///    "5qi",
	///    "arp"
	///  ],
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "arp": {
	///      "$ref": "#/components/schemas/Arp"
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
	pub struct SubscribedDefaultQos {
		pub arp: Arp,
		#[serde(rename = "5qi")]
		pub five_qi: _5qi,
		#[serde(
			rename = "priorityLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub priority_level: Option<_5qiPriorityLevel>,
	}

	impl From<&SubscribedDefaultQos> for SubscribedDefaultQos {
		fn from(value: &SubscribedDefaultQos) -> Self {
			value.clone()
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
		NewUnchecked,
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

	/// String identifying a SUPI or a SUCI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String identifying a SUPI or a SUCI.",
	///  "type": "string",
	///  "pattern":
	/// "^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|suci-(0-[0-9]{3}-[0-9]{2,
	/// 3}|[1-7]-.+)-[0-9]{1,4}-(0-0-.*
	/// |[a-fA-F1-9]-([1-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])-[a-fA-F0-9]+)|.
	/// +)$"
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
	pub struct SupiOrSuci(String);

	impl ::std::ops::Deref for SupiOrSuci {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SupiOrSuci> for String {
		fn from(value: SupiOrSuci) -> Self {
			value.0
		}
	}

	impl From<&SupiOrSuci> for SupiOrSuci {
		fn from(value: &SupiOrSuci) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SupiOrSuci {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|suci-(0-[0-9]{3}-[0-9]{2,3}|[1-7]-.\
				 +)-[0-9]{1,4}-(0-0-.*\
				 |[a-fA-F1-9]-([1-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])-[a-fA-F0-9]+)|.+)$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^(imsi-[0-9]{5,15}|nai-.+|gli-.+|gci-.+|suci-(0-[0-9]{3}-[0-9]{2,\
				            3}|[1-7]-.+)-[0-9]{1,4}-(0-0-.*\
				            |[a-fA-F1-9]-([1-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])-[a-fA-F0-9]+)|.\
				            +)$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SupiOrSuci {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SupiOrSuci {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SupiOrSuci {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SupiOrSuci {
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

	/// This data type is defined in the same way as the 'Supi' data type, but
	/// with the  OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Supi'
	/// data type, but with the  OpenAPI 'nullable: true' property. \n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^(imsi-[0-9]{5,15}|nai-.+|gci-.+|gli-.+|.+)$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SupiRm(pub Option<SupiRmInner>);

	impl ::std::ops::Deref for SupiRm {
		type Target = Option<SupiRmInner>;
		fn deref(&self) -> &Option<SupiRmInner> {
			&self.0
		}
	}

	impl From<SupiRm> for Option<SupiRmInner> {
		fn from(value: SupiRm) -> Self {
			value.0
		}
	}

	impl From<&SupiRm> for SupiRm {
		fn from(value: &SupiRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<SupiRmInner>> for SupiRm {
		fn from(value: Option<SupiRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the 'Supi' data type, but
	/// with the  OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Supi'
	/// data type, but with the  OpenAPI 'nullable: true' property. \n",
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
		NewUnchecked,
	)]
	pub struct SupiRmInner(String);

	impl ::std::ops::Deref for SupiRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SupiRmInner> for String {
		fn from(value: SupiRmInner) -> Self {
			value.0
		}
	}

	impl From<&SupiRmInner> for SupiRmInner {
		fn from(value: &SupiRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SupiRmInner {
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

	impl ::std::convert::TryFrom<&str> for SupiRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SupiRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SupiRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SupiRmInner {
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
		NewUnchecked,
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

	/// Indicates supported GAD shapes.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates supported GAD shapes.",
	///  "type": "string",
	///  "enum": [
	///    "POINT",
	///    "POINT_UNCERTAINTY_CIRCLE",
	///    "POINT_UNCERTAINTY_ELLIPSE",
	///    "POLYGON",
	///    "POINT_ALTITUDE",
	///    "POINT_ALTITUDE_UNCERTAINTY",
	///    "ELLIPSOID_ARC",
	///    "LOCAL_2D_POINT_UNCERTAINTY_ELLIPSE",
	///    "LOCAL_3D_POINT_UNCERTAINTY_ELLIPSOID"
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
	pub enum SupportedGadShapes {
		#[default]
		#[serde(rename = "POINT")]
		Point,
		#[serde(rename = "POINT_UNCERTAINTY_CIRCLE")]
		PointUncertaintyCircle,
		#[serde(rename = "POINT_UNCERTAINTY_ELLIPSE")]
		PointUncertaintyEllipse,
		#[serde(rename = "POLYGON")]
		Polygon,
		#[serde(rename = "POINT_ALTITUDE")]
		PointAltitude,
		#[serde(rename = "POINT_ALTITUDE_UNCERTAINTY")]
		PointAltitudeUncertainty,
		#[serde(rename = "ELLIPSOID_ARC")]
		EllipsoidArc,
		#[serde(rename = "LOCAL_2D_POINT_UNCERTAINTY_ELLIPSE")]
		Local2dPointUncertaintyEllipse,
		#[serde(rename = "LOCAL_3D_POINT_UNCERTAINTY_ELLIPSOID")]
		Local3dPointUncertaintyEllipsoid,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SupportedGadShapes> for SupportedGadShapes {
		fn from(value: &SupportedGadShapes) -> Self {
			value.clone()
		}
	}

	impl ToString for SupportedGadShapes {
		fn to_string(&self) -> String {
			match *self {
				Self::Point => "POINT".to_string(),
				Self::PointUncertaintyCircle => "POINT_UNCERTAINTY_CIRCLE".to_string(),
				Self::PointUncertaintyEllipse => "POINT_UNCERTAINTY_ELLIPSE".to_string(),
				Self::Polygon => "POLYGON".to_string(),
				Self::PointAltitude => "POINT_ALTITUDE".to_string(),
				Self::PointAltitudeUncertainty => "POINT_ALTITUDE_UNCERTAINTY".to_string(),
				Self::EllipsoidArc => "ELLIPSOID_ARC".to_string(),
				Self::Local2dPointUncertaintyEllipse => {
					"LOCAL_2D_POINT_UNCERTAINTY_ELLIPSE".to_string()
				}
				Self::Local3dPointUncertaintyEllipsoid => {
					"LOCAL_3D_POINT_UNCERTAINTY_ELLIPSOID".to_string()
				}
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SupportedGadShapes {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"POINT" => Ok(Self::Point),
				"POINT_UNCERTAINTY_CIRCLE" => Ok(Self::PointUncertaintyCircle),
				"POINT_UNCERTAINTY_ELLIPSE" => Ok(Self::PointUncertaintyEllipse),
				"POLYGON" => Ok(Self::Polygon),
				"POINT_ALTITUDE" => Ok(Self::PointAltitude),
				"POINT_ALTITUDE_UNCERTAINTY" => Ok(Self::PointAltitudeUncertainty),
				"ELLIPSOID_ARC" => Ok(Self::EllipsoidArc),
				"LOCAL_2D_POINT_UNCERTAINTY_ELLIPSE" => Ok(Self::Local2dPointUncertaintyEllipse),
				"LOCAL_3D_POINT_UNCERTAINTY_ELLIPSOID" => {
					Ok(Self::Local3dPointUncertaintyEllipsoid)
				}
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SupportedGadShapes {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SupportedGadShapes {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SupportedGadShapes {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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
		NewUnchecked,
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

	/// contains tracking area information (tracking area codes).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "contains tracking area information (tracking area
	/// codes).",
	///  "type": "object",
	///  "required": [
	///    "tacList"
	///  ],
	///  "properties": {
	///    "tacList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tac"
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
	pub struct TacInfo {
		#[serde(rename = "tacList")]
		pub tac_list: Vec<Tac>,
	}

	impl From<&TacInfo> for TacInfo {
		fn from(value: &TacInfo) -> Self {
			value.clone()
		}
	}

	/// This data type is defined in the same way as the 'Tac' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Tac'
	/// data type, but with the OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "(^[A-Fa-f0-9]{4}$)|(^[A-Fa-f0-9]{6}$)"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TacRm(pub Option<TacRmInner>);

	impl ::std::ops::Deref for TacRm {
		type Target = Option<TacRmInner>;
		fn deref(&self) -> &Option<TacRmInner> {
			&self.0
		}
	}

	impl From<TacRm> for Option<TacRmInner> {
		fn from(value: TacRm) -> Self {
			value.0
		}
	}

	impl From<&TacRm> for TacRm {
		fn from(value: &TacRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<TacRmInner>> for TacRm {
		fn from(value: Option<TacRmInner>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the 'Tac' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Tac'
	/// data type, but with the OpenAPI 'nullable: true' property.\n",
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
		NewUnchecked,
	)]
	pub struct TacRmInner(String);

	impl ::std::ops::Deref for TacRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TacRmInner> for String {
		fn from(value: TacRmInner) -> Self {
			value.0
		}
	}

	impl From<&TacRmInner> for TacRmInner {
		fn from(value: &TacRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TacRmInner {
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

	impl ::std::convert::TryFrom<&str> for TacRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TacRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TacRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TacRmInner {
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

	/// This data type is defined in the same way as the 'Tai' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'Tai'
	/// data type, but with the OpenAPI 'nullable: true' property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Tai"
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
	pub enum TaiRm {
		#[default]
		Tai(Tai),
		NullValue(NullValue),
	}

	impl From<&TaiRm> for TaiRm {
		fn from(value: &TaiRm) -> Self {
			value.clone()
		}
	}

	impl From<Tai> for TaiRm {
		fn from(value: Tai) -> Self {
			Self::Tai(value)
		}
	}

	impl From<NullValue> for TaiRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// "String with format 'time-numoffset' optionally appended by
	/// '<daylightSavingTime>', where
	///  - 'time-numoffset' shall represent the time zone adjusted for daylight
	///    saving time and be encoded as time-numoffset as defined in clause 5.6
	///    of IETF RFC 3339;
	///  - 'daylightSavingTime' shall represent the adjustment that has been
	///    made and shall be encoded as '+1' or '+2' for a +1 or +2 hours
	///    adjustment.
	///
	///  But with the OpenAPI 'nullable: true' property."
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "\"String with format 'time-numoffset' optionally appended by '<daylightSavingTime>', where\n  - 'time-numoffset' shall represent the time zone adjusted for daylight saving time and be\n    encoded as time-numoffset as defined in clause 5.6 of IETF RFC 3339;\n  - 'daylightSavingTime' shall represent the adjustment that has been made and shall be\n    encoded as '+1' or '+2' for a +1 or +2 hours adjustment.\n\n  But with the OpenAPI 'nullable: true' property.\"\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TimeZoneRm(pub Option<String>);

	impl ::std::ops::Deref for TimeZoneRm {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<TimeZoneRm> for Option<String> {
		fn from(value: TimeZoneRm) -> Self {
			value.0
		}
	}

	impl From<&TimeZoneRm> for TimeZoneRm {
		fn from(value: &TimeZoneRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for TimeZoneRm {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// Temporary Mobile Group Identity
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Temporary Mobile Group Identity",
	///  "type": "object",
	///  "required": [
	///    "mbsServiceId",
	///    "plmnId"
	///  ],
	///  "properties": {
	///    "mbsServiceId": {
	///      "description": "MBS Service ID",
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{6}$"
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
	pub struct Tmgi {
		/// MBS Service ID
		#[serde(rename = "mbsServiceId")]
		pub mbs_service_id: TmgiMbsServiceId,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
	}

	impl From<&Tmgi> for Tmgi {
		fn from(value: &Tmgi) -> Self {
			value.clone()
		}
	}

	/// MBS Service ID
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS Service ID",
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
		NewUnchecked,
	)]
	pub struct TmgiMbsServiceId(String);

	impl ::std::ops::Deref for TmgiMbsServiceId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TmgiMbsServiceId> for String {
		fn from(value: TmgiMbsServiceId) -> Self {
			value.0
		}
	}

	impl From<&TmgiMbsServiceId> for TmgiMbsServiceId {
		fn from(value: &TmgiMbsServiceId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TmgiMbsServiceId {
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

	impl ::std::convert::TryFrom<&str> for TmgiMbsServiceId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TmgiMbsServiceId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TmgiMbsServiceId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TmgiMbsServiceId {
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

	/// This data type is defined in the same way as the 'TnapId' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'TnapId' data type, but with the OpenAPI 'nullable: true' property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/TnapId"
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
	pub enum TnapIdRm {
		#[default]
		TnapId(TnapId),
		NullValue(NullValue),
	}

	impl From<&TnapIdRm> for TnapIdRm {
		fn from(value: &TnapIdRm) -> Self {
			value.clone()
		}
	}

	impl From<TnapId> for TnapIdRm {
		fn from(value: TnapId) -> Self {
			Self::TnapId(value)
		}
	}

	impl From<NullValue> for TnapIdRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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

	/// This enumeration is defined in the same way as the 'TraceDepth'
	/// enumeration, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This enumeration is defined in the same way as the
	/// 'TraceDepth' enumeration, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/TraceDepth"
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
	pub enum TraceDepthRm {
		#[default]
		TraceDepth(TraceDepth),
		NullValue(NullValue),
	}

	impl From<&TraceDepthRm> for TraceDepthRm {
		fn from(value: &TraceDepthRm) -> Self {
			value.clone()
		}
	}

	impl From<TraceDepth> for TraceDepthRm {
		fn from(value: TraceDepth) -> Self {
			Self::TraceDepth(value)
		}
	}

	impl From<NullValue> for TraceDepthRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// This enumeration is defined in the same way as the 'TrafficProfile'
	/// enumeration, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This enumeration is defined in the same way as the
	/// 'TrafficProfile' enumeration, but with the OpenAPI 'nullable: true'
	/// property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/TrafficProfile"
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
	pub enum TrafficProfileRm {
		#[default]
		TrafficProfile(TrafficProfile),
		NullValue(NullValue),
	}

	impl From<&TrafficProfileRm> for TrafficProfileRm {
		fn from(value: &TrafficProfileRm) -> Self {
			value.clone()
		}
	}

	impl From<TrafficProfile> for TrafficProfileRm {
		fn from(value: TrafficProfile) -> Self {
			Self::TrafficProfile(value)
		}
	}

	impl From<NullValue> for TrafficProfileRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// Tunnel address
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Tunnel address",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "ipv4Addr"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipv6Addr"
	///      ]
	///    }
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
	#[serde(untagged)]
	pub enum TunnelAddress {
		#[default]
		Variant0 {
			#[serde(rename = "ipv4Addr")]
			ipv4_addr: Ipv4Addr,
			#[serde(rename = "portNumber")]
			port_number: Uinteger,
		},
		Variant1 {
			#[serde(rename = "ipv6Addr")]
			ipv6_addr: Ipv6Addr,
			#[serde(rename = "portNumber")]
			port_number: Uinteger,
		},
	}

	impl From<&TunnelAddress> for TunnelAddress {
		fn from(value: &TunnelAddress) -> Self {
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

	/// This data type is defined in the same way as the 'TwapId' data type, but
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the 'TwapId' data type, but with the OpenAPI 'nullable: true' property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/TwapId"
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
	pub enum TwapIdRm {
		#[default]
		TwapId(TwapId),
		NullValue(NullValue),
	}

	impl From<&TwapIdRm> for TwapIdRm {
		fn from(value: &TwapIdRm) -> Self {
			value.clone()
		}
	}

	impl From<TwapId> for TwapIdRm {
		fn from(value: TwapId) -> Self {
			Self::TwapId(value)
		}
	}

	impl From<NullValue> for TwapIdRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// Type Allocation Code (TAC) of the UE, comprising the initial eight-digit
	/// portion of the 15-digit IMEI and 16-digit IMEISV codes. See clause 6.2
	/// of 3GPP TS 23.003.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Type Allocation Code (TAC) of the UE, comprising the
	/// initial eight-digit portion of the 15-digit IMEI and 16-digit IMEISV
	/// codes. See clause 6.2 of 3GPP TS 23.003.\n",
	///  "type": "string",
	///  "pattern": "^[0-9]{8}$"
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
	pub struct TypeAllocationCode(String);

	impl ::std::ops::Deref for TypeAllocationCode {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TypeAllocationCode> for String {
		fn from(value: TypeAllocationCode) -> Self {
			value.0
		}
	}

	impl From<&TypeAllocationCode> for TypeAllocationCode {
		fn from(value: &TypeAllocationCode) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TypeAllocationCode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{8}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{8}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for TypeAllocationCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TypeAllocationCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TypeAllocationCode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TypeAllocationCode {
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
	/// - AUTHORIZED: Indicates that the UE is authorized.
	/// - NOT_AUTHORIZED: Indicates that the UE is not authorized.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- AUTHORIZED: Indicates that the
	/// UE is authorized.\n- NOT_AUTHORIZED: Indicates that the UE is not
	/// authorized.\n",
	///  "type": "string",
	///  "enum": [
	///    "AUTHORIZED",
	///    "NOT_AUTHORIZED"
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
	pub enum UeAuth {
		#[default]
		#[serde(rename = "AUTHORIZED")]
		Authorized,
		#[serde(rename = "NOT_AUTHORIZED")]
		NotAuthorized,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UeAuth> for UeAuth {
		fn from(value: &UeAuth) -> Self {
			value.clone()
		}
	}

	impl ToString for UeAuth {
		fn to_string(&self) -> String {
			match *self {
				Self::Authorized => "AUTHORIZED".to_string(),
				Self::NotAuthorized => "NOT_AUTHORIZED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UeAuth {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AUTHORIZED" => Ok(Self::Authorized),
				"NOT_AUTHORIZED" => Ok(Self::NotAuthorized),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UeAuth {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UeAuth {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UeAuth {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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
	/// unsigned  16-bit integer with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Integer where the allowed values correspond to the
	/// value range of an unsigned  16-bit integer with the OpenAPI 'nullable:
	/// true' property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 65535.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Uint16Rm(pub Option<u16>);

	impl ::std::ops::Deref for Uint16Rm {
		type Target = Option<u16>;
		fn deref(&self) -> &Option<u16> {
			&self.0
		}
	}

	impl From<Uint16Rm> for Option<u16> {
		fn from(value: Uint16Rm) -> Self {
			value.0
		}
	}

	impl From<&Uint16Rm> for Uint16Rm {
		fn from(value: &Uint16Rm) -> Self {
			value.clone()
		}
	}

	impl From<Option<u16>> for Uint16Rm {
		fn from(value: Option<u16>) -> Self {
			Self(value)
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
	/// unsigned 32-bit integer with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Integer where the allowed values correspond to the
	/// value range of an unsigned 32-bit integer with the OpenAPI 'nullable:
	/// true' property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "format": "int32",
	///  "maximum": 4294967295.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Uint32Rm(pub Option<u32>);

	impl ::std::ops::Deref for Uint32Rm {
		type Target = Option<u32>;
		fn deref(&self) -> &Option<u32> {
			&self.0
		}
	}

	impl From<Uint32Rm> for Option<u32> {
		fn from(value: Uint32Rm) -> Self {
			value.0
		}
	}

	impl From<&Uint32Rm> for Uint32Rm {
		fn from(value: &Uint32Rm) -> Self {
			value.clone()
		}
	}

	impl From<Option<u32>> for Uint32Rm {
		fn from(value: Option<u32>) -> Self {
			Self(value)
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

	/// Integer where the allowed values correspond to the value range of an
	/// unsigned 16-bit integer with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Integer where the allowed values correspond to the
	/// value range of an unsigned 16-bit integer with the OpenAPI 'nullable:
	/// true' property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 1.8446744073709552e19,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Uint64Rm(pub Option<u64>);

	impl ::std::ops::Deref for Uint64Rm {
		type Target = Option<u64>;
		fn deref(&self) -> &Option<u64> {
			&self.0
		}
	}

	impl From<Uint64Rm> for Option<u64> {
		fn from(value: Uint64Rm) -> Self {
			value.0
		}
	}

	impl From<&Uint64Rm> for Uint64Rm {
		fn from(value: &Uint64Rm) -> Self {
			value.clone()
		}
	}

	impl From<Option<u64>> for Uint64Rm {
		fn from(value: Option<u64>) -> Self {
			Self(value)
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

	/// Unsigned Integer, i.e. only value 0 and integers above 0 are permissible
	/// with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned Integer, i.e. only value 0 and integers above
	/// 0 are permissible with the OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UintegerRm(pub Option<u64>);

	impl ::std::ops::Deref for UintegerRm {
		type Target = Option<u64>;
		fn deref(&self) -> &Option<u64> {
			&self.0
		}
	}

	impl From<UintegerRm> for Option<u64> {
		fn from(value: UintegerRm) -> Self {
			value.0
		}
	}

	impl From<&UintegerRm> for UintegerRm {
		fn from(value: &UintegerRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<u64>> for UintegerRm {
		fn from(value: Option<u64>) -> Self {
			Self(value)
		}
	}

	/// Uncertainty
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates value of uncertainty.",
	///  "type": "number",
	///  "format": "float",
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Uncertainty(pub f32);

	impl ::std::ops::Deref for Uncertainty {
		type Target = f32;
		fn deref(&self) -> &f32 {
			&self.0
		}
	}

	impl From<Uncertainty> for f32 {
		fn from(value: Uncertainty) -> Self {
			value.0
		}
	}

	impl From<&Uncertainty> for Uncertainty {
		fn from(value: &Uncertainty) -> Self {
			value.clone()
		}
	}

	impl From<f32> for Uncertainty {
		fn from(value: f32) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Uncertainty {
		type Err = <f32 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Uncertainty {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Uncertainty {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Uncertainty {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Uncertainty {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Ellipse with uncertainty.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Ellipse with uncertainty.",
	///  "type": "object",
	///  "required": [
	///    "orientationMajor",
	///    "semiMajor",
	///    "semiMinor"
	///  ],
	///  "properties": {
	///    "orientationMajor": {
	///      "$ref": "#/components/schemas/Orientation"
	///    },
	///    "semiMajor": {
	///      "$ref": "#/components/schemas/Uncertainty"
	///    },
	///    "semiMinor": {
	///      "$ref": "#/components/schemas/Uncertainty"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UncertaintyEllipse {
		#[serde(rename = "orientationMajor")]
		pub orientation_major: Orientation,
		#[serde(rename = "semiMajor")]
		pub semi_major: Uncertainty,
		#[serde(rename = "semiMinor")]
		pub semi_minor: Uncertainty,
	}

	impl From<&UncertaintyEllipse> for UncertaintyEllipse {
		fn from(value: &UncertaintyEllipse) -> Self {
			value.clone()
		}
	}

	/// Ellipsoid with uncertainty
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Ellipsoid with uncertainty",
	///  "type": "object",
	///  "required": [
	///    "orientationMajor",
	///    "semiMajor",
	///    "semiMinor",
	///    "vertical"
	///  ],
	///  "properties": {
	///    "orientationMajor": {
	///      "$ref": "#/components/schemas/Orientation"
	///    },
	///    "semiMajor": {
	///      "$ref": "#/components/schemas/Uncertainty"
	///    },
	///    "semiMinor": {
	///      "$ref": "#/components/schemas/Uncertainty"
	///    },
	///    "vertical": {
	///      "$ref": "#/components/schemas/Uncertainty"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UncertaintyEllipsoid {
		#[serde(rename = "orientationMajor")]
		pub orientation_major: Orientation,
		#[serde(rename = "semiMajor")]
		pub semi_major: Uncertainty,
		#[serde(rename = "semiMinor")]
		pub semi_minor: Uncertainty,
		pub vertical: Uncertainty,
	}

	impl From<&UncertaintyEllipsoid> for UncertaintyEllipsoid {
		fn from(value: &UncertaintyEllipsoid) -> Self {
			value.clone()
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
	/// needed for all the  traffic on the PDU Session. It shall comply with the
	/// provisions defined in table 5.4.3.4-1, but with the OpenAPI 'nullable:
	/// true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "indicates whether UP integrity protection is required,
	/// preferred or not needed for all the  traffic on the PDU Session. It
	/// shall comply with the provisions defined in table 5.4.3.4-1, but with
	/// the OpenAPI 'nullable: true' property. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/UpConfidentiality"
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
	pub enum UpConfidentialityRm {
		#[default]
		UpConfidentiality(UpConfidentiality),
		NullValue(NullValue),
	}

	impl From<&UpConfidentialityRm> for UpConfidentialityRm {
		fn from(value: &UpConfidentialityRm) -> Self {
			value.clone()
		}
	}

	impl From<UpConfidentiality> for UpConfidentialityRm {
		fn from(value: UpConfidentiality) -> Self {
			Self::UpConfidentiality(value)
		}
	}

	impl From<NullValue> for UpConfidentialityRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// indicates whether UP integrity protection is required, preferred or not
	/// needed for all the traffic on the PDU Session. It shall comply with the
	/// provisions defined in table 5.4.3.4-1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "indicates whether UP integrity protection is required,
	/// preferred or not needed for all the traffic on the PDU Session. It shall
	/// comply with the provisions defined in table 5.4.3.4-1. \n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/UpIntegrity"
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
	pub enum UpIntegrityRm {
		#[default]
		UpIntegrity(UpIntegrity),
		NullValue(NullValue),
	}

	impl From<&UpIntegrityRm> for UpIntegrityRm {
		fn from(value: &UpIntegrityRm) -> Self {
			value.clone()
		}
	}

	impl From<UpIntegrity> for UpIntegrityRm {
		fn from(value: UpIntegrity) -> Self {
			Self::UpIntegrity(value)
		}
	}

	impl From<NullValue> for UpIntegrityRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
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

	/// This data type is defined in the same way as the 'UpSecurity' data type,
	/// but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// 'UpSecurity' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/UpSecurity"
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
	pub enum UpSecurityRm {
		#[default]
		UpSecurity(UpSecurity),
		NullValue(NullValue),
	}

	impl From<&UpSecurityRm> for UpSecurityRm {
		fn from(value: &UpSecurityRm) -> Self {
			value.clone()
		}
	}

	impl From<UpSecurity> for UpSecurityRm {
		fn from(value: UpSecurity) -> Self {
			Self::UpSecurity(value)
		}
	}

	impl From<NullValue> for UpSecurityRm {
		fn from(value: NullValue) -> Self {
			Self::NullValue(value)
		}
	}

	/// String providing an URI formatted according to RFC 3986 with the OpenAPI
	/// 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String providing an URI formatted according to RFC 3986
	/// with the OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UriRm(pub Option<String>);

	impl ::std::ops::Deref for UriRm {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<UriRm> for Option<String> {
		fn from(value: UriRm) -> Self {
			value.0
		}
	}

	impl From<&UriRm> for UriRm {
		fn from(value: &UriRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for UriRm {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// HTTP and HTTPS URI scheme.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "HTTP and HTTPS URI scheme.",
	///  "type": "string",
	///  "enum": [
	///    "http",
	///    "https"
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
	pub enum UriScheme {
		#[default]
		#[serde(rename = "http")]
		Http,
		#[serde(rename = "https")]
		Https,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UriScheme> for UriScheme {
		fn from(value: &UriScheme) -> Self {
			value.clone()
		}
	}

	impl ToString for UriScheme {
		fn to_string(&self) -> String {
			match *self {
				Self::Http => "http".to_string(),
				Self::Https => "https".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UriScheme {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"http" => Ok(Self::Http),
				"https" => Ok(Self::Https),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UriScheme {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UriScheme {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UriScheme {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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

	/// String represents the SUPI or GPSI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String represents the SUPI or GPSI",
	///  "type": "string",
	///  "pattern":
	/// "^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.
	/// +|gli-.+|.+)$"
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
	pub struct VarUeId(String);

	impl ::std::ops::Deref for VarUeId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<VarUeId> for String {
		fn from(value: VarUeId) -> Self {
			value.0
		}
	}

	impl From<&VarUeId> for VarUeId {
		fn from(value: &VarUeId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for VarUeId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@\
				            ]+|gci-.+|gli-.+|.+)$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for VarUeId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for VarUeId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for VarUeId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for VarUeId {
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

	/// String represents the SUPI or GPSI with the OpenAPI 'nullable: true'
	/// property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String represents the SUPI or GPSI with the OpenAPI
	/// 'nullable: true' property.",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern":
	/// "^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.
	/// +|gli-.+|.+)$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VarUeIdRm(pub Option<VarUeIdRmInner>);

	impl ::std::ops::Deref for VarUeIdRm {
		type Target = Option<VarUeIdRmInner>;
		fn deref(&self) -> &Option<VarUeIdRmInner> {
			&self.0
		}
	}

	impl From<VarUeIdRm> for Option<VarUeIdRmInner> {
		fn from(value: VarUeIdRm) -> Self {
			value.0
		}
	}

	impl From<&VarUeIdRm> for VarUeIdRm {
		fn from(value: &VarUeIdRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<VarUeIdRmInner>> for VarUeIdRm {
		fn from(value: Option<VarUeIdRmInner>) -> Self {
			Self(value)
		}
	}

	/// String represents the SUPI or GPSI with the OpenAPI 'nullable: true'
	/// property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String represents the SUPI or GPSI with the OpenAPI
	/// 'nullable: true' property.",
	///  "type": "string",
	///  "pattern":
	/// "^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.
	/// +|gli-.+|.+)$"
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
	pub struct VarUeIdRmInner(String);

	impl ::std::ops::Deref for VarUeIdRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<VarUeIdRmInner> for String {
		fn from(value: VarUeIdRmInner) -> Self {
			value.0
		}
	}

	impl From<&VarUeIdRmInner> for VarUeIdRmInner {
		fn from(value: &VarUeIdRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for VarUeIdRmInner {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@]+|gci-.+|gli-.+|.+)$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^(imsi-[0-9]{5,15}|nai-.+|msisdn-[0-9]{5,15}|extid-[^@]+@[^@\
				            ]+|gci-.+|gli-.+|.+)$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for VarUeIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for VarUeIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for VarUeIdRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for VarUeIdRmInner {
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
		NewUnchecked,
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

	/// String representing the Wildcard DNN. It shall contain the string "*".
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing the Wildcard DNN. It shall contain
	/// the string \"*\".",
	///  "type": "string",
	///  "pattern": "^[*]$"
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
	pub struct WildcardDnn(String);

	impl ::std::ops::Deref for WildcardDnn {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<WildcardDnn> for String {
		fn from(value: WildcardDnn) -> Self {
			value.0
		}
	}

	impl From<&WildcardDnn> for WildcardDnn {
		fn from(value: &WildcardDnn) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for WildcardDnn {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[*]$").unwrap().find(value).is_none() {
				return Err("doesn't match pattern \"^[*]$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for WildcardDnn {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for WildcardDnn {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for WildcardDnn {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for WildcardDnn {
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

	/// String representing the Wildcard DNN. It shall contain the string '*'
	/// but with the  OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing the Wildcard DNN. It shall contain
	/// the string '*' but with the  OpenAPI 'nullable: true' property.\n",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "^[*]$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct WildcardDnnRm(pub Option<WildcardDnnRmInner>);

	impl ::std::ops::Deref for WildcardDnnRm {
		type Target = Option<WildcardDnnRmInner>;
		fn deref(&self) -> &Option<WildcardDnnRmInner> {
			&self.0
		}
	}

	impl From<WildcardDnnRm> for Option<WildcardDnnRmInner> {
		fn from(value: WildcardDnnRm) -> Self {
			value.0
		}
	}

	impl From<&WildcardDnnRm> for WildcardDnnRm {
		fn from(value: &WildcardDnnRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<WildcardDnnRmInner>> for WildcardDnnRm {
		fn from(value: Option<WildcardDnnRmInner>) -> Self {
			Self(value)
		}
	}

	/// String representing the Wildcard DNN. It shall contain the string '*'
	/// but with the  OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "String representing the Wildcard DNN. It shall contain
	/// the string '*' but with the  OpenAPI 'nullable: true' property.\n",
	///  "type": "string",
	///  "pattern": "^[*]$"
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
	pub struct WildcardDnnRmInner(String);

	impl ::std::ops::Deref for WildcardDnnRmInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<WildcardDnnRmInner> for String {
		fn from(value: WildcardDnnRmInner) -> Self {
			value.0
		}
	}

	impl From<&WildcardDnnRmInner> for WildcardDnnRmInner {
		fn from(value: &WildcardDnnRmInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for WildcardDnnRmInner {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[*]$").unwrap().find(value).is_none() {
				return Err("doesn't match pattern \"^[*]$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for WildcardDnnRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for WildcardDnnRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for WildcardDnnRmInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for WildcardDnnRmInner {
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

	/// One and only one of the "globLineIds", "hfcNIds", "areaCodeB" and
	/// "areaCodeC" attributes shall be included in a WirelineArea data
	/// structure
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "One and only one of the \"globLineIds\", \"hfcNIds\",
	/// \"areaCodeB\" and \"areaCodeC\" attributes shall be included in a
	/// WirelineArea data structure\n",
	///  "type": "object",
	///  "properties": {
	///    "areaCodeB": {
	///      "$ref": "#/components/schemas/AreaCode"
	///    },
	///    "areaCodeC": {
	///      "$ref": "#/components/schemas/AreaCode"
	///    },
	///    "globalLineIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gli"
	///      },
	///      "minItems": 1
	///    },
	///    "hfcNIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/HfcNId"
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
	pub struct WirelineArea {
		#[serde(rename = "areaCodeB", default, skip_serializing_if = "Option::is_none")]
		pub area_code_b: Option<AreaCode>,
		#[serde(rename = "areaCodeC", default, skip_serializing_if = "Option::is_none")]
		pub area_code_c: Option<AreaCode>,
		#[serde(
			rename = "globalLineIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub global_line_ids: Vec<Gli>,
		#[serde(rename = "hfcNIds", default, skip_serializing_if = "Vec::is_empty")]
		pub hfc_n_ids: Vec<HfcNId>,
	}

	impl From<&WirelineArea> for WirelineArea {
		fn from(value: &WirelineArea) -> Self {
			value.clone()
		}
	}

	/// The "restrictionType" attribute and the "areas" attribute shall be
	/// either both present or absent.  The empty array of areas is used when
	/// service is allowed/restricted nowhere.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The \"restrictionType\" attribute and the \"areas\"
	/// attribute shall be either both present or absent.  The empty array of
	/// areas is used when service is allowed/restricted nowhere.\n",
	///  "type": "object",
	///  "properties": {
	///    "areas": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/WirelineArea"
	///      }
	///    },
	///    "restrictionType": {
	///      "$ref": "#/components/schemas/RestrictionType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct WirelineServiceAreaRestriction {
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub areas: Vec<WirelineArea>,
		#[serde(
			rename = "restrictionType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub restriction_type: Option<RestrictionType>,
	}

	impl From<&WirelineServiceAreaRestriction> for WirelineServiceAreaRestriction {
		fn from(value: &WirelineServiceAreaRestriction) -> Self {
			value.clone()
		}
	}

	/// _5gMmCause
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/Uinteger"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct _5gMmCause(pub Uinteger);

	impl ::std::ops::Deref for _5gMmCause {
		type Target = Uinteger;
		fn deref(&self) -> &Uinteger {
			&self.0
		}
	}

	impl From<_5gMmCause> for Uinteger {
		fn from(value: _5gMmCause) -> Self {
			value.0
		}
	}

	impl From<&_5gMmCause> for _5gMmCause {
		fn from(value: &_5gMmCause) -> Self {
			value.clone()
		}
	}

	impl From<Uinteger> for _5gMmCause {
		fn from(value: Uinteger) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for _5gMmCause {
		type Err = <Uinteger as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for _5gMmCause {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for _5gMmCause {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for _5gMmCause {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for _5gMmCause {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// A string carrying the CP-PRUK ID of the remote UE. The CP-PRUK ID is a
	/// string in NAI format as specified in clause 28.7.11 of 3GPP TS 23.003.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A string carrying the CP-PRUK ID of the remote UE. The
	/// CP-PRUK ID is a string in NAI format as specified in clause 28.7.11 of
	/// 3GPP TS 23.003.\n",
	///  "type": "string",
	///  "pattern":
	/// "^rid[0-9]{1,4}\\.pid[0-9a-fA-F]+\\@prose-cp\\.5gc\\.mnc[0-9]{2,3}\\.
	/// mcc[0-9]{3}\\.3gppnetwork\\.org$"
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
	pub struct _5gPrukId(String);

	impl ::std::ops::Deref for _5gPrukId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<_5gPrukId> for String {
		fn from(value: _5gPrukId) -> Self {
			value.0
		}
	}

	impl From<&_5gPrukId> for _5gPrukId {
		fn from(value: &_5gPrukId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for _5gPrukId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"^rid[0-9]{1,4}\\.pid[0-9a-fA-F]+\\@prose-cp\\.5gc\\.mnc[0-9]{2,3}\\.mcc[0-9]{3}\\\
				 .3gppnetwork\\.org$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^rid[0-9]{1,4}\\.pid[0-9a-fA-F]+\\@prose-cp\\.5gc\\.mnc[0-9]{2,3}\\
				            \
				            .mcc[0-9]{3}\\.3gppnetwork\\.org$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for _5gPrukId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for _5gPrukId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for _5gPrukId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for _5gPrukId {
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

	/// This data type is defined in the same way as the '5QiPriorityLevel' data
	/// type, but with the OpenAPI 'nullable: true' property.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// '5QiPriorityLevel' data type, but with the OpenAPI 'nullable: true'
	/// property.\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 127.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct _5qiPriorityLevelRm(pub Option<i64>);

	impl ::std::ops::Deref for _5qiPriorityLevelRm {
		type Target = Option<i64>;
		fn deref(&self) -> &Option<i64> {
			&self.0
		}
	}

	impl From<_5qiPriorityLevelRm> for Option<i64> {
		fn from(value: _5qiPriorityLevelRm) -> Self {
			value.0
		}
	}

	impl From<&_5qiPriorityLevelRm> for _5qiPriorityLevelRm {
		fn from(value: &_5qiPriorityLevelRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<i64>> for _5qiPriorityLevelRm {
		fn from(value: Option<i64>) -> Self {
			Self(value)
		}
	}

	/// This data type is defined in the same way as the '5QiPriorityLevel' data
	/// type, but with the OpenAPI 'nullable: true' property. "
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "This data type is defined in the same way as the
	/// '5QiPriorityLevel' data type, but with the OpenAPI 'nullable: true'
	/// property. \"\n",
	///  "type": [
	///    "integer",
	///    "null"
	///  ],
	///  "maximum": 255.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct _5qiRm(pub Option<u8>);

	impl ::std::ops::Deref for _5qiRm {
		type Target = Option<u8>;
		fn deref(&self) -> &Option<u8> {
			&self.0
		}
	}

	impl From<_5qiRm> for Option<u8> {
		fn from(value: _5qiRm) -> Self {
			value.0
		}
	}

	impl From<&_5qiRm> for _5qiRm {
		fn from(value: &_5qiRm) -> Self {
			value.clone()
		}
	}

	impl From<Option<u8>> for _5qiRm {
		fn from(value: Option<u8>) -> Self {
			Self(value)
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

#[allow(dead_code)]
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
