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

	/// Represents the access technology
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the access technology",
	///  "type": "string",
	///  "enum": [
	///    "NR",
	///    "EUTRAN_IN_WBS1_MODE_AND_NBS1_MODE",
	///    "EUTRAN_IN_NBS1_MODE_ONLY",
	///    "EUTRAN_IN_WBS1_MODE_ONLY",
	///    "UTRAN",
	///    "GSM_AND_ECGSM_IoT",
	///    "GSM_WITHOUT_ECGSM_IoT",
	///    "ECGSM_IoT_ONLY",
	///    "CDMA_1xRTT",
	///    "CDMA_HRPD",
	///    "GSM_COMPACT"
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
	pub enum AccessTech {
		#[default]
		#[serde(rename = "NR")]
		Nr,
		#[serde(rename = "EUTRAN_IN_WBS1_MODE_AND_NBS1_MODE")]
		EutranInWbs1ModeAndNbs1Mode,
		#[serde(rename = "EUTRAN_IN_NBS1_MODE_ONLY")]
		EutranInNbs1ModeOnly,
		#[serde(rename = "EUTRAN_IN_WBS1_MODE_ONLY")]
		EutranInWbs1ModeOnly,
		#[serde(rename = "UTRAN")]
		Utran,
		#[serde(rename = "GSM_AND_ECGSM_IoT")]
		GsmAndEcgsmIoT,
		#[serde(rename = "GSM_WITHOUT_ECGSM_IoT")]
		GsmWithoutEcgsmIoT,
		#[serde(rename = "ECGSM_IoT_ONLY")]
		EcgsmIoTOnly,
		#[serde(rename = "CDMA_1xRTT")]
		Cdma1xRtt,
		#[serde(rename = "CDMA_HRPD")]
		CdmaHrpd,
		#[serde(rename = "GSM_COMPACT")]
		GsmCompact,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AccessTech> for AccessTech {
		fn from(value: &AccessTech) -> Self {
			value.clone()
		}
	}

	impl ToString for AccessTech {
		fn to_string(&self) -> String {
			match *self {
				Self::Nr => "NR".to_string(),
				Self::EutranInWbs1ModeAndNbs1Mode => {
					"EUTRAN_IN_WBS1_MODE_AND_NBS1_MODE".to_string()
				}
				Self::EutranInNbs1ModeOnly => "EUTRAN_IN_NBS1_MODE_ONLY".to_string(),
				Self::EutranInWbs1ModeOnly => "EUTRAN_IN_WBS1_MODE_ONLY".to_string(),
				Self::Utran => "UTRAN".to_string(),
				Self::GsmAndEcgsmIoT => "GSM_AND_ECGSM_IoT".to_string(),
				Self::GsmWithoutEcgsmIoT => "GSM_WITHOUT_ECGSM_IoT".to_string(),
				Self::EcgsmIoTOnly => "ECGSM_IoT_ONLY".to_string(),
				Self::Cdma1xRtt => "CDMA_1xRTT".to_string(),
				Self::CdmaHrpd => "CDMA_HRPD".to_string(),
				Self::GsmCompact => "GSM_COMPACT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AccessTech {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NR" => Ok(Self::Nr),
				"EUTRAN_IN_WBS1_MODE_AND_NBS1_MODE" => Ok(Self::EutranInWbs1ModeAndNbs1Mode),
				"EUTRAN_IN_NBS1_MODE_ONLY" => Ok(Self::EutranInNbs1ModeOnly),
				"EUTRAN_IN_WBS1_MODE_ONLY" => Ok(Self::EutranInWbs1ModeOnly),
				"UTRAN" => Ok(Self::Utran),
				"GSM_AND_ECGSM_IoT" => Ok(Self::GsmAndEcgsmIoT),
				"GSM_WITHOUT_ECGSM_IoT" => Ok(Self::GsmWithoutEcgsmIoT),
				"ECGSM_IoT_ONLY" => Ok(Self::EcgsmIoTOnly),
				"CDMA_1xRTT" => Ok(Self::Cdma1xRtt),
				"CDMA_HRPD" => Ok(Self::CdmaHrpd),
				"GSM_COMPACT" => Ok(Self::GsmCompact),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AccessTech {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AccessTech {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AccessTech {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Contains indication whether the acknowledgement from UE is needed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains indication whether the acknowledgement from UE
	/// is needed.",
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AckInd(pub bool);

	impl ::std::ops::Deref for AckInd {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<AckInd> for bool {
		fn from(value: AckInd) -> Self {
			value.0
		}
	}

	impl From<&AckInd> for AckInd {
		fn from(value: &AckInd) -> Self {
			value.clone()
		}
	}

	impl From<bool> for AckInd {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AckInd {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for AckInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AckInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AckInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for AckInd {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Indicates the result of the authentication.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the result of the authentication.",
	///  "type": "string",
	///  "enum": [
	///    "AUTHENTICATION_SUCCESS",
	///    "AUTHENTICATION_FAILURE",
	///    "AUTHENTICATION_ONGOING"
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
	pub enum AuthResult {
		#[default]
		#[serde(rename = "AUTHENTICATION_SUCCESS")]
		AuthenticationSuccess,
		#[serde(rename = "AUTHENTICATION_FAILURE")]
		AuthenticationFailure,
		#[serde(rename = "AUTHENTICATION_ONGOING")]
		AuthenticationOngoing,
	}

	impl From<&AuthResult> for AuthResult {
		fn from(value: &AuthResult) -> Self {
			value.clone()
		}
	}

	impl ToString for AuthResult {
		fn to_string(&self) -> String {
			match *self {
				Self::AuthenticationSuccess => "AUTHENTICATION_SUCCESS".to_string(),
				Self::AuthenticationFailure => "AUTHENTICATION_FAILURE".to_string(),
				Self::AuthenticationOngoing => "AUTHENTICATION_ONGOING".to_string(),
			}
		}
	}

	impl std::str::FromStr for AuthResult {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AUTHENTICATION_SUCCESS" => Ok(Self::AuthenticationSuccess),
				"AUTHENTICATION_FAILURE" => Ok(Self::AuthenticationFailure),
				"AUTHENTICATION_ONGOING" => Ok(Self::AuthenticationOngoing),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AuthResult {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AuthResult {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AuthResult {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the authentication method used.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the authentication method used.",
	///  "type": "string",
	///  "enum": [
	///    "5G_AKA",
	///    "EAP_AKA_PRIME",
	///    "EAP_TLS",
	///    "EAP_TTLS"
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
	pub enum AuthType {
		#[default]
		#[serde(rename = "5G_AKA")]
		FiveGAka,
		#[serde(rename = "EAP_AKA_PRIME")]
		EapAkaPrime,
		#[serde(rename = "EAP_TLS")]
		EapTls,
		#[serde(rename = "EAP_TTLS")]
		EapTtls,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AuthType> for AuthType {
		fn from(value: &AuthType) -> Self {
			value.clone()
		}
	}

	impl ToString for AuthType {
		fn to_string(&self) -> String {
			match *self {
				Self::FiveGAka => "5G_AKA".to_string(),
				Self::EapAkaPrime => "EAP_AKA_PRIME".to_string(),
				Self::EapTls => "EAP_TLS".to_string(),
				Self::EapTtls => "EAP_TTLS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AuthType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"5G_AKA" => Ok(Self::FiveGAka),
				"EAP_AKA_PRIME" => Ok(Self::EapAkaPrime),
				"EAP_TLS" => Ok(Self::EapTls),
				"EAP_TTLS" => Ok(Self::EapTtls),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AuthType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AuthType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AuthType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the UE id (i.e. SUCI or SUPI) and the Serving Network Name.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the UE id (i.e. SUCI or SUPI) and the Serving
	/// Network Name.",
	///  "type": "object",
	///  "required": [
	///    "servingNetworkName",
	///    "supiOrSuci"
	///  ],
	///  "properties": {
	///    "cellCagInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CagId"
	///      },
	///      "minItems": 1
	///    },
	///    "disasterRoamingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "n5gcInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "nswoInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "onboardingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "resynchronizationInfo": {
	///      "$ref": "#/components/schemas/ResynchronizationInfo"
	///    },
	///    "routingIndicator": {
	///      "type": "string",
	///      "pattern": "^[0-9]{1,4}$"
	///    },
	///    "servingNetworkName": {
	///      "$ref": "#/components/schemas/ServingNetworkName"
	///    },
	///    "supiOrSuci": {
	///      "$ref": "#/components/schemas/SupiOrSuci"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "traceData": {
	///      "$ref": "#/components/schemas/TraceData"
	///    },
	///    "udmGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AuthenticationInfo {
		#[serde(rename = "cellCagInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub cell_cag_info: Vec<CagId>,
		#[serde(rename = "disasterRoamingInd", default)]
		pub disaster_roaming_ind: bool,
		#[serde(rename = "n5gcInd", default)]
		pub n5gc_ind: bool,
		#[serde(rename = "nswoInd", default)]
		pub nswo_ind: bool,
		#[serde(rename = "onboardingInd", default)]
		pub onboarding_ind: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(
			rename = "resynchronizationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub resynchronization_info: Option<ResynchronizationInfo>,
		#[serde(
			rename = "routingIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub routing_indicator: Option<AuthenticationInfoRoutingIndicator>,
		#[serde(rename = "servingNetworkName")]
		pub serving_network_name: ServingNetworkName,
		#[serde(rename = "supiOrSuci")]
		pub supi_or_suci: SupiOrSuci,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "traceData", default, skip_serializing_if = "Option::is_none")]
		pub trace_data: Option<TraceData>,
		#[serde(
			rename = "udmGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub udm_group_id: Option<NfGroupId>,
	}

	impl From<&AuthenticationInfo> for AuthenticationInfo {
		fn from(value: &AuthenticationInfo) -> Self {
			value.clone()
		}
	}

	/// AuthenticationInfoRoutingIndicator
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]{1,4}$"
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
	pub struct AuthenticationInfoRoutingIndicator(String);

	impl ::std::ops::Deref for AuthenticationInfoRoutingIndicator {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AuthenticationInfoRoutingIndicator> for String {
		fn from(value: AuthenticationInfoRoutingIndicator) -> Self {
			value.0
		}
	}

	impl From<&AuthenticationInfoRoutingIndicator> for AuthenticationInfoRoutingIndicator {
		fn from(value: &AuthenticationInfoRoutingIndicator) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AuthenticationInfoRoutingIndicator {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{1,4}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{1,4}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for AuthenticationInfoRoutingIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AuthenticationInfoRoutingIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AuthenticationInfoRoutingIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AuthenticationInfoRoutingIndicator {
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

	/// Autn
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{32}$"
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
	pub struct Autn(String);

	impl ::std::ops::Deref for Autn {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Autn> for String {
		fn from(value: Autn) -> Self {
			value.0
		}
	}

	impl From<&Autn> for Autn {
		fn from(value: &Autn) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Autn {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{32}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{32}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Autn {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Autn {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Autn {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Autn {
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

	/// Auts
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{28}$"
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
	pub struct Auts(String);

	impl ::std::ops::Deref for Auts {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Auts> for String {
		fn from(value: Auts) -> Self {
			value.0
		}
	}

	impl From<&Auts> for Auts {
		fn from(value: &Auts) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Auts {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{28}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{28}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Auts {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Auts {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Auts {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Auts {
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

	/// Contains Authentication Vector for method 5G AKA.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains Authentication Vector for method 5G AKA.",
	///  "type": "object",
	///  "required": [
	///    "autn",
	///    "hxresStar",
	///    "rand"
	///  ],
	///  "properties": {
	///    "autn": {
	///      "$ref": "#/components/schemas/Autn"
	///    },
	///    "hxresStar": {
	///      "$ref": "#/components/schemas/HxresStar"
	///    },
	///    "rand": {
	///      "$ref": "#/components/schemas/Rand"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Av5gAka {
		pub autn: Autn,
		#[serde(rename = "hxresStar")]
		pub hxres_star: HxresStar,
		pub rand: Rand,
	}

	impl From<&Av5gAka> for Av5gAka {
		fn from(value: &Av5gAka) -> Self {
			value.clone()
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

	/// Contains the result of the authentication.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the result of the authentication.",
	///  "type": "object",
	///  "required": [
	///    "resStar"
	///  ],
	///  "properties": {
	///    "resStar": {
	///      "$ref": "#/components/schemas/ResStar"
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
	pub struct ConfirmationData {
		#[serde(rename = "resStar")]
		pub res_star: ResStar,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&ConfirmationData> for ConfirmationData {
		fn from(value: &ConfirmationData) -> Self {
			value.clone()
		}
	}

	/// Contains the result of the authentication
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the result of the authentication",
	///  "type": "object",
	///  "required": [
	///    "authResult"
	///  ],
	///  "properties": {
	///    "authResult": {
	///      "$ref": "#/components/schemas/AuthResult"
	///    },
	///    "kseaf": {
	///      "$ref": "#/components/schemas/Kseaf"
	///    },
	///    "pvsInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServerAddressingInfo"
	///      },
	///      "minItems": 1
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
	pub struct ConfirmationDataResponse {
		#[serde(rename = "authResult")]
		pub auth_result: AuthResult,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub kseaf: Option<Kseaf>,
		#[serde(rename = "pvsInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub pvs_info: Vec<ServerAddressingInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
	}

	impl From<&ConfirmationDataResponse> for ConfirmationDataResponse {
		fn from(value: &ConfirmationDataResponse) -> Self {
			value.clone()
		}
	}

	/// CounterSoR.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "CounterSoR.",
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
	pub struct CounterSor(String);

	impl ::std::ops::Deref for CounterSor {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<CounterSor> for String {
		fn from(value: CounterSor) -> Self {
			value.0
		}
	}

	impl From<&CounterSor> for CounterSor {
		fn from(value: &CounterSor) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for CounterSor {
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

	impl ::std::convert::TryFrom<&str> for CounterSor {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for CounterSor {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for CounterSor {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CounterSor {
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

	/// CounterUPU.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "CounterUPU.",
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
	pub struct CounterUpu(String);

	impl ::std::ops::Deref for CounterUpu {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<CounterUpu> for String {
		fn from(value: CounterUpu) -> Self {
			value.0
		}
	}

	impl From<&CounterUpu> for CounterUpu {
		fn from(value: &CounterUpu) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for CounterUpu {
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

	impl ::std::convert::TryFrom<&str> for CounterUpu {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for CounterUpu {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for CounterUpu {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CounterUpu {
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

	/// Contains the UE id (i.e. SUPI).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the UE id (i.e. SUPI).",
	///  "type": "object",
	///  "required": [
	///    "supi"
	///  ],
	///  "properties": {
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
	pub struct DeregistrationInfo {
		pub supi: Supi,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&DeregistrationInfo> for DeregistrationInfo {
		fn from(value: &DeregistrationInfo) -> Self {
			value.clone()
		}
	}

	/// contains an EAP packet
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "contains an EAP packet",
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
	pub struct EapPayload(pub Option<String>);

	impl ::std::ops::Deref for EapPayload {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<EapPayload> for Option<String> {
		fn from(value: EapPayload) -> Self {
			value.0
		}
	}

	impl From<&EapPayload> for EapPayload {
		fn from(value: &EapPayload) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for EapPayload {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// Contains information related to the EAP session.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains information related to the EAP session.",
	///  "type": "object",
	///  "required": [
	///    "eapPayload"
	///  ],
	///  "properties": {
	///    "_links": {
	///      "description": "A map(list of key-value pairs) where member serves
	/// as key",
	///      "type": "object",
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/LinksValueSchema"
	///      }
	///    },
	///    "authResult": {
	///      "$ref": "#/components/schemas/AuthResult"
	///    },
	///    "eapPayload": {
	///      "$ref": "#/components/schemas/EapPayload"
	///    },
	///    "kSeaf": {
	///      "$ref": "#/components/schemas/Kseaf"
	///    },
	///    "msk": {
	///      "$ref": "#/components/schemas/Msk"
	///    },
	///    "pvsInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServerAddressingInfo"
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
	pub struct EapSession {
		#[serde(
			rename = "authResult",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub auth_result: Option<AuthResult>,
		#[serde(rename = "eapPayload")]
		pub eap_payload: EapPayload,
		#[serde(rename = "kSeaf", default, skip_serializing_if = "Option::is_none")]
		pub k_seaf: Option<Kseaf>,
		/// A map(list of key-value pairs) where member serves as key
		#[serde(
			rename = "_links",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub links: ::std::collections::HashMap<String, LinksValueSchema>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub msk: Option<Msk>,
		#[serde(rename = "pvsInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub pvs_info: Vec<ServerAddressingInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&EapSession> for EapSession {
		fn from(value: &EapSession) -> Self {
			value.clone()
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

	/// Contains the HXRES*.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the HXRES*.",
	///  "type": "string",
	///  "pattern": "[A-Fa-f0-9]{32}"
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
	pub struct HxresStar(String);

	impl ::std::ops::Deref for HxresStar {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<HxresStar> for String {
		fn from(value: HxresStar) -> Self {
			value.0
		}
	}

	impl From<&HxresStar> for HxresStar {
		fn from(value: &HxresStar) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for HxresStar {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("[A-Fa-f0-9]{32}")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"[A-Fa-f0-9]{32}\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for HxresStar {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for HxresStar {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for HxresStar {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for HxresStar {
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

	/// Contains the KNR_ProSe.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the KNR_ProSe.",
	///  "type": "string",
	///  "pattern": "[A-Fa-f0-9]{64}"
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
	pub struct KnrProSe(String);

	impl ::std::ops::Deref for KnrProSe {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<KnrProSe> for String {
		fn from(value: KnrProSe) -> Self {
			value.0
		}
	}

	impl From<&KnrProSe> for KnrProSe {
		fn from(value: &KnrProSe) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for KnrProSe {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("[A-Fa-f0-9]{64}")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"[A-Fa-f0-9]{64}\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for KnrProSe {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for KnrProSe {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for KnrProSe {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for KnrProSe {
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

	/// Contains the Kseaf.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Kseaf.",
	///  "type": "string",
	///  "pattern": "[A-Fa-f0-9]{64}"
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
	pub struct Kseaf(String);

	impl ::std::ops::Deref for Kseaf {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Kseaf> for String {
		fn from(value: Kseaf) -> Self {
			value.0
		}
	}

	impl From<&Kseaf> for Kseaf {
		fn from(value: &Kseaf) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Kseaf {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("[A-Fa-f0-9]{64}")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"[A-Fa-f0-9]{64}\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Kseaf {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Kseaf {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Kseaf {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Kseaf {
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

	/// Contains the Master Session Key.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Master Session Key.",
	///  "type": "string",
	///  "pattern": "[A-Fa-f0-9]{128}"
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
	pub struct Msk(String);

	impl ::std::ops::Deref for Msk {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Msk> for String {
		fn from(value: Msk) -> Self {
			value.0
		}
	}

	impl From<&Msk> for Msk {
		fn from(value: &Msk) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Msk {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("[A-Fa-f0-9]{128}")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"[A-Fa-f0-9]{128}\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Msk {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Msk {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Msk {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Msk {
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

	/// contains an Nonce1
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "contains an Nonce1",
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
	pub struct Nonce1(pub Option<String>);

	impl ::std::ops::Deref for Nonce1 {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<Nonce1> for Option<String> {
		fn from(value: Nonce1) -> Self {
			value.0
		}
	}

	impl From<&Nonce1> for Nonce1 {
		fn from(value: &Nonce1) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for Nonce1 {
		fn from(value: Option<String>) -> Self {
			Self(value)
		}
	}

	/// contains an Nonce2
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "contains an Nonce2",
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
	pub struct Nonce2(pub Option<String>);

	impl ::std::ops::Deref for Nonce2 {
		type Target = Option<String>;
		fn deref(&self) -> &Option<String> {
			&self.0
		}
	}

	impl From<Nonce2> for Option<String> {
		fn from(value: Nonce2) -> Self {
			value.0
		}
	}

	impl From<&Nonce2> for Nonce2 {
		fn from(value: &Nonce2) -> Self {
			value.clone()
		}
	}

	impl From<Option<String>> for Nonce2 {
		fn from(value: Option<String>) -> Self {
			Self(value)
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

	/// ProSeAuthData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "oneOf": [
	///    {
	///      "$ref": "#/components/schemas/EapPayload"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProSeAuthData(pub EapPayload);

	impl ::std::ops::Deref for ProSeAuthData {
		type Target = EapPayload;
		fn deref(&self) -> &EapPayload {
			&self.0
		}
	}

	impl From<ProSeAuthData> for EapPayload {
		fn from(value: ProSeAuthData) -> Self {
			value.0
		}
	}

	impl From<&ProSeAuthData> for ProSeAuthData {
		fn from(value: &ProSeAuthData) -> Self {
			value.clone()
		}
	}

	impl From<EapPayload> for ProSeAuthData {
		fn from(value: EapPayload) -> Self {
			Self(value)
		}
	}

	/// Contains the information related to the resource generated to handle the
	/// ProSe authentication.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the information related to the resource
	/// generated to handle the ProSe authentication.",
	///  "type": "object",
	///  "required": [
	///    "_links",
	///    "authType",
	///    "proSeAuthData"
	///  ],
	///  "properties": {
	///    "_links": {
	///      "description": "A map(list of key-value pairs) where member serves
	/// as key",
	///      "type": "object",
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/LinksValueSchema"
	///      }
	///    },
	///    "authType": {
	///      "$ref": "#/components/schemas/AuthType"
	///    },
	///    "proSeAuthData": {
	///      "$ref": "#/components/schemas/ProSeAuthData"
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
	pub struct ProSeAuthenticationCtx {
		#[serde(rename = "authType")]
		pub auth_type: AuthType,
		/// A map(list of key-value pairs) where member serves as key
		#[serde(rename = "_links")]
		pub links: ::std::collections::HashMap<String, LinksValueSchema>,
		#[serde(rename = "proSeAuthData")]
		pub pro_se_auth_data: ProSeAuthData,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&ProSeAuthenticationCtx> for ProSeAuthenticationCtx {
		fn from(value: &ProSeAuthenticationCtx) -> Self {
			value.clone()
		}
	}

	/// Contains the UE id (i.e. SUCI) or CP-PRUK ID (in 5gPrukId IE), Relay
	/// Service Code and Nonce_1.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the UE id (i.e. SUCI) or CP-PRUK ID (in
	/// 5gPrukId IE), Relay Service Code and Nonce_1.\n",
	///  "type": "object",
	///  "required": [
	///    "nonce1",
	///    "relayServiceCode",
	///    "servingNetworkName"
	///  ],
	///  "properties": {
	///    "5gPrukId": {
	///      "$ref": "#/components/schemas/5GPrukId"
	///    },
	///    "nonce1": {
	///      "$ref": "#/components/schemas/Nonce1"
	///    },
	///    "relayServiceCode": {
	///      "$ref": "#/components/schemas/RelayServiceCode"
	///    },
	///    "servingNetworkName": {
	///      "$ref": "#/components/schemas/ServingNetworkName"
	///    },
	///    "supiOrSuci": {
	///      "$ref": "#/components/schemas/SupiOrSuci"
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
	pub struct ProSeAuthenticationInfo {
		#[serde(rename = "5gPrukId", default, skip_serializing_if = "Option::is_none")]
		pub five_g_pruk_id: Option<_5gPrukId>,
		pub nonce1: Nonce1,
		#[serde(rename = "relayServiceCode")]
		pub relay_service_code: RelayServiceCode,
		#[serde(rename = "servingNetworkName")]
		pub serving_network_name: ServingNetworkName,
		#[serde(
			rename = "supiOrSuci",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supi_or_suci: Option<SupiOrSuci>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&ProSeAuthenticationInfo> for ProSeAuthenticationInfo {
		fn from(value: &ProSeAuthenticationInfo) -> Self {
			value.clone()
		}
	}

	/// Successful authentication for CP-PRUK ID.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Successful authentication for CP-PRUK ID.",
	///  "type": "object",
	///  "properties": {
	///    "knrProSe": {
	///      "$ref": "#/components/schemas/KnrProSe"
	///    },
	///    "nonce2": {
	///      "$ref": "#/components/schemas/Nonce2"
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
	pub struct ProSeAuthenticationResult {
		#[serde(rename = "knrProSe", default, skip_serializing_if = "Option::is_none")]
		pub knr_pro_se: Option<KnrProSe>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nonce2: Option<Nonce2>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&ProSeAuthenticationResult> for ProSeAuthenticationResult {
		fn from(value: &ProSeAuthenticationResult) -> Self {
			value.clone()
		}
	}

	/// Contains information related to the EAP session. If present the 5gPrukId
	/// IE shall carry the CP-PRUK ID.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains information related to the EAP session. If
	/// present the 5gPrukId IE shall carry the CP-PRUK ID.",
	///  "type": "object",
	///  "required": [
	///    "eapPayload"
	///  ],
	///  "properties": {
	///    "5gPrukId": {
	///      "$ref": "#/components/schemas/5GPrukId"
	///    },
	///    "_links": {
	///      "description": "A map(list of key-value pairs) where member serves
	/// as key",
	///      "type": "object",
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/LinksValueSchema"
	///      }
	///    },
	///    "authResult": {
	///      "$ref": "#/components/schemas/AuthResult"
	///    },
	///    "eapPayload": {
	///      "$ref": "#/components/schemas/EapPayload"
	///    },
	///    "knrProSe": {
	///      "$ref": "#/components/schemas/KnrProSe"
	///    },
	///    "nonce2": {
	///      "$ref": "#/components/schemas/Nonce2"
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
	pub struct ProSeEapSession {
		#[serde(
			rename = "authResult",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub auth_result: Option<AuthResult>,
		#[serde(rename = "eapPayload")]
		pub eap_payload: EapPayload,
		#[serde(rename = "5gPrukId", default, skip_serializing_if = "Option::is_none")]
		pub five_g_pruk_id: Option<_5gPrukId>,
		#[serde(rename = "knrProSe", default, skip_serializing_if = "Option::is_none")]
		pub knr_pro_se: Option<KnrProSe>,
		/// A map(list of key-value pairs) where member serves as key
		#[serde(
			rename = "_links",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub links: ::std::collections::HashMap<String, LinksValueSchema>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nonce2: Option<Nonce2>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&ProSeEapSession> for ProSeEapSession {
		fn from(value: &ProSeEapSession) -> Self {
			value.clone()
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

	/// Rand
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{32}$"
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
	pub struct Rand(String);

	impl ::std::ops::Deref for Rand {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Rand> for String {
		fn from(value: Rand) -> Self {
			value.0
		}
	}

	impl From<&Rand> for Rand {
		fn from(value: &Rand) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Rand {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{32}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{32}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Rand {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Rand {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Rand {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Rand {
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

	/// Contains the RES*.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the RES*.",
	///  "type": [
	///    "string",
	///    "null"
	///  ],
	///  "pattern": "[A-Fa-f0-9]{32}"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ResStar(pub Option<ResStarInner>);

	impl ::std::ops::Deref for ResStar {
		type Target = Option<ResStarInner>;
		fn deref(&self) -> &Option<ResStarInner> {
			&self.0
		}
	}

	impl From<ResStar> for Option<ResStarInner> {
		fn from(value: ResStar) -> Self {
			value.0
		}
	}

	impl From<&ResStar> for ResStar {
		fn from(value: &ResStar) -> Self {
			value.clone()
		}
	}

	impl From<Option<ResStarInner>> for ResStar {
		fn from(value: Option<ResStarInner>) -> Self {
			Self(value)
		}
	}

	/// Contains the RES*.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the RES*.",
	///  "type": "string",
	///  "pattern": "[A-Fa-f0-9]{32}"
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
	pub struct ResStarInner(String);

	impl ::std::ops::Deref for ResStarInner {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ResStarInner> for String {
		fn from(value: ResStarInner) -> Self {
			value.0
		}
	}

	impl From<&ResStarInner> for ResStarInner {
		fn from(value: &ResStarInner) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ResStarInner {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("[A-Fa-f0-9]{32}")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"[A-Fa-f0-9]{32}\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for ResStarInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ResStarInner {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ResStarInner {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ResStarInner {
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

	/// ResynchronizationInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "auts",
	///    "rand"
	///  ],
	///  "properties": {
	///    "auts": {
	///      "$ref": "#/components/schemas/Auts"
	///    },
	///    "rand": {
	///      "$ref": "#/components/schemas/Rand"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ResynchronizationInfo {
		pub auts: Auts,
		pub rand: Rand,
	}

	impl From<&ResynchronizationInfo> for ResynchronizationInfo {
		fn from(value: &ResynchronizationInfo) -> Self {
			value.clone()
		}
	}

	/// Contains the UE id (i.e. SUPI) and the authentication indication.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the UE id (i.e. SUPI) and the authentication
	/// indication.",
	///  "type": "object",
	///  "required": [
	///    "authResult"
	///  ],
	///  "properties": {
	///    "authInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "authResult": {
	///      "$ref": "#/components/schemas/AuthResult"
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
	pub struct RgAuthCtx {
		#[serde(rename = "authInd", default)]
		pub auth_ind: bool,
		#[serde(rename = "authResult")]
		pub auth_result: AuthResult,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
	}

	impl From<&RgAuthCtx> for RgAuthCtx {
		fn from(value: &RgAuthCtx) -> Self {
			value.clone()
		}
	}

	/// Contains the UE id (i.e. SUCI) and the authenticated indication.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the UE id (i.e. SUCI) and the authenticated
	/// indication.",
	///  "type": "object",
	///  "required": [
	///    "authenticatedInd",
	///    "suci"
	///  ],
	///  "properties": {
	///    "authenticatedInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "suci": {
	///      "$ref": "#/components/schemas/Suci"
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
	pub struct RgAuthenticationInfo {
		#[serde(rename = "authenticatedInd")]
		pub authenticated_ind: bool,
		pub suci: Suci,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&RgAuthenticationInfo> for RgAuthenticationInfo {
		fn from(value: &RgAuthenticationInfo) -> Self {
			value.clone()
		}
	}

	/// Represents a routing indicator.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a routing indicator.",
	///  "type": "string",
	///  "pattern": "^[0-9]{1,4}$"
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
	pub struct RoutingId(String);

	impl ::std::ops::Deref for RoutingId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<RoutingId> for String {
		fn from(value: RoutingId) -> Self {
			value.0
		}
	}

	impl From<&RoutingId> for RoutingId {
		fn from(value: &RoutingId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for RoutingId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{1,4}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{1,4}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for RoutingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for RoutingId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for RoutingId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for RoutingId {
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

	/// Contains a secure packet.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a secure packet.",
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
	pub struct SecuredPacket(pub String);

	impl ::std::ops::Deref for SecuredPacket {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SecuredPacket> for String {
		fn from(value: SecuredPacket) -> Self {
			value.0
		}
	}

	impl From<&SecuredPacket> for SecuredPacket {
		fn from(value: &SecuredPacket) -> Self {
			value.clone()
		}
	}

	impl From<String> for SecuredPacket {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SecuredPacket {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for SecuredPacket {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// ServingNetworkName
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern":
	/// "^(5G:mnc[0-9]{3}[.]mcc[0-9]{3}[.]3gppnetwork[.]org(:[A-F0-9]{11})?)|5G:
	/// NSWO$"
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
	pub struct ServingNetworkName(String);

	impl ::std::ops::Deref for ServingNetworkName {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ServingNetworkName> for String {
		fn from(value: ServingNetworkName) -> Self {
			value.0
		}
	}

	impl From<&ServingNetworkName> for ServingNetworkName {
		fn from(value: &ServingNetworkName) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ServingNetworkName {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"^(5G:mnc[0-9]{3}[.]mcc[0-9]{3}[.]3gppnetwork[.]org(:[A-F0-9]{11})?)|5G:NSWO$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^(5G:mnc[0-9]{3}[.]mcc[0-9]{3}[.]3gppnetwork[.]org(:[A-F0-9]{11})?\
				            )|5G:NSWO$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for ServingNetworkName {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ServingNetworkName {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ServingNetworkName {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ServingNetworkName {
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

	/// SorHeader
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
	pub struct SorHeader(pub Bytes);

	impl ::std::ops::Deref for SorHeader {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<SorHeader> for Bytes {
		fn from(value: SorHeader) -> Self {
			value.0
		}
	}

	impl From<&SorHeader> for SorHeader {
		fn from(value: &SorHeader) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for SorHeader {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SorHeader {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SorHeader {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SorHeader {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SorHeader {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SorHeader {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains the Steering Information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Steering Information.",
	///  "type": "object",
	///  "required": [
	///    "ackInd"
	///  ],
	///  "properties": {
	///    "ackInd": {
	///      "$ref": "#/components/schemas/AckInd"
	///    },
	///    "sorHeader": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "sorTransparentInfo": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "steeringContainer": {
	///      "$ref": "#/components/schemas/SteeringContainer"
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
	pub struct SorInfo {
		#[serde(rename = "ackInd")]
		pub ack_ind: AckInd,
		#[serde(rename = "sorHeader", default, skip_serializing_if = "Option::is_none")]
		pub sor_header: Option<Bytes>,
		#[serde(
			rename = "sorTransparentInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sor_transparent_info: Option<Bytes>,
		#[serde(
			rename = "steeringContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub steering_container: Option<SteeringContainer>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&SorInfo> for SorInfo {
		fn from(value: &SorInfo) -> Self {
			value.clone()
		}
	}

	/// MAC value for protecting SOR procedure (SoR-MAC-IAUSF and SoR-XMAC-IUE).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MAC value for protecting SOR procedure (SoR-MAC-IAUSF
	/// and SoR-XMAC-IUE).",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{32}$"
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
	pub struct SorMac(String);

	impl ::std::ops::Deref for SorMac {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SorMac> for String {
		fn from(value: SorMac) -> Self {
			value.0
		}
	}

	impl From<&SorMac> for SorMac {
		fn from(value: &SorMac) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SorMac {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{32}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{32}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SorMac {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SorMac {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SorMac {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SorMac {
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

	/// Contains the material generated for securing of SoR. It contains at
	/// least the SoR-MAC-IAUSF and CounterSoR.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the material generated for securing of SoR. It
	/// contains at least the SoR-MAC-IAUSF and CounterSoR.",
	///  "type": "object",
	///  "required": [
	///    "counterSor",
	///    "sorMacIausf"
	///  ],
	///  "properties": {
	///    "counterSor": {
	///      "$ref": "#/components/schemas/CounterSor"
	///    },
	///    "sorMacIausf": {
	///      "$ref": "#/components/schemas/SorMac"
	///    },
	///    "sorXmacIue": {
	///      "$ref": "#/components/schemas/SorMac"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SorSecurityInfo {
		#[serde(rename = "counterSor")]
		pub counter_sor: CounterSor,
		#[serde(rename = "sorMacIausf")]
		pub sor_mac_iausf: SorMac,
		#[serde(
			rename = "sorXmacIue",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sor_xmac_iue: Option<SorMac>,
	}

	impl From<&SorSecurityInfo> for SorSecurityInfo {
		fn from(value: &SorSecurityInfo) -> Self {
			value.clone()
		}
	}

	/// SorTransparentInfo
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
	pub struct SorTransparentInfo(pub Bytes);

	impl ::std::ops::Deref for SorTransparentInfo {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<SorTransparentInfo> for Bytes {
		fn from(value: SorTransparentInfo) -> Self {
			value.0
		}
	}

	impl From<&SorTransparentInfo> for SorTransparentInfo {
		fn from(value: &SorTransparentInfo) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for SorTransparentInfo {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SorTransparentInfo {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SorTransparentInfo {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SorTransparentInfo {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SorTransparentInfo {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SorTransparentInfo {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains the information sent to UE.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the information sent to UE.",
	///  "oneOf": [
	///    {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SteeringInfo"
	///      },
	///      "minItems": 1
	///    },
	///    {
	///      "$ref": "#/components/schemas/SecuredPacket"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum SteeringContainer {
		#[default]
		Variant0(Vec<SteeringInfo>),
		Variant1(SecuredPacket),
	}

	impl From<&SteeringContainer> for SteeringContainer {
		fn from(value: &SteeringContainer) -> Self {
			value.clone()
		}
	}

	impl From<Vec<SteeringInfo>> for SteeringContainer {
		fn from(value: Vec<SteeringInfo>) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<SecuredPacket> for SteeringContainer {
		fn from(value: SecuredPacket) -> Self {
			Self::Variant1(value)
		}
	}

	/// Contains a combination of one PLMN identity and zero or more access
	/// technologies.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a combination of one PLMN identity and zero or
	/// more access technologies.",
	///  "type": "object",
	///  "required": [
	///    "plmnId"
	///  ],
	///  "properties": {
	///    "accessTechList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessTech"
	///      },
	///      "minItems": 1
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
	pub struct SteeringInfo {
		#[serde(
			rename = "accessTechList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub access_tech_list: Vec<AccessTech>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
	}

	impl From<&SteeringInfo> for SteeringInfo {
		fn from(value: &SteeringInfo) -> Self {
			value.clone()
		}
	}

	/// Contains the SUCI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the SUCI.",
	///  "type": "string",
	///  "pattern":
	/// "^(suci-(0-[0-9]{3}-[0-9]{2,3}|[1-7]-.+)-[0-9]{1,4}-(0-0-.
	/// +|[a-fA-F1-9]-([1-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])-[a-fA-F0-9]+)|.
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
	)]
	pub struct Suci(String);

	impl ::std::ops::Deref for Suci {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<Suci> for String {
		fn from(value: Suci) -> Self {
			value.0
		}
	}

	impl From<&Suci> for Suci {
		fn from(value: &Suci) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for Suci {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"^(suci-(0-[0-9]{3}-[0-9]{2,3}|[1-7]-.+)-[0-9]{1,4}-(0-0-.\
				 +|[a-fA-F1-9]-([1-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])-[a-fA-F0-9]+)|.+)$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^(suci-(0-[0-9]{3}-[0-9]{2,3}|[1-7]-.+)-[0-9]{1,4}-(0-0-.\
				            +|[a-fA-F1-9]-([1-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])-[a-fA-F0-9]+)|.\
				            +)$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for Suci {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for Suci {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for Suci {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for Suci {
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

	/// Contains the information related to the resource generated to handle the
	/// UE authentication. It contains at least the UE id, Serving Network, the
	/// Authentication Method and related EAP information or related 5G-AKA
	/// information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the information related to the resource
	/// generated to handle the UE authentication. It contains at least the UE
	/// id, Serving Network, the Authentication Method and related EAP
	/// information or related 5G-AKA information.",
	///  "type": "object",
	///  "required": [
	///    "5gAuthData",
	///    "_links",
	///    "authType"
	///  ],
	///  "properties": {
	///    "5gAuthData": {
	///      "oneOf": [
	///        {
	///          "$ref": "#/components/schemas/Av5gAka"
	///        },
	///        {
	///          "$ref": "#/components/schemas/EapPayload"
	///        }
	///      ]
	///    },
	///    "_links": {
	///      "description": "A map(list of key-value pairs) where member serves
	/// as key",
	///      "type": "object",
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/LinksValueSchema"
	///      }
	///    },
	///    "authType": {
	///      "$ref": "#/components/schemas/AuthType"
	///    },
	///    "servingNetworkName": {
	///      "$ref": "#/components/schemas/ServingNetworkName"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeAuthenticationCtx {
		#[serde(rename = "authType")]
		pub auth_type: AuthType,
		#[serde(rename = "5gAuthData")]
		pub five_g_auth_data: UeAuthenticationCtx5gAuthData,
		/// A map(list of key-value pairs) where member serves as key
		#[serde(rename = "_links")]
		pub links: ::std::collections::HashMap<String, LinksValueSchema>,
		#[serde(
			rename = "servingNetworkName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network_name: Option<ServingNetworkName>,
	}

	impl From<&UeAuthenticationCtx> for UeAuthenticationCtx {
		fn from(value: &UeAuthenticationCtx) -> Self {
			value.clone()
		}
	}

	/// UeAuthenticationCtx5gAuthData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "oneOf": [
	///    {
	///      "$ref": "#/components/schemas/Av5gAka"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EapPayload"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum UeAuthenticationCtx5gAuthData {
		#[default]
		Av5gAka(Av5gAka),
		EapPayload(EapPayload),
	}

	impl From<&UeAuthenticationCtx5gAuthData> for UeAuthenticationCtx5gAuthData {
		fn from(value: &UeAuthenticationCtx5gAuthData) -> Self {
			value.clone()
		}
	}

	impl From<Av5gAka> for UeAuthenticationCtx5gAuthData {
		fn from(value: Av5gAka) -> Self {
			Self::Av5gAka(value)
		}
	}

	impl From<EapPayload> for UeAuthenticationCtx5gAuthData {
		fn from(value: EapPayload) -> Self {
			Self::EapPayload(value)
		}
	}

	/// Contains the indication of whether the acknowledgement from UE is
	/// needed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the indication of whether the acknowledgement
	/// from UE is needed.",
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UpuAckInd(pub bool);

	impl ::std::ops::Deref for UpuAckInd {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<UpuAckInd> for bool {
		fn from(value: UpuAckInd) -> Self {
			value.0
		}
	}

	impl From<&UpuAckInd> for UpuAckInd {
		fn from(value: &UpuAckInd) -> Self {
			value.clone()
		}
	}

	impl From<bool> for UpuAckInd {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for UpuAckInd {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for UpuAckInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UpuAckInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UpuAckInd {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for UpuAckInd {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contains UE parameters update data set (e.g., the updated Routing ID
	/// Data or the Default configured NSSAI or the disaster roaming
	/// parameters).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains UE parameters update data set (e.g., the
	/// updated Routing ID Data or the Default configured NSSAI or the disaster
	/// roaming parameters).",
	///  "type": "object",
	///  "properties": {
	///    "aol": {
	///      "type": "boolean"
	///    },
	///    "defaultConfNssai": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "drei": {
	///      "type": "boolean"
	///    },
	///    "routingId": {
	///      "$ref": "#/components/schemas/RoutingId"
	///    },
	///    "secPacket": {
	///      "$ref": "#/components/schemas/SecuredPacket"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UpuData {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub aol: Option<bool>,
		#[serde(
			rename = "defaultConfNssai",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub default_conf_nssai: Vec<Snssai>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub drei: Option<bool>,
		#[serde(rename = "routingId", default, skip_serializing_if = "Option::is_none")]
		pub routing_id: Option<RoutingId>,
		#[serde(rename = "secPacket", default, skip_serializing_if = "Option::is_none")]
		pub sec_packet: Option<SecuredPacket>,
	}

	impl From<&UpuData> for UpuData {
		fn from(value: &UpuData) -> Self {
			value.clone()
		}
	}

	/// Contains the "UPU Header" IE as specified in clause 9.11.3.53A of 3GPP
	/// TS 24.501  (octet 4), encoded as 2 hexadecimal characters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the \"UPU Header\" IE as specified in clause
	/// 9.11.3.53A of 3GPP TS 24.501  (octet 4), encoded as 2 hexadecimal
	/// characters.",
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
	pub struct UpuHeader(String);

	impl ::std::ops::Deref for UpuHeader {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<UpuHeader> for String {
		fn from(value: UpuHeader) -> Self {
			value.0
		}
	}

	impl From<&UpuHeader> for UpuHeader {
		fn from(value: &UpuHeader) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for UpuHeader {
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

	impl ::std::convert::TryFrom<&str> for UpuHeader {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for UpuHeader {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for UpuHeader {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for UpuHeader {
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

	/// Contains the UE parameters update Information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the UE parameters update Information.",
	///  "type": "object",
	///  "required": [
	///    "upuAckInd",
	///    "upuDataList"
	///  ],
	///  "properties": {
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "upuAckInd": {
	///      "$ref": "#/components/schemas/UpuAckInd"
	///    },
	///    "upuDataList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UpuData"
	///      },
	///      "minItems": 1
	///    },
	///    "upuHeader": {
	///      "$ref": "#/components/schemas/UpuHeader"
	///    },
	///    "upuTransparentInfo": {
	///      "$ref": "#/components/schemas/Bytes"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UpuInfo {
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "upuAckInd")]
		pub upu_ack_ind: UpuAckInd,
		#[serde(rename = "upuDataList")]
		pub upu_data_list: Vec<UpuData>,
		#[serde(rename = "upuHeader", default, skip_serializing_if = "Option::is_none")]
		pub upu_header: Option<UpuHeader>,
		#[serde(
			rename = "upuTransparentInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub upu_transparent_info: Option<Bytes>,
	}

	impl From<&UpuInfo> for UpuInfo {
		fn from(value: &UpuInfo) -> Self {
			value.clone()
		}
	}

	/// MAC value for protecting UPU procedure (UPU-MAC-IAUSF and UPU-MAC-IUE).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MAC value for protecting UPU procedure (UPU-MAC-IAUSF
	/// and UPU-MAC-IUE).",
	///  "type": "string",
	///  "pattern": "^[A-Fa-f0-9]{32}$"
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
	pub struct UpuMac(String);

	impl ::std::ops::Deref for UpuMac {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<UpuMac> for String {
		fn from(value: UpuMac) -> Self {
			value.0
		}
	}

	impl From<&UpuMac> for UpuMac {
		fn from(value: &UpuMac) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for UpuMac {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[A-Fa-f0-9]{32}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[A-Fa-f0-9]{32}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for UpuMac {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for UpuMac {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for UpuMac {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for UpuMac {
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

	/// Contains the material generated for securing of UPU. It contains at
	/// least the UPU-MAC-IAUSF and CounterUPU.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the material generated for securing of UPU. It
	/// contains at least the UPU-MAC-IAUSF and CounterUPU.",
	///  "type": "object",
	///  "required": [
	///    "counterUpu",
	///    "upuMacIausf"
	///  ],
	///  "properties": {
	///    "counterUpu": {
	///      "$ref": "#/components/schemas/CounterUpu"
	///    },
	///    "upuMacIausf": {
	///      "$ref": "#/components/schemas/UpuMac"
	///    },
	///    "upuXmacIue": {
	///      "$ref": "#/components/schemas/UpuMac"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UpuSecurityInfo {
		#[serde(rename = "counterUpu")]
		pub counter_upu: CounterUpu,
		#[serde(rename = "upuMacIausf")]
		pub upu_mac_iausf: UpuMac,
		#[serde(
			rename = "upuXmacIue",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub upu_xmac_iue: Option<UpuMac>,
	}

	impl From<&UpuSecurityInfo> for UpuSecurityInfo {
		fn from(value: &UpuSecurityInfo) -> Self {
			value.clone()
		}
	}

	/// UpuTransparentInfo
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
	pub struct UpuTransparentInfo(pub Bytes);

	impl ::std::ops::Deref for UpuTransparentInfo {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<UpuTransparentInfo> for Bytes {
		fn from(value: UpuTransparentInfo) -> Self {
			value.0
		}
	}

	impl From<&UpuTransparentInfo> for UpuTransparentInfo {
		fn from(value: &UpuTransparentInfo) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for UpuTransparentInfo {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for UpuTransparentInfo {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for UpuTransparentInfo {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UpuTransparentInfo {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UpuTransparentInfo {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for UpuTransparentInfo {
		fn to_string(&self) -> String {
			self.0.to_string()
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
				            \"^rid[0-9]{1,4}\\.pid[0-9a-fA-F]+\\@prose-cp\\.5gc\\.mnc[0-9]{2,3}\\\
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
