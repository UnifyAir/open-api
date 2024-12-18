#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};

#[allow(unused_imports)]
use crate::progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use crate::progenitor_client::{ByteStream, Error, ResponseValue};

/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
	
	/// Error types.
	pub use crate::common::*;
	pub use crate::modified::nrf::*;
	/// Represents the information relative to an AAnF NF Instance.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the information relative to an AAnF NF
	/// Instance.",
	///  "type": "object",
	///  "properties": {
	///    "routingIndicators": {
	///      "type": "array",
	///      "items": {
	///        "type": "string",
	///        "pattern": "^[0-9]{1,4}$"
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
	pub struct AanfInfo {
		#[serde(
			rename = "routingIndicators",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub routing_indicators: Vec<AanfInfoRoutingIndicatorsItem>,
	}

	impl From<&AanfInfo> for AanfInfo {
		fn from(value: &AanfInfo) -> Self {
			value.clone()
		}
	}

	/// AanfInfoRoutingIndicatorsItem
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
	pub struct AanfInfoRoutingIndicatorsItem(String);

	impl ::std::ops::Deref for AanfInfoRoutingIndicatorsItem {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AanfInfoRoutingIndicatorsItem> for String {
		fn from(value: AanfInfoRoutingIndicatorsItem) -> Self {
			value.0
		}
	}

	impl From<&AanfInfoRoutingIndicatorsItem> for AanfInfoRoutingIndicatorsItem {
		fn from(value: &AanfInfoRoutingIndicatorsItem) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AanfInfoRoutingIndicatorsItem {
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

	impl ::std::convert::TryFrom<&str> for AanfInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AanfInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AanfInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AanfInfoRoutingIndicatorsItem {
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

	/// The claims data structure for the access token
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The claims data structure for the access token",
	///  "type": "object",
	///  "required": [
	///    "aud",
	///    "exp",
	///    "iss",
	///    "scope",
	///    "sub"
	///  ],
	///  "properties": {
	///    "aud": {
	///      "anyOf": [
	///        {
	///          "$ref": "#/components/schemas/NFType"
	///        },
	///        {
	///          "type": "array",
	///          "items": {
	///            "$ref": "#/components/schemas/NfInstanceId"
	///          },
	///          "minItems": 1
	///        }
	///      ]
	///    },
	///    "consumerPlmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "consumerSnpnId": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "exp": {
	///      "type": "integer"
	///    },
	///    "iss": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "producerNfServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "producerNfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "producerNsiList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "producerPlmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "producerSnpnId": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "producerSnssaiList": {
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
	///    "sub": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AccessTokenClaims {
		pub aud: AccessTokenClaimsAud,
		#[serde(
			rename = "consumerPlmnId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub consumer_plmn_id: Option<PlmnId>,
		#[serde(
			rename = "consumerSnpnId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub consumer_snpn_id: Option<PlmnIdNid>,
		pub exp: i64,
		pub iss: NfInstanceId,
		#[serde(
			rename = "producerNfServiceSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub producer_nf_service_set_id: Option<NfServiceSetId>,
		#[serde(
			rename = "producerNfSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub producer_nf_set_id: Option<NfSetId>,
		#[serde(
			rename = "producerNsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub producer_nsi_list: Vec<String>,
		#[serde(
			rename = "producerPlmnId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub producer_plmn_id: Option<PlmnId>,
		#[serde(
			rename = "producerSnpnId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub producer_snpn_id: Option<PlmnIdNid>,
		#[serde(
			rename = "producerSnssaiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub producer_snssai_list: Vec<Snssai>,
		pub scope: AccessTokenClaimsScope,
		#[serde(
			rename = "sourceNfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_nf_instance_id: Option<NfInstanceId>,
		pub sub: NfInstanceId,
	}

	impl From<&AccessTokenClaims> for AccessTokenClaims {
		fn from(value: &AccessTokenClaims) -> Self {
			value.clone()
		}
	}

	/// AccessTokenClaimsAud
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/NFType"
	///    },
	///    {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfInstanceId"
	///      },
	///      "minItems": 1
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum AccessTokenClaimsAud {
		#[default]
		Variant0(NfType),
		Variant1(Vec<NfInstanceId>),
	}

	impl From<&AccessTokenClaimsAud> for AccessTokenClaimsAud {
		fn from(value: &AccessTokenClaimsAud) -> Self {
			value.clone()
		}
	}

	impl From<NfType> for AccessTokenClaimsAud {
		fn from(value: NfType) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<Vec<NfInstanceId>> for AccessTokenClaimsAud {
		fn from(value: Vec<NfInstanceId>) -> Self {
			Self::Variant1(value)
		}
	}

	/// AccessTokenClaimsScope
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
	pub struct AccessTokenClaimsScope(String);

	impl ::std::ops::Deref for AccessTokenClaimsScope {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AccessTokenClaimsScope> for String {
		fn from(value: AccessTokenClaimsScope) -> Self {
			value.0
		}
	}

	impl From<&AccessTokenClaimsScope> for AccessTokenClaimsScope {
		fn from(value: &AccessTokenClaimsScope) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AccessTokenClaimsScope {
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

	impl ::std::convert::TryFrom<&str> for AccessTokenClaimsScope {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AccessTokenClaimsScope {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AccessTokenClaimsScope {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AccessTokenClaimsScope {
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

	/// Contains information related to the access token response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains information related to the access token
	/// response",
	///  "type": "object",
	///  "required": [
	///    "access_token",
	///    "token_type"
	///  ],
	///  "properties": {
	///    "access_token": {
	///      "description": "JWS Compact Serialized representation of JWS signed
	/// JSON object (AccessTokenClaims)\n",
	///      "type": "string"
	///    },
	///    "expires_in": {
	///      "type": "integer"
	///    },
	///    "scope": {
	///      "type": "string",
	///      "pattern": "^([a-zA-Z0-9_:-]+)( [a-zA-Z0-9_:-]+)*$"
	///    },
	///    "token_type": {
	///      "type": "string",
	///      "enum": [
	///        "Bearer"
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AccessTokenRsp {
		/// JWS Compact Serialized representation of JWS signed JSON object
		/// (AccessTokenClaims)
		pub access_token: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expires_in: Option<i64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub scope: Option<AccessTokenRspScope>,
		pub token_type: AccessTokenRspTokenType,
	}

	impl From<&AccessTokenRsp> for AccessTokenRsp {
		fn from(value: &AccessTokenRsp) -> Self {
			value.clone()
		}
	}

	/// AccessTokenRspScope
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
	pub struct AccessTokenRspScope(String);

	impl ::std::ops::Deref for AccessTokenRspScope {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AccessTokenRspScope> for String {
		fn from(value: AccessTokenRspScope) -> Self {
			value.0
		}
	}

	impl From<&AccessTokenRspScope> for AccessTokenRspScope {
		fn from(value: &AccessTokenRspScope) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AccessTokenRspScope {
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

	impl ::std::convert::TryFrom<&str> for AccessTokenRspScope {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AccessTokenRspScope {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AccessTokenRspScope {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AccessTokenRspScope {
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

	/// AccessTokenRspTokenType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "Bearer"
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
	pub enum AccessTokenRspTokenType {
		#[default]
		Bearer,
	}

	impl From<&AccessTokenRspTokenType> for AccessTokenRspTokenType {
		fn from(value: &AccessTokenRspTokenType) -> Self {
			value.clone()
		}
	}

	impl ToString for AccessTokenRspTokenType {
		fn to_string(&self) -> String {
			match *self {
				Self::Bearer => "Bearer".to_string(),
			}
		}
	}

	impl std::str::FromStr for AccessTokenRspTokenType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"Bearer" => Ok(Self::Bearer),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AccessTokenRspTokenType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AccessTokenRspTokenType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AccessTokenRspTokenType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents Application Events.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents Application Events.",
	///  "type": "string",
	///  "enum": [
	///    "SVC_EXPERIENCE",
	///    "UE_MOBILITY",
	///    "UE_COMM",
	///    "EXCEPTIONS",
	///    "USER_DATA_CONGESTION",
	///    "PERF_DATA",
	///    "DISPERSION",
	///    "COLLECTIVE_BEHAVIOUR",
	///    "MS_QOE_METRICS",
	///    "MS_CONSUMPTION",
	///    "MS_NET_ASSIST_INVOCATION",
	///    "MS_DYN_POLICY_INVOCATION",
	///    "MS_ACCESS_ACTIVITY"
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
	pub enum AfEvent {
		#[default]
		#[serde(rename = "SVC_EXPERIENCE")]
		SvcExperience,
		#[serde(rename = "UE_MOBILITY")]
		UeMobility,
		#[serde(rename = "UE_COMM")]
		UeComm,
		#[serde(rename = "EXCEPTIONS")]
		Exceptions,
		#[serde(rename = "USER_DATA_CONGESTION")]
		UserDataCongestion,
		#[serde(rename = "PERF_DATA")]
		PerfData,
		#[serde(rename = "DISPERSION")]
		Dispersion,
		#[serde(rename = "COLLECTIVE_BEHAVIOUR")]
		CollectiveBehaviour,
		#[serde(rename = "MS_QOE_METRICS")]
		MsQoeMetrics,
		#[serde(rename = "MS_CONSUMPTION")]
		MsConsumption,
		#[serde(rename = "MS_NET_ASSIST_INVOCATION")]
		MsNetAssistInvocation,
		#[serde(rename = "MS_DYN_POLICY_INVOCATION")]
		MsDynPolicyInvocation,
		#[serde(rename = "MS_ACCESS_ACTIVITY")]
		MsAccessActivity,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AfEvent> for AfEvent {
		fn from(value: &AfEvent) -> Self {
			value.clone()
		}
	}

	impl ToString for AfEvent {
		fn to_string(&self) -> String {
			match *self {
				Self::SvcExperience => "SVC_EXPERIENCE".to_string(),
				Self::UeMobility => "UE_MOBILITY".to_string(),
				Self::UeComm => "UE_COMM".to_string(),
				Self::Exceptions => "EXCEPTIONS".to_string(),
				Self::UserDataCongestion => "USER_DATA_CONGESTION".to_string(),
				Self::PerfData => "PERF_DATA".to_string(),
				Self::Dispersion => "DISPERSION".to_string(),
				Self::CollectiveBehaviour => "COLLECTIVE_BEHAVIOUR".to_string(),
				Self::MsQoeMetrics => "MS_QOE_METRICS".to_string(),
				Self::MsConsumption => "MS_CONSUMPTION".to_string(),
				Self::MsNetAssistInvocation => "MS_NET_ASSIST_INVOCATION".to_string(),
				Self::MsDynPolicyInvocation => "MS_DYN_POLICY_INVOCATION".to_string(),
				Self::MsAccessActivity => "MS_ACCESS_ACTIVITY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AfEvent {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SVC_EXPERIENCE" => Ok(Self::SvcExperience),
				"UE_MOBILITY" => Ok(Self::UeMobility),
				"UE_COMM" => Ok(Self::UeComm),
				"EXCEPTIONS" => Ok(Self::Exceptions),
				"USER_DATA_CONGESTION" => Ok(Self::UserDataCongestion),
				"PERF_DATA" => Ok(Self::PerfData),
				"DISPERSION" => Ok(Self::Dispersion),
				"COLLECTIVE_BEHAVIOUR" => Ok(Self::CollectiveBehaviour),
				"MS_QOE_METRICS" => Ok(Self::MsQoeMetrics),
				"MS_CONSUMPTION" => Ok(Self::MsConsumption),
				"MS_NET_ASSIST_INVOCATION" => Ok(Self::MsNetAssistInvocation),
				"MS_DYN_POLICY_INVOCATION" => Ok(Self::MsDynPolicyInvocation),
				"MS_ACCESS_ACTIVITY" => Ok(Self::MsAccessActivity),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AfEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AfEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AfEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// AF Event Exposure data managed by a given NEF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "AF Event Exposure data managed by a given NEF
	/// Instance",
	///  "type": "object",
	///  "required": [
	///    "afEvents"
	///  ],
	///  "properties": {
	///    "afEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AfEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "afIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "appIds": {
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
	pub struct AfEventExposureData {
		#[serde(rename = "afEvents")]
		pub af_events: Vec<AfEvent>,
		#[serde(rename = "afIds", default, skip_serializing_if = "Vec::is_empty")]
		pub af_ids: Vec<String>,
		#[serde(rename = "appIds", default, skip_serializing_if = "Vec::is_empty")]
		pub app_ids: Vec<String>,
	}

	impl From<&AfEventExposureData> for AfEventExposureData {
		fn from(value: &AfEventExposureData) -> Self {
			value.clone()
		}
	}

	/// Subscription to a set of AMFs, based on AMF Set Id and/or AMF Region Id
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of AMFs, based on AMF Set Id
	/// and/or AMF Region Id",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "amfSetId"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "amfRegionId"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "amfRegionId": {
	///      "$ref": "#/components/schemas/AmfRegionId"
	///    },
	///    "amfSetId": {
	///      "$ref": "#/components/schemas/AmfSetId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum AmfCond {
		#[default]
		Variant0 {
			#[serde(rename = "amfSetId")]
			amf_set_id: AmfSetId,
		},
		Variant1 {
			#[serde(rename = "amfRegionId")]
			amf_region_id: AmfRegionId,
		},
	}

	impl From<&AmfCond> for AmfCond {
		fn from(value: &AmfCond) -> Self {
			value.clone()
		}
	}

	/// Information of an AMF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an AMF NF Instance",
	///  "type": "object",
	///  "required": [
	///    "amfRegionId",
	///    "amfSetId",
	///    "guamiList"
	///  ],
	///  "properties": {
	///    "amfOnboardingCapability": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "amfRegionId": {
	///      "$ref": "#/components/schemas/AmfRegionId"
	///    },
	///    "amfSetId": {
	///      "$ref": "#/components/schemas/AmfSetId"
	///    },
	///    "backupInfoAmfFailure": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Guami"
	///      },
	///      "minItems": 1
	///    },
	///    "backupInfoAmfRemoval": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Guami"
	///      },
	///      "minItems": 1
	///    },
	///    "guamiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Guami"
	///      },
	///      "minItems": 1
	///    },
	///    "highLatencyCom": {
	///      "type": "boolean"
	///    },
	///    "n2InterfaceAmfInfo": {
	///      "$ref": "#/components/schemas/N2InterfaceAmfInfo"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
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
	pub struct AmfInfo {
		#[serde(rename = "amfOnboardingCapability", default)]
		pub amf_onboarding_capability: bool,
		#[serde(rename = "amfRegionId")]
		pub amf_region_id: AmfRegionId,
		#[serde(rename = "amfSetId")]
		pub amf_set_id: AmfSetId,
		#[serde(
			rename = "backupInfoAmfFailure",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub backup_info_amf_failure: Vec<crate::common::common_models::Guami>,
		#[serde(
			rename = "backupInfoAmfRemoval",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub backup_info_amf_removal: Vec<crate::common::common_models::Guami>,
		#[serde(rename = "guamiList")]
		pub guami_list: Vec<crate::common::common_models::Guami>,
		#[serde(
			rename = "highLatencyCom",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub high_latency_com: Option<bool>,
		#[serde(
			rename = "n2InterfaceAmfInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_interface_amf_info: Option<N2InterfaceAmfInfo>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&AmfInfo> for AmfInfo {
		fn from(value: &AmfInfo) -> Self {
			value.clone()
		}
	}

	/// Access Network Node Type (gNB, ng-eNB...)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Access Network Node Type (gNB, ng-eNB...)",
	///  "type": "string",
	///  "enum": [
	///    "GNB",
	///    "NG_ENB"
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
	pub enum AnNodeType {
		#[default]
		#[serde(rename = "GNB")]
		Gnb,
		#[serde(rename = "NG_ENB")]
		NgEnb,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AnNodeType> for AnNodeType {
		fn from(value: &AnNodeType) -> Self {
			value.clone()
		}
	}

	impl ToString for AnNodeType {
		fn to_string(&self) -> String {
			match *self {
				Self::Gnb => "GNB".to_string(),
				Self::NgEnb => "NG_ENB".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AnNodeType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"GNB" => Ok(Self::Gnb),
				"NG_ENB" => Ok(Self::NgEnb),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AnNodeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AnNodeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AnNodeType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Information of an AUSF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an AUSF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "routingIndicators": {
	///      "type": "array",
	///      "items": {
	///        "type": "string",
	///        "pattern": "^[0-9]{1,4}$"
	///      },
	///      "minItems": 1
	///    },
	///    "suciInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SuciInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct AusfInfo {
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "routingIndicators",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub routing_indicators: Vec<AusfInfoRoutingIndicatorsItem>,
		#[serde(rename = "suciInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub suci_infos: Vec<SuciInfo>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&AusfInfo> for AusfInfo {
		fn from(value: &AusfInfo) -> Self {
			value.clone()
		}
	}

	/// AusfInfoRoutingIndicatorsItem
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
	pub struct AusfInfoRoutingIndicatorsItem(String);

	impl ::std::ops::Deref for AusfInfoRoutingIndicatorsItem {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AusfInfoRoutingIndicatorsItem> for String {
		fn from(value: AusfInfoRoutingIndicatorsItem) -> Self {
			value.0
		}
	}

	impl From<&AusfInfoRoutingIndicatorsItem> for AusfInfoRoutingIndicatorsItem {
		fn from(value: &AusfInfoRoutingIndicatorsItem) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AusfInfoRoutingIndicatorsItem {
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

	impl ::std::convert::TryFrom<&str> for AusfInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AusfInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AusfInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AusfInfoRoutingIndicatorsItem {
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

	/// Information returned by NRF in the bootstrapping response message
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information returned by NRF in the bootstrapping
	/// response message",
	///  "type": "object",
	///  "required": [
	///    "_links"
	///  ],
	///  "properties": {
	///    "_links": {
	///      "description": "Map of link objects where the keys are the link
	/// relations defined in 3GPP TS 29.510 clause 6.4.6.3.3\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/LinksValueSchema"
	///      }
	///    },
	///    "nrfFeatures": {
	///      "description": "Map of features supported by the NRF, where the
	/// keys are the NRF services as defined in 3GPP TS 29.510 clause
	/// 6.1.6.3.11\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SupportedFeatures"
	///      }
	///    },
	///    "nrfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nrfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "oauth2Required": {
	///      "description": "Map indicating whether the NRF requires
	/// Oauth2-based authorization for accessing its services. The key of the
	/// map shall be the name of an NRF service, e.g. \"nnrf-nfm\" or
	/// \"nnrf-disc\"\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "boolean"
	///      }
	///    },
	///    "status": {
	///      "$ref": "#/components/schemas/Status"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BootstrappingInfo {
		/// Map of link objects where the keys are the link relations defined in
		/// 3GPP TS 29.510 clause 6.4.6.3.3
		#[serde(rename = "_links")]
		pub links: ::std::collections::HashMap<String, LinksValueSchema>,
		/// Map of features supported by the NRF, where the keys are the NRF
		/// services as defined in 3GPP TS 29.510 clause 6.1.6.3.11
		#[serde(
			rename = "nrfFeatures",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub nrf_features: ::std::collections::HashMap<String, SupportedFeatures>,
		#[serde(
			rename = "nrfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_instance_id: Option<NfInstanceId>,
		#[serde(rename = "nrfSetId", default, skip_serializing_if = "Option::is_none")]
		pub nrf_set_id: Option<NfSetId>,
		/// Map indicating whether the NRF requires Oauth2-based authorization
		/// for accessing its services. The key of the map shall be the name of
		/// an NRF service, e.g. "nnrf-nfm" or "nnrf-disc"
		#[serde(
			rename = "oauth2Required",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub oauth2_required: ::std::collections::HashMap<String, bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub status: Option<Status>,
	}

	impl From<&BootstrappingInfo> for BootstrappingInfo {
		fn from(value: &BootstrappingInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a BSF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a BSF NF Instance",
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
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "ipDomainList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv4AddressRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4AddressRange"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6PrefixRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6PrefixRange"
	///      },
	///      "minItems": 1
	///    },
	///    "rxDiamHost": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "rxDiamRealm": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct BsfInfo {
		#[serde(rename = "dnnList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnn_list: Vec<Dnn>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "ipDomainList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ip_domain_list: Vec<String>,
		#[serde(
			rename = "ipv4AddressRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_address_ranges: Vec<Ipv4AddressRange>,
		#[serde(
			rename = "ipv6PrefixRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_prefix_ranges: Vec<Ipv6PrefixRange>,
		#[serde(
			rename = "rxDiamHost",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rx_diam_host: Option<Fqdn>,
		#[serde(
			rename = "rxDiamRealm",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rx_diam_realm: Option<Fqdn>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&BsfInfo> for BsfInfo {
		fn from(value: &BsfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a CHF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a CHF NF Instance",
	///  "type": "object",
	///  "not": {
	///    "required": [
	///      "primaryChfInstance",
	///      "secondaryChfInstance"
	///    ]
	///  },
	///  "properties": {
	///    "gpsiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
	///      },
	///      "minItems": 1
	///    },
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "plmnRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnRange"
	///      },
	///      "minItems": 1
	///    },
	///    "primaryChfInstance": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "secondaryChfInstance": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "supiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct ChfInfo {
		#[serde(
			rename = "gpsiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub gpsi_range_list: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "plmnRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub plmn_range_list: Vec<PlmnRange>,
		#[serde(
			rename = "supiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub supi_range_list: Vec<SupiRange>,
	}

	impl From<&ChfInfo> for ChfInfo {
		fn from(value: &ChfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an collocated NF Instance registered in the NRF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an collocated NF Instance registered in
	/// the NRF",
	///  "type": "object",
	///  "required": [
	///    "nfInstanceId",
	///    "nfType"
	///  ],
	///  "properties": {
	///    "nfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nfType": {
	///      "$ref": "#/components/schemas/CollocatedNfType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CollocatedNfInstance {
		#[serde(rename = "nfInstanceId")]
		pub nf_instance_id: NfInstanceId,
		#[serde(rename = "nfType")]
		pub nf_type: CollocatedNfType,
	}

	impl From<&CollocatedNfInstance> for CollocatedNfInstance {
		fn from(value: &CollocatedNfInstance) -> Self {
			value.clone()
		}
	}

	/// NF types for a collocated NF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "NF types for a collocated NF",
	///  "type": "string",
	///  "enum": [
	///    "UPF",
	///    "SMF",
	///    "MB_UPF",
	///    "MB_SMF"
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
	pub enum CollocatedNfType {
		#[default]
		#[serde(rename = "UPF")]
		Upf,
		#[serde(rename = "SMF")]
		Smf,
		#[serde(rename = "MB_UPF")]
		MbUpf,
		#[serde(rename = "MB_SMF")]
		MbSmf,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&CollocatedNfType> for CollocatedNfType {
		fn from(value: &CollocatedNfType) -> Self {
			value.clone()
		}
	}

	impl ToString for CollocatedNfType {
		fn to_string(&self) -> String {
			match *self {
				Self::Upf => "UPF".to_string(),
				Self::Smf => "SMF".to_string(),
				Self::MbUpf => "MB_UPF".to_string(),
				Self::MbSmf => "MB_SMF".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for CollocatedNfType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UPF" => Ok(Self::Upf),
				"SMF" => Ok(Self::Smf),
				"MB_UPF" => Ok(Self::MbUpf),
				"MB_SMF" => Ok(Self::MbSmf),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for CollocatedNfType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for CollocatedNfType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for CollocatedNfType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates whether a notification is due to the NF Instance to start or
	/// stop being part of a condition for a subscription to a set of NFs
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates whether a notification is due to the NF
	/// Instance to start or stop being part of a condition for a subscription
	/// to a set of NFs\n",
	///  "type": "string",
	///  "enum": [
	///    "NF_ADDED",
	///    "NF_REMOVED"
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
	pub enum ConditionEventType {
		#[default]
		#[serde(rename = "NF_ADDED")]
		NfAdded,
		#[serde(rename = "NF_REMOVED")]
		NfRemoved,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ConditionEventType> for ConditionEventType {
		fn from(value: &ConditionEventType) -> Self {
			value.clone()
		}
	}

	impl ToString for ConditionEventType {
		fn to_string(&self) -> String {
			match *self {
				Self::NfAdded => "NF_ADDED".to_string(),
				Self::NfRemoved => "NF_REMOVED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ConditionEventType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NF_ADDED" => Ok(Self::NfAdded),
				"NF_REMOVED" => Ok(Self::NfRemoved),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ConditionEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ConditionEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ConditionEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Types of data sets and subsets stored in UDR
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Types of data sets and subsets stored in UDR",
	///  "type": "string",
	///  "enum": [
	///    "SUBSCRIPTION",
	///    "POLICY",
	///    "EXPOSURE",
	///    "APPLICATION",
	///    "A_PFD",
	///    "A_AFTI",
	///    "A_IPTV",
	///    "A_BDT",
	///    "A_SPD",
	///    "A_EASD",
	///    "A_AMI",
	///    "P_UE",
	///    "P_SCD",
	///    "P_BDT",
	///    "P_PLMNUE",
	///    "P_NSSCD",
	///    "P_MBSCD"
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
	pub enum DataSetId {
		#[default]
		#[serde(rename = "SUBSCRIPTION")]
		Subscription,
		#[serde(rename = "POLICY")]
		Policy,
		#[serde(rename = "EXPOSURE")]
		Exposure,
		#[serde(rename = "APPLICATION")]
		Application,
		#[serde(rename = "A_PFD")]
		APfd,
		#[serde(rename = "A_AFTI")]
		AAfti,
		#[serde(rename = "A_IPTV")]
		AIptv,
		#[serde(rename = "A_BDT")]
		ABdt,
		#[serde(rename = "A_SPD")]
		ASpd,
		#[serde(rename = "A_EASD")]
		AEasd,
		#[serde(rename = "A_AMI")]
		AAmi,
		#[serde(rename = "P_UE")]
		PUe,
		#[serde(rename = "P_SCD")]
		PScd,
		#[serde(rename = "P_BDT")]
		PBdt,
		#[serde(rename = "P_PLMNUE")]
		PPlmnue,
		#[serde(rename = "P_NSSCD")]
		PNsscd,
		#[serde(rename = "P_MBSCD")]
		PMbscd,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DataSetId> for DataSetId {
		fn from(value: &DataSetId) -> Self {
			value.clone()
		}
	}

	impl ToString for DataSetId {
		fn to_string(&self) -> String {
			match *self {
				Self::Subscription => "SUBSCRIPTION".to_string(),
				Self::Policy => "POLICY".to_string(),
				Self::Exposure => "EXPOSURE".to_string(),
				Self::Application => "APPLICATION".to_string(),
				Self::APfd => "A_PFD".to_string(),
				Self::AAfti => "A_AFTI".to_string(),
				Self::AIptv => "A_IPTV".to_string(),
				Self::ABdt => "A_BDT".to_string(),
				Self::ASpd => "A_SPD".to_string(),
				Self::AEasd => "A_EASD".to_string(),
				Self::AAmi => "A_AMI".to_string(),
				Self::PUe => "P_UE".to_string(),
				Self::PScd => "P_SCD".to_string(),
				Self::PBdt => "P_BDT".to_string(),
				Self::PPlmnue => "P_PLMNUE".to_string(),
				Self::PNsscd => "P_NSSCD".to_string(),
				Self::PMbscd => "P_MBSCD".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DataSetId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SUBSCRIPTION" => Ok(Self::Subscription),
				"POLICY" => Ok(Self::Policy),
				"EXPOSURE" => Ok(Self::Exposure),
				"APPLICATION" => Ok(Self::Application),
				"A_PFD" => Ok(Self::APfd),
				"A_AFTI" => Ok(Self::AAfti),
				"A_IPTV" => Ok(Self::AIptv),
				"A_BDT" => Ok(Self::ABdt),
				"A_SPD" => Ok(Self::ASpd),
				"A_EASD" => Ok(Self::AEasd),
				"A_AMI" => Ok(Self::AAmi),
				"P_UE" => Ok(Self::PUe),
				"P_SCD" => Ok(Self::PScd),
				"P_BDT" => Ok(Self::PBdt),
				"P_PLMNUE" => Ok(Self::PPlmnue),
				"P_NSSCD" => Ok(Self::PNsscd),
				"P_MBSCD" => Ok(Self::PMbscd),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DataSetId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DataSetId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DataSetId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Subscription to a set of NF Instances (DCCFs), identified by NF types,
	/// NF Set Id(s) or DCCF Serving Area information, i.e. list of TAIs served
	/// by the DCCF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NF Instances (DCCFs),
	/// identified by NF types, NF Set Id(s) or DCCF Serving Area information,
	/// i.e. list of TAIs served by the DCCF\n",
	///  "type": "object",
	///  "required": [
	///    "conditionType"
	///  ],
	///  "properties": {
	///    "conditionType": {
	///      "type": "string",
	///      "enum": [
	///        "DCCF_COND"
	///      ]
	///    },
	///    "servingNfSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFType"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
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
	pub struct DccfCond {
		#[serde(rename = "conditionType")]
		pub condition_type: DccfCondConditionType,
		#[serde(
			rename = "servingNfSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_set_id_list: Vec<NfSetId>,
		#[serde(
			rename = "servingNfTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_type_list: Vec<NfType>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&DccfCond> for DccfCond {
		fn from(value: &DccfCond) -> Self {
			value.clone()
		}
	}

	/// DccfCondConditionType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "DCCF_COND"
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
	pub enum DccfCondConditionType {
		#[default]
		#[serde(rename = "DCCF_COND")]
		DccfCond,
	}

	impl From<&DccfCondConditionType> for DccfCondConditionType {
		fn from(value: &DccfCondConditionType) -> Self {
			value.clone()
		}
	}

	impl ToString for DccfCondConditionType {
		fn to_string(&self) -> String {
			match *self {
				Self::DccfCond => "DCCF_COND".to_string(),
			}
		}
	}

	impl std::str::FromStr for DccfCondConditionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DCCF_COND" => Ok(Self::DccfCond),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DccfCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DccfCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DccfCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Information of a DCCF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a DCCF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "servingNfSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFType"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
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
	pub struct DccfInfo {
		#[serde(
			rename = "servingNfSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_set_id_list: Vec<NfSetId>,
		#[serde(
			rename = "servingNfTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_type_list: Vec<NfType>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&DccfInfo> for DccfInfo {
		fn from(value: &DccfInfo) -> Self {
			value.clone()
		}
	}

	/// Service Specific information for Default Notification Subscription.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Service Specific information for Default Notification
	/// Subscription.",
	///  "type": "object",
	///  "properties": {
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "versions": {
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
	pub struct DefSubServiceInfo {
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub versions: Vec<String>,
	}

	impl From<&DefSubServiceInfo> for DefSubServiceInfo {
		fn from(value: &DefSubServiceInfo) -> Self {
			value.clone()
		}
	}

	/// Data structure for specifying the notifications the NF service
	/// subscribes by default, along with callback URI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data structure for specifying the notifications the NF
	/// service subscribes by default, along with callback URI\n",
	///  "type": "object",
	///  "required": [
	///    "callbackUri",
	///    "notificationType"
	///  ],
	///  "properties": {
	///    "acceptedEncoding": {
	///      "type": "string"
	///    },
	///    "binding": {
	///      "type": "string"
	///    },
	///    "callbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "interPlmnCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "n1MessageClass": {
	///      "$ref": "#/components/schemas/N1MessageClass"
	///    },
	///    "n2InformationClass": {
	///      "$ref": "#/components/schemas/N2InformationClass"
	///    },
	///    "notificationType": {
	///      "$ref": "#/components/schemas/NotificationType"
	///    },
	///    "serviceInfoList": {
	///      "description": "A map of service specific information. The name of
	/// the corresponding service (as specified in ServiceName data type) is the
	/// key.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/DefSubServiceInfo"
	///      }
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "versions": {
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
	pub struct DefaultNotificationSubscription {
		#[serde(
			rename = "acceptedEncoding",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub accepted_encoding: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub binding: Option<String>,
		#[serde(rename = "callbackUri")]
		pub callback_uri: Uri,
		#[serde(
			rename = "interPlmnCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_callback_uri: Option<Uri>,
		#[serde(
			rename = "n1MessageClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_message_class: Option<N1MessageClass>,
		#[serde(
			rename = "n2InformationClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_information_class: Option<N2InformationClass>,
		#[serde(rename = "notificationType")]
		pub notification_type: NotificationType,
		/// A map of service specific information. The name of the corresponding
		/// service (as specified in ServiceName data type) is the key.
		#[serde(
			rename = "serviceInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub service_info_list: ::std::collections::HashMap<String, DefSubServiceInfo>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub versions: Vec<String>,
	}

	impl From<&DefaultNotificationSubscription> for DefaultNotificationSubscription {
		fn from(value: &DefaultNotificationSubscription) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by EASDF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by EASDF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "dnn": {
	///      "anyOf": [
	///        {
	///          "$ref": "#/components/schemas/Dnn"
	///        },
	///        {
	///          "$ref": "#/components/schemas/WildcardDnn"
	///        }
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnEasdfInfoItem {
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<Dnai>,
		pub dnn: DnnEasdfInfoItemDnn,
	}

	impl From<&DnnEasdfInfoItem> for DnnEasdfInfoItem {
		fn from(value: &DnnEasdfInfoItem) -> Self {
			value.clone()
		}
	}

	/// DnnEasdfInfoItemDnn
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnn"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnEasdfInfoItemDnn {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&DnnEasdfInfoItemDnn> for DnnEasdfInfoItemDnn {
		fn from(value: &DnnEasdfInfoItemDnn) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by NF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by NF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "anyOf": [
	///        {
	///          "$ref": "#/components/schemas/Dnn"
	///        },
	///        {
	///          "$ref": "#/components/schemas/WildcardDnn"
	///        }
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnInfoItem {
		pub dnn: DnnInfoItemDnn,
	}

	impl From<&DnnInfoItem> for DnnInfoItem {
		fn from(value: &DnnInfoItem) -> Self {
			value.clone()
		}
	}

	/// DnnInfoItemDnn
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnn"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnInfoItemDnn {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&DnnInfoItemDnn> for DnnInfoItemDnn {
		fn from(value: &DnnInfoItemDnn) -> Self {
			value.clone()
		}
	}

	/// Parameters supported by an MB-SMF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Parameters supported by an MB-SMF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "anyOf": [
	///        {
	///          "$ref": "#/components/schemas/Dnn"
	///        },
	///        {
	///          "$ref": "#/components/schemas/WildcardDnn"
	///        }
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnMbSmfInfoItem {
		pub dnn: DnnMbSmfInfoItemDnn,
	}

	impl From<&DnnMbSmfInfoItem> for DnnMbSmfInfoItem {
		fn from(value: &DnnMbSmfInfoItem) -> Self {
			value.clone()
		}
	}

	/// DnnMbSmfInfoItemDnn
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnn"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnMbSmfInfoItemDnn {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&DnnMbSmfInfoItemDnn> for DnnMbSmfInfoItemDnn {
		fn from(value: &DnnMbSmfInfoItemDnn) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by SMF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by SMF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/Dnai"
	///          },
	///          {
	///            "$ref": "#/components/schemas/WildcardDnai"
	///          }
	///        ]
	///      },
	///      "minItems": 1
	///    },
	///    "dnn": {
	///      "anyOf": [
	///        {
	///          "$ref": "#/components/schemas/Dnn"
	///        },
	///        {
	///          "$ref": "#/components/schemas/WildcardDnn"
	///        }
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnSmfInfoItem {
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<DnnSmfInfoItemDnaiListItem>,
		pub dnn: DnnSmfInfoItemDnn,
	}

	impl From<&DnnSmfInfoItem> for DnnSmfInfoItem {
		fn from(value: &DnnSmfInfoItem) -> Self {
			value.clone()
		}
	}

	/// DnnSmfInfoItemDnaiListItem
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnai"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnai"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnSmfInfoItemDnaiListItem {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnai>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnai>,
	}

	impl From<&DnnSmfInfoItemDnaiListItem> for DnnSmfInfoItemDnaiListItem {
		fn from(value: &DnnSmfInfoItemDnaiListItem) -> Self {
			value.clone()
		}
	}

	/// DnnSmfInfoItemDnn
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnn"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnSmfInfoItemDnn {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&DnnSmfInfoItemDnn> for DnnSmfInfoItemDnn {
		fn from(value: &DnnSmfInfoItemDnn) -> Self {
			value.clone()
		}
	}

	/// Parameters supported by an TSCTSF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Parameters supported by an TSCTSF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "anyOf": [
	///        {
	///          "$ref": "#/components/schemas/Dnn"
	///        },
	///        {
	///          "$ref": "#/components/schemas/WildcardDnn"
	///        }
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnTsctsfInfoItem {
		pub dnn: DnnTsctsfInfoItemDnn,
	}

	impl From<&DnnTsctsfInfoItem> for DnnTsctsfInfoItem {
		fn from(value: &DnnTsctsfInfoItem) -> Self {
			value.clone()
		}
	}

	/// DnnTsctsfInfoItemDnn
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnn"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DnnTsctsfInfoItemDnn {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&DnnTsctsfInfoItemDnn> for DnnTsctsfInfoItemDnn {
		fn from(value: &DnnTsctsfInfoItemDnn) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by UPF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by UPF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "dnaiNwInstanceList": {
	///      "description": "Map of network instance per DNAI for the DNN, where
	/// the key of the map is the DNAI. When present, the value of each entry of
	/// the map shall contain a N6 network instance that is configured for the
	/// DNAI indicated by the key.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "string"
	///      }
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "ipv4AddressRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4AddressRange"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv4IndexList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpIndex"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6IndexList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpIndex"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6PrefixRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6PrefixRange"
	///      },
	///      "minItems": 1
	///    },
	///    "pduSessionTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionType"
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
	pub struct DnnUpfInfoItem {
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<Dnai>,
		/// Map of network instance per DNAI for the DNN, where the key of the
		/// map is the DNAI. When present, the value of each entry of the map
		/// shall contain a N6 network instance that is configured for the DNAI
		/// indicated by the key.
		#[serde(
			rename = "dnaiNwInstanceList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub dnai_nw_instance_list: ::std::collections::HashMap<String, String>,
		pub dnn: Dnn,
		#[serde(
			rename = "ipv4AddressRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_address_ranges: Vec<Ipv4AddressRange>,
		#[serde(
			rename = "ipv4IndexList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_index_list: Vec<IpIndex>,
		#[serde(
			rename = "ipv6IndexList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_index_list: Vec<IpIndex>,
		#[serde(
			rename = "ipv6PrefixRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_prefix_ranges: Vec<Ipv6PrefixRange>,
		#[serde(
			rename = "pduSessionTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pdu_session_types: Vec<PduSessionType>,
	}

	impl From<&DnnUpfInfoItem> for DnnUpfInfoItem {
		fn from(value: &DnnUpfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Information of an EASDF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an EASDF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "easdfN6IpAddressList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpAddr"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssaiEasdfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SnssaiEasdfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "upfN6IpAddressList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpAddr"
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
	pub struct EasdfInfo {
		#[serde(
			rename = "easdfN6IpAddressList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub easdf_n6_ip_address_list: Vec<IpAddr>,
		#[serde(
			rename = "sNssaiEasdfInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub s_nssai_easdf_info_list: Vec<SnssaiEasdfInfoItem>,
		#[serde(
			rename = "upfN6IpAddressList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub upf_n6_ip_address_list: Vec<IpAddr>,
	}

	impl From<&EasdfInfo> for EasdfInfo {
		fn from(value: &EasdfInfo) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - LOAD_LEVEL_INFORMATION: Represent the analytics of load level
	///   information of corresponding network slice.
	/// - NETWORK_PERFORMANCE: Represent the analytics of network performance
	///   information.
	/// - NF_LOAD: Indicates that the event subscribed is NF Load.
	/// - SERVICE_EXPERIENCE: Represent the analytics of service experience
	///   information of the specific applications.
	/// - UE_MOBILITY: Represent the analytics of UE mobility.
	/// - UE_COMMUNICATION: Represent the analytics of UE communication.
	/// - QOS_SUSTAINABILITY: Represent the analytics of QoS sustainability
	///   information in the certain area.
	/// - ABNORMAL_BEHAVIOUR: Indicates that the event subscribed is abnormal
	///   behaviour information.
	/// - USER_DATA_CONGESTION: Represent the analytics of the user data
	///   congestion in the certain area.
	/// - NSI_LOAD_LEVEL: Represent the analytics of Network Slice and the
	///   optionally associated Network Slice Instance.
	/// - SM_CONGESTION: Represent the analytics of Session Management
	///   congestion control experience information for specific DNN and/or
	///   S-NSSAI.
	/// - DISPERSION: Represents the analytics of dispersion.
	/// - RED_TRANS_EXP: Represents the analytics of Redundant Transmission
	///   Experience.
	/// - WLAN_PERFORMANCE: Represents the analytics of WLAN performance.
	/// - DN_PERFORMANCE: Represents the analytics of DN performance.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- LOAD_LEVEL_INFORMATION:
	/// Represent the analytics of load level information of corresponding
	/// network slice.\n- NETWORK_PERFORMANCE: Represent the analytics of
	/// network performance information.\n- NF_LOAD: Indicates that the event
	/// subscribed is NF Load.\n- SERVICE_EXPERIENCE: Represent the analytics of
	/// service experience information of the specific applications.\n-
	/// UE_MOBILITY: Represent the analytics of UE mobility.\n-
	/// UE_COMMUNICATION: Represent the analytics of UE communication.\n-
	/// QOS_SUSTAINABILITY: Represent the analytics of QoS sustainability
	/// information in the certain area.\n- ABNORMAL_BEHAVIOUR: Indicates that
	/// the event subscribed is abnormal behaviour information.\n-
	/// USER_DATA_CONGESTION: Represent the analytics of the user data
	/// congestion in the certain area.\n- NSI_LOAD_LEVEL: Represent the
	/// analytics of Network Slice and the optionally associated Network Slice
	/// Instance.\n- SM_CONGESTION: Represent the analytics of Session
	/// Management congestion control experience information for specific DNN
	/// and/or S-NSSAI.\n- DISPERSION: Represents the analytics of
	/// dispersion.\n- RED_TRANS_EXP: Represents the analytics of Redundant
	/// Transmission Experience.\n- WLAN_PERFORMANCE: Represents the analytics
	/// of WLAN performance.\n- DN_PERFORMANCE: Represents the analytics of DN
	/// performance.\n",
	///  "type": "string",
	///  "enum": [
	///    "LOAD_LEVEL_INFORMATION",
	///    "NETWORK_PERFORMANCE",
	///    "NF_LOAD",
	///    "SERVICE_EXPERIENCE",
	///    "UE_MOBILITY",
	///    "UE_COMMUNICATION",
	///    "QOS_SUSTAINABILITY",
	///    "ABNORMAL_BEHAVIOUR",
	///    "USER_DATA_CONGESTION",
	///    "NSI_LOAD_LEVEL",
	///    "SM_CONGESTION",
	///    "DISPERSION",
	///    "RED_TRANS_EXP",
	///    "WLAN_PERFORMANCE",
	///    "DN_PERFORMANCE"
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
	pub enum EventId {
		#[default]
		#[serde(rename = "LOAD_LEVEL_INFORMATION")]
		LoadLevelInformation,
		#[serde(rename = "NETWORK_PERFORMANCE")]
		NetworkPerformance,
		#[serde(rename = "NF_LOAD")]
		NfLoad,
		#[serde(rename = "SERVICE_EXPERIENCE")]
		ServiceExperience,
		#[serde(rename = "UE_MOBILITY")]
		UeMobility,
		#[serde(rename = "UE_COMMUNICATION")]
		UeCommunication,
		#[serde(rename = "QOS_SUSTAINABILITY")]
		QosSustainability,
		#[serde(rename = "ABNORMAL_BEHAVIOUR")]
		AbnormalBehaviour,
		#[serde(rename = "USER_DATA_CONGESTION")]
		UserDataCongestion,
		#[serde(rename = "NSI_LOAD_LEVEL")]
		NsiLoadLevel,
		#[serde(rename = "SM_CONGESTION")]
		SmCongestion,
		#[serde(rename = "DISPERSION")]
		Dispersion,
		#[serde(rename = "RED_TRANS_EXP")]
		RedTransExp,
		#[serde(rename = "WLAN_PERFORMANCE")]
		WlanPerformance,
		#[serde(rename = "DN_PERFORMANCE")]
		DnPerformance,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&EventId> for EventId {
		fn from(value: &EventId) -> Self {
			value.clone()
		}
	}

	impl ToString for EventId {
		fn to_string(&self) -> String {
			match *self {
				Self::LoadLevelInformation => "LOAD_LEVEL_INFORMATION".to_string(),
				Self::NetworkPerformance => "NETWORK_PERFORMANCE".to_string(),
				Self::NfLoad => "NF_LOAD".to_string(),
				Self::ServiceExperience => "SERVICE_EXPERIENCE".to_string(),
				Self::UeMobility => "UE_MOBILITY".to_string(),
				Self::UeCommunication => "UE_COMMUNICATION".to_string(),
				Self::QosSustainability => "QOS_SUSTAINABILITY".to_string(),
				Self::AbnormalBehaviour => "ABNORMAL_BEHAVIOUR".to_string(),
				Self::UserDataCongestion => "USER_DATA_CONGESTION".to_string(),
				Self::NsiLoadLevel => "NSI_LOAD_LEVEL".to_string(),
				Self::SmCongestion => "SM_CONGESTION".to_string(),
				Self::Dispersion => "DISPERSION".to_string(),
				Self::RedTransExp => "RED_TRANS_EXP".to_string(),
				Self::WlanPerformance => "WLAN_PERFORMANCE".to_string(),
				Self::DnPerformance => "DN_PERFORMANCE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for EventId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOAD_LEVEL_INFORMATION" => Ok(Self::LoadLevelInformation),
				"NETWORK_PERFORMANCE" => Ok(Self::NetworkPerformance),
				"NF_LOAD" => Ok(Self::NfLoad),
				"SERVICE_EXPERIENCE" => Ok(Self::ServiceExperience),
				"UE_MOBILITY" => Ok(Self::UeMobility),
				"UE_COMMUNICATION" => Ok(Self::UeCommunication),
				"QOS_SUSTAINABILITY" => Ok(Self::QosSustainability),
				"ABNORMAL_BEHAVIOUR" => Ok(Self::AbnormalBehaviour),
				"USER_DATA_CONGESTION" => Ok(Self::UserDataCongestion),
				"NSI_LOAD_LEVEL" => Ok(Self::NsiLoadLevel),
				"SM_CONGESTION" => Ok(Self::SmCongestion),
				"DISPERSION" => Ok(Self::Dispersion),
				"RED_TRANS_EXP" => Ok(Self::RedTransExp),
				"WLAN_PERFORMANCE" => Ok(Self::WlanPerformance),
				"DN_PERFORMANCE" => Ok(Self::DnPerformance),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for EventId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for EventId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for EventId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// ExtGroupId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub struct ExtGroupId(String);

	impl ::std::ops::Deref for ExtGroupId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ExtGroupId> for String {
		fn from(value: ExtGroupId) -> Self {
			value.0
		}
	}

	impl From<&ExtGroupId> for ExtGroupId {
		fn from(value: &ExtGroupId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ExtGroupId {
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

	impl ::std::convert::TryFrom<&str> for ExtGroupId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ExtGroupId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ExtGroupId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ExtGroupId {
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

	/// Indicates types of External Clients.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates types of External Clients.",
	///  "type": "string",
	///  "enum": [
	///    "EMERGENCY_SERVICES",
	///    "VALUE_ADDED_SERVICES",
	///    "PLMN_OPERATOR_SERVICES",
	///    "LAWFUL_INTERCEPT_SERVICES",
	///    "PLMN_OPERATOR_BROADCAST_SERVICES",
	///    "PLMN_OPERATOR_OM",
	///    "PLMN_OPERATOR_ANONYMOUS_STATISTICS",
	///    "PLMN_OPERATOR_TARGET_MS_SERVICE_SUPPORT"
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
	pub enum ExternalClientType {
		#[default]
		#[serde(rename = "EMERGENCY_SERVICES")]
		EmergencyServices,
		#[serde(rename = "VALUE_ADDED_SERVICES")]
		ValueAddedServices,
		#[serde(rename = "PLMN_OPERATOR_SERVICES")]
		PlmnOperatorServices,
		#[serde(rename = "LAWFUL_INTERCEPT_SERVICES")]
		LawfulInterceptServices,
		#[serde(rename = "PLMN_OPERATOR_BROADCAST_SERVICES")]
		PlmnOperatorBroadcastServices,
		#[serde(rename = "PLMN_OPERATOR_OM")]
		PlmnOperatorOm,
		#[serde(rename = "PLMN_OPERATOR_ANONYMOUS_STATISTICS")]
		PlmnOperatorAnonymousStatistics,
		#[serde(rename = "PLMN_OPERATOR_TARGET_MS_SERVICE_SUPPORT")]
		PlmnOperatorTargetMsServiceSupport,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ExternalClientType> for ExternalClientType {
		fn from(value: &ExternalClientType) -> Self {
			value.clone()
		}
	}

	impl ToString for ExternalClientType {
		fn to_string(&self) -> String {
			match *self {
				Self::EmergencyServices => "EMERGENCY_SERVICES".to_string(),
				Self::ValueAddedServices => "VALUE_ADDED_SERVICES".to_string(),
				Self::PlmnOperatorServices => "PLMN_OPERATOR_SERVICES".to_string(),
				Self::LawfulInterceptServices => "LAWFUL_INTERCEPT_SERVICES".to_string(),
				Self::PlmnOperatorBroadcastServices => {
					"PLMN_OPERATOR_BROADCAST_SERVICES".to_string()
				}
				Self::PlmnOperatorOm => "PLMN_OPERATOR_OM".to_string(),
				Self::PlmnOperatorAnonymousStatistics => {
					"PLMN_OPERATOR_ANONYMOUS_STATISTICS".to_string()
				}
				Self::PlmnOperatorTargetMsServiceSupport => {
					"PLMN_OPERATOR_TARGET_MS_SERVICE_SUPPORT".to_string()
				}
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ExternalClientType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EMERGENCY_SERVICES" => Ok(Self::EmergencyServices),
				"VALUE_ADDED_SERVICES" => Ok(Self::ValueAddedServices),
				"PLMN_OPERATOR_SERVICES" => Ok(Self::PlmnOperatorServices),
				"LAWFUL_INTERCEPT_SERVICES" => Ok(Self::LawfulInterceptServices),
				"PLMN_OPERATOR_BROADCAST_SERVICES" => Ok(Self::PlmnOperatorBroadcastServices),
				"PLMN_OPERATOR_OM" => Ok(Self::PlmnOperatorOm),
				"PLMN_OPERATOR_ANONYMOUS_STATISTICS" => Ok(Self::PlmnOperatorAnonymousStatistics),
				"PLMN_OPERATOR_TARGET_MS_SERVICE_SUPPORT" => {
					Ok(Self::PlmnOperatorTargetMsServiceSupport)
				}
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ExternalClientType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ExternalClientType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ExternalClientType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Information of a GMLC NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a GMLC NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "gmlcNumbers": {
	///      "type": "array",
	///      "items": {
	///        "type": "string",
	///        "pattern": "^[0-9]{5,15}$"
	///      },
	///      "minItems": 1
	///    },
	///    "servingClientTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExternalClientType"
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
	pub struct GmlcInfo {
		#[serde(rename = "gmlcNumbers", default, skip_serializing_if = "Vec::is_empty")]
		pub gmlc_numbers: Vec<GmlcInfoGmlcNumbersItem>,
		#[serde(
			rename = "servingClientTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_client_types: Vec<ExternalClientType>,
	}

	impl From<&GmlcInfo> for GmlcInfo {
		fn from(value: &GmlcInfo) -> Self {
			value.clone()
		}
	}

	/// GmlcInfoGmlcNumbersItem
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	)]
	pub struct GmlcInfoGmlcNumbersItem(String);

	impl ::std::ops::Deref for GmlcInfoGmlcNumbersItem {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<GmlcInfoGmlcNumbersItem> for String {
		fn from(value: GmlcInfoGmlcNumbersItem) -> Self {
			value.0
		}
	}

	impl From<&GmlcInfoGmlcNumbersItem> for GmlcInfoGmlcNumbersItem {
		fn from(value: &GmlcInfoGmlcNumbersItem) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for GmlcInfoGmlcNumbersItem {
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

	impl ::std::convert::TryFrom<&str> for GmlcInfoGmlcNumbersItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for GmlcInfoGmlcNumbersItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for GmlcInfoGmlcNumbersItem {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for GmlcInfoGmlcNumbersItem {
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

	/// Subscription to a set of AMFs, based on their GUAMIs
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of AMFs, based on their GUAMIs",
	///  "type": "object",
	///  "required": [
	///    "guamiList"
	///  ],
	///  "properties": {
	///    "guamiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Guami"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GuamiListCond {
		#[serde(rename = "guamiList")]
		pub guami_list: Vec<crate::common::common_models::Guami>,
	}

	impl From<&GuamiListCond> for GuamiListCond {
		fn from(value: &GuamiListCond) -> Self {
			value.clone()
		}
	}

	/// Information of an HSS NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an HSS NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "externalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
	///      },
	///      "minItems": 1
	///    },
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "hssDiameterAddress": {
	///      "$ref": "#/components/schemas/NetworkNodeDiameterAddress"
	///    },
	///    "imsPrivateIdentityRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
	///      },
	///      "minItems": 1
	///    },
	///    "imsPublicIdentityRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
	///      },
	///      "minItems": 1
	///    },
	///    "imsiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ImsiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "msisdnRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	pub struct HssInfo {
		#[serde(
			rename = "externalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_group_identifiers_ranges: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "hssDiameterAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hss_diameter_address: Option<NetworkNodeDiameterAddress>,
		#[serde(
			rename = "imsPrivateIdentityRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ims_private_identity_ranges: Vec<IdentityRange>,
		#[serde(
			rename = "imsPublicIdentityRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ims_public_identity_ranges: Vec<IdentityRange>,
		#[serde(rename = "imsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub imsi_ranges: Vec<ImsiRange>,
		#[serde(
			rename = "msisdnRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub msisdn_ranges: Vec<IdentityRange>,
	}

	impl From<&HssInfo> for HssInfo {
		fn from(value: &HssInfo) -> Self {
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

	/// A range of IMSIs (subscriber identities), either based on a numeric
	/// range, or based on regular-expression matching
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A range of IMSIs (subscriber identities), either based
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
	pub struct ImsiRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<ImsiRangeEnd>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pattern: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<ImsiRangeStart>,
	}

	impl From<&ImsiRange> for ImsiRange {
		fn from(value: &ImsiRange) -> Self {
			value.clone()
		}
	}

	/// ImsiRangeEnd
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
	)]
	pub struct ImsiRangeEnd(String);

	impl ::std::ops::Deref for ImsiRangeEnd {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ImsiRangeEnd> for String {
		fn from(value: ImsiRangeEnd) -> Self {
			value.0
		}
	}

	impl From<&ImsiRangeEnd> for ImsiRangeEnd {
		fn from(value: &ImsiRangeEnd) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ImsiRangeEnd {
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

	impl ::std::convert::TryFrom<&str> for ImsiRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ImsiRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ImsiRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ImsiRangeEnd {
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

	/// ImsiRangeStart
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
	)]
	pub struct ImsiRangeStart(String);

	impl ::std::ops::Deref for ImsiRangeStart {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ImsiRangeStart> for String {
		fn from(value: ImsiRangeStart) -> Self {
			value.0
		}
	}

	impl From<&ImsiRangeStart> for ImsiRangeStart {
		fn from(value: &ImsiRangeStart) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for ImsiRangeStart {
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

	impl ::std::convert::TryFrom<&str> for ImsiRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for ImsiRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for ImsiRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for ImsiRangeStart {
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

	/// Information of a given IP interface of an UPF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a given IP interface of an UPF",
	///  "type": "object",
	///  "required": [
	///    "interfaceType"
	///  ],
	///  "properties": {
	///    "endpointFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "interfaceType": {
	///      "$ref": "#/components/schemas/UPInterfaceType"
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
	///    },
	///    "networkInstance": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct InterfaceUpfInfoItem {
		#[serde(
			rename = "endpointFqdn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub endpoint_fqdn: Option<Fqdn>,
		#[serde(rename = "interfaceType")]
		pub interface_type: UpInterfaceType,
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
		#[serde(
			rename = "networkInstance",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub network_instance: Option<String>,
	}

	impl From<&InterfaceUpfInfoItem> for InterfaceUpfInfoItem {
		fn from(value: &InterfaceUpfInfoItem) -> Self {
			value.clone()
		}
	}

	/// A range of Group IDs (internal group identities), either based on a
	/// numeric range, or based on regular-expression matching
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A range of Group IDs (internal group identities),
	/// either based on a numeric range, or based on regular-expression
	/// matching\n",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "$ref": "#/components/schemas/GroupId"
	///    },
	///    "pattern": {
	///      "type": "string"
	///    },
	///    "start": {
	///      "$ref": "#/components/schemas/GroupId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct InternalGroupIdRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<GroupId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pattern: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<GroupId>,
	}

	impl From<&InternalGroupIdRange> for InternalGroupIdRange {
		fn from(value: &InternalGroupIdRange) -> Self {
			value.clone()
		}
	}

	/// IP addressing information of a given NFService; it consists on, e.g. IP
	/// address, TCP port, transport protocol...
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "IP addressing information of a given NFService; it
	/// consists on, e.g. IP address, TCP port, transport protocol...\n",
	///  "type": "object",
	///  "properties": {
	///    "ipv4Address": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6Address": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "port": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "transport": {
	///      "$ref": "#/components/schemas/TransportProtocol"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct IpEndPoint {
		#[serde(
			rename = "ipv4Address",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ipv4_address: Option<Ipv4Addr>,
		#[serde(
			rename = "ipv6Address",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ipv6_address: Option<Ipv6Addr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub port: Option<u16>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub transport: Option<TransportProtocol>,
	}

	impl From<&IpEndPoint> for IpEndPoint {
		fn from(value: &IpEndPoint) -> Self {
			value.clone()
		}
	}

	/// Represents the IP Index to be sent from UDM to the SMF (its value can be
	/// either an integer or a string)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the IP Index to be sent from UDM to the SMF
	/// (its value can be either an integer or a string)",
	///  "anyOf": [
	///    {
	///      "type": "integer"
	///    },
	///    {
	///      "type": "string"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum IpIndex {
		#[default]
		Variant0(i64),
		Variant1(String),
	}

	impl From<&IpIndex> for IpIndex {
		fn from(value: &IpIndex) -> Self {
			value.clone()
		}
	}

	impl std::str::FromStr for IpIndex {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if let Ok(v) = value.parse() {
				Ok(Self::Variant0(v))
			} else if let Ok(v) = value.parse() {
				Ok(Self::Variant1(v))
			} else {
				Err("string conversion failed for all variants".into())
			}
		}
	}

	impl std::convert::TryFrom<&str> for IpIndex {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for IpIndex {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for IpIndex {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ToString for IpIndex {
		fn to_string(&self) -> String {
			match self {
				Self::Variant0(x) => x.to_string(),
				Self::Variant1(x) => x.to_string(),
			}
		}
	}

	impl From<i64> for IpIndex {
		fn from(value: i64) -> Self {
			Self::Variant0(value)
		}
	}

	/// Indicates the type(s) of IP addresses reachable via an SCP
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the type(s) of IP addresses reachable via an
	/// SCP",
	///  "type": "string",
	///  "enum": [
	///    "IPV4",
	///    "IPV6",
	///    "IPV4V6"
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
	pub enum IpReachability {
		#[default]
		#[serde(rename = "IPV4")]
		Ipv4,
		#[serde(rename = "IPV6")]
		Ipv6,
		#[serde(rename = "IPV4V6")]
		Ipv4v6,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&IpReachability> for IpReachability {
		fn from(value: &IpReachability) -> Self {
			value.clone()
		}
	}

	impl ToString for IpReachability {
		fn to_string(&self) -> String {
			match *self {
				Self::Ipv4 => "IPV4".to_string(),
				Self::Ipv6 => "IPV6".to_string(),
				Self::Ipv4v6 => "IPV4V6".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for IpReachability {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IPV4" => Ok(Self::Ipv4),
				"IPV6" => Ok(Self::Ipv6),
				"IPV4V6" => Ok(Self::Ipv4v6),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for IpReachability {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for IpReachability {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for IpReachability {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Range of IPv4 addresses
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of IPv4 addresses",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "start": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Ipv4AddressRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<Ipv4Addr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<Ipv4Addr>,
	}

	impl From<&Ipv4AddressRange> for Ipv4AddressRange {
		fn from(value: &Ipv4AddressRange) -> Self {
			value.clone()
		}
	}

	/// Range of IPv6 prefixes
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of IPv6 prefixes",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "start": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Ipv6PrefixRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<Ipv6Prefix>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<Ipv6Prefix>,
	}

	impl From<&Ipv6PrefixRange> for Ipv6PrefixRange {
		fn from(value: &Ipv6PrefixRange) -> Self {
			value.clone()
		}
	}

	/// Information of an SMS-IWMSC NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an SMS-IWMSC NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "msisdnRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
	///      },
	///      "minItems": 1
	///    },
	///    "scNumber": {
	///      "type": "string",
	///      "pattern": "^[0-9]{5,15}$"
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
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
	pub struct IwmscInfo {
		#[serde(
			rename = "msisdnRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub msisdn_ranges: Vec<IdentityRange>,
		#[serde(rename = "scNumber", default, skip_serializing_if = "Option::is_none")]
		pub sc_number: Option<IwmscInfoScNumber>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&IwmscInfo> for IwmscInfo {
		fn from(value: &IwmscInfo) -> Self {
			value.clone()
		}
	}

	/// IwmscInfoScNumber
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	)]
	pub struct IwmscInfoScNumber(String);

	impl ::std::ops::Deref for IwmscInfoScNumber {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<IwmscInfoScNumber> for String {
		fn from(value: IwmscInfoScNumber) -> Self {
			value.0
		}
	}

	impl From<&IwmscInfoScNumber> for IwmscInfoScNumber {
		fn from(value: &IwmscInfoScNumber) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for IwmscInfoScNumber {
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

	impl ::std::convert::TryFrom<&str> for IwmscInfoScNumber {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for IwmscInfoScNumber {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for IwmscInfoScNumber {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for IwmscInfoScNumber {
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
		Variant0(Vec<Link>),
		#[default]
		Variant1(Link),
	}

	impl LinksValueSchema {
		pub fn get_links(self) -> Vec<Link> {
			match self {
				LinksValueSchema::Variant0(links) => links,
				LinksValueSchema::Variant1(link) => vec![link],
			}
		}
	}

	/// LMF identification.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "LMF identification.",
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
	pub struct LmfIdentification(pub String);

	impl ::std::ops::Deref for LmfIdentification {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<LmfIdentification> for String {
		fn from(value: LmfIdentification) -> Self {
			value.0
		}
	}

	impl From<&LmfIdentification> for LmfIdentification {
		fn from(value: &LmfIdentification) -> Self {
			value.clone()
		}
	}

	impl From<String> for LmfIdentification {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for LmfIdentification {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for LmfIdentification {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Information of an LMF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an LMF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "lmfId": {
	///      "$ref": "#/components/schemas/LMFIdentification"
	///    },
	///    "servingAccessTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "servingAnNodeTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AnNodeType"
	///      },
	///      "minItems": 1
	///    },
	///    "servingClientTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExternalClientType"
	///      },
	///      "minItems": 1
	///    },
	///    "servingRatTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedGADShapes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupportedGADShapes"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
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
	pub struct LmfInfo {
		#[serde(rename = "lmfId", default, skip_serializing_if = "Option::is_none")]
		pub lmf_id: Option<LmfIdentification>,
		#[serde(
			rename = "servingAccessTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_access_types: Vec<AccessType>,
		#[serde(
			rename = "servingAnNodeTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_an_node_types: Vec<AnNodeType>,
		#[serde(
			rename = "servingClientTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_client_types: Vec<ExternalClientType>,
		#[serde(
			rename = "servingRatTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_rat_types: Vec<RatType>,
		#[serde(
			rename = "supportedGADShapes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub supported_gad_shapes: Vec<SupportedGadShapes>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&LmfInfo> for LmfInfo {
		fn from(value: &LmfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an MB-SMF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an MB-SMF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "mbsSessionList": {
	///      "description": "A map (list of key-value pairs) where a valid JSON
	/// string serves as key",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MbsSession"
	///      }
	///    },
	///    "sNssaiInfoList": {
	///      "description": "A map (list of key-value pairs) where a valid JSON
	/// string serves as key",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SnssaiMbSmfInfoItem"
	///      }
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "tmgiRangeList": {
	///      "description": "A map (list of key-value pairs) where a valid JSON
	/// string serves as key",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/TmgiRange"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbSmfInfo {
		/// A map (list of key-value pairs) where a valid JSON string serves as
		/// key
		#[serde(
			rename = "mbsSessionList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub mbs_session_list: ::std::collections::HashMap<String, MbsSession>,
		/// A map (list of key-value pairs) where a valid JSON string serves as
		/// key
		#[serde(
			rename = "sNssaiInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub s_nssai_info_list: ::std::collections::HashMap<String, SnssaiMbSmfInfoItem>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
		/// A map (list of key-value pairs) where a valid JSON string serves as
		/// key
		#[serde(
			rename = "tmgiRangeList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub tmgi_range_list: ::std::collections::HashMap<String, TmgiRange>,
	}

	impl From<&MbSmfInfo> for MbSmfInfo {
		fn from(value: &MbSmfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an MB-UPF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an MB-UPF NF Instance",
	///  "type": "object",
	///  "required": [
	///    "sNssaiMbUpfInfoList"
	///  ],
	///  "properties": {
	///    "interfaceMbUpfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/InterfaceUpfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "mbSmfServingArea": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "priority": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "sNssaiMbUpfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SnssaiUpfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedPfcpFeatures": {
	///      "type": "string"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
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
	pub struct MbUpfInfo {
		#[serde(
			rename = "interfaceMbUpfInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub interface_mb_upf_info_list: Vec<InterfaceUpfInfoItem>,
		#[serde(
			rename = "mbSmfServingArea",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mb_smf_serving_area: Vec<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub priority: Option<u16>,
		#[serde(rename = "sNssaiMbUpfInfoList")]
		pub s_nssai_mb_upf_info_list: Vec<SnssaiUpfInfoItem>,
		#[serde(
			rename = "supportedPfcpFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_pfcp_features: Option<String>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&MbUpfInfo> for MbUpfInfo {
		fn from(value: &MbUpfInfo) -> Self {
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
	///      "$ref": "#/components/schemas/Uint16"
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
		pub area_session_id: Uint16,
		#[serde(rename = "mbsServiceArea")]
		pub mbs_service_area: MbsServiceArea,
	}

	/// MBS Session currently served by an MB-SMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS Session currently served by an MB-SMF",
	///  "type": "object",
	///  "required": [
	///    "mbsSessionId"
	///  ],
	///  "properties": {
	///    "mbsAreaSessions": {
	///      "description": "A map (list of key-value pairs) where the key
	/// identifies an areaSessionId",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MbsServiceAreaInfo"
	///      }
	///    },
	///    "mbsSessionId": {
	///      "$ref": "#/components/schemas/MbsSessionId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MbsSession {
		/// A map (list of key-value pairs) where the key identifies an
		/// areaSessionId
		#[serde(
			rename = "mbsAreaSessions",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub mbs_area_sessions: ::std::collections::HashMap<String, MbsServiceAreaInfo>,
		#[serde(rename = "mbsSessionId")]
		pub mbs_session_id: MbsSessionId,
	}

	/// Information of a MFAF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a MFAF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "servingNfSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFType"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
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
	pub struct MfafInfo {
		#[serde(
			rename = "servingNfSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_set_id_list: Vec<NfSetId>,
		#[serde(
			rename = "servingNfTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_type_list: Vec<NfType>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&MfafInfo> for MfafInfo {
		fn from(value: &MfafInfo) -> Self {
			value.clone()
		}
	}

	/// ML Analytics Filter information supported by the Nnwdaf_MLModelProvision
	/// service
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "ML Analytics Filter information supported by the
	/// Nnwdaf_MLModelProvision service",
	///  "type": "object",
	///  "properties": {
	///    "mlAnalyticsIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "snssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
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
	pub struct MlAnalyticsInfo {
		#[serde(
			rename = "mlAnalyticsIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ml_analytics_ids: Vec<NwdafEvent>,
		#[serde(rename = "snssaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub snssai_list: Vec<Snssai>,
		#[serde(
			rename = "trackingAreaList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tracking_area_list: Vec<Tai>,
	}

	impl From<&MlAnalyticsInfo> for MlAnalyticsInfo {
		fn from(value: &MlAnalyticsInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an MNPF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an MNPF Instance",
	///  "type": "object",
	///  "required": [
	///    "msisdnRanges"
	///  ],
	///  "properties": {
	///    "msisdnRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	pub struct MnpfInfo {
		#[serde(rename = "msisdnRanges")]
		pub msisdn_ranges: Vec<IdentityRange>,
	}

	impl From<&MnpfInfo> for MnpfInfo {
		fn from(value: &MnpfInfo) -> Self {
			value.clone()
		}
	}

	/// Enumeration for N1 Message Class
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Enumeration for N1 Message Class",
	///  "type": "string",
	///  "enum": [
	///    "5GMM",
	///    "SM",
	///    "LPP",
	///    "SMS",
	///    "UPDP",
	///    "LCS"
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
	pub enum N1MessageClass {
		#[default]
		#[serde(rename = "5GMM")]
		FiveGmm,
		#[serde(rename = "SM")]
		Sm,
		#[serde(rename = "LPP")]
		Lpp,
		#[serde(rename = "SMS")]
		Sms,
		#[serde(rename = "UPDP")]
		Updp,
		#[serde(rename = "LCS")]
		Lcs,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&N1MessageClass> for N1MessageClass {
		fn from(value: &N1MessageClass) -> Self {
			value.clone()
		}
	}

	impl ToString for N1MessageClass {
		fn to_string(&self) -> String {
			match *self {
				Self::FiveGmm => "5GMM".to_string(),
				Self::Sm => "SM".to_string(),
				Self::Lpp => "LPP".to_string(),
				Self::Sms => "SMS".to_string(),
				Self::Updp => "UPDP".to_string(),
				Self::Lcs => "LCS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for N1MessageClass {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"5GMM" => Ok(Self::FiveGmm),
				"SM" => Ok(Self::Sm),
				"LPP" => Ok(Self::Lpp),
				"SMS" => Ok(Self::Sms),
				"UPDP" => Ok(Self::Updp),
				"LCS" => Ok(Self::Lcs),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for N1MessageClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for N1MessageClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for N1MessageClass {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Enumeration for N2 Information Class
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Enumeration for N2 Information Class",
	///  "type": "string",
	///  "enum": [
	///    "SM",
	///    "NRPPa",
	///    "PWS",
	///    "PWS-BCAL",
	///    "PWS-RF",
	///    "RAN",
	///    "V2X",
	///    "PROSE"
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
	pub enum N2InformationClass {
		#[default]
		#[serde(rename = "SM")]
		Sm,
		#[serde(rename = "NRPPa")]
		NrpPa,
		#[serde(rename = "PWS")]
		Pws,
		#[serde(rename = "PWS-BCAL")]
		PwsBcal,
		#[serde(rename = "PWS-RF")]
		PwsRf,
		#[serde(rename = "RAN")]
		Ran,
		#[serde(rename = "V2X")]
		V2x,
		#[serde(rename = "PROSE")]
		Prose,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&N2InformationClass> for N2InformationClass {
		fn from(value: &N2InformationClass) -> Self {
			value.clone()
		}
	}

	impl ToString for N2InformationClass {
		fn to_string(&self) -> String {
			match *self {
				Self::Sm => "SM".to_string(),
				Self::NrpPa => "NRPPa".to_string(),
				Self::Pws => "PWS".to_string(),
				Self::PwsBcal => "PWS-BCAL".to_string(),
				Self::PwsRf => "PWS-RF".to_string(),
				Self::Ran => "RAN".to_string(),
				Self::V2x => "V2X".to_string(),
				Self::Prose => "PROSE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for N2InformationClass {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SM" => Ok(Self::Sm),
				"NRPPa" => Ok(Self::NrpPa),
				"PWS" => Ok(Self::Pws),
				"PWS-BCAL" => Ok(Self::PwsBcal),
				"PWS-RF" => Ok(Self::PwsRf),
				"RAN" => Ok(Self::Ran),
				"V2X" => Ok(Self::V2x),
				"PROSE" => Ok(Self::Prose),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for N2InformationClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for N2InformationClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for N2InformationClass {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// AMF N2 interface information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "AMF N2 interface information",
	///  "type": "object",
	///  "properties": {
	///    "amfName": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "ipv4EndpointAddress": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6EndpointAddress": {
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
	pub struct N2InterfaceAmfInfo {
		#[serde(rename = "amfName", default, skip_serializing_if = "Option::is_none")]
		pub amf_name: Option<Fqdn>,
		#[serde(
			rename = "ipv4EndpointAddress",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_endpoint_address: Vec<Ipv4Addr>,
		#[serde(
			rename = "ipv6EndpointAddress",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_endpoint_address: Vec<Ipv6Addr>,
	}

	impl From<&N2InterfaceAmfInfo> for N2InterfaceAmfInfo {
		fn from(value: &N2InterfaceAmfInfo) -> Self {
			value.clone()
		}
	}

	/// Subscription to a set of NF Instances (NEFs), identified by Event ID(s)
	/// provided by AF, S-NSSAI(s), AF Instance ID, Application Identifier,
	/// External Identifier, External Group Identifier, or domain name.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NF Instances (NEFs),
	/// identified by Event ID(s) provided by AF, S-NSSAI(s), AF Instance ID,
	/// Application Identifier, External Identifier, External Group Identifier,
	/// or domain name.\n",
	///  "type": "object",
	///  "required": [
	///    "conditionType"
	///  ],
	///  "properties": {
	///    "afEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AfEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "conditionType": {
	///      "type": "string",
	///      "enum": [
	///        "NEF_COND"
	///      ]
	///    },
	///    "externalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	///    "pfdData": {
	///      "$ref": "#/components/schemas/PfdData"
	///    },
	///    "servedFqdnList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "snssaiList": {
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
	pub struct NefCond {
		#[serde(rename = "afEvents", default, skip_serializing_if = "Vec::is_empty")]
		pub af_events: Vec<AfEvent>,
		#[serde(rename = "conditionType")]
		pub condition_type: NefCondConditionType,
		#[serde(
			rename = "externalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_group_identifiers_ranges: Vec<IdentityRange>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "pfdData", default, skip_serializing_if = "Option::is_none")]
		pub pfd_data: Option<PfdData>,
		#[serde(
			rename = "servedFqdnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub served_fqdn_list: Vec<String>,
		#[serde(rename = "snssaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub snssai_list: Vec<Snssai>,
	}

	impl From<&NefCond> for NefCond {
		fn from(value: &NefCond) -> Self {
			value.clone()
		}
	}

	/// NefCondConditionType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "NEF_COND"
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
	pub enum NefCondConditionType {
		#[default]
		#[serde(rename = "NEF_COND")]
		NefCond,
	}

	impl From<&NefCondConditionType> for NefCondConditionType {
		fn from(value: &NefCondConditionType) -> Self {
			value.clone()
		}
	}

	impl ToString for NefCondConditionType {
		fn to_string(&self) -> String {
			match *self {
				Self::NefCond => "NEF_COND".to_string(),
			}
		}
	}

	impl std::str::FromStr for NefCondConditionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NEF_COND" => Ok(Self::NefCond),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NefCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NefCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NefCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Identity of the NEF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identity of the NEF",
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
	pub struct NefId(pub String);

	impl ::std::ops::Deref for NefId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<NefId> for String {
		fn from(value: NefId) -> Self {
			value.0
		}
	}

	impl From<&NefId> for NefId {
		fn from(value: &NefId) -> Self {
			value.clone()
		}
	}

	impl From<String> for NefId {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NefId {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for NefId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Information of an NEF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an NEF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "afEeData": {
	///      "$ref": "#/components/schemas/AfEventExposureData"
	///    },
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "externalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	///    "nefId": {
	///      "$ref": "#/components/schemas/NefId"
	///    },
	///    "pfdData": {
	///      "$ref": "#/components/schemas/PfdData"
	///    },
	///    "servedFqdnList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "uasNfFunctionalityInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "unTrustAfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UnTrustAfInfo"
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
	pub struct NefInfo {
		#[serde(rename = "afEeData", default, skip_serializing_if = "Option::is_none")]
		pub af_ee_data: Option<AfEventExposureData>,
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<Dnai>,
		#[serde(
			rename = "externalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_group_identifiers_ranges: Vec<IdentityRange>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "nefId", default, skip_serializing_if = "Option::is_none")]
		pub nef_id: Option<NefId>,
		#[serde(rename = "pfdData", default, skip_serializing_if = "Option::is_none")]
		pub pfd_data: Option<PfdData>,
		#[serde(
			rename = "servedFqdnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub served_fqdn_list: Vec<String>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
		#[serde(rename = "uasNfFunctionalityInd", default)]
		pub uas_nf_functionality_ind: bool,
		#[serde(
			rename = "unTrustAfInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub un_trust_af_info_list: Vec<UnTrustAfInfo>,
	}

	impl From<&NefInfo> for NefInfo {
		fn from(value: &NefInfo) -> Self {
			value.clone()
		}
	}

	/// NetworkNodeDiameterAddress
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "name",
	///    "realm"
	///  ],
	///  "properties": {
	///    "name": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "realm": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NetworkNodeDiameterAddress {
		pub name: Fqdn,
		pub realm: Fqdn,
	}

	impl From<&NetworkNodeDiameterAddress> for NetworkNodeDiameterAddress {
		fn from(value: &NetworkNodeDiameterAddress) -> Self {
			value.clone()
		}
	}

	/// Subscription to a set of NFs, based on the slices (S-NSSAI and NSI) they
	/// support
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NFs, based on the slices
	/// (S-NSSAI and NSI) they support",
	///  "type": "object",
	///  "required": [
	///    "snssaiList"
	///  ],
	///  "properties": {
	///    "nsiList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      }
	///    },
	///    "snssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NetworkSliceCond {
		#[serde(rename = "nsiList", default, skip_serializing_if = "Vec::is_empty")]
		pub nsi_list: Vec<String>,
		#[serde(rename = "snssaiList")]
		pub snssai_list: Vec<Snssai>,
	}

	impl From<&NetworkSliceCond> for NetworkSliceCond {
		fn from(value: &NetworkSliceCond) -> Self {
			value.clone()
		}
	}

	/// Subscription to a set of NFs based on their Group Id
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NFs based on their Group Id",
	///  "type": "object",
	///  "required": [
	///    "nfGroupId",
	///    "nfType"
	///  ],
	///  "properties": {
	///    "nfGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "nfType": {
	///      "type": "string",
	///      "enum": [
	///        "UDM",
	///        "AUSF",
	///        "UDR",
	///        "PCF",
	///        "CHF",
	///        "HSS"
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfGroupCond {
		#[serde(rename = "nfGroupId")]
		pub nf_group_id: NfGroupId,
		#[serde(rename = "nfType")]
		pub nf_type: NfGroupCondNfType,
	}

	impl From<&NfGroupCond> for NfGroupCond {
		fn from(value: &NfGroupCond) -> Self {
			value.clone()
		}
	}

	/// NfGroupCondNfType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "UDM",
	///    "AUSF",
	///    "UDR",
	///    "PCF",
	///    "CHF",
	///    "HSS"
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
	pub enum NfGroupCondNfType {
		#[default]
		#[serde(rename = "UDM")]
		Udm,
		#[serde(rename = "AUSF")]
		Ausf,
		#[serde(rename = "UDR")]
		Udr,
		#[serde(rename = "PCF")]
		Pcf,
		#[serde(rename = "CHF")]
		Chf,
		#[serde(rename = "HSS")]
		Hss,
	}

	impl From<&NfGroupCondNfType> for NfGroupCondNfType {
		fn from(value: &NfGroupCondNfType) -> Self {
			value.clone()
		}
	}

	impl ToString for NfGroupCondNfType {
		fn to_string(&self) -> String {
			match *self {
				Self::Udm => "UDM".to_string(),
				Self::Ausf => "AUSF".to_string(),
				Self::Udr => "UDR".to_string(),
				Self::Pcf => "PCF".to_string(),
				Self::Chf => "CHF".to_string(),
				Self::Hss => "HSS".to_string(),
			}
		}
	}

	impl std::str::FromStr for NfGroupCondNfType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UDM" => Ok(Self::Udm),
				"AUSF" => Ok(Self::Ausf),
				"UDR" => Ok(Self::Udr),
				"PCF" => Ok(Self::Pcf),
				"CHF" => Ok(Self::Chf),
				"HSS" => Ok(Self::Hss),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NfGroupCondNfType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NfGroupCondNfType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NfGroupCondNfType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Subscription to a set of NFs based on their Group Ids
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NFs based on their Group Ids",
	///  "type": "object",
	///  "required": [
	///    "conditionType",
	///    "nfGroupIdList",
	///    "nfType"
	///  ],
	///  "properties": {
	///    "conditionType": {
	///      "type": "string",
	///      "enum": [
	///        "NF_GROUP_LIST_COND"
	///      ]
	///    },
	///    "nfGroupIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfGroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "nfType": {
	///      "type": "string",
	///      "enum": [
	///        "UDM",
	///        "AUSF",
	///        "UDR",
	///        "PCF",
	///        "CHF",
	///        "HSS"
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfGroupListCond {
		#[serde(rename = "conditionType")]
		pub condition_type: NfGroupListCondConditionType,
		#[serde(rename = "nfGroupIdList")]
		pub nf_group_id_list: Vec<NfGroupId>,
		#[serde(rename = "nfType")]
		pub nf_type: NfGroupListCondNfType,
	}

	impl From<&NfGroupListCond> for NfGroupListCond {
		fn from(value: &NfGroupListCond) -> Self {
			value.clone()
		}
	}

	/// NfGroupListCondConditionType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "NF_GROUP_LIST_COND"
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
	pub enum NfGroupListCondConditionType {
		#[default]
		#[serde(rename = "NF_GROUP_LIST_COND")]
		NfGroupListCond,
	}

	impl From<&NfGroupListCondConditionType> for NfGroupListCondConditionType {
		fn from(value: &NfGroupListCondConditionType) -> Self {
			value.clone()
		}
	}

	impl ToString for NfGroupListCondConditionType {
		fn to_string(&self) -> String {
			match *self {
				Self::NfGroupListCond => "NF_GROUP_LIST_COND".to_string(),
			}
		}
	}

	impl std::str::FromStr for NfGroupListCondConditionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NF_GROUP_LIST_COND" => Ok(Self::NfGroupListCond),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NfGroupListCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NfGroupListCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NfGroupListCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// NfGroupListCondNfType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "UDM",
	///    "AUSF",
	///    "UDR",
	///    "PCF",
	///    "CHF",
	///    "HSS"
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
	pub enum NfGroupListCondNfType {
		#[default]
		#[serde(rename = "UDM")]
		Udm,
		#[serde(rename = "AUSF")]
		Ausf,
		#[serde(rename = "UDR")]
		Udr,
		#[serde(rename = "PCF")]
		Pcf,
		#[serde(rename = "CHF")]
		Chf,
		#[serde(rename = "HSS")]
		Hss,
	}

	impl From<&NfGroupListCondNfType> for NfGroupListCondNfType {
		fn from(value: &NfGroupListCondNfType) -> Self {
			value.clone()
		}
	}

	impl ToString for NfGroupListCondNfType {
		fn to_string(&self) -> String {
			match *self {
				Self::Udm => "UDM".to_string(),
				Self::Ausf => "AUSF".to_string(),
				Self::Udr => "UDR".to_string(),
				Self::Pcf => "PCF".to_string(),
				Self::Chf => "CHF".to_string(),
				Self::Hss => "HSS".to_string(),
			}
		}
	}

	impl std::str::FromStr for NfGroupListCondNfType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UDM" => Ok(Self::Udm),
				"AUSF" => Ok(Self::Ausf),
				"UDR" => Ok(Self::Udr),
				"PCF" => Ok(Self::Pcf),
				"CHF" => Ok(Self::Chf),
				"HSS" => Ok(Self::Hss),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NfGroupListCondNfType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NfGroupListCondNfType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NfGroupListCondNfType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Information of a generic NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a generic NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "nfType": {
	///      "$ref": "#/components/schemas/NFType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfInfo {
		#[serde(rename = "nfType", default, skip_serializing_if = "Option::is_none")]
		pub nf_type: Option<NfType>,
	}

	impl From<&NfInfo> for NfInfo {
		fn from(value: &NfInfo) -> Self {
			value.clone()
		}
	}

	/// Subscription to a given NF Instance Id
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a given NF Instance Id",
	///  "type": "object",
	///  "required": [
	///    "nfInstanceId"
	///  ],
	///  "properties": {
	///    "nfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfInstanceIdCond {
		#[serde(rename = "nfInstanceId")]
		pub nf_instance_id: NfInstanceId,
	}

	impl From<&NfInstanceIdCond> for NfInstanceIdCond {
		fn from(value: &NfInstanceIdCond) -> Self {
			value.clone()
		}
	}

	/// Subscription to a list of NF Instances
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a list of NF Instances",
	///  "type": "object",
	///  "required": [
	///    "nfInstanceIdList"
	///  ],
	///  "properties": {
	///    "nfInstanceIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfInstanceId"
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
	pub struct NfInstanceIdListCond {
		#[serde(rename = "nfInstanceIdList")]
		pub nf_instance_id_list: Vec<NfInstanceId>,
	}

	impl From<&NfInstanceIdListCond> for NfInstanceIdListCond {
		fn from(value: &NfInstanceIdListCond) -> Self {
			value.clone()
		}
	}

	/// Contains information on an NF profile matching a discovery request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains information on an NF profile matching a
	/// discovery request",
	///  "type": "object",
	///  "properties": {
	///    "nrfAlteredPriorities": {
	///      "description": "The key of the map is the JSON Pointer of the priority IE in the NFProfile data type that is altered by the NRF\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "integer",
	///        "maximum": 65535.0,
	///        "minimum": 0.0
	///      }
	///    },
	///    "nrfDiscApiUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "preferredSearch": {
	///      "$ref": "#/components/schemas/PreferredSearch"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfInstanceInfo {
		/// The key of the map is the JSON Pointer of the priority IE in the
		/// NFProfile data type that is altered by the NRF
		#[serde(
			rename = "nrfAlteredPriorities",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub nrf_altered_priorities: ::std::collections::HashMap<String, u16>,
		#[serde(
			rename = "nrfDiscApiUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_disc_api_uri: Option<Uri>,
		#[serde(
			rename = "preferredSearch",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub preferred_search: Option<PreferredSearch>,
	}

	impl From<&NfInstanceInfo> for NfInstanceInfo {
		fn from(value: &NfInstanceInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an NF Instance discovered by the NRF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an NF Instance discovered by the NRF",
	///  "type": "object",
	///  "required": [
	///    "nfInstanceId",
	///    "nfStatus",
	///    "nfType"
	///  ],
	///  "properties": {
	///    "aanfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of AanfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/AanfInfo"
	///      }
	///    },
	///    "amfInfo": {
	///      "$ref": "#/components/schemas/schemas-AmfInfo"
	///    },
	///    "amfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of AmfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-AmfInfo"
	///      }
	///    },
	///    "ausfInfo": {
	///      "$ref": "#/components/schemas/schemas-AusfInfo"
	///    },
	///    "ausfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of AusfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-AusfInfo"
	///      }
	///    },
	///    "bsfInfo": {
	///      "$ref": "#/components/schemas/schemas-BsfInfo"
	///    },
	///    "bsfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of BsfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-BsfInfo"
	///      }
	///    },
	///    "capacity": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "chfInfo": {
	///      "$ref": "#/components/schemas/schemas-ChfInfo"
	///    },
	///    "chfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of ChfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-ChfInfo"
	///      }
	///    },
	///    "collocatedNfInstances": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-CollocatedNfInstance"
	///      },
	///      "minItems": 1
	///    },
	///    "customInfo": {
	///      "type": "object"
	///    },
	///    "dccfInfo": {
	///      "$ref": "#/components/schemas/schemas-DccfInfo"
	///    },
	///    "defaultNotificationSubscriptions": {
	///      "type": "array",
	///      "items": {
	///        "$ref":
	/// "#/components/schemas/schemas-DefaultNotificationSubscription"
	///      }
	///    },
	///    "easdfInfoList": {
	///      "description": "A map(list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of EasdfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-EasdfInfo"
	///      }
	///    },
	///    "fqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "gmlcInfo": {
	///      "$ref": "#/components/schemas/schemas-GmlcInfo"
	///    },
	///    "hniList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Fqdn"
	///      },
	///      "minItems": 1
	///    },
	///    "hssInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of HssInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-HssInfo"
	///      }
	///    },
	///    "interPlmnFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
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
	///    },
	///    "iwmscInfo": {
	///      "$ref": "#/components/schemas/schemas-IwmscInfo"
	///    },
	///    "lcHSupportInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "lmfInfo": {
	///      "$ref": "#/components/schemas/schemas-LmfInfo"
	///    },
	///    "load": {
	///      "type": "integer",
	///      "maximum": 100.0,
	///      "minimum": 0.0
	///    },
	///    "loadTimeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "locality": {
	///      "type": "string"
	///    },
	///    "mbSmfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of MbSmfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-MbSmfInfo"
	///      }
	///    },
	///    "mbUpfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of MbUpfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-MbUpfInfo"
	///      }
	///    },
	///    "mfafInfo": {
	///      "$ref": "#/components/schemas/schemas-MfafInfo"
	///    },
	///    "mnpfInfo": {
	///      "$ref": "#/components/schemas/MnpfInfo"
	///    },
	///    "nefInfo": {
	///      "$ref": "#/components/schemas/schemas-NefInfo"
	///    },
	///    "nfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nfInstanceName": {
	///      "type": "string"
	///    },
	///    "nfServiceList": {
	///      "description": "A map (list of key-value pairs) where
	/// serviceInstanceId serves as key of NFService\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/NFService"
	///      }
	///    },
	///    "nfServicePersistence": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "nfServices": {
	///      "deprecated": true,
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFService"
	///      },
	///      "minItems": 1
	///    },
	///    "nfSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "nfSetRecoveryTimeList": {
	///      "description": "A map (list of key-value pairs) where NfSetId
	/// serves as key of DateTime",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/DateTime"
	///      }
	///    },
	///    "nfStatus": {
	///      "$ref": "#/components/schemas/NFStatus"
	///    },
	///    "nfType": {
	///      "$ref": "#/components/schemas/NFType"
	///    },
	///    "nsacfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of NsacfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-NsacfInfo"
	///      }
	///    },
	///    "nsiList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "nssaafInfo": {
	///      "$ref": "#/components/schemas/schemas-NssaafInfo"
	///    },
	///    "nwdafInfo": {
	///      "$ref": "#/components/schemas/schemas-NwdafInfo"
	///    },
	///    "nwdafInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of NwdafInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-NwdafInfo"
	///      }
	///    },
	///    "olcHSupportInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "pcfInfo": {
	///      "$ref": "#/components/schemas/schemas-PcfInfo"
	///    },
	///    "pcfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of PcfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-PcfInfo"
	///      }
	///    },
	///    "pcscfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of PcscfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-PcscfInfo"
	///      }
	///    },
	///    "perPlmnSnssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-PlmnSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "plmnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
	///      },
	///      "minItems": 1
	///    },
	///    "priority": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "sNssais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "scpDomains": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "scpInfo": {
	///      "$ref": "#/components/schemas/schemas-ScpInfo"
	///    },
	///    "seppInfo": {
	///      "$ref": "#/components/schemas/schemas-SeppInfo"
	///    },
	///    "serviceSetRecoveryTimeList": {
	///      "description": "A map (list of key-value pairs) where
	/// NfServiceSetId serves as key of DateTime\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/DateTime"
	///      }
	///    },
	///    "servingScope": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "smfInfo": {
	///      "$ref": "#/components/schemas/schemas-SmfInfo"
	///    },
	///    "smfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of SmfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-SmfInfo"
	///      }
	///    },
	///    "snpnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnIdNid"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedVendorSpecificFeatures": {
	///      "description": "The key of the map is the IANA-assigned SMI Network
	/// Management Private Enterprise Codes\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "array",
	///        "items": {
	///          "$ref": "#/components/schemas/VendorSpecificFeature"
	///        },
	///        "minItems": 1
	///      }
	///    },
	///    "trustAfInfo": {
	///      "$ref": "#/components/schemas/schemas-TrustAfInfo"
	///    },
	///    "tsctsfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of TsctsfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-TsctsfInfo"
	///      }
	///    },
	///    "udmInfo": {
	///      "$ref": "#/components/schemas/schemas-UdmInfo"
	///    },
	///    "udmInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of UdmInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-UdmInfo"
	///      }
	///    },
	///    "udrInfo": {
	///      "$ref": "#/components/schemas/schemas-UdrInfo"
	///    },
	///    "udrInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of UdrInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-UdrInfo"
	///      }
	///    },
	///    "udsfInfo": {
	///      "$ref": "#/components/schemas/schemas-UdsfInfo"
	///    },
	///    "udsfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of UdsfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-UdsfInfo"
	///      }
	///    },
	///    "upfInfo": {
	///      "$ref": "#/components/schemas/schemas-UpfInfo"
	///    },
	///    "upfInfoList": {
	///      "description": "A map (list of key-value pairs) where a (unique)
	/// valid JSON string serves as key of UpfInfo\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-UpfInfo"
	///      }
	///    },
	///    "vendorId": {
	///      "$ref": "#/components/schemas/VendorId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfProfile {
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of AanfInfo
		#[serde(
			rename = "aanfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub aanf_info_list: ::std::collections::HashMap<String, AanfInfo>,
		#[serde(rename = "amfInfo", default, skip_serializing_if = "Option::is_none")]
		pub amf_info: Option<SchemasAmfInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of AmfInfo
		#[serde(
			rename = "amfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub amf_info_list: ::std::collections::HashMap<String, SchemasAmfInfo>,
		#[serde(rename = "ausfInfo", default, skip_serializing_if = "Option::is_none")]
		pub ausf_info: Option<SchemasAusfInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of AusfInfo
		#[serde(
			rename = "ausfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub ausf_info_list: ::std::collections::HashMap<String, SchemasAusfInfo>,
		#[serde(rename = "bsfInfo", default, skip_serializing_if = "Option::is_none")]
		pub bsf_info: Option<SchemasBsfInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of BsfInfo
		#[serde(
			rename = "bsfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub bsf_info_list: ::std::collections::HashMap<String, SchemasBsfInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub capacity: Option<u16>,
		#[serde(rename = "chfInfo", default, skip_serializing_if = "Option::is_none")]
		pub chf_info: Option<SchemasChfInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of ChfInfo
		#[serde(
			rename = "chfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub chf_info_list: ::std::collections::HashMap<String, SchemasChfInfo>,
		#[serde(
			rename = "collocatedNfInstances",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub collocated_nf_instances: Vec<SchemasCollocatedNfInstance>,
		#[serde(
			rename = "customInfo",
			default,
			skip_serializing_if = "::serde_json::Map::is_empty"
		)]
		pub custom_info: ::serde_json::Map<String, ::serde_json::Value>,
		#[serde(rename = "dccfInfo", default, skip_serializing_if = "Option::is_none")]
		pub dccf_info: Option<SchemasDccfInfo>,
		#[serde(
			rename = "defaultNotificationSubscriptions",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub default_notification_subscriptions: Vec<SchemasDefaultNotificationSubscription>,
		/// A map(list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of EasdfInfo
		#[serde(
			rename = "easdfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub easdf_info_list: ::std::collections::HashMap<String, SchemasEasdfInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub fqdn: Option<Fqdn>,
		#[serde(rename = "gmlcInfo", default, skip_serializing_if = "Option::is_none")]
		pub gmlc_info: Option<SchemasGmlcInfo>,
		#[serde(rename = "hniList", default, skip_serializing_if = "Vec::is_empty")]
		pub hni_list: Vec<Fqdn>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of HssInfo
		#[serde(
			rename = "hssInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub hss_info_list: ::std::collections::HashMap<String, SchemasHssInfo>,
		#[serde(
			rename = "interPlmnFqdn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_fqdn: Option<Fqdn>,
		#[serde(
			rename = "ipv4Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_addresses: Vec<Ipv4Addr>,
		#[serde(
			rename = "ipv6Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_addresses: Vec<Ipv6Addr>,
		#[serde(rename = "iwmscInfo", default, skip_serializing_if = "Option::is_none")]
		pub iwmsc_info: Option<SchemasIwmscInfo>,
		#[serde(rename = "lcHSupportInd", default)]
		pub lc_h_support_ind: bool,
		#[serde(rename = "lmfInfo", default, skip_serializing_if = "Option::is_none")]
		pub lmf_info: Option<SchemasLmfInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub load: Option<i64>,
		#[serde(
			rename = "loadTimeStamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub load_time_stamp: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub locality: Option<String>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of MbSmfInfo
		#[serde(
			rename = "mbSmfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub mb_smf_info_list: ::std::collections::HashMap<String, SchemasMbSmfInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of MbUpfInfo
		#[serde(
			rename = "mbUpfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub mb_upf_info_list: ::std::collections::HashMap<String, SchemasMbUpfInfo>,
		#[serde(rename = "mfafInfo", default, skip_serializing_if = "Option::is_none")]
		pub mfaf_info: Option<SchemasMfafInfo>,
		#[serde(rename = "mnpfInfo", default, skip_serializing_if = "Option::is_none")]
		pub mnpf_info: Option<MnpfInfo>,
		#[serde(rename = "nefInfo", default, skip_serializing_if = "Option::is_none")]
		pub nef_info: Option<SchemasNefInfo>,
		#[serde(rename = "nfInstanceId")]
		pub nf_instance_id: NfInstanceId,
		#[serde(
			rename = "nfInstanceName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nf_instance_name: Option<String>,
		/// A map (list of key-value pairs) where serviceInstanceId serves as
		/// key of NFService
		#[serde(
			rename = "nfServiceList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub nf_service_list: ::std::collections::HashMap<String, NfService>,
		#[serde(rename = "nfServicePersistence", default)]
		pub nf_service_persistence: bool,
		#[serde(rename = "nfServices", default, skip_serializing_if = "Vec::is_empty")]
		pub nf_services: Vec<NfService>,
		#[serde(rename = "nfSetIdList", default, skip_serializing_if = "Vec::is_empty")]
		pub nf_set_id_list: Vec<NfSetId>,
		/// A map (list of key-value pairs) where NfSetId serves as key of
		/// DateTime
		#[serde(
			rename = "nfSetRecoveryTimeList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub nf_set_recovery_time_list: ::std::collections::HashMap<String, DateTime>,
		#[serde(rename = "nfStatus")]
		pub nf_status: NfStatus,
		#[serde(rename = "nfType")]
		pub nf_type: NfType,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of NsacfInfo
		#[serde(
			rename = "nsacfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub nsacf_info_list: ::std::collections::HashMap<String, SchemasNsacfInfo>,
		#[serde(rename = "nsiList", default, skip_serializing_if = "Vec::is_empty")]
		pub nsi_list: Vec<String>,
		#[serde(
			rename = "nssaafInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nssaaf_info: Option<SchemasNssaafInfo>,
		#[serde(rename = "nwdafInfo", default, skip_serializing_if = "Option::is_none")]
		pub nwdaf_info: Option<SchemasNwdafInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of NwdafInfo
		#[serde(
			rename = "nwdafInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub nwdaf_info_list: ::std::collections::HashMap<String, SchemasNwdafInfo>,
		#[serde(rename = "olcHSupportInd", default)]
		pub olc_h_support_ind: bool,
		#[serde(rename = "pcfInfo", default, skip_serializing_if = "Option::is_none")]
		pub pcf_info: Option<SchemasPcfInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of PcfInfo
		#[serde(
			rename = "pcfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub pcf_info_list: ::std::collections::HashMap<String, SchemasPcfInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of PcscfInfo
		#[serde(
			rename = "pcscfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub pcscf_info_list: ::std::collections::HashMap<String, SchemasPcscfInfo>,
		#[serde(
			rename = "perPlmnSnssaiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub per_plmn_snssai_list: Vec<SchemasPlmnSnssai>,
		#[serde(rename = "plmnList", default, skip_serializing_if = "Vec::is_empty")]
		pub plmn_list: Vec<PlmnId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub priority: Option<u16>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
		#[serde(rename = "sNssais", default, skip_serializing_if = "Vec::is_empty")]
		pub s_nssais: Vec<ExtSnssai>,
		#[serde(rename = "scpDomains", default, skip_serializing_if = "Vec::is_empty")]
		pub scp_domains: Vec<String>,
		#[serde(rename = "scpInfo", default, skip_serializing_if = "Option::is_none")]
		pub scp_info: Option<SchemasScpInfo>,
		#[serde(rename = "seppInfo", default, skip_serializing_if = "Option::is_none")]
		pub sepp_info: Option<SchemasSeppInfo>,
		/// A map (list of key-value pairs) where NfServiceSetId serves as key
		/// of DateTime
		#[serde(
			rename = "serviceSetRecoveryTimeList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub service_set_recovery_time_list: ::std::collections::HashMap<String, DateTime>,
		#[serde(
			rename = "servingScope",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_scope: Vec<String>,
		#[serde(rename = "smfInfo", default, skip_serializing_if = "Option::is_none")]
		pub smf_info: Option<SchemasSmfInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of SmfInfo
		#[serde(
			rename = "smfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub smf_info_list: ::std::collections::HashMap<String, SchemasSmfInfo>,
		#[serde(rename = "snpnList", default, skip_serializing_if = "Vec::is_empty")]
		pub snpn_list: Vec<PlmnIdNid>,
		/// The key of the map is the IANA-assigned SMI Network Management
		/// Private Enterprise Codes
		#[serde(
			rename = "supportedVendorSpecificFeatures",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub supported_vendor_specific_features:
			::std::collections::HashMap<String, Vec<VendorSpecificFeature>>,
		#[serde(
			rename = "trustAfInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub trust_af_info: Option<SchemasTrustAfInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of TsctsfInfo
		#[serde(
			rename = "tsctsfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub tsctsf_info_list: ::std::collections::HashMap<String, SchemasTsctsfInfo>,
		#[serde(rename = "udmInfo", default, skip_serializing_if = "Option::is_none")]
		pub udm_info: Option<SchemasUdmInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of UdmInfo
		#[serde(
			rename = "udmInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub udm_info_list: ::std::collections::HashMap<String, SchemasUdmInfo>,
		#[serde(rename = "udrInfo", default, skip_serializing_if = "Option::is_none")]
		pub udr_info: Option<SchemasUdrInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of UdrInfo
		#[serde(
			rename = "udrInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub udr_info_list: ::std::collections::HashMap<String, SchemasUdrInfo>,
		#[serde(rename = "udsfInfo", default, skip_serializing_if = "Option::is_none")]
		pub udsf_info: Option<SchemasUdsfInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of UdsfInfo
		#[serde(
			rename = "udsfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub udsf_info_list: ::std::collections::HashMap<String, SchemasUdsfInfo>,
		#[serde(rename = "upfInfo", default, skip_serializing_if = "Option::is_none")]
		pub upf_info: Option<SchemasUpfInfo>,
		/// A map (list of key-value pairs) where a (unique) valid JSON string
		/// serves as key of UpfInfo
		#[serde(
			rename = "upfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub upf_info_list: ::std::collections::HashMap<String, SchemasUpfInfo>,
		#[serde(rename = "vendorId", default, skip_serializing_if = "Option::is_none")]
		pub vendor_id: Option<VendorId>,
	}

	impl From<&NfProfile> for NfProfile {
		fn from(value: &NfProfile) -> Self {
			value.clone()
		}
	}

	/// Information of a given NF Service Instance; it is part of the NFProfile
	/// of an NF Instance discovered by the NRF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a given NF Service Instance; it is part
	/// of the NFProfile of an NF Instance discovered by the NRF\n",
	///  "type": "object",
	///  "required": [
	///    "nfServiceStatus",
	///    "scheme",
	///    "serviceInstanceId",
	///    "serviceName",
	///    "versions"
	///  ],
	///  "properties": {
	///    "allowedOperationsPerNfInstance": {
	///      "description": "A map (list of key-value pairs) where NF Instance
	/// Id serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "array",
	///        "items": {
	///          "type": "string"
	///        },
	///        "minItems": 1
	///      }
	///    },
	///    "allowedOperationsPerNfType": {
	///      "description": "A map (list of key-value pairs) where NF Type
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "array",
	///        "items": {
	///          "type": "string"
	///        },
	///        "minItems": 1
	///      }
	///    },
	///    "apiPrefix": {
	///      "type": "string"
	///    },
	///    "capacity": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "defaultNotificationSubscriptions": {
	///      "type": "array",
	///      "items": {
	///        "$ref":
	/// "#/components/schemas/schemas-DefaultNotificationSubscription"
	///      },
	///      "minItems": 1
	///    },
	///    "fqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "interPlmnFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "ipEndPoints": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-IpEndPoint"
	///      },
	///      "minItems": 1
	///    },
	///    "load": {
	///      "type": "integer",
	///      "maximum": 100.0,
	///      "minimum": 0.0
	///    },
	///    "loadTimeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "nfServiceSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfServiceSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "nfServiceStatus": {
	///      "$ref": "#/components/schemas/NFServiceStatus"
	///    },
	///    "oauth2Required": {
	///      "type": "boolean"
	///    },
	///    "perPlmnSnssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-PlmnSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "priority": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "sNssais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "scheme": {
	///      "$ref": "#/components/schemas/UriScheme"
	///    },
	///    "serviceInstanceId": {
	///      "type": "string"
	///    },
	///    "serviceName": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "supportedVendorSpecificFeatures": {
	///      "description": "The key of the map is the IANA-assigned SMI Network
	/// Management Private Enterprise Codes\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "array",
	///        "items": {
	///          "$ref": "#/components/schemas/VendorSpecificFeature"
	///        },
	///        "minItems": 1
	///      }
	///    },
	///    "vendorId": {
	///      "$ref": "#/components/schemas/VendorId"
	///    },
	///    "versions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-NFServiceVersion"
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
	pub struct NfService {
		/// A map (list of key-value pairs) where NF Instance Id serves as key
		#[serde(
			rename = "allowedOperationsPerNfInstance",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub allowed_operations_per_nf_instance: ::std::collections::HashMap<String, Vec<String>>,
		/// A map (list of key-value pairs) where NF Type serves as key
		#[serde(
			rename = "allowedOperationsPerNfType",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub allowed_operations_per_nf_type: ::std::collections::HashMap<String, Vec<String>>,
		#[serde(rename = "apiPrefix", default, skip_serializing_if = "Option::is_none")]
		pub api_prefix: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub capacity: Option<u16>,
		#[serde(
			rename = "defaultNotificationSubscriptions",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub default_notification_subscriptions: Vec<SchemasDefaultNotificationSubscription>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub fqdn: Option<Fqdn>,
		#[serde(
			rename = "interPlmnFqdn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_fqdn: Option<Fqdn>,
		#[serde(rename = "ipEndPoints", default, skip_serializing_if = "Vec::is_empty")]
		pub ip_end_points: Vec<SchemasIpEndPoint>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub load: Option<i64>,
		#[serde(
			rename = "loadTimeStamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub load_time_stamp: Option<DateTime>,
		#[serde(
			rename = "nfServiceSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nf_service_set_id_list: Vec<NfServiceSetId>,
		#[serde(rename = "nfServiceStatus")]
		pub nf_service_status: NfServiceStatus,
		#[serde(
			rename = "oauth2Required",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub oauth2_required: Option<bool>,
		#[serde(
			rename = "perPlmnSnssaiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub per_plmn_snssai_list: Vec<SchemasPlmnSnssai>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub priority: Option<u16>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
		#[serde(rename = "sNssais", default, skip_serializing_if = "Vec::is_empty")]
		pub s_nssais: Vec<ExtSnssai>,
		pub scheme: UriScheme,
		#[serde(rename = "serviceInstanceId")]
		pub service_instance_id: String,
		#[serde(rename = "serviceName")]
		pub service_name: ServiceName,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		/// The key of the map is the IANA-assigned SMI Network Management
		/// Private Enterprise Codes
		#[serde(
			rename = "supportedVendorSpecificFeatures",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub supported_vendor_specific_features:
			::std::collections::HashMap<String, Vec<VendorSpecificFeature>>,
		#[serde(rename = "vendorId", default, skip_serializing_if = "Option::is_none")]
		pub vendor_id: Option<VendorId>,
		pub versions: Vec<SchemasNfServiceVersion>,
	}

	impl From<&NfService> for NfService {
		fn from(value: &NfService) -> Self {
			value.clone()
		}
	}

	/// Information of a given NF Service Instance; it is part of the NFProfile
	/// of an NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a given NF Service Instance; it is part
	/// of the NFProfile of an NF Instance\n",
	///  "type": "object",
	///  "required": [
	///    "nfServiceStatus",
	///    "scheme",
	///    "serviceInstanceId",
	///    "serviceName",
	///    "versions"
	///  ],
	///  "properties": {
	///    "allowedNfDomains": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "allowedNfTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFType"
	///      },
	///      "minItems": 1
	///    },
	///    "allowedNssais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "allowedOperationsPerNfInstance": {
	///      "description": "A map (list of key-value pairs) where NF Instance
	/// Id serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "array",
	///        "items": {
	///          "type": "string"
	///        },
	///        "minItems": 1
	///      }
	///    },
	///    "allowedOperationsPerNfType": {
	///      "description": "A map (list of key-value pairs) where NF Type
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "array",
	///        "items": {
	///          "type": "string"
	///        },
	///        "minItems": 1
	///      }
	///    },
	///    "allowedPlmns": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
	///      },
	///      "minItems": 1
	///    },
	///    "allowedSnpns": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnIdNid"
	///      },
	///      "minItems": 1
	///    },
	///    "apiPrefix": {
	///      "type": "string"
	///    },
	///    "capacity": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "defaultNotificationSubscriptions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DefaultNotificationSubscription"
	///      },
	///      "minItems": 1
	///    },
	///    "fqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "interPlmnFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "ipEndPoints": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpEndPoint"
	///      },
	///      "minItems": 1
	///    },
	///    "load": {
	///      "type": "integer",
	///      "maximum": 100.0,
	///      "minimum": 0.0
	///    },
	///    "loadTimeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "nfServiceSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfServiceSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "nfServiceStatus": {
	///      "$ref": "#/components/schemas/NFServiceStatus"
	///    },
	///    "oauth2Required": {
	///      "type": "boolean"
	///    },
	///    "perPlmnOauth2ReqList": {
	///      "$ref": "#/components/schemas/PlmnOauth2"
	///    },
	///    "perPlmnSnssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "priority": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "recoveryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "sNssais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "scheme": {
	///      "$ref": "#/components/schemas/UriScheme"
	///    },
	///    "serviceInstanceId": {
	///      "type": "string"
	///    },
	///    "serviceName": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "supportedVendorSpecificFeatures": {
	///      "description": "A map (list of key-value pairs) where IANA-assigned
	/// SMI Network Management Private Enterprise Codes serves as key\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "array",
	///        "items": {
	///          "$ref": "#/components/schemas/VendorSpecificFeature"
	///        },
	///        "minItems": 1
	///      }
	///    },
	///    "vendorId": {
	///      "$ref": "#/components/schemas/VendorId"
	///    },
	///    "versions": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFServiceVersion"
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
	pub struct NfService1 {
		#[serde(
			rename = "allowedNfDomains",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_nf_domains: Vec<String>,
		#[serde(
			rename = "allowedNfTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_nf_types: Vec<NfType>,
		#[serde(
			rename = "allowedNssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_nssais: Vec<ExtSnssai>,
		/// A map (list of key-value pairs) where NF Instance Id serves as key
		#[serde(
			rename = "allowedOperationsPerNfInstance",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub allowed_operations_per_nf_instance: ::std::collections::HashMap<String, Vec<String>>,
		/// A map (list of key-value pairs) where NF Type serves as key
		#[serde(
			rename = "allowedOperationsPerNfType",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub allowed_operations_per_nf_type: ::std::collections::HashMap<String, Vec<String>>,
		#[serde(
			rename = "allowedPlmns",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_plmns: Vec<PlmnId>,
		#[serde(
			rename = "allowedSnpns",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_snpns: Vec<PlmnIdNid>,
		#[serde(rename = "apiPrefix", default, skip_serializing_if = "Option::is_none")]
		pub api_prefix: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub capacity: Option<u16>,
		#[serde(
			rename = "defaultNotificationSubscriptions",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub default_notification_subscriptions: Vec<DefaultNotificationSubscription>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub fqdn: Option<Fqdn>,
		#[serde(
			rename = "interPlmnFqdn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_fqdn: Option<Fqdn>,
		#[serde(rename = "ipEndPoints", default, skip_serializing_if = "Vec::is_empty")]
		pub ip_end_points: Vec<IpEndPoint>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub load: Option<i64>,
		#[serde(
			rename = "loadTimeStamp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub load_time_stamp: Option<DateTime>,
		#[serde(
			rename = "nfServiceSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nf_service_set_id_list: Vec<NfServiceSetId>,
		#[serde(rename = "nfServiceStatus")]
		pub nf_service_status: NfServiceStatus,
		#[serde(
			rename = "oauth2Required",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub oauth2_required: Option<bool>,
		#[serde(
			rename = "perPlmnOauth2ReqList",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub per_plmn_oauth2_req_list: Option<PlmnOauth2>,
		#[serde(
			rename = "perPlmnSnssaiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub per_plmn_snssai_list: Vec<PlmnSnssai>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub priority: Option<u16>,
		#[serde(
			rename = "recoveryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub recovery_time: Option<DateTime>,
		#[serde(rename = "sNssais", default, skip_serializing_if = "Vec::is_empty")]
		pub s_nssais: Vec<ExtSnssai>,
		pub scheme: UriScheme,
		#[serde(rename = "serviceInstanceId")]
		pub service_instance_id: String,
		#[serde(rename = "serviceName")]
		pub service_name: ServiceName,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		/// A map (list of key-value pairs) where IANA-assigned SMI Network
		/// Management Private Enterprise Codes serves as key
		#[serde(
			rename = "supportedVendorSpecificFeatures",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub supported_vendor_specific_features:
			::std::collections::HashMap<String, Vec<VendorSpecificFeature>>,
		#[serde(rename = "vendorId", default, skip_serializing_if = "Option::is_none")]
		pub vendor_id: Option<VendorId>,
		pub versions: Vec<NfServiceVersion>,
	}

	impl From<&NfService1> for NfService1 {
		fn from(value: &NfService1) -> Self {
			value.clone()
		}
	}

	/// NF service instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "NF service instance",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "nfInstanceId"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "nfServiceSetId"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "nfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nfServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "serviceInstanceId": {
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
	pub enum NfServiceInstance {
		#[default]
		Variant0 {
			#[serde(rename = "nfInstanceId")]
			nf_instance_id: NfInstanceId,
			#[serde(
				rename = "serviceInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			service_instance_id: Option<String>,
		},
		Variant1 {
			#[serde(rename = "nfServiceSetId")]
			nf_service_set_id: NfServiceSetId,
			#[serde(
				rename = "serviceInstanceId",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			service_instance_id: Option<String>,
		},
	}

	impl From<&NfServiceInstance> for NfServiceInstance {
		fn from(value: &NfServiceInstance) -> Self {
			value.clone()
		}
	}

	/// Subscription to a set of NFs based on their Service Set Id
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NFs based on their Service Set
	/// Id",
	///  "type": "object",
	///  "required": [
	///    "nfServiceSetId"
	///  ],
	///  "properties": {
	///    "nfServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "nfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfServiceSetCond {
		#[serde(rename = "nfServiceSetId")]
		pub nf_service_set_id: NfServiceSetId,
		#[serde(rename = "nfSetId", default, skip_serializing_if = "Option::is_none")]
		pub nf_set_id: Option<NfSetId>,
	}

	impl From<&NfServiceSetCond> for NfServiceSetCond {
		fn from(value: &NfServiceSetCond) -> Self {
			value.clone()
		}
	}

	/// Status of a given NF Service Instance of an NF Instance stored in NRF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Status of a given NF Service Instance of an NF Instance
	/// stored in NRF",
	///  "type": "string",
	///  "enum": [
	///    "REGISTERED",
	///    "SUSPENDED",
	///    "UNDISCOVERABLE"
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
	pub enum NfServiceStatus {
		#[default]
		#[serde(rename = "REGISTERED")]
		Registered,
		#[serde(rename = "SUSPENDED")]
		Suspended,
		#[serde(rename = "UNDISCOVERABLE")]
		Undiscoverable,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NfServiceStatus> for NfServiceStatus {
		fn from(value: &NfServiceStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for NfServiceStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Registered => "REGISTERED".to_string(),
				Self::Suspended => "SUSPENDED".to_string(),
				Self::Undiscoverable => "UNDISCOVERABLE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NfServiceStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REGISTERED" => Ok(Self::Registered),
				"SUSPENDED" => Ok(Self::Suspended),
				"UNDISCOVERABLE" => Ok(Self::Undiscoverable),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NfServiceStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NfServiceStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NfServiceStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the version details of an NF service
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the version details of an NF service",
	///  "type": "object",
	///  "required": [
	///    "apiFullVersion",
	///    "apiVersionInUri"
	///  ],
	///  "properties": {
	///    "apiFullVersion": {
	///      "type": "string"
	///    },
	///    "apiVersionInUri": {
	///      "type": "string"
	///    },
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfServiceVersion {
		#[serde(rename = "apiFullVersion")]
		pub api_full_version: String,
		#[serde(rename = "apiVersionInUri")]
		pub api_version_in_uri: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DateTime>,
	}

	impl From<&NfServiceVersion> for NfServiceVersion {
		fn from(value: &NfServiceVersion) -> Self {
			value.clone()
		}
	}

	/// Subscription to a set of NFs based on their Set Id
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NFs based on their Set Id",
	///  "type": "object",
	///  "required": [
	///    "nfSetId"
	///  ],
	///  "properties": {
	///    "nfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfSetCond {
		#[serde(rename = "nfSetId")]
		pub nf_set_id: NfSetId,
	}

	impl From<&NfSetCond> for NfSetCond {
		fn from(value: &NfSetCond) -> Self {
			value.clone()
		}
	}

	/// Status of a given NF Instance stored in NRF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Status of a given NF Instance stored in NRF",
	///  "type": "string",
	///  "enum": [
	///    "REGISTERED",
	///    "SUSPENDED",
	///    "UNDISCOVERABLE"
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
	pub enum NfStatus {
		#[default]
		#[serde(rename = "REGISTERED")]
		Registered,
		#[serde(rename = "SUSPENDED")]
		Suspended,
		#[serde(rename = "UNDISCOVERABLE")]
		Undiscoverable,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NfStatus> for NfStatus {
		fn from(value: &NfStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for NfStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Registered => "REGISTERED".to_string(),
				Self::Suspended => "SUSPENDED".to_string(),
				Self::Undiscoverable => "UNDISCOVERABLE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NfStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REGISTERED" => Ok(Self::Registered),
				"SUSPENDED" => Ok(Self::Suspended),
				"UNDISCOVERABLE" => Ok(Self::Undiscoverable),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NfStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NfStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NfStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Subscription to a set of NFs based on their NF Type
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NFs based on their NF Type",
	///  "type": "object",
	///  "not": {
	///    "required": [
	///      "nfGroupId"
	///    ]
	///  },
	///  "required": [
	///    "nfType"
	///  ],
	///  "properties": {
	///    "nfType": {
	///      "$ref": "#/components/schemas/NFType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NfTypeCond {
		#[serde(rename = "nfType")]
		pub nf_type: NfType,
	}

	impl From<&NfTypeCond> for NfTypeCond {
		fn from(value: &NfTypeCond) -> Self {
			value.clone()
		}
	}

	/// Provides the reason for not finding NF matching the search criteria
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Provides the reason for not finding NF matching the
	/// search criteria",
	///  "type": "object",
	///  "required": [
	///    "reason"
	///  ],
	///  "properties": {
	///    "queryParamCombinationList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QueryParamCombination"
	///      },
	///      "minItems": 1
	///    },
	///    "reason": {
	///      "$ref": "#/components/schemas/NoProfileMatchReason"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NoProfileMatchInfo {
		#[serde(
			rename = "queryParamCombinationList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub query_param_combination_list: Vec<QueryParamCombination>,
		pub reason: NoProfileMatchReason,
	}

	impl From<&NoProfileMatchInfo> for NoProfileMatchInfo {
		fn from(value: &NoProfileMatchInfo) -> Self {
			value.clone()
		}
	}

	/// No Profile Match Reason
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "No Profile Match Reason",
	///  "type": "string",
	///  "enum": [
	///    "REQUESTER_PLMN_NOT_ALLOWED",
	///    "TARGET_NF_SUSPENDED",
	///    "TARGET_NF_UNDISCOVERABLE",
	///    "QUERY_PARAMS_COMBINATION_NO_MATCH",
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
	pub enum NoProfileMatchReason {
		#[default]
		#[serde(rename = "REQUESTER_PLMN_NOT_ALLOWED")]
		RequesterPlmnNotAllowed,
		#[serde(rename = "TARGET_NF_SUSPENDED")]
		TargetNfSuspended,
		#[serde(rename = "TARGET_NF_UNDISCOVERABLE")]
		TargetNfUndiscoverable,
		#[serde(rename = "QUERY_PARAMS_COMBINATION_NO_MATCH")]
		QueryParamsCombinationNoMatch,
		#[serde(rename = "UNSPECIFIED")]
		Unspecified,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NoProfileMatchReason> for NoProfileMatchReason {
		fn from(value: &NoProfileMatchReason) -> Self {
			value.clone()
		}
	}

	impl ToString for NoProfileMatchReason {
		fn to_string(&self) -> String {
			match *self {
				Self::RequesterPlmnNotAllowed => "REQUESTER_PLMN_NOT_ALLOWED".to_string(),
				Self::TargetNfSuspended => "TARGET_NF_SUSPENDED".to_string(),
				Self::TargetNfUndiscoverable => "TARGET_NF_UNDISCOVERABLE".to_string(),
				Self::QueryParamsCombinationNoMatch => {
					"QUERY_PARAMS_COMBINATION_NO_MATCH".to_string()
				}
				Self::Unspecified => "UNSPECIFIED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NoProfileMatchReason {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REQUESTER_PLMN_NOT_ALLOWED" => Ok(Self::RequesterPlmnNotAllowed),
				"TARGET_NF_SUSPENDED" => Ok(Self::TargetNfSuspended),
				"TARGET_NF_UNDISCOVERABLE" => Ok(Self::TargetNfUndiscoverable),
				"QUERY_PARAMS_COMBINATION_NO_MATCH" => Ok(Self::QueryParamsCombinationNoMatch),
				"UNSPECIFIED" => Ok(Self::Unspecified),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NoProfileMatchReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NoProfileMatchReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NoProfileMatchReason {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Condition (list of attributes in the NF Profile) to determine whether a
	/// notification must be sent by NRF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Condition (list of attributes in the NF Profile) to
	/// determine whether a notification must be sent by NRF\n",
	///  "type": "object",
	///  "not": {
	///    "required": [
	///      "monitoredAttributes",
	///      "unmonitoredAttributes"
	///    ]
	///  },
	///  "properties": {
	///    "monitoredAttributes": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "unmonitoredAttributes": {
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
	pub struct NotifCondition {}

	impl From<&NotifCondition> for NotifCondition {
		fn from(value: &NotifCondition) -> Self {
			value.clone()
		}
	}

	/// Data sent in notifications from NRF to subscribed NF Instances
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data sent in notifications from NRF to subscribed NF
	/// Instances",
	///  "type": "object",
	///  "allOf": [
	///    {
	///      "anyOf": [
	///        {
	///          "not": {
	///            "properties": {
	///              "event": {
	///                "type": "string",
	///                "enum": [
	///                  "NF_PROFILE_CHANGED"
	///                ]
	///              }
	///            }
	///          }
	///        },
	///        {
	///          "oneOf": [
	///            {
	///              "required": [
	///                "nfProfile"
	///              ]
	///            },
	///            {
	///              "required": [
	///                "profileChanges"
	///              ]
	///            }
	///          ]
	///        }
	///      ]
	///    },
	///    {
	///      "anyOf": [
	///        {
	///          "not": {
	///            "properties": {
	///              "event": {
	///                "type": "string",
	///                "enum": [
	///                  "NF_REGISTERED"
	///                ]
	///              }
	///            }
	///          }
	///        },
	///        {
	///          "required": [
	///            "nfProfile"
	///          ]
	///        }
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "event",
	///    "nfInstanceUri"
	///  ],
	///  "properties": {
	///    "conditionEvent": {
	///      "$ref": "#/components/schemas/ConditionEventType"
	///    },
	///    "event": {
	///      "$ref": "#/components/schemas/NotificationEventType"
	///    },
	///    "nfInstanceUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nfProfile": {
	///      "allOf": [
	///        {
	///          "$ref": "#/components/schemas/NFProfile1"
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "allowedPlmns"
	///            ]
	///          }
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "allowedSnpns"
	///            ]
	///          }
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "allowedNfTypes"
	///            ]
	///          }
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "allowedNfDomains"
	///            ]
	///          }
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "allowedNssais"
	///            ]
	///          }
	///        },
	///        {
	///          "properties": {
	///            "nfServices": {
	///              "type": "array",
	///              "items": {
	///                "allOf": [
	///                  {
	///                    "$ref": "#/components/schemas/NFService1"
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "allowedPlmns"
	///                      ]
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "allowedSnpns"
	///                      ]
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "allowedNfTypes"
	///                      ]
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "allowedNfDomains"
	///                      ]
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "allowedNssais"
	///                      ]
	///                    }
	///                  }
	///                ]
	///              }
	///            }
	///          }
	///        }
	///      ]
	///    },
	///    "profileChanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ChangeItem"
	///      },
	///      "minItems": 1
	///    },
	///    "subscriptionContext": {
	///      "$ref": "#/components/schemas/SubscriptionContext"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NotificationData {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<NotificationDataSubtype0>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<NotificationDataSubtype1>,
	}

	impl From<&NotificationData> for NotificationData {
		fn from(value: &NotificationData) -> Self {
			value.clone()
		}
	}

	/// NotificationDataSubtype0
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "event",
	///    "nfInstanceUri"
	///  ],
	///  "properties": {
	///    "conditionEvent": {
	///      "$ref": "#/components/schemas/ConditionEventType"
	///    },
	///    "event": {
	///      "$ref": "#/components/schemas/NotificationEventType"
	///    },
	///    "nfInstanceUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nfProfile": {
	///      "allOf": [
	///        {
	///          "$ref": "#/components/schemas/NFProfile1"
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "allowedPlmns"
	///            ]
	///          }
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "allowedSnpns"
	///            ]
	///          }
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "allowedNfTypes"
	///            ]
	///          }
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "allowedNfDomains"
	///            ]
	///          }
	///        },
	///        {
	///          "not": {
	///            "required": [
	///              "allowedNssais"
	///            ]
	///          }
	///        },
	///        {
	///          "properties": {
	///            "nfServices": {
	///              "type": "array",
	///              "items": {
	///                "allOf": [
	///                  {
	///                    "$ref": "#/components/schemas/NFService1"
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "allowedPlmns"
	///                      ]
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "allowedSnpns"
	///                      ]
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "allowedNfTypes"
	///                      ]
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "allowedNfDomains"
	///                      ]
	///                    }
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "allowedNssais"
	///                      ]
	///                    }
	///                  }
	///                ]
	///              }
	///            }
	///          }
	///        }
	///      ]
	///    },
	///    "profileChanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ChangeItem"
	///      },
	///      "minItems": 1
	///    },
	///    "subscriptionContext": {
	///      "$ref": "#/components/schemas/SubscriptionContext"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NotificationDataSubtype0 {
		#[serde(
			rename = "conditionEvent",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub condition_event: Option<ConditionEventType>,
		pub event: NotificationEventType,
		#[serde(rename = "nfInstanceUri")]
		pub nf_instance_uri: Uri,
		#[serde(rename = "nfProfile", default, skip_serializing_if = "Option::is_none")]
		pub nf_profile: Option<NotificationDataSubtype0NfProfile>,
		#[serde(
			rename = "profileChanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub profile_changes: Vec<ChangeItem>,
		#[serde(
			rename = "subscriptionContext",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscription_context: Option<SubscriptionContext>,
	}

	impl From<&NotificationDataSubtype0> for NotificationDataSubtype0 {
		fn from(value: &NotificationDataSubtype0) -> Self {
			value.clone()
		}
	}

	/// NotificationDataSubtype0NfProfile
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/NFProfile1"
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "allowedPlmns"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "allowedSnpns"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "allowedNfTypes"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "allowedNfDomains"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "allowedNssais"
	///        ]
	///      }
	///    },
	///    {
	///      "properties": {
	///        "nfServices": {
	///          "type": "array",
	///          "items": {
	///            "allOf": [
	///              {
	///                "$ref": "#/components/schemas/NFService1"
	///              },
	///              {
	///                "not": {
	///                  "required": [
	///                    "allowedPlmns"
	///                  ]
	///                }
	///              },
	///              {
	///                "not": {
	///                  "required": [
	///                    "allowedSnpns"
	///                  ]
	///                }
	///              },
	///              {
	///                "not": {
	///                  "required": [
	///                    "allowedNfTypes"
	///                  ]
	///                }
	///              },
	///              {
	///                "not": {
	///                  "required": [
	///                    "allowedNfDomains"
	///                  ]
	///                }
	///              },
	///              {
	///                "not": {
	///                  "required": [
	///                    "allowedNssais"
	///                  ]
	///                }
	///              }
	///            ]
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
	pub enum NotificationDataSubtype0NfProfile {
		#[default]
		None,
	}

	impl From<&NotificationDataSubtype0NfProfile> for NotificationDataSubtype0NfProfile {
		fn from(value: &NotificationDataSubtype0NfProfile) -> Self {
			value.clone()
		}
	}

	/// NotificationDataSubtype1
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "type": "object",
	///      "required": [
	///        "event",
	///        "nfInstanceUri"
	///      ],
	///      "properties": {
	///        "conditionEvent": {
	///          "$ref": "#/components/schemas/ConditionEventType"
	///        },
	///        "event": {
	///          "$ref": "#/components/schemas/NotificationEventType"
	///        },
	///        "nfInstanceUri": {
	///          "$ref": "#/components/schemas/Uri"
	///        },
	///        "nfProfile": {
	///          "allOf": [
	///            {
	///              "$ref": "#/components/schemas/NFProfile1"
	///            },
	///            {
	///              "not": {
	///                "required": [
	///                  "allowedPlmns"
	///                ]
	///              }
	///            },
	///            {
	///              "not": {
	///                "required": [
	///                  "allowedSnpns"
	///                ]
	///              }
	///            },
	///            {
	///              "not": {
	///                "required": [
	///                  "allowedNfTypes"
	///                ]
	///              }
	///            },
	///            {
	///              "not": {
	///                "required": [
	///                  "allowedNfDomains"
	///                ]
	///              }
	///            },
	///            {
	///              "not": {
	///                "required": [
	///                  "allowedNssais"
	///                ]
	///              }
	///            },
	///            {
	///              "properties": {
	///                "nfServices": {
	///                  "type": "array",
	///                  "items": {
	///                    "allOf": [
	///                      {
	///                        "$ref": "#/components/schemas/NFService1"
	///                      },
	///                      {
	///                        "not": {
	///                          "required": [
	///                            "allowedPlmns"
	///                          ]
	///                        }
	///                      },
	///                      {
	///                        "not": {
	///                          "required": [
	///                            "allowedSnpns"
	///                          ]
	///                        }
	///                      },
	///                      {
	///                        "not": {
	///                          "required": [
	///                            "allowedNfTypes"
	///                          ]
	///                        }
	///                      },
	///                      {
	///                        "not": {
	///                          "required": [
	///                            "allowedNfDomains"
	///                          ]
	///                        }
	///                      },
	///                      {
	///                        "not": {
	///                          "required": [
	///                            "allowedNssais"
	///                          ]
	///                        }
	///                      }
	///                    ]
	///                  }
	///                }
	///              }
	///            }
	///          ]
	///        },
	///        "profileChanges": {
	///          "type": "array",
	///          "items": {
	///            "$ref": "#/components/schemas/ChangeItem"
	///          },
	///          "minItems": 1
	///        },
	///        "subscriptionContext": {
	///          "$ref": "#/components/schemas/SubscriptionContext"
	///        }
	///      }
	///    },
	///    {
	///      "required": [
	///        "nfProfile"
	///      ]
	///    },
	///    {
	///      "not": {
	///        "not": {
	///          "properties": {
	///            "event": {
	///              "type": "string",
	///              "enum": [
	///                "NF_REGISTERED"
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
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NotificationDataSubtype1 {
		#[serde(
			rename = "conditionEvent",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub condition_event: Option<ConditionEventType>,
		pub event: NotificationDataSubtype1Event,
		#[serde(rename = "nfInstanceUri")]
		pub nf_instance_uri: Uri,
		#[serde(rename = "nfProfile")]
		pub nf_profile: NotificationDataSubtype1NfProfile,
		#[serde(
			rename = "profileChanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub profile_changes: Vec<ChangeItem>,
		#[serde(
			rename = "subscriptionContext",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscription_context: Option<SubscriptionContext>,
	}

	impl From<&NotificationDataSubtype1> for NotificationDataSubtype1 {
		fn from(value: &NotificationDataSubtype1) -> Self {
			value.clone()
		}
	}

	/// NotificationDataSubtype1Event
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "NF_REGISTERED"
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
	pub enum NotificationDataSubtype1Event {
		#[default]
		#[serde(rename = "NF_REGISTERED")]
		NfRegistered,
	}

	impl From<&NotificationDataSubtype1Event> for NotificationDataSubtype1Event {
		fn from(value: &NotificationDataSubtype1Event) -> Self {
			value.clone()
		}
	}

	impl ToString for NotificationDataSubtype1Event {
		fn to_string(&self) -> String {
			match *self {
				Self::NfRegistered => "NF_REGISTERED".to_string(),
			}
		}
	}

	impl std::str::FromStr for NotificationDataSubtype1Event {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NF_REGISTERED" => Ok(Self::NfRegistered),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NotificationDataSubtype1Event {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NotificationDataSubtype1Event {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NotificationDataSubtype1Event {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// NotificationDataSubtype1NfProfile
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/NFProfile1"
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "allowedPlmns"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "allowedSnpns"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "allowedNfTypes"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "allowedNfDomains"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "allowedNssais"
	///        ]
	///      }
	///    },
	///    {
	///      "properties": {
	///        "nfServices": {
	///          "type": "array",
	///          "items": {
	///            "allOf": [
	///              {
	///                "$ref": "#/components/schemas/NFService1"
	///              },
	///              {
	///                "not": {
	///                  "required": [
	///                    "allowedPlmns"
	///                  ]
	///                }
	///              },
	///              {
	///                "not": {
	///                  "required": [
	///                    "allowedSnpns"
	///                  ]
	///                }
	///              },
	///              {
	///                "not": {
	///                  "required": [
	///                    "allowedNfTypes"
	///                  ]
	///                }
	///              },
	///              {
	///                "not": {
	///                  "required": [
	///                    "allowedNfDomains"
	///                  ]
	///                }
	///              },
	///              {
	///                "not": {
	///                  "required": [
	///                    "allowedNssais"
	///                  ]
	///                }
	///              }
	///            ]
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
	pub enum NotificationDataSubtype1NfProfile {
		#[default]
		None,
	}

	impl From<&NotificationDataSubtype1NfProfile> for NotificationDataSubtype1NfProfile {
		fn from(value: &NotificationDataSubtype1NfProfile) -> Self {
			value.clone()
		}
	}

	/// Types of events sent in notifications from NRF to subscribed NF
	/// Instances
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Types of events sent in notifications from NRF to
	/// subscribed NF Instances",
	///  "type": "string",
	///  "enum": [
	///    "NF_REGISTERED",
	///    "NF_DEREGISTERED",
	///    "NF_PROFILE_CHANGED"
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
	pub enum NotificationEventType {
		#[default]
		#[serde(rename = "NF_REGISTERED")]
		NfRegistered,
		#[serde(rename = "NF_DEREGISTERED")]
		NfDeregistered,
		#[serde(rename = "NF_PROFILE_CHANGED")]
		NfProfileChanged,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NotificationEventType> for NotificationEventType {
		fn from(value: &NotificationEventType) -> Self {
			value.clone()
		}
	}

	impl ToString for NotificationEventType {
		fn to_string(&self) -> String {
			match *self {
				Self::NfRegistered => "NF_REGISTERED".to_string(),
				Self::NfDeregistered => "NF_DEREGISTERED".to_string(),
				Self::NfProfileChanged => "NF_PROFILE_CHANGED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NotificationEventType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NF_REGISTERED" => Ok(Self::NfRegistered),
				"NF_DEREGISTERED" => Ok(Self::NfDeregistered),
				"NF_PROFILE_CHANGED" => Ok(Self::NfProfileChanged),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NotificationEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NotificationEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NotificationEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Types of notifications used in Default Notification URIs in the NF
	/// Profile of an NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Types of notifications used in Default Notification
	/// URIs in the NF Profile of an NF Instance\n",
	///  "type": "string",
	///  "enum": [
	///    "N1_MESSAGES",
	///    "N2_INFORMATION",
	///    "LOCATION_NOTIFICATION",
	///    "DATA_REMOVAL_NOTIFICATION",
	///    "DATA_CHANGE_NOTIFICATION",
	///    "LOCATION_UPDATE_NOTIFICATION",
	///    "NSSAA_REAUTH_NOTIFICATION",
	///    "NSSAA_REVOC_NOTIFICATION",
	///    "MATCH_INFO_NOTIFICATION",
	///    "DATA_RESTORATION_NOTIFICATION",
	///    "TSCTS_NOTIFICATION",
	///    "LCS_KEY_DELIVERY_NOTIFICATION",
	///    "UUAA_MM_AUTH_NOTIFICATION"
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
		#[serde(rename = "N1_MESSAGES")]
		N1Messages,
		#[serde(rename = "N2_INFORMATION")]
		N2Information,
		#[serde(rename = "LOCATION_NOTIFICATION")]
		LocationNotification,
		#[serde(rename = "DATA_REMOVAL_NOTIFICATION")]
		DataRemovalNotification,
		#[serde(rename = "DATA_CHANGE_NOTIFICATION")]
		DataChangeNotification,
		#[serde(rename = "LOCATION_UPDATE_NOTIFICATION")]
		LocationUpdateNotification,
		#[serde(rename = "NSSAA_REAUTH_NOTIFICATION")]
		NssaaReauthNotification,
		#[serde(rename = "NSSAA_REVOC_NOTIFICATION")]
		NssaaRevocNotification,
		#[serde(rename = "MATCH_INFO_NOTIFICATION")]
		MatchInfoNotification,
		#[serde(rename = "DATA_RESTORATION_NOTIFICATION")]
		DataRestorationNotification,
		#[serde(rename = "TSCTS_NOTIFICATION")]
		TsctsNotification,
		#[serde(rename = "LCS_KEY_DELIVERY_NOTIFICATION")]
		LcsKeyDeliveryNotification,
		#[serde(rename = "UUAA_MM_AUTH_NOTIFICATION")]
		UuaaMmAuthNotification,
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
				Self::N1Messages => "N1_MESSAGES".to_string(),
				Self::N2Information => "N2_INFORMATION".to_string(),
				Self::LocationNotification => "LOCATION_NOTIFICATION".to_string(),
				Self::DataRemovalNotification => "DATA_REMOVAL_NOTIFICATION".to_string(),
				Self::DataChangeNotification => "DATA_CHANGE_NOTIFICATION".to_string(),
				Self::LocationUpdateNotification => "LOCATION_UPDATE_NOTIFICATION".to_string(),
				Self::NssaaReauthNotification => "NSSAA_REAUTH_NOTIFICATION".to_string(),
				Self::NssaaRevocNotification => "NSSAA_REVOC_NOTIFICATION".to_string(),
				Self::MatchInfoNotification => "MATCH_INFO_NOTIFICATION".to_string(),
				Self::DataRestorationNotification => "DATA_RESTORATION_NOTIFICATION".to_string(),
				Self::TsctsNotification => "TSCTS_NOTIFICATION".to_string(),
				Self::LcsKeyDeliveryNotification => "LCS_KEY_DELIVERY_NOTIFICATION".to_string(),
				Self::UuaaMmAuthNotification => "UUAA_MM_AUTH_NOTIFICATION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NotificationType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"N1_MESSAGES" => Ok(Self::N1Messages),
				"N2_INFORMATION" => Ok(Self::N2Information),
				"LOCATION_NOTIFICATION" => Ok(Self::LocationNotification),
				"DATA_REMOVAL_NOTIFICATION" => Ok(Self::DataRemovalNotification),
				"DATA_CHANGE_NOTIFICATION" => Ok(Self::DataChangeNotification),
				"LOCATION_UPDATE_NOTIFICATION" => Ok(Self::LocationUpdateNotification),
				"NSSAA_REAUTH_NOTIFICATION" => Ok(Self::NssaaReauthNotification),
				"NSSAA_REVOC_NOTIFICATION" => Ok(Self::NssaaRevocNotification),
				"MATCH_INFO_NOTIFICATION" => Ok(Self::MatchInfoNotification),
				"DATA_RESTORATION_NOTIFICATION" => Ok(Self::DataRestorationNotification),
				"TSCTS_NOTIFICATION" => Ok(Self::TsctsNotification),
				"LCS_KEY_DELIVERY_NOTIFICATION" => Ok(Self::LcsKeyDeliveryNotification),
				"UUAA_MM_AUTH_NOTIFICATION" => Ok(Self::UuaaMmAuthNotification),
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

	/// Information of an NRF NF Instance, used in hierarchical NRF deployments
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an NRF NF Instance, used in hierarchical
	/// NRF deployments",
	///  "type": "object",
	///  "properties": {
	///    "served5gDdnmfInfo": {
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/5GDdnmfInfo"
	///      }
	///    },
	///    "servedAanfInfoList": {
	///      "description": "A map (list of key-value pairs) where NF Instance
	/// Id serves as key",
	///      "type": "object",
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/AanfInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedAmfInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/AmfInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedAmfInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/AmfInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedAusfInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/AusfInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedAusfInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/AusfInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedBsfInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/BsfInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedBsfInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/BsfInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedChfInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/ChfInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedChfInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/ChfInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedDccfInfoList": {
	///      "description": "A map (list of key-value pairs) where NF Instance
	/// Id serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/DccfInfo"
	///      }
	///    },
	///    "servedEasdfInfoList": {
	///      "description": "A map (list of key-value pairs) where NF Instance
	/// Id serves as key",
	///      "type": "object",
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "$ref": "#/components/schemas/EasdfInfo"
	///        }
	///      }
	///    },
	///    "servedGmlcInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/GmlcInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedHssInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/HssInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedLmfInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/LmfInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedMbSmfInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/MbSmfInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedMbUpfInfoList": {
	///      "description": "A map (list of key-value pairs) where NF Instance
	/// Id serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "$ref": "#/components/schemas/MbUpfInfo"
	///        }
	///      }
	///    },
	///    "servedMfafInfoList": {
	///      "description": "A map (list of key-value pairs) where NF Instance
	/// Id serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MfafInfo"
	///      }
	///    },
	///    "servedNefInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/NefInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedNfInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/NfInfo"
	///      }
	///    },
	///    "servedNssaafInfo": {
	///      "description": "A map (list of key-value pairs) where NF Instance
	/// Id serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/NssaafInfo"
	///      }
	///    },
	///    "servedNwdafInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/NwdafInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedNwdafInfoList": {
	///      "description": "A map (list of key-value pairs) where NF Instance
	/// Id serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "$ref": "#/components/schemas/NwdafInfo"
	///        }
	///      }
	///    },
	///    "servedPcfInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/PcfInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedPcfInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/PcfInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedPcscfInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/PcscfInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedScpInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/ScpInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedSeppInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/SeppInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedSmfInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/SmfInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedSmfInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/SmfInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedTrustAfInfo": {
	///      "description": "A map (list of key-value pairs) where NF Instance
	/// Id serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/TrustAfInfo"
	///      }
	///    },
	///    "servedTsctsfInfoList": {
	///      "description": "A map (list of key-value pairs) where NF Instance
	/// Id serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "$ref": "#/components/schemas/TsctsfInfo"
	///        }
	///      }
	///    },
	///    "servedUdmInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/UdmInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedUdmInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/UdmInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedUdrInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/UdrInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedUdrInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/UdrInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedUdsfInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/UdsfInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedUdsfInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/UdsfInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    },
	///    "servedUpfInfo": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/UpfInfo"
	///          },
	///          {
	///            "$ref": "#/components/schemas/EmptyObject"
	///          }
	///        ]
	///      }
	///    },
	///    "servedUpfInfoList": {
	///      "description": "A map (list of key-value pairs) where nfInstanceId
	/// serves as key",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "description": "A map (list of key-value pairs) where a valid
	/// JSON string serves as key",
	///        "type": "object",
	///        "minProperties": 1,
	///        "additionalProperties": {
	///          "anyOf": [
	///            {
	///              "$ref": "#/components/schemas/UpfInfo"
	///            },
	///            {
	///              "$ref": "#/components/schemas/EmptyObject"
	///            }
	///          ]
	///        }
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfo {
		#[serde(
			rename = "served5gDdnmfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served5g_ddnmf_info: ::std::collections::HashMap<String, _5gDdnmfInfo>,
		/// A map (list of key-value pairs) where NF Instance Id serves as key
		#[serde(
			rename = "servedAanfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_aanf_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedAanfInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedAmfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_amf_info: ::std::collections::HashMap<String, NrfInfoServedAmfInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedAmfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_amf_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedAmfInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedAusfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_ausf_info: ::std::collections::HashMap<String, NrfInfoServedAusfInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedAusfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_ausf_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedAusfInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedBsfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_bsf_info: ::std::collections::HashMap<String, NrfInfoServedBsfInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedBsfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_bsf_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedBsfInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedChfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_chf_info: ::std::collections::HashMap<String, NrfInfoServedChfInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedChfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_chf_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedChfInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where NF Instance Id serves as key
		#[serde(
			rename = "servedDccfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_dccf_info_list: ::std::collections::HashMap<String, DccfInfo>,
		/// A map (list of key-value pairs) where NF Instance Id serves as key
		#[serde(
			rename = "servedEasdfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_easdf_info_list:
			::std::collections::HashMap<String, ::std::collections::HashMap<String, EasdfInfo>>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedGmlcInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_gmlc_info: ::std::collections::HashMap<String, NrfInfoServedGmlcInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedHssInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_hss_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedHssInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedLmfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_lmf_info: ::std::collections::HashMap<String, NrfInfoServedLmfInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedMbSmfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_mb_smf_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedMbSmfInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where NF Instance Id serves as key
		#[serde(
			rename = "servedMbUpfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_mb_upf_info_list:
			::std::collections::HashMap<String, ::std::collections::HashMap<String, MbUpfInfo>>,
		/// A map (list of key-value pairs) where NF Instance Id serves as key
		#[serde(
			rename = "servedMfafInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_mfaf_info_list: ::std::collections::HashMap<String, MfafInfo>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedNefInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_nef_info: ::std::collections::HashMap<String, NrfInfoServedNefInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedNfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_nf_info: ::std::collections::HashMap<String, NfInfo>,
		/// A map (list of key-value pairs) where NF Instance Id serves as key
		#[serde(
			rename = "servedNssaafInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_nssaaf_info: ::std::collections::HashMap<String, NssaafInfo>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedNwdafInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_nwdaf_info: ::std::collections::HashMap<String, NrfInfoServedNwdafInfoValue>,
		/// A map (list of key-value pairs) where NF Instance Id serves as key
		#[serde(
			rename = "servedNwdafInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_nwdaf_info_list:
			::std::collections::HashMap<String, ::std::collections::HashMap<String, NwdafInfo>>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedPcfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_pcf_info: ::std::collections::HashMap<String, NrfInfoServedPcfInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedPcfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_pcf_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedPcfInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedPcscfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_pcscf_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedPcscfInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedScpInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_scp_info_list:
			::std::collections::HashMap<String, NrfInfoServedScpInfoListValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedSeppInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_sepp_info_list:
			::std::collections::HashMap<String, NrfInfoServedSeppInfoListValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedSmfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_smf_info: ::std::collections::HashMap<String, NrfInfoServedSmfInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedSmfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_smf_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedSmfInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where NF Instance Id serves as key
		#[serde(
			rename = "servedTrustAfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_trust_af_info: ::std::collections::HashMap<String, TrustAfInfo>,
		/// A map (list of key-value pairs) where NF Instance Id serves as key
		#[serde(
			rename = "servedTsctsfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_tsctsf_info_list:
			::std::collections::HashMap<String, ::std::collections::HashMap<String, TsctsfInfo>>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedUdmInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_udm_info: ::std::collections::HashMap<String, NrfInfoServedUdmInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedUdmInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_udm_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedUdmInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedUdrInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_udr_info: ::std::collections::HashMap<String, NrfInfoServedUdrInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedUdrInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_udr_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedUdrInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedUdsfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_udsf_info: ::std::collections::HashMap<String, NrfInfoServedUdsfInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedUdsfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_udsf_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedUdsfInfoListValueValue>,
		>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedUpfInfo",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_upf_info: ::std::collections::HashMap<String, NrfInfoServedUpfInfoValue>,
		/// A map (list of key-value pairs) where nfInstanceId serves as key
		#[serde(
			rename = "servedUpfInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub served_upf_info_list: ::std::collections::HashMap<
			String,
			::std::collections::HashMap<String, NrfInfoServedUpfInfoListValueValue>,
		>,
	}

	impl From<&NrfInfo> for NrfInfo {
		fn from(value: &NrfInfo) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedAanfInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/AanfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedAanfInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<AanfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedAanfInfoListValueValue> for NrfInfoServedAanfInfoListValueValue {
		fn from(value: &NrfInfoServedAanfInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedAmfInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/AmfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedAmfInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<AmfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedAmfInfoListValueValue> for NrfInfoServedAmfInfoListValueValue {
		fn from(value: &NrfInfoServedAmfInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedAmfInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/AmfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedAmfInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<AmfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedAmfInfoValue> for NrfInfoServedAmfInfoValue {
		fn from(value: &NrfInfoServedAmfInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedAusfInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/AusfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedAusfInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<AusfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedAusfInfoListValueValue> for NrfInfoServedAusfInfoListValueValue {
		fn from(value: &NrfInfoServedAusfInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedAusfInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/AusfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedAusfInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<AusfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedAusfInfoValue> for NrfInfoServedAusfInfoValue {
		fn from(value: &NrfInfoServedAusfInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedBsfInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/BsfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedBsfInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<BsfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedBsfInfoListValueValue> for NrfInfoServedBsfInfoListValueValue {
		fn from(value: &NrfInfoServedBsfInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedBsfInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/BsfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedBsfInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<BsfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedBsfInfoValue> for NrfInfoServedBsfInfoValue {
		fn from(value: &NrfInfoServedBsfInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedChfInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/ChfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedChfInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<ChfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedChfInfoListValueValue> for NrfInfoServedChfInfoListValueValue {
		fn from(value: &NrfInfoServedChfInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedChfInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/ChfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedChfInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<ChfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedChfInfoValue> for NrfInfoServedChfInfoValue {
		fn from(value: &NrfInfoServedChfInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedGmlcInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/GmlcInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedGmlcInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<GmlcInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedGmlcInfoValue> for NrfInfoServedGmlcInfoValue {
		fn from(value: &NrfInfoServedGmlcInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedHssInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/HssInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedHssInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<HssInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedHssInfoListValueValue> for NrfInfoServedHssInfoListValueValue {
		fn from(value: &NrfInfoServedHssInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedLmfInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/LmfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedLmfInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<LmfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedLmfInfoValue> for NrfInfoServedLmfInfoValue {
		fn from(value: &NrfInfoServedLmfInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedMbSmfInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/MbSmfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedMbSmfInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<MbSmfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedMbSmfInfoListValueValue> for NrfInfoServedMbSmfInfoListValueValue {
		fn from(value: &NrfInfoServedMbSmfInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedNefInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/NefInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedNefInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<NefInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedNefInfoValue> for NrfInfoServedNefInfoValue {
		fn from(value: &NrfInfoServedNefInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedNwdafInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/NwdafInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedNwdafInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<NwdafInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedNwdafInfoValue> for NrfInfoServedNwdafInfoValue {
		fn from(value: &NrfInfoServedNwdafInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedPcfInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/PcfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedPcfInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<PcfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedPcfInfoListValueValue> for NrfInfoServedPcfInfoListValueValue {
		fn from(value: &NrfInfoServedPcfInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedPcfInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/PcfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedPcfInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<PcfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedPcfInfoValue> for NrfInfoServedPcfInfoValue {
		fn from(value: &NrfInfoServedPcfInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedPcscfInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/PcscfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedPcscfInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<PcscfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedPcscfInfoListValueValue> for NrfInfoServedPcscfInfoListValueValue {
		fn from(value: &NrfInfoServedPcscfInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedScpInfoListValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/ScpInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedScpInfoListValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<ScpInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedScpInfoListValue> for NrfInfoServedScpInfoListValue {
		fn from(value: &NrfInfoServedScpInfoListValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedSeppInfoListValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/SeppInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedSeppInfoListValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<SeppInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedSeppInfoListValue> for NrfInfoServedSeppInfoListValue {
		fn from(value: &NrfInfoServedSeppInfoListValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedSmfInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/SmfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedSmfInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<SmfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedSmfInfoListValueValue> for NrfInfoServedSmfInfoListValueValue {
		fn from(value: &NrfInfoServedSmfInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedSmfInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/SmfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedSmfInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<SmfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedSmfInfoValue> for NrfInfoServedSmfInfoValue {
		fn from(value: &NrfInfoServedSmfInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedUdmInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/UdmInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedUdmInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<UdmInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedUdmInfoListValueValue> for NrfInfoServedUdmInfoListValueValue {
		fn from(value: &NrfInfoServedUdmInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedUdmInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/UdmInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedUdmInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<UdmInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedUdmInfoValue> for NrfInfoServedUdmInfoValue {
		fn from(value: &NrfInfoServedUdmInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedUdrInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/UdrInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedUdrInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<UdrInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedUdrInfoListValueValue> for NrfInfoServedUdrInfoListValueValue {
		fn from(value: &NrfInfoServedUdrInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedUdrInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/UdrInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedUdrInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<UdrInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedUdrInfoValue> for NrfInfoServedUdrInfoValue {
		fn from(value: &NrfInfoServedUdrInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedUdsfInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/UdsfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedUdsfInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<UdsfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedUdsfInfoListValueValue> for NrfInfoServedUdsfInfoListValueValue {
		fn from(value: &NrfInfoServedUdsfInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedUdsfInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/UdsfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedUdsfInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<UdsfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedUdsfInfoValue> for NrfInfoServedUdsfInfoValue {
		fn from(value: &NrfInfoServedUdsfInfoValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedUpfInfoListValueValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/UpfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedUpfInfoListValueValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<UpfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedUpfInfoListValueValue> for NrfInfoServedUpfInfoListValueValue {
		fn from(value: &NrfInfoServedUpfInfoListValueValue) -> Self {
			value.clone()
		}
	}

	/// NrfInfoServedUpfInfoValue
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/UpfInfo"
	///    },
	///    {
	///      "$ref": "#/components/schemas/EmptyObject"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NrfInfoServedUpfInfoValue {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<UpfInfo>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<EmptyObject>,
	}

	impl From<&NrfInfoServedUpfInfoValue> for NrfInfoServedUpfInfoValue {
		fn from(value: &NrfInfoServedUpfInfoValue) -> Self {
			value.clone()
		}
	}

	/// NSACF service capabilities (e.g. to monitor and control the number of
	/// registered UEs or established PDU sessions per network slice)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "NSACF service capabilities (e.g. to monitor and control
	/// the number of registered UEs or established PDU sessions per network
	/// slice)\n",
	///  "type": "object",
	///  "properties": {
	///    "supportPduSAC": {
	///      "description": "Indicates the service capability of the NSACF to
	/// monitor and control the number of\nestablished PDU sessions per network
	/// slice for the network slice that is subject to NSAC  \ntrue: Supported
	/// \nfalse (default): Not Supported\n",
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "supportUeSAC": {
	///      "description": "Indicates the service capability of the NSACF to
	/// monitor and control the number of\nregistered UEs per network slice for
	/// the network slice that is subject to NSAC  \ntrue: Supported  \nfalse
	/// (default): Not Supported\n",
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
	pub struct NsacfCapability {
		/// Indicates the service capability of the NSACF to monitor and control
		/// the number of established PDU sessions per network slice for
		/// the network slice that is subject to NSAC true: Supported
		/// false (default): Not Supported
		#[serde(rename = "supportPduSAC", default)]
		pub support_pdu_sac: bool,
		/// Indicates the service capability of the NSACF to monitor and control
		/// the number of registered UEs per network slice for the
		/// network slice that is subject to NSAC true: Supported
		/// false (default): Not Supported
		#[serde(rename = "supportUeSAC", default)]
		pub support_ue_sac: bool,
	}

	impl From<&NsacfCapability> for NsacfCapability {
		fn from(value: &NsacfCapability) -> Self {
			value.clone()
		}
	}

	/// Information of a NSACF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a NSACF NF Instance",
	///  "type": "object",
	///  "required": [
	///    "nsacfCapability"
	///  ],
	///  "properties": {
	///    "nsacSaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsacSai"
	///      },
	///      "minItems": 1
	///    },
	///    "nsacfCapability": {
	///      "$ref": "#/components/schemas/NsacfCapability"
	///    },
	///    "taiList": {
	///      "deprecated": true,
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "deprecated": true,
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
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
	pub struct NsacfInfo {
		#[serde(rename = "nsacSaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub nsac_sai_list: Vec<NsacSai>,
		#[serde(rename = "nsacfCapability")]
		pub nsacf_capability: NsacfCapability,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&NsacfInfo> for NsacfInfo {
		fn from(value: &NsacfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a NSSAAF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a NSSAAF Instance",
	///  "type": "object",
	///  "properties": {
	///    "internalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/InternalGroupIdRange"
	///      },
	///      "minItems": 1
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct NssaafInfo {
		#[serde(
			rename = "internalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub internal_group_identifiers_ranges: Vec<InternalGroupIdRange>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&NssaafInfo> for NssaafInfo {
		fn from(value: &NssaafInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates the capability supported by the NWDAF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the capability supported by the NWDAF",
	///  "type": "object",
	///  "properties": {
	///    "analyticsAggregation": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "analyticsMetadataProvisioning": {
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
	pub struct NwdafCapability {
		#[serde(rename = "analyticsAggregation", default)]
		pub analytics_aggregation: bool,
		#[serde(rename = "analyticsMetadataProvisioning", default)]
		pub analytics_metadata_provisioning: bool,
	}

	impl From<&NwdafCapability> for NwdafCapability {
		fn from(value: &NwdafCapability) -> Self {
			value.clone()
		}
	}

	/// Subscription to a set of NF Instances (NWDAFs), identified by Analytics
	/// ID(s), S-NSSAI(s) or NWDAF Serving Area information, i.e. list of TAIs
	/// for which the NWDAF can provide analytics.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NF Instances (NWDAFs),
	/// identified by Analytics ID(s), S-NSSAI(s) or NWDAF Serving Area
	/// information, i.e. list of TAIs for which the NWDAF can provide
	/// analytics.\n",
	///  "type": "object",
	///  "required": [
	///    "conditionType"
	///  ],
	///  "properties": {
	///    "analyticsIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "conditionType": {
	///      "type": "string",
	///      "enum": [
	///        "NWDAF_COND"
	///      ]
	///    },
	///    "mlAnalyticsList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MlAnalyticsInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFType"
	///      },
	///      "minItems": 1
	///    },
	///    "snssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
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
	pub struct NwdafCond {
		#[serde(
			rename = "analyticsIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub analytics_ids: Vec<String>,
		#[serde(rename = "conditionType")]
		pub condition_type: NwdafCondConditionType,
		#[serde(
			rename = "mlAnalyticsList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ml_analytics_list: Vec<MlAnalyticsInfo>,
		#[serde(
			rename = "servingNfSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_set_id_list: Vec<NfSetId>,
		#[serde(
			rename = "servingNfTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_type_list: Vec<NfType>,
		#[serde(rename = "snssaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub snssai_list: Vec<Snssai>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&NwdafCond> for NwdafCond {
		fn from(value: &NwdafCond) -> Self {
			value.clone()
		}
	}

	/// NwdafCondConditionType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "NWDAF_COND"
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
	pub enum NwdafCondConditionType {
		#[default]
		#[serde(rename = "NWDAF_COND")]
		NwdafCond,
	}

	impl From<&NwdafCondConditionType> for NwdafCondConditionType {
		fn from(value: &NwdafCondConditionType) -> Self {
			value.clone()
		}
	}

	impl ToString for NwdafCondConditionType {
		fn to_string(&self) -> String {
			match *self {
				Self::NwdafCond => "NWDAF_COND".to_string(),
			}
		}
	}

	impl std::str::FromStr for NwdafCondConditionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NWDAF_COND" => Ok(Self::NwdafCond),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NwdafCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NwdafCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NwdafCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - SLICE_LOAD_LEVEL: Indicates that the event subscribed is load level
	///   information of Network Slice
	/// - NETWORK_PERFORMANCE: Indicates that the event subscribed is network
	///   performance information.
	/// - NF_LOAD: Indicates that the event subscribed is load level and status
	///   of one or several Network Functions.
	/// - SERVICE_EXPERIENCE: Indicates that the event subscribed is service
	///   experience.
	/// - UE_MOBILITY: Indicates that the event subscribed is UE mobility
	///   information.
	/// - UE_COMMUNICATION: Indicates that the event subscribed is UE
	///   communication information.
	/// - QOS_SUSTAINABILITY: Indicates that the event subscribed is QoS
	///   sustainability.
	/// - ABNORMAL_BEHAVIOUR: Indicates that the event subscribed is abnormal
	///   behaviour.
	/// - USER_DATA_CONGESTION: Indicates that the event subscribed is user data
	///   congestion information.
	/// - NSI_LOAD_LEVEL: Indicates that the event subscribed is load level
	///   information of Network Slice and the optionally associated Network
	///   Slice Instance
	/// - DN_PERFORMANCE: Indicates that the event subscribed is DN performance
	///   information.
	/// - DISPERSION: Indicates that the event subscribed is dispersion
	///   information.
	/// - RED_TRANS_EXP: Indicates that the event subscribed is redundant
	///   transmission experience.
	/// - WLAN_PERFORMANCE: Indicates that the event subscribed is WLAN
	///   performance.
	/// - SM_CONGESTION: Indicates the Session Management Congestion Control
	///   Experience information for specific DNN and/or S-NSSAI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- SLICE_LOAD_LEVEL: Indicates
	/// that the event subscribed is load level information of Network Slice\n-
	/// NETWORK_PERFORMANCE: Indicates that the event subscribed is network
	/// performance information.\n- NF_LOAD: Indicates that the event subscribed
	/// is load level and status of one or several Network Functions.\n-
	/// SERVICE_EXPERIENCE: Indicates that the event subscribed is service
	/// experience.\n- UE_MOBILITY: Indicates that the event subscribed is UE
	/// mobility information.\n- UE_COMMUNICATION: Indicates that the event
	/// subscribed is UE communication information.\n- QOS_SUSTAINABILITY:
	/// Indicates that the event subscribed is QoS sustainability.\n-
	/// ABNORMAL_BEHAVIOUR: Indicates that the event subscribed is abnormal
	/// behaviour.\n- USER_DATA_CONGESTION: Indicates that the event subscribed
	/// is user data congestion information.\n- NSI_LOAD_LEVEL: Indicates that
	/// the event subscribed is load level information of Network Slice and the
	/// optionally associated Network Slice Instance\n- DN_PERFORMANCE:
	/// Indicates that the event subscribed is DN performance information.\n-
	/// DISPERSION: Indicates that the event subscribed is dispersion
	/// information.\n- RED_TRANS_EXP: Indicates that the event subscribed is
	/// redundant transmission experience.\n- WLAN_PERFORMANCE: Indicates that
	/// the event subscribed is WLAN performance.\n- SM_CONGESTION: Indicates
	/// the Session Management Congestion Control Experience information for
	/// specific DNN and/or S-NSSAI.\n",
	///  "type": "string",
	///  "enum": [
	///    "SLICE_LOAD_LEVEL",
	///    "NETWORK_PERFORMANCE",
	///    "NF_LOAD",
	///    "SERVICE_EXPERIENCE",
	///    "UE_MOBILITY",
	///    "UE_COMMUNICATION",
	///    "QOS_SUSTAINABILITY",
	///    "ABNORMAL_BEHAVIOUR",
	///    "USER_DATA_CONGESTION",
	///    "NSI_LOAD_LEVEL",
	///    "DN_PERFORMANCE",
	///    "DISPERSION",
	///    "RED_TRANS_EXP",
	///    "WLAN_PERFORMANCE",
	///    "SM_CONGESTION"
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
	pub enum NwdafEvent {
		#[default]
		#[serde(rename = "SLICE_LOAD_LEVEL")]
		SliceLoadLevel,
		#[serde(rename = "NETWORK_PERFORMANCE")]
		NetworkPerformance,
		#[serde(rename = "NF_LOAD")]
		NfLoad,
		#[serde(rename = "SERVICE_EXPERIENCE")]
		ServiceExperience,
		#[serde(rename = "UE_MOBILITY")]
		UeMobility,
		#[serde(rename = "UE_COMMUNICATION")]
		UeCommunication,
		#[serde(rename = "QOS_SUSTAINABILITY")]
		QosSustainability,
		#[serde(rename = "ABNORMAL_BEHAVIOUR")]
		AbnormalBehaviour,
		#[serde(rename = "USER_DATA_CONGESTION")]
		UserDataCongestion,
		#[serde(rename = "NSI_LOAD_LEVEL")]
		NsiLoadLevel,
		#[serde(rename = "DN_PERFORMANCE")]
		DnPerformance,
		#[serde(rename = "DISPERSION")]
		Dispersion,
		#[serde(rename = "RED_TRANS_EXP")]
		RedTransExp,
		#[serde(rename = "WLAN_PERFORMANCE")]
		WlanPerformance,
		#[serde(rename = "SM_CONGESTION")]
		SmCongestion,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NwdafEvent> for NwdafEvent {
		fn from(value: &NwdafEvent) -> Self {
			value.clone()
		}
	}

	impl ToString for NwdafEvent {
		fn to_string(&self) -> String {
			match *self {
				Self::SliceLoadLevel => "SLICE_LOAD_LEVEL".to_string(),
				Self::NetworkPerformance => "NETWORK_PERFORMANCE".to_string(),
				Self::NfLoad => "NF_LOAD".to_string(),
				Self::ServiceExperience => "SERVICE_EXPERIENCE".to_string(),
				Self::UeMobility => "UE_MOBILITY".to_string(),
				Self::UeCommunication => "UE_COMMUNICATION".to_string(),
				Self::QosSustainability => "QOS_SUSTAINABILITY".to_string(),
				Self::AbnormalBehaviour => "ABNORMAL_BEHAVIOUR".to_string(),
				Self::UserDataCongestion => "USER_DATA_CONGESTION".to_string(),
				Self::NsiLoadLevel => "NSI_LOAD_LEVEL".to_string(),
				Self::DnPerformance => "DN_PERFORMANCE".to_string(),
				Self::Dispersion => "DISPERSION".to_string(),
				Self::RedTransExp => "RED_TRANS_EXP".to_string(),
				Self::WlanPerformance => "WLAN_PERFORMANCE".to_string(),
				Self::SmCongestion => "SM_CONGESTION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NwdafEvent {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SLICE_LOAD_LEVEL" => Ok(Self::SliceLoadLevel),
				"NETWORK_PERFORMANCE" => Ok(Self::NetworkPerformance),
				"NF_LOAD" => Ok(Self::NfLoad),
				"SERVICE_EXPERIENCE" => Ok(Self::ServiceExperience),
				"UE_MOBILITY" => Ok(Self::UeMobility),
				"UE_COMMUNICATION" => Ok(Self::UeCommunication),
				"QOS_SUSTAINABILITY" => Ok(Self::QosSustainability),
				"ABNORMAL_BEHAVIOUR" => Ok(Self::AbnormalBehaviour),
				"USER_DATA_CONGESTION" => Ok(Self::UserDataCongestion),
				"NSI_LOAD_LEVEL" => Ok(Self::NsiLoadLevel),
				"DN_PERFORMANCE" => Ok(Self::DnPerformance),
				"DISPERSION" => Ok(Self::Dispersion),
				"RED_TRANS_EXP" => Ok(Self::RedTransExp),
				"WLAN_PERFORMANCE" => Ok(Self::WlanPerformance),
				"SM_CONGESTION" => Ok(Self::SmCongestion),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NwdafEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NwdafEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NwdafEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Information of a NWDAF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a NWDAF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "analyticsDelay": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "eventIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EventId"
	///      },
	///      "minItems": 1
	///    },
	///    "mlAnalyticsList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MlAnalyticsInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "nwdafCapability": {
	///      "$ref": "#/components/schemas/NwdafCapability"
	///    },
	///    "nwdafEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFType"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
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
	pub struct NwdafInfo {
		#[serde(
			rename = "analyticsDelay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub analytics_delay: Option<DurationSec>,
		#[serde(rename = "eventIds", default, skip_serializing_if = "Vec::is_empty")]
		pub event_ids: Vec<EventId>,
		#[serde(
			rename = "mlAnalyticsList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ml_analytics_list: Vec<MlAnalyticsInfo>,
		#[serde(
			rename = "nwdafCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nwdaf_capability: Option<NwdafCapability>,
		#[serde(rename = "nwdafEvents", default, skip_serializing_if = "Vec::is_empty")]
		pub nwdaf_events: Vec<NwdafEvent>,
		#[serde(
			rename = "servingNfSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_set_id_list: Vec<NfSetId>,
		#[serde(
			rename = "servingNfTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_type_list: Vec<NfType>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&NwdafInfo> for NwdafInfo {
		fn from(value: &NwdafInfo) -> Self {
			value.clone()
		}
	}

	/// Communication options of the NRF sent in response payload of OPTIONS
	/// method
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Communication options of the NRF sent in response
	/// payload of OPTIONS method",
	///  "type": "object",
	///  "properties": {
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
	pub struct OptionsResponse {
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&OptionsResponse> for OptionsResponse {
		fn from(value: &OptionsResponse) -> Self {
			value.clone()
		}
	}

	/// Information of a PCF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a PCF NF Instance",
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
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "proseCapability": {
	///      "$ref": "#/components/schemas/ProSeCapability"
	///    },
	///    "proseSupportInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "rxDiamHost": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "rxDiamRealm": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "v2xCapability": {
	///      "$ref": "#/components/schemas/V2xCapability"
	///    },
	///    "v2xSupportInd": {
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
	pub struct PcfInfo {
		#[serde(rename = "dnnList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnn_list: Vec<Dnn>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "proseCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_capability: Option<ProSeCapability>,
		#[serde(rename = "proseSupportInd", default)]
		pub prose_support_ind: bool,
		#[serde(
			rename = "rxDiamHost",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rx_diam_host: Option<Fqdn>,
		#[serde(
			rename = "rxDiamRealm",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rx_diam_realm: Option<Fqdn>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
		#[serde(
			rename = "v2xCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub v2x_capability: Option<V2xCapability>,
		#[serde(rename = "v2xSupportInd", default)]
		pub v2x_support_ind: bool,
	}

	impl From<&PcfInfo> for PcfInfo {
		fn from(value: &PcfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a P-CSCF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a P-CSCF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "accessType": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "dnnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnn"
	///      },
	///      "minItems": 1
	///    },
	///    "gmFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "gmIpv4Addresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "gmIpv6Addresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "mwFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "mwIpv4Addresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "mwIpv6Addresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "servedIpv4AddressRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4AddressRange"
	///      },
	///      "minItems": 1
	///    },
	///    "servedIpv6PrefixRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6PrefixRange"
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
	pub struct PcscfInfo {
		#[serde(rename = "accessType", default, skip_serializing_if = "Vec::is_empty")]
		pub access_type: Vec<AccessType>,
		#[serde(rename = "dnnList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnn_list: Vec<Dnn>,
		#[serde(rename = "gmFqdn", default, skip_serializing_if = "Option::is_none")]
		pub gm_fqdn: Option<Fqdn>,
		#[serde(
			rename = "gmIpv4Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub gm_ipv4_addresses: Vec<Ipv4Addr>,
		#[serde(
			rename = "gmIpv6Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub gm_ipv6_addresses: Vec<Ipv6Addr>,
		#[serde(rename = "mwFqdn", default, skip_serializing_if = "Option::is_none")]
		pub mw_fqdn: Option<Fqdn>,
		#[serde(
			rename = "mwIpv4Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mw_ipv4_addresses: Vec<Ipv4Addr>,
		#[serde(
			rename = "mwIpv6Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mw_ipv6_addresses: Vec<Ipv6Addr>,
		#[serde(
			rename = "servedIpv4AddressRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub served_ipv4_address_ranges: Vec<Ipv4AddressRange>,
		#[serde(
			rename = "servedIpv6PrefixRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub served_ipv6_prefix_ranges: Vec<Ipv6PrefixRange>,
	}

	impl From<&PcscfInfo> for PcscfInfo {
		fn from(value: &PcscfInfo) -> Self {
			value.clone()
		}
	}

	/// List of Application IDs and/or AF IDs managed by a given NEF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of Application IDs and/or AF IDs managed by a
	/// given NEF Instance",
	///  "type": "object",
	///  "properties": {
	///    "afIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "appIds": {
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
	pub struct PfdData {
		#[serde(rename = "afIds", default, skip_serializing_if = "Vec::is_empty")]
		pub af_ids: Vec<String>,
		#[serde(rename = "appIds", default, skip_serializing_if = "Vec::is_empty")]
		pub app_ids: Vec<String>,
	}

	impl From<&PfdData> for PfdData {
		fn from(value: &PfdData) -> Self {
			value.clone()
		}
	}

	/// Oauth2.0 required indication for a given PLMN ID
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Oauth2.0 required indication for a given PLMN ID",
	///  "type": "object",
	///  "properties": {
	///    "oauth2NotRequiredPlmnIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
	///      },
	///      "minItems": 1
	///    },
	///    "oauth2RequiredPlmnIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
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
	pub struct PlmnOauth2 {
		#[serde(
			rename = "oauth2NotRequiredPlmnIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub oauth2_not_required_plmn_id_list: Vec<PlmnId>,
		#[serde(
			rename = "oauth2RequiredPlmnIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub oauth2_required_plmn_id_list: Vec<PlmnId>,
	}

	impl From<&PlmnOauth2> for PlmnOauth2 {
		fn from(value: &PlmnOauth2) -> Self {
			value.clone()
		}
	}

	/// Range of PLMN IDs
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of PLMN IDs",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "type": "string",
	///      "pattern": "^[0-9]{3}[0-9]{2,3}$"
	///    },
	///    "pattern": {
	///      "type": "string"
	///    },
	///    "start": {
	///      "type": "string",
	///      "pattern": "^[0-9]{3}[0-9]{2,3}$"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PlmnRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<PlmnRangeEnd>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pattern: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<PlmnRangeStart>,
	}

	impl From<&PlmnRange> for PlmnRange {
		fn from(value: &PlmnRange) -> Self {
			value.clone()
		}
	}

	/// PlmnRangeEnd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]{3}[0-9]{2,3}$"
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
	pub struct PlmnRangeEnd(String);

	impl ::std::ops::Deref for PlmnRangeEnd {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PlmnRangeEnd> for String {
		fn from(value: PlmnRangeEnd) -> Self {
			value.0
		}
	}

	impl From<&PlmnRangeEnd> for PlmnRangeEnd {
		fn from(value: &PlmnRangeEnd) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PlmnRangeEnd {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{3}[0-9]{2,3}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{3}[0-9]{2,3}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PlmnRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PlmnRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PlmnRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PlmnRangeEnd {
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

	/// PlmnRangeStart
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]{3}[0-9]{2,3}$"
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
	pub struct PlmnRangeStart(String);

	impl ::std::ops::Deref for PlmnRangeStart {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<PlmnRangeStart> for String {
		fn from(value: PlmnRangeStart) -> Self {
			value.0
		}
	}

	impl From<&PlmnRangeStart> for PlmnRangeStart {
		fn from(value: &PlmnRangeStart) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for PlmnRangeStart {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{3}[0-9]{2,3}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{3}[0-9]{2,3}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for PlmnRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for PlmnRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for PlmnRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for PlmnRangeStart {
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

	/// List of network slices (S-NSSAIs) for a given PLMN ID
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of network slices (S-NSSAIs) for a given PLMN ID",
	///  "type": "object",
	///  "required": [
	///    "plmnId",
	///    "sNssaiList"
	///  ],
	///  "properties": {
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "sNssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
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
	pub struct PlmnSnssai {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		#[serde(rename = "sNssaiList")]
		pub s_nssai_list: Vec<ExtSnssai>,
	}

	impl From<&PlmnSnssai> for PlmnSnssai {
		fn from(value: &PlmnSnssai) -> Self {
			value.clone()
		}
	}

	/// Contains information on whether the returned NFProfiles match the
	/// preferred query parameters
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains information on whether the returned NFProfiles
	/// match the preferred query parameters\n",
	///  "type": "object",
	///  "properties": {
	///    "otherApiVersionsInd": {
	///      "type": "boolean"
	///    },
	///    "otherLocalityInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "preferredAnalyticsDelaysInd": {
	///      "type": "boolean"
	///    },
	///    "preferredApiVersionsMatchInd": {
	///      "type": "boolean"
	///    },
	///    "preferredCollocatedNfTypeInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "preferredFullPlmnMatchInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "preferredLocalityMatchInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "preferredPgwMatchInd": {
	///      "type": "boolean"
	///    },
	///    "preferredTaiMatchInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "preferredVendorSpecificFeaturesInd": {
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
	pub struct PreferredSearch {
		#[serde(
			rename = "otherApiVersionsInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub other_api_versions_ind: Option<bool>,
		#[serde(rename = "otherLocalityInd", default)]
		pub other_locality_ind: bool,
		#[serde(
			rename = "preferredAnalyticsDelaysInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub preferred_analytics_delays_ind: Option<bool>,
		#[serde(
			rename = "preferredApiVersionsMatchInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub preferred_api_versions_match_ind: Option<bool>,
		#[serde(rename = "preferredCollocatedNfTypeInd", default)]
		pub preferred_collocated_nf_type_ind: bool,
		#[serde(rename = "preferredFullPlmnMatchInd", default)]
		pub preferred_full_plmn_match_ind: bool,
		#[serde(rename = "preferredLocalityMatchInd", default)]
		pub preferred_locality_match_ind: bool,
		#[serde(
			rename = "preferredPgwMatchInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub preferred_pgw_match_ind: Option<bool>,
		#[serde(rename = "preferredTaiMatchInd", default)]
		pub preferred_tai_match_ind: bool,
		#[serde(rename = "preferredVendorSpecificFeaturesInd", default)]
		pub preferred_vendor_specific_features_ind: bool,
	}

	impl From<&PreferredSearch> for PreferredSearch {
		fn from(value: &PreferredSearch) -> Self {
			value.clone()
		}
	}

	/// Indicate the supported ProSe Capability by the PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicate the supported ProSe Capability by the PCF.",
	///  "type": "object",
	///  "properties": {
	///    "proseDirectCommunication": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "proseDirectDiscovey": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "proseL2RemoteUe": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "proseL2UetoNetworkRelay": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "proseL3RemoteUe": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "proseL3UetoNetworkRelay": {
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
	pub struct ProSeCapability {
		#[serde(rename = "proseDirectCommunication", default)]
		pub prose_direct_communication: bool,
		#[serde(rename = "proseDirectDiscovey", default)]
		pub prose_direct_discovey: bool,
		#[serde(rename = "proseL2RemoteUe", default)]
		pub prose_l2_remote_ue: bool,
		#[serde(rename = "proseL2UetoNetworkRelay", default)]
		pub prose_l2_ueto_network_relay: bool,
		#[serde(rename = "proseL3RemoteUe", default)]
		pub prose_l3_remote_ue: bool,
		#[serde(rename = "proseL3UetoNetworkRelay", default)]
		pub prose_l3_ueto_network_relay: bool,
	}

	impl From<&ProSeCapability> for ProSeCapability {
		fn from(value: &ProSeCapability) -> Self {
			value.clone()
		}
	}

	/// Contains a list of Query Parameters
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a list of Query Parameters",
	///  "type": "object",
	///  "required": [
	///    "queryParams"
	///  ],
	///  "properties": {
	///    "queryParams": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QueryParameter"
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
	pub struct QueryParamCombination {
		#[serde(rename = "queryParams")]
		pub query_params: Vec<QueryParameter>,
	}

	impl From<&QueryParamCombination> for QueryParamCombination {
		fn from(value: &QueryParamCombination) -> Self {
			value.clone()
		}
	}

	/// Contains the name and value of a query parameter
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the name and value of a query parameter",
	///  "type": "object",
	///  "required": [
	///    "name",
	///    "value"
	///  ],
	///  "properties": {
	///    "name": {
	///      "type": "string"
	///    },
	///    "value": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct QueryParameter {
		pub name: String,
		pub value: String,
	}

	impl From<&QueryParameter> for QueryParameter {
		fn from(value: &QueryParameter) -> Self {
			value.clone()
		}
	}

	/// AF Event Exposure data managed by a given NEF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "AF Event Exposure data managed by a given NEF
	/// Instance",
	///  "type": "object",
	///  "required": [
	///    "afEvents"
	///  ],
	///  "properties": {
	///    "afEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AfEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "afIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "appIds": {
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
	pub struct SchemasAfEventExposureData {
		#[serde(rename = "afEvents")]
		pub af_events: Vec<AfEvent>,
		#[serde(rename = "afIds", default, skip_serializing_if = "Vec::is_empty")]
		pub af_ids: Vec<String>,
		#[serde(rename = "appIds", default, skip_serializing_if = "Vec::is_empty")]
		pub app_ids: Vec<String>,
	}

	impl From<&SchemasAfEventExposureData> for SchemasAfEventExposureData {
		fn from(value: &SchemasAfEventExposureData) -> Self {
			value.clone()
		}
	}

	/// Information of an AMF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an AMF NF Instance",
	///  "type": "object",
	///  "required": [
	///    "amfRegionId",
	///    "amfSetId",
	///    "guamiList"
	///  ],
	///  "properties": {
	///    "amfOnboardingCapability": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "amfRegionId": {
	///      "$ref": "#/components/schemas/AmfRegionId"
	///    },
	///    "amfSetId": {
	///      "$ref": "#/components/schemas/AmfSetId"
	///    },
	///    "backupInfoAmfFailure": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Guami"
	///      },
	///      "minItems": 1
	///    },
	///    "backupInfoAmfRemoval": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Guami"
	///      },
	///      "minItems": 1
	///    },
	///    "guamiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Guami"
	///      },
	///      "minItems": 1
	///    },
	///    "highLatencyCom": {
	///      "type": "boolean"
	///    },
	///    "n2InterfaceAmfInfo": {
	///      "$ref": "#/components/schemas/schemas-N2InterfaceAmfInfo"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
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
	pub struct SchemasAmfInfo {
		#[serde(rename = "amfOnboardingCapability", default)]
		pub amf_onboarding_capability: bool,
		#[serde(rename = "amfRegionId")]
		pub amf_region_id: AmfRegionId,
		#[serde(rename = "amfSetId")]
		pub amf_set_id: AmfSetId,
		#[serde(
			rename = "backupInfoAmfFailure",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub backup_info_amf_failure: Vec<crate::common::common_models::Guami>,
		#[serde(
			rename = "backupInfoAmfRemoval",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub backup_info_amf_removal: Vec<crate::common::common_models::Guami>,
		#[serde(rename = "guamiList")]
		pub guami_list: Vec<crate::common::common_models::Guami>,
		#[serde(
			rename = "highLatencyCom",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub high_latency_com: Option<bool>,
		#[serde(
			rename = "n2InterfaceAmfInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_interface_amf_info: Option<SchemasN2InterfaceAmfInfo>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
	}

	impl From<&SchemasAmfInfo> for SchemasAmfInfo {
		fn from(value: &SchemasAmfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an AUSF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an AUSF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "routingIndicators": {
	///      "type": "array",
	///      "items": {
	///        "type": "string",
	///        "pattern": "^[0-9]{1,4}$"
	///      },
	///      "minItems": 1
	///    },
	///    "suciInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SuciInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct SchemasAusfInfo {
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "routingIndicators",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub routing_indicators: Vec<SchemasAusfInfoRoutingIndicatorsItem>,
		#[serde(rename = "suciInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub suci_infos: Vec<SuciInfo>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&SchemasAusfInfo> for SchemasAusfInfo {
		fn from(value: &SchemasAusfInfo) -> Self {
			value.clone()
		}
	}

	/// SchemasAusfInfoRoutingIndicatorsItem
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
	pub struct SchemasAusfInfoRoutingIndicatorsItem(String);

	impl ::std::ops::Deref for SchemasAusfInfoRoutingIndicatorsItem {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasAusfInfoRoutingIndicatorsItem> for String {
		fn from(value: SchemasAusfInfoRoutingIndicatorsItem) -> Self {
			value.0
		}
	}

	impl From<&SchemasAusfInfoRoutingIndicatorsItem> for SchemasAusfInfoRoutingIndicatorsItem {
		fn from(value: &SchemasAusfInfoRoutingIndicatorsItem) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SchemasAusfInfoRoutingIndicatorsItem {
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

	impl ::std::convert::TryFrom<&str> for SchemasAusfInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SchemasAusfInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SchemasAusfInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SchemasAusfInfoRoutingIndicatorsItem {
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

	/// Information of a BSF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a BSF NF Instance",
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
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "ipDomainList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv4AddressRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-Ipv4AddressRange"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6PrefixRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-Ipv6PrefixRange"
	///      },
	///      "minItems": 1
	///    },
	///    "rxDiamHost": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "rxDiamRealm": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct SchemasBsfInfo {
		#[serde(rename = "dnnList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnn_list: Vec<Dnn>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "ipDomainList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ip_domain_list: Vec<String>,
		#[serde(
			rename = "ipv4AddressRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_address_ranges: Vec<SchemasIpv4AddressRange>,
		#[serde(
			rename = "ipv6PrefixRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_prefix_ranges: Vec<SchemasIpv6PrefixRange>,
		#[serde(
			rename = "rxDiamHost",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rx_diam_host: Option<Fqdn>,
		#[serde(
			rename = "rxDiamRealm",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rx_diam_realm: Option<Fqdn>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&SchemasBsfInfo> for SchemasBsfInfo {
		fn from(value: &SchemasBsfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a CHF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a CHF NF Instance",
	///  "type": "object",
	///  "not": {
	///    "required": [
	///      "primaryChfInstance",
	///      "secondaryChfInstance"
	///    ]
	///  },
	///  "properties": {
	///    "gpsiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
	///      },
	///      "minItems": 1
	///    },
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "plmnRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnRange"
	///      },
	///      "minItems": 1
	///    },
	///    "primaryChfInstance": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "secondaryChfInstance": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "supiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct SchemasChfInfo {
		#[serde(
			rename = "gpsiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub gpsi_range_list: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "plmnRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub plmn_range_list: Vec<PlmnRange>,
		#[serde(
			rename = "supiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub supi_range_list: Vec<SupiRange>,
	}

	impl From<&SchemasChfInfo> for SchemasChfInfo {
		fn from(value: &SchemasChfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an collocated NF Instance registered in the NRF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an collocated NF Instance registered in
	/// the NRF",
	///  "type": "object",
	///  "required": [
	///    "nfInstanceId",
	///    "nfType"
	///  ],
	///  "properties": {
	///    "nfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nfType": {
	///      "$ref": "#/components/schemas/CollocatedNfType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasCollocatedNfInstance {
		#[serde(rename = "nfInstanceId")]
		pub nf_instance_id: NfInstanceId,
		#[serde(rename = "nfType")]
		pub nf_type: CollocatedNfType,
	}

	impl From<&SchemasCollocatedNfInstance> for SchemasCollocatedNfInstance {
		fn from(value: &SchemasCollocatedNfInstance) -> Self {
			value.clone()
		}
	}

	/// Information of a DCCF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a DCCF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "servingNfSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFType"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
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
	pub struct SchemasDccfInfo {
		#[serde(
			rename = "servingNfSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_set_id_list: Vec<NfSetId>,
		#[serde(
			rename = "servingNfTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_type_list: Vec<NfType>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
	}

	impl From<&SchemasDccfInfo> for SchemasDccfInfo {
		fn from(value: &SchemasDccfInfo) -> Self {
			value.clone()
		}
	}

	/// Service Specific information for Default Notification Subscription.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Service Specific information for Default Notification
	/// Subscription.",
	///  "type": "object",
	///  "properties": {
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "versions": {
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
	pub struct SchemasDefSubServiceInfo {
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub versions: Vec<String>,
	}

	impl From<&SchemasDefSubServiceInfo> for SchemasDefSubServiceInfo {
		fn from(value: &SchemasDefSubServiceInfo) -> Self {
			value.clone()
		}
	}

	/// Data structure for specifying the notifications the NF service
	/// subscribes by default, along with callback URI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data structure for specifying the notifications the NF
	/// service subscribes by default, along with callback URI\n",
	///  "type": "object",
	///  "required": [
	///    "callbackUri",
	///    "notificationType"
	///  ],
	///  "properties": {
	///    "acceptedEncoding": {
	///      "type": "string"
	///    },
	///    "binding": {
	///      "type": "string"
	///    },
	///    "callbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "interPlmnCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "n1MessageClass": {
	///      "$ref": "#/components/schemas/N1MessageClass"
	///    },
	///    "n2InformationClass": {
	///      "$ref": "#/components/schemas/N2InformationClass"
	///    },
	///    "notificationType": {
	///      "$ref": "#/components/schemas/NotificationType"
	///    },
	///    "serviceInfoList": {
	///      "description": "A map of service specific information. The name of
	/// the corresponding service (as specified in ServiceName data type) is the
	/// key.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-DefSubServiceInfo"
	///      }
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "versions": {
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
	pub struct SchemasDefaultNotificationSubscription {
		#[serde(
			rename = "acceptedEncoding",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub accepted_encoding: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub binding: Option<String>,
		#[serde(rename = "callbackUri")]
		pub callback_uri: Uri,
		#[serde(
			rename = "interPlmnCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_callback_uri: Option<Uri>,
		#[serde(
			rename = "n1MessageClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_message_class: Option<N1MessageClass>,
		#[serde(
			rename = "n2InformationClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_information_class: Option<N2InformationClass>,
		#[serde(rename = "notificationType")]
		pub notification_type: NotificationType,
		/// A map of service specific information. The name of the corresponding
		/// service (as specified in ServiceName data type) is the key.
		#[serde(
			rename = "serviceInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub service_info_list: ::std::collections::HashMap<String, SchemasDefSubServiceInfo>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub versions: Vec<String>,
	}

	impl From<&SchemasDefaultNotificationSubscription> for SchemasDefaultNotificationSubscription {
		fn from(value: &SchemasDefaultNotificationSubscription) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by EASDF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by EASDF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "dnn": {
	///      "anyOf": [
	///        {
	///          "$ref": "#/components/schemas/Dnn"
	///        },
	///        {
	///          "$ref": "#/components/schemas/WildcardDnn"
	///        }
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDnnEasdfInfoItem {
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<Dnai>,
		pub dnn: SchemasDnnEasdfInfoItemDnn,
	}

	impl From<&SchemasDnnEasdfInfoItem> for SchemasDnnEasdfInfoItem {
		fn from(value: &SchemasDnnEasdfInfoItem) -> Self {
			value.clone()
		}
	}

	/// SchemasDnnEasdfInfoItemDnn
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnn"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDnnEasdfInfoItemDnn {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&SchemasDnnEasdfInfoItemDnn> for SchemasDnnEasdfInfoItemDnn {
		fn from(value: &SchemasDnnEasdfInfoItemDnn) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by NF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by NF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "anyOf": [
	///        {
	///          "$ref": "#/components/schemas/Dnn"
	///        },
	///        {
	///          "$ref": "#/components/schemas/WildcardDnn"
	///        }
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDnnInfoItem {
		pub dnn: SchemasDnnInfoItemDnn,
	}

	impl From<&SchemasDnnInfoItem> for SchemasDnnInfoItem {
		fn from(value: &SchemasDnnInfoItem) -> Self {
			value.clone()
		}
	}

	/// SchemasDnnInfoItemDnn
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnn"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDnnInfoItemDnn {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&SchemasDnnInfoItemDnn> for SchemasDnnInfoItemDnn {
		fn from(value: &SchemasDnnInfoItemDnn) -> Self {
			value.clone()
		}
	}

	/// Parameters supported by an MB-SMF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Parameters supported by an MB-SMF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "anyOf": [
	///        {
	///          "$ref": "#/components/schemas/Dnn"
	///        },
	///        {
	///          "$ref": "#/components/schemas/WildcardDnn"
	///        }
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDnnMbSmfInfoItem {
		pub dnn: SchemasDnnMbSmfInfoItemDnn,
	}

	impl From<&SchemasDnnMbSmfInfoItem> for SchemasDnnMbSmfInfoItem {
		fn from(value: &SchemasDnnMbSmfInfoItem) -> Self {
			value.clone()
		}
	}

	/// SchemasDnnMbSmfInfoItemDnn
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnn"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDnnMbSmfInfoItemDnn {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&SchemasDnnMbSmfInfoItemDnn> for SchemasDnnMbSmfInfoItemDnn {
		fn from(value: &SchemasDnnMbSmfInfoItemDnn) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by SMF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by SMF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "anyOf": [
	///          {
	///            "$ref": "#/components/schemas/Dnai"
	///          },
	///          {
	///            "$ref": "#/components/schemas/WildcardDnai"
	///          }
	///        ]
	///      },
	///      "minItems": 1
	///    },
	///    "dnn": {
	///      "anyOf": [
	///        {
	///          "$ref": "#/components/schemas/Dnn"
	///        },
	///        {
	///          "$ref": "#/components/schemas/WildcardDnn"
	///        }
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDnnSmfInfoItem {
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<SchemasDnnSmfInfoItemDnaiListItem>,
		pub dnn: SchemasDnnSmfInfoItemDnn,
	}

	impl From<&SchemasDnnSmfInfoItem> for SchemasDnnSmfInfoItem {
		fn from(value: &SchemasDnnSmfInfoItem) -> Self {
			value.clone()
		}
	}

	/// SchemasDnnSmfInfoItemDnaiListItem
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnai"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnai"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDnnSmfInfoItemDnaiListItem {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnai>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnai>,
	}

	impl From<&SchemasDnnSmfInfoItemDnaiListItem> for SchemasDnnSmfInfoItemDnaiListItem {
		fn from(value: &SchemasDnnSmfInfoItemDnaiListItem) -> Self {
			value.clone()
		}
	}

	/// SchemasDnnSmfInfoItemDnn
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnn"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDnnSmfInfoItemDnn {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&SchemasDnnSmfInfoItemDnn> for SchemasDnnSmfInfoItemDnn {
		fn from(value: &SchemasDnnSmfInfoItemDnn) -> Self {
			value.clone()
		}
	}

	/// Parameters supported by an TSCTSF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Parameters supported by an TSCTSF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "anyOf": [
	///        {
	///          "$ref": "#/components/schemas/Dnn"
	///        },
	///        {
	///          "$ref": "#/components/schemas/WildcardDnn"
	///        }
	///      ]
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDnnTsctsfInfoItem {
		pub dnn: SchemasDnnTsctsfInfoItemDnn,
	}

	impl From<&SchemasDnnTsctsfInfoItem> for SchemasDnnTsctsfInfoItem {
		fn from(value: &SchemasDnnTsctsfInfoItem) -> Self {
			value.clone()
		}
	}

	/// SchemasDnnTsctsfInfoItemDnn
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "anyOf": [
	///    {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    {
	///      "$ref": "#/components/schemas/WildcardDnn"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDnnTsctsfInfoItemDnn {
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_0: Option<Dnn>,
		#[serde(flatten, default, skip_serializing_if = "Option::is_none")]
		pub subtype_1: Option<WildcardDnn>,
	}

	impl From<&SchemasDnnTsctsfInfoItemDnn> for SchemasDnnTsctsfInfoItemDnn {
		fn from(value: &SchemasDnnTsctsfInfoItemDnn) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by UPF for a given DNN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by UPF for a given DNN",
	///  "type": "object",
	///  "required": [
	///    "dnn"
	///  ],
	///  "properties": {
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "dnaiNwInstanceList": {
	///      "description": "Map of network instance per DNAI for the DNN, where
	/// the key of the map is the DNAI. When present, the value of each entry of
	/// the map shall contain a N6 network instance that is configured for the
	/// DNAI indicated by the key.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "string"
	///      }
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "ipv4AddressRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-Ipv4AddressRange"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv4IndexList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpIndex"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6IndexList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpIndex"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6PrefixRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-Ipv6PrefixRange"
	///      },
	///      "minItems": 1
	///    },
	///    "pduSessionTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionType"
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
	pub struct SchemasDnnUpfInfoItem {
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<Dnai>,
		/// Map of network instance per DNAI for the DNN, where the key of the
		/// map is the DNAI. When present, the value of each entry of the map
		/// shall contain a N6 network instance that is configured for the DNAI
		/// indicated by the key.
		#[serde(
			rename = "dnaiNwInstanceList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub dnai_nw_instance_list: ::std::collections::HashMap<String, String>,
		pub dnn: Dnn,
		#[serde(
			rename = "ipv4AddressRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_address_ranges: Vec<SchemasIpv4AddressRange>,
		#[serde(
			rename = "ipv4IndexList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_index_list: Vec<IpIndex>,
		#[serde(
			rename = "ipv6IndexList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_index_list: Vec<IpIndex>,
		#[serde(
			rename = "ipv6PrefixRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_prefix_ranges: Vec<SchemasIpv6PrefixRange>,
		#[serde(
			rename = "pduSessionTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pdu_session_types: Vec<PduSessionType>,
	}

	impl From<&SchemasDnnUpfInfoItem> for SchemasDnnUpfInfoItem {
		fn from(value: &SchemasDnnUpfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Information of an EASDF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an EASDF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "easdfN6IpAddressList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpAddr"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssaiEasdfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-SnssaiEasdfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "upfN6IpAddressList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpAddr"
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
	pub struct SchemasEasdfInfo {
		#[serde(
			rename = "easdfN6IpAddressList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub easdf_n6_ip_address_list: Vec<IpAddr>,
		#[serde(
			rename = "sNssaiEasdfInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub s_nssai_easdf_info_list: Vec<SchemasSnssaiEasdfInfoItem>,
		#[serde(
			rename = "upfN6IpAddressList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub upf_n6_ip_address_list: Vec<IpAddr>,
	}

	impl From<&SchemasEasdfInfo> for SchemasEasdfInfo {
		fn from(value: &SchemasEasdfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a GMLC NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a GMLC NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "gmlcNumbers": {
	///      "type": "array",
	///      "items": {
	///        "type": "string",
	///        "pattern": "^[0-9]{5,15}$"
	///      },
	///      "minItems": 1
	///    },
	///    "servingClientTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExternalClientType"
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
	pub struct SchemasGmlcInfo {
		#[serde(rename = "gmlcNumbers", default, skip_serializing_if = "Vec::is_empty")]
		pub gmlc_numbers: Vec<SchemasGmlcInfoGmlcNumbersItem>,
		#[serde(
			rename = "servingClientTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_client_types: Vec<ExternalClientType>,
	}

	impl From<&SchemasGmlcInfo> for SchemasGmlcInfo {
		fn from(value: &SchemasGmlcInfo) -> Self {
			value.clone()
		}
	}

	/// SchemasGmlcInfoGmlcNumbersItem
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	)]
	pub struct SchemasGmlcInfoGmlcNumbersItem(String);

	impl ::std::ops::Deref for SchemasGmlcInfoGmlcNumbersItem {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasGmlcInfoGmlcNumbersItem> for String {
		fn from(value: SchemasGmlcInfoGmlcNumbersItem) -> Self {
			value.0
		}
	}

	impl From<&SchemasGmlcInfoGmlcNumbersItem> for SchemasGmlcInfoGmlcNumbersItem {
		fn from(value: &SchemasGmlcInfoGmlcNumbersItem) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SchemasGmlcInfoGmlcNumbersItem {
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

	impl ::std::convert::TryFrom<&str> for SchemasGmlcInfoGmlcNumbersItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SchemasGmlcInfoGmlcNumbersItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SchemasGmlcInfoGmlcNumbersItem {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SchemasGmlcInfoGmlcNumbersItem {
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

	/// Information of an HSS NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an HSS NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "externalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
	///      },
	///      "minItems": 1
	///    },
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "hssDiameterAddress": {
	///      "$ref": "#/components/schemas/NetworkNodeDiameterAddress"
	///    },
	///    "imsPrivateIdentityRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
	///      },
	///      "minItems": 1
	///    },
	///    "imsPublicIdentityRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
	///      },
	///      "minItems": 1
	///    },
	///    "imsiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ImsiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "msisdnRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	pub struct SchemasHssInfo {
		#[serde(
			rename = "externalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_group_identifiers_ranges: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "hssDiameterAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hss_diameter_address: Option<NetworkNodeDiameterAddress>,
		#[serde(
			rename = "imsPrivateIdentityRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ims_private_identity_ranges: Vec<IdentityRange>,
		#[serde(
			rename = "imsPublicIdentityRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ims_public_identity_ranges: Vec<IdentityRange>,
		#[serde(rename = "imsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub imsi_ranges: Vec<ImsiRange>,
		#[serde(
			rename = "msisdnRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub msisdn_ranges: Vec<IdentityRange>,
	}

	impl From<&SchemasHssInfo> for SchemasHssInfo {
		fn from(value: &SchemasHssInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a given IP interface of an UPF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a given IP interface of an UPF",
	///  "type": "object",
	///  "required": [
	///    "interfaceType"
	///  ],
	///  "properties": {
	///    "endpointFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "interfaceType": {
	///      "$ref": "#/components/schemas/UPInterfaceType"
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
	///    },
	///    "networkInstance": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasInterfaceUpfInfoItem {
		#[serde(
			rename = "endpointFqdn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub endpoint_fqdn: Option<Fqdn>,
		#[serde(rename = "interfaceType")]
		pub interface_type: UpInterfaceType,
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
		#[serde(
			rename = "networkInstance",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub network_instance: Option<String>,
	}

	impl From<&SchemasInterfaceUpfInfoItem> for SchemasInterfaceUpfInfoItem {
		fn from(value: &SchemasInterfaceUpfInfoItem) -> Self {
			value.clone()
		}
	}

	/// A range of Group IDs (internal group identities), either based on a
	/// numeric range, or based on regular-expression matching
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A range of Group IDs (internal group identities),
	/// either based on a numeric range, or based on regular-expression
	/// matching\n",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "$ref": "#/components/schemas/GroupId"
	///    },
	///    "pattern": {
	///      "type": "string"
	///    },
	///    "start": {
	///      "$ref": "#/components/schemas/GroupId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasInternalGroupIdRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<GroupId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pattern: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<GroupId>,
	}

	impl From<&SchemasInternalGroupIdRange> for SchemasInternalGroupIdRange {
		fn from(value: &SchemasInternalGroupIdRange) -> Self {
			value.clone()
		}
	}

	/// IP addressing information of a given NFService; it consists on, e.g. IP
	/// address, TCP port, transport protocol...
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "IP addressing information of a given NFService; it
	/// consists on, e.g. IP address, TCP port, transport protocol...\n",
	///  "type": "object",
	///  "properties": {
	///    "ipv4Address": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "ipv6Address": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "port": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "transport": {
	///      "$ref": "#/components/schemas/TransportProtocol"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasIpEndPoint {
		#[serde(
			rename = "ipv4Address",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ipv4_address: Option<Ipv4Addr>,
		#[serde(
			rename = "ipv6Address",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ipv6_address: Option<Ipv6Addr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub port: Option<u16>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub transport: Option<TransportProtocol>,
	}

	impl From<&SchemasIpEndPoint> for SchemasIpEndPoint {
		fn from(value: &SchemasIpEndPoint) -> Self {
			value.clone()
		}
	}

	/// Range of IPv4 addresses
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of IPv4 addresses",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "start": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasIpv4AddressRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<Ipv4Addr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<Ipv4Addr>,
	}

	impl From<&SchemasIpv4AddressRange> for SchemasIpv4AddressRange {
		fn from(value: &SchemasIpv4AddressRange) -> Self {
			value.clone()
		}
	}

	/// Range of IPv6 prefixes
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of IPv6 prefixes",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    },
	///    "start": {
	///      "$ref": "#/components/schemas/Ipv6Prefix"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasIpv6PrefixRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<Ipv6Prefix>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<Ipv6Prefix>,
	}

	impl From<&SchemasIpv6PrefixRange> for SchemasIpv6PrefixRange {
		fn from(value: &SchemasIpv6PrefixRange) -> Self {
			value.clone()
		}
	}

	/// Information of an SMS-IWMSC NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an SMS-IWMSC NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "msisdnRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
	///      },
	///      "minItems": 1
	///    },
	///    "scNumber": {
	///      "type": "string",
	///      "pattern": "^[0-9]{5,15}$"
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
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
	pub struct SchemasIwmscInfo {
		#[serde(
			rename = "msisdnRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub msisdn_ranges: Vec<IdentityRange>,
		#[serde(rename = "scNumber", default, skip_serializing_if = "Option::is_none")]
		pub sc_number: Option<SchemasIwmscInfoScNumber>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
	}

	impl From<&SchemasIwmscInfo> for SchemasIwmscInfo {
		fn from(value: &SchemasIwmscInfo) -> Self {
			value.clone()
		}
	}

	/// SchemasIwmscInfoScNumber
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	)]
	pub struct SchemasIwmscInfoScNumber(String);

	impl ::std::ops::Deref for SchemasIwmscInfoScNumber {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasIwmscInfoScNumber> for String {
		fn from(value: SchemasIwmscInfoScNumber) -> Self {
			value.0
		}
	}

	impl From<&SchemasIwmscInfoScNumber> for SchemasIwmscInfoScNumber {
		fn from(value: &SchemasIwmscInfoScNumber) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SchemasIwmscInfoScNumber {
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

	impl ::std::convert::TryFrom<&str> for SchemasIwmscInfoScNumber {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SchemasIwmscInfoScNumber {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SchemasIwmscInfoScNumber {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SchemasIwmscInfoScNumber {
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

	/// Information of an LMF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an LMF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "lmfId": {
	///      "$ref": "#/components/schemas/LMFIdentification"
	///    },
	///    "servingAccessTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "servingAnNodeTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AnNodeType"
	///      },
	///      "minItems": 1
	///    },
	///    "servingClientTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExternalClientType"
	///      },
	///      "minItems": 1
	///    },
	///    "servingRatTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedGADShapes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupportedGADShapes"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
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
	pub struct SchemasLmfInfo {
		#[serde(rename = "lmfId", default, skip_serializing_if = "Option::is_none")]
		pub lmf_id: Option<LmfIdentification>,
		#[serde(
			rename = "servingAccessTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_access_types: Vec<AccessType>,
		#[serde(
			rename = "servingAnNodeTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_an_node_types: Vec<AnNodeType>,
		#[serde(
			rename = "servingClientTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_client_types: Vec<ExternalClientType>,
		#[serde(
			rename = "servingRatTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_rat_types: Vec<RatType>,
		#[serde(
			rename = "supportedGADShapes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub supported_gad_shapes: Vec<SupportedGadShapes>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
	}

	impl From<&SchemasLmfInfo> for SchemasLmfInfo {
		fn from(value: &SchemasLmfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an MB-SMF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an MB-SMF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "mbsSessionList": {
	///      "description": "A map (list of key-value pairs) where a valid JSON
	/// string serves as key",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-MbsSession"
	///      }
	///    },
	///    "sNssaiInfoList": {
	///      "description": "A map (list of key-value pairs) where a valid JSON
	/// string serves as key",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-SnssaiMbSmfInfoItem"
	///      }
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "tmgiRangeList": {
	///      "description": "A map (list of key-value pairs) where a valid JSON
	/// string serves as key",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-TmgiRange"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasMbSmfInfo {
		/// A map (list of key-value pairs) where a valid JSON string serves as
		/// key
		#[serde(
			rename = "mbsSessionList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub mbs_session_list: ::std::collections::HashMap<String, SchemasMbsSession>,
		/// A map (list of key-value pairs) where a valid JSON string serves as
		/// key
		#[serde(
			rename = "sNssaiInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub s_nssai_info_list: ::std::collections::HashMap<String, SchemasSnssaiMbSmfInfoItem>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
		/// A map (list of key-value pairs) where a valid JSON string serves as
		/// key
		#[serde(
			rename = "tmgiRangeList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub tmgi_range_list: ::std::collections::HashMap<String, SchemasTmgiRange>,
	}

	impl From<&SchemasMbSmfInfo> for SchemasMbSmfInfo {
		fn from(value: &SchemasMbSmfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an MB-UPF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an MB-UPF NF Instance",
	///  "type": "object",
	///  "required": [
	///    "sNssaiMbUpfInfoList"
	///  ],
	///  "properties": {
	///    "interfaceMbUpfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-InterfaceUpfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "mbSmfServingArea": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "priority": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "sNssaiMbUpfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-SnssaiUpfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedPfcpFeatures": {
	///      "type": "string"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
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
	pub struct SchemasMbUpfInfo {
		#[serde(
			rename = "interfaceMbUpfInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub interface_mb_upf_info_list: Vec<SchemasInterfaceUpfInfoItem>,
		#[serde(
			rename = "mbSmfServingArea",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mb_smf_serving_area: Vec<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub priority: Option<u16>,
		#[serde(rename = "sNssaiMbUpfInfoList")]
		pub s_nssai_mb_upf_info_list: Vec<SchemasSnssaiUpfInfoItem>,
		#[serde(
			rename = "supportedPfcpFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_pfcp_features: Option<String>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
	}

	impl From<&SchemasMbUpfInfo> for SchemasMbUpfInfo {
		fn from(value: &SchemasMbUpfInfo) -> Self {
			value.clone()
		}
	}

	/// MBS Session currently served by an MB-SMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "MBS Session currently served by an MB-SMF",
	///  "type": "object",
	///  "required": [
	///    "mbsSessionId"
	///  ],
	///  "properties": {
	///    "mbsAreaSessions": {
	///      "description": "A map (list of key-value pairs) where the key
	/// identifies an areaSessionId",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/MbsServiceAreaInfo"
	///      }
	///    },
	///    "mbsSessionId": {
	///      "$ref": "#/components/schemas/MbsSessionId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasMbsSession {
		/// A map (list of key-value pairs) where the key identifies an
		/// areaSessionId
		#[serde(
			rename = "mbsAreaSessions",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub mbs_area_sessions: ::std::collections::HashMap<String, MbsServiceAreaInfo>,
		#[serde(rename = "mbsSessionId")]
		pub mbs_session_id: MbsSessionId,
	}

	impl From<&SchemasMbsSession> for SchemasMbsSession {
		fn from(value: &SchemasMbsSession) -> Self {
			value.clone()
		}
	}

	/// Information of a MFAF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a MFAF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "servingNfSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFType"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
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
	pub struct SchemasMfafInfo {
		#[serde(
			rename = "servingNfSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_set_id_list: Vec<NfSetId>,
		#[serde(
			rename = "servingNfTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_type_list: Vec<NfType>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
	}

	impl From<&SchemasMfafInfo> for SchemasMfafInfo {
		fn from(value: &SchemasMfafInfo) -> Self {
			value.clone()
		}
	}

	/// ML Analytics Filter information supported by the Nnwdaf_MLModelProvision
	/// service
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "ML Analytics Filter information supported by the
	/// Nnwdaf_MLModelProvision service",
	///  "type": "object",
	///  "properties": {
	///    "mlAnalyticsIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "snssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
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
	pub struct SchemasMlAnalyticsInfo {
		#[serde(
			rename = "mlAnalyticsIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ml_analytics_ids: Vec<NwdafEvent>,
		#[serde(rename = "snssaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub snssai_list: Vec<Snssai>,
		#[serde(
			rename = "trackingAreaList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tracking_area_list: Vec<Tai>,
	}

	impl From<&SchemasMlAnalyticsInfo> for SchemasMlAnalyticsInfo {
		fn from(value: &SchemasMlAnalyticsInfo) -> Self {
			value.clone()
		}
	}

	/// AMF N2 interface information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "AMF N2 interface information",
	///  "type": "object",
	///  "properties": {
	///    "amfName": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "ipv4EndpointAddress": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6EndpointAddress": {
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
	pub struct SchemasN2InterfaceAmfInfo {
		#[serde(rename = "amfName", default, skip_serializing_if = "Option::is_none")]
		pub amf_name: Option<Fqdn>,
		#[serde(
			rename = "ipv4EndpointAddress",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_endpoint_address: Vec<Ipv4Addr>,
		#[serde(
			rename = "ipv6EndpointAddress",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_endpoint_address: Vec<Ipv6Addr>,
	}

	impl From<&SchemasN2InterfaceAmfInfo> for SchemasN2InterfaceAmfInfo {
		fn from(value: &SchemasN2InterfaceAmfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an NEF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an NEF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "afEeData": {
	///      "$ref": "#/components/schemas/schemas-AfEventExposureData"
	///    },
	///    "dnaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "externalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	///    "nefId": {
	///      "$ref": "#/components/schemas/NefId"
	///    },
	///    "pfdData": {
	///      "$ref": "#/components/schemas/PfdData"
	///    },
	///    "servedFqdnList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "uasNfFunctionalityInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "unTrustAfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-UnTrustAfInfo"
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
	pub struct SchemasNefInfo {
		#[serde(rename = "afEeData", default, skip_serializing_if = "Option::is_none")]
		pub af_ee_data: Option<SchemasAfEventExposureData>,
		#[serde(rename = "dnaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnai_list: Vec<Dnai>,
		#[serde(
			rename = "externalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_group_identifiers_ranges: Vec<IdentityRange>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "nefId", default, skip_serializing_if = "Option::is_none")]
		pub nef_id: Option<NefId>,
		#[serde(rename = "pfdData", default, skip_serializing_if = "Option::is_none")]
		pub pfd_data: Option<PfdData>,
		#[serde(
			rename = "servedFqdnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub served_fqdn_list: Vec<String>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
		#[serde(rename = "uasNfFunctionalityInd", default)]
		pub uas_nf_functionality_ind: bool,
		#[serde(
			rename = "unTrustAfInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub un_trust_af_info_list: Vec<SchemasUnTrustAfInfo>,
	}

	impl From<&SchemasNefInfo> for SchemasNefInfo {
		fn from(value: &SchemasNefInfo) -> Self {
			value.clone()
		}
	}

	/// Contains the version details of an NF service
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the version details of an NF service",
	///  "type": "object",
	///  "required": [
	///    "apiFullVersion",
	///    "apiVersionInUri"
	///  ],
	///  "properties": {
	///    "apiFullVersion": {
	///      "type": "string"
	///    },
	///    "apiVersionInUri": {
	///      "type": "string"
	///    },
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasNfServiceVersion {
		#[serde(rename = "apiFullVersion")]
		pub api_full_version: String,
		#[serde(rename = "apiVersionInUri")]
		pub api_version_in_uri: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DateTime>,
	}

	impl From<&SchemasNfServiceVersion> for SchemasNfServiceVersion {
		fn from(value: &SchemasNfServiceVersion) -> Self {
			value.clone()
		}
	}

	/// Information of a NSACF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a NSACF NF Instance",
	///  "type": "object",
	///  "required": [
	///    "nsacfCapability"
	///  ],
	///  "properties": {
	///    "nsacSaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsacSai"
	///      },
	///      "minItems": 1
	///    },
	///    "nsacfCapability": {
	///      "$ref": "#/components/schemas/NsacfCapability"
	///    },
	///    "taiList": {
	///      "deprecated": true,
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "deprecated": true,
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
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
	pub struct SchemasNsacfInfo {
		#[serde(rename = "nsacSaiList", default, skip_serializing_if = "Vec::is_empty")]
		pub nsac_sai_list: Vec<NsacSai>,
		#[serde(rename = "nsacfCapability")]
		pub nsacf_capability: NsacfCapability,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
	}

	impl From<&SchemasNsacfInfo> for SchemasNsacfInfo {
		fn from(value: &SchemasNsacfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a NSSAAF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a NSSAAF Instance",
	///  "type": "object",
	///  "properties": {
	///    "internalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-InternalGroupIdRange"
	///      },
	///      "minItems": 1
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct SchemasNssaafInfo {
		#[serde(
			rename = "internalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub internal_group_identifiers_ranges: Vec<SchemasInternalGroupIdRange>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&SchemasNssaafInfo> for SchemasNssaafInfo {
		fn from(value: &SchemasNssaafInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a NWDAF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a NWDAF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "analyticsDelay": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "eventIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EventId"
	///      },
	///      "minItems": 1
	///    },
	///    "mlAnalyticsList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-MlAnalyticsInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "nwdafCapability": {
	///      "$ref": "#/components/schemas/NwdafCapability"
	///    },
	///    "nwdafEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNfTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFType"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
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
	pub struct SchemasNwdafInfo {
		#[serde(
			rename = "analyticsDelay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub analytics_delay: Option<DurationSec>,
		#[serde(rename = "eventIds", default, skip_serializing_if = "Vec::is_empty")]
		pub event_ids: Vec<EventId>,
		#[serde(
			rename = "mlAnalyticsList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ml_analytics_list: Vec<SchemasMlAnalyticsInfo>,
		#[serde(
			rename = "nwdafCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nwdaf_capability: Option<NwdafCapability>,
		#[serde(rename = "nwdafEvents", default, skip_serializing_if = "Vec::is_empty")]
		pub nwdaf_events: Vec<NwdafEvent>,
		#[serde(
			rename = "servingNfSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_set_id_list: Vec<NfSetId>,
		#[serde(
			rename = "servingNfTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_nf_type_list: Vec<NfType>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
	}

	impl From<&SchemasNwdafInfo> for SchemasNwdafInfo {
		fn from(value: &SchemasNwdafInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a PCF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a PCF NF Instance",
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
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "proseCapability": {
	///      "$ref": "#/components/schemas/ProSeCapability"
	///    },
	///    "proseSupportInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "rxDiamHost": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "rxDiamRealm": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "v2xCapability": {
	///      "$ref": "#/components/schemas/V2xCapability"
	///    },
	///    "v2xSupportInd": {
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
	pub struct SchemasPcfInfo {
		#[serde(rename = "dnnList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnn_list: Vec<Dnn>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "proseCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_capability: Option<ProSeCapability>,
		#[serde(rename = "proseSupportInd", default)]
		pub prose_support_ind: bool,
		#[serde(
			rename = "rxDiamHost",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rx_diam_host: Option<Fqdn>,
		#[serde(
			rename = "rxDiamRealm",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rx_diam_realm: Option<Fqdn>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
		#[serde(
			rename = "v2xCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub v2x_capability: Option<V2xCapability>,
		#[serde(rename = "v2xSupportInd", default)]
		pub v2x_support_ind: bool,
	}

	impl From<&SchemasPcfInfo> for SchemasPcfInfo {
		fn from(value: &SchemasPcfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a P-CSCF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a P-CSCF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "accessType": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "dnnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnn"
	///      },
	///      "minItems": 1
	///    },
	///    "gmFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "gmIpv4Addresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "gmIpv6Addresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "mwFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "mwIpv4Addresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "mwIpv6Addresses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "servedIpv4AddressRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-Ipv4AddressRange"
	///      },
	///      "minItems": 1
	///    },
	///    "servedIpv6PrefixRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-Ipv6PrefixRange"
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
	pub struct SchemasPcscfInfo {
		#[serde(rename = "accessType", default, skip_serializing_if = "Vec::is_empty")]
		pub access_type: Vec<AccessType>,
		#[serde(rename = "dnnList", default, skip_serializing_if = "Vec::is_empty")]
		pub dnn_list: Vec<Dnn>,
		#[serde(rename = "gmFqdn", default, skip_serializing_if = "Option::is_none")]
		pub gm_fqdn: Option<Fqdn>,
		#[serde(
			rename = "gmIpv4Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub gm_ipv4_addresses: Vec<Ipv4Addr>,
		#[serde(
			rename = "gmIpv6Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub gm_ipv6_addresses: Vec<Ipv6Addr>,
		#[serde(rename = "mwFqdn", default, skip_serializing_if = "Option::is_none")]
		pub mw_fqdn: Option<Fqdn>,
		#[serde(
			rename = "mwIpv4Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mw_ipv4_addresses: Vec<Ipv4Addr>,
		#[serde(
			rename = "mwIpv6Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mw_ipv6_addresses: Vec<Ipv6Addr>,
		#[serde(
			rename = "servedIpv4AddressRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub served_ipv4_address_ranges: Vec<SchemasIpv4AddressRange>,
		#[serde(
			rename = "servedIpv6PrefixRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub served_ipv6_prefix_ranges: Vec<SchemasIpv6PrefixRange>,
	}

	impl From<&SchemasPcscfInfo> for SchemasPcscfInfo {
		fn from(value: &SchemasPcscfInfo) -> Self {
			value.clone()
		}
	}

	/// List of network slices (S-NSSAIs) for a given PLMN ID
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of network slices (S-NSSAIs) for a given PLMN ID",
	///  "type": "object",
	///  "required": [
	///    "plmnId",
	///    "sNssaiList"
	///  ],
	///  "properties": {
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "sNssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
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
	pub struct SchemasPlmnSnssai {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		#[serde(rename = "sNssaiList")]
		pub s_nssai_list: Vec<ExtSnssai>,
	}

	impl From<&SchemasPlmnSnssai> for SchemasPlmnSnssai {
		fn from(value: &SchemasPlmnSnssai) -> Self {
			value.clone()
		}
	}

	/// SCP Domain specific information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "SCP Domain specific information",
	///  "type": "object",
	///  "properties": {
	///    "scpFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "scpIpEndPoints": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-IpEndPoint"
	///      },
	///      "minItems": 1
	///    },
	///    "scpPorts": {
	///      "description": "Port numbers for HTTP and HTTPS. The key of the map
	/// shall be \"http\" or \"https\".\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "integer",
	///        "maximum": 65535.0,
	///        "minimum": 0.0
	///      }
	///    },
	///    "scpPrefix": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasScpDomainInfo {
		#[serde(rename = "scpFqdn", default, skip_serializing_if = "Option::is_none")]
		pub scp_fqdn: Option<Fqdn>,
		#[serde(
			rename = "scpIpEndPoints",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub scp_ip_end_points: Vec<SchemasIpEndPoint>,
		/// Port numbers for HTTP and HTTPS. The key of the map shall be "http"
		/// or "https".
		#[serde(
			rename = "scpPorts",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub scp_ports: ::std::collections::HashMap<String, u16>,
		#[serde(rename = "scpPrefix", default, skip_serializing_if = "Option::is_none")]
		pub scp_prefix: Option<String>,
	}

	impl From<&SchemasScpDomainInfo> for SchemasScpDomainInfo {
		fn from(value: &SchemasScpDomainInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an SCP Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an SCP Instance",
	///  "type": "object",
	///  "properties": {
	///    "addressDomains": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "ipReachability": {
	///      "$ref": "#/components/schemas/IpReachability"
	///    },
	///    "ipv4AddrRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-Ipv4AddressRange"
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
	///    "ipv6PrefixRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-Ipv6PrefixRange"
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
	///    "remotePlmnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
	///      },
	///      "minItems": 1
	///    },
	///    "remoteSnpnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnIdNid"
	///      },
	///      "minItems": 1
	///    },
	///    "scpCapabilities": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ScpCapability"
	///      }
	///    },
	///    "scpDomainInfoList": {
	///      "description": "A map (list of key-value pairs) where the key of
	/// the map shall be the string identifying an SCP domain\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-ScpDomainInfo"
	///      }
	///    },
	///    "scpPorts": {
	///      "description": "Port numbers for HTTP and HTTPS. The key of the map
	/// shall be \"http\" or \"https\".\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "integer",
	///        "maximum": 65535.0,
	///        "minimum": 0.0
	///      }
	///    },
	///    "scpPrefix": {
	///      "type": "string"
	///    },
	///    "servedNfSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
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
	pub struct SchemasScpInfo {
		#[serde(
			rename = "addressDomains",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub address_domains: Vec<String>,
		#[serde(
			rename = "ipReachability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ip_reachability: Option<IpReachability>,
		#[serde(
			rename = "ipv4AddrRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_addr_ranges: Vec<SchemasIpv4AddressRange>,
		#[serde(
			rename = "ipv4Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_addresses: Vec<Ipv4Addr>,
		#[serde(
			rename = "ipv6PrefixRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_prefix_ranges: Vec<SchemasIpv6PrefixRange>,
		#[serde(
			rename = "ipv6Prefixes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_prefixes: Vec<Ipv6Prefix>,
		#[serde(
			rename = "remotePlmnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub remote_plmn_list: Vec<PlmnId>,
		#[serde(
			rename = "remoteSnpnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub remote_snpn_list: Vec<PlmnIdNid>,
		#[serde(
			rename = "scpCapabilities",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub scp_capabilities: Vec<ScpCapability>,
		/// A map (list of key-value pairs) where the key of the map shall be
		/// the string identifying an SCP domain
		#[serde(
			rename = "scpDomainInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub scp_domain_info_list: ::std::collections::HashMap<String, SchemasScpDomainInfo>,
		/// Port numbers for HTTP and HTTPS. The key of the map shall be "http"
		/// or "https".
		#[serde(
			rename = "scpPorts",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub scp_ports: ::std::collections::HashMap<String, u16>,
		#[serde(rename = "scpPrefix", default, skip_serializing_if = "Option::is_none")]
		pub scp_prefix: Option<String>,
		#[serde(
			rename = "servedNfSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub served_nf_set_id_list: Vec<NfSetId>,
	}

	impl From<&SchemasScpInfo> for SchemasScpInfo {
		fn from(value: &SchemasScpInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a SEPP Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a SEPP Instance",
	///  "type": "object",
	///  "properties": {
	///    "remotePlmnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
	///      },
	///      "minItems": 1
	///    },
	///    "remoteSnpnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnIdNid"
	///      },
	///      "minItems": 1
	///    },
	///    "seppPorts": {
	///      "description": "Port numbers for HTTP and HTTPS. The key of the map
	/// shall be \"http\" or \"https\".\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "integer",
	///        "maximum": 65535.0,
	///        "minimum": 0.0
	///      }
	///    },
	///    "seppPrefix": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasSeppInfo {
		#[serde(
			rename = "remotePlmnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub remote_plmn_list: Vec<PlmnId>,
		#[serde(
			rename = "remoteSnpnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub remote_snpn_list: Vec<PlmnIdNid>,
		/// Port numbers for HTTP and HTTPS. The key of the map shall be "http"
		/// or "https".
		#[serde(
			rename = "seppPorts",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub sepp_ports: ::std::collections::HashMap<String, u16>,
		#[serde(
			rename = "seppPrefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sepp_prefix: Option<String>,
	}

	impl From<&SchemasSeppInfo> for SchemasSeppInfo {
		fn from(value: &SchemasSeppInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an SMF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an SMF NF Instance",
	///  "type": "object",
	///  "required": [
	///    "sNssaiSmfInfoList"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "ismfSupportInd": {
	///      "type": "boolean"
	///    },
	///    "pgwFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "pgwFqdnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Fqdn"
	///      },
	///      "minItems": 1
	///    },
	///    "pgwIpAddrList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpAddr"
	///      },
	///      "minItems": 1
	///    },
	///    "priority": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "sNssaiSmfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-SnssaiSmfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "smfOnboardingCapability": {
	///      "default": false,
	///      "deprecated": true,
	///      "type": "boolean"
	///    },
	///    "smfUPRPCapability": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "vsmfSupportInd": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasSmfInfo {
		#[serde(rename = "accessType", default, skip_serializing_if = "Vec::is_empty")]
		pub access_type: Vec<AccessType>,
		#[serde(
			rename = "ismfSupportInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ismf_support_ind: Option<bool>,
		#[serde(rename = "pgwFqdn", default, skip_serializing_if = "Option::is_none")]
		pub pgw_fqdn: Option<Fqdn>,
		#[serde(rename = "pgwFqdnList", default, skip_serializing_if = "Vec::is_empty")]
		pub pgw_fqdn_list: Vec<Fqdn>,
		#[serde(
			rename = "pgwIpAddrList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pgw_ip_addr_list: Vec<IpAddr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub priority: Option<u16>,
		#[serde(rename = "sNssaiSmfInfoList")]
		pub s_nssai_smf_info_list: Vec<SchemasSnssaiSmfInfoItem>,
		#[serde(rename = "smfOnboardingCapability", default)]
		pub smf_onboarding_capability: bool,
		#[serde(rename = "smfUPRPCapability", default)]
		pub smf_uprp_capability: bool,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
		#[serde(
			rename = "vsmfSupportInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vsmf_support_ind: Option<bool>,
	}

	impl From<&SchemasSmfInfo> for SchemasSmfInfo {
		fn from(value: &SchemasSmfInfo) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by EASDF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by EASDF for a given
	/// S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "dnnEasdfInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnEasdfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-DnnEasdfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasSnssaiEasdfInfoItem {
		#[serde(rename = "dnnEasdfInfoList")]
		pub dnn_easdf_info_list: Vec<SchemasDnnEasdfInfoItem>,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SchemasSnssaiEasdfInfoItem> for SchemasSnssaiEasdfInfoItem {
		fn from(value: &SchemasSnssaiEasdfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Parameters supported by an NF for a given S-NSSAI Set of parameters
	/// supported by NF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Parameters supported by an NF for a given S-NSSAI Set
	/// of parameters supported by NF for a given S-NSSAI\n",
	///  "type": "object",
	///  "required": [
	///    "dnnInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-DnnInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasSnssaiInfoItem {
		#[serde(rename = "dnnInfoList")]
		pub dnn_info_list: Vec<SchemasDnnInfoItem>,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SchemasSnssaiInfoItem> for SchemasSnssaiInfoItem {
		fn from(value: &SchemasSnssaiInfoItem) -> Self {
			value.clone()
		}
	}

	/// Parameters supported by an MB-SMF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Parameters supported by an MB-SMF for a given S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "dnnInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-DnnMbSmfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasSnssaiMbSmfInfoItem {
		#[serde(rename = "dnnInfoList")]
		pub dnn_info_list: Vec<SchemasDnnMbSmfInfoItem>,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SchemasSnssaiMbSmfInfoItem> for SchemasSnssaiMbSmfInfoItem {
		fn from(value: &SchemasSnssaiMbSmfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by SMF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by SMF for a given
	/// S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "dnnSmfInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnSmfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-DnnSmfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasSnssaiSmfInfoItem {
		#[serde(rename = "dnnSmfInfoList")]
		pub dnn_smf_info_list: Vec<SchemasDnnSmfInfoItem>,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SchemasSnssaiSmfInfoItem> for SchemasSnssaiSmfInfoItem {
		fn from(value: &SchemasSnssaiSmfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by TSCTSF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by TSCTSF for a given
	/// S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "dnnInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-DnnTsctsfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasSnssaiTsctsfInfoItem {
		#[serde(rename = "dnnInfoList")]
		pub dnn_info_list: Vec<SchemasDnnTsctsfInfoItem>,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SchemasSnssaiTsctsfInfoItem> for SchemasSnssaiTsctsfInfoItem {
		fn from(value: &SchemasSnssaiTsctsfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by UPF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by UPF for a given
	/// S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "dnnUpfInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnUpfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-DnnUpfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "redundantTransport": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasSnssaiUpfInfoItem {
		#[serde(rename = "dnnUpfInfoList")]
		pub dnn_upf_info_list: Vec<SchemasDnnUpfInfoItem>,
		#[serde(rename = "redundantTransport", default)]
		pub redundant_transport: bool,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SchemasSnssaiUpfInfoItem> for SchemasSnssaiUpfInfoItem {
		fn from(value: &SchemasSnssaiUpfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Range of TAIs (Tracking Area Identities)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of TAIs (Tracking Area Identities)",
	///  "type": "object",
	///  "required": [
	///    "plmnId",
	///    "tacRangeList"
	///  ],
	///  "properties": {
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "tacRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TacRange"
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
	pub struct SchemasTaiRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		#[serde(rename = "tacRangeList")]
		pub tac_range_list: Vec<TacRange>,
	}

	impl From<&SchemasTaiRange> for SchemasTaiRange {
		fn from(value: &SchemasTaiRange) -> Self {
			value.clone()
		}
	}

	/// Range of TMGIs
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of TMGIs",
	///  "type": "object",
	///  "required": [
	///    "mbsServiceIdEnd",
	///    "mbsServiceIdStart",
	///    "plmnId"
	///  ],
	///  "properties": {
	///    "mbsServiceIdEnd": {
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{6}$"
	///    },
	///    "mbsServiceIdStart": {
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{6}$"
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
	pub struct SchemasTmgiRange {
		#[serde(rename = "mbsServiceIdEnd")]
		pub mbs_service_id_end: SchemasTmgiRangeMbsServiceIdEnd,
		#[serde(rename = "mbsServiceIdStart")]
		pub mbs_service_id_start: SchemasTmgiRangeMbsServiceIdStart,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
	}

	impl From<&SchemasTmgiRange> for SchemasTmgiRange {
		fn from(value: &SchemasTmgiRange) -> Self {
			value.clone()
		}
	}

	/// SchemasTmgiRangeMbsServiceIdEnd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub struct SchemasTmgiRangeMbsServiceIdEnd(String);

	impl ::std::ops::Deref for SchemasTmgiRangeMbsServiceIdEnd {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasTmgiRangeMbsServiceIdEnd> for String {
		fn from(value: SchemasTmgiRangeMbsServiceIdEnd) -> Self {
			value.0
		}
	}

	impl From<&SchemasTmgiRangeMbsServiceIdEnd> for SchemasTmgiRangeMbsServiceIdEnd {
		fn from(value: &SchemasTmgiRangeMbsServiceIdEnd) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SchemasTmgiRangeMbsServiceIdEnd {
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

	impl ::std::convert::TryFrom<&str> for SchemasTmgiRangeMbsServiceIdEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SchemasTmgiRangeMbsServiceIdEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SchemasTmgiRangeMbsServiceIdEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SchemasTmgiRangeMbsServiceIdEnd {
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

	/// SchemasTmgiRangeMbsServiceIdStart
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub struct SchemasTmgiRangeMbsServiceIdStart(String);

	impl ::std::ops::Deref for SchemasTmgiRangeMbsServiceIdStart {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasTmgiRangeMbsServiceIdStart> for String {
		fn from(value: SchemasTmgiRangeMbsServiceIdStart) -> Self {
			value.0
		}
	}

	impl From<&SchemasTmgiRangeMbsServiceIdStart> for SchemasTmgiRangeMbsServiceIdStart {
		fn from(value: &SchemasTmgiRangeMbsServiceIdStart) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SchemasTmgiRangeMbsServiceIdStart {
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

	impl ::std::convert::TryFrom<&str> for SchemasTmgiRangeMbsServiceIdStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SchemasTmgiRangeMbsServiceIdStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SchemasTmgiRangeMbsServiceIdStart {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SchemasTmgiRangeMbsServiceIdStart {
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
	pub struct SchemasTngfInfo {
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

	impl From<&SchemasTngfInfo> for SchemasTngfInfo {
		fn from(value: &SchemasTngfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a trusted AF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a trusted AF Instance",
	///  "type": "object",
	///  "properties": {
	///    "afEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AfEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "appIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "internalGroupId": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "mappingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "sNssaiInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-SnssaiInfoItem"
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
	pub struct SchemasTrustAfInfo {
		#[serde(rename = "afEvents", default, skip_serializing_if = "Vec::is_empty")]
		pub af_events: Vec<AfEvent>,
		#[serde(rename = "appIds", default, skip_serializing_if = "Vec::is_empty")]
		pub app_ids: Vec<String>,
		#[serde(
			rename = "internalGroupId",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub internal_group_id: Vec<GroupId>,
		#[serde(rename = "mappingInd", default)]
		pub mapping_ind: bool,
		#[serde(
			rename = "sNssaiInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub s_nssai_info_list: Vec<SchemasSnssaiInfoItem>,
	}

	impl From<&SchemasTrustAfInfo> for SchemasTrustAfInfo {
		fn from(value: &SchemasTrustAfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a TSCTSF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a TSCTSF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "externalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	///    "internalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-InternalGroupIdRange"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssaiInfoList": {
	///      "description": "A map (list of key-value pairs) where a valid JSON
	/// string serves as key",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/schemas-SnssaiTsctsfInfoItem"
	///      }
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct SchemasTsctsfInfo {
		#[serde(
			rename = "externalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_group_identifiers_ranges: Vec<IdentityRange>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(
			rename = "internalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub internal_group_identifiers_ranges: Vec<SchemasInternalGroupIdRange>,
		/// A map (list of key-value pairs) where a valid JSON string serves as
		/// key
		#[serde(
			rename = "sNssaiInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub s_nssai_info_list: ::std::collections::HashMap<String, SchemasSnssaiTsctsfInfoItem>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&SchemasTsctsfInfo> for SchemasTsctsfInfo {
		fn from(value: &SchemasTsctsfInfo) -> Self {
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
	pub struct SchemasTwifInfo {
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

	impl From<&SchemasTwifInfo> for SchemasTwifInfo {
		fn from(value: &SchemasTwifInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an UDM NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an UDM NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "externalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "internalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-InternalGroupIdRange"
	///      },
	///      "minItems": 1
	///    },
	///    "routingIndicators": {
	///      "type": "array",
	///      "items": {
	///        "type": "string",
	///        "pattern": "^[0-9]{1,4}$"
	///      },
	///      "minItems": 1
	///    },
	///    "suciInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SuciInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct SchemasUdmInfo {
		#[serde(
			rename = "externalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_group_identifiers_ranges: Vec<IdentityRange>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "internalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub internal_group_identifiers_ranges: Vec<SchemasInternalGroupIdRange>,
		#[serde(
			rename = "routingIndicators",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub routing_indicators: Vec<SchemasUdmInfoRoutingIndicatorsItem>,
		#[serde(rename = "suciInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub suci_infos: Vec<SuciInfo>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&SchemasUdmInfo> for SchemasUdmInfo {
		fn from(value: &SchemasUdmInfo) -> Self {
			value.clone()
		}
	}

	/// SchemasUdmInfoRoutingIndicatorsItem
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
	pub struct SchemasUdmInfoRoutingIndicatorsItem(String);

	impl ::std::ops::Deref for SchemasUdmInfoRoutingIndicatorsItem {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasUdmInfoRoutingIndicatorsItem> for String {
		fn from(value: SchemasUdmInfoRoutingIndicatorsItem) -> Self {
			value.0
		}
	}

	impl From<&SchemasUdmInfoRoutingIndicatorsItem> for SchemasUdmInfoRoutingIndicatorsItem {
		fn from(value: &SchemasUdmInfoRoutingIndicatorsItem) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SchemasUdmInfoRoutingIndicatorsItem {
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

	impl ::std::convert::TryFrom<&str> for SchemasUdmInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SchemasUdmInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SchemasUdmInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SchemasUdmInfoRoutingIndicatorsItem {
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

	/// Information of an UDR NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an UDR NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "externalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "sharedDataIdRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SharedDataIdRange"
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
	///    "supportedDataSets": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DataSetId"
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
	pub struct SchemasUdrInfo {
		#[serde(
			rename = "externalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_group_identifiers_ranges: Vec<IdentityRange>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "sharedDataIdRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub shared_data_id_ranges: Vec<SharedDataIdRange>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
		#[serde(
			rename = "supportedDataSets",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub supported_data_sets: Vec<DataSetId>,
	}

	impl From<&SchemasUdrInfo> for SchemasUdrInfo {
		fn from(value: &SchemasUdrInfo) -> Self {
			value.clone()
		}
	}

	/// Information related to UDSF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information related to UDSF",
	///  "type": "object",
	///  "properties": {
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "storageIdRanges": {
	///      "description": "A map (list of key-value pairs) where realmId
	/// serves as key and each value in the map is an array of IdentityRanges.
	/// Each IdentityRange is a range of storageIds.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "array",
	///        "items": {
	///          "$ref": "#/components/schemas/IdentityRange"
	///        },
	///        "minItems": 1
	///      }
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct SchemasUdsfInfo {
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		/// A map (list of key-value pairs) where realmId serves as key and each
		/// value in the map is an array of IdentityRanges. Each IdentityRange
		/// is a range of storageIds.
		#[serde(
			rename = "storageIdRanges",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub storage_id_ranges: ::std::collections::HashMap<String, Vec<IdentityRange>>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&SchemasUdsfInfo> for SchemasUdsfInfo {
		fn from(value: &SchemasUdsfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a untrusted AF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a untrusted AF Instance",
	///  "type": "object",
	///  "required": [
	///    "afId"
	///  ],
	///  "properties": {
	///    "afId": {
	///      "type": "string"
	///    },
	///    "mappingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "sNssaiInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-SnssaiInfoItem"
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
	pub struct SchemasUnTrustAfInfo {
		#[serde(rename = "afId")]
		pub af_id: String,
		#[serde(rename = "mappingInd", default)]
		pub mapping_ind: bool,
		#[serde(
			rename = "sNssaiInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub s_nssai_info_list: Vec<SchemasSnssaiInfoItem>,
	}

	impl From<&SchemasUnTrustAfInfo> for SchemasUnTrustAfInfo {
		fn from(value: &SchemasUnTrustAfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of an UPF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an UPF NF Instance",
	///  "type": "object",
	///  "required": [
	///    "sNssaiUpfInfoList"
	///  ],
	///  "properties": {
	///    "atsssCapability": {
	///      "$ref": "#/components/schemas/AtsssCapability"
	///    },
	///    "dataForwarding": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "interfaceUpfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-InterfaceUpfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "ipups": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "iwkEpsInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "pduSessionTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionType"
	///      },
	///      "minItems": 1
	///    },
	///    "priority": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "redundantGtpu": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "sNssaiUpfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-SnssaiUpfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "smfServingArea": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedPfcpFeatures": {
	///      "type": "string"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TaiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "tngfInfo": {
	///      "$ref": "#/components/schemas/schemas-TngfInfo"
	///    },
	///    "twifInfo": {
	///      "$ref": "#/components/schemas/schemas-TwifInfo"
	///    },
	///    "ueIpAddrInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "wAgfInfo": {
	///      "$ref": "#/components/schemas/schemas-WAgfInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasUpfInfo {
		#[serde(
			rename = "atsssCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub atsss_capability: Option<AtsssCapability>,
		#[serde(rename = "dataForwarding", default)]
		pub data_forwarding: bool,
		#[serde(
			rename = "interfaceUpfInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub interface_upf_info_list: Vec<SchemasInterfaceUpfInfoItem>,
		#[serde(default)]
		pub ipups: bool,
		#[serde(rename = "iwkEpsInd", default)]
		pub iwk_eps_ind: bool,
		#[serde(
			rename = "pduSessionTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pdu_session_types: Vec<PduSessionType>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub priority: Option<u16>,
		#[serde(rename = "redundantGtpu", default)]
		pub redundant_gtpu: bool,
		#[serde(rename = "sNssaiUpfInfoList")]
		pub s_nssai_upf_info_list: Vec<SchemasSnssaiUpfInfoItem>,
		#[serde(
			rename = "smfServingArea",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub smf_serving_area: Vec<String>,
		#[serde(
			rename = "supportedPfcpFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_pfcp_features: Option<String>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<SchemasTaiRange>,
		#[serde(rename = "tngfInfo", default, skip_serializing_if = "Option::is_none")]
		pub tngf_info: Option<SchemasTngfInfo>,
		#[serde(rename = "twifInfo", default, skip_serializing_if = "Option::is_none")]
		pub twif_info: Option<SchemasTwifInfo>,
		#[serde(rename = "ueIpAddrInd", default)]
		pub ue_ip_addr_ind: bool,
		#[serde(rename = "wAgfInfo", default, skip_serializing_if = "Option::is_none")]
		pub w_agf_info: Option<SchemasWAgfInfo>,
	}

	impl From<&SchemasUpfInfo> for SchemasUpfInfo {
		fn from(value: &SchemasUpfInfo) -> Self {
			value.clone()
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
	pub struct SchemasWAgfInfo {
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

	impl From<&SchemasWAgfInfo> for SchemasWAgfInfo {
		fn from(value: &SchemasWAgfInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates the capabilities supported by an SCP
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the capabilities supported by an SCP",
	///  "type": "string",
	///  "enum": [
	///    "INDIRECT_COM_WITH_DELEG_DISC"
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
	pub enum ScpCapability {
		#[default]
		#[serde(rename = "INDIRECT_COM_WITH_DELEG_DISC")]
		IndirectComWithDelegDisc,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ScpCapability> for ScpCapability {
		fn from(value: &ScpCapability) -> Self {
			value.clone()
		}
	}

	impl ToString for ScpCapability {
		fn to_string(&self) -> String {
			match *self {
				Self::IndirectComWithDelegDisc => "INDIRECT_COM_WITH_DELEG_DISC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ScpCapability {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"INDIRECT_COM_WITH_DELEG_DISC" => Ok(Self::IndirectComWithDelegDisc),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ScpCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ScpCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ScpCapability {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Subscription to a set of NF or SCP or SEPP instances belonging to
	/// certain SCP domains
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NF or SCP or SEPP instances
	/// belonging to certain SCP domains\n",
	///  "type": "object",
	///  "required": [
	///    "scpDomains"
	///  ],
	///  "properties": {
	///    "nfTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFType"
	///      },
	///      "minItems": 1
	///    },
	///    "scpDomains": {
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
	pub struct ScpDomainCond {
		#[serde(rename = "nfTypeList", default, skip_serializing_if = "Vec::is_empty")]
		pub nf_type_list: Vec<NfType>,
		#[serde(rename = "scpDomains")]
		pub scp_domains: Vec<String>,
	}

	impl From<&ScpDomainCond> for ScpDomainCond {
		fn from(value: &ScpDomainCond) -> Self {
			value.clone()
		}
	}

	/// SCP Domain Connectivity Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "SCP Domain Connectivity Information",
	///  "type": "object",
	///  "required": [
	///    "connectedScpDomainList"
	///  ],
	///  "properties": {
	///    "connectedScpDomainList": {
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
	pub struct ScpDomainConnectivity {
		#[serde(rename = "connectedScpDomainList")]
		pub connected_scp_domain_list: Vec<String>,
	}

	impl From<&ScpDomainConnectivity> for ScpDomainConnectivity {
		fn from(value: &ScpDomainConnectivity) -> Self {
			value.clone()
		}
	}

	/// SCP Domain specific information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "SCP Domain specific information",
	///  "type": "object",
	///  "properties": {
	///    "scpFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "scpIpEndPoints": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpEndPoint"
	///      },
	///      "minItems": 1
	///    },
	///    "scpPorts": {
	///      "description": "Port numbers for HTTP and HTTPS. The key of the map
	/// shall be \"http\" or \"https\".\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "integer",
	///        "maximum": 65535.0,
	///        "minimum": 0.0
	///      }
	///    },
	///    "scpPrefix": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ScpDomainInfo {
		#[serde(rename = "scpFqdn", default, skip_serializing_if = "Option::is_none")]
		pub scp_fqdn: Option<Fqdn>,
		#[serde(
			rename = "scpIpEndPoints",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub scp_ip_end_points: Vec<IpEndPoint>,
		/// Port numbers for HTTP and HTTPS. The key of the map shall be "http"
		/// or "https".
		#[serde(
			rename = "scpPorts",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub scp_ports: ::std::collections::HashMap<String, u16>,
		#[serde(rename = "scpPrefix", default, skip_serializing_if = "Option::is_none")]
		pub scp_prefix: Option<String>,
	}

	impl From<&ScpDomainInfo> for ScpDomainInfo {
		fn from(value: &ScpDomainInfo) -> Self {
			value.clone()
		}
	}

	/// SCP Domain Routing Information Notification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "SCP Domain Routing Information Notification",
	///  "type": "object",
	///  "required": [
	///    "routingInfo"
	///  ],
	///  "properties": {
	///    "localInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "routingInfo": {
	///      "$ref": "#/components/schemas/ScpDomainRoutingInformation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ScpDomainRoutingInfoNotification {
		#[serde(rename = "localInd", default)]
		pub local_ind: bool,
		#[serde(rename = "routingInfo")]
		pub routing_info: ScpDomainRoutingInformation,
	}

	impl From<&ScpDomainRoutingInfoNotification> for ScpDomainRoutingInfoNotification {
		fn from(value: &ScpDomainRoutingInfoNotification) -> Self {
			value.clone()
		}
	}

	/// SCP Domain Routing Information Subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "SCP Domain Routing Information Subscription",
	///  "type": "object",
	///  "required": [
	///    "callbackUri"
	///  ],
	///  "properties": {
	///    "callbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "localInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "reqInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
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
	pub struct ScpDomainRoutingInfoSubscription {
		#[serde(rename = "callbackUri")]
		pub callback_uri: Uri,
		#[serde(rename = "localInd", default)]
		pub local_ind: bool,
		#[serde(
			rename = "reqInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub req_instance_id: Option<NfInstanceId>,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&ScpDomainRoutingInfoSubscription> for ScpDomainRoutingInfoSubscription {
		fn from(value: &ScpDomainRoutingInfoSubscription) -> Self {
			value.clone()
		}
	}

	/// SCP Domain Routing Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "SCP Domain Routing Information",
	///  "type": "object",
	///  "required": [
	///    "scpDomainList"
	///  ],
	///  "properties": {
	///    "scpDomainList": {
	///      "description": "This IE shall contain a map of SCP domain
	/// interconnection information, where\nthe key of the map is a SCP domain.
	/// The value of each entry shall be the\ninterconnectivity information of
	/// the the SCP domain indicated by the key.\nAn empty map indicates that
	/// there is no SCP domain currently registered in\nthe NRF.\n",
	///      "type": "object",
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/ScpDomainConnectivity"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ScpDomainRoutingInformation {
		/// This IE shall contain a map of SCP domain interconnection
		/// information, where the key of the map is a SCP domain. The
		/// value of each entry shall be the interconnectivity
		/// information of the the SCP domain indicated by the key.
		/// An empty map indicates that there is no SCP domain currently
		/// registered in the NRF.
		#[serde(rename = "scpDomainList")]
		pub scp_domain_list: ::std::collections::HashMap<String, ScpDomainConnectivity>,
	}

	impl From<&ScpDomainRoutingInformation> for ScpDomainRoutingInformation {
		fn from(value: &ScpDomainRoutingInformation) -> Self {
			value.clone()
		}
	}

	/// Information of an SCP Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an SCP Instance",
	///  "type": "object",
	///  "properties": {
	///    "addressDomains": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "ipReachability": {
	///      "$ref": "#/components/schemas/IpReachability"
	///    },
	///    "ipv4AddrRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4AddressRange"
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
	///    "ipv6PrefixRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv6PrefixRange"
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
	///    "remotePlmnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
	///      },
	///      "minItems": 1
	///    },
	///    "remoteSnpnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnIdNid"
	///      },
	///      "minItems": 1
	///    },
	///    "scpCapabilities": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ScpCapability"
	///      }
	///    },
	///    "scpDomainInfoList": {
	///      "description": "A map (list of key-value pairs) where the key of
	/// the map shall be the string identifying an SCP domain\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/ScpDomainInfo"
	///      }
	///    },
	///    "scpPorts": {
	///      "description": "Port numbers for HTTP and HTTPS. The key of the map
	/// shall be \"http\" or \"https\".\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "integer",
	///        "maximum": 65535.0,
	///        "minimum": 0.0
	///      }
	///    },
	///    "scpPrefix": {
	///      "type": "string"
	///    },
	///    "servedNfSetIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
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
	pub struct ScpInfo {
		#[serde(
			rename = "addressDomains",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub address_domains: Vec<String>,
		#[serde(
			rename = "ipReachability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ip_reachability: Option<IpReachability>,
		#[serde(
			rename = "ipv4AddrRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_addr_ranges: Vec<Ipv4AddressRange>,
		#[serde(
			rename = "ipv4Addresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv4_addresses: Vec<Ipv4Addr>,
		#[serde(
			rename = "ipv6PrefixRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_prefix_ranges: Vec<Ipv6PrefixRange>,
		#[serde(
			rename = "ipv6Prefixes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ipv6_prefixes: Vec<Ipv6Prefix>,
		#[serde(
			rename = "remotePlmnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub remote_plmn_list: Vec<PlmnId>,
		#[serde(
			rename = "remoteSnpnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub remote_snpn_list: Vec<PlmnIdNid>,
		#[serde(
			rename = "scpCapabilities",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub scp_capabilities: Vec<ScpCapability>,
		/// A map (list of key-value pairs) where the key of the map shall be
		/// the string identifying an SCP domain
		#[serde(
			rename = "scpDomainInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub scp_domain_info_list: ::std::collections::HashMap<String, ScpDomainInfo>,
		/// Port numbers for HTTP and HTTPS. The key of the map shall be "http"
		/// or "https".
		#[serde(
			rename = "scpPorts",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub scp_ports: ::std::collections::HashMap<String, u16>,
		#[serde(rename = "scpPrefix", default, skip_serializing_if = "Option::is_none")]
		pub scp_prefix: Option<String>,
		#[serde(
			rename = "servedNfSetIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub served_nf_set_id_list: Vec<NfSetId>,
	}

	impl From<&ScpInfo> for ScpInfo {
		fn from(value: &ScpInfo) -> Self {
			value.clone()
		}
	}

	/// Contains the list of NF Profiles returned in a Discovery response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the list of NF Profiles returned in a
	/// Discovery response",
	///  "type": "object",
	///  "required": [
	///    "nfInstances"
	///  ],
	///  "properties": {
	///    "alteredPriorityInd": {
	///      "type": "boolean"
	///    },
	///    "nfInstanceList": {
	///      "description": "List of matching NF instances. The key of the map
	/// is the NF instance ID.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/NfInstanceInfo"
	///      }
	///    },
	///    "nfInstances": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFProfile"
	///      }
	///    },
	///    "noProfileMatchInfo": {
	///      "$ref": "#/components/schemas/NoProfileMatchInfo"
	///    },
	///    "nrfSupportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "numNfInstComplete": {
	///      "$ref": "#/components/schemas/Uint32"
	///    },
	///    "preferredSearch": {
	///      "$ref": "#/components/schemas/PreferredSearch"
	///    },
	///    "searchId": {
	///      "type": "string"
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
	pub struct SearchResult {
		#[serde(
			rename = "alteredPriorityInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub altered_priority_ind: Option<bool>,
		/// List of matching NF instances. The key of the map is the NF instance
		/// ID.
		#[serde(
			rename = "nfInstanceList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub nf_instance_list: ::std::collections::HashMap<String, NfInstanceInfo>,
		#[serde(rename = "nfInstances")]
		pub nf_instances: Vec<NfProfile>,
		#[serde(
			rename = "noProfileMatchInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub no_profile_match_info: Option<NoProfileMatchInfo>,
		#[serde(
			rename = "nrfSupportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "numNfInstComplete",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub num_nf_inst_complete: Option<Uint32>,
		#[serde(
			rename = "preferredSearch",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub preferred_search: Option<PreferredSearch>,
		#[serde(rename = "searchId", default, skip_serializing_if = "Option::is_none")]
		pub search_id: Option<String>,
		#[serde(
			rename = "validityPeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_period: Option<i64>,
	}

	impl From<&SearchResult> for SearchResult {
		fn from(value: &SearchResult) -> Self {
			value.clone()
		}
	}

	/// Information of a SEPP Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a SEPP Instance",
	///  "type": "object",
	///  "properties": {
	///    "remotePlmnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
	///      },
	///      "minItems": 1
	///    },
	///    "remoteSnpnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnIdNid"
	///      },
	///      "minItems": 1
	///    },
	///    "seppPorts": {
	///      "description": "Port numbers for HTTP and HTTPS. The key of the map
	/// shall be \"http\" or \"https\".\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "integer",
	///        "maximum": 65535.0,
	///        "minimum": 0.0
	///      }
	///    },
	///    "seppPrefix": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SeppInfo {
		#[serde(
			rename = "remotePlmnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub remote_plmn_list: Vec<PlmnId>,
		#[serde(
			rename = "remoteSnpnList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub remote_snpn_list: Vec<PlmnIdNid>,
		/// Port numbers for HTTP and HTTPS. The key of the map shall be "http"
		/// or "https".
		#[serde(
			rename = "seppPorts",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub sepp_ports: ::std::collections::HashMap<String, u16>,
		#[serde(
			rename = "seppPrefix",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sepp_prefix: Option<String>,
	}

	impl From<&SeppInfo> for SeppInfo {
		fn from(value: &SeppInfo) -> Self {
			value.clone()
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

	/// Subscription to a set of NFs based on their support for a given Service
	/// Name
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NFs based on their support for
	/// a given Service Name",
	///  "type": "object",
	///  "required": [
	///    "serviceName"
	///  ],
	///  "properties": {
	///    "serviceName": {
	///      "$ref": "#/components/schemas/ServiceName"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ServiceNameCond {
		#[serde(rename = "serviceName")]
		pub service_name: ServiceName,
	}

	impl From<&ServiceNameCond> for ServiceNameCond {
		fn from(value: &ServiceNameCond) -> Self {
			value.clone()
		}
	}

	/// Subscription to a set of NFs based on their support for a Service Name
	/// in the Servic Name list
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NFs based on their support for
	/// a Service Name in the Servic Name list\n",
	///  "type": "object",
	///  "required": [
	///    "conditionType",
	///    "serviceNameList"
	///  ],
	///  "properties": {
	///    "conditionType": {
	///      "type": "string",
	///      "enum": [
	///        "SERVICE_NAME_LIST_COND"
	///      ]
	///    },
	///    "serviceNameList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServiceName"
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
	pub struct ServiceNameListCond {
		#[serde(rename = "conditionType")]
		pub condition_type: ServiceNameListCondConditionType,
		#[serde(rename = "serviceNameList")]
		pub service_name_list: Vec<ServiceName>,
	}

	impl From<&ServiceNameListCond> for ServiceNameListCond {
		fn from(value: &ServiceNameListCond) -> Self {
			value.clone()
		}
	}

	/// ServiceNameListCondConditionType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "SERVICE_NAME_LIST_COND"
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
	pub enum ServiceNameListCondConditionType {
		#[default]
		#[serde(rename = "SERVICE_NAME_LIST_COND")]
		ServiceNameListCond,
	}

	impl From<&ServiceNameListCondConditionType> for ServiceNameListCondConditionType {
		fn from(value: &ServiceNameListCondConditionType) -> Self {
			value.clone()
		}
	}

	impl ToString for ServiceNameListCondConditionType {
		fn to_string(&self) -> String {
			match *self {
				Self::ServiceNameListCond => "SERVICE_NAME_LIST_COND".to_string(),
			}
		}
	}

	impl std::str::FromStr for ServiceNameListCondConditionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SERVICE_NAME_LIST_COND" => Ok(Self::ServiceNameListCond),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ServiceNameListCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ServiceNameListCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ServiceNameListCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SharedDataId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]{5,6}-.+$"
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
	pub struct SharedDataId(String);

	impl ::std::ops::Deref for SharedDataId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SharedDataId> for String {
		fn from(value: SharedDataId) -> Self {
			value.0
		}
	}

	impl From<&SharedDataId> for SharedDataId {
		fn from(value: &SharedDataId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SharedDataId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{5,6}-.+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{5,6}-.+$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SharedDataId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SharedDataId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SharedDataId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SharedDataId {
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

	/// A range of SharedDataIds based on regular-expression matching
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "A range of SharedDataIds based on regular-expression
	/// matching",
	///  "type": "object",
	///  "properties": {
	///    "pattern": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SharedDataIdRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pattern: Option<String>,
	}

	impl From<&SharedDataIdRange> for SharedDataIdRange {
		fn from(value: &SharedDataIdRange) -> Self {
			value.clone()
		}
	}

	/// Information of an SMF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an SMF NF Instance",
	///  "type": "object",
	///  "required": [
	///    "sNssaiSmfInfoList"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "ismfSupportInd": {
	///      "type": "boolean"
	///    },
	///    "pgwFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "pgwFqdnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Fqdn"
	///      },
	///      "minItems": 1
	///    },
	///    "pgwIpAddrList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpAddr"
	///      },
	///      "minItems": 1
	///    },
	///    "priority": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "sNssaiSmfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SnssaiSmfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "smfOnboardingCapability": {
	///      "default": false,
	///      "deprecated": true,
	///      "type": "boolean"
	///    },
	///    "smfUPRPCapability": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "vsmfSupportInd": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmfInfo {
		#[serde(rename = "accessType", default, skip_serializing_if = "Vec::is_empty")]
		pub access_type: Vec<AccessType>,
		#[serde(
			rename = "ismfSupportInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ismf_support_ind: Option<bool>,
		#[serde(rename = "pgwFqdn", default, skip_serializing_if = "Option::is_none")]
		pub pgw_fqdn: Option<Fqdn>,
		#[serde(rename = "pgwFqdnList", default, skip_serializing_if = "Vec::is_empty")]
		pub pgw_fqdn_list: Vec<Fqdn>,
		#[serde(
			rename = "pgwIpAddrList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pgw_ip_addr_list: Vec<IpAddr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub priority: Option<u16>,
		#[serde(rename = "sNssaiSmfInfoList")]
		pub s_nssai_smf_info_list: Vec<SnssaiSmfInfoItem>,
		#[serde(rename = "smfOnboardingCapability", default)]
		pub smf_onboarding_capability: bool,
		#[serde(rename = "smfUPRPCapability", default)]
		pub smf_uprp_capability: bool,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
		#[serde(
			rename = "vsmfSupportInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vsmf_support_ind: Option<bool>,
	}

	impl From<&SmfInfo> for SmfInfo {
		fn from(value: &SmfInfo) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by EASDF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by EASDF for a given
	/// S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "dnnEasdfInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnEasdfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DnnEasdfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SnssaiEasdfInfoItem {
		#[serde(rename = "dnnEasdfInfoList")]
		pub dnn_easdf_info_list: Vec<DnnEasdfInfoItem>,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SnssaiEasdfInfoItem> for SnssaiEasdfInfoItem {
		fn from(value: &SnssaiEasdfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Parameters supported by an NF for a given S-NSSAI Set of parameters
	/// supported by NF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Parameters supported by an NF for a given S-NSSAI Set
	/// of parameters supported by NF for a given S-NSSAI\n",
	///  "type": "object",
	///  "required": [
	///    "dnnInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DnnInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SnssaiInfoItem {
		#[serde(rename = "dnnInfoList")]
		pub dnn_info_list: Vec<DnnInfoItem>,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SnssaiInfoItem> for SnssaiInfoItem {
		fn from(value: &SnssaiInfoItem) -> Self {
			value.clone()
		}
	}

	/// Parameters supported by an MB-SMF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Parameters supported by an MB-SMF for a given S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "dnnInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DnnMbSmfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SnssaiMbSmfInfoItem {
		#[serde(rename = "dnnInfoList")]
		pub dnn_info_list: Vec<DnnMbSmfInfoItem>,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SnssaiMbSmfInfoItem> for SnssaiMbSmfInfoItem {
		fn from(value: &SnssaiMbSmfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by SMF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by SMF for a given
	/// S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "dnnSmfInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnSmfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DnnSmfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SnssaiSmfInfoItem {
		#[serde(rename = "dnnSmfInfoList")]
		pub dnn_smf_info_list: Vec<DnnSmfInfoItem>,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SnssaiSmfInfoItem> for SnssaiSmfInfoItem {
		fn from(value: &SnssaiSmfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by TSCTSF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by TSCTSF for a given
	/// S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "dnnInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DnnTsctsfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SnssaiTsctsfInfoItem {
		#[serde(rename = "dnnInfoList")]
		pub dnn_info_list: Vec<DnnTsctsfInfoItem>,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SnssaiTsctsfInfoItem> for SnssaiTsctsfInfoItem {
		fn from(value: &SnssaiTsctsfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Set of parameters supported by UPF for a given S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Set of parameters supported by UPF for a given
	/// S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "dnnUpfInfoList",
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "dnnUpfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DnnUpfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "redundantTransport": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/ExtSnssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SnssaiUpfInfoItem {
		#[serde(rename = "dnnUpfInfoList")]
		pub dnn_upf_info_list: Vec<DnnUpfInfoItem>,
		#[serde(rename = "redundantTransport", default)]
		pub redundant_transport: bool,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SnssaiUpfInfoItem> for SnssaiUpfInfoItem {
		fn from(value: &SnssaiUpfInfoItem) -> Self {
			value.clone()
		}
	}

	/// Overal status of the NRF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Overal status of the NRF",
	///  "type": "string",
	///  "enum": [
	///    "OPERATIVE",
	///    "NON_OPERATIVE"
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
	pub enum Status {
		#[default]
		#[serde(rename = "OPERATIVE")]
		Operative,
		#[serde(rename = "NON_OPERATIVE")]
		NonOperative,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&Status> for Status {
		fn from(value: &Status) -> Self {
			value.clone()
		}
	}

	impl ToString for Status {
		fn to_string(&self) -> String {
			match *self {
				Self::Operative => "OPERATIVE".to_string(),
				Self::NonOperative => "NON_OPERATIVE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for Status {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"OPERATIVE" => Ok(Self::Operative),
				"NON_OPERATIVE" => Ok(Self::NonOperative),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for Status {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Status {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Status {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains a complete search result (i.e. a number of discovered NF
	/// Instances), stored by NRF as a consequence of a prior search result
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains a complete search result (i.e. a number of
	/// discovered NF Instances), stored by NRF as a consequence of a prior
	/// search result\n",
	///  "type": "object",
	///  "required": [
	///    "nfInstances"
	///  ],
	///  "properties": {
	///    "nfInstances": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFProfile"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct StoredSearchResult {
		#[serde(rename = "nfInstances")]
		pub nf_instances: Vec<NfProfile>,
	}

	impl From<&StoredSearchResult> for StoredSearchResult {
		fn from(value: &StoredSearchResult) -> Self {
			value.clone()
		}
	}

	/// Condition to determine the set of NFs to monitor under a certain
	/// subscription in NRF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Condition to determine the set of NFs to monitor under
	/// a certain subscription in NRF\n",
	///  "oneOf": [
	///    {
	///      "$ref": "#/components/schemas/NfInstanceIdCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/NfInstanceIdListCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/NfTypeCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/ServiceNameCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/ServiceNameListCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/AmfCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/GuamiListCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/NetworkSliceCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/NfGroupCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/NfGroupListCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/NfSetCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/NfServiceSetCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/UpfCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/ScpDomainCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/NwdafCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/NefCond"
	///    },
	///    {
	///      "$ref": "#/components/schemas/DccfCond"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum SubscrCond {
		#[default]
		NfInstanceIdCond(NfInstanceIdCond),
		NfInstanceIdListCond(NfInstanceIdListCond),
		NfTypeCond(NfTypeCond),
		ServiceNameCond(ServiceNameCond),
		ServiceNameListCond(ServiceNameListCond),
		AmfCond(AmfCond),
		GuamiListCond(GuamiListCond),
		NetworkSliceCond(NetworkSliceCond),
		NfGroupCond(NfGroupCond),
		NfGroupListCond(NfGroupListCond),
		NfSetCond(NfSetCond),
		NfServiceSetCond(NfServiceSetCond),
		UpfCond(UpfCond),
		ScpDomainCond(ScpDomainCond),
		NwdafCond(NwdafCond),
		NefCond(NefCond),
		DccfCond(DccfCond),
	}

	impl From<&SubscrCond> for SubscrCond {
		fn from(value: &SubscrCond) -> Self {
			value.clone()
		}
	}

	impl From<NfInstanceIdCond> for SubscrCond {
		fn from(value: NfInstanceIdCond) -> Self {
			Self::NfInstanceIdCond(value)
		}
	}

	impl From<NfInstanceIdListCond> for SubscrCond {
		fn from(value: NfInstanceIdListCond) -> Self {
			Self::NfInstanceIdListCond(value)
		}
	}

	impl From<NfTypeCond> for SubscrCond {
		fn from(value: NfTypeCond) -> Self {
			Self::NfTypeCond(value)
		}
	}

	impl From<ServiceNameCond> for SubscrCond {
		fn from(value: ServiceNameCond) -> Self {
			Self::ServiceNameCond(value)
		}
	}

	impl From<ServiceNameListCond> for SubscrCond {
		fn from(value: ServiceNameListCond) -> Self {
			Self::ServiceNameListCond(value)
		}
	}

	impl From<AmfCond> for SubscrCond {
		fn from(value: AmfCond) -> Self {
			Self::AmfCond(value)
		}
	}

	impl From<GuamiListCond> for SubscrCond {
		fn from(value: GuamiListCond) -> Self {
			Self::GuamiListCond(value)
		}
	}

	impl From<NetworkSliceCond> for SubscrCond {
		fn from(value: NetworkSliceCond) -> Self {
			Self::NetworkSliceCond(value)
		}
	}

	impl From<NfGroupCond> for SubscrCond {
		fn from(value: NfGroupCond) -> Self {
			Self::NfGroupCond(value)
		}
	}

	impl From<NfGroupListCond> for SubscrCond {
		fn from(value: NfGroupListCond) -> Self {
			Self::NfGroupListCond(value)
		}
	}

	impl From<NfSetCond> for SubscrCond {
		fn from(value: NfSetCond) -> Self {
			Self::NfSetCond(value)
		}
	}

	impl From<NfServiceSetCond> for SubscrCond {
		fn from(value: NfServiceSetCond) -> Self {
			Self::NfServiceSetCond(value)
		}
	}

	impl From<UpfCond> for SubscrCond {
		fn from(value: UpfCond) -> Self {
			Self::UpfCond(value)
		}
	}

	impl From<ScpDomainCond> for SubscrCond {
		fn from(value: ScpDomainCond) -> Self {
			Self::ScpDomainCond(value)
		}
	}

	impl From<NwdafCond> for SubscrCond {
		fn from(value: NwdafCond) -> Self {
			Self::NwdafCond(value)
		}
	}

	impl From<NefCond> for SubscrCond {
		fn from(value: NefCond) -> Self {
			Self::NefCond(value)
		}
	}

	impl From<DccfCond> for SubscrCond {
		fn from(value: DccfCond) -> Self {
			Self::DccfCond(value)
		}
	}

	/// Context data related to a created subscription, to be included in
	/// notifications sent by NRF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Context data related to a created subscription, to be
	/// included in notifications sent by NRF\n",
	///  "type": "object",
	///  "required": [
	///    "subscriptionId"
	///  ],
	///  "properties": {
	///    "subscrCond": {
	///      "$ref": "#/components/schemas/SubscrCond"
	///    },
	///    "subscriptionId": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SubscriptionContext {
		#[serde(
			rename = "subscrCond",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscr_cond: Option<SubscrCond>,
		#[serde(rename = "subscriptionId")]
		pub subscription_id: String,
	}

	impl From<&SubscriptionContext> for SubscriptionContext {
		fn from(value: &SubscriptionContext) -> Self {
			value.clone()
		}
	}

	/// Information of a subscription to notifications to NRF events, included
	/// in subscription requests and responses
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a subscription to notifications to NRF
	/// events, included in subscription requests and responses\n",
	///  "type": "object",
	///  "required": [
	///    "nfStatusNotificationUri",
	///    "subscriptionId"
	///  ],
	///  "properties": {
	///    "hnrfUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nfStatusNotificationUri": {
	///      "type": "string"
	///    },
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    },
	///    "notifCondition": {
	///      "$ref": "#/components/schemas/NotifCondition"
	///    },
	///    "nrfSupportedFeatures": {
	///      "readOnly": true,
	///      "allOf": [
	///        {
	///          "$ref": "#/components/schemas/SupportedFeatures"
	///        }
	///      ]
	///    },
	///    "onboardingCapability": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "preferredLocality": {
	///      "type": "string"
	///    },
	///    "reqNfFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "reqNfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "reqNfType": {
	///      "$ref": "#/components/schemas/NFType"
	///    },
	///    "reqNotifEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NotificationEventType"
	///      },
	///      "minItems": 1
	///    },
	///    "reqPerPlmnSnssais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "reqPlmnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnId"
	///      },
	///      "minItems": 1
	///    },
	///    "reqSnpnList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PlmnIdNid"
	///      },
	///      "minItems": 1
	///    },
	///    "reqSnssais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "requesterFeatures": {
	///      "writeOnly": true,
	///      "allOf": [
	///        {
	///          "$ref": "#/components/schemas/SupportedFeatures"
	///        }
	///      ]
	///    },
	///    "servingScope": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "subscrCond": {
	///      "$ref": "#/components/schemas/SubscrCond"
	///    },
	///    "subscriptionId": {
	///      "readOnly": true,
	///      "type": "string",
	///      "pattern": "^([0-9]{5,6}-(x3Lf57A:nid=[A-Fa-f0-9]{11}:)?)?[^-]+$"
	///    },
	///    "targetHni": {
	///      "$ref": "#/components/schemas/Fqdn"
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
	pub struct SubscriptionData {
		#[serde(rename = "hnrfUri", default, skip_serializing_if = "Option::is_none")]
		pub hnrf_uri: Option<Uri>,
		#[serde(rename = "nfStatusNotificationUri")]
		pub nf_status_notification_uri: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
		#[serde(
			rename = "notifCondition",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notif_condition: Option<NotifCondition>,
		#[serde(
			rename = "nrfSupportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_supported_features: Option<SupportedFeatures>,
		#[serde(rename = "onboardingCapability", default)]
		pub onboarding_capability: bool,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnId>,
		#[serde(
			rename = "preferredLocality",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub preferred_locality: Option<String>,
		#[serde(rename = "reqNfFqdn", default, skip_serializing_if = "Option::is_none")]
		pub req_nf_fqdn: Option<Fqdn>,
		#[serde(
			rename = "reqNfInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub req_nf_instance_id: Option<NfInstanceId>,
		#[serde(rename = "reqNfType", default, skip_serializing_if = "Option::is_none")]
		pub req_nf_type: Option<NfType>,
		#[serde(
			rename = "reqNotifEvents",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub req_notif_events: Vec<NotificationEventType>,
		#[serde(
			rename = "reqPerPlmnSnssais",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub req_per_plmn_snssais: Vec<PlmnSnssai>,
		#[serde(rename = "reqPlmnList", default, skip_serializing_if = "Vec::is_empty")]
		pub req_plmn_list: Vec<PlmnId>,
		#[serde(rename = "reqSnpnList", default, skip_serializing_if = "Vec::is_empty")]
		pub req_snpn_list: Vec<PlmnIdNid>,
		#[serde(rename = "reqSnssais", default, skip_serializing_if = "Vec::is_empty")]
		pub req_snssais: Vec<ExtSnssai>,
		#[serde(
			rename = "requesterFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub requester_features: Option<SupportedFeatures>,
		#[serde(
			rename = "servingScope",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub serving_scope: Vec<String>,
		#[serde(
			rename = "subscrCond",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscr_cond: Option<SubscrCond>,
		#[serde(rename = "subscriptionId")]
		pub subscription_id: SubscriptionDataSubscriptionId,
		#[serde(rename = "targetHni", default, skip_serializing_if = "Option::is_none")]
		pub target_hni: Option<Fqdn>,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&SubscriptionData> for SubscriptionData {
		fn from(value: &SubscriptionData) -> Self {
			value.clone()
		}
	}

	/// SubscriptionDataSubscriptionId
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "readOnly": true,
	///  "type": "string",
	///  "pattern": "^([0-9]{5,6}-(x3Lf57A:nid=[A-Fa-f0-9]{11}:)?)?[^-]+$"
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
	pub struct SubscriptionDataSubscriptionId(String);

	impl ::std::ops::Deref for SubscriptionDataSubscriptionId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SubscriptionDataSubscriptionId> for String {
		fn from(value: SubscriptionDataSubscriptionId) -> Self {
			value.0
		}
	}

	impl From<&SubscriptionDataSubscriptionId> for SubscriptionDataSubscriptionId {
		fn from(value: &SubscriptionDataSubscriptionId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SubscriptionDataSubscriptionId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^([0-9]{5,6}-(x3Lf57A:nid=[A-Fa-f0-9]{11}:)?)?[^-]+$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \
				            \"^([0-9]{5,6}-(x3Lf57A:nid=[A-Fa-f0-9]{11}:)?)?[^-]+$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for SubscriptionDataSubscriptionId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SubscriptionDataSubscriptionId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SubscriptionDataSubscriptionId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SubscriptionDataSubscriptionId {
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

	/// SUCI information containing Routing Indicator and Home Network Public
	/// Key ID
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "SUCI information containing Routing Indicator and Home
	/// Network Public Key ID",
	///  "type": "object",
	///  "properties": {
	///    "hNwPubKeyIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "integer"
	///      },
	///      "minItems": 1
	///    },
	///    "routingInds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string",
	///        "pattern": "^[0-9]{1,4}$"
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
	pub struct SuciInfo {
		#[serde(
			rename = "hNwPubKeyIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub h_nw_pub_key_ids: Vec<i64>,
		#[serde(rename = "routingInds", default, skip_serializing_if = "Vec::is_empty")]
		pub routing_inds: Vec<SuciInfoRoutingIndsItem>,
	}

	impl From<&SuciInfo> for SuciInfo {
		fn from(value: &SuciInfo) -> Self {
			value.clone()
		}
	}

	/// SuciInfoRoutingIndsItem
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
	pub struct SuciInfoRoutingIndsItem(String);

	impl ::std::ops::Deref for SuciInfoRoutingIndsItem {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SuciInfoRoutingIndsItem> for String {
		fn from(value: SuciInfoRoutingIndsItem) -> Self {
			value.0
		}
	}

	impl From<&SuciInfoRoutingIndsItem> for SuciInfoRoutingIndsItem {
		fn from(value: &SuciInfoRoutingIndsItem) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SuciInfoRoutingIndsItem {
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

	impl ::std::convert::TryFrom<&str> for SuciInfoRoutingIndsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SuciInfoRoutingIndsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SuciInfoRoutingIndsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SuciInfoRoutingIndsItem {
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

	/// Range of TACs (Tracking Area Codes)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of TACs (Tracking Area Codes)",
	///  "type": "object",
	///  "properties": {
	///    "end": {
	///      "type": "string",
	///      "pattern": "^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$"
	///    },
	///    "pattern": {
	///      "type": "string"
	///    },
	///    "start": {
	///      "type": "string",
	///      "pattern": "^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TacRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub end: Option<TacRangeEnd>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pattern: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<TacRangeStart>,
	}

	impl From<&TacRange> for TacRange {
		fn from(value: &TacRange) -> Self {
			value.clone()
		}
	}

	/// TacRangeEnd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$"
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
	pub struct TacRangeEnd(String);

	impl ::std::ops::Deref for TacRangeEnd {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TacRangeEnd> for String {
		fn from(value: TacRangeEnd) -> Self {
			value.0
		}
	}

	impl From<&TacRangeEnd> for TacRangeEnd {
		fn from(value: &TacRangeEnd) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TacRangeEnd {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for TacRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TacRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TacRangeEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TacRangeEnd {
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

	/// TacRangeStart
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$"
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
	pub struct TacRangeStart(String);

	impl ::std::ops::Deref for TacRangeStart {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TacRangeStart> for String {
		fn from(value: TacRangeStart) -> Self {
			value.0
		}
	}

	impl From<&TacRangeStart> for TacRangeStart {
		fn from(value: &TacRangeStart) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TacRangeStart {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^([A-Fa-f0-9]{4}|[A-Fa-f0-9]{6})$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for TacRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TacRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TacRangeStart {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TacRangeStart {
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

	/// Range of TAIs (Tracking Area Identities)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of TAIs (Tracking Area Identities)",
	///  "type": "object",
	///  "required": [
	///    "plmnId",
	///    "tacRangeList"
	///  ],
	///  "properties": {
	///    "nid": {
	///      "$ref": "#/components/schemas/Nid"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "tacRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TacRange"
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
	pub struct TaiRange {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
		#[serde(rename = "tacRangeList")]
		pub tac_range_list: Vec<TacRange>,
	}

	impl From<&TaiRange> for TaiRange {
		fn from(value: &TaiRange) -> Self {
			value.clone()
		}
	}

	/// Range of TMGIs
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Range of TMGIs",
	///  "type": "object",
	///  "required": [
	///    "mbsServiceIdEnd",
	///    "mbsServiceIdStart",
	///    "plmnId"
	///  ],
	///  "properties": {
	///    "mbsServiceIdEnd": {
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{6}$"
	///    },
	///    "mbsServiceIdStart": {
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]{6}$"
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
	pub struct TmgiRange {
		#[serde(rename = "mbsServiceIdEnd")]
		pub mbs_service_id_end: TmgiRangeMbsServiceIdEnd,
		#[serde(rename = "mbsServiceIdStart")]
		pub mbs_service_id_start: TmgiRangeMbsServiceIdStart,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nid: Option<Nid>,
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
	}

	impl From<&TmgiRange> for TmgiRange {
		fn from(value: &TmgiRange) -> Self {
			value.clone()
		}
	}

	/// TmgiRangeMbsServiceIdEnd
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub struct TmgiRangeMbsServiceIdEnd(String);

	impl ::std::ops::Deref for TmgiRangeMbsServiceIdEnd {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TmgiRangeMbsServiceIdEnd> for String {
		fn from(value: TmgiRangeMbsServiceIdEnd) -> Self {
			value.0
		}
	}

	impl From<&TmgiRangeMbsServiceIdEnd> for TmgiRangeMbsServiceIdEnd {
		fn from(value: &TmgiRangeMbsServiceIdEnd) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TmgiRangeMbsServiceIdEnd {
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

	impl ::std::convert::TryFrom<&str> for TmgiRangeMbsServiceIdEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TmgiRangeMbsServiceIdEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TmgiRangeMbsServiceIdEnd {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TmgiRangeMbsServiceIdEnd {
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

	/// TmgiRangeMbsServiceIdStart
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub struct TmgiRangeMbsServiceIdStart(String);

	impl ::std::ops::Deref for TmgiRangeMbsServiceIdStart {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<TmgiRangeMbsServiceIdStart> for String {
		fn from(value: TmgiRangeMbsServiceIdStart) -> Self {
			value.0
		}
	}

	impl From<&TmgiRangeMbsServiceIdStart> for TmgiRangeMbsServiceIdStart {
		fn from(value: &TmgiRangeMbsServiceIdStart) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for TmgiRangeMbsServiceIdStart {
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

	impl ::std::convert::TryFrom<&str> for TmgiRangeMbsServiceIdStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for TmgiRangeMbsServiceIdStart {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for TmgiRangeMbsServiceIdStart {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for TmgiRangeMbsServiceIdStart {
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

	/// Types of transport protocol used in a given IP endpoint of an NF Service
	/// Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Types of transport protocol used in a given IP endpoint
	/// of an NF Service Instance",
	///  "type": "string",
	///  "enum": [
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
		#[serde(rename = "TCP")]
		Tcp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl ToString for TransportProtocol {
		fn to_string(&self) -> String {
			match *self {
				Self::Tcp => "TCP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TransportProtocol {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TCP" => Ok(Self::Tcp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	/// Information of a trusted AF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a trusted AF Instance",
	///  "type": "object",
	///  "properties": {
	///    "afEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AfEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "appIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "internalGroupId": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "mappingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "sNssaiInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SnssaiInfoItem"
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
	pub struct TrustAfInfo {
		#[serde(rename = "afEvents", default, skip_serializing_if = "Vec::is_empty")]
		pub af_events: Vec<AfEvent>,
		#[serde(rename = "appIds", default, skip_serializing_if = "Vec::is_empty")]
		pub app_ids: Vec<String>,
		#[serde(
			rename = "internalGroupId",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub internal_group_id: Vec<GroupId>,
		#[serde(rename = "mappingInd", default)]
		pub mapping_ind: bool,
		#[serde(
			rename = "sNssaiInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub s_nssai_info_list: Vec<SnssaiInfoItem>,
	}

	impl From<&TrustAfInfo> for TrustAfInfo {
		fn from(value: &TrustAfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a TSCTSF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a TSCTSF NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "externalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	///    "internalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/InternalGroupIdRange"
	///      },
	///      "minItems": 1
	///    },
	///    "sNssaiInfoList": {
	///      "description": "A map (list of key-value pairs) where a valid JSON
	/// string serves as key",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SnssaiTsctsfInfoItem"
	///      }
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct TsctsfInfo {
		#[serde(
			rename = "externalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_group_identifiers_ranges: Vec<IdentityRange>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(
			rename = "internalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub internal_group_identifiers_ranges: Vec<InternalGroupIdRange>,
		/// A map (list of key-value pairs) where a valid JSON string serves as
		/// key
		#[serde(
			rename = "sNssaiInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub s_nssai_info_list: ::std::collections::HashMap<String, SnssaiTsctsfInfoItem>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&TsctsfInfo> for TsctsfInfo {
		fn from(value: &TsctsfInfo) -> Self {
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

	/// Information of an UDM NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an UDM NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "externalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "internalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/InternalGroupIdRange"
	///      },
	///      "minItems": 1
	///    },
	///    "routingIndicators": {
	///      "type": "array",
	///      "items": {
	///        "type": "string",
	///        "pattern": "^[0-9]{1,4}$"
	///      },
	///      "minItems": 1
	///    },
	///    "suciInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SuciInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct UdmInfo {
		#[serde(
			rename = "externalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_group_identifiers_ranges: Vec<IdentityRange>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "internalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub internal_group_identifiers_ranges: Vec<InternalGroupIdRange>,
		#[serde(
			rename = "routingIndicators",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub routing_indicators: Vec<UdmInfoRoutingIndicatorsItem>,
		#[serde(rename = "suciInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub suci_infos: Vec<SuciInfo>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&UdmInfo> for UdmInfo {
		fn from(value: &UdmInfo) -> Self {
			value.clone()
		}
	}

	/// UdmInfoRoutingIndicatorsItem
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
	pub struct UdmInfoRoutingIndicatorsItem(String);

	impl ::std::ops::Deref for UdmInfoRoutingIndicatorsItem {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<UdmInfoRoutingIndicatorsItem> for String {
		fn from(value: UdmInfoRoutingIndicatorsItem) -> Self {
			value.0
		}
	}

	impl From<&UdmInfoRoutingIndicatorsItem> for UdmInfoRoutingIndicatorsItem {
		fn from(value: &UdmInfoRoutingIndicatorsItem) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for UdmInfoRoutingIndicatorsItem {
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

	impl ::std::convert::TryFrom<&str> for UdmInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for UdmInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for UdmInfoRoutingIndicatorsItem {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for UdmInfoRoutingIndicatorsItem {
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

	/// Information of an UDR NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an UDR NF Instance",
	///  "type": "object",
	///  "properties": {
	///    "externalGroupIdentifiersRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IdentityRange"
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
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "sharedDataIdRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SharedDataIdRange"
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
	///    "supportedDataSets": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DataSetId"
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
	pub struct UdrInfo {
		#[serde(
			rename = "externalGroupIdentifiersRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub external_group_identifiers_ranges: Vec<IdentityRange>,
		#[serde(rename = "gpsiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_ranges: Vec<IdentityRange>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		#[serde(
			rename = "sharedDataIdRanges",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub shared_data_id_ranges: Vec<SharedDataIdRange>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
		#[serde(
			rename = "supportedDataSets",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub supported_data_sets: Vec<DataSetId>,
	}

	impl From<&UdrInfo> for UdrInfo {
		fn from(value: &UdrInfo) -> Self {
			value.clone()
		}
	}

	/// Information related to UDSF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information related to UDSF",
	///  "type": "object",
	///  "properties": {
	///    "groupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "storageIdRanges": {
	///      "description": "A map (list of key-value pairs) where realmId
	/// serves as key and each value in the map is an array of IdentityRanges.
	/// Each IdentityRange is a range of storageIds.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "type": "array",
	///        "items": {
	///          "$ref": "#/components/schemas/IdentityRange"
	///        },
	///        "minItems": 1
	///      }
	///    },
	///    "supiRanges": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupiRange"
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
	pub struct UdsfInfo {
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<NfGroupId>,
		/// A map (list of key-value pairs) where realmId serves as key and each
		/// value in the map is an array of IdentityRanges. Each IdentityRange
		/// is a range of storageIds.
		#[serde(
			rename = "storageIdRanges",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub storage_id_ranges: ::std::collections::HashMap<String, Vec<IdentityRange>>,
		#[serde(rename = "supiRanges", default, skip_serializing_if = "Vec::is_empty")]
		pub supi_ranges: Vec<SupiRange>,
	}

	impl From<&UdsfInfo> for UdsfInfo {
		fn from(value: &UdsfInfo) -> Self {
			value.clone()
		}
	}

	/// Information of a untrusted AF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of a untrusted AF Instance",
	///  "type": "object",
	///  "required": [
	///    "afId"
	///  ],
	///  "properties": {
	///    "afId": {
	///      "type": "string"
	///    },
	///    "mappingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "sNssaiInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SnssaiInfoItem"
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
	pub struct UnTrustAfInfo {
		#[serde(rename = "afId")]
		pub af_id: String,
		#[serde(rename = "mappingInd", default)]
		pub mapping_ind: bool,
		#[serde(
			rename = "sNssaiInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub s_nssai_info_list: Vec<SnssaiInfoItem>,
	}

	impl From<&UnTrustAfInfo> for UnTrustAfInfo {
		fn from(value: &UnTrustAfInfo) -> Self {
			value.clone()
		}
	}

	/// Types of User-Plane interfaces of the UPF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Types of User-Plane interfaces of the UPF",
	///  "type": "string",
	///  "enum": [
	///    "N3",
	///    "N6",
	///    "N9",
	///    "DATA_FORWARDING",
	///    "N3MB",
	///    "N6MB",
	///    "N19MB",
	///    "NMB9"
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
	pub enum UpInterfaceType {
		#[default]
		N3,
		N6,
		N9,
		#[serde(rename = "DATA_FORWARDING")]
		DataForwarding,
		#[serde(rename = "N3MB")]
		N3mb,
		#[serde(rename = "N6MB")]
		N6mb,
		#[serde(rename = "N19MB")]
		N19mb,
		#[serde(rename = "NMB9")]
		Nmb9,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UpInterfaceType> for UpInterfaceType {
		fn from(value: &UpInterfaceType) -> Self {
			value.clone()
		}
	}

	impl ToString for UpInterfaceType {
		fn to_string(&self) -> String {
			match *self {
				Self::N3 => "N3".to_string(),
				Self::N6 => "N6".to_string(),
				Self::N9 => "N9".to_string(),
				Self::DataForwarding => "DATA_FORWARDING".to_string(),
				Self::N3mb => "N3MB".to_string(),
				Self::N6mb => "N6MB".to_string(),
				Self::N19mb => "N19MB".to_string(),
				Self::Nmb9 => "NMB9".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UpInterfaceType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"N3" => Ok(Self::N3),
				"N6" => Ok(Self::N6),
				"N9" => Ok(Self::N9),
				"DATA_FORWARDING" => Ok(Self::DataForwarding),
				"N3MB" => Ok(Self::N3mb),
				"N6MB" => Ok(Self::N6mb),
				"N19MB" => Ok(Self::N19mb),
				"NMB9" => Ok(Self::Nmb9),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UpInterfaceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UpInterfaceType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UpInterfaceType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Subscription to a set of NF Instances (UPFs), able to serve a certain
	/// service area (i.e. SMF serving area or TAI list)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Subscription to a set of NF Instances (UPFs), able to
	/// serve a certain service area (i.e. SMF serving area or TAI list)\n",
	///  "type": "object",
	///  "required": [
	///    "conditionType"
	///  ],
	///  "properties": {
	///    "conditionType": {
	///      "type": "string",
	///      "enum": [
	///        "UPF_COND"
	///      ]
	///    },
	///    "smfServingArea": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "taiList": {
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
	pub struct UpfCond {
		#[serde(rename = "conditionType")]
		pub condition_type: UpfCondConditionType,
		#[serde(
			rename = "smfServingArea",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub smf_serving_area: Vec<String>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
	}

	impl From<&UpfCond> for UpfCond {
		fn from(value: &UpfCond) -> Self {
			value.clone()
		}
	}

	/// UpfCondConditionType
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "UPF_COND"
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
	pub enum UpfCondConditionType {
		#[default]
		#[serde(rename = "UPF_COND")]
		UpfCond,
	}

	impl From<&UpfCondConditionType> for UpfCondConditionType {
		fn from(value: &UpfCondConditionType) -> Self {
			value.clone()
		}
	}

	impl ToString for UpfCondConditionType {
		fn to_string(&self) -> String {
			match *self {
				Self::UpfCond => "UPF_COND".to_string(),
			}
		}
	}

	impl std::str::FromStr for UpfCondConditionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UPF_COND" => Ok(Self::UpfCond),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UpfCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UpfCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UpfCondConditionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Information of an UPF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an UPF NF Instance",
	///  "type": "object",
	///  "required": [
	///    "sNssaiUpfInfoList"
	///  ],
	///  "properties": {
	///    "atsssCapability": {
	///      "$ref": "#/components/schemas/AtsssCapability"
	///    },
	///    "dataForwarding": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "interfaceUpfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/InterfaceUpfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "ipups": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "iwkEpsInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "pduSessionTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionType"
	///      },
	///      "minItems": 1
	///    },
	///    "priority": {
	///      "type": "integer",
	///      "maximum": 65535.0,
	///      "minimum": 0.0
	///    },
	///    "redundantGtpu": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "sNssaiUpfInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SnssaiUpfInfoItem"
	///      },
	///      "minItems": 1
	///    },
	///    "smfServingArea": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedPfcpFeatures": {
	///      "type": "string"
	///    },
	///    "taiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Tai"
	///      },
	///      "minItems": 1
	///    },
	///    "taiRangeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TaiRange"
	///      },
	///      "minItems": 1
	///    },
	///    "tngfInfo": {
	///      "$ref": "#/components/schemas/TngfInfo"
	///    },
	///    "twifInfo": {
	///      "$ref": "#/components/schemas/TwifInfo"
	///    },
	///    "ueIpAddrInd": {
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
	pub struct UpfInfo {
		#[serde(
			rename = "atsssCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub atsss_capability: Option<AtsssCapability>,
		#[serde(rename = "dataForwarding", default)]
		pub data_forwarding: bool,
		#[serde(
			rename = "interfaceUpfInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub interface_upf_info_list: Vec<InterfaceUpfInfoItem>,
		#[serde(default)]
		pub ipups: bool,
		#[serde(rename = "iwkEpsInd", default)]
		pub iwk_eps_ind: bool,
		#[serde(
			rename = "pduSessionTypes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pdu_session_types: Vec<PduSessionType>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub priority: Option<u16>,
		#[serde(rename = "redundantGtpu", default)]
		pub redundant_gtpu: bool,
		#[serde(rename = "sNssaiUpfInfoList")]
		pub s_nssai_upf_info_list: Vec<SnssaiUpfInfoItem>,
		#[serde(
			rename = "smfServingArea",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub smf_serving_area: Vec<String>,
		#[serde(
			rename = "supportedPfcpFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_pfcp_features: Option<String>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
		#[serde(rename = "tngfInfo", default, skip_serializing_if = "Option::is_none")]
		pub tngf_info: Option<TngfInfo>,
		#[serde(rename = "twifInfo", default, skip_serializing_if = "Option::is_none")]
		pub twif_info: Option<TwifInfo>,
		#[serde(rename = "ueIpAddrInd", default)]
		pub ue_ip_addr_ind: bool,
		#[serde(rename = "wAgfInfo", default, skip_serializing_if = "Option::is_none")]
		pub w_agf_info: Option<WAgfInfo>,
	}

	impl From<&UpfInfo> for UpfInfo {
		fn from(value: &UpfInfo) -> Self {
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
	//    #[derive(
	//        :: serde :: Deserialize,
	//        :: serde :: Serialize,
	//        Clone,
	//        Debug,
	//        Eq,
	//        Hash,
	//        Ord,
	//        PartialEq,
	//        PartialOrd,
	//        smart_default::SmartDefault,
	//    )]
	//    pub struct Uri(pub String);
	//    impl ::std::ops::Deref for Uri {
	//        type Target = String;
	//        fn deref(&self) -> &String {
	//            &self.0
	//        }
	//    }
	//
	//    impl From<Uri> for String {
	//        fn from(value: Uri) -> Self {
	//            value.0
	//        }
	//    }
	//
	//    impl From<&Uri> for Uri {
	//        fn from(value: &Uri) -> Self {
	//            value.clone()
	//        }
	//    }
	//
	//    impl From<String> for Uri {
	//        fn from(value: String) -> Self {
	//            Self(value)
	//        }
	//    }

	/// Represents a set of URIs following the 3GPP hypermedia format
	/// (containing a "_links" attribute).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a set of URIs following the 3GPP hypermedia
	/// format (containing a \"_links\" attribute).\n",
	///  "type": "object",
	///  "properties": {
	///    "_links": {
	///      "description": "List of the URI of NF instances. It has two members
	/// whose names are item and self. The item attribute contains an array of
	/// URIs.\n",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/LinksValueSchema"
	///      }
	///    },
	///    "totalItemCount": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UriList {
		/// List of the URI of NF instances. It has two members whose names are
		/// item and self. The item attribute contains an array of URIs.
		#[serde(
			rename = "_links",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub links: ::std::collections::HashMap<String, LinksValueSchema>,
		#[serde(
			rename = "totalItemCount",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub total_item_count: Option<i64>,
	}

	impl From<&UriList> for UriList {
		fn from(value: &UriList) -> Self {
			value.clone()
		}
	}

	/// Indicate the supported V2X Capability by the PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicate the supported V2X Capability by the PCF.",
	///  "type": "object",
	///  "properties": {
	///    "lteV2x": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "nrV2x": {
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
	pub struct V2xCapability {
		#[serde(rename = "lteV2x", default)]
		pub lte_v2x: bool,
		#[serde(rename = "nrV2x", default)]
		pub nr_v2x: bool,
	}

	impl From<&V2xCapability> for V2xCapability {
		fn from(value: &V2xCapability) -> Self {
			value.clone()
		}
	}

	/// Vendor ID of the NF Service instance (Private Enterprise Number assigned
	/// by IANA)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Vendor ID of the NF Service instance (Private
	/// Enterprise Number assigned by IANA)",
	///  "type": "string",
	///  "pattern": "^[0-9]{6}$"
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
	pub struct VendorId(String);

	impl ::std::ops::Deref for VendorId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<VendorId> for String {
		fn from(value: VendorId) -> Self {
			value.0
		}
	}

	impl From<&VendorId> for VendorId {
		fn from(value: &VendorId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for VendorId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{6}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{6}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for VendorId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for VendorId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for VendorId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for VendorId {
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

	/// Information about a vendor-specific feature
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information about a vendor-specific feature",
	///  "type": "object",
	///  "required": [
	///    "featureName",
	///    "featureVersion"
	///  ],
	///  "properties": {
	///    "featureName": {
	///      "type": "string"
	///    },
	///    "featureVersion": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VendorSpecificFeature {
		#[serde(rename = "featureName")]
		pub feature_name: String,
		#[serde(rename = "featureVersion")]
		pub feature_version: String,
	}

	impl From<&VendorSpecificFeature> for VendorSpecificFeature {
		fn from(value: &VendorSpecificFeature) -> Self {
			value.clone()
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

	/// Wildcard DNAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Wildcard DNAI",
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
	)]
	pub struct WildcardDnai(String);

	impl ::std::ops::Deref for WildcardDnai {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<WildcardDnai> for String {
		fn from(value: WildcardDnai) -> Self {
			value.0
		}
	}

	impl From<&WildcardDnai> for WildcardDnai {
		fn from(value: &WildcardDnai) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for WildcardDnai {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[*]$").unwrap().find(value).is_none() {
				return Err("doesn't match pattern \"^[*]$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for WildcardDnai {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for WildcardDnai {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for WildcardDnai {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for WildcardDnai {
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

	/// Information of an 5G DDNMF NF Instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of an 5G DDNMF NF Instance",
	///  "type": "object",
	///  "required": [
	///    "plmnId"
	///  ],
	///  "properties": {
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
	pub struct _5gDdnmfInfo {
		#[serde(rename = "plmnId")]
		pub plmn_id: PlmnId,
	}

	impl From<&_5gDdnmfInfo> for _5gDdnmfInfo {
		fn from(value: &_5gDdnmfInfo) -> Self {
			value.clone()
		}
	}
}
