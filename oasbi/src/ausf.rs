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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
		NewUnchecked,
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
}
