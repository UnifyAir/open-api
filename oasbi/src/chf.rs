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

	/// AccessNetworkInfoChange
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "accessNetworkInformation": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/OctetString"
	///      },
	///      "minItems": 0
	///    },
	///    "cellularNetworkInformation": {
	///      "$ref": "#/components/schemas/OctetString"
	///    },
	///    "changeTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AccessNetworkInfoChange {
		#[serde(
			rename = "accessNetworkInformation",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub access_network_information: Vec<OctetString>,
		#[serde(
			rename = "cellularNetworkInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cellular_network_information: Option<OctetString>,
		#[serde(
			rename = "changeTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub change_time: Option<DateTime>,
	}

	impl From<&AccessNetworkInfoChange> for AccessNetworkInfoChange {
		fn from(value: &AccessNetworkInfoChange) -> Self {
			value.clone()
		}
	}

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

	/// AccessTransferInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "accessNetworkInformation": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/OctetString"
	///      },
	///      "minItems": 0
	///    },
	///    "accessTransferType": {
	///      "$ref": "#/components/schemas/AccessTransferType"
	///    },
	///    "cellularNetworkInformation": {
	///      "$ref": "#/components/schemas/OctetString"
	///    },
	///    "changeTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "instanceId": {
	///      "type": "string"
	///    },
	///    "interUETransfer": {
	///      "$ref": "#/components/schemas/UETransferType"
	///    },
	///    "relatedIMSChargingIdentifier": {
	///      "type": "string"
	///    },
	///    "relatedIMSChargingIdentifierNode": {
	///      "$ref": "#/components/schemas/IMSAddress"
	///    },
	///    "userEquipmentInfo": {
	///      "$ref": "#/components/schemas/Pei"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AccessTransferInformation {
		#[serde(
			rename = "accessNetworkInformation",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub access_network_information: Vec<OctetString>,
		#[serde(
			rename = "accessTransferType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_transfer_type: Option<AccessTransferType>,
		#[serde(
			rename = "cellularNetworkInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cellular_network_information: Option<OctetString>,
		#[serde(
			rename = "changeTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub change_time: Option<DateTime>,
		#[serde(
			rename = "instanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub instance_id: Option<String>,
		#[serde(
			rename = "interUETransfer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_ue_transfer: Option<UeTransferType>,
		#[serde(
			rename = "relatedIMSChargingIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub related_ims_charging_identifier: Option<String>,
		#[serde(
			rename = "relatedIMSChargingIdentifierNode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub related_ims_charging_identifier_node: Option<ImsAddress>,
		#[serde(
			rename = "userEquipmentInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_equipment_info: Option<Pei>,
	}

	impl From<&AccessTransferInformation> for AccessTransferInformation {
		fn from(value: &AccessTransferInformation) -> Self {
			value.clone()
		}
	}

	/// AccessTransferType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "PS_TO_CS",
	///    "CS_TO_PS",
	///    "PS_TO_PS",
	///    "CS_TO_CS"
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
	pub enum AccessTransferType {
		#[default]
		#[serde(rename = "PS_TO_CS")]
		PsToCs,
		#[serde(rename = "CS_TO_PS")]
		CsToPs,
		#[serde(rename = "PS_TO_PS")]
		PsToPs,
		#[serde(rename = "CS_TO_CS")]
		CsToCs,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AccessTransferType> for AccessTransferType {
		fn from(value: &AccessTransferType) -> Self {
			value.clone()
		}
	}

	impl ToString for AccessTransferType {
		fn to_string(&self) -> String {
			match *self {
				Self::PsToCs => "PS_TO_CS".to_string(),
				Self::CsToPs => "CS_TO_PS".to_string(),
				Self::PsToPs => "PS_TO_PS".to_string(),
				Self::CsToCs => "CS_TO_CS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AccessTransferType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PS_TO_CS" => Ok(Self::PsToCs),
				"CS_TO_PS" => Ok(Self::CsToPs),
				"PS_TO_PS" => Ok(Self::PsToPs),
				"CS_TO_CS" => Ok(Self::CsToCs),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AccessTransferType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AccessTransferType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AccessTransferType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// AdministrativeState
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "LOCKED",
	///    "UNLOCKED"
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
	pub enum AdministrativeState {
		#[default]
		#[serde(rename = "LOCKED")]
		Locked,
		#[serde(rename = "UNLOCKED")]
		Unlocked,
	}

	impl From<&AdministrativeState> for AdministrativeState {
		fn from(value: &AdministrativeState) -> Self {
			value.clone()
		}
	}

	impl ToString for AdministrativeState {
		fn to_string(&self) -> String {
			match *self {
				Self::Locked => "LOCKED".to_string(),
				Self::Unlocked => "UNLOCKED".to_string(),
			}
		}
	}

	impl std::str::FromStr for AdministrativeState {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOCKED" => Ok(Self::Locked),
				"UNLOCKED" => Ok(Self::Unlocked),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AdministrativeState {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AdministrativeState {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AdministrativeState {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// AffinityAntiAffinity
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "affinityEAS": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      }
	///    },
	///    "antiAffinityEAS": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AffinityAntiAffinity {
		#[serde(rename = "affinityEAS", default, skip_serializing_if = "Vec::is_empty")]
		pub affinity_eas: Vec<String>,
		#[serde(
			rename = "antiAffinityEAS",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub anti_affinity_eas: Vec<String>,
	}

	impl From<&AffinityAntiAffinity> for AffinityAntiAffinity {
		fn from(value: &AffinityAntiAffinity) -> Self {
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

	/// AnnouncementInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "Language": {
	///      "$ref": "#/components/schemas/Language"
	///    },
	///    "announcementIdentifier": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "announcementPriority": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "announcementPrivacyIndicator": {
	///      "$ref": "#/components/schemas/AnnouncementPrivacyIndicator"
	///    },
	///    "announcementReference": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "playToParty": {
	///      "$ref": "#/components/schemas/PlayToParty"
	///    },
	///    "quotaConsumptionIndicator": {
	///      "$ref": "#/components/schemas/QuotaConsumptionIndicator"
	///    },
	///    "timeToPlay": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "variableParts": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/VariablePart"
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
	pub struct AnnouncementInformation {
		#[serde(
			rename = "announcementIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub announcement_identifier: Option<Uint32>,
		#[serde(
			rename = "announcementPriority",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub announcement_priority: Option<Uint32>,
		#[serde(
			rename = "announcementPrivacyIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub announcement_privacy_indicator: Option<AnnouncementPrivacyIndicator>,
		#[serde(
			rename = "announcementReference",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub announcement_reference: Option<Uri>,
		#[serde(rename = "Language", default, skip_serializing_if = "Option::is_none")]
		pub language: Option<Language>,
		#[serde(
			rename = "playToParty",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub play_to_party: Option<PlayToParty>,
		#[serde(
			rename = "quotaConsumptionIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub quota_consumption_indicator: Option<QuotaConsumptionIndicator>,
		#[serde(
			rename = "timeToPlay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_to_play: Option<DurationSec>,
		#[serde(
			rename = "variableParts",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub variable_parts: Vec<VariablePart>,
	}

	impl From<&AnnouncementInformation> for AnnouncementInformation {
		fn from(value: &AnnouncementInformation) -> Self {
			value.clone()
		}
	}

	/// AnnouncementPrivacyIndicator
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "NOT_PRIVATE",
	///    "PRIVATE"
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
	pub enum AnnouncementPrivacyIndicator {
		#[default]
		#[serde(rename = "NOT_PRIVATE")]
		NotPrivate,
		#[serde(rename = "PRIVATE")]
		Private,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AnnouncementPrivacyIndicator> for AnnouncementPrivacyIndicator {
		fn from(value: &AnnouncementPrivacyIndicator) -> Self {
			value.clone()
		}
	}

	impl ToString for AnnouncementPrivacyIndicator {
		fn to_string(&self) -> String {
			match *self {
				Self::NotPrivate => "NOT_PRIVATE".to_string(),
				Self::Private => "PRIVATE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AnnouncementPrivacyIndicator {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NOT_PRIVATE" => Ok(Self::NotPrivate),
				"PRIVATE" => Ok(Self::Private),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AnnouncementPrivacyIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AnnouncementPrivacyIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AnnouncementPrivacyIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ApiDirection
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "INVOCATION",
	///    "NOTIFICATION"
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
	pub enum ApiDirection {
		#[default]
		#[serde(rename = "INVOCATION")]
		Invocation,
		#[serde(rename = "NOTIFICATION")]
		Notification,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ApiDirection> for ApiDirection {
		fn from(value: &ApiDirection) -> Self {
			value.clone()
		}
	}

	impl ToString for ApiDirection {
		fn to_string(&self) -> String {
			match *self {
				Self::Invocation => "INVOCATION".to_string(),
				Self::Notification => "NOTIFICATION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ApiDirection {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"INVOCATION" => Ok(Self::Invocation),
				"NOTIFICATION" => Ok(Self::Notification),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ApiDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ApiDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ApiDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Represents the Authorized Default QoS.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the Authorized Default QoS.",
	///  "type": "object",
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "arp": {
	///      "$ref": "#/components/schemas/Arp"
	///    },
	///    "averWindow": {
	///      "$ref": "#/components/schemas/AverWindowRm"
	///    },
	///    "extMaxDataBurstVol": {
	///      "$ref": "#/components/schemas/ExtMaxDataBurstVolRm"
	///    },
	///    "gbrDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "gbrUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "maxDataBurstVol": {
	///      "$ref": "#/components/schemas/MaxDataBurstVolRm"
	///    },
	///    "maxbrDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "maxbrUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "priorityLevel": {
	///      "$ref": "#/components/schemas/5QiPriorityLevelRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AuthorizedDefaultQos {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub arp: Option<Arp>,
		#[serde(
			rename = "averWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub aver_window: Option<AverWindowRm>,
		#[serde(
			rename = "extMaxDataBurstVol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ext_max_data_burst_vol: Option<ExtMaxDataBurstVolRm>,
		#[serde(rename = "5qi", default, skip_serializing_if = "Option::is_none")]
		pub five_qi: Option<_5qi>,
		#[serde(rename = "gbrDl", default, skip_serializing_if = "Option::is_none")]
		pub gbr_dl: Option<BitRateRm>,
		#[serde(rename = "gbrUl", default, skip_serializing_if = "Option::is_none")]
		pub gbr_ul: Option<BitRateRm>,
		#[serde(
			rename = "maxDataBurstVol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_data_burst_vol: Option<MaxDataBurstVolRm>,
		#[serde(rename = "maxbrDl", default, skip_serializing_if = "Option::is_none")]
		pub maxbr_dl: Option<BitRateRm>,
		#[serde(rename = "maxbrUl", default, skip_serializing_if = "Option::is_none")]
		pub maxbr_ul: Option<BitRateRm>,
		#[serde(
			rename = "priorityLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub priority_level: Option<_5qiPriorityLevelRm>,
	}

	impl From<&AuthorizedDefaultQos> for AuthorizedDefaultQos {
		fn from(value: &AuthorizedDefaultQos) -> Self {
			value.clone()
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

	/// CalledIdentityChange
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "calledIdentity": {
	///      "type": "string"
	///    },
	///    "changeTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CalledIdentityChange {
		#[serde(
			rename = "calledIdentity",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub called_identity: Option<String>,
		#[serde(
			rename = "changeTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub change_time: Option<DateTime>,
	}

	impl From<&CalledIdentityChange> for CalledIdentityChange {
		fn from(value: &CalledIdentityChange) -> Self {
			value.clone()
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

	/// ChargingCharacteristicsSelectionMode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "HOME_DEFAULT",
	///    "ROAMING_DEFAULT",
	///    "VISITING_DEFAULT"
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
	pub enum ChargingCharacteristicsSelectionMode {
		#[default]
		#[serde(rename = "HOME_DEFAULT")]
		HomeDefault,
		#[serde(rename = "ROAMING_DEFAULT")]
		RoamingDefault,
		#[serde(rename = "VISITING_DEFAULT")]
		VisitingDefault,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ChargingCharacteristicsSelectionMode> for ChargingCharacteristicsSelectionMode {
		fn from(value: &ChargingCharacteristicsSelectionMode) -> Self {
			value.clone()
		}
	}

	impl ToString for ChargingCharacteristicsSelectionMode {
		fn to_string(&self) -> String {
			match *self {
				Self::HomeDefault => "HOME_DEFAULT".to_string(),
				Self::RoamingDefault => "ROAMING_DEFAULT".to_string(),
				Self::VisitingDefault => "VISITING_DEFAULT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ChargingCharacteristicsSelectionMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"HOME_DEFAULT" => Ok(Self::HomeDefault),
				"ROAMING_DEFAULT" => Ok(Self::RoamingDefault),
				"VISITING_DEFAULT" => Ok(Self::VisitingDefault),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ChargingCharacteristicsSelectionMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ChargingCharacteristicsSelectionMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ChargingCharacteristicsSelectionMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ChargingDataRequest
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "invocationSequenceNumber",
	///    "invocationTimeStamp",
	///    "nfConsumerIdentification"
	///  ],
	///  "properties": {
	///    "aMFId": {
	///      "$ref": "#/components/schemas/AmfId"
	///    },
	///    "chargingId": {
	///      "$ref": "#/components/schemas/ChargingId"
	///    },
	///    "directEdgeEnablingServiceChargingInformation": {
	///      "$ref": "#/components/schemas/NEFChargingInformation"
	///    },
	///    "eASDeploymentChargingInformation": {
	///      "$ref": "#/components/schemas/EASDeploymentChargingInformation"
	///    },
	///    "eASProviderIdentifier": {
	///      "type": "string"
	///    },
	///    "easid": {
	///      "type": "string"
	///    },
	///    "edgeInfrastructureUsageChargingInformation'": {
	///      "$ref":
	/// "#/components/schemas/EdgeInfrastructureUsageChargingInformation"
	///    },
	///    "ednid": {
	///      "type": "string"
	///    },
	///    "exposedEdgeEnablingServiceChargingInformation": {
	///      "$ref": "#/components/schemas/NEFChargingInformation"
	///    },
	///    "iMSChargingInformation": {
	///      "$ref": "#/components/schemas/IMSChargingInformation"
	///    },
	///    "invocationSequenceNumber": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "invocationTimeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "locationReportingChargingInformation": {
	///      "$ref": "#/components/schemas/LocationReportingChargingInformation"
	///    },
	///    "mMTelChargingInformation": {
	///      "$ref": "#/components/schemas/MMTelChargingInformation"
	///    },
	///    "mnSConsumerIdentifier": {
	///      "type": "string"
	///    },
	///    "multipleUnitUsage": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MultipleUnitUsage"
	///      },
	///      "minItems": 0
	///    },
	///    "n2ConnectionChargingInformation": {
	///      "$ref": "#/components/schemas/N2ConnectionChargingInformation"
	///    },
	///    "nEFChargingInformation": {
	///      "$ref": "#/components/schemas/NEFChargingInformation"
	///    },
	///    "nSMChargingInformation": {
	///      "$ref": "#/components/schemas/NSMChargingInformation"
	///    },
	///    "nSPAChargingInformation": {
	///      "$ref": "#/components/schemas/NSPAChargingInformation"
	///    },
	///    "nfConsumerIdentification": {
	///      "$ref": "#/components/schemas/NFIdentification"
	///    },
	///    "notifyUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "oneTimeEvent": {
	///      "type": "boolean"
	///    },
	///    "oneTimeEventType": {
	///      "$ref": "#/components/schemas/oneTimeEventType"
	///    },
	///    "pDUSessionChargingInformation": {
	///      "$ref": "#/components/schemas/PDUSessionChargingInformation"
	///    },
	///    "proSeChargingInformation": {
	///      "$ref": "#/components/schemas/ProseChargingInformation"
	///    },
	///    "registrationChargingInformation": {
	///      "$ref": "#/components/schemas/RegistrationChargingInformation"
	///    },
	///    "retransmissionIndicator": {
	///      "type": "boolean"
	///    },
	///    "roamingQBCInformation": {
	///      "$ref": "#/components/schemas/RoamingQBCInformation"
	///    },
	///    "sMSChargingInformation": {
	///      "$ref": "#/components/schemas/SMSChargingInformation"
	///    },
	///    "serviceSpecificationInfo": {
	///      "type": "string"
	///    },
	///    "subscriberIdentifier": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "tenantIdentifier": {
	///      "type": "string"
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
	pub struct ChargingDataRequest {
		#[serde(rename = "aMFId", default, skip_serializing_if = "Option::is_none")]
		pub a_mf_id: Option<AmfId>,
		#[serde(
			rename = "chargingId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub charging_id: Option<ChargingId>,
		#[serde(
			rename = "directEdgeEnablingServiceChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub direct_edge_enabling_service_charging_information: Option<NefChargingInformation>,
		#[serde(
			rename = "eASDeploymentChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub e_as_deployment_charging_information: Option<EasDeploymentChargingInformation>,
		#[serde(
			rename = "eASProviderIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub e_as_provider_identifier: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub easid: Option<String>,
		#[serde(
			rename = "edgeInfrastructureUsageChargingInformation'",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub edge_infrastructure_usage_charging_information:
			Option<EdgeInfrastructureUsageChargingInformation>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ednid: Option<String>,
		#[serde(
			rename = "exposedEdgeEnablingServiceChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub exposed_edge_enabling_service_charging_information: Option<NefChargingInformation>,
		#[serde(
			rename = "iMSChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub i_ms_charging_information: Option<ImsChargingInformation>,
		#[serde(rename = "invocationSequenceNumber")]
		pub invocation_sequence_number: Uint32,
		#[serde(rename = "invocationTimeStamp")]
		pub invocation_time_stamp: DateTime,
		#[serde(
			rename = "locationReportingChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub location_reporting_charging_information: Option<LocationReportingChargingInformation>,
		#[serde(
			rename = "mMTelChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub m_m_tel_charging_information: Option<MmTelChargingInformation>,
		#[serde(
			rename = "mnSConsumerIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mn_s_consumer_identifier: Option<String>,
		#[serde(
			rename = "multipleUnitUsage",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub multiple_unit_usage: Vec<MultipleUnitUsage>,
		#[serde(
			rename = "n2ConnectionChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_connection_charging_information: Option<N2ConnectionChargingInformation>,
		#[serde(
			rename = "nEFChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n_ef_charging_information: Option<NefChargingInformation>,
		#[serde(
			rename = "nSMChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n_sm_charging_information: Option<NsmChargingInformation>,
		#[serde(
			rename = "nSPAChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n_spa_charging_information: Option<NspaChargingInformation>,
		#[serde(rename = "nfConsumerIdentification")]
		pub nf_consumer_identification: NfIdentification,
		#[serde(rename = "notifyUri", default, skip_serializing_if = "Option::is_none")]
		pub notify_uri: Option<Uri>,
		#[serde(
			rename = "oneTimeEvent",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub one_time_event: Option<bool>,
		#[serde(
			rename = "oneTimeEventType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub one_time_event_type: Option<OneTimeEventType>,
		#[serde(
			rename = "pDUSessionChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub p_du_session_charging_information: Option<PduSessionChargingInformation>,
		#[serde(
			rename = "proSeChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pro_se_charging_information: Option<ProseChargingInformation>,
		#[serde(
			rename = "registrationChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub registration_charging_information: Option<RegistrationChargingInformation>,
		#[serde(
			rename = "retransmissionIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub retransmission_indicator: Option<bool>,
		#[serde(
			rename = "roamingQBCInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub roaming_qbc_information: Option<RoamingQbcInformation>,
		#[serde(
			rename = "sMSChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_ms_charging_information: Option<SmsChargingInformation>,
		#[serde(
			rename = "serviceSpecificationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_specification_info: Option<String>,
		#[serde(
			rename = "subscriberIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscriber_identifier: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "tenantIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tenant_identifier: Option<String>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub triggers: Vec<Trigger>,
	}

	impl From<&ChargingDataRequest> for ChargingDataRequest {
		fn from(value: &ChargingDataRequest) -> Self {
			value.clone()
		}
	}

	/// ChargingDataResponse
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "invocationSequenceNumber",
	///    "invocationTimeStamp"
	///  ],
	///  "properties": {
	///    "invocationResult": {
	///      "$ref": "#/components/schemas/InvocationResult"
	///    },
	///    "invocationSequenceNumber": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "invocationTimeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "locationReportingChargingInformation": {
	///      "$ref": "#/components/schemas/LocationReportingChargingInformation"
	///    },
	///    "multipleUnitInformation": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MultipleUnitInformation"
	///      },
	///      "minItems": 0
	///    },
	///    "pDUSessionChargingInformation": {
	///      "$ref": "#/components/schemas/PDUSessionChargingInformation"
	///    },
	///    "roamingQBCInformation": {
	///      "$ref": "#/components/schemas/RoamingQBCInformation"
	///    },
	///    "sessionFailover": {
	///      "$ref": "#/components/schemas/SessionFailover"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
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
	pub struct ChargingDataResponse {
		#[serde(
			rename = "invocationResult",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub invocation_result: Option<InvocationResult>,
		#[serde(rename = "invocationSequenceNumber")]
		pub invocation_sequence_number: Uint32,
		#[serde(rename = "invocationTimeStamp")]
		pub invocation_time_stamp: DateTime,
		#[serde(
			rename = "locationReportingChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub location_reporting_charging_information: Option<LocationReportingChargingInformation>,
		#[serde(
			rename = "multipleUnitInformation",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub multiple_unit_information: Vec<MultipleUnitInformation>,
		#[serde(
			rename = "pDUSessionChargingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub p_du_session_charging_information: Option<PduSessionChargingInformation>,
		#[serde(
			rename = "roamingQBCInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub roaming_qbc_information: Option<RoamingQbcInformation>,
		#[serde(
			rename = "sessionFailover",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub session_failover: Option<SessionFailover>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub triggers: Vec<Trigger>,
	}

	impl From<&ChargingDataResponse> for ChargingDataResponse {
		fn from(value: &ChargingDataResponse) -> Self {
			value.clone()
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

	/// ChargingNotifyRequest
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "notificationType"
	///  ],
	///  "properties": {
	///    "notificationType": {
	///      "$ref": "#/components/schemas/NotificationType"
	///    },
	///    "reauthorizationDetails": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ReauthorizationDetails"
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
	pub struct ChargingNotifyRequest {
		#[serde(rename = "notificationType")]
		pub notification_type: NotificationType,
		#[serde(
			rename = "reauthorizationDetails",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub reauthorization_details: Vec<ReauthorizationDetails>,
	}

	impl From<&ChargingNotifyRequest> for ChargingNotifyRequest {
		fn from(value: &ChargingNotifyRequest) -> Self {
			value.clone()
		}
	}

	/// ChargingNotifyResponse
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "invocationResult": {
	///      "$ref": "#/components/schemas/InvocationResult"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ChargingNotifyResponse {
		#[serde(
			rename = "invocationResult",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub invocation_result: Option<InvocationResult>,
	}

	impl From<&ChargingNotifyResponse> for ChargingNotifyResponse {
		fn from(value: &ChargingNotifyResponse) -> Self {
			value.clone()
		}
	}

	/// ClassIdentifier
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "PERSONAL",
	///    "ADVERTISEMENT",
	///    "INFORMATIONAL",
	///    "AUTO"
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
	pub enum ClassIdentifier {
		#[default]
		#[serde(rename = "PERSONAL")]
		Personal,
		#[serde(rename = "ADVERTISEMENT")]
		Advertisement,
		#[serde(rename = "INFORMATIONAL")]
		Informational,
		#[serde(rename = "AUTO")]
		Auto,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ClassIdentifier> for ClassIdentifier {
		fn from(value: &ClassIdentifier) -> Self {
			value.clone()
		}
	}

	impl ToString for ClassIdentifier {
		fn to_string(&self) -> String {
			match *self {
				Self::Personal => "PERSONAL".to_string(),
				Self::Advertisement => "ADVERTISEMENT".to_string(),
				Self::Informational => "INFORMATIONAL".to_string(),
				Self::Auto => "AUTO".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ClassIdentifier {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PERSONAL" => Ok(Self::Personal),
				"ADVERTISEMENT" => Ok(Self::Advertisement),
				"INFORMATIONAL" => Ok(Self::Informational),
				"AUTO" => Ok(Self::Auto),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ClassIdentifier {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ClassIdentifier {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ClassIdentifier {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// CoverageInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "changeTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "coverageStatus": {
	///      "type": "boolean"
	///    },
	///    "locationInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UserLocation"
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
	pub struct CoverageInfo {
		#[serde(
			rename = "changeTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub change_time: Option<DateTime>,
		#[serde(
			rename = "coverageStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub coverage_status: Option<bool>,
		#[serde(
			rename = "locationInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub location_info: Vec<UserLocation>,
	}

	impl From<&CoverageInfo> for CoverageInfo {
		fn from(value: &CoverageInfo) -> Self {
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

	/// DeliveryReportRequested
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "YES",
	///    "NO"
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
	pub enum DeliveryReportRequested {
		#[default]
		#[serde(rename = "YES")]
		Yes,
		#[serde(rename = "NO")]
		No,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DeliveryReportRequested> for DeliveryReportRequested {
		fn from(value: &DeliveryReportRequested) -> Self {
			value.clone()
		}
	}

	impl ToString for DeliveryReportRequested {
		fn to_string(&self) -> String {
			match *self {
				Self::Yes => "YES".to_string(),
				Self::No => "NO".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DeliveryReportRequested {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"YES" => Ok(Self::Yes),
				"NO" => Ok(Self::No),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DeliveryReportRequested {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DeliveryReportRequested {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DeliveryReportRequested {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Diagnostics
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "integer"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Diagnostics(pub i64);

	impl ::std::ops::Deref for Diagnostics {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<Diagnostics> for i64 {
		fn from(value: Diagnostics) -> Self {
			value.0
		}
	}

	impl From<&Diagnostics> for Diagnostics {
		fn from(value: &Diagnostics) -> Self {
			value.clone()
		}
	}

	impl From<i64> for Diagnostics {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Diagnostics {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Diagnostics {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Diagnostics {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Diagnostics {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Diagnostics {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// DirectDiscoveryModel
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "MODEL_A",
	///    "MODEL_B"
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
	pub enum DirectDiscoveryModel {
		#[default]
		#[serde(rename = "MODEL_A")]
		ModelA,
		#[serde(rename = "MODEL_B")]
		ModelB,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DirectDiscoveryModel> for DirectDiscoveryModel {
		fn from(value: &DirectDiscoveryModel) -> Self {
			value.clone()
		}
	}

	impl ToString for DirectDiscoveryModel {
		fn to_string(&self) -> String {
			match *self {
				Self::ModelA => "MODEL_A".to_string(),
				Self::ModelB => "MODEL_B".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DirectDiscoveryModel {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MODEL_A" => Ok(Self::ModelA),
				"MODEL_B" => Ok(Self::ModelB),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DirectDiscoveryModel {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DirectDiscoveryModel {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DirectDiscoveryModel {
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

	/// DnnSelectionMode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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

	/// E164
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9a-fA-F]+$"
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
	pub struct E164(String);

	impl ::std::ops::Deref for E164 {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<E164> for String {
		fn from(value: E164) -> Self {
			value.0
		}
	}

	impl From<&E164> for E164 {
		fn from(value: &E164) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for E164 {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9a-fA-F]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9a-fA-F]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for E164 {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for E164 {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for E164 {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for E164 {
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

	/// EarlyMediaDescription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "sDPMediaComponent": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SDPMediaComponent"
	///      },
	///      "minItems": 0
	///    },
	///    "sDPSessionDescription": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 0
	///    },
	///    "sDPTimeStamps": {
	///      "$ref": "#/components/schemas/SDPTimeStamps"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EarlyMediaDescription {
		#[serde(
			rename = "sDPMediaComponent",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub s_dp_media_component: Vec<SdpMediaComponent>,
		#[serde(
			rename = "sDPSessionDescription",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub s_dp_session_description: Vec<String>,
		#[serde(
			rename = "sDPTimeStamps",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_dp_time_stamps: Option<SdpTimeStamps>,
	}

	impl From<&EarlyMediaDescription> for EarlyMediaDescription {
		fn from(value: &EarlyMediaDescription) -> Self {
			value.clone()
		}
	}

	/// EasDeploymentChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "eEASDeploymentRequirements": {
	///      "$ref": "#/components/schemas/EASRequirements"
	///    },
	///    "lCMEndTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "lCMEventType": {
	///      "$ref": "#/components/schemas/ManagementOperation"
	///    },
	///    "lCMStartTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EasDeploymentChargingInformation {
		#[serde(
			rename = "eEASDeploymentRequirements",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub e_eas_deployment_requirements: Option<EasRequirements>,
		#[serde(
			rename = "lCMEndTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub l_cm_end_time: Option<DateTime>,
		#[serde(
			rename = "lCMEventType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub l_cm_event_type: Option<ManagementOperation>,
		#[serde(
			rename = "lCMStartTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub l_cm_start_time: Option<DateTime>,
	}

	impl From<&EasDeploymentChargingInformation> for EasDeploymentChargingInformation {
		fn from(value: &EasDeploymentChargingInformation) -> Self {
			value.clone()
		}
	}

	/// EasRequirements
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "affinityAntiAffinity": {
	///      "$ref": "#/components/schemas/AffinityAntiAffinity"
	///    },
	///    "requiredEASservingLocation": {
	///      "$ref": "#/components/schemas/ServingLocation"
	///    },
	///    "serviceContinuity": {
	///      "type": "boolean"
	///    },
	///    "softwareImageInfo": {
	///      "$ref": "#/components/schemas/SoftwareImageInfo"
	///    },
	///    "virtualResource": {
	///      "$ref": "#/components/schemas/VirtualResource"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EasRequirements {
		#[serde(
			rename = "affinityAntiAffinity",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub affinity_anti_affinity: Option<AffinityAntiAffinity>,
		#[serde(
			rename = "requiredEASservingLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub required_ea_sserving_location: Option<ServingLocation>,
		#[serde(
			rename = "serviceContinuity",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_continuity: Option<bool>,
		#[serde(
			rename = "softwareImageInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub software_image_info: Option<SoftwareImageInfo>,
		#[serde(
			rename = "virtualResource",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub virtual_resource: Option<VirtualResource>,
	}

	impl From<&EasRequirements> for EasRequirements {
		fn from(value: &EasRequirements) -> Self {
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

	/// EdgeInfrastructureUsageChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "durationEndTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "durationStartTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "meanVirtualCPUUsage": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "meanVirtualDiskUsage": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "meanVirtualMemoryUsage": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "measuredInBytes": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "measuredOutBytes": {
	///      "$ref": "#/components/schemas/Uint64"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EdgeInfrastructureUsageChargingInformation {
		#[serde(
			rename = "durationEndTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub duration_end_time: Option<DateTime>,
		#[serde(
			rename = "durationStartTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub duration_start_time: Option<DateTime>,
		#[serde(
			rename = "meanVirtualCPUUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mean_virtual_cpu_usage: Option<Float>,
		#[serde(
			rename = "meanVirtualDiskUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mean_virtual_disk_usage: Option<Float>,
		#[serde(
			rename = "meanVirtualMemoryUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mean_virtual_memory_usage: Option<Float>,
		#[serde(
			rename = "measuredInBytes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub measured_in_bytes: Option<Uint64>,
		#[serde(
			rename = "measuredOutBytes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub measured_out_bytes: Option<Uint64>,
	}

	impl From<&EdgeInfrastructureUsageChargingInformation>
		for EdgeInfrastructureUsageChargingInformation
	{
		fn from(value: &EdgeInfrastructureUsageChargingInformation) -> Self {
			value.clone()
		}
	}

	/// EnhancedDiagnostics5G
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "$ref": "#/components/schemas/RanNasCauseList"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EnhancedDiagnostics5G(pub RanNasCauseList);

	impl ::std::ops::Deref for EnhancedDiagnostics5G {
		type Target = RanNasCauseList;
		fn deref(&self) -> &RanNasCauseList {
			&self.0
		}
	}

	impl From<EnhancedDiagnostics5G> for RanNasCauseList {
		fn from(value: EnhancedDiagnostics5G) -> Self {
			value.0
		}
	}

	impl From<&EnhancedDiagnostics5G> for EnhancedDiagnostics5G {
		fn from(value: &EnhancedDiagnostics5G) -> Self {
			value.clone()
		}
	}

	impl From<RanNasCauseList> for EnhancedDiagnostics5G {
		fn from(value: RanNasCauseList) -> Self {
			Self(value)
		}
	}

	/// Defines the EPS RAN/NAS release cause.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Defines the EPS RAN/NAS release cause.",
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
	pub struct EpsRanNasRelCause(pub String);

	impl ::std::ops::Deref for EpsRanNasRelCause {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<EpsRanNasRelCause> for String {
		fn from(value: EpsRanNasRelCause) -> Self {
			value.0
		}
	}

	impl From<&EpsRanNasRelCause> for EpsRanNasRelCause {
		fn from(value: &EpsRanNasRelCause) -> Self {
			value.clone()
		}
	}

	impl From<String> for EpsRanNasRelCause {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for EpsRanNasRelCause {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for EpsRanNasRelCause {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// FailureHandling
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "TERMINATE",
	///    "CONTINUE",
	///    "RETRY_AND_TERMINATE"
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
	pub enum FailureHandling {
		#[default]
		#[serde(rename = "TERMINATE")]
		Terminate,
		#[serde(rename = "CONTINUE")]
		Continue,
		#[serde(rename = "RETRY_AND_TERMINATE")]
		RetryAndTerminate,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&FailureHandling> for FailureHandling {
		fn from(value: &FailureHandling) -> Self {
			value.clone()
		}
	}

	impl ToString for FailureHandling {
		fn to_string(&self) -> String {
			match *self {
				Self::Terminate => "TERMINATE".to_string(),
				Self::Continue => "CONTINUE".to_string(),
				Self::RetryAndTerminate => "RETRY_AND_TERMINATE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for FailureHandling {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TERMINATE" => Ok(Self::Terminate),
				"CONTINUE" => Ok(Self::Continue),
				"RETRY_AND_TERMINATE" => Ok(Self::RetryAndTerminate),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for FailureHandling {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for FailureHandling {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for FailureHandling {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// FinalUnitAction
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "TERMINATE",
	///    "REDIRECT",
	///    "RESTRICT_ACCESS"
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
	pub enum FinalUnitAction {
		#[default]
		#[serde(rename = "TERMINATE")]
		Terminate,
		#[serde(rename = "REDIRECT")]
		Redirect,
		#[serde(rename = "RESTRICT_ACCESS")]
		RestrictAccess,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&FinalUnitAction> for FinalUnitAction {
		fn from(value: &FinalUnitAction) -> Self {
			value.clone()
		}
	}

	impl ToString for FinalUnitAction {
		fn to_string(&self) -> String {
			match *self {
				Self::Terminate => "TERMINATE".to_string(),
				Self::Redirect => "REDIRECT".to_string(),
				Self::RestrictAccess => "RESTRICT_ACCESS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for FinalUnitAction {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TERMINATE" => Ok(Self::Terminate),
				"REDIRECT" => Ok(Self::Redirect),
				"RESTRICT_ACCESS" => Ok(Self::RestrictAccess),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for FinalUnitAction {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for FinalUnitAction {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for FinalUnitAction {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// FinalUnitIndication
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "finalUnitAction"
	///  ],
	///  "properties": {
	///    "filterId": {
	///      "type": "string"
	///    },
	///    "filterIdList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "finalUnitAction": {
	///      "$ref": "#/components/schemas/FinalUnitAction"
	///    },
	///    "redirectServer": {
	///      "$ref": "#/components/schemas/RedirectServer"
	///    },
	///    "restrictionFilterRule": {
	///      "$ref": "#/components/schemas/IPFilterRule"
	///    },
	///    "restrictionFilterRuleList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IPFilterRule"
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
	pub struct FinalUnitIndication {
		#[serde(rename = "filterId", default, skip_serializing_if = "Option::is_none")]
		pub filter_id: Option<String>,
		#[serde(
			rename = "filterIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub filter_id_list: Vec<String>,
		#[serde(rename = "finalUnitAction")]
		pub final_unit_action: FinalUnitAction,
		#[serde(
			rename = "redirectServer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub redirect_server: Option<RedirectServer>,
		#[serde(
			rename = "restrictionFilterRule",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub restriction_filter_rule: Option<IpFilterRule>,
		#[serde(
			rename = "restrictionFilterRuleList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub restriction_filter_rule_list: Vec<IpFilterRule>,
	}

	impl From<&FinalUnitIndication> for FinalUnitIndication {
		fn from(value: &FinalUnitIndication) -> Self {
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

	/// GeoLoc
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "civicLocation": {
	///      "type": "string"
	///    },
	///    "geographicalCoordinates": {
	///      "$ref": "#/components/schemas/GeographicalCoordinates"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GeoLoc {
		#[serde(
			rename = "civicLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub civic_location: Option<String>,
		#[serde(
			rename = "geographicalCoordinates",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub geographical_coordinates: Option<GeographicalCoordinates>,
	}

	impl From<&GeoLoc> for GeoLoc {
		fn from(value: &GeoLoc) -> Self {
			value.clone()
		}
	}

	/// GeographicalCoordinates
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "lattitude": {
	///      "type": "integer"
	///    },
	///    "longitude": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GeographicalCoordinates {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub lattitude: Option<i64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub longitude: Option<i64>,
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

	/// GrantedUnit
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "serviceSpecificUnits": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "tariffTimeChange": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "time": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "totalVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "uplinkVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GrantedUnit {
		#[serde(
			rename = "downlinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub downlink_volume: Option<Uint64>,
		#[serde(
			rename = "serviceSpecificUnits",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_specific_units: Option<Uint64>,
		#[serde(
			rename = "tariffTimeChange",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tariff_time_change: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub time: Option<Uint32>,
		#[serde(
			rename = "totalVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub total_volume: Option<Uint64>,
		#[serde(
			rename = "uplinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uplink_volume: Option<Uint64>,
	}

	impl From<&GrantedUnit> for GrantedUnit {
		fn from(value: &GrantedUnit) -> Self {
			value.clone()
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

	/// ImsAddress
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	///    },
	///    {
	///      "required": [
	///        "e164"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "e164": {
	///      "$ref": "#/components/schemas/E164"
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
	#[serde(untagged)]
	pub enum ImsAddress {
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
			e164: E164,
		},
	}

	impl From<&ImsAddress> for ImsAddress {
		fn from(value: &ImsAddress) -> Self {
			value.clone()
		}
	}

	/// ImsChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "3gppPSDataOffStatus": {
	///      "$ref": "#/components/schemas/3GPPPSDataOffStatus"
	///    },
	///    "accessNetworkInfoChange": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessNetworkInfoChange"
	///      },
	///      "minItems": 1
	///    },
	///    "accessNetworkInformation": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "accessTransferInformation": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessTransferInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "additionalAccessNetworkInformation": {
	///      "type": "string"
	///    },
	///    "alternateChargedPartyAddress": {
	///      "type": "string"
	///    },
	///    "applicationServerInformation": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "associatedURI": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uri"
	///      },
	///      "minItems": 1
	///    },
	///    "bearerService": {
	///      "type": "string"
	///    },
	///    "calledAssertedIdentities": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "calledIdentityChanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CalledIdentityChange"
	///      },
	///      "minItems": 1
	///    },
	///    "calledPartyAddress": {
	///      "type": "string"
	///    },
	///    "callingPartyAddresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uri"
	///      },
	///      "minItems": 1
	///    },
	///    "carrierSelectRoutingInformation": {
	///      "type": "string"
	///    },
	///    "causeCode": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "cellularNetworkInformation": {
	///      "type": "string"
	///    },
	///    "controlPlaneAddress": {
	///      "$ref": "#/components/schemas/IMSAddress"
	///    },
	///    "earlyMediaDescription": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EarlyMediaDescription"
	///      },
	///      "minItems": 1
	///    },
	///    "eventType": {
	///      "$ref": "#/components/schemas/SIPEventType"
	///    },
	///    "feIdentifierList": {
	///      "type": "string"
	///    },
	///    "fromAddress": {
	///      "type": "string"
	///    },
	///    "iMSNodeFunctionality": {
	///      "$ref": "#/components/schemas/IMSNodeFunctionality"
	///    },
	///    "imsApplicationReferenceID": {
	///      "type": "string"
	///    },
	///    "imsChargingIdentifier": {
	///      "type": "string"
	///    },
	///    "imsCommunicationServiceID": {
	///      "type": "string"
	///    },
	///    "imsEmergencyIndication": {
	///      "type": "boolean"
	///    },
	///    "imsServiceId": {
	///      "type": "string"
	///    },
	///    "imsVisitedNetworkIdentifier": {
	///      "type": "string"
	///    },
	///    "initialIMSChargingIdentifier": {
	///      "type": "string"
	///    },
	///    "interOperatorIdentifier": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/InterOperatorIdentifier"
	///      },
	///      "minItems": 1
	///    },
	///    "isupCause": {
	///      "$ref": "#/components/schemas/ISUPCause"
	///    },
	///    "messageBodies": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MessageBody"
	///      },
	///      "minItems": 1
	///    },
	///    "mscAddress": {
	///      "$ref": "#/components/schemas/E164"
	///    },
	///    "nniInformation": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NNIInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "numberPortabilityRoutinginformation": {
	///      "type": "string"
	///    },
	///    "outgoingSessionID": {
	///      "type": "string"
	///    },
	///    "reasonHeader": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "relatedICID": {
	///      "type": "string"
	///    },
	///    "relatedICIDGenerationNode": {
	///      "type": "string"
	///    },
	///    "requestedPartyAddress": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "roleOfNode": {
	///      "$ref": "#/components/schemas/RoleOfIMSNode"
	///    },
	///    "sdpMediaComponent": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SDPMediaComponent"
	///      },
	///      "minItems": 1
	///    },
	///    "sdpSessionDescription": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "servedPartyIPAddress": {
	///      "$ref": "#/components/schemas/IMSAddress"
	///    },
	///    "serverCapabilities": {
	///      "$ref": "#/components/schemas/ServerCapabilities"
	///    },
	///    "sessionPriority": {
	///      "$ref": "#/components/schemas/IMSSessionPriority"
	///    },
	///    "sipRouteHeaderReceived": {
	///      "type": "string"
	///    },
	///    "sipRouteHeaderTransmitted": {
	///      "type": "string"
	///    },
	///    "tadIdentifier": {
	///      "$ref": "#/components/schemas/TADIdentifier"
	///    },
	///    "timeStamps": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "transitIOIList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "trunkGroupID": {
	///      "$ref": "#/components/schemas/TrunkGroupID"
	///    },
	///    "ueTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userInformation": {
	///      "$ref": "#/components/schemas/UserInformation"
	///    },
	///    "userLocationInfo": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "userSessionID": {
	///      "type": "string"
	///    },
	///    "vlrNumber": {
	///      "$ref": "#/components/schemas/E164"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ImsChargingInformation {
		#[serde(
			rename = "accessNetworkInfoChange",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub access_network_info_change: Vec<AccessNetworkInfoChange>,
		#[serde(
			rename = "accessNetworkInformation",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub access_network_information: Vec<String>,
		#[serde(
			rename = "accessTransferInformation",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub access_transfer_information: Vec<AccessTransferInformation>,
		#[serde(
			rename = "additionalAccessNetworkInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_access_network_information: Option<String>,
		#[serde(
			rename = "alternateChargedPartyAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub alternate_charged_party_address: Option<String>,
		#[serde(
			rename = "applicationServerInformation",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub application_server_information: Vec<String>,
		#[serde(
			rename = "associatedURI",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub associated_uri: Vec<Uri>,
		#[serde(
			rename = "bearerService",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub bearer_service: Option<String>,
		#[serde(
			rename = "calledAssertedIdentities",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub called_asserted_identities: Vec<String>,
		#[serde(
			rename = "calledIdentityChanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub called_identity_changes: Vec<CalledIdentityChange>,
		#[serde(
			rename = "calledPartyAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub called_party_address: Option<String>,
		#[serde(
			rename = "callingPartyAddresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub calling_party_addresses: Vec<Uri>,
		#[serde(
			rename = "carrierSelectRoutingInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub carrier_select_routing_information: Option<String>,
		#[serde(rename = "causeCode", default, skip_serializing_if = "Option::is_none")]
		pub cause_code: Option<Uint32>,
		#[serde(
			rename = "cellularNetworkInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cellular_network_information: Option<String>,
		#[serde(
			rename = "controlPlaneAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub control_plane_address: Option<ImsAddress>,
		#[serde(
			rename = "earlyMediaDescription",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub early_media_description: Vec<EarlyMediaDescription>,
		#[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
		pub event_type: Option<SipEventType>,
		#[serde(
			rename = "feIdentifierList",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub fe_identifier_list: Option<String>,
		#[serde(
			rename = "fromAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub from_address: Option<String>,
		#[serde(
			rename = "iMSNodeFunctionality",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub i_ms_node_functionality: Option<ImsNodeFunctionality>,
		#[serde(
			rename = "imsApplicationReferenceID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ims_application_reference_id: Option<String>,
		#[serde(
			rename = "imsChargingIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ims_charging_identifier: Option<String>,
		#[serde(
			rename = "imsCommunicationServiceID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ims_communication_service_id: Option<String>,
		#[serde(
			rename = "imsEmergencyIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ims_emergency_indication: Option<bool>,
		#[serde(
			rename = "imsServiceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ims_service_id: Option<String>,
		#[serde(
			rename = "imsVisitedNetworkIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ims_visited_network_identifier: Option<String>,
		#[serde(
			rename = "initialIMSChargingIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub initial_ims_charging_identifier: Option<String>,
		#[serde(
			rename = "interOperatorIdentifier",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub inter_operator_identifier: Vec<InterOperatorIdentifier>,
		#[serde(rename = "isupCause", default, skip_serializing_if = "Option::is_none")]
		pub isup_cause: Option<IsupCause>,
		#[serde(
			rename = "messageBodies",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub message_bodies: Vec<MessageBody>,
		#[serde(
			rename = "mscAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub msc_address: Option<E164>,
		#[serde(
			rename = "nniInformation",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nni_information: Vec<NniInformation>,
		#[serde(
			rename = "numberPortabilityRoutinginformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub number_portability_routinginformation: Option<String>,
		#[serde(
			rename = "outgoingSessionID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub outgoing_session_id: Option<String>,
		#[serde(
			rename = "reasonHeader",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub reason_header: Vec<String>,
		#[serde(
			rename = "relatedICID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub related_icid: Option<String>,
		#[serde(
			rename = "relatedICIDGenerationNode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub related_icid_generation_node: Option<String>,
		#[serde(
			rename = "requestedPartyAddress",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub requested_party_address: Vec<String>,
		#[serde(
			rename = "roleOfNode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub role_of_node: Option<RoleOfImsNode>,
		#[serde(
			rename = "sdpMediaComponent",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub sdp_media_component: Vec<SdpMediaComponent>,
		#[serde(
			rename = "sdpSessionDescription",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub sdp_session_description: Vec<String>,
		#[serde(
			rename = "servedPartyIPAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub served_party_ip_address: Option<ImsAddress>,
		#[serde(
			rename = "serverCapabilities",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub server_capabilities: Option<ServerCapabilities>,
		#[serde(
			rename = "sessionPriority",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub session_priority: Option<ImsSessionPriority>,
		#[serde(
			rename = "sipRouteHeaderReceived",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sip_route_header_received: Option<String>,
		#[serde(
			rename = "sipRouteHeaderTransmitted",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sip_route_header_transmitted: Option<String>,
		#[serde(
			rename = "tadIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tad_identifier: Option<TadIdentifier>,
		#[serde(
			rename = "3gppPSDataOffStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gpp_ps_data_off_status: Option<_3gpppsDataOffStatus>,
		#[serde(
			rename = "timeStamps",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_stamps: Option<DateTime>,
		#[serde(
			rename = "transitIOIList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub transit_ioi_list: Vec<String>,
		#[serde(
			rename = "trunkGroupID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub trunk_group_id: Option<TrunkGroupId>,
		#[serde(
			rename = "ueTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_time_zone: Option<TimeZone>,
		#[serde(
			rename = "userInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_information: Option<UserInformation>,
		#[serde(
			rename = "userLocationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location_info: Option<UserLocation>,
		#[serde(
			rename = "userSessionID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_session_id: Option<String>,
		#[serde(rename = "vlrNumber", default, skip_serializing_if = "Option::is_none")]
		pub vlr_number: Option<E164>,
	}

	impl From<&ImsChargingInformation> for ImsChargingInformation {
		fn from(value: &ImsChargingInformation) -> Self {
			value.clone()
		}
	}

	/// ImsNodeFunctionality
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "S_CSCF",
	///    "P_CSCF",
	///    "I_CSCF",
	///    "MRFC",
	///    "MGCF",
	///    "BGCF",
	///    "AS",
	///    "IBCF",
	///    "S-GW",
	///    "P-GW",
	///    "HSGW",
	///    "E-CSCF",
	///    "MME",
	///    "TRF",
	///    "TF",
	///    "ATCF",
	///    "PROXY",
	///    "EPDG",
	///    "TDF",
	///    "TWAG",
	///    "SCEF",
	///    "IWK_SCEF",
	///    "IMS_GWF"
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
	pub enum ImsNodeFunctionality {
		#[default]
		#[serde(rename = "S_CSCF")]
		SCscf,
		#[serde(rename = "P_CSCF")]
		PCscf,
		#[serde(rename = "I_CSCF")]
		ICscf,
		#[serde(rename = "MRFC")]
		Mrfc,
		#[serde(rename = "MGCF")]
		Mgcf,
		#[serde(rename = "BGCF")]
		Bgcf,
		#[serde(rename = "AS")]
		As,
		#[serde(rename = "IBCF")]
		Ibcf,
		#[serde(rename = "S-GW")]
		SGw,
		#[serde(rename = "P-GW")]
		PGw,
		#[serde(rename = "HSGW")]
		Hsgw,
		#[serde(rename = "E-CSCF")]
		ECscf,
		#[serde(rename = "MME")]
		Mme,
		#[serde(rename = "TRF")]
		Trf,
		#[serde(rename = "TF")]
		Tf,
		#[serde(rename = "ATCF")]
		Atcf,
		#[serde(rename = "PROXY")]
		Proxy,
		#[serde(rename = "EPDG")]
		Epdg,
		#[serde(rename = "TDF")]
		Tdf,
		#[serde(rename = "TWAG")]
		Twag,
		#[serde(rename = "SCEF")]
		Scef,
		#[serde(rename = "IWK_SCEF")]
		IwkScef,
		#[serde(rename = "IMS_GWF")]
		ImsGwf,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ImsNodeFunctionality> for ImsNodeFunctionality {
		fn from(value: &ImsNodeFunctionality) -> Self {
			value.clone()
		}
	}

	impl ToString for ImsNodeFunctionality {
		fn to_string(&self) -> String {
			match *self {
				Self::SCscf => "S_CSCF".to_string(),
				Self::PCscf => "P_CSCF".to_string(),
				Self::ICscf => "I_CSCF".to_string(),
				Self::Mrfc => "MRFC".to_string(),
				Self::Mgcf => "MGCF".to_string(),
				Self::Bgcf => "BGCF".to_string(),
				Self::As => "AS".to_string(),
				Self::Ibcf => "IBCF".to_string(),
				Self::SGw => "S-GW".to_string(),
				Self::PGw => "P-GW".to_string(),
				Self::Hsgw => "HSGW".to_string(),
				Self::ECscf => "E-CSCF".to_string(),
				Self::Mme => "MME".to_string(),
				Self::Trf => "TRF".to_string(),
				Self::Tf => "TF".to_string(),
				Self::Atcf => "ATCF".to_string(),
				Self::Proxy => "PROXY".to_string(),
				Self::Epdg => "EPDG".to_string(),
				Self::Tdf => "TDF".to_string(),
				Self::Twag => "TWAG".to_string(),
				Self::Scef => "SCEF".to_string(),
				Self::IwkScef => "IWK_SCEF".to_string(),
				Self::ImsGwf => "IMS_GWF".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ImsNodeFunctionality {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"S_CSCF" => Ok(Self::SCscf),
				"P_CSCF" => Ok(Self::PCscf),
				"I_CSCF" => Ok(Self::ICscf),
				"MRFC" => Ok(Self::Mrfc),
				"MGCF" => Ok(Self::Mgcf),
				"BGCF" => Ok(Self::Bgcf),
				"AS" => Ok(Self::As),
				"IBCF" => Ok(Self::Ibcf),
				"S-GW" => Ok(Self::SGw),
				"P-GW" => Ok(Self::PGw),
				"HSGW" => Ok(Self::Hsgw),
				"E-CSCF" => Ok(Self::ECscf),
				"MME" => Ok(Self::Mme),
				"TRF" => Ok(Self::Trf),
				"TF" => Ok(Self::Tf),
				"ATCF" => Ok(Self::Atcf),
				"PROXY" => Ok(Self::Proxy),
				"EPDG" => Ok(Self::Epdg),
				"TDF" => Ok(Self::Tdf),
				"TWAG" => Ok(Self::Twag),
				"SCEF" => Ok(Self::Scef),
				"IWK_SCEF" => Ok(Self::IwkScef),
				"IMS_GWF" => Ok(Self::ImsGwf),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ImsNodeFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ImsNodeFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ImsNodeFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ImsSessionPriority
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "PRIORITY_0",
	///    "PRIORITY_1",
	///    "PRIORITY_2",
	///    "PRIORITY_3",
	///    "PRIORITY_4"
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
	pub enum ImsSessionPriority {
		#[default]
		#[serde(rename = "PRIORITY_0")]
		Priority0,
		#[serde(rename = "PRIORITY_1")]
		Priority1,
		#[serde(rename = "PRIORITY_2")]
		Priority2,
		#[serde(rename = "PRIORITY_3")]
		Priority3,
		#[serde(rename = "PRIORITY_4")]
		Priority4,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ImsSessionPriority> for ImsSessionPriority {
		fn from(value: &ImsSessionPriority) -> Self {
			value.clone()
		}
	}

	impl ToString for ImsSessionPriority {
		fn to_string(&self) -> String {
			match *self {
				Self::Priority0 => "PRIORITY_0".to_string(),
				Self::Priority1 => "PRIORITY_1".to_string(),
				Self::Priority2 => "PRIORITY_2".to_string(),
				Self::Priority3 => "PRIORITY_3".to_string(),
				Self::Priority4 => "PRIORITY_4".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ImsSessionPriority {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PRIORITY_0" => Ok(Self::Priority0),
				"PRIORITY_1" => Ok(Self::Priority1),
				"PRIORITY_2" => Ok(Self::Priority2),
				"PRIORITY_3" => Ok(Self::Priority3),
				"PRIORITY_4" => Ok(Self::Priority4),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ImsSessionPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ImsSessionPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ImsSessionPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// InterOperatorIdentifier
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "originatingIOI": {
	///      "type": "string"
	///    },
	///    "terminatingIOI": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct InterOperatorIdentifier {
		#[serde(
			rename = "originatingIOI",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub originating_ioi: Option<String>,
		#[serde(
			rename = "terminatingIOI",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub terminating_ioi: Option<String>,
	}

	impl From<&InterOperatorIdentifier> for InterOperatorIdentifier {
		fn from(value: &InterOperatorIdentifier) -> Self {
			value.clone()
		}
	}

	/// InterfaceType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "UNKNOWN",
	///    "MOBILE_ORIGINATING",
	///    "MOBILE_TERMINATING",
	///    "APPLICATION_ORIGINATING",
	///    "APPLICATION_TERMINATING"
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
	pub enum InterfaceType {
		#[default]
		#[serde(rename = "UNKNOWN")]
		Unknown,
		#[serde(rename = "MOBILE_ORIGINATING")]
		MobileOriginating,
		#[serde(rename = "MOBILE_TERMINATING")]
		MobileTerminating,
		#[serde(rename = "APPLICATION_ORIGINATING")]
		ApplicationOriginating,
		#[serde(rename = "APPLICATION_TERMINATING")]
		ApplicationTerminating,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&InterfaceType> for InterfaceType {
		fn from(value: &InterfaceType) -> Self {
			value.clone()
		}
	}

	impl ToString for InterfaceType {
		fn to_string(&self) -> String {
			match *self {
				Self::Unknown => "UNKNOWN".to_string(),
				Self::MobileOriginating => "MOBILE_ORIGINATING".to_string(),
				Self::MobileTerminating => "MOBILE_TERMINATING".to_string(),
				Self::ApplicationOriginating => "APPLICATION_ORIGINATING".to_string(),
				Self::ApplicationTerminating => "APPLICATION_TERMINATING".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for InterfaceType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UNKNOWN" => Ok(Self::Unknown),
				"MOBILE_ORIGINATING" => Ok(Self::MobileOriginating),
				"MOBILE_TERMINATING" => Ok(Self::MobileTerminating),
				"APPLICATION_ORIGINATING" => Ok(Self::ApplicationOriginating),
				"APPLICATION_TERMINATING" => Ok(Self::ApplicationTerminating),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for InterfaceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for InterfaceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for InterfaceType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// InvocationResult
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "error": {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    "failureHandling": {
	///      "$ref": "#/components/schemas/FailureHandling"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct InvocationResult {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub error: Option<ProblemDetails>,
		#[serde(
			rename = "failureHandling",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub failure_handling: Option<FailureHandling>,
	}

	impl From<&InvocationResult> for InvocationResult {
		fn from(value: &InvocationResult) -> Self {
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

	/// IpFilterRule
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub struct IpFilterRule(pub String);

	impl ::std::ops::Deref for IpFilterRule {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<IpFilterRule> for String {
		fn from(value: IpFilterRule) -> Self {
			value.0
		}
	}

	impl From<&IpFilterRule> for IpFilterRule {
		fn from(value: &IpFilterRule) -> Self {
			value.clone()
		}
	}

	impl From<String> for IpFilterRule {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for IpFilterRule {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for IpFilterRule {
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

	/// IsupCause
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "iSUPCauseDiagnostics": {
	///      "$ref": "#/components/schemas/OctetString"
	///    },
	///    "iSUPCauseLocation": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "iSUPCauseValue": {
	///      "$ref": "#/components/schemas/Uint32"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct IsupCause {
		#[serde(
			rename = "iSUPCauseDiagnostics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub i_sup_cause_diagnostics: Option<OctetString>,
		#[serde(
			rename = "iSUPCauseLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub i_sup_cause_location: Option<Uint32>,
		#[serde(
			rename = "iSUPCauseValue",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub i_sup_cause_value: Option<Uint32>,
	}

	impl From<&IsupCause> for IsupCause {
		fn from(value: &IsupCause) -> Self {
			value.clone()
		}
	}

	/// Language
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub struct Language(pub String);

	impl ::std::ops::Deref for Language {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Language> for String {
		fn from(value: Language) -> Self {
			value.0
		}
	}

	impl From<&Language> for Language {
		fn from(value: &Language) -> Self {
			value.clone()
		}
	}

	impl From<String> for Language {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Language {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for Language {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// Load level information of the network slice and the optionally
	/// associated network slice instance.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Load level information of the network slice and the
	/// optionally associated network slice instance.\n",
	///  "type": "integer"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LoadLevelInformation(pub i64);

	impl ::std::ops::Deref for LoadLevelInformation {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<LoadLevelInformation> for i64 {
		fn from(value: LoadLevelInformation) -> Self {
			value.0
		}
	}

	impl From<&LoadLevelInformation> for LoadLevelInformation {
		fn from(value: &LoadLevelInformation) -> Self {
			value.clone()
		}
	}

	impl From<i64> for LoadLevelInformation {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for LoadLevelInformation {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for LoadLevelInformation {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LoadLevelInformation {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LoadLevelInformation {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for LoadLevelInformation {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// Represents UE location information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents UE location information.",
	///  "type": "object",
	///  "required": [
	///    "loc"
	///  ],
	///  "properties": {
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "loc": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "ratio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LocationInfo {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub confidence: Option<Uinteger>,
		pub loc: UserLocation,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ratio: Option<SamplingRatio>,
	}

	impl From<&LocationInfo> for LocationInfo {
		fn from(value: &LocationInfo) -> Self {
			value.clone()
		}
	}

	/// LocationReportingChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "locationReportingMessageType"
	///  ],
	///  "properties": {
	///    "locationReportingMessageType": {
	///      "$ref": "#/components/schemas/LocationReportingMessageType"
	///    },
	///    "pSCellInformation": {
	///      "$ref": "#/components/schemas/PSCellInformation"
	///    },
	///    "presenceReportingAreaInformation": {
	///      "type": "object",
	///      "minProperties": 0,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "rATType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "uetimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userInformation": {
	///      "$ref": "#/components/schemas/UserInformation"
	///    },
	///    "userLocationinfo": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LocationReportingChargingInformation {
		#[serde(rename = "locationReportingMessageType")]
		pub location_reporting_message_type: LocationReportingMessageType,
		#[serde(
			rename = "pSCellInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub p_s_cell_information: Option<PsCellInformation>,
		#[serde(
			rename = "presenceReportingAreaInformation",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub presence_reporting_area_information: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(rename = "rATType", default, skip_serializing_if = "Option::is_none")]
		pub r_at_type: Option<RatType>,
		#[serde(
			rename = "uetimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uetime_zone: Option<TimeZone>,
		#[serde(
			rename = "userInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_information: Option<UserInformation>,
		#[serde(
			rename = "userLocationinfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_locationinfo: Option<UserLocation>,
	}

	impl From<&LocationReportingChargingInformation> for LocationReportingChargingInformation {
		fn from(value: &LocationReportingChargingInformation) -> Self {
			value.clone()
		}
	}

	/// LocationReportingMessageType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "integer"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LocationReportingMessageType(pub i64);

	impl ::std::ops::Deref for LocationReportingMessageType {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<LocationReportingMessageType> for i64 {
		fn from(value: LocationReportingMessageType) -> Self {
			value.0
		}
	}

	impl From<&LocationReportingMessageType> for LocationReportingMessageType {
		fn from(value: &LocationReportingMessageType) -> Self {
			value.clone()
		}
	}

	impl From<i64> for LocationReportingMessageType {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for LocationReportingMessageType {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for LocationReportingMessageType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LocationReportingMessageType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LocationReportingMessageType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for LocationReportingMessageType {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains the MA PDU session indication, i.e., MA PDU Request or MA PDU
	/// Network-Upgrade Allowed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the MA PDU session indication, i.e., MA PDU
	/// Request or MA PDU Network-Upgrade Allowed.",
	///  "type": "string",
	///  "enum": [
	///    "MA_PDU_REQUEST",
	///    "MA_PDU_NETWORK_UPGRADE_ALLOWED"
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
	pub enum MaPduIndication {
		#[default]
		#[serde(rename = "MA_PDU_REQUEST")]
		MaPduRequest,
		#[serde(rename = "MA_PDU_NETWORK_UPGRADE_ALLOWED")]
		MaPduNetworkUpgradeAllowed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MaPduIndication> for MaPduIndication {
		fn from(value: &MaPduIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for MaPduIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::MaPduRequest => "MA_PDU_REQUEST".to_string(),
				Self::MaPduNetworkUpgradeAllowed => "MA_PDU_NETWORK_UPGRADE_ALLOWED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MaPduIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MA_PDU_REQUEST" => Ok(Self::MaPduRequest),
				"MA_PDU_NETWORK_UPGRADE_ALLOWED" => Ok(Self::MaPduNetworkUpgradeAllowed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MaPduIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MaPduIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MaPduIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ManagementOperation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "CreateMOI",
	///    "ModifyMOIAttributes",
	///    "DeleteMOI",
	///    "CREATE_MOI",
	///    "MODIFY_MOI_ATTR",
	///    "DELETE_MOI",
	///    "NOTIFY_MOI_CREATION",
	///    "NOTIFY_MOI_ATTR_CHANGE",
	///    "NOTIFY_MOI_DELETION"
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
	pub enum ManagementOperation {
		#[default]
		#[serde(rename = "CreateMOI")]
		CreateMoi,
		#[serde(rename = "ModifyMOIAttributes")]
		ModifyMoiAttributes,
		#[serde(rename = "DeleteMOI")]
		DeleteMoi,
		#[serde(rename = "CREATE_MOI")]
		CreateMoi29,
		#[serde(rename = "MODIFY_MOI_ATTR")]
		ModifyMoiAttr,
		#[serde(rename = "DELETE_MOI")]
		DeleteMoiE9,
		#[serde(rename = "NOTIFY_MOI_CREATION")]
		NotifyMoiCreation,
		#[serde(rename = "NOTIFY_MOI_ATTR_CHANGE")]
		NotifyMoiAttrChange,
		#[serde(rename = "NOTIFY_MOI_DELETION")]
		NotifyMoiDeletion,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ManagementOperation> for ManagementOperation {
		fn from(value: &ManagementOperation) -> Self {
			value.clone()
		}
	}

	impl ToString for ManagementOperation {
		fn to_string(&self) -> String {
			match *self {
				Self::CreateMoi => "CreateMOI".to_string(),
				Self::ModifyMoiAttributes => "ModifyMOIAttributes".to_string(),
				Self::DeleteMoi => "DeleteMOI".to_string(),
				Self::CreateMoi29 => "CREATE_MOI".to_string(),
				Self::ModifyMoiAttr => "MODIFY_MOI_ATTR".to_string(),
				Self::DeleteMoiE9 => "DELETE_MOI".to_string(),
				Self::NotifyMoiCreation => "NOTIFY_MOI_CREATION".to_string(),
				Self::NotifyMoiAttrChange => "NOTIFY_MOI_ATTR_CHANGE".to_string(),
				Self::NotifyMoiDeletion => "NOTIFY_MOI_DELETION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ManagementOperation {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CreateMOI" => Ok(Self::CreateMoi),
				"ModifyMOIAttributes" => Ok(Self::ModifyMoiAttributes),
				"DeleteMOI" => Ok(Self::DeleteMoi),
				"CREATE_MOI" => Ok(Self::CreateMoi29),
				"MODIFY_MOI_ATTR" => Ok(Self::ModifyMoiAttr),
				"DELETE_MOI" => Ok(Self::DeleteMoiE9),
				"NOTIFY_MOI_CREATION" => Ok(Self::NotifyMoiCreation),
				"NOTIFY_MOI_ATTR_CHANGE" => Ok(Self::NotifyMoiAttrChange),
				"NOTIFY_MOI_DELETION" => Ok(Self::NotifyMoiDeletion),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ManagementOperation {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ManagementOperation {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ManagementOperation {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ManagementOperationStatus
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "OPERATION_SUCCEEDED",
	///    "OPERATION_FAILED"
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
	pub enum ManagementOperationStatus {
		#[default]
		#[serde(rename = "OPERATION_SUCCEEDED")]
		OperationSucceeded,
		#[serde(rename = "OPERATION_FAILED")]
		OperationFailed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ManagementOperationStatus> for ManagementOperationStatus {
		fn from(value: &ManagementOperationStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for ManagementOperationStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::OperationSucceeded => "OPERATION_SUCCEEDED".to_string(),
				Self::OperationFailed => "OPERATION_FAILED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ManagementOperationStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"OPERATION_SUCCEEDED" => Ok(Self::OperationSucceeded),
				"OPERATION_FAILED" => Ok(Self::OperationFailed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ManagementOperationStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ManagementOperationStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ManagementOperationStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// MapduSessionInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "aTSSSCapability": {
	///      "$ref": "#/components/schemas/AtsssCapability"
	///    },
	///    "mAPDUSessionIndicator": {
	///      "$ref": "#/components/schemas/MaPduIndication"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MapduSessionInformation {
		#[serde(
			rename = "aTSSSCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub a_tsss_capability: Option<AtsssCapability>,
		#[serde(
			rename = "mAPDUSessionIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub m_apdu_session_indicator: Option<MaPduIndication>,
	}

	impl From<&MapduSessionInformation> for MapduSessionInformation {
		fn from(value: &MapduSessionInformation) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - ASCENDING: Threshold is crossed in ascending direction.
	/// - DESCENDING: Threshold is crossed in descending direction.
	/// - CROSSED: Threshold is crossed either in ascending or descending
	///   direction.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- ASCENDING: Threshold is crossed
	/// in ascending direction.\n- DESCENDING: Threshold is crossed in
	/// descending direction.\n- CROSSED: Threshold is crossed either in
	/// ascending or descending direction.\n",
	///  "type": "string",
	///  "enum": [
	///    "ASCENDING",
	///    "DESCENDING",
	///    "CROSSED"
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
	pub enum MatchingDirection {
		#[default]
		#[serde(rename = "ASCENDING")]
		Ascending,
		#[serde(rename = "DESCENDING")]
		Descending,
		#[serde(rename = "CROSSED")]
		Crossed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MatchingDirection> for MatchingDirection {
		fn from(value: &MatchingDirection) -> Self {
			value.clone()
		}
	}

	impl ToString for MatchingDirection {
		fn to_string(&self) -> String {
			match *self {
				Self::Ascending => "ASCENDING".to_string(),
				Self::Descending => "DESCENDING".to_string(),
				Self::Crossed => "CROSSED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MatchingDirection {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ASCENDING" => Ok(Self::Ascending),
				"DESCENDING" => Ok(Self::Descending),
				"CROSSED" => Ok(Self::Crossed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MatchingDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MatchingDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MatchingDirection {
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

	/// MediaInitiatorFlag
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "CALLED_PARTY",
	///    "CALLING_PARTY",
	///    "UNKNOWN"
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
	pub enum MediaInitiatorFlag {
		#[default]
		#[serde(rename = "CALLED_PARTY")]
		CalledParty,
		#[serde(rename = "CALLING_PARTY")]
		CallingParty,
		#[serde(rename = "UNKNOWN")]
		Unknown,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MediaInitiatorFlag> for MediaInitiatorFlag {
		fn from(value: &MediaInitiatorFlag) -> Self {
			value.clone()
		}
	}

	impl ToString for MediaInitiatorFlag {
		fn to_string(&self) -> String {
			match *self {
				Self::CalledParty => "CALLED_PARTY".to_string(),
				Self::CallingParty => "CALLING_PARTY".to_string(),
				Self::Unknown => "UNKNOWN".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MediaInitiatorFlag {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CALLED_PARTY" => Ok(Self::CalledParty),
				"CALLING_PARTY" => Ok(Self::CallingParty),
				"UNKNOWN" => Ok(Self::Unknown),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MediaInitiatorFlag {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MediaInitiatorFlag {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MediaInitiatorFlag {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// MessageBody
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "contentLength",
	///    "contentType"
	///  ],
	///  "properties": {
	///    "contentDisposition": {
	///      "type": "string"
	///    },
	///    "contentLength": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "contentType": {
	///      "type": "string"
	///    },
	///    "originator": {
	///      "$ref": "#/components/schemas/OriginatorPartyType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MessageBody {
		#[serde(
			rename = "contentDisposition",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub content_disposition: Option<String>,
		#[serde(rename = "contentLength")]
		pub content_length: Uint32,
		#[serde(rename = "contentType")]
		pub content_type: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub originator: Option<OriginatorPartyType>,
	}

	impl From<&MessageBody> for MessageBody {
		fn from(value: &MessageBody) -> Self {
			value.clone()
		}
	}

	/// MessageClass
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "classIdentifier": {
	///      "$ref": "#/components/schemas/ClassIdentifier"
	///    },
	///    "tokenText": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MessageClass {
		#[serde(
			rename = "classIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub class_identifier: Option<ClassIdentifier>,
		#[serde(rename = "tokenText", default, skip_serializing_if = "Option::is_none")]
		pub token_text: Option<String>,
	}

	impl From<&MessageClass> for MessageClass {
		fn from(value: &MessageClass) -> Self {
			value.clone()
		}
	}

	/// MicoModeIndication
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "MICO_MODE",
	///    "NO_MICO_MODE"
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
	pub enum MicoModeIndication {
		#[default]
		#[serde(rename = "MICO_MODE")]
		MicoMode,
		#[serde(rename = "NO_MICO_MODE")]
		NoMicoMode,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&MicoModeIndication> for MicoModeIndication {
		fn from(value: &MicoModeIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for MicoModeIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::MicoMode => "MICO_MODE".to_string(),
				Self::NoMicoMode => "NO_MICO_MODE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for MicoModeIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MICO_MODE" => Ok(Self::MicoMode),
				"NO_MICO_MODE" => Ok(Self::NoMicoMode),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for MicoModeIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MicoModeIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MicoModeIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// MmTelChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "supplementaryServices": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupplementaryService"
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
	pub struct MmTelChargingInformation {
		#[serde(
			rename = "supplementaryServices",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub supplementary_services: Vec<SupplementaryService>,
	}

	impl From<&MmTelChargingInformation> for MmTelChargingInformation {
		fn from(value: &MmTelChargingInformation) -> Self {
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

	/// MultipleQfIcontainer
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "localSequenceNumber"
	///  ],
	///  "properties": {
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "localSequenceNumber": {
	///      "type": "integer"
	///    },
	///    "qFIContainerInformation": {
	///      "$ref": "#/components/schemas/QFIContainerInformation"
	///    },
	///    "time": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "totalVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "triggerTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "triggers": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Trigger"
	///      },
	///      "minItems": 0
	///    },
	///    "uplinkVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MultipleQfIcontainer {
		#[serde(
			rename = "downlinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub downlink_volume: Option<Uint64>,
		#[serde(rename = "localSequenceNumber")]
		pub local_sequence_number: i64,
		#[serde(
			rename = "qFIContainerInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub q_fi_container_information: Option<QfiContainerInformation>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub time: Option<Uint32>,
		#[serde(
			rename = "totalVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub total_volume: Option<Uint64>,
		#[serde(
			rename = "triggerTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub trigger_timestamp: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub triggers: Vec<Trigger>,
		#[serde(
			rename = "uplinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uplink_volume: Option<Uint64>,
	}

	impl From<&MultipleQfIcontainer> for MultipleQfIcontainer {
		fn from(value: &MultipleQfIcontainer) -> Self {
			value.clone()
		}
	}

	/// MultipleUnitInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "ratingGroup"
	///  ],
	///  "properties": {
	///    "announcementInformation": {
	///      "$ref": "#/components/schemas/AnnouncementInformation"
	///    },
	///    "finalUnitIndication": {
	///      "$ref": "#/components/schemas/FinalUnitIndication"
	///    },
	///    "grantedUnit": {
	///      "$ref": "#/components/schemas/GrantedUnit"
	///    },
	///    "quotaHoldingTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "ratingGroup": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "resultCode": {
	///      "$ref": "#/components/schemas/ResultCode"
	///    },
	///    "timeQuotaThreshold": {
	///      "type": "integer"
	///    },
	///    "triggers": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Trigger"
	///      },
	///      "minItems": 0
	///    },
	///    "uPFID": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "unitQuotaThreshold": {
	///      "type": "integer"
	///    },
	///    "validityTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "volumeQuotaThreshold": {
	///      "$ref": "#/components/schemas/Uint64"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MultipleUnitInformation {
		#[serde(
			rename = "announcementInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub announcement_information: Option<AnnouncementInformation>,
		#[serde(
			rename = "finalUnitIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub final_unit_indication: Option<FinalUnitIndication>,
		#[serde(
			rename = "grantedUnit",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub granted_unit: Option<GrantedUnit>,
		#[serde(
			rename = "quotaHoldingTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub quota_holding_time: Option<DurationSec>,
		#[serde(rename = "ratingGroup")]
		pub rating_group: Uint32,
		#[serde(
			rename = "resultCode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub result_code: Option<ResultCode>,
		#[serde(
			rename = "timeQuotaThreshold",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_quota_threshold: Option<i64>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub triggers: Vec<Trigger>,
		#[serde(rename = "uPFID", default, skip_serializing_if = "Option::is_none")]
		pub u_pfid: Option<NfInstanceId>,
		#[serde(
			rename = "unitQuotaThreshold",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub unit_quota_threshold: Option<i64>,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DurationSec>,
		#[serde(
			rename = "volumeQuotaThreshold",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub volume_quota_threshold: Option<Uint64>,
	}

	impl From<&MultipleUnitInformation> for MultipleUnitInformation {
		fn from(value: &MultipleUnitInformation) -> Self {
			value.clone()
		}
	}

	/// MultipleUnitUsage
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "ratingGroup"
	///  ],
	///  "properties": {
	///    "multihomedPDUAddress": {
	///      "$ref": "#/components/schemas/PDUAddress"
	///    },
	///    "ratingGroup": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "requestedUnit": {
	///      "$ref": "#/components/schemas/RequestedUnit"
	///    },
	///    "uPFID": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "usedUnitContainer": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UsedUnitContainer"
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
	pub struct MultipleUnitUsage {
		#[serde(
			rename = "multihomedPDUAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub multihomed_pdu_address: Option<PduAddress>,
		#[serde(rename = "ratingGroup")]
		pub rating_group: Uint32,
		#[serde(
			rename = "requestedUnit",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub requested_unit: Option<RequestedUnit>,
		#[serde(rename = "uPFID", default, skip_serializing_if = "Option::is_none")]
		pub u_pfid: Option<NfInstanceId>,
		#[serde(
			rename = "usedUnitContainer",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub used_unit_container: Vec<UsedUnitContainer>,
	}

	impl From<&MultipleUnitUsage> for MultipleUnitUsage {
		fn from(value: &MultipleUnitUsage) -> Self {
			value.clone()
		}
	}

	/// N2ConnectionChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "n2ConnectionMessageType"
	///  ],
	///  "properties": {
	///    "allowedNSSAI": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 0
	///    },
	///    "amfUeNgapId": {
	///      "type": "integer"
	///    },
	///    "forbiddenAreaList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Area"
	///      },
	///      "minItems": 0
	///    },
	///    "n2ConnectionMessageType": {
	///      "$ref": "#/components/schemas/N2ConnectionMessageType"
	///    },
	///    "pSCellInformation": {
	///      "$ref": "#/components/schemas/PSCellInformation"
	///    },
	///    "rATType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "ranNodeId": {
	///      "$ref": "#/components/schemas/GlobalRanNodeId"
	///    },
	///    "ranUeNgapId": {
	///      "type": "integer"
	///    },
	///    "restrictedCnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CoreNetworkType"
	///      },
	///      "minItems": 0
	///    },
	///    "restrictedRatList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "minItems": 0
	///    },
	///    "rrcEstCause": {
	///      "type": "string",
	///      "pattern": "^[0-9a-fA-F]+$"
	///    },
	///    "serviceAreaRestriction": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServiceAreaRestriction"
	///      },
	///      "minItems": 0
	///    },
	///    "uetimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userInformation": {
	///      "$ref": "#/components/schemas/UserInformation"
	///    },
	///    "userLocationinfo": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N2ConnectionChargingInformation {
		#[serde(
			rename = "allowedNSSAI",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_nssai: Vec<Snssai>,
		#[serde(
			rename = "amfUeNgapId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_ue_ngap_id: Option<i64>,
		#[serde(
			rename = "forbiddenAreaList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub forbidden_area_list: Vec<Area>,
		#[serde(rename = "n2ConnectionMessageType")]
		pub n2_connection_message_type: N2ConnectionMessageType,
		#[serde(
			rename = "pSCellInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub p_s_cell_information: Option<PsCellInformation>,
		#[serde(rename = "rATType", default, skip_serializing_if = "Option::is_none")]
		pub r_at_type: Option<RatType>,
		#[serde(rename = "ranNodeId", default, skip_serializing_if = "Option::is_none")]
		pub ran_node_id: Option<GlobalRanNodeId>,
		#[serde(
			rename = "ranUeNgapId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ran_ue_ngap_id: Option<i64>,
		#[serde(
			rename = "restrictedCnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub restricted_cn_list: Vec<CoreNetworkType>,
		#[serde(
			rename = "restrictedRatList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub restricted_rat_list: Vec<RatType>,
		#[serde(
			rename = "rrcEstCause",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rrc_est_cause: Option<N2ConnectionChargingInformationRrcEstCause>,
		#[serde(
			rename = "serviceAreaRestriction",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub service_area_restriction: Vec<ServiceAreaRestriction>,
		#[serde(
			rename = "uetimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uetime_zone: Option<TimeZone>,
		#[serde(
			rename = "userInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_information: Option<UserInformation>,
		#[serde(
			rename = "userLocationinfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_locationinfo: Option<UserLocation>,
	}

	impl From<&N2ConnectionChargingInformation> for N2ConnectionChargingInformation {
		fn from(value: &N2ConnectionChargingInformation) -> Self {
			value.clone()
		}
	}

	/// N2ConnectionChargingInformationRrcEstCause
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9a-fA-F]+$"
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
	pub struct N2ConnectionChargingInformationRrcEstCause(String);

	impl ::std::ops::Deref for N2ConnectionChargingInformationRrcEstCause {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<N2ConnectionChargingInformationRrcEstCause> for String {
		fn from(value: N2ConnectionChargingInformationRrcEstCause) -> Self {
			value.0
		}
	}

	impl From<&N2ConnectionChargingInformationRrcEstCause>
		for N2ConnectionChargingInformationRrcEstCause
	{
		fn from(value: &N2ConnectionChargingInformationRrcEstCause) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for N2ConnectionChargingInformationRrcEstCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9a-fA-F]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9a-fA-F]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for N2ConnectionChargingInformationRrcEstCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for N2ConnectionChargingInformationRrcEstCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for N2ConnectionChargingInformationRrcEstCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for N2ConnectionChargingInformationRrcEstCause {
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

	/// N2ConnectionMessageType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "integer"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N2ConnectionMessageType(pub i64);

	impl ::std::ops::Deref for N2ConnectionMessageType {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<N2ConnectionMessageType> for i64 {
		fn from(value: N2ConnectionMessageType) -> Self {
			value.0
		}
	}

	impl From<&N2ConnectionMessageType> for N2ConnectionMessageType {
		fn from(value: &N2ConnectionMessageType) -> Self {
			value.clone()
		}
	}

	impl From<i64> for N2ConnectionMessageType {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for N2ConnectionMessageType {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for N2ConnectionMessageType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for N2ConnectionMessageType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for N2ConnectionMessageType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for N2ConnectionMessageType {
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

	/// NefChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "aPIName"
	///  ],
	///  "properties": {
	///    "aPIContent": {
	///      "type": "string"
	///    },
	///    "aPIDirection": {
	///      "$ref": "#/components/schemas/APIDirection"
	///    },
	///    "aPIName": {
	///      "type": "string"
	///    },
	///    "aPIReference": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "aPIResultCode": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "aPITargetNetworkFunction": {
	///      "$ref": "#/components/schemas/NFIdentification"
	///    },
	///    "externalGroupIdentifier": {
	///      "$ref": "#/components/schemas/ExternalGroupId"
	///    },
	///    "externalIndividualIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "externalIndividualIdentifier": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "groupIdentifier": {
	///      "$ref": "#/components/schemas/GroupId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NefChargingInformation {
		#[serde(
			rename = "aPIContent",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub a_pi_content: Option<String>,
		#[serde(
			rename = "aPIDirection",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub a_pi_direction: Option<ApiDirection>,
		#[serde(rename = "aPIName")]
		pub a_pi_name: String,
		#[serde(
			rename = "aPIReference",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub a_pi_reference: Option<Uri>,
		#[serde(
			rename = "aPIResultCode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub a_pi_result_code: Option<Uint32>,
		#[serde(
			rename = "aPITargetNetworkFunction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub a_pi_target_network_function: Option<NfIdentification>,
		#[serde(
			rename = "externalGroupIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub external_group_identifier: Option<ExternalGroupId>,
		#[serde(
			rename = "externalIndividualIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_individual_id_list: Vec<Gpsi>,
		#[serde(
			rename = "externalIndividualIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub external_individual_identifier: Option<Gpsi>,
		#[serde(
			rename = "groupIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub group_identifier: Option<GroupId>,
	}

	impl From<&NefChargingInformation> for NefChargingInformation {
		fn from(value: &NefChargingInformation) -> Self {
			value.clone()
		}
	}

	/// Describes a network area information in which the NF service consumer
	/// requests the number of UEs.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes a network area information in which the NF
	/// service consumer requests the number of UEs.\n",
	///  "type": "object",
	///  "properties": {
	///    "ecgis": {
	///      "description": "Contains a list of E-UTRA cell identities.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ecgi"
	///      },
	///      "minItems": 1
	///    },
	///    "gRanNodeIds": {
	///      "description": "Contains a list of NG RAN nodes.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GlobalRanNodeId"
	///      },
	///      "minItems": 1
	///    },
	///    "ncgis": {
	///      "description": "Contains a list of NR cell identities.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ncgi"
	///      },
	///      "minItems": 1
	///    },
	///    "tais": {
	///      "description": "Contains a list of tracking area identities.",
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
	pub struct NetworkAreaInfo {
		/// Contains a list of E-UTRA cell identities.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub ecgis: Vec<Ecgi>,
		/// Contains a list of NG RAN nodes.
		#[serde(rename = "gRanNodeIds", default, skip_serializing_if = "Vec::is_empty")]
		pub g_ran_node_ids: Vec<GlobalRanNodeId>,
		/// Contains a list of NR cell identities.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub ncgis: Vec<Ncgi>,
		/// Contains a list of tracking area identities.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub tais: Vec<Tai>,
	}

	impl From<&NetworkAreaInfo> for NetworkAreaInfo {
		fn from(value: &NetworkAreaInfo) -> Self {
			value.clone()
		}
	}

	/// NetworkSlicingInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "sNSSAI"
	///  ],
	///  "properties": {
	///    "sNSSAI": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NetworkSlicingInfo {
		#[serde(rename = "sNSSAI")]
		pub s_nssai: Snssai,
	}

	impl From<&NetworkSlicingInfo> for NetworkSlicingInfo {
		fn from(value: &NetworkSlicingInfo) -> Self {
			value.clone()
		}
	}

	/// NfIdentification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "nodeFunctionality"
	///  ],
	///  "properties": {
	///    "nFFqdn": {
	///      "type": "string"
	///    },
	///    "nFIPv4Address": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "nFIPv6Address": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "nFName": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nFPLMNID": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "nodeFunctionality": {
	///      "$ref": "#/components/schemas/NodeFunctionality"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfIdentification {
		#[serde(rename = "nFFqdn", default, skip_serializing_if = "Option::is_none")]
		pub n_f_fqdn: Option<String>,
		#[serde(rename = "nFName", default, skip_serializing_if = "Option::is_none")]
		pub n_f_name: Option<NfInstanceId>,
		#[serde(
			rename = "nFIPv4Address",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n_fi_pv4_address: Option<Ipv4Addr>,
		#[serde(
			rename = "nFIPv6Address",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n_fi_pv6_address: Option<Ipv6Addr>,
		#[serde(rename = "nFPLMNID", default, skip_serializing_if = "Option::is_none")]
		pub n_fplmnid: Option<PlmnId>,
		#[serde(rename = "nodeFunctionality")]
		pub node_functionality: NodeFunctionality,
	}

	impl From<&NfIdentification> for NfIdentification {
		fn from(value: &NfIdentification) -> Self {
			value.clone()
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

	/// NniInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "nNIType": {
	///      "$ref": "#/components/schemas/NNIType"
	///    },
	///    "neighbourNodeAddress": {
	///      "$ref": "#/components/schemas/IMSAddress"
	///    },
	///    "relationshipMode": {
	///      "$ref": "#/components/schemas/NNIRelationshipMode"
	///    },
	///    "sessionDirection": {
	///      "$ref": "#/components/schemas/NNISessionDirection"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NniInformation {
		#[serde(rename = "nNIType", default, skip_serializing_if = "Option::is_none")]
		pub n_ni_type: Option<NniType>,
		#[serde(
			rename = "neighbourNodeAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub neighbour_node_address: Option<ImsAddress>,
		#[serde(
			rename = "relationshipMode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub relationship_mode: Option<NniRelationshipMode>,
		#[serde(
			rename = "sessionDirection",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub session_direction: Option<NniSessionDirection>,
	}

	impl From<&NniInformation> for NniInformation {
		fn from(value: &NniInformation) -> Self {
			value.clone()
		}
	}

	/// NniRelationshipMode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "TRUSTED",
	///    "NON_TRUSTED"
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
	pub enum NniRelationshipMode {
		#[default]
		#[serde(rename = "TRUSTED")]
		Trusted,
		#[serde(rename = "NON_TRUSTED")]
		NonTrusted,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NniRelationshipMode> for NniRelationshipMode {
		fn from(value: &NniRelationshipMode) -> Self {
			value.clone()
		}
	}

	impl ToString for NniRelationshipMode {
		fn to_string(&self) -> String {
			match *self {
				Self::Trusted => "TRUSTED".to_string(),
				Self::NonTrusted => "NON_TRUSTED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NniRelationshipMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TRUSTED" => Ok(Self::Trusted),
				"NON_TRUSTED" => Ok(Self::NonTrusted),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NniRelationshipMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NniRelationshipMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NniRelationshipMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// NniSessionDirection
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "INBOUND",
	///    "OUTBOUND"
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
	pub enum NniSessionDirection {
		#[default]
		#[serde(rename = "INBOUND")]
		Inbound,
		#[serde(rename = "OUTBOUND")]
		Outbound,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NniSessionDirection> for NniSessionDirection {
		fn from(value: &NniSessionDirection) -> Self {
			value.clone()
		}
	}

	impl ToString for NniSessionDirection {
		fn to_string(&self) -> String {
			match *self {
				Self::Inbound => "INBOUND".to_string(),
				Self::Outbound => "OUTBOUND".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NniSessionDirection {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"INBOUND" => Ok(Self::Inbound),
				"OUTBOUND" => Ok(Self::Outbound),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NniSessionDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NniSessionDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NniSessionDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// NniType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "NON_ROAMING",
	///    "ROAMING_NO_LOOPBACK",
	///    "ROAMING_LOOPBACK"
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
	pub enum NniType {
		#[default]
		#[serde(rename = "NON_ROAMING")]
		NonRoaming,
		#[serde(rename = "ROAMING_NO_LOOPBACK")]
		RoamingNoLoopback,
		#[serde(rename = "ROAMING_LOOPBACK")]
		RoamingLoopback,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NniType> for NniType {
		fn from(value: &NniType) -> Self {
			value.clone()
		}
	}

	impl ToString for NniType {
		fn to_string(&self) -> String {
			match *self {
				Self::NonRoaming => "NON_ROAMING".to_string(),
				Self::RoamingNoLoopback => "ROAMING_NO_LOOPBACK".to_string(),
				Self::RoamingLoopback => "ROAMING_LOOPBACK".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NniType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NON_ROAMING" => Ok(Self::NonRoaming),
				"ROAMING_NO_LOOPBACK" => Ok(Self::RoamingNoLoopback),
				"ROAMING_LOOPBACK" => Ok(Self::RoamingLoopback),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NniType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NniType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NniType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// NodeFunctionality
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "AMF",
	///    "SMF",
	///    "SMS",
	///    "SMSF",
	///    "PGW_C_SMF",
	///    "NEFF",
	///    "SGW",
	///    "I_SMF",
	///    "ePDG",
	///    "CEF",
	///    "NEF",
	///    "MnS_Producer",
	///    "SGSN",
	///    "V_SMF",
	///    "5G_DDNMF",
	///    "IMS_Node",
	///    "EES",
	///    "PCF",
	///    "UDM",
	///    "UPF"
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
	pub enum NodeFunctionality {
		#[default]
		#[serde(rename = "AMF")]
		Amf,
		#[serde(rename = "SMF")]
		Smf,
		#[serde(rename = "SMS")]
		Sms,
		#[serde(rename = "SMSF")]
		Smsf,
		#[serde(rename = "PGW_C_SMF")]
		PgwCSmf,
		#[serde(rename = "NEFF")]
		Neff,
		#[serde(rename = "SGW")]
		Sgw,
		#[serde(rename = "I_SMF")]
		ISmf,
		#[serde(rename = "ePDG")]
		EPdg,
		#[serde(rename = "CEF")]
		Cef,
		#[serde(rename = "NEF")]
		Nef,
		#[serde(rename = "MnS_Producer")]
		MnSProducer,
		#[serde(rename = "SGSN")]
		Sgsn,
		#[serde(rename = "V_SMF")]
		VSmf,
		#[serde(rename = "5G_DDNMF")]
		FiveGDdnmf,
		#[serde(rename = "IMS_Node")]
		ImsNode,
		#[serde(rename = "EES")]
		Ees,
		#[serde(rename = "PCF")]
		Pcf,
		#[serde(rename = "UDM")]
		Udm,
		#[serde(rename = "UPF")]
		Upf,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NodeFunctionality> for NodeFunctionality {
		fn from(value: &NodeFunctionality) -> Self {
			value.clone()
		}
	}

	impl ToString for NodeFunctionality {
		fn to_string(&self) -> String {
			match *self {
				Self::Amf => "AMF".to_string(),
				Self::Smf => "SMF".to_string(),
				Self::Sms => "SMS".to_string(),
				Self::Smsf => "SMSF".to_string(),
				Self::PgwCSmf => "PGW_C_SMF".to_string(),
				Self::Neff => "NEFF".to_string(),
				Self::Sgw => "SGW".to_string(),
				Self::ISmf => "I_SMF".to_string(),
				Self::EPdg => "ePDG".to_string(),
				Self::Cef => "CEF".to_string(),
				Self::Nef => "NEF".to_string(),
				Self::MnSProducer => "MnS_Producer".to_string(),
				Self::Sgsn => "SGSN".to_string(),
				Self::VSmf => "V_SMF".to_string(),
				Self::FiveGDdnmf => "5G_DDNMF".to_string(),
				Self::ImsNode => "IMS_Node".to_string(),
				Self::Ees => "EES".to_string(),
				Self::Pcf => "PCF".to_string(),
				Self::Udm => "UDM".to_string(),
				Self::Upf => "UPF".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NodeFunctionality {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AMF" => Ok(Self::Amf),
				"SMF" => Ok(Self::Smf),
				"SMS" => Ok(Self::Sms),
				"SMSF" => Ok(Self::Smsf),
				"PGW_C_SMF" => Ok(Self::PgwCSmf),
				"NEFF" => Ok(Self::Neff),
				"SGW" => Ok(Self::Sgw),
				"I_SMF" => Ok(Self::ISmf),
				"ePDG" => Ok(Self::EPdg),
				"CEF" => Ok(Self::Cef),
				"NEF" => Ok(Self::Nef),
				"MnS_Producer" => Ok(Self::MnSProducer),
				"SGSN" => Ok(Self::Sgsn),
				"V_SMF" => Ok(Self::VSmf),
				"5G_DDNMF" => Ok(Self::FiveGDdnmf),
				"IMS_Node" => Ok(Self::ImsNode),
				"EES" => Ok(Self::Ees),
				"PCF" => Ok(Self::Pcf),
				"UDM" => Ok(Self::Udm),
				"UPF" => Ok(Self::Upf),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NodeFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NodeFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NodeFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// NotificationType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "REAUTHORIZATION",
	///    "ABORT_CHARGING"
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
	pub enum NotificationType {
		#[default]
		#[serde(rename = "REAUTHORIZATION")]
		Reauthorization,
		#[serde(rename = "ABORT_CHARGING")]
		AbortCharging,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NotificationType> for NotificationType {
		fn from(value: &NotificationType) -> Self {
			value.clone()
		}
	}

	impl ToString for NotificationType {
		fn to_string(&self) -> String {
			match *self {
				Self::Reauthorization => "REAUTHORIZATION".to_string(),
				Self::AbortCharging => "ABORT_CHARGING".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NotificationType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REAUTHORIZATION" => Ok(Self::Reauthorization),
				"ABORT_CHARGING" => Ok(Self::AbortCharging),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NotificationType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NotificationType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NotificationType {
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

	/// Contains the Identifier of the selected Network Slice instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Identifier of the selected Network Slice
	/// instance",
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
	pub struct NsiId(pub String);

	impl ::std::ops::Deref for NsiId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NsiId> for String {
		fn from(value: NsiId) -> Self {
			value.0
		}
	}

	impl From<&NsiId> for NsiId {
		fn from(value: &NsiId) -> Self {
			value.clone()
		}
	}

	impl From<String> for NsiId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NsiId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for NsiId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Represents the network slice and optionally the associated network slice
	/// instance and the load level information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the network slice and optionally the associated network slice instance and the load level information.\n",
	///  "type": "object",
	///  "required": [
	///    "loadLevelInformation",
	///    "snssai"
	///  ],
	///  "properties": {
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "exceedLoadLevelThrInd": {
	///      "type": "boolean"
	///    },
	///    "loadLevelInformation": {
	///      "$ref": "#/components/schemas/LoadLevelInformation"
	///    },
	///    "networkArea": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "nsiId": {
	///      "$ref": "#/components/schemas/NsiId"
	///    },
	///    "numOfExceedLoadLevelThr": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "numOfPduSess": {
	///      "$ref": "#/components/schemas/NumberAverage"
	///    },
	///    "numOfUes": {
	///      "$ref": "#/components/schemas/NumberAverage"
	///    },
	///    "resUsage": {
	///      "$ref": "#/components/schemas/ResourceUsage"
	///    },
	///    "resUsgThrCrossTimePeriod": {
	///      "description": "Each element indicates the time elapsed between
	/// times each threshold is met or exceeded or crossed. The start time and
	/// end time are the exact time stamps of the resource usage threshold is
	/// reached or exceeded. May be present if the \"listOfAnaSubsets\"
	/// attribute is\n provided and the maximum number of instances shall not
	/// exceed the value provided in the\n \"numOfExceedLoadLevelThr\"
	/// attribute.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TimeWindow"
	///      },
	///      "minItems": 1
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "timePeriod": {
	///      "$ref": "#/components/schemas/TimeWindow"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NsiLoadLevelInfo {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub confidence: Option<Uinteger>,
		#[serde(
			rename = "exceedLoadLevelThrInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub exceed_load_level_thr_ind: Option<bool>,
		#[serde(rename = "loadLevelInformation")]
		pub load_level_information: LoadLevelInformation,
		#[serde(
			rename = "networkArea",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub network_area: Option<NetworkAreaInfo>,
		#[serde(rename = "nsiId", default, skip_serializing_if = "Option::is_none")]
		pub nsi_id: Option<NsiId>,
		#[serde(
			rename = "numOfExceedLoadLevelThr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub num_of_exceed_load_level_thr: Option<Uinteger>,
		#[serde(
			rename = "numOfPduSess",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub num_of_pdu_sess: Option<NumberAverage>,
		#[serde(rename = "numOfUes", default, skip_serializing_if = "Option::is_none")]
		pub num_of_ues: Option<NumberAverage>,
		#[serde(rename = "resUsage", default, skip_serializing_if = "Option::is_none")]
		pub res_usage: Option<ResourceUsage>,
		/// Each element indicates the time elapsed between times each threshold
		/// is met or exceeded or crossed. The start time and end time are the
		/// exact time stamps of the resource usage threshold is reached or
		/// exceeded. May be present if the "listOfAnaSubsets" attribute is
		/// provided and the maximum number of instances shall not exceed the
		/// value provided in the "numOfExceedLoadLevelThr" attribute.
		#[serde(
			rename = "resUsgThrCrossTimePeriod",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub res_usg_thr_cross_time_period: Vec<TimeWindow>,
		pub snssai: Snssai,
		#[serde(
			rename = "timePeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_period: Option<TimeWindow>,
	}

	impl From<&NsiLoadLevelInfo> for NsiLoadLevelInfo {
		fn from(value: &NsiLoadLevelInfo) -> Self {
			value.clone()
		}
	}

	/// NsmChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "managementOperation"
	///  ],
	///  "properties": {
	///    "idNetworkSliceInstance": {
	///      "type": "string"
	///    },
	///    "listOfserviceProfileChargingInformation": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServiceProfileChargingInformation"
	///      },
	///      "minItems": 0
	///    },
	///    "managementAdministrativeState": {
	///      "$ref": "#/components/schemas/AdministrativeState"
	///    },
	///    "managementOperation": {
	///      "$ref": "#/components/schemas/ManagementOperation"
	///    },
	///    "managementOperationStatus": {
	///      "$ref": "#/components/schemas/ManagementOperationStatus"
	///    },
	///    "managementOperationalState": {
	///      "$ref": "#/components/schemas/OperationalState"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NsmChargingInformation {
		#[serde(
			rename = "idNetworkSliceInstance",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub id_network_slice_instance: Option<String>,
		#[serde(
			rename = "listOfserviceProfileChargingInformation",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub list_ofservice_profile_charging_information: Vec<ServiceProfileChargingInformation>,
		#[serde(
			rename = "managementAdministrativeState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub management_administrative_state: Option<AdministrativeState>,
		#[serde(rename = "managementOperation")]
		pub management_operation: ManagementOperation,
		#[serde(
			rename = "managementOperationStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub management_operation_status: Option<ManagementOperationStatus>,
		#[serde(
			rename = "managementOperationalState",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub management_operational_state: Option<OperationalState>,
	}

	impl From<&NsmChargingInformation> for NsmChargingInformation {
		fn from(value: &NsmChargingInformation) -> Self {
			value.clone()
		}
	}

	/// NspaChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "singleNSSAI"
	///  ],
	///  "properties": {
	///    "singleNSSAI": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NspaChargingInformation {
		#[serde(rename = "singleNSSAI")]
		pub single_nssai: Snssai,
	}

	impl From<&NspaChargingInformation> for NspaChargingInformation {
		fn from(value: &NspaChargingInformation) -> Self {
			value.clone()
		}
	}

	/// NspaContainerInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "downlinkLatency": {
	///      "type": "integer"
	///    },
	///    "downlinkThroughput": {
	///      "$ref": "#/components/schemas/Throughput"
	///    },
	///    "latency": {
	///      "type": "integer"
	///    },
	///    "loadLevel": {
	///      "$ref": "#/components/schemas/NsiLoadLevelInfo"
	///    },
	///    "maximumPacketLossRate": {
	///      "type": "string"
	///    },
	///    "maximumPacketLossRateDL": {
	///      "type": "integer"
	///    },
	///    "maximumPacketLossRateUL": {
	///      "type": "integer"
	///    },
	///    "serviceExperienceStatisticsData": {
	///      "$ref": "#/components/schemas/ServiceExperienceInfo"
	///    },
	///    "theNumberOfPDUSessions": {
	///      "type": "integer"
	///    },
	///    "theNumberOfRegisteredSubscribers": {
	///      "type": "integer"
	///    },
	///    "throughput": {
	///      "$ref": "#/components/schemas/Throughput"
	///    },
	///    "uplinkLatency": {
	///      "type": "integer"
	///    },
	///    "uplinkThroughput": {
	///      "$ref": "#/components/schemas/Throughput"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NspaContainerInformation {
		#[serde(
			rename = "downlinkLatency",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub downlink_latency: Option<i64>,
		#[serde(
			rename = "downlinkThroughput",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub downlink_throughput: Option<Throughput>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub latency: Option<i64>,
		#[serde(rename = "loadLevel", default, skip_serializing_if = "Option::is_none")]
		pub load_level: Option<NsiLoadLevelInfo>,
		#[serde(
			rename = "maximumPacketLossRate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub maximum_packet_loss_rate: Option<String>,
		#[serde(
			rename = "maximumPacketLossRateDL",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub maximum_packet_loss_rate_dl: Option<i64>,
		#[serde(
			rename = "maximumPacketLossRateUL",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub maximum_packet_loss_rate_ul: Option<i64>,
		#[serde(
			rename = "serviceExperienceStatisticsData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_experience_statistics_data: Option<ServiceExperienceInfo>,
		#[serde(
			rename = "theNumberOfPDUSessions",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub the_number_of_pdu_sessions: Option<i64>,
		#[serde(
			rename = "theNumberOfRegisteredSubscribers",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub the_number_of_registered_subscribers: Option<i64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub throughput: Option<Throughput>,
		#[serde(
			rename = "uplinkLatency",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uplink_latency: Option<i64>,
		#[serde(
			rename = "uplinkThroughput",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uplink_throughput: Option<Throughput>,
	}

	impl From<&NspaContainerInformation> for NspaContainerInformation {
		fn from(value: &NspaContainerInformation) -> Self {
			value.clone()
		}
	}

	/// NssaiMap
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "homeSnssai",
	///    "servingSnssai"
	///  ],
	///  "properties": {
	///    "homeSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "servingSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NssaiMap {
		#[serde(rename = "homeSnssai")]
		pub home_snssai: Snssai,
		#[serde(rename = "servingSnssai")]
		pub serving_snssai: Snssai,
	}

	impl From<&NssaiMap> for NssaiMap {
		fn from(value: &NssaiMap) -> Self {
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

	/// Represents average and variance information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents average and variance information.",
	///  "type": "object",
	///  "required": [
	///    "number",
	///    "variance"
	///  ],
	///  "properties": {
	///    "number": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "skewness": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "variance": {
	///      "$ref": "#/components/schemas/Float"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NumberAverage {
		pub number: Float,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub skewness: Option<Float>,
		pub variance: Float,
	}

	impl From<&NumberAverage> for NumberAverage {
		fn from(value: &NumberAverage) -> Self {
			value.clone()
		}
	}

	/// OctetString
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9a-fA-F]+$"
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
	pub struct OctetString(String);

	impl ::std::ops::Deref for OctetString {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<OctetString> for String {
		fn from(value: OctetString) -> Self {
			value.0
		}
	}

	impl From<&OctetString> for OctetString {
		fn from(value: &OctetString) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for OctetString {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9a-fA-F]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9a-fA-F]+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for OctetString {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for OctetString {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for OctetString {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for OctetString {
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

	/// OneTimeEventType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "IEC",
	///    "PEC"
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
	pub enum OneTimeEventType {
		#[default]
		#[serde(rename = "IEC")]
		Iec,
		#[serde(rename = "PEC")]
		Pec,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&OneTimeEventType> for OneTimeEventType {
		fn from(value: &OneTimeEventType) -> Self {
			value.clone()
		}
	}

	impl ToString for OneTimeEventType {
		fn to_string(&self) -> String {
			match *self {
				Self::Iec => "IEC".to_string(),
				Self::Pec => "PEC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for OneTimeEventType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IEC" => Ok(Self::Iec),
				"PEC" => Ok(Self::Pec),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for OneTimeEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for OneTimeEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for OneTimeEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// OperationalState
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "ENABLED",
	///    "DISABLED"
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
	pub enum OperationalState {
		#[default]
		#[serde(rename = "ENABLED")]
		Enabled,
		#[serde(rename = "DISABLED")]
		Disabled,
	}

	impl From<&OperationalState> for OperationalState {
		fn from(value: &OperationalState) -> Self {
			value.clone()
		}
	}

	impl ToString for OperationalState {
		fn to_string(&self) -> String {
			match *self {
				Self::Enabled => "ENABLED".to_string(),
				Self::Disabled => "DISABLED".to_string(),
			}
		}
	}

	impl std::str::FromStr for OperationalState {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ENABLED" => Ok(Self::Enabled),
				"DISABLED" => Ok(Self::Disabled),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for OperationalState {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for OperationalState {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for OperationalState {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// OriginatorInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "originatorGPSI": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "originatorOtherAddress": {
	///      "$ref": "#/components/schemas/SMAddressInfo"
	///    },
	///    "originatorReceivedAddress": {
	///      "$ref": "#/components/schemas/SMAddressInfo"
	///    },
	///    "originatorSCCPAddress": {
	///      "type": "string"
	///    },
	///    "originatorSUPI": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "sMOriginatorInterface": {
	///      "$ref": "#/components/schemas/SMInterface"
	///    },
	///    "sMOriginatorProtocolId": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct OriginatorInfo {
		#[serde(
			rename = "originatorGPSI",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub originator_gpsi: Option<Gpsi>,
		#[serde(
			rename = "originatorOtherAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub originator_other_address: Option<SmAddressInfo>,
		#[serde(
			rename = "originatorReceivedAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub originator_received_address: Option<SmAddressInfo>,
		#[serde(
			rename = "originatorSCCPAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub originator_sccp_address: Option<String>,
		#[serde(
			rename = "originatorSUPI",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub originator_supi: Option<Supi>,
		#[serde(
			rename = "sMOriginatorInterface",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_originator_interface: Option<SmInterface>,
		#[serde(
			rename = "sMOriginatorProtocolId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_originator_protocol_id: Option<String>,
	}

	impl From<&OriginatorInfo> for OriginatorInfo {
		fn from(value: &OriginatorInfo) -> Self {
			value.clone()
		}
	}

	/// OriginatorPartyType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "CALLING",
	///    "CALLED"
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
	pub enum OriginatorPartyType {
		#[default]
		#[serde(rename = "CALLING")]
		Calling,
		#[serde(rename = "CALLED")]
		Called,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&OriginatorPartyType> for OriginatorPartyType {
		fn from(value: &OriginatorPartyType) -> Self {
			value.clone()
		}
	}

	impl ToString for OriginatorPartyType {
		fn to_string(&self) -> String {
			match *self {
				Self::Calling => "CALLING".to_string(),
				Self::Called => "CALLED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for OriginatorPartyType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CALLING" => Ok(Self::Calling),
				"CALLED" => Ok(Self::Called),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for OriginatorPartyType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for OriginatorPartyType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for OriginatorPartyType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// ParticipantActionType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "CREATE",
	///    "JOIN",
	///    "INVITE_INTO",
	///    "QUIT"
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
	pub enum ParticipantActionType {
		#[default]
		#[serde(rename = "CREATE")]
		Create,
		#[serde(rename = "JOIN")]
		Join,
		#[serde(rename = "INVITE_INTO")]
		InviteInto,
		#[serde(rename = "QUIT")]
		Quit,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ParticipantActionType> for ParticipantActionType {
		fn from(value: &ParticipantActionType) -> Self {
			value.clone()
		}
	}

	impl ToString for ParticipantActionType {
		fn to_string(&self) -> String {
			match *self {
				Self::Create => "CREATE".to_string(),
				Self::Join => "JOIN".to_string(),
				Self::InviteInto => "INVITE_INTO".to_string(),
				Self::Quit => "QUIT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ParticipantActionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CREATE" => Ok(Self::Create),
				"JOIN" => Ok(Self::Join),
				"INVITE_INTO" => Ok(Self::InviteInto),
				"QUIT" => Ok(Self::Quit),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ParticipantActionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ParticipantActionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ParticipantActionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Pc5ContainerInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "coverageInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CoverageInfo"
	///      }
	///    },
	///    "radioParameterSetInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RadioParameterSetInfo"
	///      }
	///    },
	///    "timeOfFirst Reception": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "timeOfFirst Transmission": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "transmitterInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TransmitterInfo"
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
	pub struct Pc5ContainerInformation {
		#[serde(
			rename = "coverageInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub coverage_info_list: Vec<CoverageInfo>,
		#[serde(
			rename = "radioParameterSetInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub radio_parameter_set_info_list: Vec<RadioParameterSetInfo>,
		#[serde(
			rename = "timeOfFirst Reception",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_of_first_reception: Option<DateTime>,
		#[serde(
			rename = "timeOfFirst Transmission",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_of_first_transmission: Option<DateTime>,
		#[serde(
			rename = "transmitterInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub transmitter_info_list: Vec<TransmitterInfo>,
	}

	impl From<&Pc5ContainerInformation> for Pc5ContainerInformation {
		fn from(value: &Pc5ContainerInformation) -> Self {
			value.clone()
		}
	}

	/// Pc5DataContainer
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "changeCondition": {
	///      "type": "string"
	///    },
	///    "changeTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "coverageStatus": {
	///      "type": "boolean"
	///    },
	///    "dataVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "localSequenceNumber": {
	///      "type": "string"
	///    },
	///    "pC5RadioTechnology": {
	///      "type": "string"
	///    },
	///    "radioFrequency": {
	///      "type": "string"
	///    },
	///    "radioResourcesId": {
	///      "$ref": "#/components/schemas/RadioResourcesId"
	///    },
	///    "userLocationInformation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Pc5DataContainer {
		#[serde(
			rename = "changeCondition",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub change_condition: Option<String>,
		#[serde(
			rename = "changeTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub change_time: Option<DateTime>,
		#[serde(
			rename = "coverageStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub coverage_status: Option<bool>,
		#[serde(
			rename = "dataVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub data_volume: Option<Uint64>,
		#[serde(
			rename = "localSequenceNumber",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub local_sequence_number: Option<String>,
		#[serde(
			rename = "pC5RadioTechnology",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub p_c5_radio_technology: Option<String>,
		#[serde(
			rename = "radioFrequency",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub radio_frequency: Option<String>,
		#[serde(
			rename = "radioResourcesId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub radio_resources_id: Option<RadioResourcesId>,
		#[serde(
			rename = "userLocationInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location_information: Option<UserLocation>,
	}

	impl From<&Pc5DataContainer> for Pc5DataContainer {
		fn from(value: &Pc5DataContainer) -> Self {
			value.clone()
		}
	}

	/// PduAddress
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "addIpv6AddrPrefixList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Prefix"
	///      }
	///    },
	///    "addIpv6AddrPrefixes": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "iPv4dynamicAddressFlag": {
	///      "type": "boolean"
	///    },
	///    "iPv6dynamicPrefixFlag": {
	///      "type": "boolean"
	///    },
	///    "pduAddressprefixlength": {
	///      "type": "integer"
	///    },
	///    "pduIPv4Address": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "pduIPv6AddresswithPrefix": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduAddress {
		#[serde(
			rename = "addIpv6AddrPrefixList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub add_ipv6_addr_prefix_list: Vec<Ipv6Prefix>,
		#[serde(
			rename = "addIpv6AddrPrefixes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_ipv6_addr_prefixes: Option<Ipv6Prefix>,
		#[serde(
			rename = "iPv4dynamicAddressFlag",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub i_pv4dynamic_address_flag: Option<bool>,
		#[serde(
			rename = "iPv6dynamicPrefixFlag",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub i_pv6dynamic_prefix_flag: Option<bool>,
		#[serde(
			rename = "pduAddressprefixlength",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_addressprefixlength: Option<i64>,
		#[serde(
			rename = "pduIPv4Address",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_i_pv4_address: Option<Ipv4Addr>,
		#[serde(
			rename = "pduIPv6AddresswithPrefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_i_pv6_addresswith_prefix: Option<Ipv6Addr>,
	}

	impl From<&PduAddress> for PduAddress {
		fn from(value: &PduAddress) -> Self {
			value.clone()
		}
	}

	/// PduContainerInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "3gppPSDataOffStatus": {
	///      "$ref": "#/components/schemas/3GPPPSDataOffStatus"
	///    },
	///    "afChargingIdString": {
	///      "$ref": "#/components/schemas/ApplicationChargingId"
	///    },
	///    "afChargingIdentifier": {
	///      "$ref": "#/components/schemas/ChargingId"
	///    },
	///    "applicationserviceProviderIdentity": {
	///      "type": "string"
	///    },
	///    "chargingRuleBaseName": {
	///      "type": "string"
	///    },
	///    "mAPDUSteeringFunctionality": {
	///      "$ref": "#/components/schemas/SteeringFunctionality"
	///    },
	///    "mAPDUSteeringMode": {
	///      "$ref": "#/components/schemas/SteeringMode"
	///    },
	///    "presenceReportingAreaInformation": {
	///      "type": "object",
	///      "minProperties": 0,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "qoSCharacteristics": {
	///      "$ref": "#/components/schemas/QosCharacteristics"
	///    },
	///    "qoSInformation": {
	///      "$ref": "#/components/schemas/QosData"
	///    },
	///    "qosMonitoringReport": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosMonitoringReport"
	///      },
	///      "minItems": 0
	///    },
	///    "rATType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "servingNodeID": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServingNetworkFunctionID"
	///      },
	///      "minItems": 0
	///    },
	///    "sponsorIdentity": {
	///      "type": "string"
	///    },
	///    "timeofFirstUsage": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "timeofLastUsage": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "trafficForwardingWay": {
	///      "$ref": "#/components/schemas/TrafficForwardingWay"
	///    },
	///    "uetimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userLocationInformation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduContainerInformation {
		#[serde(
			rename = "afChargingIdString",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub af_charging_id_string: Option<ApplicationChargingId>,
		#[serde(
			rename = "afChargingIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub af_charging_identifier: Option<ChargingId>,
		#[serde(
			rename = "applicationserviceProviderIdentity",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub applicationservice_provider_identity: Option<String>,
		#[serde(
			rename = "chargingRuleBaseName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub charging_rule_base_name: Option<String>,
		#[serde(
			rename = "mAPDUSteeringFunctionality",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub m_apdu_steering_functionality: Option<SteeringFunctionality>,
		#[serde(
			rename = "mAPDUSteeringMode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub m_apdu_steering_mode: Option<SteeringMode>,
		#[serde(
			rename = "presenceReportingAreaInformation",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub presence_reporting_area_information: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(
			rename = "qoSCharacteristics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qo_s_characteristics: Option<QosCharacteristics>,
		#[serde(
			rename = "qoSInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qo_s_information: Option<QosData>,
		#[serde(
			rename = "qosMonitoringReport",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_monitoring_report: Vec<QosMonitoringReport>,
		#[serde(rename = "rATType", default, skip_serializing_if = "Option::is_none")]
		pub r_at_type: Option<RatType>,
		#[serde(
			rename = "servingNodeID",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_node_id: Vec<ServingNetworkFunctionId>,
		#[serde(
			rename = "sponsorIdentity",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sponsor_identity: Option<String>,
		#[serde(
			rename = "3gppPSDataOffStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gpp_ps_data_off_status: Option<_3gpppsDataOffStatus>,
		#[serde(
			rename = "timeofFirstUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub timeof_first_usage: Option<DateTime>,
		#[serde(
			rename = "timeofLastUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub timeof_last_usage: Option<DateTime>,
		#[serde(
			rename = "trafficForwardingWay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub traffic_forwarding_way: Option<TrafficForwardingWay>,
		#[serde(
			rename = "uetimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uetime_zone: Option<TimeZone>,
		#[serde(
			rename = "userLocationInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location_information: Option<UserLocation>,
	}

	impl From<&PduContainerInformation> for PduContainerInformation {
		fn from(value: &PduContainerInformation) -> Self {
			value.clone()
		}
	}

	/// PduSessionChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "chargingId": {
	///      "$ref": "#/components/schemas/ChargingId"
	///    },
	///    "homeProvidedChargingId": {
	///      "$ref": "#/components/schemas/ChargingId"
	///    },
	///    "mAPDUNon3GPPUserLocationInfo": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "mAPDUNon3GPPUserLocationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "non3GPPUserLocationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "pduSessionInformation": {
	///      "$ref": "#/components/schemas/PDUSessionInformation"
	///    },
	///    "presenceReportingAreaInformation": {
	///      "type": "object",
	///      "minProperties": 0,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "rANSecondaryRATUsageReport": {
	///      "$ref": "#/components/schemas/RANSecondaryRATUsageReport"
	///    },
	///    "sMFHomeProvidedChargingId": {
	///      "type": "string"
	///    },
	///    "sMFchargingId": {
	///      "type": "string"
	///    },
	///    "uetimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "unitCountInactivityTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "userInformation": {
	///      "$ref": "#/components/schemas/UserInformation"
	///    },
	///    "userLocationinfo": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionChargingInformation {
		#[serde(
			rename = "chargingId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub charging_id: Option<ChargingId>,
		#[serde(
			rename = "homeProvidedChargingId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub home_provided_charging_id: Option<ChargingId>,
		#[serde(
			rename = "mAPDUNon3GPPUserLocationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub m_apdu_non3_gpp_user_location_info: Option<UserLocation>,
		#[serde(
			rename = "mAPDUNon3GPPUserLocationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub m_apdu_non3_gpp_user_location_time: Option<DateTime>,
		#[serde(
			rename = "non3GPPUserLocationTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub non3_gpp_user_location_time: Option<DateTime>,
		#[serde(
			rename = "pduSessionInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_information: Option<PduSessionInformation>,
		#[serde(
			rename = "presenceReportingAreaInformation",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub presence_reporting_area_information: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(
			rename = "rANSecondaryRATUsageReport",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub r_an_secondary_rat_usage_report: Option<RanSecondaryRatUsageReport>,
		#[serde(
			rename = "sMFchargingId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_fcharging_id: Option<String>,
		#[serde(
			rename = "sMFHomeProvidedChargingId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_mf_home_provided_charging_id: Option<String>,
		#[serde(
			rename = "uetimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uetime_zone: Option<TimeZone>,
		#[serde(
			rename = "unitCountInactivityTimer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub unit_count_inactivity_timer: Option<DurationSec>,
		#[serde(
			rename = "userInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_information: Option<UserInformation>,
		#[serde(
			rename = "userLocationinfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_locationinfo: Option<UserLocation>,
	}

	impl From<&PduSessionChargingInformation> for PduSessionChargingInformation {
		fn from(value: &PduSessionChargingInformation) -> Self {
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

	/// PduSessionInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "dnnId",
	///    "pduSessionID"
	///  ],
	///  "properties": {
	///    "3gppPSDataOffStatus": {
	///      "$ref": "#/components/schemas/3GPPPSDataOffStatus"
	///    },
	///    "5GLANTypeService": {
	///      "$ref": "#/components/schemas/5GLANTypeService"
	///    },
	///    "5GSControlPlaneOnlyIndicator": {
	///      "type": "boolean"
	///    },
	///    "authorizedQoSInformation": {
	///      "$ref": "#/components/schemas/AuthorizedDefaultQos"
	///    },
	///    "authorizedSessionAMBR": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "chargingCharacteristics": {
	///      "type": "string",
	///      "pattern": "^[0-9a-fA-F]{1,4}$"
	///    },
	///    "chargingCharacteristicsSelectionMode": {
	///      "$ref": "#/components/schemas/ChargingCharacteristicsSelectionMode"
	///    },
	///    "cpCIoTOptimisationIndicator": {
	///      "type": "boolean"
	///    },
	///    "diagnostics": {
	///      "$ref": "#/components/schemas/Diagnostics"
	///    },
	///    "dnnId": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "dnnSelectionMode": {
	///      "$ref": "#/components/schemas/dnnSelectionMode"
	///    },
	///    "enhancedDiagnostics": {
	///      "$ref": "#/components/schemas/EnhancedDiagnostics5G"
	///    },
	///    "hPlmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "mAPDUNon3GPPRATType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "mAPDUSessionInformation": {
	///      "$ref": "#/components/schemas/MAPDUSessionInformation"
	///    },
	///    "networkSlicingInfo": {
	///      "$ref": "#/components/schemas/NetworkSlicingInfo"
	///    },
	///    "pDUSessionPairID": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "pduAddress": {
	///      "$ref": "#/components/schemas/PDUAddress"
	///    },
	///    "pduSessionID": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "pduType": {
	///      "$ref": "#/components/schemas/PduSessionType"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "redundantTransmissionType": {
	///      "$ref": "#/components/schemas/RedundantTransmissionType"
	///    },
	///    "servingCNPlmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "servingNetworkFunctionID": {
	///      "$ref": "#/components/schemas/ServingNetworkFunctionID"
	///    },
	///    "sessionStopIndicator": {
	///      "type": "boolean"
	///    },
	///    "smallDataRateControlIndicator": {
	///      "type": "boolean"
	///    },
	///    "sscMode": {
	///      "$ref": "#/components/schemas/SscMode"
	///    },
	///    "startTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "stopTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "subscribedQoSInformation": {
	///      "$ref": "#/components/schemas/SubscribedDefaultQos"
	///    },
	///    "subscribedSessionAMBR": {
	///      "$ref": "#/components/schemas/Ambr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionInformation {
		#[serde(
			rename = "authorizedQoSInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub authorized_qo_s_information: Option<AuthorizedDefaultQos>,
		#[serde(
			rename = "authorizedSessionAMBR",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub authorized_session_ambr: Option<Ambr>,
		#[serde(
			rename = "chargingCharacteristics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub charging_characteristics: Option<PduSessionInformationChargingCharacteristics>,
		#[serde(
			rename = "chargingCharacteristicsSelectionMode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub charging_characteristics_selection_mode: Option<ChargingCharacteristicsSelectionMode>,
		#[serde(
			rename = "cpCIoTOptimisationIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cp_c_io_t_optimisation_indicator: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub diagnostics: Option<Diagnostics>,
		#[serde(rename = "dnnId")]
		pub dnn_id: Dnn,
		#[serde(
			rename = "dnnSelectionMode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dnn_selection_mode: Option<DnnSelectionMode>,
		#[serde(
			rename = "enhancedDiagnostics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub enhanced_diagnostics: Option<EnhancedDiagnostics5G>,
		#[serde(
			rename = "5GLANTypeService",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_glan_type_service: Option<_5glanTypeService>,
		#[serde(
			rename = "5GSControlPlaneOnlyIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_gs_control_plane_only_indicator: Option<bool>,
		#[serde(rename = "hPlmnId", default, skip_serializing_if = "Option::is_none")]
		pub h_plmn_id: Option<PlmnId>,
		#[serde(
			rename = "mAPDUNon3GPPRATType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub m_apdu_non3_gpprat_type: Option<RatType>,
		#[serde(
			rename = "mAPDUSessionInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub m_apdu_session_information: Option<MapduSessionInformation>,
		#[serde(
			rename = "networkSlicingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub network_slicing_info: Option<NetworkSlicingInfo>,
		#[serde(
			rename = "pDUSessionPairID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub p_du_session_pair_id: Option<Uint32>,
		#[serde(
			rename = "pduAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_address: Option<PduAddress>,
		#[serde(rename = "pduSessionID")]
		pub pdu_session_id: PduSessionId,
		#[serde(rename = "pduType", default, skip_serializing_if = "Option::is_none")]
		pub pdu_type: Option<PduSessionType>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "redundantTransmissionType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub redundant_transmission_type: Option<RedundantTransmissionType>,
		#[serde(
			rename = "servingCNPlmnId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_cn_plmn_id: Option<PlmnId>,
		#[serde(
			rename = "servingNetworkFunctionID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network_function_id: Option<ServingNetworkFunctionId>,
		#[serde(
			rename = "sessionStopIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub session_stop_indicator: Option<bool>,
		#[serde(
			rename = "smallDataRateControlIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub small_data_rate_control_indicator: Option<bool>,
		#[serde(rename = "sscMode", default, skip_serializing_if = "Option::is_none")]
		pub ssc_mode: Option<SscMode>,
		#[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
		pub start_time: Option<DateTime>,
		#[serde(rename = "stopTime", default, skip_serializing_if = "Option::is_none")]
		pub stop_time: Option<DateTime>,
		#[serde(
			rename = "subscribedQoSInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscribed_qo_s_information: Option<SubscribedDefaultQos>,
		#[serde(
			rename = "subscribedSessionAMBR",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscribed_session_ambr: Option<Ambr>,
		#[serde(
			rename = "3gppPSDataOffStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gpp_ps_data_off_status: Option<_3gpppsDataOffStatus>,
	}

	impl From<&PduSessionInformation> for PduSessionInformation {
		fn from(value: &PduSessionInformation) -> Self {
			value.clone()
		}
	}

	/// PduSessionInformationChargingCharacteristics
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9a-fA-F]{1,4}$"
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
	pub struct PduSessionInformationChargingCharacteristics(String);

	impl ::std::ops::Deref for PduSessionInformationChargingCharacteristics {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PduSessionInformationChargingCharacteristics> for String {
		fn from(value: PduSessionInformationChargingCharacteristics) -> Self {
			value.0
		}
	}

	impl From<&PduSessionInformationChargingCharacteristics>
		for PduSessionInformationChargingCharacteristics
	{
		fn from(value: &PduSessionInformationChargingCharacteristics) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PduSessionInformationChargingCharacteristics {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9a-fA-F]{1,4}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9a-fA-F]{1,4}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PduSessionInformationChargingCharacteristics {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PduSessionInformationChargingCharacteristics {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PduSessionInformationChargingCharacteristics {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PduSessionInformationChargingCharacteristics {
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

	/// Represents the data structure presenting the pending policy counter
	/// status.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the data structure presenting the pending
	/// policy counter status.\n",
	///  "type": "object",
	///  "required": [
	///    "activationTime",
	///    "policyCounterStatus"
	///  ],
	///  "properties": {
	///    "activationTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "policyCounterStatus": {
	///      "description": "Identifies the policy counter status applicable for
	/// a specific policy counter identified by the policyCounterId. The values
	/// (e.g. valid, invalid or any other status) are not specified. The
	/// interpretation and actions related to the defined values are out of
	/// scope of 3GPP.\n",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PendingPolicyCounterStatus {
		#[serde(rename = "activationTime")]
		pub activation_time: DateTime,
		/// Identifies the policy counter status applicable for a specific
		/// policy counter identified by the policyCounterId. The values (e.g.
		/// valid, invalid or any other status) are not specified. The
		/// interpretation and actions related to the defined values are out of
		/// scope of 3GPP.
		#[serde(rename = "policyCounterStatus")]
		pub policy_counter_status: String,
	}

	impl From<&PendingPolicyCounterStatus> for PendingPolicyCounterStatus {
		fn from(value: &PendingPolicyCounterStatus) -> Self {
			value.clone()
		}
	}

	/// PfiContainerInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "pFI": {
	///      "type": "string"
	///    },
	///    "presenceReportingAreaInformation": {
	///      "type": "object",
	///      "minProperties": 0,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "qoSCharacteristics": {
	///      "$ref": "#/components/schemas/QosCharacteristics"
	///    },
	///    "qoSInformation": {
	///      "$ref": "#/components/schemas/QosData"
	///    },
	///    "reportTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "timeofFirstUsage": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "timeofLastUsage": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "uetimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userLocationInformation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PfiContainerInformation {
		#[serde(rename = "pFI", default, skip_serializing_if = "Option::is_none")]
		pub p_fi: Option<String>,
		#[serde(
			rename = "presenceReportingAreaInformation",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub presence_reporting_area_information: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(
			rename = "qoSCharacteristics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qo_s_characteristics: Option<QosCharacteristics>,
		#[serde(
			rename = "qoSInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qo_s_information: Option<QosData>,
		#[serde(
			rename = "reportTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub report_time: Option<DateTime>,
		#[serde(
			rename = "timeofFirstUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub timeof_first_usage: Option<DateTime>,
		#[serde(
			rename = "timeofLastUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub timeof_last_usage: Option<DateTime>,
		#[serde(
			rename = "uetimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uetime_zone: Option<TimeZone>,
		#[serde(
			rename = "userLocationInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location_information: Option<UserLocation>,
	}

	impl From<&PfiContainerInformation> for PfiContainerInformation {
		fn from(value: &PfiContainerInformation) -> Self {
			value.clone()
		}
	}

	/// PlayToParty
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "SERVED",
	///    "REMOTE"
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
	pub enum PlayToParty {
		#[default]
		#[serde(rename = "SERVED")]
		Served,
		#[serde(rename = "REMOTE")]
		Remote,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PlayToParty> for PlayToParty {
		fn from(value: &PlayToParty) -> Self {
			value.clone()
		}
	}

	impl ToString for PlayToParty {
		fn to_string(&self) -> String {
			match *self {
				Self::Served => "SERVED".to_string(),
				Self::Remote => "REMOTE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PlayToParty {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SERVED" => Ok(Self::Served),
				"REMOTE" => Ok(Self::Remote),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PlayToParty {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PlayToParty {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PlayToParty {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Identifies a policy counter.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies a policy counter.",
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
	pub struct PolicyCounterId(pub String);

	impl ::std::ops::Deref for PolicyCounterId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PolicyCounterId> for String {
		fn from(value: PolicyCounterId) -> Self {
			value.0
		}
	}

	impl From<&PolicyCounterId> for PolicyCounterId {
		fn from(value: &PolicyCounterId) -> Self {
			value.clone()
		}
	}

	impl From<String> for PolicyCounterId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for PolicyCounterId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for PolicyCounterId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Represents the data structure presenting the policy counter status.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the data structure presenting the policy
	/// counter status.",
	///  "type": "object",
	///  "required": [
	///    "currentStatus",
	///    "policyCounterId"
	///  ],
	///  "properties": {
	///    "currentStatus": {
	///      "description": "Identifies the policy counter status applicable for
	/// a specific policy counter identified by the policyCounterId. The values
	/// (e.g. valid, invalid or any other status) are not specified. The
	/// interpretation and actions related to the defined values are out of
	/// scope of 3GPP.\n",
	///      "type": "string"
	///    },
	///    "penPolCounterStatuses": {
	///      "description": "Provides the pending policy counter status.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PendingPolicyCounterStatus"
	///      },
	///      "minItems": 1
	///    },
	///    "policyCounterId": {
	///      "$ref": "#/components/schemas/PolicyCounterId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PolicyCounterInfo {
		/// Identifies the policy counter status applicable for a specific
		/// policy counter identified by the policyCounterId. The values (e.g.
		/// valid, invalid or any other status) are not specified. The
		/// interpretation and actions related to the defined values are out of
		/// scope of 3GPP.
		#[serde(rename = "currentStatus")]
		pub current_status: String,
		/// Provides the pending policy counter status.
		#[serde(
			rename = "penPolCounterStatuses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pen_pol_counter_statuses: Vec<PendingPolicyCounterStatus>,
		#[serde(rename = "policyCounterId")]
		pub policy_counter_id: PolicyCounterId,
	}

	impl From<&PolicyCounterInfo> for PolicyCounterInfo {
		fn from(value: &PolicyCounterInfo) -> Self {
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

	/// ProseChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "aPIName"
	///  ],
	///  "properties": {
	///    "ApplicationId": {
	///      "type": "string"
	///    },
	///    "announcingPlmnID": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "announcingUeHplmnIdentifier": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "announcingUeVplmnIdentifier": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "applicationSpecificDataList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 0
	///    },
	///    "directDiscoveryModel": {
	///      "$ref": "#/components/schemas/DirectDiscoveryModel"
	///    },
	///    "discovereeUeHplmnIdentifier": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "discovereeUeVplmnIdentifier": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "discovererUeHplmnIdentifier": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "discovererUeVplmnIdentifier": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "monitoredPlmnIdentifier": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "monitoringUEIdentifier": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "monitoringUeHplmnIdentifier": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "monitoringUeVplmnIdentifier": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "pC3ProtocolCause": {
	///      "type": "integer"
	///    },
	///    "pFIContainerInformation": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PFIContainerInformation"
	///      },
	///      "minItems": 0
	///    },
	///    "proseApplicationID": {
	///      "type": "string"
	///    },
	///    "proseDestinationLayer2ID": {
	///      "type": "string"
	///    },
	///    "proseEventType": {
	///      "$ref": "#/components/schemas/ProseEventType"
	///    },
	///    "proseFunctionality": {
	///      "$ref": "#/components/schemas/ProseFunctionality"
	///    },
	///    "proseRequestTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "proseUEToNetworkRelayUEID": {
	///      "type": "string"
	///    },
	///    "proximityAlertIndication": {
	///      "type": "boolean"
	///    },
	///    "proximityAlertTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "proximityCancellationTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "rangeClass": {
	///      "$ref": "#/components/schemas/RangeClass"
	///    },
	///    "receptionDataContainer": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PC5DataContainer"
	///      },
	///      "minItems": 0
	///    },
	///    "relayIPAddress": {
	///      "$ref": "#/components/schemas/IpAddr"
	///    },
	///    "requestedPLMNIdentifier": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "roleOfUE": {
	///      "$ref": "#/components/schemas/RoleOfUE"
	///    },
	///    "timeWindow": {
	///      "type": "integer"
	///    },
	///    "transmissionDataContainer": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PC5DataContainer"
	///      },
	///      "minItems": 0
	///    },
	///    "validityPeriod": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProseChargingInformation {
		#[serde(rename = "aPIName")]
		pub a_pi_name: ::serde_json::Value,
		#[serde(
			rename = "announcingPlmnID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub announcing_plmn_id: Option<PlmnId>,
		#[serde(
			rename = "announcingUeHplmnIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub announcing_ue_hplmn_identifier: Option<PlmnId>,
		#[serde(
			rename = "announcingUeVplmnIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub announcing_ue_vplmn_identifier: Option<PlmnId>,
		#[serde(
			rename = "ApplicationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub application_id: Option<String>,
		#[serde(
			rename = "applicationSpecificDataList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub application_specific_data_list: Vec<String>,
		#[serde(
			rename = "directDiscoveryModel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub direct_discovery_model: Option<DirectDiscoveryModel>,
		#[serde(
			rename = "discovereeUeHplmnIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub discoveree_ue_hplmn_identifier: Option<PlmnId>,
		#[serde(
			rename = "discovereeUeVplmnIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub discoveree_ue_vplmn_identifier: Option<PlmnId>,
		#[serde(
			rename = "discovererUeHplmnIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub discoverer_ue_hplmn_identifier: Option<PlmnId>,
		#[serde(
			rename = "discovererUeVplmnIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub discoverer_ue_vplmn_identifier: Option<PlmnId>,
		#[serde(
			rename = "monitoredPlmnIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub monitored_plmn_identifier: Option<PlmnId>,
		#[serde(
			rename = "monitoringUeHplmnIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub monitoring_ue_hplmn_identifier: Option<PlmnId>,
		#[serde(
			rename = "monitoringUEIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub monitoring_ue_identifier: Option<Supi>,
		#[serde(
			rename = "monitoringUeVplmnIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub monitoring_ue_vplmn_identifier: Option<PlmnId>,
		#[serde(
			rename = "pC3ProtocolCause",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub p_c3_protocol_cause: Option<i64>,
		#[serde(
			rename = "pFIContainerInformation",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub p_fi_container_information: Vec<PfiContainerInformation>,
		#[serde(
			rename = "proseApplicationID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_application_id: Option<String>,
		#[serde(
			rename = "proseDestinationLayer2ID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_destination_layer2_id: Option<String>,
		#[serde(
			rename = "proseEventType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_event_type: Option<ProseEventType>,
		#[serde(
			rename = "proseFunctionality",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_functionality: Option<ProseFunctionality>,
		#[serde(
			rename = "proseRequestTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_request_timestamp: Option<DateTime>,
		#[serde(
			rename = "proseUEToNetworkRelayUEID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_ue_to_network_relay_ueid: Option<String>,
		#[serde(
			rename = "proximityAlertIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub proximity_alert_indication: Option<bool>,
		#[serde(
			rename = "proximityAlertTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub proximity_alert_timestamp: Option<DateTime>,
		#[serde(
			rename = "proximityCancellationTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub proximity_cancellation_timestamp: Option<DateTime>,
		#[serde(
			rename = "rangeClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub range_class: Option<RangeClass>,
		#[serde(
			rename = "receptionDataContainer",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub reception_data_container: Vec<Pc5DataContainer>,
		#[serde(
			rename = "relayIPAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub relay_ip_address: Option<IpAddr>,
		#[serde(
			rename = "requestedPLMNIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub requested_plmn_identifier: Option<PlmnId>,
		#[serde(rename = "roleOfUE", default, skip_serializing_if = "Option::is_none")]
		pub role_of_ue: Option<RoleOfUe>,
		#[serde(
			rename = "timeWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_window: Option<i64>,
		#[serde(
			rename = "transmissionDataContainer",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub transmission_data_container: Vec<Pc5DataContainer>,
		#[serde(
			rename = "validityPeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_period: Option<i64>,
	}

	impl From<&ProseChargingInformation> for ProseChargingInformation {
		fn from(value: &ProseChargingInformation) -> Self {
			value.clone()
		}
	}

	/// ProseEventType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "ANNOUNCING",
	///    "MONITORING",
	///    "MATCH_REPORT"
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
	pub enum ProseEventType {
		#[default]
		#[serde(rename = "ANNOUNCING")]
		Announcing,
		#[serde(rename = "MONITORING")]
		Monitoring,
		#[serde(rename = "MATCH_REPORT")]
		MatchReport,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ProseEventType> for ProseEventType {
		fn from(value: &ProseEventType) -> Self {
			value.clone()
		}
	}

	impl ToString for ProseEventType {
		fn to_string(&self) -> String {
			match *self {
				Self::Announcing => "ANNOUNCING".to_string(),
				Self::Monitoring => "MONITORING".to_string(),
				Self::MatchReport => "MATCH_REPORT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ProseEventType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ANNOUNCING" => Ok(Self::Announcing),
				"MONITORING" => Ok(Self::Monitoring),
				"MATCH_REPORT" => Ok(Self::MatchReport),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ProseEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ProseEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ProseEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ProseFunctionality
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "DIRECT_DISCOVERY",
	///    "DIRECT_COMMUNICATION"
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
	pub enum ProseFunctionality {
		#[default]
		#[serde(rename = "DIRECT_DISCOVERY")]
		DirectDiscovery,
		#[serde(rename = "DIRECT_COMMUNICATION")]
		DirectCommunication,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ProseFunctionality> for ProseFunctionality {
		fn from(value: &ProseFunctionality) -> Self {
			value.clone()
		}
	}

	impl ToString for ProseFunctionality {
		fn to_string(&self) -> String {
			match *self {
				Self::DirectDiscovery => "DIRECT_DISCOVERY".to_string(),
				Self::DirectCommunication => "DIRECT_COMMUNICATION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ProseFunctionality {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DIRECT_DISCOVERY" => Ok(Self::DirectDiscovery),
				"DIRECT_COMMUNICATION" => Ok(Self::DirectCommunication),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ProseFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ProseFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ProseFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// PsCellInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "ecgi": {
	///      "$ref": "#/components/schemas/Ecgi"
	///    },
	///    "nrcgi": {
	///      "$ref": "#/components/schemas/Ncgi"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PsCellInformation {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ecgi: Option<Ecgi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nrcgi: Option<Ncgi>,
	}

	impl From<&PsCellInformation> for PsCellInformation {
		fn from(value: &PsCellInformation) -> Self {
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

	/// QfiContainerInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "reportTime"
	///  ],
	///  "properties": {
	///    "3gppChargingId": {
	///      "$ref": "#/components/schemas/ChargingId"
	///    },
	///    "3gppPSDataOffStatus": {
	///      "$ref": "#/components/schemas/3GPPPSDataOffStatus"
	///    },
	///    "diagnostics": {
	///      "$ref": "#/components/schemas/Diagnostics"
	///    },
	///    "enhancedDiagnostics": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      }
	///    },
	///    "presenceReportingAreaInformation": {
	///      "type": "object",
	///      "minProperties": 0,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "qFI": {
	///      "$ref": "#/components/schemas/Qfi"
	///    },
	///    "qoSCharacteristics": {
	///      "$ref": "#/components/schemas/QosCharacteristics"
	///    },
	///    "qoSInformation": {
	///      "$ref": "#/components/schemas/QosData"
	///    },
	///    "rATType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "reportTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "servingNetworkFunctionID": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServingNetworkFunctionID"
	///      },
	///      "minItems": 0
	///    },
	///    "timeofFirstUsage": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "timeofLastUsage": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "uetimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userLocationInformation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QfiContainerInformation {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub diagnostics: Option<Diagnostics>,
		#[serde(
			rename = "enhancedDiagnostics",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub enhanced_diagnostics: Vec<String>,
		#[serde(
			rename = "presenceReportingAreaInformation",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub presence_reporting_area_information: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(rename = "qFI", default, skip_serializing_if = "Option::is_none")]
		pub q_fi: Option<Qfi>,
		#[serde(
			rename = "qoSCharacteristics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qo_s_characteristics: Option<QosCharacteristics>,
		#[serde(
			rename = "qoSInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub qo_s_information: Option<QosData>,
		#[serde(rename = "rATType", default, skip_serializing_if = "Option::is_none")]
		pub r_at_type: Option<RatType>,
		#[serde(rename = "reportTime")]
		pub report_time: DateTime,
		#[serde(
			rename = "servingNetworkFunctionID",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_network_function_id: Vec<ServingNetworkFunctionId>,
		#[serde(
			rename = "3gppChargingId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gpp_charging_id: Option<ChargingId>,
		#[serde(
			rename = "3gppPSDataOffStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gpp_ps_data_off_status: Option<_3gpppsDataOffStatus>,
		#[serde(
			rename = "timeofFirstUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub timeof_first_usage: Option<DateTime>,
		#[serde(
			rename = "timeofLastUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub timeof_last_usage: Option<DateTime>,
		#[serde(
			rename = "uetimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uetime_zone: Option<TimeZone>,
		#[serde(
			rename = "userLocationInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location_information: Option<UserLocation>,
	}

	impl From<&QfiContainerInformation> for QfiContainerInformation {
		fn from(value: &QfiContainerInformation) -> Self {
			value.clone()
		}
	}

	/// Contains QoS characteristics for a non-standardized or a non-configured
	/// 5QI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains QoS characteristics for a non-standardized or
	/// a non-configured 5QI.",
	///  "type": "object",
	///  "required": [
	///    "5qi",
	///    "packetDelayBudget",
	///    "packetErrorRate",
	///    "priorityLevel",
	///    "resourceType"
	///  ],
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "averagingWindow": {
	///      "$ref": "#/components/schemas/AverWindow"
	///    },
	///    "extMaxDataBurstVol": {
	///      "$ref": "#/components/schemas/ExtMaxDataBurstVol"
	///    },
	///    "maxDataBurstVol": {
	///      "$ref": "#/components/schemas/MaxDataBurstVol"
	///    },
	///    "packetDelayBudget": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "packetErrorRate": {
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
	pub struct QosCharacteristics {
		#[serde(
			rename = "averagingWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub averaging_window: Option<AverWindow>,
		#[serde(
			rename = "extMaxDataBurstVol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ext_max_data_burst_vol: Option<ExtMaxDataBurstVol>,
		#[serde(rename = "5qi")]
		pub five_qi: _5qi,
		#[serde(
			rename = "maxDataBurstVol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_data_burst_vol: Option<MaxDataBurstVol>,
		#[serde(rename = "packetDelayBudget")]
		pub packet_delay_budget: PacketDelBudget,
		#[serde(rename = "packetErrorRate")]
		pub packet_error_rate: PacketErrRate,
		#[serde(rename = "priorityLevel")]
		pub priority_level: _5qiPriorityLevel,
		#[serde(rename = "resourceType")]
		pub resource_type: QosResourceType,
	}

	impl From<&QosCharacteristics> for QosCharacteristics {
		fn from(value: &QosCharacteristics) -> Self {
			value.clone()
		}
	}

	/// Contains the QoS parameters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the QoS parameters.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "qosId"
	///  ],
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "arp": {
	///      "$ref": "#/components/schemas/Arp"
	///    },
	///    "averWindow": {
	///      "$ref": "#/components/schemas/AverWindowRm"
	///    },
	///    "defQosFlowIndication": {
	///      "description": "Indicates that the dynamic PCC rule shall always
	/// have its binding with the QoS Flow associated with the default QoS
	/// rule",
	///      "type": "boolean"
	///    },
	///    "extMaxDataBurstVol": {
	///      "$ref": "#/components/schemas/ExtMaxDataBurstVolRm"
	///    },
	///    "gbrDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "gbrUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "maxDataBurstVol": {
	///      "$ref": "#/components/schemas/MaxDataBurstVolRm"
	///    },
	///    "maxPacketLossRateDl": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "maxPacketLossRateUl": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "maxbrDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "maxbrUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "packetDelayBudget": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "packetErrorRate": {
	///      "$ref": "#/components/schemas/PacketErrRate"
	///    },
	///    "priorityLevel": {
	///      "$ref": "#/components/schemas/5QiPriorityLevelRm"
	///    },
	///    "qnc": {
	///      "description": "Indicates whether notifications are requested from
	/// 3GPP NG-RAN when the GFBR can no longer (or again) be guaranteed for a
	/// QoS Flow during the lifetime of the QoS Flow.\n",
	///      "type": "boolean"
	///    },
	///    "qosId": {
	///      "description": "Univocally identifies the QoS control policy data
	/// within a PDU session.",
	///      "type": "string"
	///    },
	///    "reflectiveQos": {
	///      "description": "Indicates whether the QoS information is reflective
	/// for the corresponding service data flow.",
	///      "type": "boolean"
	///    },
	///    "sharingKeyDl": {
	///      "description": "Indicates, by containing the same value, what PCC
	/// rules may share resource in downlink direction.",
	///      "type": "string"
	///    },
	///    "sharingKeyUl": {
	///      "description": "Indicates, by containing the same value, what PCC
	/// rules may share resource in uplink direction.",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosData(pub Option<QosDataInner>);

	impl ::std::ops::Deref for QosData {
		type Target = Option<QosDataInner>;
		fn deref(&self) -> &Option<QosDataInner> {
			&self.0
		}
	}

	impl From<QosData> for Option<QosDataInner> {
		fn from(value: QosData) -> Self {
			value.0
		}
	}

	impl From<&QosData> for QosData {
		fn from(value: &QosData) -> Self {
			value.clone()
		}
	}

	impl From<Option<QosDataInner>> for QosData {
		fn from(value: Option<QosDataInner>) -> Self {
			Self(value)
		}
	}

	/// Contains the QoS parameters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the QoS parameters.",
	///  "type": "object",
	///  "required": [
	///    "qosId"
	///  ],
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "arp": {
	///      "$ref": "#/components/schemas/Arp"
	///    },
	///    "averWindow": {
	///      "$ref": "#/components/schemas/AverWindowRm"
	///    },
	///    "defQosFlowIndication": {
	///      "description": "Indicates that the dynamic PCC rule shall always
	/// have its binding with the QoS Flow associated with the default QoS
	/// rule",
	///      "type": "boolean"
	///    },
	///    "extMaxDataBurstVol": {
	///      "$ref": "#/components/schemas/ExtMaxDataBurstVolRm"
	///    },
	///    "gbrDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "gbrUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "maxDataBurstVol": {
	///      "$ref": "#/components/schemas/MaxDataBurstVolRm"
	///    },
	///    "maxPacketLossRateDl": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "maxPacketLossRateUl": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "maxbrDl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "maxbrUl": {
	///      "$ref": "#/components/schemas/BitRateRm"
	///    },
	///    "packetDelayBudget": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "packetErrorRate": {
	///      "$ref": "#/components/schemas/PacketErrRate"
	///    },
	///    "priorityLevel": {
	///      "$ref": "#/components/schemas/5QiPriorityLevelRm"
	///    },
	///    "qnc": {
	///      "description": "Indicates whether notifications are requested from
	/// 3GPP NG-RAN when the GFBR can no longer (or again) be guaranteed for a
	/// QoS Flow during the lifetime of the QoS Flow.\n",
	///      "type": "boolean"
	///    },
	///    "qosId": {
	///      "description": "Univocally identifies the QoS control policy data
	/// within a PDU session.",
	///      "type": "string"
	///    },
	///    "reflectiveQos": {
	///      "description": "Indicates whether the QoS information is reflective
	/// for the corresponding service data flow.",
	///      "type": "boolean"
	///    },
	///    "sharingKeyDl": {
	///      "description": "Indicates, by containing the same value, what PCC
	/// rules may share resource in downlink direction.",
	///      "type": "string"
	///    },
	///    "sharingKeyUl": {
	///      "description": "Indicates, by containing the same value, what PCC
	/// rules may share resource in uplink direction.",
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosDataInner {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub arp: Option<Arp>,
		#[serde(
			rename = "averWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub aver_window: Option<AverWindowRm>,
		/// Indicates that the dynamic PCC rule shall always have its binding
		/// with the QoS Flow associated with the default QoS rule
		#[serde(
			rename = "defQosFlowIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub def_qos_flow_indication: Option<bool>,
		#[serde(
			rename = "extMaxDataBurstVol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ext_max_data_burst_vol: Option<ExtMaxDataBurstVolRm>,
		#[serde(rename = "5qi", default, skip_serializing_if = "Option::is_none")]
		pub five_qi: Option<_5qi>,
		#[serde(rename = "gbrDl", default, skip_serializing_if = "Option::is_none")]
		pub gbr_dl: Option<BitRateRm>,
		#[serde(rename = "gbrUl", default, skip_serializing_if = "Option::is_none")]
		pub gbr_ul: Option<BitRateRm>,
		#[serde(
			rename = "maxDataBurstVol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_data_burst_vol: Option<MaxDataBurstVolRm>,
		#[serde(
			rename = "maxPacketLossRateDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_packet_loss_rate_dl: Option<PacketLossRateRm>,
		#[serde(
			rename = "maxPacketLossRateUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_packet_loss_rate_ul: Option<PacketLossRateRm>,
		#[serde(rename = "maxbrDl", default, skip_serializing_if = "Option::is_none")]
		pub maxbr_dl: Option<BitRateRm>,
		#[serde(rename = "maxbrUl", default, skip_serializing_if = "Option::is_none")]
		pub maxbr_ul: Option<BitRateRm>,
		#[serde(
			rename = "packetDelayBudget",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub packet_delay_budget: Option<PacketDelBudget>,
		#[serde(
			rename = "packetErrorRate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub packet_error_rate: Option<PacketErrRate>,
		#[serde(
			rename = "priorityLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub priority_level: Option<_5qiPriorityLevelRm>,
		/// Indicates whether notifications are requested from 3GPP NG-RAN when
		/// the GFBR can no longer (or again) be guaranteed for a QoS Flow
		/// during the lifetime of the QoS Flow.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub qnc: Option<bool>,
		/// Univocally identifies the QoS control policy data within a PDU
		/// session.
		#[serde(rename = "qosId")]
		pub qos_id: String,
		/// Indicates whether the QoS information is reflective for the
		/// corresponding service data flow.
		#[serde(
			rename = "reflectiveQos",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reflective_qos: Option<bool>,
		/// Indicates, by containing the same value, what PCC rules may share
		/// resource in downlink direction.
		#[serde(
			rename = "sharingKeyDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sharing_key_dl: Option<String>,
		/// Indicates, by containing the same value, what PCC rules may share
		/// resource in uplink direction.
		#[serde(
			rename = "sharingKeyUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sharing_key_ul: Option<String>,
	}

	impl From<&QosDataInner> for QosDataInner {
		fn from(value: &QosDataInner) -> Self {
			value.clone()
		}
	}

	/// QosFlowsUsageReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "endTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "qFI": {
	///      "$ref": "#/components/schemas/Qfi"
	///    },
	///    "startTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "uplinkVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QosFlowsUsageReport {
		#[serde(
			rename = "downlinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub downlink_volume: Option<Uint64>,
		#[serde(
			rename = "endTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub end_timestamp: Option<DateTime>,
		#[serde(rename = "qFI", default, skip_serializing_if = "Option::is_none")]
		pub q_fi: Option<Qfi>,
		#[serde(
			rename = "startTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub start_timestamp: Option<DateTime>,
		#[serde(
			rename = "uplinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uplink_volume: Option<Uint64>,
	}

	impl From<&QosFlowsUsageReport> for QosFlowsUsageReport {
		fn from(value: &QosFlowsUsageReport) -> Self {
			value.clone()
		}
	}

	/// Contains reporting information on QoS monitoring.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains reporting information on QoS monitoring.",
	///  "type": "object",
	///  "properties": {
	///    "dlDelays": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
	///      },
	///      "minItems": 0
	///    },
	///    "rtDelays": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
	///      },
	///      "minItems": 0
	///    },
	///    "ulDelays": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
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
	pub struct QosMonitoringReport {
		#[serde(rename = "dlDelays", default, skip_serializing_if = "Vec::is_empty")]
		pub dl_delays: Vec<i64>,
		#[serde(rename = "rtDelays", default, skip_serializing_if = "Vec::is_empty")]
		pub rt_delays: Vec<i64>,
		#[serde(rename = "ulDelays", default, skip_serializing_if = "Vec::is_empty")]
		pub ul_delays: Vec<i64>,
	}

	impl From<&QosMonitoringReport> for QosMonitoringReport {
		fn from(value: &QosMonitoringReport) -> Self {
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

	/// QuotaConsumptionIndicator
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "QUOTA_NOT_USED",
	///    "QUOTA_IS_USED"
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
	pub enum QuotaConsumptionIndicator {
		#[default]
		#[serde(rename = "QUOTA_NOT_USED")]
		QuotaNotUsed,
		#[serde(rename = "QUOTA_IS_USED")]
		QuotaIsUsed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&QuotaConsumptionIndicator> for QuotaConsumptionIndicator {
		fn from(value: &QuotaConsumptionIndicator) -> Self {
			value.clone()
		}
	}

	impl ToString for QuotaConsumptionIndicator {
		fn to_string(&self) -> String {
			match *self {
				Self::QuotaNotUsed => "QUOTA_NOT_USED".to_string(),
				Self::QuotaIsUsed => "QUOTA_IS_USED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for QuotaConsumptionIndicator {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"QUOTA_NOT_USED" => Ok(Self::QuotaNotUsed),
				"QUOTA_IS_USED" => Ok(Self::QuotaIsUsed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for QuotaConsumptionIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for QuotaConsumptionIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for QuotaConsumptionIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// QuotaManagementIndicator
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "ONLINE_CHARGING",
	///    "OFFLINE_CHARGING",
	///    "QUOTA_MANAGEMENT_SUSPENDED"
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
	pub enum QuotaManagementIndicator {
		#[default]
		#[serde(rename = "ONLINE_CHARGING")]
		OnlineCharging,
		#[serde(rename = "OFFLINE_CHARGING")]
		OfflineCharging,
		#[serde(rename = "QUOTA_MANAGEMENT_SUSPENDED")]
		QuotaManagementSuspended,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&QuotaManagementIndicator> for QuotaManagementIndicator {
		fn from(value: &QuotaManagementIndicator) -> Self {
			value.clone()
		}
	}

	impl ToString for QuotaManagementIndicator {
		fn to_string(&self) -> String {
			match *self {
				Self::OnlineCharging => "ONLINE_CHARGING".to_string(),
				Self::OfflineCharging => "OFFLINE_CHARGING".to_string(),
				Self::QuotaManagementSuspended => "QUOTA_MANAGEMENT_SUSPENDED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for QuotaManagementIndicator {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ONLINE_CHARGING" => Ok(Self::OnlineCharging),
				"OFFLINE_CHARGING" => Ok(Self::OfflineCharging),
				"QUOTA_MANAGEMENT_SUSPENDED" => Ok(Self::QuotaManagementSuspended),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for QuotaManagementIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for QuotaManagementIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for QuotaManagementIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// RadioParameterSetInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "changeTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "radioParameterSetValues": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/OctetString"
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
	pub struct RadioParameterSetInfo {
		#[serde(
			rename = "changeTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub change_timestamp: Option<DateTime>,
		#[serde(
			rename = "radioParameterSetValues",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub radio_parameter_set_values: Vec<OctetString>,
	}

	impl From<&RadioParameterSetInfo> for RadioParameterSetInfo {
		fn from(value: &RadioParameterSetInfo) -> Self {
			value.clone()
		}
	}

	/// RadioResourcesId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "OPERATOR_PROVIDED",
	///    "CONFIGURED"
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
	pub enum RadioResourcesId {
		#[default]
		#[serde(rename = "OPERATOR_PROVIDED")]
		OperatorProvided,
		#[serde(rename = "CONFIGURED")]
		Configured,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RadioResourcesId> for RadioResourcesId {
		fn from(value: &RadioResourcesId) -> Self {
			value.clone()
		}
	}

	impl ToString for RadioResourcesId {
		fn to_string(&self) -> String {
			match *self {
				Self::OperatorProvided => "OPERATOR_PROVIDED".to_string(),
				Self::Configured => "CONFIGURED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RadioResourcesId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"OPERATOR_PROVIDED" => Ok(Self::OperatorProvided),
				"CONFIGURED" => Ok(Self::Configured),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RadioResourcesId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RadioResourcesId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RadioResourcesId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// RanNasCauseList
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "array",
	///  "items": {
	///    "$ref": "#/components/schemas/RanNasRelCause"
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RanNasCauseList(pub Vec<RanNasRelCause>);

	impl ::std::ops::Deref for RanNasCauseList {
		type Target = Vec<RanNasRelCause>;
		fn deref(&self) -> &Vec<RanNasRelCause> {
			&self.0
		}
	}

	impl From<RanNasCauseList> for Vec<RanNasRelCause> {
		fn from(value: RanNasCauseList) -> Self {
			value.0
		}
	}

	impl From<&RanNasCauseList> for RanNasCauseList {
		fn from(value: &RanNasCauseList) -> Self {
			value.clone()
		}
	}

	impl From<Vec<RanNasRelCause>> for RanNasCauseList {
		fn from(value: Vec<RanNasRelCause>) -> Self {
			Self(value)
		}
	}

	/// Contains the RAN/NAS release cause.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the RAN/NAS release cause.",
	///  "type": "object",
	///  "properties": {
	///    "5gMmCause": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "5gSmCause": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "epsCause": {
	///      "$ref": "#/components/schemas/EpsRanNasRelCause"
	///    },
	///    "ngApCause": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RanNasRelCause {
		#[serde(rename = "epsCause", default, skip_serializing_if = "Option::is_none")]
		pub eps_cause: Option<EpsRanNasRelCause>,
		#[serde(rename = "5gMmCause", default, skip_serializing_if = "Option::is_none")]
		pub five_g_mm_cause: Option<Uinteger>,
		#[serde(rename = "5gSmCause", default, skip_serializing_if = "Option::is_none")]
		pub five_g_sm_cause: Option<Uinteger>,
		#[serde(rename = "ngApCause", default, skip_serializing_if = "Option::is_none")]
		pub ng_ap_cause: Option<NgApCause>,
	}

	impl From<&RanNasRelCause> for RanNasRelCause {
		fn from(value: &RanNasRelCause) -> Self {
			value.clone()
		}
	}

	/// RanSecondaryRatUsageReport
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "qosFlowsUsageReports": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosFlowsUsageReport"
	///      }
	///    },
	///    "rANSecondaryRATType": {
	///      "$ref": "#/components/schemas/RatType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RanSecondaryRatUsageReport {
		#[serde(
			rename = "qosFlowsUsageReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flows_usage_reports: Vec<QosFlowsUsageReport>,
		#[serde(
			rename = "rANSecondaryRATType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub r_an_secondary_rat_type: Option<RatType>,
	}

	impl From<&RanSecondaryRatUsageReport> for RanSecondaryRatUsageReport {
		fn from(value: &RanSecondaryRatUsageReport) -> Self {
			value.clone()
		}
	}

	/// RangeClass
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "RESERVED",
	///    "50_METER",
	///    "100_METER",
	///    "200_METER",
	///    "500_METER",
	///    "1000_METER",
	///    "UNUSED"
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
	pub enum RangeClass {
		#[default]
		#[serde(rename = "RESERVED")]
		Reserved,
		#[serde(rename = "50_METER")]
		FiftyMeter,
		#[serde(rename = "100_METER")]
		OneHundredMeter,
		#[serde(rename = "200_METER")]
		TwoHundredMeter,
		#[serde(rename = "500_METER")]
		FiveHundredMeter,
		#[serde(rename = "1000_METER")]
		OneThousandMeter,
		#[serde(rename = "UNUSED")]
		Unused,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RangeClass> for RangeClass {
		fn from(value: &RangeClass) -> Self {
			value.clone()
		}
	}

	impl ToString for RangeClass {
		fn to_string(&self) -> String {
			match *self {
				Self::Reserved => "RESERVED".to_string(),
				Self::FiftyMeter => "50_METER".to_string(),
				Self::OneHundredMeter => "100_METER".to_string(),
				Self::TwoHundredMeter => "200_METER".to_string(),
				Self::FiveHundredMeter => "500_METER".to_string(),
				Self::OneThousandMeter => "1000_METER".to_string(),
				Self::Unused => "UNUSED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RangeClass {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"RESERVED" => Ok(Self::Reserved),
				"50_METER" => Ok(Self::FiftyMeter),
				"100_METER" => Ok(Self::OneHundredMeter),
				"200_METER" => Ok(Self::TwoHundredMeter),
				"500_METER" => Ok(Self::FiveHundredMeter),
				"1000_METER" => Ok(Self::OneThousandMeter),
				"UNUSED" => Ok(Self::Unused),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RangeClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RangeClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RangeClass {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the RAT type and/or Frequency information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the RAT type and/or Frequency information.",
	///  "type": "object",
	///  "properties": {
	///    "allFreq": {
	///      "description": "Set to \"true\" to indicate to handle all the
	/// frequencies the NWDAF received, otherwise set to \"false\" or omit. The
	/// \"allFreq\" attribute and the \"freq\" attribute are mutually
	/// exclusive.\n",
	///      "type": "boolean"
	///    },
	///    "allRat": {
	///      "description": "Set to \"true\" to indicate to handle all the RAT
	/// Types the NWDAF received, otherwise set to \"false\" or omit. The
	/// \"allRat\" attribute and the \"ratType\" attribute are mutually
	/// exclusive.\n",
	///      "type": "boolean"
	///    },
	///    "freq": {
	///      "$ref": "#/components/schemas/ArfcnValueNR"
	///    },
	///    "matchingDir": {
	///      "$ref": "#/components/schemas/MatchingDirection"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "svcExpThreshold": {
	///      "$ref": "#/components/schemas/ThresholdLevel"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RatFreqInformation {
		/// Set to "true" to indicate to handle all the frequencies the NWDAF
		/// received, otherwise set to "false" or omit. The "allFreq" attribute
		/// and the "freq" attribute are mutually exclusive.
		#[serde(rename = "allFreq", default, skip_serializing_if = "Option::is_none")]
		pub all_freq: Option<bool>,
		/// Set to "true" to indicate to handle all the RAT Types the NWDAF
		/// received, otherwise set to "false" or omit. The "allRat" attribute
		/// and the "ratType" attribute are mutually exclusive.
		#[serde(rename = "allRat", default, skip_serializing_if = "Option::is_none")]
		pub all_rat: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub freq: Option<ArfcnValueNr>,
		#[serde(
			rename = "matchingDir",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub matching_dir: Option<MatchingDirection>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "svcExpThreshold",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub svc_exp_threshold: Option<ThresholdLevel>,
	}

	impl From<&RatFreqInformation> for RatFreqInformation {
		fn from(value: &RatFreqInformation) -> Self {
			value.clone()
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

	/// ReauthorizationDetails
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "quotaManagementIndicator": {
	///      "$ref": "#/components/schemas/QuotaManagementIndicator"
	///    },
	///    "ratingGroup": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "serviceId": {
	///      "$ref": "#/components/schemas/Uint32"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReauthorizationDetails {
		#[serde(
			rename = "quotaManagementIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub quota_management_indicator: Option<QuotaManagementIndicator>,
		#[serde(
			rename = "ratingGroup",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rating_group: Option<Uint32>,
		#[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
		pub service_id: Option<Uint32>,
	}

	impl From<&ReauthorizationDetails> for ReauthorizationDetails {
		fn from(value: &ReauthorizationDetails) -> Self {
			value.clone()
		}
	}

	/// RecipientAddress
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "recipientAddressInfo": {
	///      "$ref": "#/components/schemas/SMAddressInfo"
	///    },
	///    "sMaddresseeType": {
	///      "$ref": "#/components/schemas/SMAddresseeType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RecipientAddress {
		#[serde(
			rename = "recipientAddressInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recipient_address_info: Option<SmAddressInfo>,
		#[serde(
			rename = "sMaddresseeType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_maddressee_type: Option<SmAddresseeType>,
	}

	impl From<&RecipientAddress> for RecipientAddress {
		fn from(value: &RecipientAddress) -> Self {
			value.clone()
		}
	}

	/// RecipientInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "recipientGPSI": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "recipientOtherAddress": {
	///      "$ref": "#/components/schemas/SMAddressInfo"
	///    },
	///    "recipientReceivedAddress": {
	///      "$ref": "#/components/schemas/SMAddressInfo"
	///    },
	///    "recipientSCCPAddress": {
	///      "type": "string"
	///    },
	///    "recipientSUPI": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "sMDestinationInterface": {
	///      "$ref": "#/components/schemas/SMInterface"
	///    },
	///    "sMrecipientProtocolId": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RecipientInfo {
		#[serde(
			rename = "recipientGPSI",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recipient_gpsi: Option<Gpsi>,
		#[serde(
			rename = "recipientOtherAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recipient_other_address: Option<SmAddressInfo>,
		#[serde(
			rename = "recipientReceivedAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recipient_received_address: Option<SmAddressInfo>,
		#[serde(
			rename = "recipientSCCPAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recipient_sccp_address: Option<String>,
		#[serde(
			rename = "recipientSUPI",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recipient_supi: Option<Supi>,
		#[serde(
			rename = "sMDestinationInterface",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_destination_interface: Option<SmInterface>,
		#[serde(
			rename = "sMrecipientProtocolId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_mrecipient_protocol_id: Option<String>,
	}

	impl From<&RecipientInfo> for RecipientInfo {
		fn from(value: &RecipientInfo) -> Self {
			value.clone()
		}
	}

	/// RedirectAddressType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "IPV4",
	///    "IPV6",
	///    "URL",
	///    "URI"
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
	pub enum RedirectAddressType {
		#[default]
		#[serde(rename = "IPV4")]
		Ipv4,
		#[serde(rename = "IPV6")]
		Ipv6,
		#[serde(rename = "URL")]
		Url,
		#[serde(rename = "URI")]
		Uri,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RedirectAddressType> for RedirectAddressType {
		fn from(value: &RedirectAddressType) -> Self {
			value.clone()
		}
	}

	impl ToString for RedirectAddressType {
		fn to_string(&self) -> String {
			match *self {
				Self::Ipv4 => "IPV4".to_string(),
				Self::Ipv6 => "IPV6".to_string(),
				Self::Url => "URL".to_string(),
				Self::Uri => "URI".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RedirectAddressType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IPV4" => Ok(Self::Ipv4),
				"IPV6" => Ok(Self::Ipv6),
				"URL" => Ok(Self::Url),
				"URI" => Ok(Self::Uri),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RedirectAddressType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RedirectAddressType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RedirectAddressType {
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

	/// RedirectServer
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "redirectAddressType",
	///    "redirectServerAddress"
	///  ],
	///  "properties": {
	///    "redirectAddressType": {
	///      "$ref": "#/components/schemas/RedirectAddressType"
	///    },
	///    "redirectServerAddress": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RedirectServer {
		#[serde(rename = "redirectAddressType")]
		pub redirect_address_type: RedirectAddressType,
		#[serde(rename = "redirectServerAddress")]
		pub redirect_server_address: String,
	}

	impl From<&RedirectServer> for RedirectServer {
		fn from(value: &RedirectServer) -> Self {
			value.clone()
		}
	}

	/// RedundantTransmissionType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "NON_TRANSMISSION",
	///    "END_TO_END_USER_PLANE_PATHS",
	///    "N3/N9",
	///    "TRANSPORT_LAYER"
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
	pub enum RedundantTransmissionType {
		#[default]
		#[serde(rename = "NON_TRANSMISSION")]
		NonTransmission,
		#[serde(rename = "END_TO_END_USER_PLANE_PATHS")]
		EndToEndUserPlanePaths,
		#[serde(rename = "N3/N9")]
		N3N9,
		#[serde(rename = "TRANSPORT_LAYER")]
		TransportLayer,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RedundantTransmissionType> for RedundantTransmissionType {
		fn from(value: &RedundantTransmissionType) -> Self {
			value.clone()
		}
	}

	impl ToString for RedundantTransmissionType {
		fn to_string(&self) -> String {
			match *self {
				Self::NonTransmission => "NON_TRANSMISSION".to_string(),
				Self::EndToEndUserPlanePaths => "END_TO_END_USER_PLANE_PATHS".to_string(),
				Self::N3N9 => "N3/N9".to_string(),
				Self::TransportLayer => "TRANSPORT_LAYER".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RedundantTransmissionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NON_TRANSMISSION" => Ok(Self::NonTransmission),
				"END_TO_END_USER_PLANE_PATHS" => Ok(Self::EndToEndUserPlanePaths),
				"N3/N9" => Ok(Self::N3N9),
				"TRANSPORT_LAYER" => Ok(Self::TransportLayer),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RedundantTransmissionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RedundantTransmissionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RedundantTransmissionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// RegistrationChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "registrationMessagetype"
	///  ],
	///  "properties": {
	///    "5GMMCapability": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "allowedNSSAI": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 0
	///    },
	///    "amfUeNgapId": {
	///      "type": "integer"
	///    },
	///    "mICOModeIndication": {
	///      "$ref": "#/components/schemas/MICOModeIndication"
	///    },
	///    "nSSAIMapList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NSSAIMap"
	///      },
	///      "minItems": 0
	///    },
	///    "pSCellInformation": {
	///      "$ref": "#/components/schemas/PSCellInformation"
	///    },
	///    "rATType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "ranNodeId": {
	///      "$ref": "#/components/schemas/GlobalRanNodeId"
	///    },
	///    "ranUeNgapId": {
	///      "type": "integer"
	///    },
	///    "registrationMessagetype": {
	///      "$ref": "#/components/schemas/RegistrationMessageType"
	///    },
	///    "rejectedNSSAI": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 0
	///    },
	///    "requestedNSSAI": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 0
	///    },
	///    "serviceAreaRestriction": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServiceAreaRestriction"
	///      },
	///      "minItems": 0
	///    },
	///    "smsIndication": {
	///      "$ref": "#/components/schemas/SmsIndication"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 0
	///    },
	///    "uetimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userInformation": {
	///      "$ref": "#/components/schemas/UserInformation"
	///    },
	///    "userLocationinfo": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RegistrationChargingInformation {
		#[serde(
			rename = "allowedNSSAI",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_nssai: Vec<Snssai>,
		#[serde(
			rename = "amfUeNgapId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub amf_ue_ngap_id: Option<i64>,
		#[serde(
			rename = "5GMMCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_gmm_capability: Option<Bytes>,
		#[serde(
			rename = "mICOModeIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub m_ico_mode_indication: Option<MicoModeIndication>,
		#[serde(
			rename = "nSSAIMapList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub n_ssai_map_list: Vec<NssaiMap>,
		#[serde(
			rename = "pSCellInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub p_s_cell_information: Option<PsCellInformation>,
		#[serde(rename = "rATType", default, skip_serializing_if = "Option::is_none")]
		pub r_at_type: Option<RatType>,
		#[serde(rename = "ranNodeId", default, skip_serializing_if = "Option::is_none")]
		pub ran_node_id: Option<GlobalRanNodeId>,
		#[serde(
			rename = "ranUeNgapId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ran_ue_ngap_id: Option<i64>,
		#[serde(rename = "registrationMessagetype")]
		pub registration_messagetype: RegistrationMessageType,
		#[serde(
			rename = "rejectedNSSAI",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub rejected_nssai: Vec<Snssai>,
		#[serde(
			rename = "requestedNSSAI",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub requested_nssai: Vec<Snssai>,
		#[serde(
			rename = "serviceAreaRestriction",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub service_area_restriction: Vec<ServiceAreaRestriction>,
		#[serde(
			rename = "smsIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sms_indication: Option<SmsIndication>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "uetimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uetime_zone: Option<TimeZone>,
		#[serde(
			rename = "userInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_information: Option<UserInformation>,
		#[serde(
			rename = "userLocationinfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_locationinfo: Option<UserLocation>,
	}

	impl From<&RegistrationChargingInformation> for RegistrationChargingInformation {
		fn from(value: &RegistrationChargingInformation) -> Self {
			value.clone()
		}
	}

	/// RegistrationMessageType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "INITIAL",
	///    "MOBILITY",
	///    "PERIODIC",
	///    "EMERGENCY",
	///    "DEREGISTRATION"
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
	pub enum RegistrationMessageType {
		#[default]
		#[serde(rename = "INITIAL")]
		Initial,
		#[serde(rename = "MOBILITY")]
		Mobility,
		#[serde(rename = "PERIODIC")]
		Periodic,
		#[serde(rename = "EMERGENCY")]
		Emergency,
		#[serde(rename = "DEREGISTRATION")]
		Deregistration,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RegistrationMessageType> for RegistrationMessageType {
		fn from(value: &RegistrationMessageType) -> Self {
			value.clone()
		}
	}

	impl ToString for RegistrationMessageType {
		fn to_string(&self) -> String {
			match *self {
				Self::Initial => "INITIAL".to_string(),
				Self::Mobility => "MOBILITY".to_string(),
				Self::Periodic => "PERIODIC".to_string(),
				Self::Emergency => "EMERGENCY".to_string(),
				Self::Deregistration => "DEREGISTRATION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RegistrationMessageType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"INITIAL" => Ok(Self::Initial),
				"MOBILITY" => Ok(Self::Mobility),
				"PERIODIC" => Ok(Self::Periodic),
				"EMERGENCY" => Ok(Self::Emergency),
				"DEREGISTRATION" => Ok(Self::Deregistration),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RegistrationMessageType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RegistrationMessageType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RegistrationMessageType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ReplyPathRequested
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "NO_REPLY_PATH_SET",
	///    "REPLY_PATH_SET"
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
	pub enum ReplyPathRequested {
		#[default]
		#[serde(rename = "NO_REPLY_PATH_SET")]
		NoReplyPathSet,
		#[serde(rename = "REPLY_PATH_SET")]
		ReplyPathSet,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReplyPathRequested> for ReplyPathRequested {
		fn from(value: &ReplyPathRequested) -> Self {
			value.clone()
		}
	}

	impl ToString for ReplyPathRequested {
		fn to_string(&self) -> String {
			match *self {
				Self::NoReplyPathSet => "NO_REPLY_PATH_SET".to_string(),
				Self::ReplyPathSet => "REPLY_PATH_SET".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReplyPathRequested {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NO_REPLY_PATH_SET" => Ok(Self::NoReplyPathSet),
				"REPLY_PATH_SET" => Ok(Self::ReplyPathSet),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReplyPathRequested {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReplyPathRequested {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReplyPathRequested {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// RequestedUnit
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "serviceSpecificUnits": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "time": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "totalVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "uplinkVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RequestedUnit {
		#[serde(
			rename = "downlinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub downlink_volume: Option<Uint64>,
		#[serde(
			rename = "serviceSpecificUnits",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_specific_units: Option<Uint64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub time: Option<Uint32>,
		#[serde(
			rename = "totalVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub total_volume: Option<Uint64>,
		#[serde(
			rename = "uplinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uplink_volume: Option<Uint64>,
	}

	impl From<&RequestedUnit> for RequestedUnit {
		fn from(value: &RequestedUnit) -> Self {
			value.clone()
		}
	}

	/// The current usage of the virtual resources assigned to the NF instances
	/// belonging to a particular network slice instance.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The current usage of the virtual resources assigned to the NF instances belonging to a particular network slice instance.\n",
	///  "type": "object",
	///  "properties": {
	///    "cpuUsage": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "memoryUsage": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "storageUsage": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ResourceUsage {
		#[serde(rename = "cpuUsage", default, skip_serializing_if = "Option::is_none")]
		pub cpu_usage: Option<Uinteger>,
		#[serde(
			rename = "memoryUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub memory_usage: Option<Uinteger>,
		#[serde(
			rename = "storageUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub storage_usage: Option<Uinteger>,
	}

	impl From<&ResourceUsage> for ResourceUsage {
		fn from(value: &ResourceUsage) -> Self {
			value.clone()
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

	/// ResultCode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "SUCCESS",
	///    "END_USER_SERVICE_DENIED",
	///    "QUOTA_MANAGEMENT_NOT_APPLICABLE",
	///    "QUOTA_LIMIT_REACHED",
	///    "END_USER_SERVICE_REJECTED",
	///    "USER_UNKNOWN",
	///    "RATING_FAILED",
	///    "QUOTA_MANAGEMENT"
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
	pub enum ResultCode {
		#[default]
		#[serde(rename = "SUCCESS")]
		Success,
		#[serde(rename = "END_USER_SERVICE_DENIED")]
		EndUserServiceDenied,
		#[serde(rename = "QUOTA_MANAGEMENT_NOT_APPLICABLE")]
		QuotaManagementNotApplicable,
		#[serde(rename = "QUOTA_LIMIT_REACHED")]
		QuotaLimitReached,
		#[serde(rename = "END_USER_SERVICE_REJECTED")]
		EndUserServiceRejected,
		#[serde(rename = "USER_UNKNOWN")]
		UserUnknown,
		#[serde(rename = "RATING_FAILED")]
		RatingFailed,
		#[serde(rename = "QUOTA_MANAGEMENT")]
		QuotaManagement,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ResultCode> for ResultCode {
		fn from(value: &ResultCode) -> Self {
			value.clone()
		}
	}

	impl ToString for ResultCode {
		fn to_string(&self) -> String {
			match *self {
				Self::Success => "SUCCESS".to_string(),
				Self::EndUserServiceDenied => "END_USER_SERVICE_DENIED".to_string(),
				Self::QuotaManagementNotApplicable => "QUOTA_MANAGEMENT_NOT_APPLICABLE".to_string(),
				Self::QuotaLimitReached => "QUOTA_LIMIT_REACHED".to_string(),
				Self::EndUserServiceRejected => "END_USER_SERVICE_REJECTED".to_string(),
				Self::UserUnknown => "USER_UNKNOWN".to_string(),
				Self::RatingFailed => "RATING_FAILED".to_string(),
				Self::QuotaManagement => "QUOTA_MANAGEMENT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ResultCode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SUCCESS" => Ok(Self::Success),
				"END_USER_SERVICE_DENIED" => Ok(Self::EndUserServiceDenied),
				"QUOTA_MANAGEMENT_NOT_APPLICABLE" => Ok(Self::QuotaManagementNotApplicable),
				"QUOTA_LIMIT_REACHED" => Ok(Self::QuotaLimitReached),
				"END_USER_SERVICE_REJECTED" => Ok(Self::EndUserServiceRejected),
				"USER_UNKNOWN" => Ok(Self::UserUnknown),
				"RATING_FAILED" => Ok(Self::RatingFailed),
				"QUOTA_MANAGEMENT" => Ok(Self::QuotaManagement),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ResultCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ResultCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ResultCode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// RoamerInOut
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "IN_BOUND",
	///    "OUT_BOUND"
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
	pub enum RoamerInOut {
		#[default]
		#[serde(rename = "IN_BOUND")]
		InBound,
		#[serde(rename = "OUT_BOUND")]
		OutBound,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RoamerInOut> for RoamerInOut {
		fn from(value: &RoamerInOut) -> Self {
			value.clone()
		}
	}

	impl ToString for RoamerInOut {
		fn to_string(&self) -> String {
			match *self {
				Self::InBound => "IN_BOUND".to_string(),
				Self::OutBound => "OUT_BOUND".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RoamerInOut {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IN_BOUND" => Ok(Self::InBound),
				"OUT_BOUND" => Ok(Self::OutBound),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RoamerInOut {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RoamerInOut {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RoamerInOut {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// RoamingQbcInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "multipleQFIcontainer": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MultipleQFIcontainer"
	///      },
	///      "minItems": 0
	///    },
	///    "roamingChargingProfile": {
	///      "$ref": "#/components/schemas/RoamingChargingProfile"
	///    },
	///    "uPFID": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RoamingQbcInformation {
		#[serde(
			rename = "multipleQFIcontainer",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub multiple_qf_icontainer: Vec<MultipleQfIcontainer>,
		#[serde(
			rename = "roamingChargingProfile",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub roaming_charging_profile: Option<RoamingChargingProfile>,
		#[serde(rename = "uPFID", default, skip_serializing_if = "Option::is_none")]
		pub u_pfid: Option<NfInstanceId>,
	}

	impl From<&RoamingQbcInformation> for RoamingQbcInformation {
		fn from(value: &RoamingQbcInformation) -> Self {
			value.clone()
		}
	}

	/// RoleOfImsNode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "ORIGINATING",
	///    "TERMINATING",
	///    "FORWARDING"
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
	pub enum RoleOfImsNode {
		#[default]
		#[serde(rename = "ORIGINATING")]
		Originating,
		#[serde(rename = "TERMINATING")]
		Terminating,
		#[serde(rename = "FORWARDING")]
		Forwarding,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RoleOfImsNode> for RoleOfImsNode {
		fn from(value: &RoleOfImsNode) -> Self {
			value.clone()
		}
	}

	impl ToString for RoleOfImsNode {
		fn to_string(&self) -> String {
			match *self {
				Self::Originating => "ORIGINATING".to_string(),
				Self::Terminating => "TERMINATING".to_string(),
				Self::Forwarding => "FORWARDING".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RoleOfImsNode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ORIGINATING" => Ok(Self::Originating),
				"TERMINATING" => Ok(Self::Terminating),
				"FORWARDING" => Ok(Self::Forwarding),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RoleOfImsNode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RoleOfImsNode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RoleOfImsNode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// RoleOfUe
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "ANNOUNCING_UE",
	///    "MONITORING_UE",
	///    "REQUESTOR_UE",
	///    "REQUESTED_UE"
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
	pub enum RoleOfUe {
		#[default]
		#[serde(rename = "ANNOUNCING_UE")]
		AnnouncingUe,
		#[serde(rename = "MONITORING_UE")]
		MonitoringUe,
		#[serde(rename = "REQUESTOR_UE")]
		RequestorUe,
		#[serde(rename = "REQUESTED_UE")]
		RequestedUe,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RoleOfUe> for RoleOfUe {
		fn from(value: &RoleOfUe) -> Self {
			value.clone()
		}
	}

	impl ToString for RoleOfUe {
		fn to_string(&self) -> String {
			match *self {
				Self::AnnouncingUe => "ANNOUNCING_UE".to_string(),
				Self::MonitoringUe => "MONITORING_UE".to_string(),
				Self::RequestorUe => "REQUESTOR_UE".to_string(),
				Self::RequestedUe => "REQUESTED_UE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RoleOfUe {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ANNOUNCING_UE" => Ok(Self::AnnouncingUe),
				"MONITORING_UE" => Ok(Self::MonitoringUe),
				"REQUESTOR_UE" => Ok(Self::RequestorUe),
				"REQUESTED_UE" => Ok(Self::RequestedUe),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RoleOfUe {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RoleOfUe {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RoleOfUe {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// SchemasMcc
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]{3}$"
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
	pub struct SchemasMcc(String);

	impl ::std::ops::Deref for SchemasMcc {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasMcc> for String {
		fn from(value: SchemasMcc) -> Self {
			value.0
		}
	}

	impl From<&SchemasMcc> for SchemasMcc {
		fn from(value: &SchemasMcc) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SchemasMcc {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{3}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{3}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SchemasMcc {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SchemasMcc {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SchemasMcc {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SchemasMcc {
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

	/// SchemasMnc
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]{2,3}$"
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
	pub struct SchemasMnc(String);

	impl ::std::ops::Deref for SchemasMnc {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasMnc> for String {
		fn from(value: SchemasMnc) -> Self {
			value.0
		}
	}

	impl From<&SchemasMnc> for SchemasMnc {
		fn from(value: &SchemasMnc) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SchemasMnc {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{2,3}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{2,3}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SchemasMnc {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SchemasMnc {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SchemasMnc {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SchemasMnc {
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

	/// SchemasPlmnId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "mcc": {
	///      "$ref": "#/components/schemas/schemas-Mcc"
	///    },
	///    "mnc": {
	///      "$ref": "#/components/schemas/schemas-Mnc"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasPlmnId {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub mcc: Option<SchemasMcc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub mnc: Option<SchemasMnc>,
	}

	impl From<&SchemasPlmnId> for SchemasPlmnId {
		fn from(value: &SchemasPlmnId) -> Self {
			value.clone()
		}
	}

	/// SchemasTac
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub struct SchemasTac(String);

	impl ::std::ops::Deref for SchemasTac {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasTac> for String {
		fn from(value: SchemasTac) -> Self {
			value.0
		}
	}

	impl From<&SchemasTac> for SchemasTac {
		fn from(value: &SchemasTac) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SchemasTac {
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

	impl ::std::convert::TryFrom<&str> for SchemasTac {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SchemasTac {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SchemasTac {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SchemasTac {
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

	/// SchemasTai
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "mcc": {
	///      "$ref": "#/components/schemas/schemas-Mcc"
	///    },
	///    "mnc": {
	///      "$ref": "#/components/schemas/schemas-Mnc"
	///    },
	///    "tac": {
	///      "$ref": "#/components/schemas/schemas-Tac"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasTai {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub mcc: Option<SchemasMcc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub mnc: Option<SchemasMnc>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub tac: Option<SchemasTac>,
	}

	impl From<&SchemasTai> for SchemasTai {
		fn from(value: &SchemasTai) -> Self {
			value.clone()
		}
	}

	/// SdpMediaComponent
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "SDPMediaDescription": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 0
	///    },
	///    "accessNetworkChargingIdentifierValue": {
	///      "$ref": "#/components/schemas/OctetString"
	///    },
	///    "ipRealmDefaultIndication": {
	///      "type": "boolean"
	///    },
	///    "localGWInsertedIndication": {
	///      "type": "boolean"
	///    },
	///    "mediaInitiatorFlag": {
	///      "$ref": "#/components/schemas/MediaInitiatorFlag"
	///    },
	///    "mediaInitiatorParty": {
	///      "type": "string"
	///    },
	///    "sDPMediaName": {
	///      "type": "string"
	///    },
	///    "sDPType": {
	///      "$ref": "#/components/schemas/SDPType"
	///    },
	///    "threeGPPChargingId": {
	///      "$ref": "#/components/schemas/OctetString"
	///    },
	///    "transcoderInsertedIndication": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SdpMediaComponent {
		#[serde(
			rename = "accessNetworkChargingIdentifierValue",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_network_charging_identifier_value: Option<OctetString>,
		#[serde(
			rename = "ipRealmDefaultIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ip_realm_default_indication: Option<bool>,
		#[serde(
			rename = "localGWInsertedIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub local_gw_inserted_indication: Option<bool>,
		#[serde(
			rename = "mediaInitiatorFlag",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub media_initiator_flag: Option<MediaInitiatorFlag>,
		#[serde(
			rename = "mediaInitiatorParty",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub media_initiator_party: Option<String>,
		#[serde(
			rename = "sDPMediaName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_dp_media_name: Option<String>,
		#[serde(rename = "sDPType", default, skip_serializing_if = "Option::is_none")]
		pub s_dp_type: Option<SdpType>,
		#[serde(
			rename = "SDPMediaDescription",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub sdp_media_description: Vec<String>,
		#[serde(
			rename = "threeGPPChargingId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gpp_charging_id: Option<OctetString>,
		#[serde(
			rename = "transcoderInsertedIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub transcoder_inserted_indication: Option<bool>,
	}

	impl From<&SdpMediaComponent> for SdpMediaComponent {
		fn from(value: &SdpMediaComponent) -> Self {
			value.clone()
		}
	}

	/// SdpTimeStamps
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "sDPAnswerTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "sDPOfferTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SdpTimeStamps {
		#[serde(
			rename = "sDPAnswerTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_dp_answer_timestamp: Option<DateTime>,
		#[serde(
			rename = "sDPOfferTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_dp_offer_timestamp: Option<DateTime>,
	}

	impl From<&SdpTimeStamps> for SdpTimeStamps {
		fn from(value: &SdpTimeStamps) -> Self {
			value.clone()
		}
	}

	/// SdpType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "OFFER",
	///    "ANSWER"
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
	pub enum SdpType {
		#[default]
		#[serde(rename = "OFFER")]
		Offer,
		#[serde(rename = "ANSWER")]
		Answer,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SdpType> for SdpType {
		fn from(value: &SdpType) -> Self {
			value.clone()
		}
	}

	impl ToString for SdpType {
		fn to_string(&self) -> String {
			match *self {
				Self::Offer => "OFFER".to_string(),
				Self::Answer => "ANSWER".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SdpType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"OFFER" => Ok(Self::Offer),
				"ANSWER" => Ok(Self::Answer),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SdpType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SdpType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SdpType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ServerCapabilities
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "mandatoryCapability": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uint32"
	///      },
	///      "minItems": 0
	///    },
	///    "optionalCapability ": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uint32"
	///      },
	///      "minItems": 0
	///    },
	///    "serverName": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
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
	pub struct ServerCapabilities {
		#[serde(
			rename = "mandatoryCapability",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mandatory_capability: Vec<Uint32>,
		#[serde(
			rename = "optionalCapability\u{a0}",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub optional_capability: Vec<Uint32>,
		#[serde(rename = "serverName", default, skip_serializing_if = "Vec::is_empty")]
		pub server_name: Vec<String>,
	}

	impl From<&ServerCapabilities> for ServerCapabilities {
		fn from(value: &ServerCapabilities) -> Self {
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

	/// Represents service experience information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents service experience information.",
	///  "type": "object",
	///  "required": [
	///    "svcExprc"
	///  ],
	///  "properties": {
	///    "appId": {
	///      "$ref": "#/components/schemas/ApplicationId"
	///    },
	///    "appServerInst": {
	///      "$ref": "#/components/schemas/AddrFqdn"
	///    },
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "dnai": {
	///      "$ref": "#/components/schemas/Dnai"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "networkArea": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "nsiId": {
	///      "$ref": "#/components/schemas/NsiId"
	///    },
	///    "ratFreq": {
	///      "$ref": "#/components/schemas/RatFreqInformation"
	///    },
	///    "ratio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "srvExpcType": {
	///      "$ref": "#/components/schemas/ServiceExperienceType"
	///    },
	///    "supis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "svcExprc": {
	///      "$ref": "#/components/schemas/SvcExperience"
	///    },
	///    "svcExprcVariance": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "ueLocs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/LocationInfo"
	///      },
	///      "minItems": 1
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
	pub struct ServiceExperienceInfo {
		#[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
		pub app_id: Option<ApplicationId>,
		#[serde(
			rename = "appServerInst",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub app_server_inst: Option<AddrFqdn>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub confidence: Option<Uinteger>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnai: Option<Dnai>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "networkArea",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub network_area: Option<NetworkAreaInfo>,
		#[serde(rename = "nsiId", default, skip_serializing_if = "Option::is_none")]
		pub nsi_id: Option<NsiId>,
		#[serde(rename = "ratFreq", default, skip_serializing_if = "Option::is_none")]
		pub rat_freq: Option<RatFreqInformation>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ratio: Option<SamplingRatio>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(
			rename = "srvExpcType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub srv_expc_type: Option<ServiceExperienceType>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub supis: Vec<Supi>,
		#[serde(rename = "svcExprc")]
		pub svc_exprc: SvcExperience,
		#[serde(
			rename = "svcExprcVariance",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub svc_exprc_variance: Option<Float>,
		#[serde(rename = "ueLocs", default, skip_serializing_if = "Vec::is_empty")]
		pub ue_locs: Vec<LocationInfo>,
		#[serde(rename = "upfInfo", default, skip_serializing_if = "Option::is_none")]
		pub upf_info: Option<UpfInformation>,
	}

	impl From<&ServiceExperienceInfo> for ServiceExperienceInfo {
		fn from(value: &ServiceExperienceInfo) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - VOICE: Indicates that the service experience analytics is for voice
	///   service.
	/// - VIDEO: Indicates that the service experience analytics is for video
	///   service.
	/// - OTHER: Indicates that the service experience analytics is for other
	///   service.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- VOICE: Indicates that the
	/// service experience analytics is for voice service.\n- VIDEO: Indicates
	/// that the service experience analytics is for video service.\n- OTHER:
	/// Indicates that the service experience analytics is for other
	/// service.\n",
	///  "type": "string",
	///  "enum": [
	///    "VOICE",
	///    "VIDEO",
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
	pub enum ServiceExperienceType {
		#[default]
		#[serde(rename = "VOICE")]
		Voice,
		#[serde(rename = "VIDEO")]
		Video,
		#[serde(rename = "OTHER")]
		Other,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ServiceExperienceType> for ServiceExperienceType {
		fn from(value: &ServiceExperienceType) -> Self {
			value.clone()
		}
	}

	impl ToString for ServiceExperienceType {
		fn to_string(&self) -> String {
			match *self {
				Self::Voice => "VOICE".to_string(),
				Self::Video => "VIDEO".to_string(),
				Self::Other => "OTHER".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ServiceExperienceType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"VOICE" => Ok(Self::Voice),
				"VIDEO" => Ok(Self::Video),
				"OTHER" => Ok(Self::Other),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ServiceExperienceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ServiceExperienceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ServiceExperienceType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ServiceProfileChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "addServiceProfileInfo": {
	///      "type": "string"
	///    },
	///    "availability": {
	///      "type": "number"
	///    },
	///    "coverageArea": {
	///      "type": "string"
	///    },
	///    "dLThptPerSlice": {
	///      "$ref": "#/components/schemas/Throughput"
	///    },
	///    "dLThptPerUE": {
	///      "$ref": "#/components/schemas/Throughput"
	///    },
	///    "jitter": {
	///      "type": "integer"
	///    },
	///    "kPIMonitoringList": {
	///      "type": "string"
	///    },
	///    "latency": {
	///      "type": "integer"
	///    },
	///    "maxNumberofPDUsessions": {
	///      "type": "integer"
	///    },
	///    "maxNumberofUEs": {
	///      "type": "integer"
	///    },
	///    "reliability": {
	///      "type": "string"
	///    },
	///    "resourceSharingLevel": {
	///      "$ref": "#/components/schemas/SharingLevel"
	///    },
	///    "sNSSAIList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 0
	///    },
	///    "sST": {
	///      "$ref": "#/components/schemas/Sst"
	///    },
	///    "serviceProfileIdentifier": {
	///      "type": "string"
	///    },
	///    "supportedAccessTechnology": {
	///      "type": "integer"
	///    },
	///    "uLThptPerSlice": {
	///      "$ref": "#/components/schemas/Throughput"
	///    },
	///    "uLThptPerUE": {
	///      "$ref": "#/components/schemas/Throughput"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ServiceProfileChargingInformation {
		#[serde(
			rename = "addServiceProfileInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub add_service_profile_info: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub availability: Option<f64>,
		#[serde(
			rename = "coverageArea",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub coverage_area: Option<String>,
		#[serde(
			rename = "dLThptPerSlice",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub d_l_thpt_per_slice: Option<Throughput>,
		#[serde(
			rename = "dLThptPerUE",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub d_l_thpt_per_ue: Option<Throughput>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub jitter: Option<i64>,
		#[serde(
			rename = "kPIMonitoringList",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub k_pi_monitoring_list: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub latency: Option<i64>,
		#[serde(
			rename = "maxNumberofPDUsessions",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_numberof_pd_usessions: Option<i64>,
		#[serde(
			rename = "maxNumberofUEs",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_numberof_u_es: Option<i64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub reliability: Option<String>,
		#[serde(
			rename = "resourceSharingLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub resource_sharing_level: Option<SharingLevel>,
		#[serde(rename = "sNSSAIList", default, skip_serializing_if = "Vec::is_empty")]
		pub s_nssai_list: Vec<Snssai>,
		#[serde(rename = "sST", default, skip_serializing_if = "Option::is_none")]
		pub s_st: Option<Sst>,
		#[serde(
			rename = "serviceProfileIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_profile_identifier: Option<String>,
		#[serde(
			rename = "supportedAccessTechnology",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_access_technology: Option<i64>,
		#[serde(
			rename = "uLThptPerSlice",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub u_l_thpt_per_slice: Option<Throughput>,
		#[serde(
			rename = "uLThptPerUE",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub u_l_thpt_per_ue: Option<Throughput>,
	}

	impl From<&ServiceProfileChargingInformation> for ServiceProfileChargingInformation {
		fn from(value: &ServiceProfileChargingInformation) -> Self {
			value.clone()
		}
	}

	/// ServingLocation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "geographicalLocation": {
	///      "$ref": "#/components/schemas/GeoLoc"
	///    },
	///    "topologicalLocation": {
	///      "$ref": "#/components/schemas/TopologicalServiceArea"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ServingLocation {
		#[serde(
			rename = "geographicalLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub geographical_location: Option<GeoLoc>,
		#[serde(
			rename = "topologicalLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub topological_location: Option<TopologicalServiceArea>,
	}

	impl From<&ServingLocation> for ServingLocation {
		fn from(value: &ServingLocation) -> Self {
			value.clone()
		}
	}

	/// ServingNetworkFunctionId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "servingNetworkFunctionInformation"
	///  ],
	///  "properties": {
	///    "aMFId": {
	///      "$ref": "#/components/schemas/AmfId"
	///    },
	///    "servingNetworkFunctionInformation": {
	///      "$ref": "#/components/schemas/NFIdentification"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ServingNetworkFunctionId {
		#[serde(rename = "aMFId", default, skip_serializing_if = "Option::is_none")]
		pub a_mf_id: Option<AmfId>,
		#[serde(rename = "servingNetworkFunctionInformation")]
		pub serving_network_function_information: NfIdentification,
	}

	impl From<&ServingNetworkFunctionId> for ServingNetworkFunctionId {
		fn from(value: &ServingNetworkFunctionId) -> Self {
			value.clone()
		}
	}

	/// ServingNodeAddress
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	///  "properties": {
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
	#[serde(untagged)]
	pub enum ServingNodeAddress {
		#[default]
		Variant0 {
			#[serde(rename = "ipv4Addr")]
			ipv4_addr: Ipv4Addr,
		},
		Variant1 {
			#[serde(rename = "ipv6Addr")]
			ipv6_addr: Ipv6Addr,
		},
	}

	impl From<&ServingNodeAddress> for ServingNodeAddress {
		fn from(value: &ServingNodeAddress) -> Self {
			value.clone()
		}
	}

	/// SessionFailover
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "FAILOVER_NOT_SUPPORTED",
	///    "FAILOVER_SUPPORTED"
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
	pub enum SessionFailover {
		#[default]
		#[serde(rename = "FAILOVER_NOT_SUPPORTED")]
		FailoverNotSupported,
		#[serde(rename = "FAILOVER_SUPPORTED")]
		FailoverSupported,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SessionFailover> for SessionFailover {
		fn from(value: &SessionFailover) -> Self {
			value.clone()
		}
	}

	impl ToString for SessionFailover {
		fn to_string(&self) -> String {
			match *self {
				Self::FailoverNotSupported => "FAILOVER_NOT_SUPPORTED".to_string(),
				Self::FailoverSupported => "FAILOVER_SUPPORTED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SessionFailover {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"FAILOVER_NOT_SUPPORTED" => Ok(Self::FailoverNotSupported),
				"FAILOVER_SUPPORTED" => Ok(Self::FailoverSupported),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SessionFailover {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SessionFailover {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SessionFailover {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SharingLevel
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "SHARED",
	///    "NON_SHARED"
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
	pub enum SharingLevel {
		#[default]
		#[serde(rename = "SHARED")]
		Shared,
		#[serde(rename = "NON_SHARED")]
		NonShared,
	}

	impl From<&SharingLevel> for SharingLevel {
		fn from(value: &SharingLevel) -> Self {
			value.clone()
		}
	}

	impl ToString for SharingLevel {
		fn to_string(&self) -> String {
			match *self {
				Self::Shared => "SHARED".to_string(),
				Self::NonShared => "NON_SHARED".to_string(),
			}
		}
	}

	impl std::str::FromStr for SharingLevel {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SHARED" => Ok(Self::Shared),
				"NON_SHARED" => Ok(Self::NonShared),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SharingLevel {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SharingLevel {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SharingLevel {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SipEventType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "eventHeader": {
	///      "type": "string"
	///    },
	///    "expiresHeader": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "sIPMethod": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SipEventType {
		#[serde(
			rename = "eventHeader",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_header: Option<String>,
		#[serde(
			rename = "expiresHeader",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub expires_header: Option<Uint32>,
		#[serde(rename = "sIPMethod", default, skip_serializing_if = "Option::is_none")]
		pub s_ip_method: Option<String>,
	}

	impl From<&SipEventType> for SipEventType {
		fn from(value: &SipEventType) -> Self {
			value.clone()
		}
	}

	/// SmAddressDomain
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "3GPPIMSIMCCMNC": {
	///      "type": "string"
	///    },
	///    "domainName": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmAddressDomain {
		#[serde(
			rename = "domainName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub domain_name: Option<String>,
		#[serde(
			rename = "3GPPIMSIMCCMNC",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub three_gppimsimccmnc: Option<String>,
	}

	impl From<&SmAddressDomain> for SmAddressDomain {
		fn from(value: &SmAddressDomain) -> Self {
			value.clone()
		}
	}

	/// SmAddressInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "sMaddressData": {
	///      "type": "string"
	///    },
	///    "sMaddressDomain": {
	///      "$ref": "#/components/schemas/SMAddressDomain"
	///    },
	///    "sMaddressType": {
	///      "$ref": "#/components/schemas/SMAddressType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmAddressInfo {
		#[serde(
			rename = "sMaddressData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_maddress_data: Option<String>,
		#[serde(
			rename = "sMaddressDomain",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_maddress_domain: Option<SmAddressDomain>,
		#[serde(
			rename = "sMaddressType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_maddress_type: Option<SmAddressType>,
	}

	impl From<&SmAddressInfo> for SmAddressInfo {
		fn from(value: &SmAddressInfo) -> Self {
			value.clone()
		}
	}

	/// SmAddressType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "EMAIL_ADDRESS",
	///    "MSISDN",
	///    "IPV4_ADDRESS",
	///    "IPV6_ADDRESS",
	///    "NUMERIC_SHORTCODE",
	///    "ALPHANUMERIC_SHORTCODE",
	///    "OTHER",
	///    "IMSI"
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
	pub enum SmAddressType {
		#[default]
		#[serde(rename = "EMAIL_ADDRESS")]
		EmailAddress,
		#[serde(rename = "MSISDN")]
		Msisdn,
		#[serde(rename = "IPV4_ADDRESS")]
		Ipv4Address,
		#[serde(rename = "IPV6_ADDRESS")]
		Ipv6Address,
		#[serde(rename = "NUMERIC_SHORTCODE")]
		NumericShortcode,
		#[serde(rename = "ALPHANUMERIC_SHORTCODE")]
		AlphanumericShortcode,
		#[serde(rename = "OTHER")]
		Other,
		#[serde(rename = "IMSI")]
		Imsi,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmAddressType> for SmAddressType {
		fn from(value: &SmAddressType) -> Self {
			value.clone()
		}
	}

	impl ToString for SmAddressType {
		fn to_string(&self) -> String {
			match *self {
				Self::EmailAddress => "EMAIL_ADDRESS".to_string(),
				Self::Msisdn => "MSISDN".to_string(),
				Self::Ipv4Address => "IPV4_ADDRESS".to_string(),
				Self::Ipv6Address => "IPV6_ADDRESS".to_string(),
				Self::NumericShortcode => "NUMERIC_SHORTCODE".to_string(),
				Self::AlphanumericShortcode => "ALPHANUMERIC_SHORTCODE".to_string(),
				Self::Other => "OTHER".to_string(),
				Self::Imsi => "IMSI".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmAddressType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EMAIL_ADDRESS" => Ok(Self::EmailAddress),
				"MSISDN" => Ok(Self::Msisdn),
				"IPV4_ADDRESS" => Ok(Self::Ipv4Address),
				"IPV6_ADDRESS" => Ok(Self::Ipv6Address),
				"NUMERIC_SHORTCODE" => Ok(Self::NumericShortcode),
				"ALPHANUMERIC_SHORTCODE" => Ok(Self::AlphanumericShortcode),
				"OTHER" => Ok(Self::Other),
				"IMSI" => Ok(Self::Imsi),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmAddressType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmAddressType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmAddressType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SmAddresseeType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "TO",
	///    "CC",
	///    "BCC"
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
	pub enum SmAddresseeType {
		#[default]
		#[serde(rename = "TO")]
		To,
		#[serde(rename = "CC")]
		Cc,
		#[serde(rename = "BCC")]
		Bcc,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmAddresseeType> for SmAddresseeType {
		fn from(value: &SmAddresseeType) -> Self {
			value.clone()
		}
	}

	impl ToString for SmAddresseeType {
		fn to_string(&self) -> String {
			match *self {
				Self::To => "TO".to_string(),
				Self::Cc => "CC".to_string(),
				Self::Bcc => "BCC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmAddresseeType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TO" => Ok(Self::To),
				"CC" => Ok(Self::Cc),
				"BCC" => Ok(Self::Bcc),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmAddresseeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmAddresseeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmAddresseeType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SmInterface
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "interfaceId": {
	///      "type": "string"
	///    },
	///    "interfacePort": {
	///      "type": "string"
	///    },
	///    "interfaceText": {
	///      "type": "string"
	///    },
	///    "interfaceType": {
	///      "$ref": "#/components/schemas/InterfaceType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmInterface {
		#[serde(
			rename = "interfaceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub interface_id: Option<String>,
		#[serde(
			rename = "interfacePort",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub interface_port: Option<String>,
		#[serde(
			rename = "interfaceText",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub interface_text: Option<String>,
		#[serde(
			rename = "interfaceType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub interface_type: Option<InterfaceType>,
	}

	impl From<&SmInterface> for SmInterface {
		fn from(value: &SmInterface) -> Self {
			value.clone()
		}
	}

	/// SmMessageType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "SUBMISSION",
	///    "DELIVERY_REPORT",
	///    "SM_SERVICE_REQUEST",
	///    "DELIVERY"
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
	pub enum SmMessageType {
		#[default]
		#[serde(rename = "SUBMISSION")]
		Submission,
		#[serde(rename = "DELIVERY_REPORT")]
		DeliveryReport,
		#[serde(rename = "SM_SERVICE_REQUEST")]
		SmServiceRequest,
		#[serde(rename = "DELIVERY")]
		Delivery,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmMessageType> for SmMessageType {
		fn from(value: &SmMessageType) -> Self {
			value.clone()
		}
	}

	impl ToString for SmMessageType {
		fn to_string(&self) -> String {
			match *self {
				Self::Submission => "SUBMISSION".to_string(),
				Self::DeliveryReport => "DELIVERY_REPORT".to_string(),
				Self::SmServiceRequest => "SM_SERVICE_REQUEST".to_string(),
				Self::Delivery => "DELIVERY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmMessageType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SUBMISSION" => Ok(Self::Submission),
				"DELIVERY_REPORT" => Ok(Self::DeliveryReport),
				"SM_SERVICE_REQUEST" => Ok(Self::SmServiceRequest),
				"DELIVERY" => Ok(Self::Delivery),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmMessageType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmMessageType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmMessageType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SmPriority
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "LOW",
	///    "NORMAL",
	///    "HIGH"
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
	pub enum SmPriority {
		#[default]
		#[serde(rename = "LOW")]
		Low,
		#[serde(rename = "NORMAL")]
		Normal,
		#[serde(rename = "HIGH")]
		High,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmPriority> for SmPriority {
		fn from(value: &SmPriority) -> Self {
			value.clone()
		}
	}

	impl ToString for SmPriority {
		fn to_string(&self) -> String {
			match *self {
				Self::Low => "LOW".to_string(),
				Self::Normal => "NORMAL".to_string(),
				Self::High => "HIGH".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmPriority {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOW" => Ok(Self::Low),
				"NORMAL" => Ok(Self::Normal),
				"HIGH" => Ok(Self::High),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SmServiceType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "VAS4SMS_SHORT_MESSAGE_CONTENT_PROCESSING",
	///    "VAS4SMS_SHORT_MESSAGE_FORWARDING",
	///    "VAS4SMS_SHORT_MESSAGE_FORWARDING_MULTIPLE_SUBSCRIPTIONS",
	///    "VAS4SMS_SHORT_MESSAGE_FILTERING",
	///    "VAS4SMS_SHORT_MESSAGE_RECEIPT",
	///    "VAS4SMS_SHORT_MESSAGE_NETWORK_STORAGE",
	///    "VAS4SMS_SHORT_MESSAGE_TO_MULTIPLE_DESTINATIONS",
	///    "VAS4SMS_SHORT_MESSAGE_VIRTUAL_PRIVATE_NETWORK(VPN)",
	///    "VAS4SMS_SHORT_MESSAGE_AUTO_REPLY",
	///    "VAS4SMS_SHORT_MESSAGE_PERSONAL_SIGNATURE",
	///    "VAS4SMS_SHORT_MESSAGE_DEFERRED_DELIVERY"
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
	pub enum SmServiceType {
		#[default]
		#[serde(rename = "VAS4SMS_SHORT_MESSAGE_CONTENT_PROCESSING")]
		Vas4smsShortMessageContentProcessing,
		#[serde(rename = "VAS4SMS_SHORT_MESSAGE_FORWARDING")]
		Vas4smsShortMessageForwarding,
		#[serde(rename = "VAS4SMS_SHORT_MESSAGE_FORWARDING_MULTIPLE_SUBSCRIPTIONS")]
		Vas4smsShortMessageForwardingMultipleSubscriptions,
		#[serde(rename = "VAS4SMS_SHORT_MESSAGE_FILTERING")]
		Vas4smsShortMessageFiltering,
		#[serde(rename = "VAS4SMS_SHORT_MESSAGE_RECEIPT")]
		Vas4smsShortMessageReceipt,
		#[serde(rename = "VAS4SMS_SHORT_MESSAGE_NETWORK_STORAGE")]
		Vas4smsShortMessageNetworkStorage,
		#[serde(rename = "VAS4SMS_SHORT_MESSAGE_TO_MULTIPLE_DESTINATIONS")]
		Vas4smsShortMessageToMultipleDestinations,
		#[serde(rename = "VAS4SMS_SHORT_MESSAGE_VIRTUAL_PRIVATE_NETWORK(VPN)")]
		Vas4smsShortMessageVirtualPrivateNetworkVpn,
		#[serde(rename = "VAS4SMS_SHORT_MESSAGE_AUTO_REPLY")]
		Vas4smsShortMessageAutoReply,
		#[serde(rename = "VAS4SMS_SHORT_MESSAGE_PERSONAL_SIGNATURE")]
		Vas4smsShortMessagePersonalSignature,
		#[serde(rename = "VAS4SMS_SHORT_MESSAGE_DEFERRED_DELIVERY")]
		Vas4smsShortMessageDeferredDelivery,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmServiceType> for SmServiceType {
		fn from(value: &SmServiceType) -> Self {
			value.clone()
		}
	}

	impl ToString for SmServiceType {
		fn to_string(&self) -> String {
			match *self {
				Self::Vas4smsShortMessageContentProcessing => {
					"VAS4SMS_SHORT_MESSAGE_CONTENT_PROCESSING".to_string()
				}
				Self::Vas4smsShortMessageForwarding => {
					"VAS4SMS_SHORT_MESSAGE_FORWARDING".to_string()
				}
				Self::Vas4smsShortMessageForwardingMultipleSubscriptions => {
					"VAS4SMS_SHORT_MESSAGE_FORWARDING_MULTIPLE_SUBSCRIPTIONS".to_string()
				}
				Self::Vas4smsShortMessageFiltering => "VAS4SMS_SHORT_MESSAGE_FILTERING".to_string(),
				Self::Vas4smsShortMessageReceipt => "VAS4SMS_SHORT_MESSAGE_RECEIPT".to_string(),
				Self::Vas4smsShortMessageNetworkStorage => {
					"VAS4SMS_SHORT_MESSAGE_NETWORK_STORAGE".to_string()
				}
				Self::Vas4smsShortMessageToMultipleDestinations => {
					"VAS4SMS_SHORT_MESSAGE_TO_MULTIPLE_DESTINATIONS".to_string()
				}
				Self::Vas4smsShortMessageVirtualPrivateNetworkVpn => {
					"VAS4SMS_SHORT_MESSAGE_VIRTUAL_PRIVATE_NETWORK(VPN)".to_string()
				}
				Self::Vas4smsShortMessageAutoReply => {
					"VAS4SMS_SHORT_MESSAGE_AUTO_REPLY".to_string()
				}
				Self::Vas4smsShortMessagePersonalSignature => {
					"VAS4SMS_SHORT_MESSAGE_PERSONAL_SIGNATURE".to_string()
				}
				Self::Vas4smsShortMessageDeferredDelivery => {
					"VAS4SMS_SHORT_MESSAGE_DEFERRED_DELIVERY".to_string()
				}
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmServiceType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"VAS4SMS_SHORT_MESSAGE_CONTENT_PROCESSING" => {
					Ok(Self::Vas4smsShortMessageContentProcessing)
				}
				"VAS4SMS_SHORT_MESSAGE_FORWARDING" => Ok(Self::Vas4smsShortMessageForwarding),
				"VAS4SMS_SHORT_MESSAGE_FORWARDING_MULTIPLE_SUBSCRIPTIONS" => {
					Ok(Self::Vas4smsShortMessageForwardingMultipleSubscriptions)
				}
				"VAS4SMS_SHORT_MESSAGE_FILTERING" => Ok(Self::Vas4smsShortMessageFiltering),
				"VAS4SMS_SHORT_MESSAGE_RECEIPT" => Ok(Self::Vas4smsShortMessageReceipt),
				"VAS4SMS_SHORT_MESSAGE_NETWORK_STORAGE" => {
					Ok(Self::Vas4smsShortMessageNetworkStorage)
				}
				"VAS4SMS_SHORT_MESSAGE_TO_MULTIPLE_DESTINATIONS" => {
					Ok(Self::Vas4smsShortMessageToMultipleDestinations)
				}
				"VAS4SMS_SHORT_MESSAGE_VIRTUAL_PRIVATE_NETWORK(VPN)" => {
					Ok(Self::Vas4smsShortMessageVirtualPrivateNetworkVpn)
				}
				"VAS4SMS_SHORT_MESSAGE_AUTO_REPLY" => Ok(Self::Vas4smsShortMessageAutoReply),
				"VAS4SMS_SHORT_MESSAGE_PERSONAL_SIGNATURE" => {
					Ok(Self::Vas4smsShortMessagePersonalSignature)
				}
				"VAS4SMS_SHORT_MESSAGE_DEFERRED_DELIVERY" => {
					Ok(Self::Vas4smsShortMessageDeferredDelivery)
				}
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SmsChargingInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "deliveryReportRequested": {
	///      "$ref": "#/components/schemas/DeliveryReportRequested"
	///    },
	///    "messageClass": {
	///      "$ref": "#/components/schemas/MessageClass"
	///    },
	///    "messageReference": {
	///      "type": "string"
	///    },
	///    "messageSize": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "numberofMessagesSent": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "originatorInfo": {
	///      "$ref": "#/components/schemas/OriginatorInfo"
	///    },
	///    "rATType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "recipientInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RecipientInfo"
	///      },
	///      "minItems": 0
	///    },
	///    "roamerInOut": {
	///      "$ref": "#/components/schemas/RoamerInOut"
	///    },
	///    "sMDataCodingScheme": {
	///      "type": "integer"
	///    },
	///    "sMDischargeTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "sMMessageType": {
	///      "$ref": "#/components/schemas/SMMessageType"
	///    },
	///    "sMPriority": {
	///      "$ref": "#/components/schemas/SMPriority"
	///    },
	///    "sMReplyPathRequested": {
	///      "$ref": "#/components/schemas/ReplyPathRequested"
	///    },
	///    "sMSCAddress": {
	///      "type": "string"
	///    },
	///    "sMSequenceNumber": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "sMServiceType": {
	///      "$ref": "#/components/schemas/SMServiceType"
	///    },
	///    "sMSresult": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "sMStatus": {
	///      "type": "string",
	///      "pattern": "^[0-7]?[0-9a-fA-F]$"
	///    },
	///    "sMUserDataHeader": {
	///      "type": "string"
	///    },
	///    "submissionTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "uetimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "userEquipmentInfo": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "userLocationinfo": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmsChargingInformation {
		#[serde(
			rename = "deliveryReportRequested",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub delivery_report_requested: Option<DeliveryReportRequested>,
		#[serde(
			rename = "messageClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub message_class: Option<MessageClass>,
		#[serde(
			rename = "messageReference",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub message_reference: Option<String>,
		#[serde(
			rename = "messageSize",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub message_size: Option<Uint32>,
		#[serde(
			rename = "numberofMessagesSent",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub numberof_messages_sent: Option<Uint32>,
		#[serde(
			rename = "originatorInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub originator_info: Option<OriginatorInfo>,
		#[serde(rename = "rATType", default, skip_serializing_if = "Option::is_none")]
		pub r_at_type: Option<RatType>,
		#[serde(
			rename = "recipientInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub recipient_info: Vec<RecipientInfo>,
		#[serde(
			rename = "roamerInOut",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub roamer_in_out: Option<RoamerInOut>,
		#[serde(
			rename = "sMDataCodingScheme",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_data_coding_scheme: Option<i64>,
		#[serde(
			rename = "sMDischargeTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_discharge_time: Option<DateTime>,
		#[serde(
			rename = "sMMessageType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_message_type: Option<SmMessageType>,
		#[serde(
			rename = "sMPriority",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_priority: Option<SmPriority>,
		#[serde(
			rename = "sMReplyPathRequested",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_reply_path_requested: Option<ReplyPathRequested>,
		#[serde(
			rename = "sMSequenceNumber",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_sequence_number: Option<Uint32>,
		#[serde(
			rename = "sMServiceType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_service_type: Option<SmServiceType>,
		#[serde(rename = "sMSresult", default, skip_serializing_if = "Option::is_none")]
		pub s_m_sresult: Option<Uint32>,
		#[serde(rename = "sMStatus", default, skip_serializing_if = "Option::is_none")]
		pub s_m_status: Option<SmsChargingInformationSMStatus>,
		#[serde(
			rename = "sMUserDataHeader",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_m_user_data_header: Option<String>,
		#[serde(
			rename = "sMSCAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s_msc_address: Option<String>,
		#[serde(
			rename = "submissionTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub submission_time: Option<DateTime>,
		#[serde(
			rename = "uetimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uetime_zone: Option<TimeZone>,
		#[serde(
			rename = "userEquipmentInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_equipment_info: Option<Pei>,
		#[serde(
			rename = "userLocationinfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_locationinfo: Option<UserLocation>,
	}

	impl From<&SmsChargingInformation> for SmsChargingInformation {
		fn from(value: &SmsChargingInformation) -> Self {
			value.clone()
		}
	}

	/// SmsChargingInformationSMStatus
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-7]?[0-9a-fA-F]$"
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
	pub struct SmsChargingInformationSMStatus(String);

	impl ::std::ops::Deref for SmsChargingInformationSMStatus {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SmsChargingInformationSMStatus> for String {
		fn from(value: SmsChargingInformationSMStatus) -> Self {
			value.0
		}
	}

	impl From<&SmsChargingInformationSMStatus> for SmsChargingInformationSMStatus {
		fn from(value: &SmsChargingInformationSMStatus) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SmsChargingInformationSMStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-7]?[0-9a-fA-F]$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-7]?[0-9a-fA-F]$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SmsChargingInformationSMStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SmsChargingInformationSMStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SmsChargingInformationSMStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SmsChargingInformationSMStatus {
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

	/// SmsIndication
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "SMS_SUPPORTED",
	///    "SMS_NOT_SUPPORTED"
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
	pub enum SmsIndication {
		#[default]
		#[serde(rename = "SMS_SUPPORTED")]
		SmsSupported,
		#[serde(rename = "SMS_NOT_SUPPORTED")]
		SmsNotSupported,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmsIndication> for SmsIndication {
		fn from(value: &SmsIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for SmsIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::SmsSupported => "SMS_SUPPORTED".to_string(),
				Self::SmsNotSupported => "SMS_NOT_SUPPORTED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmsIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SMS_SUPPORTED" => Ok(Self::SmsSupported),
				"SMS_NOT_SUPPORTED" => Ok(Self::SmsNotSupported),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmsIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmsIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmsIndication {
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

	/// SoftwareImageInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "discFormat": {
	///      "type": "string"
	///    },
	///    "minimumDisk": {
	///      "type": "integer"
	///    },
	///    "minimumRAM": {
	///      "type": "integer"
	///    },
	///    "operatingSystem": {
	///      "type": "string"
	///    },
	///    "swImageRef": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SoftwareImageInfo {
		#[serde(
			rename = "discFormat",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub disc_format: Option<String>,
		#[serde(
			rename = "minimumDisk",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub minimum_disk: Option<i64>,
		#[serde(
			rename = "minimumRAM",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub minimum_ram: Option<i64>,
		#[serde(
			rename = "operatingSystem",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub operating_system: Option<String>,
		#[serde(
			rename = "swImageRef",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sw_image_ref: Option<String>,
	}

	impl From<&SoftwareImageInfo> for SoftwareImageInfo {
		fn from(value: &SoftwareImageInfo) -> Self {
			value.clone()
		}
	}

	/// Represents the subscription data structure required for an individual
	/// CHF spending limit subscription.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the subscription data structure required for
	/// an individual CHF spending limit subscription.\n",
	///  "type": "object",
	///  "properties": {
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "notifId": {
	///      "type": "string"
	///    },
	///    "notifUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "policyCounterIds": {
	///      "description": "This is a list of policy counter identifier(s),
	/// which identifies policy counters maintained per subscriber within the
	/// CHF.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PolicyCounterId"
	///      },
	///      "minItems": 1
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
	pub struct SpendingLimitContext {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "notifId", default, skip_serializing_if = "Option::is_none")]
		pub notif_id: Option<String>,
		#[serde(rename = "notifUri", default, skip_serializing_if = "Option::is_none")]
		pub notif_uri: Option<Uri>,
		/// This is a list of policy counter identifier(s), which identifies
		/// policy counters maintained per subscriber within the CHF.
		#[serde(
			rename = "policyCounterIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub policy_counter_ids: Vec<PolicyCounterId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&SpendingLimitContext> for SpendingLimitContext {
		fn from(value: &SpendingLimitContext) -> Self {
			value.clone()
		}
	}

	/// Represents the data structure presenting the statuses of policy
	/// counters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the data structure presenting the statuses
	/// of policy counters.\n",
	///  "type": "object",
	///  "properties": {
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "notifId": {
	///      "type": "string"
	///    },
	///    "statusInfos": {
	///      "description": "Status of the requested policy counters. The key of
	/// the map is the attribute \"policyCounterId\".\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PolicyCounterInfo"
	///      }
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
	pub struct SpendingLimitStatus {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DateTime>,
		#[serde(rename = "notifId", default, skip_serializing_if = "Option::is_none")]
		pub notif_id: Option<String>,
		/// Status of the requested policy counters. The key of the map is the
		/// attribute "policyCounterId".
		#[serde(
			rename = "statusInfos",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub status_infos: ::std::collections::HashMap<String, PolicyCounterInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&SpendingLimitStatus> for SpendingLimitStatus {
		fn from(value: &SpendingLimitStatus) -> Self {
			value.clone()
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

	/// Sst
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "integer",
	///  "maximum": 255.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Sst(pub u8);

	impl ::std::ops::Deref for Sst {
		type Target = u8;
		fn deref(&self) -> &u8 {
			&self.0
		}
	}

	impl From<Sst> for u8 {
		fn from(value: Sst) -> Self {
			value.0
		}
	}

	impl From<&Sst> for Sst {
		fn from(value: &Sst) -> Self {
			value.clone()
		}
	}

	impl From<u8> for Sst {
		fn from(value: u8) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Sst {
		type Err = <u8 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Sst {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Sst {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Sst {
		type Error = <u8 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Sst {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains Autonomous load-balance indicator or UE-assistance indicator.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains Autonomous load-balance indicator or
	/// UE-assistance indicator.",
	///  "type": "string",
	///  "enum": [
	///    "AUTO_LOAD_BALANCE",
	///    "UE_ASSISTANCE"
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
	pub enum SteerModeIndicator {
		#[default]
		#[serde(rename = "AUTO_LOAD_BALANCE")]
		AutoLoadBalance,
		#[serde(rename = "UE_ASSISTANCE")]
		UeAssistance,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SteerModeIndicator> for SteerModeIndicator {
		fn from(value: &SteerModeIndicator) -> Self {
			value.clone()
		}
	}

	impl ToString for SteerModeIndicator {
		fn to_string(&self) -> String {
			match *self {
				Self::AutoLoadBalance => "AUTO_LOAD_BALANCE".to_string(),
				Self::UeAssistance => "UE_ASSISTANCE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SteerModeIndicator {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AUTO_LOAD_BALANCE" => Ok(Self::AutoLoadBalance),
				"UE_ASSISTANCE" => Ok(Self::UeAssistance),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SteerModeIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SteerModeIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SteerModeIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the steering mode value determined by the PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the steering mode value determined by the
	/// PCF.",
	///  "type": "string",
	///  "enum": [
	///    "ACTIVE_STANDBY",
	///    "LOAD_BALANCING",
	///    "SMALLEST_DELAY",
	///    "PRIORITY_BASED"
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
	pub enum SteerModeValue {
		#[default]
		#[serde(rename = "ACTIVE_STANDBY")]
		ActiveStandby,
		#[serde(rename = "LOAD_BALANCING")]
		LoadBalancing,
		#[serde(rename = "SMALLEST_DELAY")]
		SmallestDelay,
		#[serde(rename = "PRIORITY_BASED")]
		PriorityBased,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SteerModeValue> for SteerModeValue {
		fn from(value: &SteerModeValue) -> Self {
			value.clone()
		}
	}

	impl ToString for SteerModeValue {
		fn to_string(&self) -> String {
			match *self {
				Self::ActiveStandby => "ACTIVE_STANDBY".to_string(),
				Self::LoadBalancing => "LOAD_BALANCING".to_string(),
				Self::SmallestDelay => "SMALLEST_DELAY".to_string(),
				Self::PriorityBased => "PRIORITY_BASED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SteerModeValue {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ACTIVE_STANDBY" => Ok(Self::ActiveStandby),
				"LOAD_BALANCING" => Ok(Self::LoadBalancing),
				"SMALLEST_DELAY" => Ok(Self::SmallestDelay),
				"PRIORITY_BASED" => Ok(Self::PriorityBased),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SteerModeValue {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SteerModeValue {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SteerModeValue {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are
	///  - MPTCP: Indicates that PCF authorizes the MPTCP functionality to
	///    support traffic steering, switching and splitting.
	///  - ATSSS_LL: Indicates that PCF authorizes the ATSSS-LL functionality to
	///    support traffic steering, switching and splitting.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are\n  - MPTCP: Indicates that PCF
	/// authorizes the MPTCP functionality to support traffic steering,
	/// switching and splitting.\n  - ATSSS_LL: Indicates that PCF authorizes
	/// the ATSSS-LL functionality to support traffic steering, switching and
	/// splitting.\n",
	///  "type": "string",
	///  "enum": [
	///    "MPTCP",
	///    "ATSSS_LL"
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
	pub enum SteeringFunctionality {
		#[default]
		#[serde(rename = "MPTCP")]
		Mptcp,
		#[serde(rename = "ATSSS_LL")]
		AtsssLl,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SteeringFunctionality> for SteeringFunctionality {
		fn from(value: &SteeringFunctionality) -> Self {
			value.clone()
		}
	}

	impl ToString for SteeringFunctionality {
		fn to_string(&self) -> String {
			match *self {
				Self::Mptcp => "MPTCP".to_string(),
				Self::AtsssLl => "ATSSS_LL".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SteeringFunctionality {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MPTCP" => Ok(Self::Mptcp),
				"ATSSS_LL" => Ok(Self::AtsssLl),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SteeringFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SteeringFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SteeringFunctionality {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the steering mode value and parameters determined by the PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the steering mode value and parameters
	/// determined by the PCF.",
	///  "type": "object",
	///  "required": [
	///    "steerModeValue"
	///  ],
	///  "properties": {
	///    "3gLoad": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "active": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "prioAcc": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "standby": {
	///      "$ref": "#/components/schemas/AccessTypeRm"
	///    },
	///    "steerModeInd": {
	///      "$ref": "#/components/schemas/SteerModeIndicator"
	///    },
	///    "steerModeValue": {
	///      "$ref": "#/components/schemas/SteerModeValue"
	///    },
	///    "thresValue": {
	///      "$ref": "#/components/schemas/ThresholdValue"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SteeringMode {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub active: Option<AccessType>,
		#[serde(rename = "prioAcc", default, skip_serializing_if = "Option::is_none")]
		pub prio_acc: Option<AccessType>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub standby: Option<AccessTypeRm>,
		#[serde(
			rename = "steerModeInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub steer_mode_ind: Option<SteerModeIndicator>,
		#[serde(rename = "steerModeValue")]
		pub steer_mode_value: SteerModeValue,
		#[serde(rename = "3gLoad", default, skip_serializing_if = "Option::is_none")]
		pub three_g_load: Option<Uinteger>,
		#[serde(
			rename = "thresValue",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub thres_value: Option<ThresholdValue>,
	}

	impl From<&SteeringMode> for SteeringMode {
		fn from(value: &SteeringMode) -> Self {
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

	/// Represents the data structure presenting the indication of the
	/// termination of the  subscription.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the data structure presenting the indication
	/// of the termination of the  subscription.\n",
	///  "type": "object",
	///  "required": [
	///    "supi"
	///  ],
	///  "properties": {
	///    "notifId": {
	///      "type": "string"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "termCause": {
	///      "$ref": "#/components/schemas/TerminationCause"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SubscriptionTerminationInfo {
		#[serde(rename = "notifId", default, skip_serializing_if = "Option::is_none")]
		pub notif_id: Option<String>,
		pub supi: Supi,
		#[serde(rename = "termCause", default, skip_serializing_if = "Option::is_none")]
		pub term_cause: Option<TerminationCause>,
	}

	impl From<&SubscriptionTerminationInfo> for SubscriptionTerminationInfo {
		fn from(value: &SubscriptionTerminationInfo) -> Self {
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

	/// SupplementaryService
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "associatedPartyAddress": {
	///      "type": "string"
	///    },
	///    "cUGInformation": {
	///      "$ref": "#/components/schemas/OctetString"
	///    },
	///    "changeTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "conferenceId": {
	///      "type": "string"
	///    },
	///    "numberOfDiversions": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "numberOfParticipants": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "participantActionType": {
	///      "$ref": "#/components/schemas/ParticipantActionType"
	///    },
	///    "supplementaryServiceMode": {
	///      "$ref": "#/components/schemas/SupplementaryServiceMode"
	///    },
	///    "supplementaryServiceType": {
	///      "$ref": "#/components/schemas/SupplementaryServiceType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SupplementaryService {
		#[serde(
			rename = "associatedPartyAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub associated_party_address: Option<String>,
		#[serde(
			rename = "cUGInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub c_ug_information: Option<OctetString>,
		#[serde(
			rename = "changeTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub change_time: Option<DateTime>,
		#[serde(
			rename = "conferenceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub conference_id: Option<String>,
		#[serde(
			rename = "numberOfDiversions",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub number_of_diversions: Option<Uint32>,
		#[serde(
			rename = "numberOfParticipants",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub number_of_participants: Option<Uint32>,
		#[serde(
			rename = "participantActionType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub participant_action_type: Option<ParticipantActionType>,
		#[serde(
			rename = "supplementaryServiceMode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supplementary_service_mode: Option<SupplementaryServiceMode>,
		#[serde(
			rename = "supplementaryServiceType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supplementary_service_type: Option<SupplementaryServiceType>,
	}

	impl From<&SupplementaryService> for SupplementaryService {
		fn from(value: &SupplementaryService) -> Self {
			value.clone()
		}
	}

	/// SupplementaryServiceMode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "CFU",
	///    "CFB",
	///    "CFNR",
	///    "CFNL",
	///    "CD",
	///    "CFNRC",
	///    "ICB",
	///    "OCB",
	///    "ACR",
	///    "BLIND_TRANFER",
	///    "CONSULTATIVE_TRANFER"
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
	pub enum SupplementaryServiceMode {
		#[default]
		#[serde(rename = "CFU")]
		Cfu,
		#[serde(rename = "CFB")]
		Cfb,
		#[serde(rename = "CFNR")]
		Cfnr,
		#[serde(rename = "CFNL")]
		Cfnl,
		#[serde(rename = "CD")]
		Cd,
		#[serde(rename = "CFNRC")]
		Cfnrc,
		#[serde(rename = "ICB")]
		Icb,
		#[serde(rename = "OCB")]
		Ocb,
		#[serde(rename = "ACR")]
		Acr,
		#[serde(rename = "BLIND_TRANFER")]
		BlindTranfer,
		#[serde(rename = "CONSULTATIVE_TRANFER")]
		ConsultativeTranfer,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SupplementaryServiceMode> for SupplementaryServiceMode {
		fn from(value: &SupplementaryServiceMode) -> Self {
			value.clone()
		}
	}

	impl ToString for SupplementaryServiceMode {
		fn to_string(&self) -> String {
			match *self {
				Self::Cfu => "CFU".to_string(),
				Self::Cfb => "CFB".to_string(),
				Self::Cfnr => "CFNR".to_string(),
				Self::Cfnl => "CFNL".to_string(),
				Self::Cd => "CD".to_string(),
				Self::Cfnrc => "CFNRC".to_string(),
				Self::Icb => "ICB".to_string(),
				Self::Ocb => "OCB".to_string(),
				Self::Acr => "ACR".to_string(),
				Self::BlindTranfer => "BLIND_TRANFER".to_string(),
				Self::ConsultativeTranfer => "CONSULTATIVE_TRANFER".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SupplementaryServiceMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CFU" => Ok(Self::Cfu),
				"CFB" => Ok(Self::Cfb),
				"CFNR" => Ok(Self::Cfnr),
				"CFNL" => Ok(Self::Cfnl),
				"CD" => Ok(Self::Cd),
				"CFNRC" => Ok(Self::Cfnrc),
				"ICB" => Ok(Self::Icb),
				"OCB" => Ok(Self::Ocb),
				"ACR" => Ok(Self::Acr),
				"BLIND_TRANFER" => Ok(Self::BlindTranfer),
				"CONSULTATIVE_TRANFER" => Ok(Self::ConsultativeTranfer),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SupplementaryServiceMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SupplementaryServiceMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SupplementaryServiceMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SupplementaryServiceType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "OIP",
	///    "OIR",
	///    "TIP",
	///    "TIR",
	///    "HOLD",
	///    "CB",
	///    "CDIV",
	///    "CW",
	///    "MWI",
	///    "CONF",
	///    "FA",
	///    "CCBS",
	///    "CCNR",
	///    "MCID",
	///    "CAT",
	///    "CUG",
	///    "PNM",
	///    "CRS",
	///    "ECT"
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
	pub enum SupplementaryServiceType {
		#[default]
		#[serde(rename = "OIP")]
		Oip,
		#[serde(rename = "OIR")]
		Oir,
		#[serde(rename = "TIP")]
		Tip,
		#[serde(rename = "TIR")]
		Tir,
		#[serde(rename = "HOLD")]
		Hold,
		#[serde(rename = "CB")]
		Cb,
		#[serde(rename = "CDIV")]
		Cdiv,
		#[serde(rename = "CW")]
		Cw,
		#[serde(rename = "MWI")]
		Mwi,
		#[serde(rename = "CONF")]
		Conf,
		#[serde(rename = "FA")]
		Fa,
		#[serde(rename = "CCBS")]
		Ccbs,
		#[serde(rename = "CCNR")]
		Ccnr,
		#[serde(rename = "MCID")]
		Mcid,
		#[serde(rename = "CAT")]
		Cat,
		#[serde(rename = "CUG")]
		Cug,
		#[serde(rename = "PNM")]
		Pnm,
		#[serde(rename = "CRS")]
		Crs,
		#[serde(rename = "ECT")]
		Ect,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SupplementaryServiceType> for SupplementaryServiceType {
		fn from(value: &SupplementaryServiceType) -> Self {
			value.clone()
		}
	}

	impl ToString for SupplementaryServiceType {
		fn to_string(&self) -> String {
			match *self {
				Self::Oip => "OIP".to_string(),
				Self::Oir => "OIR".to_string(),
				Self::Tip => "TIP".to_string(),
				Self::Tir => "TIR".to_string(),
				Self::Hold => "HOLD".to_string(),
				Self::Cb => "CB".to_string(),
				Self::Cdiv => "CDIV".to_string(),
				Self::Cw => "CW".to_string(),
				Self::Mwi => "MWI".to_string(),
				Self::Conf => "CONF".to_string(),
				Self::Fa => "FA".to_string(),
				Self::Ccbs => "CCBS".to_string(),
				Self::Ccnr => "CCNR".to_string(),
				Self::Mcid => "MCID".to_string(),
				Self::Cat => "CAT".to_string(),
				Self::Cug => "CUG".to_string(),
				Self::Pnm => "PNM".to_string(),
				Self::Crs => "CRS".to_string(),
				Self::Ect => "ECT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SupplementaryServiceType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"OIP" => Ok(Self::Oip),
				"OIR" => Ok(Self::Oir),
				"TIP" => Ok(Self::Tip),
				"TIR" => Ok(Self::Tir),
				"HOLD" => Ok(Self::Hold),
				"CB" => Ok(Self::Cb),
				"CDIV" => Ok(Self::Cdiv),
				"CW" => Ok(Self::Cw),
				"MWI" => Ok(Self::Mwi),
				"CONF" => Ok(Self::Conf),
				"FA" => Ok(Self::Fa),
				"CCBS" => Ok(Self::Ccbs),
				"CCNR" => Ok(Self::Ccnr),
				"MCID" => Ok(Self::Mcid),
				"CAT" => Ok(Self::Cat),
				"CUG" => Ok(Self::Cug),
				"PNM" => Ok(Self::Pnm),
				"CRS" => Ok(Self::Crs),
				"ECT" => Ok(Self::Ect),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SupplementaryServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SupplementaryServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SupplementaryServiceType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Contains a mean opinion score with the customized range.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a mean opinion score with the customized
	/// range.",
	///  "type": "object",
	///  "properties": {
	///    "lowerRange": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "mos": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "upperRange": {
	///      "$ref": "#/components/schemas/Float"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SvcExperience {
		#[serde(
			rename = "lowerRange",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lower_range: Option<Float>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub mos: Option<Float>,
		#[serde(
			rename = "upperRange",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub upper_range: Option<Float>,
	}

	impl From<&SvcExperience> for SvcExperience {
		fn from(value: &SvcExperience) -> Self {
			value.clone()
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

	/// TadIdentifier
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "CS",
	///    "PS"
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
	pub enum TadIdentifier {
		#[default]
		#[serde(rename = "CS")]
		Cs,
		#[serde(rename = "PS")]
		Ps,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&TadIdentifier> for TadIdentifier {
		fn from(value: &TadIdentifier) -> Self {
			value.clone()
		}
	}

	impl ToString for TadIdentifier {
		fn to_string(&self) -> String {
			match *self {
				Self::Cs => "CS".to_string(),
				Self::Ps => "PS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TadIdentifier {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CS" => Ok(Self::Cs),
				"PS" => Ok(Self::Ps),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for TadIdentifier {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TadIdentifier {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TadIdentifier {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// TaiList
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "array",
	///  "items": {
	///    "$ref": "#/components/schemas/schemas-Tai"
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TaiList(pub Vec<SchemasTai>);

	impl ::std::ops::Deref for TaiList {
		type Target = Vec<SchemasTai>;
		fn deref(&self) -> &Vec<SchemasTai> {
			&self.0
		}
	}

	impl From<TaiList> for Vec<SchemasTai> {
		fn from(value: TaiList) -> Self {
			value.0
		}
	}

	impl From<&TaiList> for TaiList {
		fn from(value: &TaiList) -> Self {
			value.clone()
		}
	}

	impl From<Vec<SchemasTai>> for TaiList {
		fn from(value: Vec<SchemasTai>) -> Self {
			Self(value)
		}
	}

	/// Represents the cause for requesting the termination of the subscription
	/// to policy counter status changes.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the cause for requesting the termination of
	/// the subscription to policy counter status changes.\n",
	///  "type": "string",
	///  "enum": [
	///    "REMOVED_SUBSCRIBER"
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
	pub enum TerminationCause {
		#[default]
		#[serde(rename = "REMOVED_SUBSCRIBER")]
		RemovedSubscriber,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&TerminationCause> for TerminationCause {
		fn from(value: &TerminationCause) -> Self {
			value.clone()
		}
	}

	impl ToString for TerminationCause {
		fn to_string(&self) -> String {
			match *self {
				Self::RemovedSubscriber => "REMOVED_SUBSCRIBER".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TerminationCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REMOVED_SUBSCRIBER" => Ok(Self::RemovedSubscriber),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for TerminationCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TerminationCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TerminationCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents a threshold level.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a threshold level.",
	///  "type": "object",
	///  "properties": {
	///    "avgPacketDelay": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "avgPacketLossRate": {
	///      "$ref": "#/components/schemas/PacketLossRate"
	///    },
	///    "avgTrafficRate": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "congLevel": {
	///      "type": "integer"
	///    },
	///    "maxPacketDelay": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "maxTrafficRate": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "nfCpuUsage": {
	///      "type": "integer"
	///    },
	///    "nfLoadLevel": {
	///      "type": "integer"
	///    },
	///    "nfMemoryUsage": {
	///      "type": "integer"
	///    },
	///    "nfStorageUsage": {
	///      "type": "integer"
	///    },
	///    "svcExpLevel": {
	///      "$ref": "#/components/schemas/Float"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ThresholdLevel {
		#[serde(
			rename = "avgPacketDelay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub avg_packet_delay: Option<PacketDelBudget>,
		#[serde(
			rename = "avgPacketLossRate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub avg_packet_loss_rate: Option<PacketLossRate>,
		#[serde(
			rename = "avgTrafficRate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub avg_traffic_rate: Option<BitRate>,
		#[serde(rename = "congLevel", default, skip_serializing_if = "Option::is_none")]
		pub cong_level: Option<i64>,
		#[serde(
			rename = "maxPacketDelay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_packet_delay: Option<PacketDelBudget>,
		#[serde(
			rename = "maxTrafficRate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_traffic_rate: Option<BitRate>,
		#[serde(
			rename = "nfCpuUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nf_cpu_usage: Option<i64>,
		#[serde(
			rename = "nfLoadLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nf_load_level: Option<i64>,
		#[serde(
			rename = "nfMemoryUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nf_memory_usage: Option<i64>,
		#[serde(
			rename = "nfStorageUsage",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nf_storage_usage: Option<i64>,
		#[serde(
			rename = "svcExpLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub svc_exp_level: Option<Float>,
	}

	impl From<&ThresholdLevel> for ThresholdLevel {
		fn from(value: &ThresholdLevel) -> Self {
			value.clone()
		}
	}

	/// Indicates the threshold value(s) for RTT and/or Packet Loss Rate.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the threshold value(s) for RTT and/or Packet
	/// Loss Rate.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "plrThres": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "rttThres": {
	///      "$ref": "#/components/schemas/UintegerRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ThresholdValue(pub Option<ThresholdValueInner>);

	impl ::std::ops::Deref for ThresholdValue {
		type Target = Option<ThresholdValueInner>;
		fn deref(&self) -> &Option<ThresholdValueInner> {
			&self.0
		}
	}

	impl From<ThresholdValue> for Option<ThresholdValueInner> {
		fn from(value: ThresholdValue) -> Self {
			value.0
		}
	}

	impl From<&ThresholdValue> for ThresholdValue {
		fn from(value: &ThresholdValue) -> Self {
			value.clone()
		}
	}

	impl From<Option<ThresholdValueInner>> for ThresholdValue {
		fn from(value: Option<ThresholdValueInner>) -> Self {
			Self(value)
		}
	}

	/// Indicates the threshold value(s) for RTT and/or Packet Loss Rate.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the threshold value(s) for RTT and/or Packet
	/// Loss Rate.",
	///  "type": "object",
	///  "properties": {
	///    "plrThres": {
	///      "$ref": "#/components/schemas/PacketLossRateRm"
	///    },
	///    "rttThres": {
	///      "$ref": "#/components/schemas/UintegerRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ThresholdValueInner {
		#[serde(rename = "plrThres", default, skip_serializing_if = "Option::is_none")]
		pub plr_thres: Option<PacketLossRateRm>,
		#[serde(rename = "rttThres", default, skip_serializing_if = "Option::is_none")]
		pub rtt_thres: Option<UintegerRm>,
	}

	impl From<&ThresholdValueInner> for ThresholdValueInner {
		fn from(value: &ThresholdValueInner) -> Self {
			value.clone()
		}
	}

	/// Throughput
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "guaranteedThpt": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "maximumThpt": {
	///      "$ref": "#/components/schemas/Float"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Throughput {
		#[serde(
			rename = "guaranteedThpt",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub guaranteed_thpt: Option<Float>,
		#[serde(
			rename = "maximumThpt",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub maximum_thpt: Option<Float>,
	}

	impl From<&Throughput> for Throughput {
		fn from(value: &Throughput) -> Self {
			value.clone()
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

	/// TopologicalServiceArea
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "cellIdList": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
	///      }
	///    },
	///    "servingPLMN": {
	///      "$ref": "#/components/schemas/schemas-PlmnId"
	///    },
	///    "trackingAreaIdList": {
	///      "$ref": "#/components/schemas/TaiList"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TopologicalServiceArea {
		#[serde(rename = "cellIdList", default, skip_serializing_if = "Vec::is_empty")]
		pub cell_id_list: Vec<i64>,
		#[serde(
			rename = "servingPLMN",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_plmn: Option<SchemasPlmnId>,
		#[serde(
			rename = "trackingAreaIdList",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub tracking_area_id_list: Option<TaiList>,
	}

	impl From<&TopologicalServiceArea> for TopologicalServiceArea {
		fn from(value: &TopologicalServiceArea) -> Self {
			value.clone()
		}
	}

	/// TrafficForwardingWay
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "N6",
	///    "N19",
	///    "LOCAL_SWITCH"
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
	pub enum TrafficForwardingWay {
		#[default]
		N6,
		N19,
		#[serde(rename = "LOCAL_SWITCH")]
		LocalSwitch,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&TrafficForwardingWay> for TrafficForwardingWay {
		fn from(value: &TrafficForwardingWay) -> Self {
			value.clone()
		}
	}

	impl ToString for TrafficForwardingWay {
		fn to_string(&self) -> String {
			match *self {
				Self::N6 => "N6".to_string(),
				Self::N19 => "N19".to_string(),
				Self::LocalSwitch => "LOCAL_SWITCH".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TrafficForwardingWay {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"N6" => Ok(Self::N6),
				"N19" => Ok(Self::N19),
				"LOCAL_SWITCH" => Ok(Self::LocalSwitch),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for TrafficForwardingWay {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TrafficForwardingWay {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TrafficForwardingWay {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// TransmitterInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "proseSourceIPAddress": {
	///      "$ref": "#/components/schemas/IpAddr"
	///    },
	///    "proseSourceL2Id": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TransmitterInfo {
		#[serde(
			rename = "proseSourceIPAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_source_ip_address: Option<IpAddr>,
		#[serde(
			rename = "proseSourceL2Id",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_source_l2_id: Option<String>,
	}

	impl From<&TransmitterInfo> for TransmitterInfo {
		fn from(value: &TransmitterInfo) -> Self {
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

	/// TrunkGroupId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "incomingTrunkGroupID": {
	///      "type": "string"
	///    },
	///    "outgoingTrunkGroupID": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TrunkGroupId {
		#[serde(
			rename = "incomingTrunkGroupID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub incoming_trunk_group_id: Option<String>,
		#[serde(
			rename = "outgoingTrunkGroupID",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub outgoing_trunk_group_id: Option<String>,
	}

	impl From<&TrunkGroupId> for TrunkGroupId {
		fn from(value: &TrunkGroupId) -> Self {
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

	/// UeTransferType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "INTRA_UE",
	///    "INTER_UE"
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
	pub enum UeTransferType {
		#[default]
		#[serde(rename = "INTRA_UE")]
		IntraUe,
		#[serde(rename = "INTER_UE")]
		InterUe,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UeTransferType> for UeTransferType {
		fn from(value: &UeTransferType) -> Self {
			value.clone()
		}
	}

	impl ToString for UeTransferType {
		fn to_string(&self) -> String {
			match *self {
				Self::IntraUe => "INTRA_UE".to_string(),
				Self::InterUe => "INTER_UE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UeTransferType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"INTRA_UE" => Ok(Self::IntraUe),
				"INTER_UE" => Ok(Self::InterUe),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UeTransferType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UeTransferType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UeTransferType {
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

	/// UsedUnitContainer
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "localSequenceNumber"
	///  ],
	///  "properties": {
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "eventTimeStamps": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DateTime"
	///      },
	///      "minItems": 0
	///    },
	///    "localSequenceNumber": {
	///      "type": "integer"
	///    },
	///    "nSPAContainerInformation": {
	///      "$ref": "#/components/schemas/NSPAContainerInformation"
	///    },
	///    "pC5ContainerInformation": {
	///      "$ref": "#/components/schemas/PC5ContainerInformation"
	///    },
	///    "pDUContainerInformation": {
	///      "$ref": "#/components/schemas/PDUContainerInformation"
	///    },
	///    "quotaManagementIndicator": {
	///      "$ref": "#/components/schemas/QuotaManagementIndicator"
	///    },
	///    "serviceId": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "serviceSpecificUnits": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "time": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "totalVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    },
	///    "triggerTimestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "triggers": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Trigger"
	///      },
	///      "minItems": 0
	///    },
	///    "uplinkVolume": {
	///      "$ref": "#/components/schemas/Uint64"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UsedUnitContainer {
		#[serde(
			rename = "downlinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub downlink_volume: Option<Uint64>,
		#[serde(
			rename = "eventTimeStamps",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub event_time_stamps: Vec<DateTime>,
		#[serde(rename = "localSequenceNumber")]
		pub local_sequence_number: i64,
		#[serde(
			rename = "nSPAContainerInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n_spa_container_information: Option<NspaContainerInformation>,
		#[serde(
			rename = "pC5ContainerInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub p_c5_container_information: Option<Pc5ContainerInformation>,
		#[serde(
			rename = "pDUContainerInformation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub p_du_container_information: Option<PduContainerInformation>,
		#[serde(
			rename = "quotaManagementIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub quota_management_indicator: Option<QuotaManagementIndicator>,
		#[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
		pub service_id: Option<Uint32>,
		#[serde(
			rename = "serviceSpecificUnits",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_specific_units: Option<Uint64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub time: Option<Uint32>,
		#[serde(
			rename = "totalVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub total_volume: Option<Uint64>,
		#[serde(
			rename = "triggerTimestamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub trigger_timestamp: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub triggers: Vec<Trigger>,
		#[serde(
			rename = "uplinkVolume",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uplink_volume: Option<Uint64>,
	}

	impl From<&UsedUnitContainer> for UsedUnitContainer {
		fn from(value: &UsedUnitContainer) -> Self {
			value.clone()
		}
	}

	/// UserInformation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "roamerInOut": {
	///      "$ref": "#/components/schemas/RoamerInOut"
	///    },
	///    "servedGPSI": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "servedPEI": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "unauthenticatedFlag": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UserInformation {
		#[serde(
			rename = "roamerInOut",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub roamer_in_out: Option<RoamerInOut>,
		#[serde(
			rename = "servedGPSI",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub served_gpsi: Option<Gpsi>,
		#[serde(rename = "servedPEI", default, skip_serializing_if = "Option::is_none")]
		pub served_pei: Option<Pei>,
		#[serde(
			rename = "unauthenticatedFlag",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub unauthenticated_flag: Option<bool>,
	}

	impl From<&UserInformation> for UserInformation {
		fn from(value: &UserInformation) -> Self {
			value.clone()
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

	/// VariablePart
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "variablePartType",
	///    "variablePartValue"
	///  ],
	///  "properties": {
	///    "variablePartOrder": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "variablePartType": {
	///      "$ref": "#/components/schemas/VariablePartType"
	///    },
	///    "variablePartValue": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
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
	pub struct VariablePart {
		#[serde(
			rename = "variablePartOrder",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub variable_part_order: Option<Uint32>,
		#[serde(rename = "variablePartType")]
		pub variable_part_type: VariablePartType,
		#[serde(rename = "variablePartValue")]
		pub variable_part_value: Vec<String>,
	}

	impl From<&VariablePart> for VariablePart {
		fn from(value: &VariablePart) -> Self {
			value.clone()
		}
	}

	/// VariablePartType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "INTEGER",
	///    "NUMBER",
	///    "TIME",
	///    "DATE",
	///    "CURRENCY"
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
	pub enum VariablePartType {
		#[default]
		#[serde(rename = "INTEGER")]
		Integer,
		#[serde(rename = "NUMBER")]
		Number,
		#[serde(rename = "TIME")]
		Time,
		#[serde(rename = "DATE")]
		Date,
		#[serde(rename = "CURRENCY")]
		Currency,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&VariablePartType> for VariablePartType {
		fn from(value: &VariablePartType) -> Self {
			value.clone()
		}
	}

	impl ToString for VariablePartType {
		fn to_string(&self) -> String {
			match *self {
				Self::Integer => "INTEGER".to_string(),
				Self::Number => "NUMBER".to_string(),
				Self::Time => "TIME".to_string(),
				Self::Date => "DATE".to_string(),
				Self::Currency => "CURRENCY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for VariablePartType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"INTEGER" => Ok(Self::Integer),
				"NUMBER" => Ok(Self::Number),
				"TIME" => Ok(Self::Time),
				"DATE" => Ok(Self::Date),
				"CURRENCY" => Ok(Self::Currency),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for VariablePartType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for VariablePartType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for VariablePartType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// VirtualResource
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "virtualDisk": {
	///      "type": "integer"
	///    },
	///    "virtualMemory": {
	///      "type": "integer"
	///    },
	///    "virutalCPU": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VirtualResource {
		#[serde(
			rename = "virtualDisk",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub virtual_disk: Option<i64>,
		#[serde(
			rename = "virtualMemory",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub virtual_memory: Option<i64>,
		#[serde(
			rename = "virutalCPU",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub virutal_cpu: Option<String>,
	}

	impl From<&VirtualResource> for VirtualResource {
		fn from(value: &VirtualResource) -> Self {
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

	/// _3gpppsDataOffStatus
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub enum _3gpppsDataOffStatus {
		#[default]
		#[serde(rename = "ACTIVE")]
		Active,
		#[serde(rename = "INACTIVE")]
		Inactive,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&_3gpppsDataOffStatus> for _3gpppsDataOffStatus {
		fn from(value: &_3gpppsDataOffStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for _3gpppsDataOffStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Active => "ACTIVE".to_string(),
				Self::Inactive => "INACTIVE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for _3gpppsDataOffStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ACTIVE" => Ok(Self::Active),
				"INACTIVE" => Ok(Self::Inactive),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for _3gpppsDataOffStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for _3gpppsDataOffStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for _3gpppsDataOffStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// _5glanTypeService
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "internalGroupIdentifier": {
	///      "$ref": "#/components/schemas/GroupId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct _5glanTypeService {
		#[serde(
			rename = "internalGroupIdentifier",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub internal_group_identifier: Option<GroupId>,
	}

	impl From<&_5glanTypeService> for _5glanTypeService {
		fn from(value: &_5glanTypeService) -> Self {
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
