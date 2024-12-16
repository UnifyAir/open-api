#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};

#[allow(unused_imports)]
use crate::progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use crate::progenitor_client::{ByteStream, Error, ResponseValue};

/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
	pub use crate::common::{common_models::*, *};

	/// Represents the abnormal behaviour information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the abnormal behaviour information.",
	///  "type": "object",
	///  "required": [
	///    "excep"
	///  ],
	///  "properties": {
	///    "addtMeasInfo": {
	///      "$ref": "#/components/schemas/AdditionalMeasurement"
	///    },
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "excep": {
	///      "$ref": "#/components/schemas/Exception"
	///    },
	///    "ratio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "supis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
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
	pub struct AbnormalBehaviour {
		#[serde(
			rename = "addtMeasInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub addt_meas_info: Option<AdditionalMeasurement>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub confidence: Option<Uinteger>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		pub excep: Exception,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ratio: Option<SamplingRatio>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub supis: Vec<Supi>,
	}

	impl From<&AbnormalBehaviour> for AbnormalBehaviour {
		fn from(value: &AbnormalBehaviour) -> Self {
			value.clone()
		}
	}

	/// Access State Transition Type.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Access State Transition Type.",
	///  "type": "string",
	///  "enum": [
	///    "ACCESS_TYPE_CHANGE_3GPP",
	///    "ACCESS_TYPE_CHANGE_N3GPP",
	///    "RM_STATE_CHANGE_DEREGISTERED",
	///    "RM_STATE_CHANGE_REGISTERED",
	///    "CM_STATE_CHANGE_IDLE",
	///    "CM_STATE_CHANGE_CONNECTED",
	///    "HANDOVER",
	///    "MOBILITY_REGISTRATION_UPDATE"
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
	pub enum AccessStateTransitionType {
		#[default]
		#[serde(rename = "ACCESS_TYPE_CHANGE_3GPP")]
		AccessTypeChange3gpp,
		#[serde(rename = "ACCESS_TYPE_CHANGE_N3GPP")]
		AccessTypeChangeN3gpp,
		#[serde(rename = "RM_STATE_CHANGE_DEREGISTERED")]
		RmStateChangeDeregistered,
		#[serde(rename = "RM_STATE_CHANGE_REGISTERED")]
		RmStateChangeRegistered,
		#[serde(rename = "CM_STATE_CHANGE_IDLE")]
		CmStateChangeIdle,
		#[serde(rename = "CM_STATE_CHANGE_CONNECTED")]
		CmStateChangeConnected,
		#[serde(rename = "HANDOVER")]
		Handover,
		#[serde(rename = "MOBILITY_REGISTRATION_UPDATE")]
		MobilityRegistrationUpdate,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AccessStateTransitionType> for AccessStateTransitionType {
		fn from(value: &AccessStateTransitionType) -> Self {
			value.clone()
		}
	}

	impl ToString for AccessStateTransitionType {
		fn to_string(&self) -> String {
			match *self {
				Self::AccessTypeChange3gpp => "ACCESS_TYPE_CHANGE_3GPP".to_string(),
				Self::AccessTypeChangeN3gpp => "ACCESS_TYPE_CHANGE_N3GPP".to_string(),
				Self::RmStateChangeDeregistered => "RM_STATE_CHANGE_DEREGISTERED".to_string(),
				Self::RmStateChangeRegistered => "RM_STATE_CHANGE_REGISTERED".to_string(),
				Self::CmStateChangeIdle => "CM_STATE_CHANGE_IDLE".to_string(),
				Self::CmStateChangeConnected => "CM_STATE_CHANGE_CONNECTED".to_string(),
				Self::Handover => "HANDOVER".to_string(),
				Self::MobilityRegistrationUpdate => "MOBILITY_REGISTRATION_UPDATE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AccessStateTransitionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ACCESS_TYPE_CHANGE_3GPP" => Ok(Self::AccessTypeChange3gpp),
				"ACCESS_TYPE_CHANGE_N3GPP" => Ok(Self::AccessTypeChangeN3gpp),
				"RM_STATE_CHANGE_DEREGISTERED" => Ok(Self::RmStateChangeDeregistered),
				"RM_STATE_CHANGE_REGISTERED" => Ok(Self::RmStateChangeRegistered),
				"CM_STATE_CHANGE_IDLE" => Ok(Self::CmStateChangeIdle),
				"CM_STATE_CHANGE_CONNECTED" => Ok(Self::CmStateChangeConnected),
				"HANDOVER" => Ok(Self::Handover),
				"MOBILITY_REGISTRATION_UPDATE" => Ok(Self::MobilityRegistrationUpdate),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AccessStateTransitionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AccessStateTransitionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AccessStateTransitionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - LOW: Low accuracy.
	/// - HIGH: High accuracy.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- LOW: Low accuracy.\n- HIGH:
	/// High accuracy.\n",
	///  "type": "string",
	///  "enum": [
	///    "LOW",
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
	pub enum Accuracy {
		#[default]
		#[serde(rename = "LOW")]
		Low,
		#[serde(rename = "HIGH")]
		High,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&Accuracy> for Accuracy {
		fn from(value: &Accuracy) -> Self {
			value.clone()
		}
	}

	impl ToString for Accuracy {
		fn to_string(&self) -> String {
			match *self {
				Self::Low => "LOW".to_string(),
				Self::High => "HIGH".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for Accuracy {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOW" => Ok(Self::Low),
				"HIGH" => Ok(Self::High),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for Accuracy {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Accuracy {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Accuracy {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates fulfilment of requested accuracy.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates fulfilment of requested accuracy.",
	///  "type": "string",
	///  "enum": [
	///    "REQUESTED_ACCURACY_FULFILLED",
	///    "REQUESTED_ACCURACY_NOT_FULFILLED"
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
	pub enum AccuracyFulfilmentIndicator {
		#[default]
		#[serde(rename = "REQUESTED_ACCURACY_FULFILLED")]
		RequestedAccuracyFulfilled,
		#[serde(rename = "REQUESTED_ACCURACY_NOT_FULFILLED")]
		RequestedAccuracyNotFulfilled,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AccuracyFulfilmentIndicator> for AccuracyFulfilmentIndicator {
		fn from(value: &AccuracyFulfilmentIndicator) -> Self {
			value.clone()
		}
	}

	impl ToString for AccuracyFulfilmentIndicator {
		fn to_string(&self) -> String {
			match *self {
				Self::RequestedAccuracyFulfilled => "REQUESTED_ACCURACY_FULFILLED".to_string(),
				Self::RequestedAccuracyNotFulfilled => {
					"REQUESTED_ACCURACY_NOT_FULFILLED".to_string()
				}
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AccuracyFulfilmentIndicator {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REQUESTED_ACCURACY_FULFILLED" => Ok(Self::RequestedAccuracyFulfilled),
				"REQUESTED_ACCURACY_NOT_FULFILLED" => Ok(Self::RequestedAccuracyNotFulfilled),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AccuracyFulfilmentIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AccuracyFulfilmentIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AccuracyFulfilmentIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Additional information to be returned in EnableUeReachability error
	/// response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Additional information to be returned in
	/// EnableUeReachability error response",
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
	pub struct AdditionInfoEnableUeReachability {
		#[serde(
			rename = "maxWaitingTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_waiting_time: Option<DurationSec>,
	}

	impl From<&AdditionInfoEnableUeReachability> for AdditionInfoEnableUeReachability {
		fn from(value: &AdditionInfoEnableUeReachability) -> Self {
			value.clone()
		}
	}

	/// Represents additional measurement information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents additional measurement information.",
	///  "type": "object",
	///  "properties": {
	///    "circums": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CircumstanceDescription"
	///      },
	///      "minItems": 1
	///    },
	///    "ddosAttack": {
	///      "$ref": "#/components/schemas/AddressList"
	///    },
	///    "unexpFlowTeps": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpEthFlowDescription"
	///      },
	///      "minItems": 1
	///    },
	///    "unexpLoc": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "unexpWakes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DateTime"
	///      },
	///      "minItems": 1
	///    },
	///    "wrgDest": {
	///      "$ref": "#/components/schemas/AddressList"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AdditionalMeasurement {
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub circums: Vec<CircumstanceDescription>,
		#[serde(
			rename = "ddosAttack",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ddos_attack: Option<AddressList>,
		#[serde(
			rename = "unexpFlowTeps",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub unexp_flow_teps: Vec<IpEthFlowDescription>,
		#[serde(rename = "unexpLoc", default, skip_serializing_if = "Option::is_none")]
		pub unexp_loc: Option<NetworkAreaInfo>,
		#[serde(rename = "unexpWakes", default, skip_serializing_if = "Vec::is_empty")]
		pub unexp_wakes: Vec<DateTime>,
		#[serde(rename = "wrgDest", default, skip_serializing_if = "Option::is_none")]
		pub wrg_dest: Option<AddressList>,
	}

	impl From<&AdditionalMeasurement> for AdditionalMeasurement {
		fn from(value: &AdditionalMeasurement) -> Self {
			value.clone()
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

	/// Represents a list of IPv4 and/or IPv6 addresses.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a list of IPv4 and/or IPv6 addresses.",
	///  "type": "object",
	///  "properties": {
	///    "ipv4Addrs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ipv4Addr"
	///      },
	///      "minItems": 1
	///    },
	///    "ipv6Addrs": {
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
	pub struct AddressList {
		#[serde(rename = "ipv4Addrs", default, skip_serializing_if = "Vec::is_empty")]
		pub ipv4_addrs: Vec<Ipv4Addr>,
		#[serde(rename = "ipv6Addrs", default, skip_serializing_if = "Vec::is_empty")]
		pub ipv6_addrs: Vec<Ipv6Addr>,
	}

	impl From<&AddressList> for AddressList {
		fn from(value: &AddressList) -> Self {
			value.clone()
		}
	}

	/// Indicates value of the age of the location estimate.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates value of the age of the location estimate.",
	///  "type": "integer",
	///  "maximum": 32767.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AgeOfLocationEstimate(pub i64);

	impl ::std::ops::Deref for AgeOfLocationEstimate {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<AgeOfLocationEstimate> for i64 {
		fn from(value: AgeOfLocationEstimate) -> Self {
			value.0
		}
	}

	impl From<&AgeOfLocationEstimate> for AgeOfLocationEstimate {
		fn from(value: &AgeOfLocationEstimate) -> Self {
			value.clone()
		}
	}

	impl From<i64> for AgeOfLocationEstimate {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AgeOfLocationEstimate {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for AgeOfLocationEstimate {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AgeOfLocationEstimate {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AgeOfLocationEstimate {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for AgeOfLocationEstimate {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Alert Limit.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Alert Limit.",
	///  "type": "object",
	///  "required": [
	///    "horizontalProtectionLevel"
	///  ],
	///  "properties": {
	///    "horizontalProtectionLevel": {
	///      "$ref": "#/components/schemas/HorizontalProtectionLevel"
	///    },
	///    "verticalProtectionLevel": {
	///      "$ref": "#/components/schemas/VerticalProtectionLevel"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AlertLimit {
		#[serde(rename = "horizontalProtectionLevel")]
		pub horizontal_protection_level: HorizontalProtectionLevel,
		#[serde(
			rename = "verticalProtectionLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vertical_protection_level: Option<VerticalProtectionLevel>,
	}

	impl From<&AlertLimit> for AlertLimit {
		fn from(value: &AlertLimit) -> Self {
			value.clone()
		}
	}

	/// Contains an array of allowed S-NSSAI that constitute the allowed NSSAI
	/// information for the authorized network slice information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains an array of allowed S-NSSAI that constitute
	/// the allowed NSSAI information for the authorized network slice
	/// information\n",
	///  "type": "object",
	///  "required": [
	///    "accessType",
	///    "allowedSnssaiList"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "allowedSnssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AllowedSnssai"
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
	pub struct AllowedNssai {
		#[serde(rename = "accessType")]
		pub access_type: AccessType,
		#[serde(rename = "allowedSnssaiList")]
		pub allowed_snssai_list: Vec<AllowedSnssai>,
	}

	impl From<&AllowedNssai> for AllowedNssai {
		fn from(value: &AllowedNssai) -> Self {
			value.clone()
		}
	}

	/// Contains the authorized S-NSSAI and optional mapped home S-NSSAI and
	/// network slice instance information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the authorized S-NSSAI and optional mapped
	/// home S-NSSAI and network slice instance information\n",
	///  "type": "object",
	///  "required": [
	///    "allowedSnssai"
	///  ],
	///  "properties": {
	///    "allowedSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "mappedHomeSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "nsiInformationList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsiInformation"
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
	pub struct AllowedSnssai {
		#[serde(rename = "allowedSnssai")]
		pub allowed_snssai: Snssai,
		#[serde(
			rename = "mappedHomeSnssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mapped_home_snssai: Option<Snssai>,
		#[serde(
			rename = "nsiInformationList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nsi_information_list: Vec<NsiInformation>,
	}

	impl From<&AllowedSnssai> for AllowedSnssai {
		fn from(value: &AllowedSnssai) -> Self {
			value.clone()
		}
	}

	/// Data within a create AMF event subscription request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a create AMF event subscription request",
	///  "type": "object",
	///  "required": [
	///    "subscription"
	///  ],
	///  "properties": {
	///    "oldGuami": {
	///      "$ref": "#/components/schemas/crate::common::common_models::Guami"
	///    },
	///    "subscription": {
	///      "$ref": "#/components/schemas/AmfEventSubscription"
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
	pub struct AmfCreateEventSubscription {
		#[serde(rename = "oldGuami", default, skip_serializing_if = "Option::is_none")]
		pub old_guami: Option<crate::common::common_models::Guami>,
		pub subscription: AmfEventSubscription,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&AmfCreateEventSubscription> for AmfCreateEventSubscription {
		fn from(value: &AmfCreateEventSubscription) -> Self {
			value.clone()
		}
	}

	/// Data within a create AMF event subscription response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a create AMF event subscription response",
	///  "type": "object",
	///  "required": [
	///    "subscription",
	///    "subscriptionId"
	///  ],
	///  "properties": {
	///    "reportList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmfEventReport"
	///      },
	///      "minItems": 1
	///    },
	///    "subscription": {
	///      "$ref": "#/components/schemas/AmfEventSubscription"
	///    },
	///    "subscriptionId": {
	///      "$ref": "#/components/schemas/Uri"
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
	pub struct AmfCreatedEventSubscription {
		#[serde(rename = "reportList", default, skip_serializing_if = "Vec::is_empty")]
		pub report_list: Vec<AmfEventReport>,
		pub subscription: AmfEventSubscription,
		#[serde(rename = "subscriptionId")]
		pub subscription_id: Uri,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&AmfCreatedEventSubscription> for AmfCreatedEventSubscription {
		fn from(value: &AmfCreatedEventSubscription) -> Self {
			value.clone()
		}
	}

	/// Describes an event to be subscribed
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes an event to be subscribed",
	///  "type": "object",
	///  "required": [
	///    "type"
	///  ],
	///  "properties": {
	///    "areaList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmfEventArea"
	///      },
	///      "minItems": 1
	///    },
	///    "dispersionArea": {
	///      "$ref": "#/components/schemas/DispersionArea"
	///    },
	///    "idleStatusInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "immediateFlag": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "locationFilterList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/LocationFilter"
	///      },
	///      "minItems": 1
	///    },
	///    "maxReports": {
	///      "type": "integer"
	///    },
	///    "maxResponseTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "minInterval": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "nextPeriodicReportTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "nextReport": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "presenceInfoList": {
	///      "description": "A map(list of key-value pairs) where praId serves
	/// as key.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "reachabilityFilter": {
	///      "$ref": "#/components/schemas/ReachabilityFilter"
	///    },
	///    "refId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "reportUeReachable": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "snssaiFilter": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "targetArea": {
	///      "$ref": "#/components/schemas/TargetArea"
	///    },
	///    "trafficDescriptorList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TrafficDescriptor"
	///      },
	///      "minItems": 1
	///    },
	///    "type": {
	///      "$ref": "#/components/schemas/AmfEventType"
	///    },
	///    "udmDetectInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ueInAreaFilter": {
	///      "$ref": "#/components/schemas/UeInAreaFilter"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfEvent {
		#[serde(rename = "areaList", default, skip_serializing_if = "Vec::is_empty")]
		pub area_list: Vec<AmfEventArea>,
		#[serde(
			rename = "dispersionArea",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dispersion_area: Option<DispersionArea>,
		#[serde(rename = "idleStatusInd", default)]
		pub idle_status_ind: bool,
		#[serde(rename = "immediateFlag", default)]
		pub immediate_flag: bool,
		#[serde(
			rename = "locationFilterList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub location_filter_list: Vec<LocationFilter>,
		#[serde(
			rename = "maxReports",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_reports: Option<i64>,
		#[serde(
			rename = "maxResponseTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_response_time: Option<DurationSec>,
		#[serde(
			rename = "minInterval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub min_interval: Option<DurationSec>,
		#[serde(
			rename = "nextPeriodicReportTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_periodic_report_time: Option<DateTime>,
		#[serde(
			rename = "nextReport",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_report: Option<DateTime>,
		/// A map(list of key-value pairs) where praId serves as key.
		#[serde(
			rename = "presenceInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub presence_info_list: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(
			rename = "reachabilityFilter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reachability_filter: Option<ReachabilityFilter>,
		#[serde(rename = "refId", default, skip_serializing_if = "Option::is_none")]
		pub ref_id: Option<ReferenceId>,
		#[serde(rename = "reportUeReachable", default)]
		pub report_ue_reachable: bool,
		#[serde(
			rename = "snssaiFilter",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub snssai_filter: Vec<ExtSnssai>,
		#[serde(
			rename = "targetArea",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_area: Option<TargetArea>,
		#[serde(
			rename = "trafficDescriptorList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub traffic_descriptor_list: Vec<TrafficDescriptor>,
		#[serde(rename = "type")]
		pub type_: AmfEventType,
		#[serde(rename = "udmDetectInd", default)]
		pub udm_detect_ind: bool,
		#[serde(
			rename = "ueInAreaFilter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_in_area_filter: Option<UeInAreaFilter>,
	}

	impl From<&AmfEvent> for AmfEvent {
		fn from(value: &AmfEvent) -> Self {
			value.clone()
		}
	}

	/// Represents an area to be monitored by an AMF event
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an area to be monitored by an AMF event",
	///  "type": "object",
	///  "properties": {
	///    "ladnInfo": {
	///      "$ref": "#/components/schemas/LadnInfo"
	///    },
	///    "nsiId": {
	///      "$ref": "#/components/schemas/NsiId"
	///    },
	///    "presenceInfo": {
	///      "$ref": "#/components/schemas/PresenceInfo"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfEventArea {
		#[serde(rename = "ladnInfo", default, skip_serializing_if = "Option::is_none")]
		pub ladn_info: Option<LadnInfo>,
		#[serde(rename = "nsiId", default, skip_serializing_if = "Option::is_none")]
		pub nsi_id: Option<NsiId>,
		#[serde(
			rename = "presenceInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub presence_info: Option<PresenceInfo>,
		#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
		pub s_nssai: Option<Snssai>,
	}

	impl From<&AmfEventArea> for AmfEventArea {
		fn from(value: &AmfEventArea) -> Self {
			value.clone()
		}
	}

	/// Describes how the reports shall be generated by a subscribed event
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes how the reports shall be generated by a
	/// subscribed event",
	///  "type": "object",
	///  "required": [
	///    "trigger"
	///  ],
	///  "properties": {
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "maxReports": {
	///      "type": "integer"
	///    },
	///    "notifFlag": {
	///      "$ref": "#/components/schemas/NotificationFlag"
	///    },
	///    "partitioningCriteria": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PartitioningCriteria"
	///      },
	///      "minItems": 1
	///    },
	///    "repPeriod": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "sampRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "trigger": {
	///      "$ref": "#/components/schemas/AmfEventTrigger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfEventMode {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DateTime>,
		#[serde(
			rename = "maxReports",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_reports: Option<i64>,
		#[serde(rename = "notifFlag", default, skip_serializing_if = "Option::is_none")]
		pub notif_flag: Option<NotificationFlag>,
		#[serde(
			rename = "partitioningCriteria",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub partitioning_criteria: Vec<PartitioningCriteria>,
		#[serde(rename = "repPeriod", default, skip_serializing_if = "Option::is_none")]
		pub rep_period: Option<DurationSec>,
		#[serde(rename = "sampRatio", default, skip_serializing_if = "Option::is_none")]
		pub samp_ratio: Option<SamplingRatio>,
		pub trigger: AmfEventTrigger,
	}

	impl From<&AmfEventMode> for AmfEventMode {
		fn from(value: &AmfEventMode) -> Self {
			value.clone()
		}
	}

	/// Data within a AMF Event Notification request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a AMF Event Notification request",
	///  "type": "object",
	///  "properties": {
	///    "eventSubsSyncInfo": {
	///      "$ref": "#/components/schemas/AmfEventSubsSyncInfo"
	///    },
	///    "notifyCorrelationId": {
	///      "type": "string"
	///    },
	///    "reportList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmfEventReport"
	///      },
	///      "minItems": 1
	///    },
	///    "subsChangeNotifyCorrelationId": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfEventNotification {
		#[serde(
			rename = "eventSubsSyncInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_subs_sync_info: Option<AmfEventSubsSyncInfo>,
		#[serde(
			rename = "notifyCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notify_correlation_id: Option<String>,
		#[serde(rename = "reportList", default, skip_serializing_if = "Vec::is_empty")]
		pub report_list: Vec<AmfEventReport>,
		#[serde(
			rename = "subsChangeNotifyCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_change_notify_correlation_id: Option<String>,
	}

	impl From<&AmfEventNotification> for AmfEventNotification {
		fn from(value: &AmfEventNotification) -> Self {
			value.clone()
		}
	}

	/// Represents a report triggered by a subscribed event type
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a report triggered by a subscribed event
	/// type",
	///  "type": "object",
	///  "required": [
	///    "state",
	///    "timeStamp",
	///    "type"
	///  ],
	///  "properties": {
	///    "5gsUserStateList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/5GsUserStateInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "accessTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "additionalLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "anyUe": {
	///      "type": "boolean"
	///    },
	///    "areaList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmfEventArea"
	///      },
	///      "minItems": 1
	///    },
	///    "cmInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CmInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "commFailure": {
	///      "$ref": "#/components/schemas/CommunicationFailure"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "idleStatusIndication": {
	///      "$ref": "#/components/schemas/IdleStatusIndication"
	///    },
	///    "location": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "lossOfConnectReason": {
	///      "$ref": "#/components/schemas/LossOfConnectivityReason"
	///    },
	///    "maxAvailabilityTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "mmTransLocationReportList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MmTransactionLocationReportItem"
	///      },
	///      "minItems": 1
	///    },
	///    "mmTransSliceReportList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MmTransactionSliceReportItem"
	///      },
	///      "minItems": 1
	///    },
	///    "numberOfUes": {
	///      "type": "integer"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "reachability": {
	///      "$ref": "#/components/schemas/UeReachability"
	///    },
	///    "refId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "registrationNumber": {
	///      "type": "integer"
	///    },
	///    "rmInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RmInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "snssaiTaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SnssaiTaiMapping"
	///      },
	///      "minItems": 1
	///    },
	///    "state": {
	///      "$ref": "#/components/schemas/AmfEventState"
	///    },
	///    "subscriptionId": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "timeStamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "timezone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "type": {
	///      "$ref": "#/components/schemas/AmfEventType"
	///    },
	///    "typeCode": {
	///      "type": "string",
	///      "pattern": "^imeitac-[0-9]{8}$"
	///    },
	///    "ueAccessBehaviorTrends": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeAccessBehaviorReportItem"
	///      },
	///      "minItems": 1
	///    },
	///    "ueIdExt": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UEIdExt"
	///      },
	///      "minItems": 1
	///    },
	///    "ueLocationTrends": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeLocationTrendsReportItem"
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
	pub struct AmfEventReport {
		#[serde(
			rename = "accessTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub access_type_list: Vec<AccessType>,
		#[serde(
			rename = "additionalLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_location: Option<UserLocation>,
		#[serde(rename = "anyUe", default, skip_serializing_if = "Option::is_none")]
		pub any_ue: Option<bool>,
		#[serde(rename = "areaList", default, skip_serializing_if = "Vec::is_empty")]
		pub area_list: Vec<AmfEventArea>,
		#[serde(rename = "cmInfoList", default, skip_serializing_if = "Vec::is_empty")]
		pub cm_info_list: Vec<CmInfo>,
		#[serde(
			rename = "commFailure",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub comm_failure: Option<CommunicationFailure>,
		#[serde(
			rename = "5gsUserStateList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub five_gs_user_state_list: Vec<_5gsUserStateInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(
			rename = "idleStatusIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub idle_status_indication: Option<IdleStatusIndication>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub location: Option<UserLocation>,
		#[serde(
			rename = "lossOfConnectReason",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub loss_of_connect_reason: Option<LossOfConnectivityReason>,
		#[serde(
			rename = "maxAvailabilityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_availability_time: Option<DateTime>,
		#[serde(
			rename = "mmTransLocationReportList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mm_trans_location_report_list: Vec<MmTransactionLocationReportItem>,
		#[serde(
			rename = "mmTransSliceReportList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mm_trans_slice_report_list: Vec<MmTransactionSliceReportItem>,
		#[serde(
			rename = "numberOfUes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub number_of_ues: Option<i64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub reachability: Option<UeReachability>,
		#[serde(rename = "refId", default, skip_serializing_if = "Option::is_none")]
		pub ref_id: Option<ReferenceId>,
		#[serde(
			rename = "registrationNumber",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub registration_number: Option<i64>,
		#[serde(rename = "rmInfoList", default, skip_serializing_if = "Vec::is_empty")]
		pub rm_info_list: Vec<RmInfo>,
		#[serde(
			rename = "snssaiTaiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub snssai_tai_list: Vec<SnssaiTaiMapping>,
		pub state: AmfEventState,
		#[serde(
			rename = "subscriptionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscription_id: Option<Uri>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(rename = "timeStamp")]
		pub time_stamp: DateTime,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub timezone: Option<TimeZone>,
		#[serde(rename = "type")]
		pub type_: AmfEventType,
		#[serde(rename = "typeCode", default, skip_serializing_if = "Option::is_none")]
		pub type_code: Option<AmfEventReportTypeCode>,
		#[serde(
			rename = "ueAccessBehaviorTrends",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ue_access_behavior_trends: Vec<UeAccessBehaviorReportItem>,
		#[serde(rename = "ueIdExt", default, skip_serializing_if = "Vec::is_empty")]
		pub ue_id_ext: Vec<UeIdExt>,
		#[serde(
			rename = "ueLocationTrends",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ue_location_trends: Vec<UeLocationTrendsReportItem>,
	}

	impl From<&AmfEventReport> for AmfEventReport {
		fn from(value: &AmfEventReport) -> Self {
			value.clone()
		}
	}

	/// AmfEventReportTypeCode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^imeitac-[0-9]{8}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub struct AmfEventReportTypeCode(String);

	impl ::std::ops::Deref for AmfEventReportTypeCode {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AmfEventReportTypeCode> for String {
		fn from(value: AmfEventReportTypeCode) -> Self {
			value.0
		}
	}

	impl From<&AmfEventReportTypeCode> for AmfEventReportTypeCode {
		fn from(value: &AmfEventReportTypeCode) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AmfEventReportTypeCode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^imeitac-[0-9]{8}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^imeitac-[0-9]{8}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for AmfEventReportTypeCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AmfEventReportTypeCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AmfEventReportTypeCode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AmfEventReportTypeCode {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// Represents the state of a subscribed event
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the state of a subscribed event",
	///  "type": "object",
	///  "required": [
	///    "active"
	///  ],
	///  "properties": {
	///    "active": {
	///      "type": "boolean"
	///    },
	///    "remainDuration": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "remainReports": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfEventState {
		pub active: bool,
		#[serde(
			rename = "remainDuration",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remain_duration: Option<DurationSec>,
		#[serde(
			rename = "remainReports",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub remain_reports: Option<i64>,
	}

	impl From<&AmfEventState> for AmfEventState {
		fn from(value: &AmfEventState) -> Self {
			value.clone()
		}
	}

	/// AMF Event Subscriptions Information for synchronization
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "AMF Event Subscriptions Information for
	/// synchronization",
	///  "type": "object",
	///  "required": [
	///    "subscriptionList"
	///  ],
	///  "properties": {
	///    "subscriptionList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmfEventSubscriptionInfo"
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
	pub struct AmfEventSubsSyncInfo {
		#[serde(rename = "subscriptionList")]
		pub subscription_list: Vec<AmfEventSubscriptionInfo>,
	}

	impl From<&AmfEventSubsSyncInfo> for AmfEventSubsSyncInfo {
		fn from(value: &AmfEventSubsSyncInfo) -> Self {
			value.clone()
		}
	}

	/// Represents an individual event subscription resource on AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an individual event subscription resource on
	/// AMF",
	///  "type": "object",
	///  "required": [
	///    "eventList",
	///    "eventNotifyUri",
	///    "nfId",
	///    "notifyCorrelationId"
	///  ],
	///  "properties": {
	///    "anyUE": {
	///      "type": "boolean"
	///    },
	///    "eventList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmfEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "eventNotifyUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "excludeGpsiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "excludeSupiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "groupId": {
	///      "$ref": "#/components/schemas/GroupId"
	///    },
	///    "includeGpsiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "includeSupiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "nfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "notifyCorrelationId": {
	///      "type": "string"
	///    },
	///    "options": {
	///      "$ref": "#/components/schemas/AmfEventMode"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "sourceNfType": {
	///      "$ref": "#/components/schemas/NFType"
	///    },
	///    "subsChangeNotifyCorrelationId": {
	///      "type": "string"
	///    },
	///    "subsChangeNotifyUri": {
	///      "$ref": "#/components/schemas/Uri"
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
	pub struct AmfEventSubscription {
		#[serde(rename = "anyUE", default, skip_serializing_if = "Option::is_none")]
		pub any_ue: Option<bool>,
		#[serde(rename = "eventList")]
		pub event_list: Vec<AmfEvent>,
		#[serde(rename = "eventNotifyUri")]
		pub event_notify_uri: Uri,
		#[serde(
			rename = "excludeGpsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub exclude_gpsi_list: Vec<Gpsi>,
		#[serde(
			rename = "excludeSupiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub exclude_supi_list: Vec<Supi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<GroupId>,
		#[serde(
			rename = "includeGpsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub include_gpsi_list: Vec<Gpsi>,
		#[serde(
			rename = "includeSupiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub include_supi_list: Vec<Supi>,
		#[serde(rename = "nfId")]
		pub nf_id: NfInstanceId,
		#[serde(rename = "notifyCorrelationId")]
		pub notify_correlation_id: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub options: Option<AmfEventMode>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(
			rename = "sourceNfType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_nf_type: Option<NfType>,
		#[serde(
			rename = "subsChangeNotifyCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_change_notify_correlation_id: Option<String>,
		#[serde(
			rename = "subsChangeNotifyUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_change_notify_uri: Option<Uri>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
	}

	impl From<&AmfEventSubscription> for AmfEventSubscription {
		fn from(value: &AmfEventSubscription) -> Self {
			value.clone()
		}
	}

	/// Additional information received for an AMF event subscription, e.g.
	/// binding indications
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Additional information received for an AMF event
	/// subscription, e.g. binding indications",
	///  "type": "object",
	///  "properties": {
	///    "aoiStateList": {
	///      "description": "Map of subscribed Area of Interest (AoI) Event
	/// State in the old AMF. The JSON pointer to an AmfEventArea element in the
	/// areaList IE (or a PresenceInfo element in  presenceInfoList IE) of the
	/// AmfEvent data type shall be the key of the map.\n",
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/AreaOfInterestEventState"
	///      }
	///    },
	///    "bindingInfo": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "eventSyncInd": {
	///      "type": "boolean"
	///    },
	///    "nfConsumerInfo": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "subscribingNfType": {
	///      "$ref": "#/components/schemas/NFType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfEventSubscriptionAddInfo {
		/// Map of subscribed Area of Interest (AoI) Event State in the old AMF.
		/// The JSON pointer to an AmfEventArea element in the areaList IE (or a
		/// PresenceInfo element in  presenceInfoList IE) of the AmfEvent data
		/// type shall be the key of the map.
		#[serde(
			rename = "aoiStateList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub aoi_state_list: ::std::collections::HashMap<String, AreaOfInterestEventState>,
		#[serde(rename = "bindingInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub binding_info: Vec<String>,
		#[serde(
			rename = "eventSyncInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_sync_ind: Option<bool>,
		#[serde(
			rename = "nfConsumerInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nf_consumer_info: Vec<String>,
		#[serde(
			rename = "subscribingNfType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscribing_nf_type: Option<NfType>,
	}

	impl From<&AmfEventSubscriptionAddInfo> for AmfEventSubscriptionAddInfo {
		fn from(value: &AmfEventSubscriptionAddInfo) -> Self {
			value.clone()
		}
	}

	/// Individual AMF Event Subscription Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Individual AMF Event Subscription Information",
	///  "type": "object",
	///  "required": [
	///    "refIdList",
	///    "subId"
	///  ],
	///  "properties": {
	///    "notifyCorrelationId": {
	///      "type": "string"
	///    },
	///    "oldSubId": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "refIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ReferenceId"
	///      },
	///      "minItems": 1
	///    },
	///    "subId": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfEventSubscriptionInfo {
		#[serde(
			rename = "notifyCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notify_correlation_id: Option<String>,
		#[serde(rename = "oldSubId", default, skip_serializing_if = "Option::is_none")]
		pub old_sub_id: Option<Uri>,
		#[serde(rename = "refIdList")]
		pub ref_id_list: Vec<ReferenceId>,
		#[serde(rename = "subId")]
		pub sub_id: Uri,
	}

	impl From<&AmfEventSubscriptionInfo> for AmfEventSubscriptionInfo {
		fn from(value: &AmfEventSubscriptionInfo) -> Self {
			value.clone()
		}
	}

	/// Describes how AMF should generate the report for the event
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes how AMF should generate the report for the
	/// event",
	///  "type": "string",
	///  "enum": [
	///    "ONE_TIME",
	///    "CONTINUOUS",
	///    "PERIODIC"
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
	pub enum AmfEventTrigger {
		#[default]
		#[serde(rename = "ONE_TIME")]
		OneTime,
		#[serde(rename = "CONTINUOUS")]
		Continuous,
		#[serde(rename = "PERIODIC")]
		Periodic,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AmfEventTrigger> for AmfEventTrigger {
		fn from(value: &AmfEventTrigger) -> Self {
			value.clone()
		}
	}

	impl ToString for AmfEventTrigger {
		fn to_string(&self) -> String {
			match *self {
				Self::OneTime => "ONE_TIME".to_string(),
				Self::Continuous => "CONTINUOUS".to_string(),
				Self::Periodic => "PERIODIC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AmfEventTrigger {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ONE_TIME" => Ok(Self::OneTime),
				"CONTINUOUS" => Ok(Self::Continuous),
				"PERIODIC" => Ok(Self::Periodic),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AmfEventTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AmfEventTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AmfEventTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Describes the supported event types of Namf_EventExposure Service
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the supported event types of
	/// Namf_EventExposure Service",
	///  "type": "string",
	///  "enum": [
	///    "LOCATION_REPORT",
	///    "PRESENCE_IN_AOI_REPORT",
	///    "TIMEZONE_REPORT",
	///    "ACCESS_TYPE_REPORT",
	///    "REGISTRATION_STATE_REPORT",
	///    "CONNECTIVITY_STATE_REPORT",
	///    "REACHABILITY_REPORT",
	///    "COMMUNICATION_FAILURE_REPORT",
	///    "UES_IN_AREA_REPORT",
	///    "SUBSCRIPTION_ID_CHANGE",
	///    "SUBSCRIPTION_ID_ADDITION",
	///    "LOSS_OF_CONNECTIVITY",
	///    "5GS_USER_STATE_REPORT",
	///    "AVAILABILITY_AFTER_DDN_FAILURE",
	///    "TYPE_ALLOCATION_CODE_REPORT",
	///    "FREQUENT_MOBILITY_REGISTRATION_REPORT",
	///    "SNSSAI_TA_MAPPING_REPORT",
	///    "UE_LOCATION_TRENDS",
	///    "UE_ACCESS_BEHAVIOR_TRENDS",
	///    "UE_MM_TRANSACTION_REPORT"
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
	pub enum AmfEventType {
		#[default]
		#[serde(rename = "LOCATION_REPORT")]
		LocationReport,
		#[serde(rename = "PRESENCE_IN_AOI_REPORT")]
		PresenceInAoiReport,
		#[serde(rename = "TIMEZONE_REPORT")]
		TimezoneReport,
		#[serde(rename = "ACCESS_TYPE_REPORT")]
		AccessTypeReport,
		#[serde(rename = "REGISTRATION_STATE_REPORT")]
		RegistrationStateReport,
		#[serde(rename = "CONNECTIVITY_STATE_REPORT")]
		ConnectivityStateReport,
		#[serde(rename = "REACHABILITY_REPORT")]
		ReachabilityReport,
		#[serde(rename = "COMMUNICATION_FAILURE_REPORT")]
		CommunicationFailureReport,
		#[serde(rename = "UES_IN_AREA_REPORT")]
		UesInAreaReport,
		#[serde(rename = "SUBSCRIPTION_ID_CHANGE")]
		SubscriptionIdChange,
		#[serde(rename = "SUBSCRIPTION_ID_ADDITION")]
		SubscriptionIdAddition,
		#[serde(rename = "LOSS_OF_CONNECTIVITY")]
		LossOfConnectivity,
		#[serde(rename = "5GS_USER_STATE_REPORT")]
		FiveGsUserStateReport,
		#[serde(rename = "AVAILABILITY_AFTER_DDN_FAILURE")]
		AvailabilityAfterDdnFailure,
		#[serde(rename = "TYPE_ALLOCATION_CODE_REPORT")]
		TypeAllocationCodeReport,
		#[serde(rename = "FREQUENT_MOBILITY_REGISTRATION_REPORT")]
		FrequentMobilityRegistrationReport,
		#[serde(rename = "SNSSAI_TA_MAPPING_REPORT")]
		SnssaiTaMappingReport,
		#[serde(rename = "UE_LOCATION_TRENDS")]
		UeLocationTrends,
		#[serde(rename = "UE_ACCESS_BEHAVIOR_TRENDS")]
		UeAccessBehaviorTrends,
		#[serde(rename = "UE_MM_TRANSACTION_REPORT")]
		UeMmTransactionReport,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AmfEventType> for AmfEventType {
		fn from(value: &AmfEventType) -> Self {
			value.clone()
		}
	}

	impl ToString for AmfEventType {
		fn to_string(&self) -> String {
			match *self {
				Self::LocationReport => "LOCATION_REPORT".to_string(),
				Self::PresenceInAoiReport => "PRESENCE_IN_AOI_REPORT".to_string(),
				Self::TimezoneReport => "TIMEZONE_REPORT".to_string(),
				Self::AccessTypeReport => "ACCESS_TYPE_REPORT".to_string(),
				Self::RegistrationStateReport => "REGISTRATION_STATE_REPORT".to_string(),
				Self::ConnectivityStateReport => "CONNECTIVITY_STATE_REPORT".to_string(),
				Self::ReachabilityReport => "REACHABILITY_REPORT".to_string(),
				Self::CommunicationFailureReport => "COMMUNICATION_FAILURE_REPORT".to_string(),
				Self::UesInAreaReport => "UES_IN_AREA_REPORT".to_string(),
				Self::SubscriptionIdChange => "SUBSCRIPTION_ID_CHANGE".to_string(),
				Self::SubscriptionIdAddition => "SUBSCRIPTION_ID_ADDITION".to_string(),
				Self::LossOfConnectivity => "LOSS_OF_CONNECTIVITY".to_string(),
				Self::FiveGsUserStateReport => "5GS_USER_STATE_REPORT".to_string(),
				Self::AvailabilityAfterDdnFailure => "AVAILABILITY_AFTER_DDN_FAILURE".to_string(),
				Self::TypeAllocationCodeReport => "TYPE_ALLOCATION_CODE_REPORT".to_string(),
				Self::FrequentMobilityRegistrationReport => {
					"FREQUENT_MOBILITY_REGISTRATION_REPORT".to_string()
				}
				Self::SnssaiTaMappingReport => "SNSSAI_TA_MAPPING_REPORT".to_string(),
				Self::UeLocationTrends => "UE_LOCATION_TRENDS".to_string(),
				Self::UeAccessBehaviorTrends => "UE_ACCESS_BEHAVIOR_TRENDS".to_string(),
				Self::UeMmTransactionReport => "UE_MM_TRANSACTION_REPORT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AmfEventType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOCATION_REPORT" => Ok(Self::LocationReport),
				"PRESENCE_IN_AOI_REPORT" => Ok(Self::PresenceInAoiReport),
				"TIMEZONE_REPORT" => Ok(Self::TimezoneReport),
				"ACCESS_TYPE_REPORT" => Ok(Self::AccessTypeReport),
				"REGISTRATION_STATE_REPORT" => Ok(Self::RegistrationStateReport),
				"CONNECTIVITY_STATE_REPORT" => Ok(Self::ConnectivityStateReport),
				"REACHABILITY_REPORT" => Ok(Self::ReachabilityReport),
				"COMMUNICATION_FAILURE_REPORT" => Ok(Self::CommunicationFailureReport),
				"UES_IN_AREA_REPORT" => Ok(Self::UesInAreaReport),
				"SUBSCRIPTION_ID_CHANGE" => Ok(Self::SubscriptionIdChange),
				"SUBSCRIPTION_ID_ADDITION" => Ok(Self::SubscriptionIdAddition),
				"LOSS_OF_CONNECTIVITY" => Ok(Self::LossOfConnectivity),
				"_5GS_USER_STATE_REPORT" => Ok(Self::FiveGsUserStateReport),
				"AVAILABILITY_AFTER_DDN_FAILURE" => Ok(Self::AvailabilityAfterDdnFailure),
				"TYPE_ALLOCATION_CODE_REPORT" => Ok(Self::TypeAllocationCodeReport),
				"FREQUENT_MOBILITY_REGISTRATION_REPORT" => {
					Ok(Self::FrequentMobilityRegistrationReport)
				}
				"SNSSAI_TA_MAPPING_REPORT" => Ok(Self::SnssaiTaMappingReport),
				"UE_LOCATION_TRENDS" => Ok(Self::UeLocationTrends),
				"UE_ACCESS_BEHAVIOR_TRENDS" => Ok(Self::UeAccessBehaviorTrends),
				"UE_MM_TRANSACTION_REPORT" => Ok(Self::UeMmTransactionReport),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AmfEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AmfEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AmfEventType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within an AMF Status Change Notification request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within an AMF Status Change Notification request",
	///  "type": "object",
	///  "required": [
	///    "amfStatusInfoList"
	///  ],
	///  "properties": {
	///    "amfStatusInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmfStatusInfo"
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
	pub struct AmfStatusChangeNotification {
		#[serde(rename = "amfStatusInfoList")]
		pub amf_status_info_list: Vec<AmfStatusInfo>,
	}

	impl From<&AmfStatusChangeNotification> for AmfStatusChangeNotification {
		fn from(value: &AmfStatusChangeNotification) -> Self {
			value.clone()
		}
	}

	/// AMF Status Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "AMF Status Information",
	///  "type": "object",
	///  "required": [
	///    "guamiList",
	///    "statusChange"
	///  ],
	///  "properties": {
	///    "guamiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Guami"
	///      },
	///      "minItems": 1
	///    },
	///    "statusChange": {
	///      "$ref": "#/components/schemas/StatusChange"
	///    },
	///    "targetAmfFailure": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "targetAmfRemoval": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfStatusInfo {
		#[serde(rename = "guamiList")]
		pub guami_list: Vec<crate::common::common_models::Guami>,
		#[serde(rename = "statusChange")]
		pub status_change: StatusChange,
		#[serde(
			rename = "targetAmfFailure",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_amf_failure: Option<Fqdn>,
		#[serde(
			rename = "targetAmfRemoval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_amf_removal: Option<Fqdn>,
	}

	impl From<&AmfStatusInfo> for AmfStatusInfo {
		fn from(value: &AmfStatusInfo) -> Self {
			value.clone()
		}
	}

	/// Document describing the modifications to AMF event subscription options
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Document describing the modifications to AMF event
	/// subscription options",
	///  "type": "object",
	///  "required": [
	///    "op",
	///    "path",
	///    "value"
	///  ],
	///  "properties": {
	///    "notifFlag": {
	///      "$ref": "#/components/schemas/NotificationFlag"
	///    },
	///    "op": {
	///      "type": "string",
	///      "enum": [
	///        "replace"
	///      ]
	///    },
	///    "path": {
	///      "type": "string",
	///      "pattern": "^(\\/options\\/expiry|\\/options\\/notifFlag)$"
	///    },
	///    "value": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfUpdateEventOptionItem {
		#[serde(rename = "notifFlag", default, skip_serializing_if = "Option::is_none")]
		pub notif_flag: Option<NotificationFlag>,
		pub op: AmfUpdateEventOptionItemOp,
		pub path: AmfUpdateEventOptionItemPath,
		pub value: DateTime,
	}

	impl From<&AmfUpdateEventOptionItem> for AmfUpdateEventOptionItem {
		fn from(value: &AmfUpdateEventOptionItem) -> Self {
			value.clone()
		}
	}

	/// AmfUpdateEventOptionItemOp
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "replace"
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
	pub enum AmfUpdateEventOptionItemOp {
		#[default]
		#[serde(rename = "replace")]
		Replace,
	}

	impl From<&AmfUpdateEventOptionItemOp> for AmfUpdateEventOptionItemOp {
		fn from(value: &AmfUpdateEventOptionItemOp) -> Self {
			value.clone()
		}
	}

	impl ToString for AmfUpdateEventOptionItemOp {
		fn to_string(&self) -> String {
			match *self {
				Self::Replace => "replace".to_string(),
			}
		}
	}

	impl std::str::FromStr for AmfUpdateEventOptionItemOp {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"replace" => Ok(Self::Replace),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AmfUpdateEventOptionItemOp {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AmfUpdateEventOptionItemOp {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AmfUpdateEventOptionItemOp {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// AmfUpdateEventOptionItemPath
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^(\\/options\\/expiry|\\/options\\/notifFlag)$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub struct AmfUpdateEventOptionItemPath(String);

	impl ::std::ops::Deref for AmfUpdateEventOptionItemPath {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AmfUpdateEventOptionItemPath> for String {
		fn from(value: AmfUpdateEventOptionItemPath) -> Self {
			value.0
		}
	}

	impl From<&AmfUpdateEventOptionItemPath> for AmfUpdateEventOptionItemPath {
		fn from(value: &AmfUpdateEventOptionItemPath) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AmfUpdateEventOptionItemPath {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^(\\/options\\/expiry|\\/options\\/notifFlag)$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \
				            \"^(\\/options\\/expiry|\\/options\\/notifFlag)$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for AmfUpdateEventOptionItemPath {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AmfUpdateEventOptionItemPath {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AmfUpdateEventOptionItemPath {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AmfUpdateEventOptionItemPath {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// Document describing the modification(s) to an AMF Event Subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Document describing the modification(s) to an AMF Event
	/// Subscription",
	///  "type": "object",
	///  "required": [
	///    "op",
	///    "path"
	///  ],
	///  "properties": {
	///    "excludeGpsiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "excludeSupiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "includeGpsiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "includeSupiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "op": {
	///      "type": "string",
	///      "enum": [
	///        "add",
	///        "remove",
	///        "replace"
	///      ]
	///    },
	///    "path": {
	///      "type": "string",
	///      "pattern":
	/// "^\\/eventList\\/-|(\\/eventList\\/0|\\/eventList\\/[1-9][0-9]*){1}(\\/
	/// presenceInfoList\\/0|\\/presenceInfoList\\/[1-9][0-9]*)?|\\/
	/// excludeSupiList|\\/excludeGpsiList|\\/includeSupiList|\\/
	/// includeGpsiList$"
	///    },
	///    "presenceInfo": {
	///      "$ref": "#/components/schemas/PresenceInfo"
	///    },
	///    "value": {
	///      "$ref": "#/components/schemas/AmfEvent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfUpdateEventSubscriptionItem {
		#[serde(
			rename = "excludeGpsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub exclude_gpsi_list: Vec<Gpsi>,
		#[serde(
			rename = "excludeSupiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub exclude_supi_list: Vec<Supi>,
		#[serde(
			rename = "includeGpsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub include_gpsi_list: Vec<Gpsi>,
		#[serde(
			rename = "includeSupiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub include_supi_list: Vec<Supi>,
		pub op: AmfUpdateEventSubscriptionItemOp,
		pub path: AmfUpdateEventSubscriptionItemPath,
		#[serde(
			rename = "presenceInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub presence_info: Option<PresenceInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub value: Option<AmfEvent>,
	}

	impl From<&AmfUpdateEventSubscriptionItem> for AmfUpdateEventSubscriptionItem {
		fn from(value: &AmfUpdateEventSubscriptionItem) -> Self {
			value.clone()
		}
	}

	/// AmfUpdateEventSubscriptionItemOp
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "enum": [
	///    "add",
	///    "remove",
	///    "replace"
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
	pub enum AmfUpdateEventSubscriptionItemOp {
		#[default]
		#[serde(rename = "add")]
		Add,
		#[serde(rename = "remove")]
		Remove,
		#[serde(rename = "replace")]
		Replace,
	}

	impl From<&AmfUpdateEventSubscriptionItemOp> for AmfUpdateEventSubscriptionItemOp {
		fn from(value: &AmfUpdateEventSubscriptionItemOp) -> Self {
			value.clone()
		}
	}

	impl ToString for AmfUpdateEventSubscriptionItemOp {
		fn to_string(&self) -> String {
			match *self {
				Self::Add => "add".to_string(),
				Self::Remove => "remove".to_string(),
				Self::Replace => "replace".to_string(),
			}
		}
	}

	impl std::str::FromStr for AmfUpdateEventSubscriptionItemOp {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"add" => Ok(Self::Add),
				"remove" => Ok(Self::Remove),
				"replace" => Ok(Self::Replace),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AmfUpdateEventSubscriptionItemOp {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AmfUpdateEventSubscriptionItemOp {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AmfUpdateEventSubscriptionItemOp {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// AmfUpdateEventSubscriptionItemPath
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern":
	/// "^\\/eventList\\/-|(\\/eventList\\/0|\\/eventList\\/[1-9][0-9]*){1}(\\/
	/// presenceInfoList\\/0|\\/presenceInfoList\\/[1-9][0-9]*)?|\\/
	/// excludeSupiList|\\/excludeGpsiList|\\/includeSupiList|\\/
	/// includeGpsiList$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub struct AmfUpdateEventSubscriptionItemPath(String);

	impl ::std::ops::Deref for AmfUpdateEventSubscriptionItemPath {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<AmfUpdateEventSubscriptionItemPath> for String {
		fn from(value: AmfUpdateEventSubscriptionItemPath) -> Self {
			value.0
		}
	}

	impl From<&AmfUpdateEventSubscriptionItemPath> for AmfUpdateEventSubscriptionItemPath {
		fn from(value: &AmfUpdateEventSubscriptionItemPath) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for AmfUpdateEventSubscriptionItemPath {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new(
				"^\\/eventList\\/-|(\\/eventList\\/0|\\/eventList\\/[1-9][0-9]*){1}(\\/\
				 presenceInfoList\\/0|\\/presenceInfoList\\/[1-9][0-9]*)?|\\/excludeSupiList|\\/\
				 excludeGpsiList|\\/includeSupiList|\\/includeGpsiList$",
			)
			.unwrap()
			.find(value)
			.is_none()
			{
				return Err("doesn't match pattern \
				            \"^\\/eventList\\/-|(\\/eventList\\/0|\\/eventList\\/[1-9][0-9]*\
				            ){1}(\\/presenceInfoList\\/0|\\/presenceInfoList\\/[1-9][0-9]*)?|\\/\
				            excludeSupiList|\\/excludeGpsiList|\\/includeSupiList|\\/\
				            includeGpsiList$\""
					.into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for AmfUpdateEventSubscriptionItemPath {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for AmfUpdateEventSubscriptionItemPath {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for AmfUpdateEventSubscriptionItemPath {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for AmfUpdateEventSubscriptionItemPath {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// Represents a successful update on an AMF Event Subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a successful update on an AMF Event
	/// Subscription",
	///  "type": "object",
	///  "required": [
	///    "subscription"
	///  ],
	///  "properties": {
	///    "reportList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AmfEventReport"
	///      },
	///      "minItems": 1
	///    },
	///    "subscription": {
	///      "$ref": "#/components/schemas/AmfEventSubscription"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AmfUpdatedEventSubscription {
		#[serde(rename = "reportList", default, skip_serializing_if = "Vec::is_empty")]
		pub report_list: Vec<AmfEventReport>,
		pub subscription: AmfEventSubscription,
	}

	impl From<&AmfUpdatedEventSubscription> for AmfUpdatedEventSubscription {
		fn from(value: &AmfUpdatedEventSubscription) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - NUM_OF_SAMPLES: Number of data samples used for the generation of the
	///   output analytics.
	/// - DATA_WINDOW: Data time window of the data samples.
	/// - DATA_STAT_PROPS: Dataset statistical properties of the data used to
	///   generate the analytics.
	/// - STRATEGY: Output strategy used for the reporting of the analytics.
	/// - ACCURACY: Level of accuracy reached for the analytics.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- NUM_OF_SAMPLES: Number of data
	/// samples used for the generation of the output analytics.\n- DATA_WINDOW:
	/// Data time window of the data samples.\n- DATA_STAT_PROPS: Dataset
	/// statistical properties of the data used to generate the analytics.\n-
	/// STRATEGY: Output strategy used for the reporting of the analytics.\n-
	/// ACCURACY: Level of accuracy reached for the analytics.\n",
	///  "type": "string",
	///  "enum": [
	///    "NUM_OF_SAMPLES",
	///    "DATA_WINDOW",
	///    "DATA_STAT_PROPS",
	///    "STRATEGY",
	///    "ACCURACY"
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
	pub enum AnalyticsMetadata {
		#[default]
		#[serde(rename = "NUM_OF_SAMPLES")]
		NumOfSamples,
		#[serde(rename = "DATA_WINDOW")]
		DataWindow,
		#[serde(rename = "DATA_STAT_PROPS")]
		DataStatProps,
		#[serde(rename = "STRATEGY")]
		Strategy,
		#[serde(rename = "ACCURACY")]
		Accuracy,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AnalyticsMetadata> for AnalyticsMetadata {
		fn from(value: &AnalyticsMetadata) -> Self {
			value.clone()
		}
	}

	impl ToString for AnalyticsMetadata {
		fn to_string(&self) -> String {
			match *self {
				Self::NumOfSamples => "NUM_OF_SAMPLES".to_string(),
				Self::DataWindow => "DATA_WINDOW".to_string(),
				Self::DataStatProps => "DATA_STAT_PROPS".to_string(),
				Self::Strategy => "STRATEGY".to_string(),
				Self::Accuracy => "ACCURACY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AnalyticsMetadata {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NUM_OF_SAMPLES" => Ok(Self::NumOfSamples),
				"DATA_WINDOW" => Ok(Self::DataWindow),
				"DATA_STAT_PROPS" => Ok(Self::DataStatProps),
				"STRATEGY" => Ok(Self::Strategy),
				"ACCURACY" => Ok(Self::Accuracy),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AnalyticsMetadata {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AnalyticsMetadata {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AnalyticsMetadata {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains analytics metadata information requested to be used during
	/// analytics generation.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains analytics metadata information requested to be
	/// used during analytics generation.\n",
	///  "type": "object",
	///  "properties": {
	///    "aggrNwdafIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfInstanceId"
	///      },
	///      "minItems": 1
	///    },
	///    "dataStatProps": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DatasetStatisticalProperty"
	///      },
	///      "minItems": 1
	///    },
	///    "dataWindow": {
	///      "$ref": "#/components/schemas/TimeWindow"
	///    },
	///    "strategy": {
	///      "$ref": "#/components/schemas/OutputStrategy"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AnalyticsMetadataIndication {
		#[serde(
			rename = "aggrNwdafIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub aggr_nwdaf_ids: Vec<NfInstanceId>,
		#[serde(
			rename = "dataStatProps",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub data_stat_props: Vec<DatasetStatisticalProperty>,
		#[serde(
			rename = "dataWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub data_window: Option<TimeWindow>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub strategy: Option<OutputStrategy>,
	}

	impl From<&AnalyticsMetadataIndication> for AnalyticsMetadataIndication {
		fn from(value: &AnalyticsMetadataIndication) -> Self {
			value.clone()
		}
	}

	/// Contains analytics metadata information required for analytics
	/// aggregation.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains analytics metadata information required for
	/// analytics aggregation.",
	///  "type": "object",
	///  "properties": {
	///    "accuracy": {
	///      "$ref": "#/components/schemas/Accuracy"
	///    },
	///    "dataStatProps": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DatasetStatisticalProperty"
	///      },
	///      "minItems": 1
	///    },
	///    "dataWindow": {
	///      "$ref": "#/components/schemas/TimeWindow"
	///    },
	///    "numSamples": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "strategy": {
	///      "$ref": "#/components/schemas/OutputStrategy"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AnalyticsMetadataInfo {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub accuracy: Option<Accuracy>,
		#[serde(
			rename = "dataStatProps",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub data_stat_props: Vec<DatasetStatisticalProperty>,
		#[serde(
			rename = "dataWindow",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub data_window: Option<TimeWindow>,
		#[serde(
			rename = "numSamples",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub num_samples: Option<Uinteger>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub strategy: Option<OutputStrategy>,
	}

	impl From<&AnalyticsMetadataInfo> for AnalyticsMetadataInfo {
		fn from(value: &AnalyticsMetadataInfo) -> Self {
			value.clone()
		}
	}

	/// Analytics subscriptions created in the NWDAF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Analytics subscriptions created in the NWDAF.",
	///  "type": "object",
	///  "required": [
	///    "nwdafSubscriptionList"
	///  ],
	///  "properties": {
	///    "nwdafId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nwdafSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "nwdafSubscriptionList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafSubscription"
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
	pub struct AnalyticsSubscription {
		#[serde(rename = "nwdafId", default, skip_serializing_if = "Option::is_none")]
		pub nwdaf_id: Option<NfInstanceId>,
		#[serde(
			rename = "nwdafSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nwdaf_set_id: Option<NfSetId>,
		#[serde(rename = "nwdafSubscriptionList")]
		pub nwdaf_subscription_list: Vec<NwdafSubscription>,
	}

	impl From<&AnalyticsSubscription> for AnalyticsSubscription {
		fn from(value: &AnalyticsSubscription) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - NUM_OF_UE_REG: The number of UE registered. This value is only
	///   applicable to NSI_LOAD_LEVEL event.
	/// - NUM_OF_PDU_SESS_ESTBL: The number of PDU sessions established. This
	///   value is only applicable to NSI_LOAD_LEVEL event.
	/// - RES_USAGE: The current usage of the virtual resources assigned to the
	///   NF instances belonging to a particular network slice instance. This
	///   value is only applicable to NSI_LOAD_LEVEL event.
	/// - NUM_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR: The number of times the
	///   resource usage threshold of the network slice instance is reached or
	///   exceeded if a threshold value is provided by the consumer. This value
	///   is only applicable to NSI_LOAD_LEVEL event.
	/// - PERIOD_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR: The time interval between
	///   each time the threshold being met or exceeded on the network slice
	///   (instance). This value is only applicable to NSI_LOAD_LEVEL event.
	/// - EXCEED_LOAD_LEVEL_THR_IND: Whether the Load Level Threshold is met or
	///   exceeded by the statistics value. This value is only applicable to
	///   NSI_LOAD_LEVEL event.
	/// - LIST_OF_TOP_APP_UL: The list of applications that contribute the most
	///   to the traffic in the UL direction. This value is only applicable to
	///   USER_DATA_CONGESTION event.
	/// - LIST_OF_TOP_APP_DL: The list of applications that contribute the most
	///   to the traffic in the DL direction. This value is only applicable to
	///   USER_DATA_CONGESTION event.
	/// - NF_STATUS: The availability status of the NF on the Analytics target
	///   period, expressed as a percentage of time per status value
	///   (registered, suspended, undiscoverable). This value is only applicable
	///   to NF_LOAD event.
	/// - NF_RESOURCE_USAGE: The average usage of assigned resources (CPU,
	///   memory, storage). This value is only applicable to NF_LOAD event.
	/// - NF_LOAD: The average load of the NF instance over the Analytics target
	///   period. This value is only applicable to NF_LOAD event.
	/// - NF_PEAK_LOAD: The maximum load of the NF instance over the Analytics
	///   target period. This value is only applicable to NF_LOAD event.
	/// - NF_LOAD_AVG_IN_AOI: The average load of the NF instances over the area
	///   of interest. This value is only applicable to NF_LOAD event.
	/// - DISPER_AMOUNT: Indicates the dispersion amount of the reported data
	///   volume or transaction dispersion type. This value is only applicable
	///   to DISPERSION event.
	/// - DISPER_CLASS: Indicates the dispersion mobility class: fixed, camper,
	///   traveller upon set its usage threshold, and/or the top-heavy class
	///   upon set its percentile rating threshold. This value is only
	///   applicable to DISPERSION event.
	/// - RANKING: Data/transaction usage ranking high (i.e.value 1), medium (2)
	///   or low (3). This value is only applicable to DISPERSION event.
	/// - PERCENTILE_RANKING: Percentile ranking of the target UE in the
	///   Cumulative Distribution Function of data usage for the population of
	///   all UEs. This value is only applicable to DISPERSION event.
	/// - RSSI: Indicated the RSSI in the unit of dBm. This value is only
	///   applicable to WLAN_PERFORMANCE event.
	/// - RTT: Indicates the RTT in the unit of millisecond. This value is only
	///   applicable to WLAN_PERFORMANCE event.
	/// - TRAFFIC_INFO: Traffic information including UL/DL data rate and/or
	///   Traffic volume. This value is only applicable to WLAN_PERFORMANCE
	///   event.
	/// - NUMBER_OF_UES: Number of UEs observed for the SSID. This value is only
	///   applicable to WLAN_PERFORMANCE event.
	/// - APP_LIST_FOR_UE_COMM: The analytics of the application list used by
	///   UE. This value is only applicable to UE_COMM event.
	/// - N4_SESS_INACT_TIMER_FOR_UE_COMM: The N4 Session inactivity timer. This
	///   value is only applicable to UE_COMM event.
	/// - AVG_TRAFFIC_RATE: Indicates average traffic rate. This value is only
	///   applicable to DN_PERFORMANCE event.
	/// - MAX_TRAFFIC_RATE: Indicates maximum traffic rate. This value is only
	///   applicable to DN_PERFORMANCE event.
	/// - AVG_PACKET_DELAY: Indicates average Packet Delay. This value is only
	///   applicable to DN_PERFORMANCE event.
	/// - MAX_PACKET_DELAY: Indicates maximum Packet Delay. This value is only
	///   applicable to DN_PERFORMANCE event.
	/// - AVG_PACKET_LOSS_RATE: Indicates average Loss Rate. This value is only
	///   applicable to DN_PERFORMANCE event.
	/// - UE_LOCATION: Indicates UE location information. This value is only
	///   applicable to SERVICE_EXPERIENCE event.
	/// - LIST_OF_HIGH_EXP_UE: Indicates list of high experienced UE. This value
	///   is only applicable to SM_CONGESTION event.
	/// - LIST_OF_MEDIUM_EXP_UE: Indicates list of medium experienced UE. This
	///   value is only applicable to SM_CONGESTION event.
	/// - LIST_OF_LOW_EXP_UE: Indicates list of low experienced UE. This value
	///   is only applicable to SM_CONGESTION event.
	/// - AVG_UL_PKT_DROP_RATE: Indicates average uplink packet drop rate on
	///   GTP-U path on N3. This value is only applicable to RED_TRANS_EXP
	///   event.
	/// - VAR_UL_PKT_DROP_RATE: Indicates variance of uplink packet drop rate on
	///   GTP-U path on N3. This value is only applicable to RED_TRANS_EXP
	///   event.
	/// - AVG_DL_PKT_DROP_RATE: Indicates average downlink packet drop rate on
	///   GTP-U path on N3. This value is only applicable to RED_TRANS_EXP
	///   event.
	/// - VAR_DL_PKT_DROP_RATE: Indicates variance of downlink packet drop rate
	///   on GTP-U path on N3. This value is only applicable to RED_TRANS_EXP
	///   event.
	/// - AVG_UL_PKT_DELAY: Indicates average uplink packet delay round trip on
	///   GTP-U path on N3. This value is only applicable to RED_TRANS_EXP
	///   event.
	/// - VAR_UL_PKT_DELAY: Indicates variance uplink packet delay round trip on
	///   GTP-U path on N3. This value is only applicable to RED_TRANS_EXP
	///   event.
	/// - AVG_DL_PKT_DELAY: Indicates average downlink packet delay round trip
	///   on GTP-U path on N3. This value is only applicable to RED_TRANS_EXP
	///   event.
	/// - VAR_DL_PKT_DELAY: Indicates variance downlink packet delay round trip
	///   on GTP-U path on N3. This value is only applicable to RED_TRANS_EXP
	///   event.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- NUM_OF_UE_REG: The number of UE
	/// registered. This value is only applicable to NSI_LOAD_LEVEL event.\n-
	/// NUM_OF_PDU_SESS_ESTBL: The number of PDU sessions established. This
	/// value is only applicable to NSI_LOAD_LEVEL event.\n- RES_USAGE: The
	/// current usage of the virtual resources assigned to the NF instances
	/// belonging to a particular network slice instance. This value is only
	/// applicable to NSI_LOAD_LEVEL event.\n-
	/// NUM_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR: The number of times the resource
	/// usage threshold of the network slice instance is reached or exceeded if
	/// a threshold value is provided by the consumer. This value is only
	/// applicable to NSI_LOAD_LEVEL event.\n-
	/// PERIOD_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR: The time interval between
	/// each time the threshold being met or exceeded on the network slice
	/// (instance). This value is only applicable to NSI_LOAD_LEVEL event.\n-
	/// EXCEED_LOAD_LEVEL_THR_IND: Whether the Load Level Threshold is met or
	/// exceeded by the statistics value. This value is only applicable to
	/// NSI_LOAD_LEVEL event.\n- LIST_OF_TOP_APP_UL: The list of applications
	/// that contribute the most to the traffic in the UL direction. This value
	/// is only applicable to USER_DATA_CONGESTION event.\n- LIST_OF_TOP_APP_DL:
	/// The list of applications that contribute the most to the traffic in the
	/// DL direction. This value is only applicable to USER_DATA_CONGESTION
	/// event.\n- NF_STATUS: The availability status of the NF on the Analytics
	/// target period, expressed as a percentage of time per status value
	/// (registered, suspended, undiscoverable). This value is only applicable
	/// to NF_LOAD event.\n- NF_RESOURCE_USAGE: The average usage of assigned
	/// resources (CPU, memory, storage). This value is only applicable to
	/// NF_LOAD event.\n- NF_LOAD: The average load of the NF instance over the
	/// Analytics target period. This value is only applicable to NF_LOAD
	/// event.\n- NF_PEAK_LOAD: The maximum load of the NF instance over the
	/// Analytics target period. This value is only applicable to NF_LOAD
	/// event.\n- NF_LOAD_AVG_IN_AOI: The average load of the NF instances over
	/// the area of interest. This value is only applicable to NF_LOAD event.\n-
	/// DISPER_AMOUNT: Indicates the dispersion amount of the reported data
	/// volume or transaction dispersion type. This value is only applicable to
	/// DISPERSION event.\n- DISPER_CLASS: Indicates the dispersion mobility
	/// class: fixed, camper, traveller upon set its usage threshold, and/or the
	/// top-heavy class upon set its percentile rating threshold. This value is
	/// only applicable to DISPERSION event.\n- RANKING: Data/transaction usage
	/// ranking high (i.e.value 1), medium (2) or low (3). This value is only
	/// applicable to DISPERSION event.\n- PERCENTILE_RANKING: Percentile
	/// ranking of the target UE in the Cumulative Distribution Function of data
	/// usage for the population of all UEs. This value is only applicable to
	/// DISPERSION event.\n- RSSI: Indicated the RSSI in the unit of dBm. This
	/// value is only applicable to WLAN_PERFORMANCE event.\n- RTT: Indicates
	/// the RTT in the unit of millisecond. This value is only applicable to
	/// WLAN_PERFORMANCE event.\n- TRAFFIC_INFO: Traffic information including
	/// UL/DL data rate and/or Traffic volume. This value is only applicable to
	/// WLAN_PERFORMANCE event.\n- NUMBER_OF_UES: Number of UEs observed for the
	/// SSID. This value is only applicable to WLAN_PERFORMANCE event.\n-
	/// APP_LIST_FOR_UE_COMM: The analytics of the application list used by UE.
	/// This value is only applicable to UE_COMM event.\n-
	/// N4_SESS_INACT_TIMER_FOR_UE_COMM: The N4 Session inactivity timer. This
	/// value is only applicable to UE_COMM event.\n- AVG_TRAFFIC_RATE:
	/// Indicates average traffic rate. This value is only applicable to
	/// DN_PERFORMANCE event.\n- MAX_TRAFFIC_RATE: Indicates maximum traffic
	/// rate. This value is only applicable to DN_PERFORMANCE event.\n-
	/// AVG_PACKET_DELAY: Indicates average Packet Delay. This value is only
	/// applicable to DN_PERFORMANCE event.\n- MAX_PACKET_DELAY: Indicates
	/// maximum Packet Delay. This value is only applicable to DN_PERFORMANCE
	/// event.\n- AVG_PACKET_LOSS_RATE: Indicates average Loss Rate. This value
	/// is only applicable to DN_PERFORMANCE event.\n- UE_LOCATION: Indicates UE
	/// location information. This value is only applicable to
	/// SERVICE_EXPERIENCE event.\n- LIST_OF_HIGH_EXP_UE: Indicates list of high
	/// experienced UE. This value is only applicable to SM_CONGESTION event.\n-
	/// LIST_OF_MEDIUM_EXP_UE: Indicates list of medium experienced UE. This
	/// value is only applicable to SM_CONGESTION event.\n- LIST_OF_LOW_EXP_UE:
	/// Indicates list of low experienced UE. This value is only applicable to
	/// SM_CONGESTION event.\n- AVG_UL_PKT_DROP_RATE: Indicates average uplink
	/// packet drop rate on GTP-U path on N3. This value is only applicable to
	/// RED_TRANS_EXP event.\n- VAR_UL_PKT_DROP_RATE: Indicates variance of
	/// uplink packet drop rate on GTP-U path on N3. This value is only
	/// applicable to RED_TRANS_EXP event.\n- AVG_DL_PKT_DROP_RATE: Indicates
	/// average downlink packet drop rate on GTP-U path on N3. This value is
	/// only applicable to RED_TRANS_EXP event.\n- VAR_DL_PKT_DROP_RATE:
	/// Indicates variance of downlink packet drop rate on GTP-U path on N3.
	/// This value is only applicable to RED_TRANS_EXP event.\n-
	/// AVG_UL_PKT_DELAY: Indicates average uplink packet delay round trip on
	/// GTP-U path on N3. This value is only applicable to RED_TRANS_EXP
	/// event.\n- VAR_UL_PKT_DELAY: Indicates variance uplink packet delay round
	/// trip on GTP-U path on N3. This value is only applicable to RED_TRANS_EXP
	/// event.\n- AVG_DL_PKT_DELAY: Indicates average downlink packet delay
	/// round trip on GTP-U path on N3. This value is only applicable to
	/// RED_TRANS_EXP event.\n- VAR_DL_PKT_DELAY: Indicates variance downlink
	/// packet delay round trip on GTP-U path on N3. This value is only
	/// applicable to RED_TRANS_EXP event.\n",
	///  "type": "string",
	///  "enum": [
	///    "NUM_OF_UE_REG",
	///    "NUM_OF_PDU_SESS_ESTBL",
	///    "RES_USAGE",
	///    "NUM_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR",
	///    "PERIOD_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR",
	///    "EXCEED_LOAD_LEVEL_THR_IND",
	///    "LIST_OF_TOP_APP_UL",
	///    "LIST_OF_TOP_APP_DL",
	///    "NF_STATUS",
	///    "NF_RESOURCE_USAGE",
	///    "NF_LOAD",
	///    "NF_PEAK_LOAD",
	///    "NF_LOAD_AVG_IN_AOI",
	///    "DISPER_AMOUNT",
	///    "DISPER_CLASS",
	///    "RANKING",
	///    "PERCENTILE_RANKING",
	///    "RSSI",
	///    "RTT",
	///    "TRAFFIC_INFO",
	///    "NUMBER_OF_UES",
	///    "APP_LIST_FOR_UE_COMM",
	///    "N4_SESS_INACT_TIMER_FOR_UE_COMM",
	///    "AVG_TRAFFIC_RATE",
	///    "MAX_TRAFFIC_RATE",
	///    "AVG_PACKET_DELAY",
	///    "MAX_PACKET_DELAY",
	///    "AVG_PACKET_LOSS_RATE",
	///    "UE_LOCATION",
	///    "LIST_OF_HIGH_EXP_UE",
	///    "LIST_OF_MEDIUM_EXP_UE",
	///    "LIST_OF_LOW_EXP_UE",
	///    "AVG_UL_PKT_DROP_RATE",
	///    "VAR_UL_PKT_DROP_RATE",
	///    "AVG_DL_PKT_DROP_RATE",
	///    "VAR_DL_PKT_DROP_RATE",
	///    "AVG_UL_PKT_DELAY",
	///    "VAR_UL_PKT_DELAY",
	///    "AVG_DL_PKT_DELAY",
	///    "VAR_DL_PKT_DELAY"
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
	pub enum AnalyticsSubset {
		#[default]
		#[serde(rename = "NUM_OF_UE_REG")]
		NumOfUeReg,
		#[serde(rename = "NUM_OF_PDU_SESS_ESTBL")]
		NumOfPduSessEstbl,
		#[serde(rename = "RES_USAGE")]
		ResUsage,
		#[serde(rename = "NUM_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR")]
		NumOfExceedResUsageLoadLevelThr,
		#[serde(rename = "PERIOD_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR")]
		PeriodOfExceedResUsageLoadLevelThr,
		#[serde(rename = "EXCEED_LOAD_LEVEL_THR_IND")]
		ExceedLoadLevelThrInd,
		#[serde(rename = "LIST_OF_TOP_APP_UL")]
		ListOfTopAppUl,
		#[serde(rename = "LIST_OF_TOP_APP_DL")]
		ListOfTopAppDl,
		#[serde(rename = "NF_STATUS")]
		NfStatus,
		#[serde(rename = "NF_RESOURCE_USAGE")]
		NfResourceUsage,
		#[serde(rename = "NF_LOAD")]
		NfLoad,
		#[serde(rename = "NF_PEAK_LOAD")]
		NfPeakLoad,
		#[serde(rename = "NF_LOAD_AVG_IN_AOI")]
		NfLoadAvgInAoi,
		#[serde(rename = "DISPER_AMOUNT")]
		DisperAmount,
		#[serde(rename = "DISPER_CLASS")]
		DisperClass,
		#[serde(rename = "RANKING")]
		Ranking,
		#[serde(rename = "PERCENTILE_RANKING")]
		PercentileRanking,
		#[serde(rename = "RSSI")]
		Rssi,
		#[serde(rename = "RTT")]
		Rtt,
		#[serde(rename = "TRAFFIC_INFO")]
		TrafficInfo,
		#[serde(rename = "NUMBER_OF_UES")]
		NumberOfUes,
		#[serde(rename = "APP_LIST_FOR_UE_COMM")]
		AppListForUeComm,
		#[serde(rename = "N4_SESS_INACT_TIMER_FOR_UE_COMM")]
		N4SessInactTimerForUeComm,
		#[serde(rename = "AVG_TRAFFIC_RATE")]
		AvgTrafficRate,
		#[serde(rename = "MAX_TRAFFIC_RATE")]
		MaxTrafficRate,
		#[serde(rename = "AVG_PACKET_DELAY")]
		AvgPacketDelay,
		#[serde(rename = "MAX_PACKET_DELAY")]
		MaxPacketDelay,
		#[serde(rename = "AVG_PACKET_LOSS_RATE")]
		AvgPacketLossRate,
		#[serde(rename = "UE_LOCATION")]
		UeLocation,
		#[serde(rename = "LIST_OF_HIGH_EXP_UE")]
		ListOfHighExpUe,
		#[serde(rename = "LIST_OF_MEDIUM_EXP_UE")]
		ListOfMediumExpUe,
		#[serde(rename = "LIST_OF_LOW_EXP_UE")]
		ListOfLowExpUe,
		#[serde(rename = "AVG_UL_PKT_DROP_RATE")]
		AvgUlPktDropRate,
		#[serde(rename = "VAR_UL_PKT_DROP_RATE")]
		VarUlPktDropRate,
		#[serde(rename = "AVG_DL_PKT_DROP_RATE")]
		AvgDlPktDropRate,
		#[serde(rename = "VAR_DL_PKT_DROP_RATE")]
		VarDlPktDropRate,
		#[serde(rename = "AVG_UL_PKT_DELAY")]
		AvgUlPktDelay,
		#[serde(rename = "VAR_UL_PKT_DELAY")]
		VarUlPktDelay,
		#[serde(rename = "AVG_DL_PKT_DELAY")]
		AvgDlPktDelay,
		#[serde(rename = "VAR_DL_PKT_DELAY")]
		VarDlPktDelay,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&AnalyticsSubset> for AnalyticsSubset {
		fn from(value: &AnalyticsSubset) -> Self {
			value.clone()
		}
	}

	impl ToString for AnalyticsSubset {
		fn to_string(&self) -> String {
			match *self {
				Self::NumOfUeReg => "NUM_OF_UE_REG".to_string(),
				Self::NumOfPduSessEstbl => "NUM_OF_PDU_SESS_ESTBL".to_string(),
				Self::ResUsage => "RES_USAGE".to_string(),
				Self::NumOfExceedResUsageLoadLevelThr => {
					"NUM_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR".to_string()
				}
				Self::PeriodOfExceedResUsageLoadLevelThr => {
					"PERIOD_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR".to_string()
				}
				Self::ExceedLoadLevelThrInd => "EXCEED_LOAD_LEVEL_THR_IND".to_string(),
				Self::ListOfTopAppUl => "LIST_OF_TOP_APP_UL".to_string(),
				Self::ListOfTopAppDl => "LIST_OF_TOP_APP_DL".to_string(),
				Self::NfStatus => "NF_STATUS".to_string(),
				Self::NfResourceUsage => "NF_RESOURCE_USAGE".to_string(),
				Self::NfLoad => "NF_LOAD".to_string(),
				Self::NfPeakLoad => "NF_PEAK_LOAD".to_string(),
				Self::NfLoadAvgInAoi => "NF_LOAD_AVG_IN_AOI".to_string(),
				Self::DisperAmount => "DISPER_AMOUNT".to_string(),
				Self::DisperClass => "DISPER_CLASS".to_string(),
				Self::Ranking => "RANKING".to_string(),
				Self::PercentileRanking => "PERCENTILE_RANKING".to_string(),
				Self::Rssi => "RSSI".to_string(),
				Self::Rtt => "RTT".to_string(),
				Self::TrafficInfo => "TRAFFIC_INFO".to_string(),
				Self::NumberOfUes => "NUMBER_OF_UES".to_string(),
				Self::AppListForUeComm => "APP_LIST_FOR_UE_COMM".to_string(),
				Self::N4SessInactTimerForUeComm => "N4_SESS_INACT_TIMER_FOR_UE_COMM".to_string(),
				Self::AvgTrafficRate => "AVG_TRAFFIC_RATE".to_string(),
				Self::MaxTrafficRate => "MAX_TRAFFIC_RATE".to_string(),
				Self::AvgPacketDelay => "AVG_PACKET_DELAY".to_string(),
				Self::MaxPacketDelay => "MAX_PACKET_DELAY".to_string(),
				Self::AvgPacketLossRate => "AVG_PACKET_LOSS_RATE".to_string(),
				Self::UeLocation => "UE_LOCATION".to_string(),
				Self::ListOfHighExpUe => "LIST_OF_HIGH_EXP_UE".to_string(),
				Self::ListOfMediumExpUe => "LIST_OF_MEDIUM_EXP_UE".to_string(),
				Self::ListOfLowExpUe => "LIST_OF_LOW_EXP_UE".to_string(),
				Self::AvgUlPktDropRate => "AVG_UL_PKT_DROP_RATE".to_string(),
				Self::VarUlPktDropRate => "VAR_UL_PKT_DROP_RATE".to_string(),
				Self::AvgDlPktDropRate => "AVG_DL_PKT_DROP_RATE".to_string(),
				Self::VarDlPktDropRate => "VAR_DL_PKT_DROP_RATE".to_string(),
				Self::AvgUlPktDelay => "AVG_UL_PKT_DELAY".to_string(),
				Self::VarUlPktDelay => "VAR_UL_PKT_DELAY".to_string(),
				Self::AvgDlPktDelay => "AVG_DL_PKT_DELAY".to_string(),
				Self::VarDlPktDelay => "VAR_DL_PKT_DELAY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for AnalyticsSubset {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NUM_OF_UE_REG" => Ok(Self::NumOfUeReg),
				"NUM_OF_PDU_SESS_ESTBL" => Ok(Self::NumOfPduSessEstbl),
				"RES_USAGE" => Ok(Self::ResUsage),
				"NUM_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR" => {
					Ok(Self::NumOfExceedResUsageLoadLevelThr)
				}
				"PERIOD_OF_EXCEED_RES_USAGE_LOAD_LEVEL_THR" => {
					Ok(Self::PeriodOfExceedResUsageLoadLevelThr)
				}
				"EXCEED_LOAD_LEVEL_THR_IND" => Ok(Self::ExceedLoadLevelThrInd),
				"LIST_OF_TOP_APP_UL" => Ok(Self::ListOfTopAppUl),
				"LIST_OF_TOP_APP_DL" => Ok(Self::ListOfTopAppDl),
				"NF_STATUS" => Ok(Self::NfStatus),
				"NF_RESOURCE_USAGE" => Ok(Self::NfResourceUsage),
				"NF_LOAD" => Ok(Self::NfLoad),
				"NF_PEAK_LOAD" => Ok(Self::NfPeakLoad),
				"NF_LOAD_AVG_IN_AOI" => Ok(Self::NfLoadAvgInAoi),
				"DISPER_AMOUNT" => Ok(Self::DisperAmount),
				"DISPER_CLASS" => Ok(Self::DisperClass),
				"RANKING" => Ok(Self::Ranking),
				"PERCENTILE_RANKING" => Ok(Self::PercentileRanking),
				"RSSI" => Ok(Self::Rssi),
				"RTT" => Ok(Self::Rtt),
				"TRAFFIC_INFO" => Ok(Self::TrafficInfo),
				"NUMBER_OF_UES" => Ok(Self::NumberOfUes),
				"APP_LIST_FOR_UE_COMM" => Ok(Self::AppListForUeComm),
				"N4_SESS_INACT_TIMER_FOR_UE_COMM" => Ok(Self::N4SessInactTimerForUeComm),
				"AVG_TRAFFIC_RATE" => Ok(Self::AvgTrafficRate),
				"MAX_TRAFFIC_RATE" => Ok(Self::MaxTrafficRate),
				"AVG_PACKET_DELAY" => Ok(Self::AvgPacketDelay),
				"MAX_PACKET_DELAY" => Ok(Self::MaxPacketDelay),
				"AVG_PACKET_LOSS_RATE" => Ok(Self::AvgPacketLossRate),
				"UE_LOCATION" => Ok(Self::UeLocation),
				"LIST_OF_HIGH_EXP_UE" => Ok(Self::ListOfHighExpUe),
				"LIST_OF_MEDIUM_EXP_UE" => Ok(Self::ListOfMediumExpUe),
				"LIST_OF_LOW_EXP_UE" => Ok(Self::ListOfLowExpUe),
				"AVG_UL_PKT_DROP_RATE" => Ok(Self::AvgUlPktDropRate),
				"VAR_UL_PKT_DROP_RATE" => Ok(Self::VarUlPktDropRate),
				"AVG_DL_PKT_DROP_RATE" => Ok(Self::AvgDlPktDropRate),
				"VAR_DL_PKT_DROP_RATE" => Ok(Self::VarDlPktDropRate),
				"AVG_UL_PKT_DELAY" => Ok(Self::AvgUlPktDelay),
				"VAR_UL_PKT_DELAY" => Ok(Self::VarUlPktDelay),
				"AVG_DL_PKT_DELAY" => Ok(Self::AvgDlPktDelay),
				"VAR_DL_PKT_DELAY" => Ok(Self::VarDlPktDelay),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for AnalyticsSubset {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AnalyticsSubset {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AnalyticsSubset {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// "false" represents not applicable for all slices. "true" represents
	/// applicable for all slices.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "\"false\" represents not applicable for all slices.
	/// \"true\" represents applicable for all slices.\n",
	///  "type": "boolean"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AnySlice(pub bool);

	impl ::std::ops::Deref for AnySlice {
		type Target = bool;
		fn deref(&self) -> &bool {
			&self.0
		}
	}

	impl From<AnySlice> for bool {
		fn from(value: AnySlice) -> Self {
			value.0
		}
	}

	impl From<&AnySlice> for AnySlice {
		fn from(value: &AnySlice) -> Self {
			value.clone()
		}
	}

	impl From<bool> for AnySlice {
		fn from(value: bool) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for AnySlice {
		type Err = <bool as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for AnySlice {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for AnySlice {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for AnySlice {
		type Error = <bool as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for AnySlice {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Represents the analytics of the application list used by UE.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the analytics of the application list used
	/// by UE.",
	///  "type": "object",
	///  "required": [
	///    "appId"
	///  ],
	///  "properties": {
	///    "appDur": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "appId": {
	///      "$ref": "#/components/schemas/ApplicationId"
	///    },
	///    "occurRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "spatialValidity": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "startTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AppListForUeComm {
		#[serde(rename = "appDur", default, skip_serializing_if = "Option::is_none")]
		pub app_dur: Option<DurationSec>,
		#[serde(rename = "appId")]
		pub app_id: ApplicationId,
		#[serde(
			rename = "occurRatio",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub occur_ratio: Option<SamplingRatio>,
		#[serde(
			rename = "spatialValidity",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub spatial_validity: Option<NetworkAreaInfo>,
		#[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
		pub start_time: Option<DateTime>,
	}

	impl From<&AppListForUeComm> for AppListForUeComm {
		fn from(value: &AppListForUeComm) -> Self {
			value.clone()
		}
	}

	/// Application data volume per Application Id.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Application data volume per Application Id.",
	///  "type": "object",
	///  "required": [
	///    "appId",
	///    "appVolume"
	///  ],
	///  "properties": {
	///    "appId": {
	///      "$ref": "#/components/schemas/ApplicationId"
	///    },
	///    "appVolume": {
	///      "$ref": "#/components/schemas/Volume"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ApplicationVolume {
		#[serde(rename = "appId")]
		pub app_id: ApplicationId,
		#[serde(rename = "appVolume")]
		pub app_volume: Volume,
	}

	impl From<&ApplicationVolume> for ApplicationVolume {
		fn from(value: &ApplicationVolume) -> Self {
			value.clone()
		}
	}

	/// Indicates the information of area based event reporting.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the information of area based event
	/// reporting.",
	///  "type": "object",
	///  "required": [
	///    "areaDefinition"
	///  ],
	///  "properties": {
	///    "areaDefinition": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ReportingArea"
	///      },
	///      "maxItems": 250,
	///      "minItems": 1
	///    },
	///    "maximumInterval": {
	///      "$ref": "#/components/schemas/MaximumInterval"
	///    },
	///    "minimumInterval": {
	///      "$ref": "#/components/schemas/MinimumInterval"
	///    },
	///    "occurrenceInfo": {
	///      "$ref": "#/components/schemas/OccurrenceInfo"
	///    },
	///    "reportingDuration": {
	///      "$ref": "#/components/schemas/ReportingDuration"
	///    },
	///    "reportingLocationReq": {
	///      "default": true,
	///      "type": "boolean"
	///    },
	///    "samplingInterval": {
	///      "$ref": "#/components/schemas/SamplingInterval"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AreaEventInfo {
		#[serde(rename = "areaDefinition")]
		pub area_definition: Vec<ReportingArea>,
		#[serde(
			rename = "maximumInterval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub maximum_interval: Option<MaximumInterval>,
		#[serde(
			rename = "minimumInterval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub minimum_interval: Option<MinimumInterval>,
		#[serde(
			rename = "occurrenceInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub occurrence_info: Option<OccurrenceInfo>,
		#[serde(
			rename = "reportingDuration",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reporting_duration: Option<ReportingDuration>,
		#[serde(
			rename = "reportingLocationReq",
			default = "defaults::default_bool::<true>"
		)]
		pub reporting_location_req: bool,
		#[serde(
			rename = "samplingInterval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sampling_interval: Option<SamplingInterval>,
	}

	impl From<&AreaEventInfo> for AreaEventInfo {
		fn from(value: &AreaEventInfo) -> Self {
			value.clone()
		}
	}

	/// Event State of AoI event in old AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Event State of AoI event in old AMF",
	///  "type": "object",
	///  "required": [
	///    "presence"
	///  ],
	///  "properties": {
	///    "individualPraIdList": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "presence": {
	///      "$ref": "#/components/schemas/PresenceState"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AreaOfInterestEventState {
		#[serde(
			rename = "individualPraIdList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub individual_pra_id_list: Vec<String>,
		pub presence: PresenceState,
	}

	impl From<&AreaOfInterestEventState> for AreaOfInterestEventState {
		fn from(value: &AreaOfInterestEventState) -> Self {
			value.clone()
		}
	}

	/// Area of validity information for N2 information transfer
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Area of validity information for N2 information
	/// transfer",
	///  "type": "object",
	///  "required": [
	///    "taiList"
	///  ],
	///  "properties": {
	///    "taiList": {
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
	pub struct AreaOfValidity {
		#[serde(rename = "taiList")]
		pub tai_list: Vec<Tai>,
	}

	impl From<&AreaOfValidity> for AreaOfValidity {
		fn from(value: &AreaOfValidity) -> Self {
			value.clone()
		}
	}

	/// Contains the 5G acess stratum time distribution parameters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the 5G acess stratum time distribution
	/// parameters.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "asTimeDistInd": {
	///      "type": "boolean"
	///    },
	///    "uuErrorBudget": {
	///      "$ref": "#/components/schemas/UintegerRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AsTimeDistributionParam(pub Option<AsTimeDistributionParamInner>);

	impl ::std::ops::Deref for AsTimeDistributionParam {
		type Target = Option<AsTimeDistributionParamInner>;
		fn deref(&self) -> &Option<AsTimeDistributionParamInner> {
			&self.0
		}
	}

	impl From<AsTimeDistributionParam> for Option<AsTimeDistributionParamInner> {
		fn from(value: AsTimeDistributionParam) -> Self {
			value.0
		}
	}

	impl From<&AsTimeDistributionParam> for AsTimeDistributionParam {
		fn from(value: &AsTimeDistributionParam) -> Self {
			value.clone()
		}
	}

	impl From<Option<AsTimeDistributionParamInner>> for AsTimeDistributionParam {
		fn from(value: Option<AsTimeDistributionParamInner>) -> Self {
			Self(value)
		}
	}

	/// Contains the 5G acess stratum time distribution parameters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the 5G acess stratum time distribution
	/// parameters.",
	///  "type": "object",
	///  "properties": {
	///    "asTimeDistInd": {
	///      "type": "boolean"
	///    },
	///    "uuErrorBudget": {
	///      "$ref": "#/components/schemas/UintegerRm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AsTimeDistributionParamInner {
		#[serde(
			rename = "asTimeDistInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub as_time_dist_ind: Option<bool>,
		#[serde(
			rename = "uuErrorBudget",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uu_error_budget: Option<UintegerRm>,
	}

	impl From<&AsTimeDistributionParamInner> for AsTimeDistributionParamInner {
		fn from(value: &AsTimeDistributionParamInner) -> Self {
			value.clone()
		}
	}

	/// Data within an EBI assignment request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within an EBI assignment request",
	///  "type": "object",
	///  "required": [
	///    "pduSessionId"
	///  ],
	///  "properties": {
	///    "arpList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Arp"
	///      },
	///      "minItems": 1
	///    },
	///    "modifiedEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EbiArpMapping"
	///      },
	///      "minItems": 1
	///    },
	///    "oldGuami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "releasedEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerId"
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
	pub struct AssignEbiData {
		#[serde(rename = "arpList", default, skip_serializing_if = "Vec::is_empty")]
		pub arp_list: Vec<Arp>,
		#[serde(
			rename = "modifiedEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub modified_ebi_list: Vec<EbiArpMapping>,
		#[serde(rename = "oldGuami", default, skip_serializing_if = "Option::is_none")]
		pub old_guami: Option<crate::common::common_models::Guami>,
		#[serde(rename = "pduSessionId")]
		pub pdu_session_id: PduSessionId,
		#[serde(
			rename = "releasedEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub released_ebi_list: Vec<EpsBearerId>,
	}

	impl From<&AssignEbiData> for AssignEbiData {
		fn from(value: &AssignEbiData) -> Self {
			value.clone()
		}
	}

	/// Data within a failure response to the EBI assignment request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a failure response to the EBI assignment
	/// request",
	///  "type": "object",
	///  "required": [
	///    "error",
	///    "failureDetails"
	///  ],
	///  "properties": {
	///    "error": {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    "failureDetails": {
	///      "$ref": "#/components/schemas/AssignEbiFailed"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AssignEbiError {
		pub error: ProblemDetails,
		#[serde(rename = "failureDetails")]
		pub failure_details: AssignEbiFailed,
	}

	impl From<&AssignEbiError> for AssignEbiError {
		fn from(value: &AssignEbiError) -> Self {
			value.clone()
		}
	}

	/// Represents failed assignment of EBI(s)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents failed assignment of EBI(s)",
	///  "type": "object",
	///  "required": [
	///    "pduSessionId"
	///  ],
	///  "properties": {
	///    "failedArpList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Arp"
	///      },
	///      "minItems": 1
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct AssignEbiFailed {
		#[serde(
			rename = "failedArpList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub failed_arp_list: Vec<Arp>,
		#[serde(rename = "pduSessionId")]
		pub pdu_session_id: PduSessionId,
	}

	impl From<&AssignEbiFailed> for AssignEbiFailed {
		fn from(value: &AssignEbiFailed) -> Self {
			value.clone()
		}
	}

	/// Data within a successful response to an EBI assignment request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a successful response to an EBI assignment
	/// request",
	///  "type": "object",
	///  "required": [
	///    "assignedEbiList",
	///    "pduSessionId"
	///  ],
	///  "properties": {
	///    "assignedEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EbiArpMapping"
	///      },
	///      "minItems": 0
	///    },
	///    "failedArpList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Arp"
	///      },
	///      "minItems": 1
	///    },
	///    "modifiedEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerId"
	///      },
	///      "minItems": 1
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "releasedEbiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EpsBearerId"
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
	pub struct AssignedEbiData {
		#[serde(rename = "assignedEbiList")]
		pub assigned_ebi_list: Vec<EbiArpMapping>,
		#[serde(
			rename = "failedArpList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub failed_arp_list: Vec<Arp>,
		#[serde(
			rename = "modifiedEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub modified_ebi_list: Vec<EpsBearerId>,
		#[serde(rename = "pduSessionId")]
		pub pdu_session_id: PduSessionId,
		#[serde(
			rename = "releasedEbiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub released_ebi_list: Vec<EpsBearerId>,
	}

	impl From<&AssignedEbiData> for AssignedEbiData {
		fn from(value: &AssignedEbiData) -> Self {
			value.clone()
		}
	}

	/// Specifies the measured uncompensated atmospheric pressure.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Specifies the measured uncompensated atmospheric
	/// pressure.",
	///  "type": "integer",
	///  "maximum": 115000.0,
	///  "minimum": 30000.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BarometricPressure(pub i64);

	impl ::std::ops::Deref for BarometricPressure {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<BarometricPressure> for i64 {
		fn from(value: BarometricPressure) -> Self {
			value.0
		}
	}

	impl From<&BarometricPressure> for BarometricPressure {
		fn from(value: &BarometricPressure) -> Self {
			value.clone()
		}
	}

	impl From<i64> for BarometricPressure {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for BarometricPressure {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for BarometricPressure {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for BarometricPressure {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for BarometricPressure {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for BarometricPressure {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Represents bandwidth requirements.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents bandwidth requirements.",
	///  "type": "object",
	///  "required": [
	///    "appId"
	///  ],
	///  "properties": {
	///    "appId": {
	///      "$ref": "#/components/schemas/ApplicationId"
	///    },
	///    "marBwDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "marBwUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "mirBwDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "mirBwUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct BwRequirement {
		#[serde(rename = "appId")]
		pub app_id: ApplicationId,
		#[serde(rename = "marBwDl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_dl: Option<BitRate>,
		#[serde(rename = "marBwUl", default, skip_serializing_if = "Option::is_none")]
		pub mar_bw_ul: Option<BitRate>,
		#[serde(rename = "mirBwDl", default, skip_serializing_if = "Option::is_none")]
		pub mir_bw_dl: Option<BitRate>,
		#[serde(rename = "mirBwUl", default, skip_serializing_if = "Option::is_none")]
		pub mir_bw_ul: Option<BitRate>,
	}

	impl From<&BwRequirement> for BwRequirement {
		fn from(value: &BwRequirement) -> Self {
			value.clone()
		}
	}

	/// CagData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "cagInfos"
	///  ],
	///  "properties": {
	///    "cagInfos": {
	///      "description": "A map (list of key-value pairs where PlmnId serves
	/// as key) of CagInfo",
	///      "type": "object",
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/CagInfo"
	///      }
	///    },
	///    "provisioningTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CagData {
		/// A map (list of key-value pairs where PlmnId serves as key) of
		/// CagInfo
		#[serde(rename = "cagInfos")]
		pub cag_infos: ::std::collections::HashMap<String, CagInfo>,
		#[serde(
			rename = "provisioningTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub provisioning_time: Option<DateTime>,
	}

	impl From<&CagData> for CagData {
		fn from(value: &CagData) -> Self {
			value.clone()
		}
	}

	/// CagInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "allowedCagList"
	///  ],
	///  "properties": {
	///    "allowedCagList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CagId"
	///      }
	///    },
	///    "cagOnlyIndicator": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CagInfo {
		#[serde(rename = "allowedCagList")]
		pub allowed_cag_list: Vec<CagId>,
		#[serde(
			rename = "cagOnlyIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cag_only_indicator: Option<bool>,
	}

	impl From<&CagInfo> for CagInfo {
		fn from(value: &CagInfo) -> Self {
			value.clone()
		}
	}

	/// Data within a Cancel Location Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a Cancel Location Request",
	///  "type": "object",
	///  "required": [
	///    "hgmlcCallBackURI",
	///    "ldrReference",
	///    "supi"
	///  ],
	///  "properties": {
	///    "hgmlcCallBackURI": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "ldrReference": {
	///      "$ref": "#/components/schemas/LdrReference"
	///    },
	///    "servingLMFIdentification": {
	///      "$ref": "#/components/schemas/LMFIdentification"
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
	pub struct CancelPosInfo {
		#[serde(rename = "hgmlcCallBackURI")]
		pub hgmlc_call_back_uri: Uri,
		#[serde(rename = "ldrReference")]
		pub ldr_reference: LdrReference,
		#[serde(
			rename = "servingLMFIdentification",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_lmf_identification: Option<LmfIdentification>,
		pub supi: Supi,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&CancelPosInfo> for CancelPosInfo {
		fn from(value: &CancelPosInfo) -> Self {
			value.clone()
		}
	}

	/// Represents a list of candidate DNNs for replacement for an S-NSSAI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a list of candidate DNNs for replacement for
	/// an S-NSSAI.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "required": [
	///    "snssai"
	///  ],
	///  "properties": {
	///    "dnns": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/Dnn"
	///      },
	///      "minItems": 1
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
	pub struct CandidateForReplacement(pub Option<CandidateForReplacementInner>);

	impl ::std::ops::Deref for CandidateForReplacement {
		type Target = Option<CandidateForReplacementInner>;
		fn deref(&self) -> &Option<CandidateForReplacementInner> {
			&self.0
		}
	}

	impl From<CandidateForReplacement> for Option<CandidateForReplacementInner> {
		fn from(value: CandidateForReplacement) -> Self {
			value.0
		}
	}

	impl From<&CandidateForReplacement> for CandidateForReplacement {
		fn from(value: &CandidateForReplacement) -> Self {
			value.clone()
		}
	}

	impl From<Option<CandidateForReplacementInner>> for CandidateForReplacement {
		fn from(value: Option<CandidateForReplacementInner>) -> Self {
			Self(value)
		}
	}

	/// Represents a list of candidate DNNs for replacement for an S-NSSAI.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a list of candidate DNNs for replacement for
	/// an S-NSSAI.",
	///  "type": "object",
	///  "required": [
	///    "snssai"
	///  ],
	///  "properties": {
	///    "dnns": {
	///      "type": [
	///        "array",
	///        "null"
	///      ],
	///      "items": {
	///        "$ref": "#/components/schemas/Dnn"
	///      },
	///      "minItems": 1
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
	pub struct CandidateForReplacementInner {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnns: Option<Vec<Dnn>>,
		pub snssai: Snssai,
	}

	impl From<&CandidateForReplacementInner> for CandidateForReplacementInner {
		fn from(value: &CandidateForReplacementInner) -> Self {
			value.clone()
		}
	}

	/// CE-mode-B Support Indicator.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "CE-mode-B Support Indicator.",
	///  "type": "object",
	///  "required": [
	///    "ceModeBSupportInd"
	///  ],
	///  "properties": {
	///    "ceModeBSupportInd": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CeModeBInd {
		#[serde(rename = "ceModeBSupportInd")]
		pub ce_mode_b_support_ind: bool,
	}

	impl From<&CeModeBInd> for CeModeBInd {
		fn from(value: &CeModeBInd) -> Self {
			value.clone()
		}
	}

	/// Indicates the supported Ciphering Algorithm
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the supported Ciphering Algorithm",
	///  "type": "string",
	///  "enum": [
	///    "NEA0",
	///    "NEA1",
	///    "NEA2",
	///    "NEA3"
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
	pub enum CipheringAlgorithm {
		#[default]
		#[serde(rename = "NEA0")]
		Nea0,
		#[serde(rename = "NEA1")]
		Nea1,
		#[serde(rename = "NEA2")]
		Nea2,
		#[serde(rename = "NEA3")]
		Nea3,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&CipheringAlgorithm> for CipheringAlgorithm {
		fn from(value: &CipheringAlgorithm) -> Self {
			value.clone()
		}
	}

	impl ToString for CipheringAlgorithm {
		fn to_string(&self) -> String {
			match *self {
				Self::Nea0 => "NEA0".to_string(),
				Self::Nea1 => "NEA1".to_string(),
				Self::Nea2 => "NEA2".to_string(),
				Self::Nea3 => "NEA3".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for CipheringAlgorithm {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NEA0" => Ok(Self::Nea0),
				"NEA1" => Ok(Self::Nea1),
				"NEA2" => Ok(Self::Nea2),
				"NEA3" => Ok(Self::Nea3),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for CipheringAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for CipheringAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for CipheringAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the description of a circumstance.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the description of a circumstance.",
	///  "type": "object",
	///  "properties": {
	///    "freq": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "locArea": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "tm": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "vol": {
	///      "$ref": "#/components/schemas/Volume"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CircumstanceDescription {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub freq: Option<Float>,
		#[serde(rename = "locArea", default, skip_serializing_if = "Option::is_none")]
		pub loc_area: Option<NetworkAreaInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub tm: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub vol: Option<Volume>,
	}

	impl From<&CircumstanceDescription> for CircumstanceDescription {
		fn from(value: &CircumstanceDescription) -> Self {
			value.clone()
		}
	}

	/// Indicates the dispersion class criterion for fixed, camper and/or
	/// traveller UE, and/or the top-heavy UE dispersion class criterion.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the dispersion class criterion for fixed,
	/// camper and/or traveller UE, and/or the top-heavy UE dispersion class
	/// criterion.\n",
	///  "type": "object",
	///  "required": [
	///    "classThreshold",
	///    "disperClass",
	///    "thresMatch"
	///  ],
	///  "properties": {
	///    "classThreshold": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "disperClass": {
	///      "$ref": "#/components/schemas/DispersionClass"
	///    },
	///    "thresMatch": {
	///      "$ref": "#/components/schemas/MatchingDirection"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ClassCriterion {
		#[serde(rename = "classThreshold")]
		pub class_threshold: SamplingRatio,
		#[serde(rename = "disperClass")]
		pub disper_class: DispersionClass,
		#[serde(rename = "thresMatch")]
		pub thres_match: MatchingDirection,
	}

	impl From<&ClassCriterion> for ClassCriterion {
		fn from(value: &ClassCriterion) -> Self {
			value.clone()
		}
	}

	/// Represents the connection management state of a UE for an access type
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the connection management state of a UE for
	/// an access type",
	///  "type": "object",
	///  "required": [
	///    "accessType",
	///    "cmState"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "cmState": {
	///      "$ref": "#/components/schemas/CmState"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct CmInfo {
		#[serde(rename = "accessType")]
		pub access_type: AccessType,
		#[serde(rename = "cmState")]
		pub cm_state: CmState,
	}

	impl From<&CmInfo> for CmInfo {
		fn from(value: &CmInfo) -> Self {
			value.clone()
		}
	}

	/// Describes the connection management state of a UE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the connection management state of a UE",
	///  "type": "string",
	///  "enum": [
	///    "IDLE",
	///    "CONNECTED"
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
	pub enum CmState {
		#[default]
		#[serde(rename = "IDLE")]
		Idle,
		#[serde(rename = "CONNECTED")]
		Connected,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&CmState> for CmState {
		fn from(value: &CmState) -> Self {
			value.clone()
		}
	}

	impl ToString for CmState {
		fn to_string(&self) -> String {
			match *self {
				Self::Idle => "IDLE".to_string(),
				Self::Connected => "CONNECTED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for CmState {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"IDLE" => Ok(Self::Idle),
				"CONNECTED" => Ok(Self::Connected),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for CmState {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for CmState {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for CmState {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Contains the codeword
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the codeword",
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
	pub struct CodeWord(pub String);

	impl ::std::ops::Deref for CodeWord {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<CodeWord> for String {
		fn from(value: CodeWord) -> Self {
			value.0
		}
	}

	impl From<&CodeWord> for CodeWord {
		fn from(value: &CodeWord) -> Self {
			value.clone()
		}
	}

	impl From<String> for CodeWord {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for CodeWord {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for CodeWord {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// Contains the configured S-NSSAI(s) authorized by the NSSF in the serving
	/// PLMN and optional mapped home S-NSSAI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the configured S-NSSAI(s) authorized by the
	/// NSSF in the serving PLMN and optional mapped home S-NSSAI",
	///  "type": "object",
	///  "required": [
	///    "configuredSnssai"
	///  ],
	///  "properties": {
	///    "configuredSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "mappedHomeSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ConfiguredSnssai {
		#[serde(rename = "configuredSnssai")]
		pub configured_snssai: Snssai,
		#[serde(
			rename = "mappedHomeSnssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mapped_home_snssai: Option<Snssai>,
	}

	impl From<&ConfiguredSnssai> for ConfiguredSnssai {
		fn from(value: &ConfiguredSnssai) -> Self {
			value.clone()
		}
	}

	/// Represents the congestion information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the congestion information.",
	///  "type": "object",
	///  "required": [
	///    "congType",
	///    "nsi",
	///    "timeIntev"
	///  ],
	///  "properties": {
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "congType": {
	///      "$ref": "#/components/schemas/CongestionType"
	///    },
	///    "nsi": {
	///      "$ref": "#/components/schemas/ThresholdLevel"
	///    },
	///    "timeIntev": {
	///      "$ref": "#/components/schemas/TimeWindow"
	///    },
	///    "topAppListDl": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TopApplication"
	///      },
	///      "minItems": 1
	///    },
	///    "topAppListUl": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/TopApplication"
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
	pub struct CongestionInfo {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub confidence: Option<Uinteger>,
		#[serde(rename = "congType")]
		pub cong_type: CongestionType,
		pub nsi: ThresholdLevel,
		#[serde(rename = "timeIntev")]
		pub time_intev: TimeWindow,
		#[serde(
			rename = "topAppListDl",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub top_app_list_dl: Vec<TopApplication>,
		#[serde(
			rename = "topAppListUl",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub top_app_list_ul: Vec<TopApplication>,
	}

	impl From<&CongestionInfo> for CongestionInfo {
		fn from(value: &CongestionInfo) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - USER_PLANE: The congestion analytics type is User Plane.
	/// - CONTROL_PLANE: The congestion analytics type is Control Plane.
	/// - USER_AND_CONTROL_PLANE: The congestion analytics type is User Plane
	///   and Control Plane.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- USER_PLANE: The congestion
	/// analytics type is User Plane.\n- CONTROL_PLANE: The congestion analytics
	/// type is Control Plane.\n- USER_AND_CONTROL_PLANE: The congestion
	/// analytics type is User Plane and Control Plane.\n",
	///  "type": "string",
	///  "enum": [
	///    "USER_PLANE",
	///    "CONTROL_PLANE",
	///    "USER_AND_CONTROL_PLANE"
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
	pub enum CongestionType {
		#[default]
		#[serde(rename = "USER_PLANE")]
		UserPlane,
		#[serde(rename = "CONTROL_PLANE")]
		ControlPlane,
		#[serde(rename = "USER_AND_CONTROL_PLANE")]
		UserAndControlPlane,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&CongestionType> for CongestionType {
		fn from(value: &CongestionType) -> Self {
			value.clone()
		}
	}

	impl ToString for CongestionType {
		fn to_string(&self) -> String {
			match *self {
				Self::UserPlane => "USER_PLANE".to_string(),
				Self::ControlPlane => "CONTROL_PLANE".to_string(),
				Self::UserAndControlPlane => "USER_AND_CONTROL_PLANE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for CongestionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"USER_PLANE" => Ok(Self::UserPlane),
				"CONTROL_PLANE" => Ok(Self::ControlPlane),
				"USER_AND_CONTROL_PLANE" => Ok(Self::UserAndControlPlane),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for CongestionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for CongestionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for CongestionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the analytics consumer NF Information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the analytics consumer NF Information.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "oneOf": [
	///        {
	///          "required": [
	///            "nfId"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "nfSetId"
	///          ]
	///        }
	///      ]
	///    },
	///    {
	///      "required": [
	///        "taiList"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "nfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
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
	#[serde(untagged)]
	pub enum ConsumerNfInformation {
		#[default]
		Variant0(ConsumerNfInformationVariant0),
		Variant1 {
			#[serde(rename = "nfId", default, skip_serializing_if = "Option::is_none")]
			nf_id: Option<NfInstanceId>,
			#[serde(rename = "nfSetId", default, skip_serializing_if = "Option::is_none")]
			nf_set_id: Option<NfSetId>,
			#[serde(rename = "taiList")]
			tai_list: Vec<Tai>,
		},
	}

	impl From<&ConsumerNfInformation> for ConsumerNfInformation {
		fn from(value: &ConsumerNfInformation) -> Self {
			value.clone()
		}
	}

	impl From<ConsumerNfInformationVariant0> for ConsumerNfInformation {
		fn from(value: ConsumerNfInformationVariant0) -> Self {
			Self::Variant0(value)
		}
	}

	/// ConsumerNfInformationVariant0
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "type": "object",
	///      "properties": {
	///        "nfId": {
	///          "$ref": "#/components/schemas/NfInstanceId"
	///        },
	///        "nfSetId": {
	///          "$ref": "#/components/schemas/NfSetId"
	///        },
	///        "taiList": {
	///          "type": "array",
	///          "items": {
	///            "$ref": "#/components/schemas/Tai"
	///          },
	///          "minItems": 1
	///        }
	///      }
	///    },
	///    {
	///      "oneOf": [
	///        {
	///          "required": [
	///            "nfId"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "nfSetId"
	///          ]
	///        }
	///      ]
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "taiList"
	///        ]
	///      }
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum ConsumerNfInformationVariant0 {
		#[default]
		Variant0 {
			#[serde(rename = "nfId")]
			nf_id: NfInstanceId,
			#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
			tai_list: Vec<Tai>,
		},
		Variant1 {
			#[serde(rename = "nfSetId")]
			nf_set_id: NfSetId,
			#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
			tai_list: Vec<Tai>,
		},
	}

	impl From<&ConsumerNfInformationVariant0> for ConsumerNfInformationVariant0 {
		fn from(value: &ConsumerNfInformationVariant0) -> Self {
			value.clone()
		}
	}

	/// LCS Correlation ID.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "LCS Correlation ID.",
	///  "type": "string",
	///  "maxLength": 255,
	///  "minLength": 1
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub struct CorrelationId(String);

	impl ::std::ops::Deref for CorrelationId {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<CorrelationId> for String {
		fn from(value: CorrelationId) -> Self {
			value.0
		}
	}

	impl From<&CorrelationId> for CorrelationId {
		fn from(value: &CorrelationId) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for CorrelationId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if value.len() > 255usize {
				return Err("longer than 255 characters".into());
			}
			if value.len() < 1usize {
				return Err("shorter than 1 characters".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for CorrelationId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for CorrelationId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for CorrelationId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for CorrelationId {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
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
	/// - UNIFORM_DIST_DATA: Indicates the use of data samples that are
	///   uniformly distributed according to the different aspects of the
	///   requested analytics.
	/// - NO_OUTLIERS: Indicates that the data samples shall disregard data
	///   samples that are at the extreme boundaries of the value range.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- UNIFORM_DIST_DATA: Indicates
	/// the use of data samples that are uniformly distributed according to the
	/// different aspects of the requested analytics.\n- NO_OUTLIERS: Indicates
	/// that the data samples shall disregard data samples that are at the
	/// extreme boundaries of the value range.\n",
	///  "type": "string",
	///  "enum": [
	///    "UNIFORM_DIST_DATA",
	///    "NO_OUTLIERS"
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
	pub enum DatasetStatisticalProperty {
		#[default]
		#[serde(rename = "UNIFORM_DIST_DATA")]
		UniformDistData,
		#[serde(rename = "NO_OUTLIERS")]
		NoOutliers,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DatasetStatisticalProperty> for DatasetStatisticalProperty {
		fn from(value: &DatasetStatisticalProperty) -> Self {
			value.clone()
		}
	}

	impl ToString for DatasetStatisticalProperty {
		fn to_string(&self) -> String {
			match *self {
				Self::UniformDistData => "UNIFORM_DIST_DATA".to_string(),
				Self::NoOutliers => "NO_OUTLIERS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DatasetStatisticalProperty {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UNIFORM_DIST_DATA" => Ok(Self::UniformDistData),
				"NO_OUTLIERS" => Ok(Self::NoOutliers),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DatasetStatisticalProperty {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DatasetStatisticalProperty {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DatasetStatisticalProperty {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Dispersion Area
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Dispersion Area",
	///  "type": "object",
	///  "properties": {
	///    "ecgiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ecgi"
	///      },
	///      "minItems": 1
	///    },
	///    "n3gaInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ncgiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ncgi"
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
	pub struct DispersionArea {
		#[serde(rename = "ecgiList", default, skip_serializing_if = "Vec::is_empty")]
		pub ecgi_list: Vec<Ecgi>,
		#[serde(rename = "n3gaInd", default)]
		pub n3ga_ind: bool,
		#[serde(rename = "ncgiList", default, skip_serializing_if = "Vec::is_empty")]
		pub ncgi_list: Vec<Ncgi>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
	}

	impl From<&DispersionArea> for DispersionArea {
		fn from(value: &DispersionArea) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - FIXED: Dispersion class as fixed UE its data or transaction usage at a
	///   location or a slice, is higher than its class threshold set for its
	///   all data or transaction usage.
	/// - CAMPER: Dispersion class as camper UE, its data or transaction usage
	///   at a location or a slice, is higher than its class threshold and lower
	///   than the fixed class threshold set for its all data or transaction
	///   usage..
	/// - TRAVELLER: Dispersion class as traveller UE, its data or transaction
	///   usage at a location or a slice, is lower than the camper class
	///   threshold set for its all data or transaction usage.
	/// - TOP_HEAVY: Dispersion class as Top_Heavy UE, who's dispersion
	///   percentile rating at a location or a slice, is higher than its class
	///   threshold.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- FIXED: Dispersion class as
	/// fixed UE its data or transaction usage at a location or a slice, is
	/// higher than its class threshold set for its all data or transaction
	/// usage.\n- CAMPER: Dispersion class as camper UE, its data or transaction
	/// usage at a location or a slice, is higher than its class threshold and
	/// lower than the fixed class threshold set for its all data or transaction
	/// usage..\n- TRAVELLER: Dispersion class as traveller UE, its data or
	/// transaction usage at a location or a slice, is lower than the camper
	/// class threshold set for its all data or transaction usage.\n- TOP_HEAVY:
	/// Dispersion class as Top_Heavy UE, who's dispersion percentile rating at
	/// a location or a slice, is higher than its class threshold.\n",
	///  "type": "string",
	///  "enum": [
	///    "FIXED",
	///    "CAMPER",
	///    "TRAVELLER",
	///    "TOP_HEAVY"
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
	pub enum DispersionClass {
		#[default]
		#[serde(rename = "FIXED")]
		Fixed,
		#[serde(rename = "CAMPER")]
		Camper,
		#[serde(rename = "TRAVELLER")]
		Traveller,
		#[serde(rename = "TOP_HEAVY")]
		TopHeavy,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DispersionClass> for DispersionClass {
		fn from(value: &DispersionClass) -> Self {
			value.clone()
		}
	}

	impl ToString for DispersionClass {
		fn to_string(&self) -> String {
			match *self {
				Self::Fixed => "FIXED".to_string(),
				Self::Camper => "CAMPER".to_string(),
				Self::Traveller => "TRAVELLER".to_string(),
				Self::TopHeavy => "TOP_HEAVY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DispersionClass {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"FIXED" => Ok(Self::Fixed),
				"CAMPER" => Ok(Self::Camper),
				"TRAVELLER" => Ok(Self::Traveller),
				"TOP_HEAVY" => Ok(Self::TopHeavy),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DispersionClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DispersionClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DispersionClass {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Dispersion collection per UE location or per slice.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Dispersion collection per UE location or per slice.",
	///  "type": "object",
	///  "allOf": [
	///    {
	///      "oneOf": [
	///        {
	///          "required": [
	///            "ueLoc"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "snssai"
	///          ]
	///        }
	///      ]
	///    },
	///    {
	///      "anyOf": [
	///        {
	///          "required": [
	///            "disperAmount"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "disperClass"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "usageRank"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "percentileRank"
	///          ]
	///        }
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "appVolumes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ApplicationVolume"
	///      },
	///      "minItems": 1
	///    },
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "disperAmount": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "disperClass": {
	///      "$ref": "#/components/schemas/DispersionClass"
	///    },
	///    "gpsis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "percentileRank": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "supis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "ueLoc": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "ueRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "usageRank": {
	///      "description": "Integer where the allowed values correspond to 1,
	/// 2, 3 only.",
	///      "type": "integer",
	///      "maximum": 3.0,
	///      "minimum": 1.0
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum DispersionCollection {
		#[default]
		Variant0(DispersionCollectionVariant0),
		Variant1(DispersionCollectionVariant1),
		Variant2(DispersionCollectionVariant2),
		Variant3(DispersionCollectionVariant3),
	}

	impl From<&DispersionCollection> for DispersionCollection {
		fn from(value: &DispersionCollection) -> Self {
			value.clone()
		}
	}

	impl From<DispersionCollectionVariant0> for DispersionCollection {
		fn from(value: DispersionCollectionVariant0) -> Self {
			Self::Variant0(value)
		}
	}

	impl From<DispersionCollectionVariant1> for DispersionCollection {
		fn from(value: DispersionCollectionVariant1) -> Self {
			Self::Variant1(value)
		}
	}

	impl From<DispersionCollectionVariant2> for DispersionCollection {
		fn from(value: DispersionCollectionVariant2) -> Self {
			Self::Variant2(value)
		}
	}

	impl From<DispersionCollectionVariant3> for DispersionCollection {
		fn from(value: DispersionCollectionVariant3) -> Self {
			Self::Variant3(value)
		}
	}

	/// DispersionCollectionVariant0
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "oneOf": [
	///        {
	///          "allOf": [
	///            {},
	///            {
	///              "allOf": [
	///                {
	///                  "type": "object",
	///                  "properties": {
	///                    "appVolumes": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/ApplicationVolume"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "confidence": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperAmount": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperClass": {
	///                      "$ref": "#/components/schemas/DispersionClass"
	///                    },
	///                    "gpsis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Gpsi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "percentileRank": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "snssai": {
	///                      "$ref": "#/components/schemas/Snssai"
	///                    },
	///                    "supis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Supi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "ueLoc": {
	///                      "$ref": "#/components/schemas/UserLocation"
	///                    },
	///                    "ueRatio": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "usageRank": {
	///                      "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                      "type": "integer",
	///                      "maximum": 3.0,
	///                      "minimum": 1.0
	///                    }
	///                  }
	///                },
	///                {
	///                  "required": [
	///                    "ueLoc"
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "snssai"
	///                    ]
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "type": "object",
	///                    "properties": {
	///                      "appVolumes": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref":
	/// "#/components/schemas/ApplicationVolume"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "confidence": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperAmount": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperClass": {
	///                        "$ref": "#/components/schemas/DispersionClass"
	///                      },
	///                      "gpsis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Gpsi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "percentileRank": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "snssai": {
	///                        "$ref": "#/components/schemas/Snssai"
	///                      },
	///                      "supis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Supi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "ueLoc": {
	///                        "$ref": "#/components/schemas/UserLocation"
	///                      },
	///                      "ueRatio": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "usageRank": {
	///                        "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                        "type": "integer",
	///                        "maximum": 3.0,
	///                        "minimum": 1.0
	///                      }
	///                    }
	///                  },
	///                  {
	///                    "required": [
	///                      "snssai"
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "ueLoc"
	///                      ]
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
	///                  "type": "object",
	///                  "properties": {
	///                    "appVolumes": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/ApplicationVolume"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "confidence": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperAmount": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperClass": {
	///                      "$ref": "#/components/schemas/DispersionClass"
	///                    },
	///                    "gpsis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Gpsi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "percentileRank": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "snssai": {
	///                      "$ref": "#/components/schemas/Snssai"
	///                    },
	///                    "supis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Supi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "ueLoc": {
	///                      "$ref": "#/components/schemas/UserLocation"
	///                    },
	///                    "ueRatio": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "usageRank": {
	///                      "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                      "type": "integer",
	///                      "maximum": 3.0,
	///                      "minimum": 1.0
	///                    }
	///                  }
	///                },
	///                {
	///                  "required": [
	///                    "snssai"
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "ueLoc"
	///                    ]
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "type": "object",
	///                    "properties": {
	///                      "appVolumes": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref":
	/// "#/components/schemas/ApplicationVolume"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "confidence": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperAmount": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperClass": {
	///                        "$ref": "#/components/schemas/DispersionClass"
	///                      },
	///                      "gpsis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Gpsi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "percentileRank": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "snssai": {
	///                        "$ref": "#/components/schemas/Snssai"
	///                      },
	///                      "supis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Supi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "ueLoc": {
	///                        "$ref": "#/components/schemas/UserLocation"
	///                      },
	///                      "ueRatio": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "usageRank": {
	///                        "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                        "type": "integer",
	///                        "maximum": 3.0,
	///                        "minimum": 1.0
	///                      }
	///                    }
	///                  },
	///                  {
	///                    "required": [
	///                      "ueLoc"
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "snssai"
	///                      ]
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
	///      "required": [
	///        "disperAmount"
	///      ]
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "disperClass"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "usageRank"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "percentileRank"
	///        ]
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
	pub enum DispersionCollectionVariant0 {
		#[default]
		None,
	}

	impl From<&DispersionCollectionVariant0> for DispersionCollectionVariant0 {
		fn from(value: &DispersionCollectionVariant0) -> Self {
			value.clone()
		}
	}

	/// DispersionCollectionVariant1
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "oneOf": [
	///        {
	///          "allOf": [
	///            {},
	///            {
	///              "allOf": [
	///                {
	///                  "type": "object",
	///                  "properties": {
	///                    "appVolumes": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/ApplicationVolume"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "confidence": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperAmount": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperClass": {
	///                      "$ref": "#/components/schemas/DispersionClass"
	///                    },
	///                    "gpsis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Gpsi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "percentileRank": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "snssai": {
	///                      "$ref": "#/components/schemas/Snssai"
	///                    },
	///                    "supis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Supi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "ueLoc": {
	///                      "$ref": "#/components/schemas/UserLocation"
	///                    },
	///                    "ueRatio": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "usageRank": {
	///                      "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                      "type": "integer",
	///                      "maximum": 3.0,
	///                      "minimum": 1.0
	///                    }
	///                  }
	///                },
	///                {
	///                  "required": [
	///                    "ueLoc"
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "snssai"
	///                    ]
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "type": "object",
	///                    "properties": {
	///                      "appVolumes": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref":
	/// "#/components/schemas/ApplicationVolume"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "confidence": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperAmount": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperClass": {
	///                        "$ref": "#/components/schemas/DispersionClass"
	///                      },
	///                      "gpsis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Gpsi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "percentileRank": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "snssai": {
	///                        "$ref": "#/components/schemas/Snssai"
	///                      },
	///                      "supis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Supi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "ueLoc": {
	///                        "$ref": "#/components/schemas/UserLocation"
	///                      },
	///                      "ueRatio": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "usageRank": {
	///                        "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                        "type": "integer",
	///                        "maximum": 3.0,
	///                        "minimum": 1.0
	///                      }
	///                    }
	///                  },
	///                  {
	///                    "required": [
	///                      "snssai"
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "ueLoc"
	///                      ]
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
	///                  "type": "object",
	///                  "properties": {
	///                    "appVolumes": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/ApplicationVolume"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "confidence": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperAmount": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperClass": {
	///                      "$ref": "#/components/schemas/DispersionClass"
	///                    },
	///                    "gpsis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Gpsi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "percentileRank": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "snssai": {
	///                      "$ref": "#/components/schemas/Snssai"
	///                    },
	///                    "supis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Supi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "ueLoc": {
	///                      "$ref": "#/components/schemas/UserLocation"
	///                    },
	///                    "ueRatio": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "usageRank": {
	///                      "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                      "type": "integer",
	///                      "maximum": 3.0,
	///                      "minimum": 1.0
	///                    }
	///                  }
	///                },
	///                {
	///                  "required": [
	///                    "snssai"
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "ueLoc"
	///                    ]
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "type": "object",
	///                    "properties": {
	///                      "appVolumes": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref":
	/// "#/components/schemas/ApplicationVolume"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "confidence": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperAmount": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperClass": {
	///                        "$ref": "#/components/schemas/DispersionClass"
	///                      },
	///                      "gpsis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Gpsi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "percentileRank": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "snssai": {
	///                        "$ref": "#/components/schemas/Snssai"
	///                      },
	///                      "supis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Supi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "ueLoc": {
	///                        "$ref": "#/components/schemas/UserLocation"
	///                      },
	///                      "ueRatio": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "usageRank": {
	///                        "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                        "type": "integer",
	///                        "maximum": 3.0,
	///                        "minimum": 1.0
	///                      }
	///                    }
	///                  },
	///                  {
	///                    "required": [
	///                      "ueLoc"
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "snssai"
	///                      ]
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
	///      "required": [
	///        "disperClass"
	///      ]
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "disperAmount"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "usageRank"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "percentileRank"
	///        ]
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
	pub enum DispersionCollectionVariant1 {
		#[default]
		None,
	}

	impl From<&DispersionCollectionVariant1> for DispersionCollectionVariant1 {
		fn from(value: &DispersionCollectionVariant1) -> Self {
			value.clone()
		}
	}

	/// DispersionCollectionVariant2
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "oneOf": [
	///        {
	///          "allOf": [
	///            {},
	///            {
	///              "allOf": [
	///                {
	///                  "type": "object",
	///                  "properties": {
	///                    "appVolumes": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/ApplicationVolume"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "confidence": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperAmount": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperClass": {
	///                      "$ref": "#/components/schemas/DispersionClass"
	///                    },
	///                    "gpsis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Gpsi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "percentileRank": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "snssai": {
	///                      "$ref": "#/components/schemas/Snssai"
	///                    },
	///                    "supis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Supi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "ueLoc": {
	///                      "$ref": "#/components/schemas/UserLocation"
	///                    },
	///                    "ueRatio": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "usageRank": {
	///                      "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                      "type": "integer",
	///                      "maximum": 3.0,
	///                      "minimum": 1.0
	///                    }
	///                  }
	///                },
	///                {
	///                  "required": [
	///                    "ueLoc"
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "snssai"
	///                    ]
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "type": "object",
	///                    "properties": {
	///                      "appVolumes": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref":
	/// "#/components/schemas/ApplicationVolume"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "confidence": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperAmount": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperClass": {
	///                        "$ref": "#/components/schemas/DispersionClass"
	///                      },
	///                      "gpsis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Gpsi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "percentileRank": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "snssai": {
	///                        "$ref": "#/components/schemas/Snssai"
	///                      },
	///                      "supis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Supi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "ueLoc": {
	///                        "$ref": "#/components/schemas/UserLocation"
	///                      },
	///                      "ueRatio": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "usageRank": {
	///                        "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                        "type": "integer",
	///                        "maximum": 3.0,
	///                        "minimum": 1.0
	///                      }
	///                    }
	///                  },
	///                  {
	///                    "required": [
	///                      "snssai"
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "ueLoc"
	///                      ]
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
	///                  "type": "object",
	///                  "properties": {
	///                    "appVolumes": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/ApplicationVolume"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "confidence": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperAmount": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperClass": {
	///                      "$ref": "#/components/schemas/DispersionClass"
	///                    },
	///                    "gpsis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Gpsi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "percentileRank": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "snssai": {
	///                      "$ref": "#/components/schemas/Snssai"
	///                    },
	///                    "supis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Supi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "ueLoc": {
	///                      "$ref": "#/components/schemas/UserLocation"
	///                    },
	///                    "ueRatio": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "usageRank": {
	///                      "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                      "type": "integer",
	///                      "maximum": 3.0,
	///                      "minimum": 1.0
	///                    }
	///                  }
	///                },
	///                {
	///                  "required": [
	///                    "snssai"
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "ueLoc"
	///                    ]
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "type": "object",
	///                    "properties": {
	///                      "appVolumes": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref":
	/// "#/components/schemas/ApplicationVolume"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "confidence": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperAmount": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperClass": {
	///                        "$ref": "#/components/schemas/DispersionClass"
	///                      },
	///                      "gpsis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Gpsi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "percentileRank": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "snssai": {
	///                        "$ref": "#/components/schemas/Snssai"
	///                      },
	///                      "supis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Supi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "ueLoc": {
	///                        "$ref": "#/components/schemas/UserLocation"
	///                      },
	///                      "ueRatio": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "usageRank": {
	///                        "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                        "type": "integer",
	///                        "maximum": 3.0,
	///                        "minimum": 1.0
	///                      }
	///                    }
	///                  },
	///                  {
	///                    "required": [
	///                      "ueLoc"
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "snssai"
	///                      ]
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
	///      "required": [
	///        "usageRank"
	///      ]
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "disperAmount"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "disperClass"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "percentileRank"
	///        ]
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
	pub enum DispersionCollectionVariant2 {
		#[default]
		None,
	}

	impl From<&DispersionCollectionVariant2> for DispersionCollectionVariant2 {
		fn from(value: &DispersionCollectionVariant2) -> Self {
			value.clone()
		}
	}

	/// DispersionCollectionVariant3
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "allOf": [
	///    {
	///      "oneOf": [
	///        {
	///          "allOf": [
	///            {},
	///            {
	///              "allOf": [
	///                {
	///                  "type": "object",
	///                  "properties": {
	///                    "appVolumes": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/ApplicationVolume"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "confidence": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperAmount": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperClass": {
	///                      "$ref": "#/components/schemas/DispersionClass"
	///                    },
	///                    "gpsis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Gpsi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "percentileRank": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "snssai": {
	///                      "$ref": "#/components/schemas/Snssai"
	///                    },
	///                    "supis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Supi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "ueLoc": {
	///                      "$ref": "#/components/schemas/UserLocation"
	///                    },
	///                    "ueRatio": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "usageRank": {
	///                      "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                      "type": "integer",
	///                      "maximum": 3.0,
	///                      "minimum": 1.0
	///                    }
	///                  }
	///                },
	///                {
	///                  "required": [
	///                    "ueLoc"
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "snssai"
	///                    ]
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "type": "object",
	///                    "properties": {
	///                      "appVolumes": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref":
	/// "#/components/schemas/ApplicationVolume"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "confidence": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperAmount": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperClass": {
	///                        "$ref": "#/components/schemas/DispersionClass"
	///                      },
	///                      "gpsis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Gpsi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "percentileRank": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "snssai": {
	///                        "$ref": "#/components/schemas/Snssai"
	///                      },
	///                      "supis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Supi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "ueLoc": {
	///                        "$ref": "#/components/schemas/UserLocation"
	///                      },
	///                      "ueRatio": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "usageRank": {
	///                        "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                        "type": "integer",
	///                        "maximum": 3.0,
	///                        "minimum": 1.0
	///                      }
	///                    }
	///                  },
	///                  {
	///                    "required": [
	///                      "snssai"
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "ueLoc"
	///                      ]
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
	///                  "type": "object",
	///                  "properties": {
	///                    "appVolumes": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/ApplicationVolume"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "confidence": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperAmount": {
	///                      "$ref": "#/components/schemas/Uinteger"
	///                    },
	///                    "disperClass": {
	///                      "$ref": "#/components/schemas/DispersionClass"
	///                    },
	///                    "gpsis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Gpsi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "percentileRank": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "snssai": {
	///                      "$ref": "#/components/schemas/Snssai"
	///                    },
	///                    "supis": {
	///                      "type": "array",
	///                      "items": {
	///                        "$ref": "#/components/schemas/Supi"
	///                      },
	///                      "minItems": 1
	///                    },
	///                    "ueLoc": {
	///                      "$ref": "#/components/schemas/UserLocation"
	///                    },
	///                    "ueRatio": {
	///                      "$ref": "#/components/schemas/SamplingRatio"
	///                    },
	///                    "usageRank": {
	///                      "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                      "type": "integer",
	///                      "maximum": 3.0,
	///                      "minimum": 1.0
	///                    }
	///                  }
	///                },
	///                {
	///                  "required": [
	///                    "snssai"
	///                  ]
	///                },
	///                {
	///                  "not": {
	///                    "required": [
	///                      "ueLoc"
	///                    ]
	///                  }
	///                }
	///              ]
	///            },
	///            {
	///              "not": {
	///                "allOf": [
	///                  {
	///                    "type": "object",
	///                    "properties": {
	///                      "appVolumes": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref":
	/// "#/components/schemas/ApplicationVolume"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "confidence": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperAmount": {
	///                        "$ref": "#/components/schemas/Uinteger"
	///                      },
	///                      "disperClass": {
	///                        "$ref": "#/components/schemas/DispersionClass"
	///                      },
	///                      "gpsis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Gpsi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "percentileRank": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "snssai": {
	///                        "$ref": "#/components/schemas/Snssai"
	///                      },
	///                      "supis": {
	///                        "type": "array",
	///                        "items": {
	///                          "$ref": "#/components/schemas/Supi"
	///                        },
	///                        "minItems": 1
	///                      },
	///                      "ueLoc": {
	///                        "$ref": "#/components/schemas/UserLocation"
	///                      },
	///                      "ueRatio": {
	///                        "$ref": "#/components/schemas/SamplingRatio"
	///                      },
	///                      "usageRank": {
	///                        "description": "Integer where the allowed values
	/// correspond to 1, 2, 3 only.",
	///                        "type": "integer",
	///                        "maximum": 3.0,
	///                        "minimum": 1.0
	///                      }
	///                    }
	///                  },
	///                  {
	///                    "required": [
	///                      "ueLoc"
	///                    ]
	///                  },
	///                  {
	///                    "not": {
	///                      "required": [
	///                        "snssai"
	///                      ]
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
	///      "required": [
	///        "percentileRank"
	///      ]
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "disperAmount"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "disperClass"
	///        ]
	///      }
	///    },
	///    {
	///      "not": {
	///        "required": [
	///          "usageRank"
	///        ]
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
	pub enum DispersionCollectionVariant3 {
		#[default]
		None,
	}

	impl From<&DispersionCollectionVariant3> for DispersionCollectionVariant3 {
		fn from(value: &DispersionCollectionVariant3) -> Self {
			value.clone()
		}
	}

	/// Represents the Dispersion information. When subscribed event is
	/// "DISPERSION", the "disperInfos" attribute shall be included.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the Dispersion information. When subscribed
	/// event is \"DISPERSION\", the \"disperInfos\" attribute shall be
	/// included.\n",
	///  "type": "object",
	///  "required": [
	///    "disperCollects",
	///    "disperType",
	///    "tsDuration",
	///    "tsStart"
	///  ],
	///  "properties": {
	///    "disperCollects": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DispersionCollection"
	///      },
	///      "minItems": 1
	///    },
	///    "disperType": {
	///      "$ref": "#/components/schemas/DispersionType"
	///    },
	///    "tsDuration": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "tsStart": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct DispersionInfo {
		#[serde(rename = "disperCollects")]
		pub disper_collects: Vec<DispersionCollection>,
		#[serde(rename = "disperType")]
		pub disper_type: DispersionType,
		#[serde(rename = "tsDuration")]
		pub ts_duration: DurationSec,
		#[serde(rename = "tsStart")]
		pub ts_start: DateTime,
	}

	impl From<&DispersionInfo> for DispersionInfo {
		fn from(value: &DispersionInfo) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - TIME_SLOT_START: Indicates the order of time slot start.
	/// - DISPERSION: Indicates the order of data/transaction dispersion.
	/// - CLASSIFICATION: Indicates the order of data/transaction
	///   classification.
	/// - RANKING: Indicates the order of data/transaction ranking.
	/// - PERCENTILE_RANKING: Indicates the order of data/transaction percentile
	///   ranking.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- TIME_SLOT_START: Indicates the
	/// order of time slot start.\n- DISPERSION: Indicates the order of
	/// data/transaction dispersion.\n- CLASSIFICATION: Indicates the order of
	/// data/transaction classification.\n- RANKING: Indicates the order of
	/// data/transaction ranking.\n- PERCENTILE_RANKING: Indicates the order of
	/// data/transaction percentile ranking.\n",
	///  "type": "string",
	///  "enum": [
	///    "TIME_SLOT_START",
	///    "DISPERSION",
	///    "CLASSIFICATION",
	///    "RANKING",
	///    "PERCENTILE_RANKING"
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
	pub enum DispersionOrderingCriterion {
		#[default]
		#[serde(rename = "TIME_SLOT_START")]
		TimeSlotStart,
		#[serde(rename = "DISPERSION")]
		Dispersion,
		#[serde(rename = "CLASSIFICATION")]
		Classification,
		#[serde(rename = "RANKING")]
		Ranking,
		#[serde(rename = "PERCENTILE_RANKING")]
		PercentileRanking,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DispersionOrderingCriterion> for DispersionOrderingCriterion {
		fn from(value: &DispersionOrderingCriterion) -> Self {
			value.clone()
		}
	}

	impl ToString for DispersionOrderingCriterion {
		fn to_string(&self) -> String {
			match *self {
				Self::TimeSlotStart => "TIME_SLOT_START".to_string(),
				Self::Dispersion => "DISPERSION".to_string(),
				Self::Classification => "CLASSIFICATION".to_string(),
				Self::Ranking => "RANKING".to_string(),
				Self::PercentileRanking => "PERCENTILE_RANKING".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DispersionOrderingCriterion {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TIME_SLOT_START" => Ok(Self::TimeSlotStart),
				"DISPERSION" => Ok(Self::Dispersion),
				"CLASSIFICATION" => Ok(Self::Classification),
				"RANKING" => Ok(Self::Ranking),
				"PERCENTILE_RANKING" => Ok(Self::PercentileRanking),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DispersionOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DispersionOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DispersionOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the dispersion analytics requirements.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the dispersion analytics requirements.",
	///  "type": "object",
	///  "required": [
	///    "disperType"
	///  ],
	///  "properties": {
	///    "classCriters": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ClassCriterion"
	///      },
	///      "minItems": 1
	///    },
	///    "dispOrderCriter": {
	///      "$ref": "#/components/schemas/DispersionOrderingCriterion"
	///    },
	///    "disperType": {
	///      "$ref": "#/components/schemas/DispersionType"
	///    },
	///    "order": {
	///      "$ref": "#/components/schemas/MatchingDirection"
	///    },
	///    "rankCriters": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RankingCriterion"
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
	pub struct DispersionRequirement {
		#[serde(
			rename = "classCriters",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub class_criters: Vec<ClassCriterion>,
		#[serde(
			rename = "dispOrderCriter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub disp_order_criter: Option<DispersionOrderingCriterion>,
		#[serde(rename = "disperType")]
		pub disper_type: DispersionType,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub order: Option<MatchingDirection>,
		#[serde(rename = "rankCriters", default, skip_serializing_if = "Vec::is_empty")]
		pub rank_criters: Vec<RankingCriterion>,
	}

	impl From<&DispersionRequirement> for DispersionRequirement {
		fn from(value: &DispersionRequirement) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	///  - DVDA: Data Volume Dispersion Analytics.
	///  - TDA: Transactions Dispersion Analytics.
	///  - DVDA_AND_TDA: Data Volume Dispersion Analytics and Transactions
	///    Dispersion Analytics.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n  - DVDA: Data Volume Dispersion
	/// Analytics.\n  - TDA: Transactions Dispersion Analytics.\n  -
	/// DVDA_AND_TDA: Data Volume Dispersion Analytics and Transactions
	/// Dispersion Analytics.\n",
	///  "type": "string",
	///  "enum": [
	///    "DVDA",
	///    "TDA",
	///    "DVDA_AND_TDA"
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
	pub enum DispersionType {
		#[default]
		#[serde(rename = "DVDA")]
		Dvda,
		#[serde(rename = "TDA")]
		Tda,
		#[serde(rename = "DVDA_AND_TDA")]
		DvdaAndTda,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DispersionType> for DispersionType {
		fn from(value: &DispersionType) -> Self {
			value.clone()
		}
	}

	impl ToString for DispersionType {
		fn to_string(&self) -> String {
			match *self {
				Self::Dvda => "DVDA".to_string(),
				Self::Tda => "TDA".to_string(),
				Self::DvdaAndTda => "DVDA_AND_TDA".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DispersionType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DVDA" => Ok(Self::Dvda),
				"TDA" => Ok(Self::Tda),
				"DVDA_AND_TDA" => Ok(Self::DvdaAndTda),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DispersionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DispersionType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DispersionType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents DN performance for the application.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents DN performance for the application.",
	///  "type": "object",
	///  "required": [
	///    "perfData"
	///  ],
	///  "properties": {
	///    "appServerInsAddr": {
	///      "$ref": "#/components/schemas/AddrFqdn"
	///    },
	///    "dnai": {
	///      "$ref": "#/components/schemas/Dnai"
	///    },
	///    "perfData": {
	///      "$ref": "#/components/schemas/PerfData"
	///    },
	///    "spatialValidCon": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "temporalValidCon": {
	///      "$ref": "#/components/schemas/TimeWindow"
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
	pub struct DnPerf {
		#[serde(
			rename = "appServerInsAddr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub app_server_ins_addr: Option<AddrFqdn>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnai: Option<Dnai>,
		#[serde(rename = "perfData")]
		pub perf_data: PerfData,
		#[serde(
			rename = "spatialValidCon",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub spatial_valid_con: Option<NetworkAreaInfo>,
		#[serde(
			rename = "temporalValidCon",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub temporal_valid_con: Option<TimeWindow>,
		#[serde(rename = "upfInfo", default, skip_serializing_if = "Option::is_none")]
		pub upf_info: Option<UpfInformation>,
	}

	impl From<&DnPerf> for DnPerf {
		fn from(value: &DnPerf) -> Self {
			value.clone()
		}
	}

	/// Represents DN performance information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents DN performance information.",
	///  "type": "object",
	///  "required": [
	///    "dnPerf"
	///  ],
	///  "properties": {
	///    "appId": {
	///      "$ref": "#/components/schemas/ApplicationId"
	///    },
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "dnPerf": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DnPerf"
	///      },
	///      "minItems": 1
	///    },
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
	pub struct DnPerfInfo {
		#[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
		pub app_id: Option<ApplicationId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub confidence: Option<Uinteger>,
		#[serde(rename = "dnPerf")]
		pub dn_perf: Vec<DnPerf>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
	}

	impl From<&DnPerfInfo> for DnPerfInfo {
		fn from(value: &DnPerfInfo) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - AVERAGE_TRAFFIC_RATE: Indicates the average traffic rate.
	/// - MAXIMUM_TRAFFIC_RATE: Indicates the maximum traffic rate.
	/// - AVERAGE_PACKET_DELAY: Indicates the average packet delay.
	/// - MAXIMUM_PACKET_DELAY: Indicates the maximum packet delay.
	/// - AVERAGE_PACKET_LOSS_RATE: Indicates the average packet loss rate.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- AVERAGE_TRAFFIC_RATE: Indicates the average traffic rate.\n- MAXIMUM_TRAFFIC_RATE: Indicates the maximum traffic rate.\n- AVERAGE_PACKET_DELAY: Indicates the average packet delay.\n- MAXIMUM_PACKET_DELAY: Indicates the maximum packet delay.\n- AVERAGE_PACKET_LOSS_RATE: Indicates the average packet loss rate.\n",
	///  "type": "string",
	///  "enum": [
	///    "AVERAGE_TRAFFIC_RATE",
	///    "MAXIMUM_TRAFFIC_RATE",
	///    "AVERAGE_PACKET_DELAY",
	///    "MAXIMUM_PACKET_DELAY",
	///    "AVERAGE_PACKET_LOSS_RATE"
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
	pub enum DnPerfOrderingCriterion {
		#[default]
		#[serde(rename = "AVERAGE_TRAFFIC_RATE")]
		AverageTrafficRate,
		#[serde(rename = "MAXIMUM_TRAFFIC_RATE")]
		MaximumTrafficRate,
		#[serde(rename = "AVERAGE_PACKET_DELAY")]
		AveragePacketDelay,
		#[serde(rename = "MAXIMUM_PACKET_DELAY")]
		MaximumPacketDelay,
		#[serde(rename = "AVERAGE_PACKET_LOSS_RATE")]
		AveragePacketLossRate,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&DnPerfOrderingCriterion> for DnPerfOrderingCriterion {
		fn from(value: &DnPerfOrderingCriterion) -> Self {
			value.clone()
		}
	}

	impl ToString for DnPerfOrderingCriterion {
		fn to_string(&self) -> String {
			match *self {
				Self::AverageTrafficRate => "AVERAGE_TRAFFIC_RATE".to_string(),
				Self::MaximumTrafficRate => "MAXIMUM_TRAFFIC_RATE".to_string(),
				Self::AveragePacketDelay => "AVERAGE_PACKET_DELAY".to_string(),
				Self::MaximumPacketDelay => "MAXIMUM_PACKET_DELAY".to_string(),
				Self::AveragePacketLossRate => "AVERAGE_PACKET_LOSS_RATE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for DnPerfOrderingCriterion {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AVERAGE_TRAFFIC_RATE" => Ok(Self::AverageTrafficRate),
				"MAXIMUM_TRAFFIC_RATE" => Ok(Self::MaximumTrafficRate),
				"AVERAGE_PACKET_DELAY" => Ok(Self::AveragePacketDelay),
				"MAXIMUM_PACKET_DELAY" => Ok(Self::MaximumPacketDelay),
				"AVERAGE_PACKET_LOSS_RATE" => Ok(Self::AveragePacketLossRate),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for DnPerfOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DnPerfOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DnPerfOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents other DN performance analytics requirements.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents other DN performance analytics
	/// requirements.",
	///  "type": "object",
	///  "properties": {
	///    "dnPerfOrderCriter": {
	///      "$ref": "#/components/schemas/DnPerfOrderingCriterion"
	///    },
	///    "order": {
	///      "$ref": "#/components/schemas/MatchingDirection"
	///    },
	///    "reportThresholds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ThresholdLevel"
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
	pub struct DnPerformanceReq {
		#[serde(
			rename = "dnPerfOrderCriter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dn_perf_order_criter: Option<DnPerfOrderingCriterion>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub order: Option<MatchingDirection>,
		#[serde(
			rename = "reportThresholds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub report_thresholds: Vec<ThresholdLevel>,
	}

	impl From<&DnPerformanceReq> for DnPerformanceReq {
		fn from(value: &DnPerformanceReq) -> Self {
			value.clone()
		}
	}

	/// DrxParameter
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
	pub struct DrxParameter(pub Bytes);

	impl ::std::ops::Deref for DrxParameter {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<DrxParameter> for Bytes {
		fn from(value: DrxParameter) -> Self {
			value.0
		}
	}

	impl From<&DrxParameter> for DrxParameter {
		fn from(value: &DrxParameter) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for DrxParameter {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for DrxParameter {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for DrxParameter {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for DrxParameter {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for DrxParameter {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for DrxParameter {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// E164Number
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "string",
	///  "pattern": "^[0-9]{1,15}$"
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub struct E164Number(String);

	impl ::std::ops::Deref for E164Number {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<E164Number> for String {
		fn from(value: E164Number) -> Self {
			value.0
		}
	}

	impl From<&E164Number> for E164Number {
		fn from(value: &E164Number) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for E164Number {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if regress::Regex::new("^[0-9]{1,15}$")
				.unwrap()
				.find(value)
				.is_none()
			{
				return Err("doesn't match pattern \"^[0-9]{1,15}$\"".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for E164Number {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for E164Number {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for E164Number {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for E164Number {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
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

	/// Enhanced Coverage Restriction Data for WB-N1 mode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Enhanced Coverage Restriction Data for WB-N1 mode",
	///  "type": "object",
	///  "required": [
	///    "ecModeBRestricted"
	///  ],
	///  "properties": {
	///    "ecModeARestricted": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ecModeBRestricted": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EcRestrictionDataWb {
		#[serde(rename = "ecModeARestricted", default)]
		pub ec_mode_a_restricted: bool,
		#[serde(rename = "ecModeBRestricted")]
		pub ec_mode_b_restricted: bool,
	}

	impl From<&EcRestrictionDataWb> for EcRestrictionDataWb {
		fn from(value: &EcRestrictionDataWb) -> Self {
			value.clone()
		}
	}

	/// Data within the Enable Group Reachability Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within the Enable Group Reachability Request",
	///  "type": "object",
	///  "required": [
	///    "tmgi",
	///    "ueInfoList"
	///  ],
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "arp": {
	///      "$ref": "#/components/schemas/Arp"
	///    },
	///    "mbsServiceAreaInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MbsServiceAreaInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "reachabilityNotifyUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "tmgi": {
	///      "$ref": "#/components/schemas/Tmgi"
	///    },
	///    "ueInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeInfo"
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
	pub struct EnableGroupReachabilityReqData {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub arp: Option<Arp>,
		#[serde(rename = "5qi", default, skip_serializing_if = "Option::is_none")]
		pub five_qi: Option<_5qi>,
		#[serde(
			rename = "mbsServiceAreaInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mbs_service_area_info_list: Vec<MbsServiceAreaInfo>,
		#[serde(
			rename = "reachabilityNotifyUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reachability_notify_uri: Option<Uri>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		pub tmgi: Tmgi,
		#[serde(rename = "ueInfoList")]
		pub ue_info_list: Vec<UeInfo>,
	}

	impl From<&EnableGroupReachabilityReqData> for EnableGroupReachabilityReqData {
		fn from(value: &EnableGroupReachabilityReqData) -> Self {
			value.clone()
		}
	}

	/// Data within the Enable Group Reachability Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within the Enable Group Reachability Response",
	///  "type": "object",
	///  "properties": {
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "ueConnectedList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
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
	pub struct EnableGroupReachabilityRspData {
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "ueConnectedList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ue_connected_list: Vec<Supi>,
	}

	impl From<&EnableGroupReachabilityRspData> for EnableGroupReachabilityRspData {
		fn from(value: &EnableGroupReachabilityRspData) -> Self {
			value.clone()
		}
	}

	/// Data within the Enable UE Reachability Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within the Enable UE Reachability Request",
	///  "type": "object",
	///  "required": [
	///    "reachability"
	///  ],
	///  "properties": {
	///    "extBufSupport": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "oldGuami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "reachability": {
	///      "$ref": "#/components/schemas/UeReachability"
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
	pub struct EnableUeReachabilityReqData {
		#[serde(rename = "extBufSupport", default)]
		pub ext_buf_support: bool,
		#[serde(rename = "oldGuami", default, skip_serializing_if = "Option::is_none")]
		pub old_guami: Option<crate::common::common_models::Guami>,
		pub reachability: UeReachability,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&EnableUeReachabilityReqData> for EnableUeReachabilityReqData {
		fn from(value: &EnableUeReachabilityReqData) -> Self {
			value.clone()
		}
	}

	/// Data within the Enable UE Reachability Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within the Enable UE Reachability Response",
	///  "type": "object",
	///  "required": [
	///    "reachability"
	///  ],
	///  "properties": {
	///    "reachability": {
	///      "$ref": "#/components/schemas/UeReachability"
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
	pub struct EnableUeReachabilityRspData {
		pub reachability: UeReachability,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&EnableUeReachabilityRspData> for EnableUeReachabilityRspData {
		fn from(value: &EnableUeReachabilityRspData) -> Self {
			value.clone()
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

	/// EpsInterworkingInfo
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "epsIwkPgws": {
	///      "description": "A map (list of key-value pairs where Dnn serves as
	/// key) of EpsIwkPgws",
	///      "type": "object",
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/EpsIwkPgw"
	///      }
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EpsInterworkingInfo {
		/// A map (list of key-value pairs where Dnn serves as key) of
		/// EpsIwkPgws
		#[serde(
			rename = "epsIwkPgws",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub eps_iwk_pgws: ::std::collections::HashMap<String, EpsIwkPgw>,
	}

	impl From<&EpsInterworkingInfo> for EpsInterworkingInfo {
		fn from(value: &EpsInterworkingInfo) -> Self {
			value.clone()
		}
	}

	/// EpsIwkPgw
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "pgwFqdn",
	///    "smfInstanceId"
	///  ],
	///  "properties": {
	///    "pgwFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "smfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EpsIwkPgw {
		#[serde(rename = "pgwFqdn")]
		pub pgw_fqdn: Fqdn,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnId>,
		#[serde(rename = "smfInstanceId")]
		pub smf_instance_id: NfInstanceId,
	}

	impl From<&EpsIwkPgw> for EpsIwkPgw {
		fn from(value: &EpsIwkPgw) -> Self {
			value.clone()
		}
	}

	/// Indicates the supported EPS NAS Ciphering Algorithm
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the supported EPS NAS Ciphering Algorithm",
	///  "type": "string",
	///  "enum": [
	///    "EEA0",
	///    "EEA1",
	///    "EEA2",
	///    "EEA3"
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
	pub enum EpsNasCipheringAlgorithm {
		#[default]
		#[serde(rename = "EEA0")]
		Eea0,
		#[serde(rename = "EEA1")]
		Eea1,
		#[serde(rename = "EEA2")]
		Eea2,
		#[serde(rename = "EEA3")]
		Eea3,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&EpsNasCipheringAlgorithm> for EpsNasCipheringAlgorithm {
		fn from(value: &EpsNasCipheringAlgorithm) -> Self {
			value.clone()
		}
	}

	impl ToString for EpsNasCipheringAlgorithm {
		fn to_string(&self) -> String {
			match *self {
				Self::Eea0 => "EEA0".to_string(),
				Self::Eea1 => "EEA1".to_string(),
				Self::Eea2 => "EEA2".to_string(),
				Self::Eea3 => "EEA3".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for EpsNasCipheringAlgorithm {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EEA0" => Ok(Self::Eea0),
				"EEA1" => Ok(Self::Eea1),
				"EEA2" => Ok(Self::Eea2),
				"EEA3" => Ok(Self::Eea3),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for EpsNasCipheringAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for EpsNasCipheringAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for EpsNasCipheringAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the supported EPS NAS Integrity Algorithm
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the supported EPS NAS Integrity Algorithm",
	///  "type": "string",
	///  "enum": [
	///    "EIA0",
	///    "EIA1",
	///    "EIA2",
	///    "EIA3"
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
	pub enum EpsNasIntegrityAlgorithm {
		#[default]
		#[serde(rename = "EIA0")]
		Eia0,
		#[serde(rename = "EIA1")]
		Eia1,
		#[serde(rename = "EIA2")]
		Eia2,
		#[serde(rename = "EIA3")]
		Eia3,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&EpsNasIntegrityAlgorithm> for EpsNasIntegrityAlgorithm {
		fn from(value: &EpsNasIntegrityAlgorithm) -> Self {
			value.clone()
		}
	}

	impl ToString for EpsNasIntegrityAlgorithm {
		fn to_string(&self) -> String {
			match *self {
				Self::Eia0 => "EIA0".to_string(),
				Self::Eia1 => "EIA1".to_string(),
				Self::Eia2 => "EIA2".to_string(),
				Self::Eia3 => "EIA3".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for EpsNasIntegrityAlgorithm {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EIA0" => Ok(Self::Eia0),
				"EIA1" => Ok(Self::Eia1),
				"EIA2" => Ok(Self::Eia2),
				"EIA3" => Ok(Self::Eia3),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for EpsNasIntegrityAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for EpsNasIntegrityAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for EpsNasIntegrityAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the EPS NAS Security Mode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the EPS NAS Security Mode",
	///  "type": "object",
	///  "required": [
	///    "cipheringAlgorithm",
	///    "integrityAlgorithm"
	///  ],
	///  "properties": {
	///    "cipheringAlgorithm": {
	///      "$ref": "#/components/schemas/EpsNasCipheringAlgorithm"
	///    },
	///    "integrityAlgorithm": {
	///      "$ref": "#/components/schemas/EpsNasIntegrityAlgorithm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EpsNasSecurityMode {
		#[serde(rename = "cipheringAlgorithm")]
		pub ciphering_algorithm: EpsNasCipheringAlgorithm,
		#[serde(rename = "integrityAlgorithm")]
		pub integrity_algorithm: EpsNasIntegrityAlgorithm,
	}

	impl From<&EpsNasSecurityMode> for EpsNasSecurityMode {
		fn from(value: &EpsNasSecurityMode) -> Self {
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

	/// Represents a notification on events that occurred.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a notification on events that occurred.",
	///  "type": "object",
	///  "required": [
	///    "event"
	///  ],
	///  "properties": {
	///    "abnorBehavrs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AbnormalBehaviour"
	///      },
	///      "minItems": 1
	///    },
	///    "anaMetaInfo": {
	///      "$ref": "#/components/schemas/AnalyticsMetadataInfo"
	///    },
	///    "disperInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DispersionInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "dnPerfInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DnPerfInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "event": {
	///      "$ref": "#/components/schemas/NwdafEvent"
	///    },
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "failNotifyCode": {
	///      "$ref": "#/components/schemas/NwdafFailureCode"
	///    },
	///    "nfLoadLevelInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfLoadLevelInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "nsiLoadLevelInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsiLoadLevelInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "nwPerfs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NetworkPerfInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "qosSustainInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/QosSustainabilityInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "redTransInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RedundantTransmissionExpInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "rvWaitTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "sliceLoadLevelInfo": {
	///      "$ref": "#/components/schemas/SliceLoadLevelInformation"
	///    },
	///    "smccExps": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SmcceInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "start": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "svcExps": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ServiceExperienceInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "timeStampGen": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "ueComms": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeCommunication"
	///      },
	///      "minItems": 1
	///    },
	///    "ueMobs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeMobility"
	///      },
	///      "minItems": 1
	///    },
	///    "userDataCongInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UserDataCongestionInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "wlanInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/WlanPerformanceInfo"
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
	pub struct EventNotification {
		#[serde(
			rename = "abnorBehavrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub abnor_behavrs: Vec<AbnormalBehaviour>,
		#[serde(
			rename = "anaMetaInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ana_meta_info: Option<AnalyticsMetadataInfo>,
		#[serde(rename = "disperInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub disper_infos: Vec<DispersionInfo>,
		#[serde(rename = "dnPerfInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub dn_perf_infos: Vec<DnPerfInfo>,
		pub event: NwdafEvent,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DateTime>,
		#[serde(
			rename = "failNotifyCode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub fail_notify_code: Option<NwdafFailureCode>,
		#[serde(
			rename = "nfLoadLevelInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nf_load_level_infos: Vec<NfLoadLevelInformation>,
		#[serde(
			rename = "nsiLoadLevelInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nsi_load_level_infos: Vec<NsiLoadLevelInfo>,
		#[serde(rename = "nwPerfs", default, skip_serializing_if = "Vec::is_empty")]
		pub nw_perfs: Vec<NetworkPerfInfo>,
		#[serde(
			rename = "qosSustainInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_sustain_infos: Vec<QosSustainabilityInfo>,
		#[serde(
			rename = "redTransInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub red_trans_infos: Vec<RedundantTransmissionExpInfo>,
		#[serde(
			rename = "rvWaitTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rv_wait_time: Option<DurationSec>,
		#[serde(
			rename = "sliceLoadLevelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub slice_load_level_info: Option<SliceLoadLevelInformation>,
		#[serde(rename = "smccExps", default, skip_serializing_if = "Vec::is_empty")]
		pub smcc_exps: Vec<SmcceInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub start: Option<DateTime>,
		#[serde(rename = "svcExps", default, skip_serializing_if = "Vec::is_empty")]
		pub svc_exps: Vec<ServiceExperienceInfo>,
		#[serde(
			rename = "timeStampGen",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_stamp_gen: Option<DateTime>,
		#[serde(rename = "ueComms", default, skip_serializing_if = "Vec::is_empty")]
		pub ue_comms: Vec<UeCommunication>,
		#[serde(rename = "ueMobs", default, skip_serializing_if = "Vec::is_empty")]
		pub ue_mobs: Vec<UeMobility>,
		#[serde(
			rename = "userDataCongInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub user_data_cong_infos: Vec<UserDataCongestionInfo>,
		#[serde(rename = "wlanInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub wlan_infos: Vec<WlanPerformanceInfo>,
	}

	impl From<&EventNotification> for EventNotification {
		fn from(value: &EventNotification) -> Self {
			value.clone()
		}
	}

	/// Represents the type of reporting that the subscription requires.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the type of reporting that the subscription
	/// requires.",
	///  "type": "object",
	///  "properties": {
	///    "accPerSubset": {
	///      "description": "Each element indicates the preferred accuracy level
	/// per analytics subset. It may be present if the \"listOfAnaSubsets\"
	/// attribute is present in the subscription request when the subscription
	/// event is NF_LOAD, UE_COMM, DISPERSION, NETWORK_PERFORMANCE,
	/// WLAN_PERFORMANCE, DN_PERFORMANCE or SERVICE_EXPERIENCE.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Accuracy"
	///      },
	///      "minItems": 1
	///    },
	///    "accuracy": {
	///      "$ref": "#/components/schemas/Accuracy"
	///    },
	///    "anaMeta": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AnalyticsMetadata"
	///      },
	///      "minItems": 1
	///    },
	///    "anaMetaInd": {
	///      "$ref": "#/components/schemas/AnalyticsMetadataIndication"
	///    },
	///    "endTs": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "histAnaTimePeriod": {
	///      "$ref": "#/components/schemas/TimeWindow"
	///    },
	///    "maxObjectNbr": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "maxSupiNbr": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "offsetPeriod": {
	///      "description": "Offset period in units of seconds to the reporting
	/// time, if the value is negative means statistics in the past offset
	/// period, otherwise a positive value means prediction in the future offset
	/// period. May be present if the \"repPeriod\" attribute is included within
	/// the \"evtReq\" attribute.\n",
	///      "type": "integer"
	///    },
	///    "sampRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "startTs": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "timeAnaNeeded": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct EventReportingRequirement {
		/// Each element indicates the preferred accuracy level per analytics
		/// subset. It may be present if the "listOfAnaSubsets" attribute is
		/// present in the subscription request when the subscription event is
		/// NF_LOAD, UE_COMM, DISPERSION, NETWORK_PERFORMANCE, WLAN_PERFORMANCE,
		/// DN_PERFORMANCE or SERVICE_EXPERIENCE.
		#[serde(
			rename = "accPerSubset",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub acc_per_subset: Vec<Accuracy>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub accuracy: Option<Accuracy>,
		#[serde(rename = "anaMeta", default, skip_serializing_if = "Vec::is_empty")]
		pub ana_meta: Vec<AnalyticsMetadata>,
		#[serde(
			rename = "anaMetaInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ana_meta_ind: Option<AnalyticsMetadataIndication>,
		#[serde(rename = "endTs", default, skip_serializing_if = "Option::is_none")]
		pub end_ts: Option<DateTime>,
		#[serde(
			rename = "histAnaTimePeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hist_ana_time_period: Option<TimeWindow>,
		#[serde(
			rename = "maxObjectNbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_object_nbr: Option<Uinteger>,
		#[serde(
			rename = "maxSupiNbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_supi_nbr: Option<Uinteger>,
		/// Offset period in units of seconds to the reporting time, if the
		/// value is negative means statistics in the past offset period,
		/// otherwise a positive value means prediction in the future offset
		/// period. May be present if the "repPeriod" attribute is included
		/// within the "evtReq" attribute.
		#[serde(
			rename = "offsetPeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub offset_period: Option<i64>,
		#[serde(rename = "sampRatio", default, skip_serializing_if = "Option::is_none")]
		pub samp_ratio: Option<SamplingRatio>,
		#[serde(rename = "startTs", default, skip_serializing_if = "Option::is_none")]
		pub start_ts: Option<DateTime>,
		#[serde(
			rename = "timeAnaNeeded",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_ana_needed: Option<DateTime>,
	}

	impl From<&EventReportingRequirement> for EventReportingRequirement {
		fn from(value: &EventReportingRequirement) -> Self {
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
	///    "anySlice": {
	///      "$ref": "#/components/schemas/AnySlice"
	///    },
	///    "appIds": {
	///      "description": "Identification(s) of application to which the
	/// subscription applies.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ApplicationId"
	///      },
	///      "minItems": 1
	///    },
	///    "appServerAddrs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AddrFqdn"
	///      },
	///      "minItems": 1
	///    },
	///    "bwRequs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/BwRequirement"
	///      },
	///      "minItems": 1
	///    },
	///    "congThresholds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ThresholdLevel"
	///      },
	///      "minItems": 1
	///    },
	///    "disperReqs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DispersionRequirement"
	///      },
	///      "minItems": 1
	///    },
	///    "dnPerfReqs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DnPerformanceReq"
	///      },
	///      "minItems": 1
	///    },
	///    "dnais": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnai"
	///      },
	///      "minItems": 1
	///    },
	///    "dnns": {
	///      "description": "Identification(s) of DNN to which the subscription
	/// applies.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnn"
	///      },
	///      "minItems": 1
	///    },
	///    "event": {
	///      "$ref": "#/components/schemas/NwdafEvent"
	///    },
	///    "excepRequs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Exception"
	///      },
	///      "minItems": 1
	///    },
	///    "exptAnaType": {
	///      "$ref": "#/components/schemas/ExpectedAnalyticsType"
	///    },
	///    "exptUeBehav": {
	///      "$ref": "#/components/schemas/ExpectedUeBehaviourData"
	///    },
	///    "extraReportReq": {
	///      "$ref": "#/components/schemas/EventReportingRequirement"
	///    },
	///    "ladnDnns": {
	///      "description": "Identification(s) of LADN DNN to indicate the LADN
	/// service area as the AOI.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Dnn"
	///      },
	///      "minItems": 1
	///    },
	///    "listOfAnaSubsets": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AnalyticsSubset"
	///      },
	///      "minItems": 1
	///    },
	///    "loadLevelThreshold": {
	///      "description": "Indicates that the NWDAF shall report the
	/// corresponding network slice load level to the NF service consumer where
	/// the load level of the network slice identified by snssais is
	/// reached.\n",
	///      "type": "integer"
	///    },
	///    "matchingDir": {
	///      "$ref": "#/components/schemas/MatchingDirection"
	///    },
	///    "maxTopAppDlNbr": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "maxTopAppUlNbr": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "networkArea": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "nfInstanceIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfInstanceId"
	///      },
	///      "minItems": 1
	///    },
	///    "nfLoadLvlThds": {
	///      "description": "Shall be supplied in order to start reporting when
	/// an average load level is reached.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ThresholdLevel"
	///      },
	///      "minItems": 1
	///    },
	///    "nfSetIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NfSetId"
	///      },
	///      "minItems": 1
	///    },
	///    "nfTypes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NFType"
	///      },
	///      "minItems": 1
	///    },
	///    "notificationMethod": {
	///      "$ref": "#/components/schemas/NotificationMethod"
	///    },
	///    "nsiIdInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsiIdInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "nsiLevelThrds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uinteger"
	///      },
	///      "minItems": 1
	///    },
	///    "nwPerfRequs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NetworkPerfRequirement"
	///      },
	///      "minItems": 1
	///    },
	///    "qosFlowRetThds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RetainabilityThreshold"
	///      },
	///      "minItems": 1
	///    },
	///    "qosRequ": {
	///      "$ref": "#/components/schemas/QosRequirement"
	///    },
	///    "ranUeThrouThds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/BitRate"
	///      },
	///      "minItems": 1
	///    },
	///    "ratFreqs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatFreqInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "redTransReqs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RedundantTransmissionExpReq"
	///      },
	///      "minItems": 1
	///    },
	///    "repetitionPeriod": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "snssaia": {
	///      "description": "Identification(s) of network slice to which the
	/// subscription applies. It corresponds to snssais in the data model
	/// definition of 3GPP TS 29.520.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "tgtUe": {
	///      "$ref": "#/components/schemas/TargetUeInformation"
	///    },
	///    "upfInfo": {
	///      "$ref": "#/components/schemas/UpfInformation"
	///    },
	///    "visitedAreas": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NetworkAreaInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "wlanReqs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/WlanPerformanceReq"
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
	pub struct EventSubscription {
		#[serde(rename = "anySlice", default, skip_serializing_if = "Option::is_none")]
		pub any_slice: Option<AnySlice>,
		/// Identification(s) of application to which the subscription applies.
		#[serde(rename = "appIds", default, skip_serializing_if = "Vec::is_empty")]
		pub app_ids: Vec<ApplicationId>,
		#[serde(
			rename = "appServerAddrs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub app_server_addrs: Vec<AddrFqdn>,
		#[serde(rename = "bwRequs", default, skip_serializing_if = "Vec::is_empty")]
		pub bw_requs: Vec<BwRequirement>,
		#[serde(
			rename = "congThresholds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub cong_thresholds: Vec<ThresholdLevel>,
		#[serde(rename = "disperReqs", default, skip_serializing_if = "Vec::is_empty")]
		pub disper_reqs: Vec<DispersionRequirement>,
		#[serde(rename = "dnPerfReqs", default, skip_serializing_if = "Vec::is_empty")]
		pub dn_perf_reqs: Vec<DnPerformanceReq>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub dnais: Vec<Dnai>,
		/// Identification(s) of DNN to which the subscription applies.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub dnns: Vec<Dnn>,
		pub event: NwdafEvent,
		#[serde(rename = "excepRequs", default, skip_serializing_if = "Vec::is_empty")]
		pub excep_requs: Vec<Exception>,
		#[serde(
			rename = "exptAnaType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub expt_ana_type: Option<ExpectedAnalyticsType>,
		#[serde(
			rename = "exptUeBehav",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub expt_ue_behav: Option<ExpectedUeBehaviourData>,
		#[serde(
			rename = "extraReportReq",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub extra_report_req: Option<EventReportingRequirement>,
		/// Identification(s) of LADN DNN to indicate the LADN service area as
		/// the AOI.
		#[serde(rename = "ladnDnns", default, skip_serializing_if = "Vec::is_empty")]
		pub ladn_dnns: Vec<Dnn>,
		#[serde(
			rename = "listOfAnaSubsets",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub list_of_ana_subsets: Vec<AnalyticsSubset>,
		/// Indicates that the NWDAF shall report the corresponding network
		/// slice load level to the NF service consumer where the load level of
		/// the network slice identified by snssais is reached.
		#[serde(
			rename = "loadLevelThreshold",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub load_level_threshold: Option<i64>,
		#[serde(
			rename = "matchingDir",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub matching_dir: Option<MatchingDirection>,
		#[serde(
			rename = "maxTopAppDlNbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_top_app_dl_nbr: Option<Uinteger>,
		#[serde(
			rename = "maxTopAppUlNbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_top_app_ul_nbr: Option<Uinteger>,
		#[serde(
			rename = "networkArea",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub network_area: Option<NetworkAreaInfo>,
		#[serde(
			rename = "nfInstanceIds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nf_instance_ids: Vec<NfInstanceId>,
		/// Shall be supplied in order to start reporting when an average load
		/// level is reached.
		#[serde(
			rename = "nfLoadLvlThds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nf_load_lvl_thds: Vec<ThresholdLevel>,
		#[serde(rename = "nfSetIds", default, skip_serializing_if = "Vec::is_empty")]
		pub nf_set_ids: Vec<NfSetId>,
		#[serde(rename = "nfTypes", default, skip_serializing_if = "Vec::is_empty")]
		pub nf_types: Vec<NfType>,
		#[serde(
			rename = "notificationMethod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notification_method: Option<NotificationMethod>,
		#[serde(rename = "nsiIdInfos", default, skip_serializing_if = "Vec::is_empty")]
		pub nsi_id_infos: Vec<NsiIdInfo>,
		#[serde(
			rename = "nsiLevelThrds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nsi_level_thrds: Vec<Uinteger>,
		#[serde(rename = "nwPerfRequs", default, skip_serializing_if = "Vec::is_empty")]
		pub nw_perf_requs: Vec<NetworkPerfRequirement>,
		#[serde(
			rename = "qosFlowRetThds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub qos_flow_ret_thds: Vec<RetainabilityThreshold>,
		#[serde(rename = "qosRequ", default, skip_serializing_if = "Option::is_none")]
		pub qos_requ: Option<QosRequirement>,
		#[serde(
			rename = "ranUeThrouThds",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ran_ue_throu_thds: Vec<BitRate>,
		#[serde(rename = "ratFreqs", default, skip_serializing_if = "Vec::is_empty")]
		pub rat_freqs: Vec<RatFreqInformation>,
		#[serde(
			rename = "redTransReqs",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub red_trans_reqs: Vec<RedundantTransmissionExpReq>,
		#[serde(
			rename = "repetitionPeriod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub repetition_period: Option<DurationSec>,
		/// Identification(s) of network slice to which the subscription
		/// applies. It corresponds to snssais in the data model definition of
		/// 3GPP TS 29.520.
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub snssaia: Vec<Snssai>,
		#[serde(rename = "tgtUe", default, skip_serializing_if = "Option::is_none")]
		pub tgt_ue: Option<TargetUeInformation>,
		#[serde(rename = "upfInfo", default, skip_serializing_if = "Option::is_none")]
		pub upf_info: Option<UpfInformation>,
		#[serde(
			rename = "visitedAreas",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub visited_areas: Vec<NetworkAreaInfo>,
		#[serde(rename = "wlanReqs", default, skip_serializing_if = "Vec::is_empty")]
		pub wlan_reqs: Vec<WlanPerformanceReq>,
	}

	impl From<&EventSubscription> for EventSubscription {
		fn from(value: &EventSubscription) -> Self {
			value.clone()
		}
	}

	/// Represents the Exception information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the Exception information.",
	///  "type": "object",
	///  "required": [
	///    "excepId"
	///  ],
	///  "properties": {
	///    "excepId": {
	///      "$ref": "#/components/schemas/ExceptionId"
	///    },
	///    "excepLevel": {
	///      "type": "integer"
	///    },
	///    "excepTrend": {
	///      "$ref": "#/components/schemas/ExceptionTrend"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Exception {
		#[serde(rename = "excepId")]
		pub excep_id: ExceptionId,
		#[serde(
			rename = "excepLevel",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub excep_level: Option<i64>,
		#[serde(
			rename = "excepTrend",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub excep_trend: Option<ExceptionTrend>,
	}

	impl From<&Exception> for Exception {
		fn from(value: &Exception) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - UNEXPECTED_UE_LOCATION: Unexpected UE location
	/// - UNEXPECTED_LONG_LIVE_FLOW: Unexpected long-live rate flows
	/// - UNEXPECTED_LARGE_RATE_FLOW: Unexpected large rate flows
	/// - UNEXPECTED_WAKEUP: Unexpected wakeup
	/// - SUSPICION_OF_DDOS_ATTACK: Suspicion of DDoS attack
	/// - WRONG_DESTINATION_ADDRESS: Wrong destination address
	/// - TOO_FREQUENT_SERVICE_ACCESS: Too frequent Service Access
	/// - UNEXPECTED_RADIO_LINK_FAILURES: Unexpected radio link failures
	/// - PING_PONG_ACROSS_CELLS: Ping-ponging across neighbouring cells
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- UNEXPECTED_UE_LOCATION:
	/// Unexpected UE location\n- UNEXPECTED_LONG_LIVE_FLOW: Unexpected
	/// long-live rate flows\n- UNEXPECTED_LARGE_RATE_FLOW: Unexpected large
	/// rate flows\n- UNEXPECTED_WAKEUP: Unexpected wakeup\n-
	/// SUSPICION_OF_DDOS_ATTACK: Suspicion of DDoS attack\n-
	/// WRONG_DESTINATION_ADDRESS: Wrong destination address\n-
	/// TOO_FREQUENT_SERVICE_ACCESS: Too frequent Service Access\n-
	/// UNEXPECTED_RADIO_LINK_FAILURES: Unexpected radio link failures\n-
	/// PING_PONG_ACROSS_CELLS: Ping-ponging across neighbouring cells\n",
	///  "type": "string",
	///  "enum": [
	///    "UNEXPECTED_UE_LOCATION",
	///    "UNEXPECTED_LONG_LIVE_FLOW",
	///    "UNEXPECTED_LARGE_RATE_FLOW",
	///    "UNEXPECTED_WAKEUP",
	///    "SUSPICION_OF_DDOS_ATTACK",
	///    "WRONG_DESTINATION_ADDRESS",
	///    "TOO_FREQUENT_SERVICE_ACCESS",
	///    "UNEXPECTED_RADIO_LINK_FAILURES",
	///    "PING_PONG_ACROSS_CELLS"
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
	pub enum ExceptionId {
		#[default]
		#[serde(rename = "UNEXPECTED_UE_LOCATION")]
		UnexpectedUeLocation,
		#[serde(rename = "UNEXPECTED_LONG_LIVE_FLOW")]
		UnexpectedLongLiveFlow,
		#[serde(rename = "UNEXPECTED_LARGE_RATE_FLOW")]
		UnexpectedLargeRateFlow,
		#[serde(rename = "UNEXPECTED_WAKEUP")]
		UnexpectedWakeup,
		#[serde(rename = "SUSPICION_OF_DDOS_ATTACK")]
		SuspicionOfDdosAttack,
		#[serde(rename = "WRONG_DESTINATION_ADDRESS")]
		WrongDestinationAddress,
		#[serde(rename = "TOO_FREQUENT_SERVICE_ACCESS")]
		TooFrequentServiceAccess,
		#[serde(rename = "UNEXPECTED_RADIO_LINK_FAILURES")]
		UnexpectedRadioLinkFailures,
		#[serde(rename = "PING_PONG_ACROSS_CELLS")]
		PingPongAcrossCells,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ExceptionId> for ExceptionId {
		fn from(value: &ExceptionId) -> Self {
			value.clone()
		}
	}

	impl ToString for ExceptionId {
		fn to_string(&self) -> String {
			match *self {
				Self::UnexpectedUeLocation => "UNEXPECTED_UE_LOCATION".to_string(),
				Self::UnexpectedLongLiveFlow => "UNEXPECTED_LONG_LIVE_FLOW".to_string(),
				Self::UnexpectedLargeRateFlow => "UNEXPECTED_LARGE_RATE_FLOW".to_string(),
				Self::UnexpectedWakeup => "UNEXPECTED_WAKEUP".to_string(),
				Self::SuspicionOfDdosAttack => "SUSPICION_OF_DDOS_ATTACK".to_string(),
				Self::WrongDestinationAddress => "WRONG_DESTINATION_ADDRESS".to_string(),
				Self::TooFrequentServiceAccess => "TOO_FREQUENT_SERVICE_ACCESS".to_string(),
				Self::UnexpectedRadioLinkFailures => "UNEXPECTED_RADIO_LINK_FAILURES".to_string(),
				Self::PingPongAcrossCells => "PING_PONG_ACROSS_CELLS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ExceptionId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UNEXPECTED_UE_LOCATION" => Ok(Self::UnexpectedUeLocation),
				"UNEXPECTED_LONG_LIVE_FLOW" => Ok(Self::UnexpectedLongLiveFlow),
				"UNEXPECTED_LARGE_RATE_FLOW" => Ok(Self::UnexpectedLargeRateFlow),
				"UNEXPECTED_WAKEUP" => Ok(Self::UnexpectedWakeup),
				"SUSPICION_OF_DDOS_ATTACK" => Ok(Self::SuspicionOfDdosAttack),
				"WRONG_DESTINATION_ADDRESS" => Ok(Self::WrongDestinationAddress),
				"TOO_FREQUENT_SERVICE_ACCESS" => Ok(Self::TooFrequentServiceAccess),
				"UNEXPECTED_RADIO_LINK_FAILURES" => Ok(Self::UnexpectedRadioLinkFailures),
				"PING_PONG_ACROSS_CELLS" => Ok(Self::PingPongAcrossCells),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ExceptionId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ExceptionId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ExceptionId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - UP: Up trend of the exception level.
	/// - DOWN: Down trend of the exception level.
	/// - UNKNOW: Unknown trend of the exception level.
	/// - STABLE: Stable trend of the exception level.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- UP: Up trend of the exception
	/// level.\n- DOWN: Down trend of the exception level.\n- UNKNOW: Unknown
	/// trend of the exception level.\n- STABLE: Stable trend of the exception
	/// level.\n",
	///  "type": "string",
	///  "enum": [
	///    "UP",
	///    "DOWN",
	///    "UNKNOW",
	///    "STABLE"
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
	pub enum ExceptionTrend {
		#[default]
		#[serde(rename = "UP")]
		Up,
		#[serde(rename = "DOWN")]
		Down,
		#[serde(rename = "UNKNOW")]
		Unknow,
		#[serde(rename = "STABLE")]
		Stable,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ExceptionTrend> for ExceptionTrend {
		fn from(value: &ExceptionTrend) -> Self {
			value.clone()
		}
	}

	impl ToString for ExceptionTrend {
		fn to_string(&self) -> String {
			match *self {
				Self::Up => "UP".to_string(),
				Self::Down => "DOWN".to_string(),
				Self::Unknow => "UNKNOW".to_string(),
				Self::Stable => "STABLE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ExceptionTrend {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UP" => Ok(Self::Up),
				"DOWN" => Ok(Self::Down),
				"UNKNOW" => Ok(Self::Unknow),
				"STABLE" => Ok(Self::Stable),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ExceptionTrend {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ExceptionTrend {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ExceptionTrend {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Possible values are:
	/// - MOBILITY: Mobility related abnormal behaviour analytics is expected by
	///   the consumer.
	/// - COMMUN: Communication related abnormal behaviour analytics is expected
	///   by the consumer.
	/// - MOBILITY_AND_COMMUN: Both mobility and communication related abnormal
	///   behaviour analytics is expected by the consumer.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- MOBILITY: Mobility related abnormal behaviour analytics is expected by the consumer.\n- COMMUN: Communication related abnormal behaviour analytics is expected by the consumer.\n- MOBILITY_AND_COMMUN: Both mobility and communication related abnormal behaviour analytics is expected by the consumer.\n",
	///  "type": "string",
	///  "enum": [
	///    "MOBILITY",
	///    "COMMUN",
	///    "MOBILITY_AND_COMMUN"
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
	pub enum ExpectedAnalyticsType {
		#[default]
		#[serde(rename = "MOBILITY")]
		Mobility,
		#[serde(rename = "COMMUN")]
		Commun,
		#[serde(rename = "MOBILITY_AND_COMMUN")]
		MobilityAndCommun,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ExpectedAnalyticsType> for ExpectedAnalyticsType {
		fn from(value: &ExpectedAnalyticsType) -> Self {
			value.clone()
		}
	}

	impl ToString for ExpectedAnalyticsType {
		fn to_string(&self) -> String {
			match *self {
				Self::Mobility => "MOBILITY".to_string(),
				Self::Commun => "COMMUN".to_string(),
				Self::MobilityAndCommun => "MOBILITY_AND_COMMUN".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ExpectedAnalyticsType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MOBILITY" => Ok(Self::Mobility),
				"COMMUN" => Ok(Self::Commun),
				"MOBILITY_AND_COMMUN" => Ok(Self::MobilityAndCommun),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ExpectedAnalyticsType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ExpectedAnalyticsType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ExpectedAnalyticsType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the expected UE behavior (e.g. UE moving trajectory) and its
	/// validity period
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the expected UE behavior (e.g. UE moving
	/// trajectory) and its validity period",
	///  "type": "object",
	///  "required": [
	///    "expMoveTrajectory",
	///    "validityTime"
	///  ],
	///  "properties": {
	///    "expMoveTrajectory": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UserLocation"
	///      },
	///      "minItems": 1
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
	pub struct ExpectedUeBehavior {
		#[serde(rename = "expMoveTrajectory")]
		pub exp_move_trajectory: Vec<UserLocation>,
		#[serde(rename = "validityTime")]
		pub validity_time: DateTime,
	}

	impl From<&ExpectedUeBehavior> for ExpectedUeBehavior {
		fn from(value: &ExpectedUeBehavior) -> Self {
			value.clone()
		}
	}

	/// ExpectedUeBehaviourData
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "batteryIndication": {
	///      "$ref": "#/components/schemas/BatteryIndication"
	///    },
	///    "communicationDurationTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "expectedUmts": {
	///      "description": "Identifies the UE's expected geographical movement.
	/// The attribute is only applicable in 5G.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/LocationArea"
	///      },
	///      "minItems": 1
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
	pub struct ExpectedUeBehaviourData {
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
		/// Identifies the UE's expected geographical movement. The attribute is
		/// only applicable in 5G.
		#[serde(
			rename = "expectedUmts",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub expected_umts: Vec<LocationArea>,
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
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&ExpectedUeBehaviourData> for ExpectedUeBehaviourData {
		fn from(value: &ExpectedUeBehaviourData) -> Self {
			value.clone()
		}
	}

	/// AMF event subscription extended with additional information received for
	/// the subscription
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "AMF event subscription extended with additional
	/// information received for the subscription",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/schemas-AmfEventSubscription"
	///    },
	///    {
	///      "$ref": "#/components/schemas/AmfEventSubscriptionAddInfo"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ExtAmfEventSubscription {
		#[serde(rename = "anyUE", default, skip_serializing_if = "Option::is_none")]
		pub any_ue: Option<bool>,
		/// Map of subscribed Area of Interest (AoI) Event State in the old AMF.
		/// The JSON pointer to an AmfEventArea element in the areaList IE (or a
		/// PresenceInfo element in  presenceInfoList IE) of the AmfEvent data
		/// type shall be the key of the map.
		#[serde(
			rename = "aoiStateList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub aoi_state_list: ::std::collections::HashMap<String, AreaOfInterestEventState>,
		#[serde(rename = "bindingInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub binding_info: Vec<String>,
		#[serde(rename = "eventList")]
		pub event_list: Vec<SchemasAmfEvent>,
		#[serde(rename = "eventNotifyUri")]
		pub event_notify_uri: Uri,
		#[serde(
			rename = "eventSyncInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_sync_ind: Option<bool>,
		#[serde(
			rename = "excludeGpsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub exclude_gpsi_list: Vec<Gpsi>,
		#[serde(
			rename = "excludeSupiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub exclude_supi_list: Vec<Supi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<GroupId>,
		#[serde(
			rename = "includeGpsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub include_gpsi_list: Vec<Gpsi>,
		#[serde(
			rename = "includeSupiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub include_supi_list: Vec<Supi>,
		#[serde(
			rename = "nfConsumerInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nf_consumer_info: Vec<String>,
		#[serde(rename = "nfId")]
		pub nf_id: NfInstanceId,
		#[serde(rename = "notifyCorrelationId")]
		pub notify_correlation_id: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub options: Option<SchemasAmfEventMode>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(
			rename = "sourceNfType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_nf_type: Option<NfType>,
		#[serde(
			rename = "subsChangeNotifyCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_change_notify_correlation_id: Option<String>,
		#[serde(
			rename = "subsChangeNotifyUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_change_notify_uri: Option<Uri>,
		#[serde(
			rename = "subscribingNfType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subscribing_nf_type: Option<NfType>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
	}

	impl From<&ExtAmfEventSubscription> for ExtAmfEventSubscription {
		fn from(value: &ExtAmfEventSubscription) -> Self {
			value.clone()
		}
	}

	/// Contains the external client identification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the external client identification",
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
	pub struct ExternalClientIdentification(pub String);

	impl ::std::ops::Deref for ExternalClientIdentification {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<ExternalClientIdentification> for String {
		fn from(value: ExternalClientIdentification) -> Self {
			value.0
		}
	}

	impl From<&ExternalClientIdentification> for ExternalClientIdentification {
		fn from(value: &ExternalClientIdentification) -> Self {
			value.clone()
		}
	}

	impl From<String> for ExternalClientIdentification {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ExternalClientIdentification {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for ExternalClientIdentification {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// Contains information on the event for which the subscription is not
	/// successful.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains information on the event for which the
	/// subscription is not successful.",
	///  "type": "object",
	///  "required": [
	///    "event",
	///    "failureCode"
	///  ],
	///  "properties": {
	///    "event": {
	///      "$ref": "#/components/schemas/NwdafEvent"
	///    },
	///    "failureCode": {
	///      "$ref": "#/components/schemas/NwdafFailureCode"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct FailureEventInfo {
		pub event: NwdafEvent,
		#[serde(rename = "failureCode")]
		pub failure_code: NwdafFailureCode,
	}

	impl From<&FailureEventInfo> for FailureEventInfo {
		fn from(value: &FailureEventInfo) -> Self {
			value.clone()
		}
	}

	/// Specifies the positioning fix type.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Specifies the positioning fix type.",
	///  "type": "string",
	///  "enum": [
	///    "CARRIER_PHASE_FLOAT",
	///    "CARRIER_PHASE_FIX"
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
	pub enum FixType {
		#[default]
		#[serde(rename = "CARRIER_PHASE_FLOAT")]
		CarrierPhaseFloat,
		#[serde(rename = "CARRIER_PHASE_FIX")]
		CarrierPhaseFix,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&FixType> for FixType {
		fn from(value: &FixType) -> Self {
			value.clone()
		}
	}

	impl ToString for FixType {
		fn to_string(&self) -> String {
			match *self {
				Self::CarrierPhaseFloat => "CARRIER_PHASE_FLOAT".to_string(),
				Self::CarrierPhaseFix => "CARRIER_PHASE_FIX".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for FixType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CARRIER_PHASE_FLOAT" => Ok(Self::CarrierPhaseFloat),
				"CARRIER_PHASE_FIX" => Ok(Self::CarrierPhaseFix),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for FixType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for FixType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for FixType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Represents IP flow information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents IP flow information.",
	///  "type": "object",
	///  "required": [
	///    "flowId"
	///  ],
	///  "properties": {
	///    "flowDescriptions": {
	///      "description": "Indicates the packet filters of the IP flow. Refer
	/// to clause 5.3.8 of 3GPP TS 29.214 for encoding. It shall contain UL
	/// and/or DL IP flow description.\n",
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "flowId": {
	///      "description": "Indicates the IP flow identifier.",
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct FlowInfo {
		/// Indicates the packet filters of the IP flow. Refer to clause 5.3.8
		/// of 3GPP TS 29.214 for encoding. It shall contain UL and/or DL IP
		/// flow description.
		#[serde(
			rename = "flowDescriptions",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub flow_descriptions: Vec<String>,
		/// Indicates the IP flow identifier.
		#[serde(rename = "flowId")]
		pub flow_id: i64,
	}

	impl From<&FlowInfo> for FlowInfo {
		fn from(value: &FlowInfo) -> Self {
			value.clone()
		}
	}

	/// Global Navigation Satellite System (GNSS) ID.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Global Navigation Satellite System (GNSS) ID.",
	///  "type": "string",
	///  "enum": [
	///    "GPS",
	///    "GALILEO",
	///    "SBAS",
	///    "MODERNIZED_GPS",
	///    "QZSS",
	///    "GLONASS",
	///    "BDS",
	///    "NAVIC"
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
	pub enum GnssId {
		#[default]
		#[serde(rename = "GPS")]
		Gps,
		#[serde(rename = "GALILEO")]
		Galileo,
		#[serde(rename = "SBAS")]
		Sbas,
		#[serde(rename = "MODERNIZED_GPS")]
		ModernizedGps,
		#[serde(rename = "QZSS")]
		Qzss,
		#[serde(rename = "GLONASS")]
		Glonass,
		#[serde(rename = "BDS")]
		Bds,
		#[serde(rename = "NAVIC")]
		Navic,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&GnssId> for GnssId {
		fn from(value: &GnssId) -> Self {
			value.clone()
		}
	}

	impl ToString for GnssId {
		fn to_string(&self) -> String {
			match *self {
				Self::Gps => "GPS".to_string(),
				Self::Galileo => "GALILEO".to_string(),
				Self::Sbas => "SBAS".to_string(),
				Self::ModernizedGps => "MODERNIZED_GPS".to_string(),
				Self::Qzss => "QZSS".to_string(),
				Self::Glonass => "GLONASS".to_string(),
				Self::Bds => "BDS".to_string(),
				Self::Navic => "NAVIC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for GnssId {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"GPS" => Ok(Self::Gps),
				"GALILEO" => Ok(Self::Galileo),
				"SBAS" => Ok(Self::Sbas),
				"MODERNIZED_GPS" => Ok(Self::ModernizedGps),
				"QZSS" => Ok(Self::Qzss),
				"GLONASS" => Ok(Self::Glonass),
				"BDS" => Ok(Self::Bds),
				"NAVIC" => Ok(Self::Navic),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for GnssId {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for GnssId {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for GnssId {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the usage of a Global Navigation Satellite System (GNSS)
	/// positioning method.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the usage of a Global Navigation Satellite
	/// System (GNSS) positioning method.",
	///  "type": "object",
	///  "required": [
	///    "gnss",
	///    "mode",
	///    "usage"
	///  ],
	///  "properties": {
	///    "gnss": {
	///      "$ref": "#/components/schemas/GnssId"
	///    },
	///    "mode": {
	///      "$ref": "#/components/schemas/PositioningMode"
	///    },
	///    "usage": {
	///      "$ref": "#/components/schemas/Usage"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct GnssPositioningMethodAndUsage {
		pub gnss: GnssId,
		pub mode: PositioningMode,
		pub usage: Usage,
	}

	impl From<&GnssPositioningMethodAndUsage> for GnssPositioningMethodAndUsage {
		fn from(value: &GnssPositioningMethodAndUsage) -> Self {
			value.clone()
		}
	}

	/// High Accuracy GNSS Positioning Metrics.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "High Accuracy GNSS Positioning Metrics.",
	///  "type": "object",
	///  "properties": {
	///    "age": {
	///      "type": "integer",
	///      "maximum": 99.0,
	///      "minimum": 0.0
	///    },
	///    "fixType": {
	///      "$ref": "#/components/schemas/FixType"
	///    },
	///    "hdopi": {
	///      "type": "integer",
	///      "maximum": 256.0,
	///      "minimum": 1.0
	///    },
	///    "nrOfUsedSatellites": {
	///      "type": "integer",
	///      "maximum": 64.0,
	///      "minimum": 0.0
	///    },
	///    "pdopi": {
	///      "type": "integer",
	///      "maximum": 256.0,
	///      "minimum": 1.0
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HighAccuracyGnssMetrics {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub age: Option<i64>,
		#[serde(rename = "fixType", default, skip_serializing_if = "Option::is_none")]
		pub fix_type: Option<FixType>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub hdopi: Option<i64>,
		#[serde(
			rename = "nrOfUsedSatellites",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nr_of_used_satellites: Option<i64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pdopi: Option<i64>,
	}

	impl From<&HighAccuracyGnssMetrics> for HighAccuracyGnssMetrics {
		fn from(value: &HighAccuracyGnssMetrics) -> Self {
			value.clone()
		}
	}

	/// Contains the Horizontal Protection Level
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Horizontal Protection Level",
	///  "type": "integer",
	///  "maximum": 50000.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HorizontalProtectionLevel(pub i64);

	impl ::std::ops::Deref for HorizontalProtectionLevel {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<HorizontalProtectionLevel> for i64 {
		fn from(value: HorizontalProtectionLevel) -> Self {
			value.0
		}
	}

	impl From<&HorizontalProtectionLevel> for HorizontalProtectionLevel {
		fn from(value: &HorizontalProtectionLevel) -> Self {
			value.clone()
		}
	}

	impl From<i64> for HorizontalProtectionLevel {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for HorizontalProtectionLevel {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for HorizontalProtectionLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for HorizontalProtectionLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for HorizontalProtectionLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for HorizontalProtectionLevel {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// HorizontalSpeed
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates value of horizontal speed.",
	///  "type": "number",
	///  "format": "float",
	///  "maximum": 2047.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HorizontalSpeed(pub f32);

	impl ::std::ops::Deref for HorizontalSpeed {
		type Target = f32;
		fn deref(&self) -> &f32 {
			&self.0
		}
	}

	impl From<HorizontalSpeed> for f32 {
		fn from(value: HorizontalSpeed) -> Self {
			value.0
		}
	}

	impl From<&HorizontalSpeed> for HorizontalSpeed {
		fn from(value: &HorizontalSpeed) -> Self {
			value.clone()
		}
	}

	impl From<f32> for HorizontalSpeed {
		fn from(value: f32) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for HorizontalSpeed {
		type Err = <f32 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for HorizontalSpeed {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for HorizontalSpeed {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for HorizontalSpeed {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for HorizontalSpeed {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Horizontal velocity.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Horizontal velocity.",
	///  "type": "object",
	///  "required": [
	///    "bearing",
	///    "hSpeed"
	///  ],
	///  "properties": {
	///    "bearing": {
	///      "$ref": "#/components/schemas/Angle"
	///    },
	///    "hSpeed": {
	///      "$ref": "#/components/schemas/HorizontalSpeed"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HorizontalVelocity {
		pub bearing: Angle,
		#[serde(rename = "hSpeed")]
		pub h_speed: HorizontalSpeed,
	}

	impl From<&HorizontalVelocity> for HorizontalVelocity {
		fn from(value: &HorizontalVelocity) -> Self {
			value.clone()
		}
	}

	/// Horizontal velocity with speed uncertainty.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Horizontal velocity with speed uncertainty.",
	///  "type": "object",
	///  "required": [
	///    "bearing",
	///    "hSpeed",
	///    "hUncertainty"
	///  ],
	///  "properties": {
	///    "bearing": {
	///      "$ref": "#/components/schemas/Angle"
	///    },
	///    "hSpeed": {
	///      "$ref": "#/components/schemas/HorizontalSpeed"
	///    },
	///    "hUncertainty": {
	///      "$ref": "#/components/schemas/SpeedUncertainty"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HorizontalVelocityWithUncertainty {
		pub bearing: Angle,
		#[serde(rename = "hSpeed")]
		pub h_speed: HorizontalSpeed,
		#[serde(rename = "hUncertainty")]
		pub h_uncertainty: SpeedUncertainty,
	}

	impl From<&HorizontalVelocityWithUncertainty> for HorizontalVelocityWithUncertainty {
		fn from(value: &HorizontalVelocityWithUncertainty) -> Self {
			value.clone()
		}
	}

	/// Horizontal and vertical velocity.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Horizontal and vertical velocity.",
	///  "type": "object",
	///  "required": [
	///    "bearing",
	///    "hSpeed",
	///    "vDirection",
	///    "vSpeed"
	///  ],
	///  "properties": {
	///    "bearing": {
	///      "$ref": "#/components/schemas/Angle"
	///    },
	///    "hSpeed": {
	///      "$ref": "#/components/schemas/HorizontalSpeed"
	///    },
	///    "vDirection": {
	///      "$ref": "#/components/schemas/VerticalDirection"
	///    },
	///    "vSpeed": {
	///      "$ref": "#/components/schemas/VerticalSpeed"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HorizontalWithVerticalVelocity {
		pub bearing: Angle,
		#[serde(rename = "hSpeed")]
		pub h_speed: HorizontalSpeed,
		#[serde(rename = "vDirection")]
		pub v_direction: VerticalDirection,
		#[serde(rename = "vSpeed")]
		pub v_speed: VerticalSpeed,
	}

	impl From<&HorizontalWithVerticalVelocity> for HorizontalWithVerticalVelocity {
		fn from(value: &HorizontalWithVerticalVelocity) -> Self {
			value.clone()
		}
	}

	/// Horizontal and vertical velocity with speed uncertainty.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Horizontal and vertical velocity with speed
	/// uncertainty.",
	///  "type": "object",
	///  "required": [
	///    "bearing",
	///    "hSpeed",
	///    "hUncertainty",
	///    "vDirection",
	///    "vSpeed",
	///    "vUncertainty"
	///  ],
	///  "properties": {
	///    "bearing": {
	///      "$ref": "#/components/schemas/Angle"
	///    },
	///    "hSpeed": {
	///      "$ref": "#/components/schemas/HorizontalSpeed"
	///    },
	///    "hUncertainty": {
	///      "$ref": "#/components/schemas/SpeedUncertainty"
	///    },
	///    "vDirection": {
	///      "$ref": "#/components/schemas/VerticalDirection"
	///    },
	///    "vSpeed": {
	///      "$ref": "#/components/schemas/VerticalSpeed"
	///    },
	///    "vUncertainty": {
	///      "$ref": "#/components/schemas/SpeedUncertainty"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct HorizontalWithVerticalVelocityAndUncertainty {
		pub bearing: Angle,
		#[serde(rename = "hSpeed")]
		pub h_speed: HorizontalSpeed,
		#[serde(rename = "hUncertainty")]
		pub h_uncertainty: SpeedUncertainty,
		#[serde(rename = "vDirection")]
		pub v_direction: VerticalDirection,
		#[serde(rename = "vSpeed")]
		pub v_speed: VerticalSpeed,
		#[serde(rename = "vUncertainty")]
		pub v_uncertainty: SpeedUncertainty,
	}

	impl From<&HorizontalWithVerticalVelocityAndUncertainty>
		for HorizontalWithVerticalVelocityAndUncertainty
	{
		fn from(value: &HorizontalWithVerticalVelocityAndUncertainty) -> Self {
			value.clone()
		}
	}

	/// Represents the idle status indication.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the idle status indication.",
	///  "type": "object",
	///  "properties": {
	///    "activeTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "edrxCycleLength": {
	///      "type": "integer"
	///    },
	///    "subsRegTimer": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "suggestedNumOfDlPackets": {
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
	pub struct IdleStatusIndication {
		#[serde(
			rename = "activeTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub active_time: Option<DurationSec>,
		#[serde(
			rename = "edrxCycleLength",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub edrx_cycle_length: Option<i64>,
		#[serde(
			rename = "subsRegTimer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_reg_timer: Option<DurationSec>,
		#[serde(
			rename = "suggestedNumOfDlPackets",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub suggested_num_of_dl_packets: Option<i64>,
		#[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
		pub time_stamp: Option<DateTime>,
	}

	impl From<&IdleStatusIndication> for IdleStatusIndication {
		fn from(value: &IdleStatusIndication) -> Self {
			value.clone()
		}
	}

	/// Immediate MDT Configuration
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Immediate MDT Configuration",
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
	///    "eventThresholdRsrp": {
	///      "type": "integer",
	///      "maximum": 97.0,
	///      "minimum": 0.0
	///    },
	///    "eventThresholdRsrpNr": {
	///      "type": "integer",
	///      "maximum": 127.0,
	///      "minimum": 0.0
	///    },
	///    "eventThresholdRsrq": {
	///      "type": "integer",
	///      "maximum": 34.0,
	///      "minimum": 0.0
	///    },
	///    "eventThresholdRsrqNr": {
	///      "type": "integer",
	///      "maximum": 127.0,
	///      "minimum": 0.0
	///    },
	///    "jobType": {
	///      "$ref": "#/components/schemas/JobType"
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
	///      },
	///      "minItems": 1
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
	pub struct ImmediateMdtConf {
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
		#[serde(
			rename = "eventThresholdRsrp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_threshold_rsrp: Option<i64>,
		#[serde(
			rename = "eventThresholdRsrpNr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_threshold_rsrp_nr: Option<i64>,
		#[serde(
			rename = "eventThresholdRsrq",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_threshold_rsrq: Option<i64>,
		#[serde(
			rename = "eventThresholdRsrqNr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub event_threshold_rsrq_nr: Option<i64>,
		#[serde(rename = "jobType")]
		pub job_type: JobType,
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

	impl From<&ImmediateMdtConf> for ImmediateMdtConf {
		fn from(value: &ImmediateMdtConf) -> Self {
			value.clone()
		}
	}

	/// Indicates the supported Integrity Algorithm
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the supported Integrity Algorithm",
	///  "type": "string",
	///  "enum": [
	///    "NIA0",
	///    "NIA1",
	///    "NIA2",
	///    "NIA3"
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
	pub enum IntegrityAlgorithm {
		#[default]
		#[serde(rename = "NIA0")]
		Nia0,
		#[serde(rename = "NIA1")]
		Nia1,
		#[serde(rename = "NIA2")]
		Nia2,
		#[serde(rename = "NIA3")]
		Nia3,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&IntegrityAlgorithm> for IntegrityAlgorithm {
		fn from(value: &IntegrityAlgorithm) -> Self {
			value.clone()
		}
	}

	impl ToString for IntegrityAlgorithm {
		fn to_string(&self) -> String {
			match *self {
				Self::Nia0 => "NIA0".to_string(),
				Self::Nia1 => "NIA1".to_string(),
				Self::Nia2 => "NIA2".to_string(),
				Self::Nia3 => "NIA3".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for IntegrityAlgorithm {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NIA0" => Ok(Self::Nia0),
				"NIA1" => Ok(Self::Nia1),
				"NIA2" => Ok(Self::Nia2),
				"NIA3" => Ok(Self::Nia3),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for IntegrityAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for IntegrityAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for IntegrityAlgorithm {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// integrity requirements.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "integrity requirements.",
	///  "type": "object",
	///  "properties": {
	///    "alertLimit": {
	///      "$ref": "#/components/schemas/AlertLimit"
	///    },
	///    "targetIntegrityRisk": {
	///      "$ref": "#/components/schemas/TargetIntegrityRisk"
	///    },
	///    "timeToAlert": {
	///      "$ref": "#/components/schemas/TimeToAlert"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct IntegrityRequirements {
		#[serde(
			rename = "alertLimit",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub alert_limit: Option<AlertLimit>,
		#[serde(
			rename = "targetIntegrityRisk",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_integrity_risk: Option<TargetIntegrityRisk>,
		#[serde(
			rename = "timeToAlert",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_to_alert: Option<TimeToAlert>,
	}

	impl From<&IntegrityRequirements> for IntegrityRequirements {
		fn from(value: &IntegrityRequirements) -> Self {
			value.clone()
		}
	}

	/// IpAddress
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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

	/// Contains the description of an Uplink and/or Downlink Ethernet flow.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the description of an Uplink and/or Downlink
	/// Ethernet flow.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "ipTrafficFilter"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ethTrafficFilter"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "ethTrafficFilter": {
	///      "$ref": "#/components/schemas/EthFlowDescription"
	///    },
	///    "ipTrafficFilter": {
	///      "$ref": "#/components/schemas/FlowDescription"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum IpEthFlowDescription {
		#[default]
		Variant0 {
			#[serde(rename = "ipTrafficFilter")]
			ip_traffic_filter: FlowDescription,
		},
		Variant1 {
			#[serde(rename = "ethTrafficFilter")]
			eth_traffic_filter: EthFlowDescription,
		},
	}

	impl From<&IpEthFlowDescription> for IpEthFlowDescription {
		fn from(value: &IpEthFlowDescription) -> Self {
			value.clone()
		}
	}

	/// Represents the Kamf or K'amf
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the Kamf or K'amf",
	///  "type": "object",
	///  "required": [
	///    "keyType",
	///    "keyVal"
	///  ],
	///  "properties": {
	///    "keyType": {
	///      "$ref": "#/components/schemas/KeyAmfType"
	///    },
	///    "keyVal": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct KeyAmf {
		#[serde(rename = "keyType")]
		pub key_type: KeyAmfType,
		#[serde(rename = "keyVal")]
		pub key_val: String,
	}

	impl From<&KeyAmf> for KeyAmf {
		fn from(value: &KeyAmf) -> Self {
			value.clone()
		}
	}

	/// Indicates the Kamf type
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the Kamf type",
	///  "type": "string",
	///  "enum": [
	///    "KAMF",
	///    "KPRIMEAMF"
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
	pub enum KeyAmfType {
		#[default]
		#[serde(rename = "KAMF")]
		Kamf,
		#[serde(rename = "KPRIMEAMF")]
		Kprimeamf,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&KeyAmfType> for KeyAmfType {
		fn from(value: &KeyAmfType) -> Self {
			value.clone()
		}
	}

	impl ToString for KeyAmfType {
		fn to_string(&self) -> String {
			match *self {
				Self::Kamf => "KAMF".to_string(),
				Self::Kprimeamf => "KPRIMEAMF".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for KeyAmfType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"KAMF" => Ok(Self::Kamf),
				"KPRIMEAMF" => Ok(Self::Kprimeamf),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for KeyAmfType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for KeyAmfType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for KeyAmfType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// LADN Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "LADN Information",
	///  "type": "object",
	///  "required": [
	///    "ladn"
	///  ],
	///  "properties": {
	///    "ladn": {
	///      "type": "string"
	///    },
	///    "presence": {
	///      "$ref": "#/components/schemas/PresenceState"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LadnInfo {
		pub ladn: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub presence: Option<PresenceState>,
	}

	impl From<&LadnInfo> for LadnInfo {
		fn from(value: &LadnInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates priority of the LCS client.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates priority of the LCS client.",
	///  "type": "string",
	///  "enum": [
	///    "HIGHEST_PRIORITY",
	///    "NORMAL_PRIORITY"
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
	pub enum LcsPriority {
		#[default]
		#[serde(rename = "HIGHEST_PRIORITY")]
		HighestPriority,
		#[serde(rename = "NORMAL_PRIORITY")]
		NormalPriority,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LcsPriority> for LcsPriority {
		fn from(value: &LcsPriority) -> Self {
			value.clone()
		}
	}

	impl ToString for LcsPriority {
		fn to_string(&self) -> String {
			match *self {
				Self::HighestPriority => "HIGHEST_PRIORITY".to_string(),
				Self::NormalPriority => "NORMAL_PRIORITY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LcsPriority {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"HIGHEST_PRIORITY" => Ok(Self::HighestPriority),
				"NORMAL_PRIORITY" => Ok(Self::NormalPriority),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LcsPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LcsPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LcsPriority {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Specifies LCS QoS class.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Specifies LCS QoS class.",
	///  "type": "string",
	///  "enum": [
	///    "BEST_EFFORT",
	///    "ASSURED",
	///    "MULTIPLE_QOS"
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
	pub enum LcsQosClass {
		#[default]
		#[serde(rename = "BEST_EFFORT")]
		BestEffort,
		#[serde(rename = "ASSURED")]
		Assured,
		#[serde(rename = "MULTIPLE_QOS")]
		MultipleQos,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LcsQosClass> for LcsQosClass {
		fn from(value: &LcsQosClass) -> Self {
			value.clone()
		}
	}

	impl ToString for LcsQosClass {
		fn to_string(&self) -> String {
			match *self {
				Self::BestEffort => "BEST_EFFORT".to_string(),
				Self::Assured => "ASSURED".to_string(),
				Self::MultipleQos => "MULTIPLE_QOS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LcsQosClass {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"BEST_EFFORT" => Ok(Self::BestEffort),
				"ASSURED" => Ok(Self::Assured),
				"MULTIPLE_QOS" => Ok(Self::MultipleQos),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LcsQosClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LcsQosClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LcsQosClass {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// LCS service type.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "LCS service type.",
	///  "type": "integer",
	///  "maximum": 127.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LcsServiceType(pub i64);

	impl ::std::ops::Deref for LcsServiceType {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<LcsServiceType> for i64 {
		fn from(value: LcsServiceType) -> Self {
			value.0
		}
	}

	impl From<&LcsServiceType> for LcsServiceType {
		fn from(value: &LcsServiceType) -> Self {
			value.clone()
		}
	}

	impl From<i64> for LcsServiceType {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for LcsServiceType {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for LcsServiceType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LcsServiceType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LcsServiceType {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for LcsServiceType {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// LDR Reference.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "LDR Reference.",
	///  "type": "string",
	///  "maxLength": 510,
	///  "minLength": 2
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Serialize,
		Clone,
		Debug,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd,
		smart_default::SmartDefault,
	)]
	pub struct LdrReference(String);

	impl ::std::ops::Deref for LdrReference {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<LdrReference> for String {
		fn from(value: LdrReference) -> Self {
			value.0
		}
	}

	impl From<&LdrReference> for LdrReference {
		fn from(value: &LdrReference) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for LdrReference {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			if value.len() > 510usize {
				return Err("longer than 510 characters".into());
			}
			if value.len() < 2usize {
				return Err("shorter than 2 characters".into());
			}
			Ok(Self(value.to_string()))
		}
	}

	impl ::std::convert::TryFrom<&str> for LdrReference {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for LdrReference {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for LdrReference {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for LdrReference {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// Indicates LDR types.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates LDR types.",
	///  "type": "string",
	///  "enum": [
	///    "UE_AVAILABLE",
	///    "PERIODIC",
	///    "ENTERING_INTO_AREA",
	///    "LEAVING_FROM_AREA",
	///    "BEING_INSIDE_AREA",
	///    "MOTION"
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
	pub enum LdrType {
		#[default]
		#[serde(rename = "UE_AVAILABLE")]
		UeAvailable,
		#[serde(rename = "PERIODIC")]
		Periodic,
		#[serde(rename = "ENTERING_INTO_AREA")]
		EnteringIntoArea,
		#[serde(rename = "LEAVING_FROM_AREA")]
		LeavingFromArea,
		#[serde(rename = "BEING_INSIDE_AREA")]
		BeingInsideArea,
		#[serde(rename = "MOTION")]
		Motion,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LdrType> for LdrType {
		fn from(value: &LdrType) -> Self {
			value.clone()
		}
	}

	impl ToString for LdrType {
		fn to_string(&self) -> String {
			match *self {
				Self::UeAvailable => "UE_AVAILABLE".to_string(),
				Self::Periodic => "PERIODIC".to_string(),
				Self::EnteringIntoArea => "ENTERING_INTO_AREA".to_string(),
				Self::LeavingFromArea => "LEAVING_FROM_AREA".to_string(),
				Self::BeingInsideArea => "BEING_INSIDE_AREA".to_string(),
				Self::Motion => "MOTION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LdrType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UE_AVAILABLE" => Ok(Self::UeAvailable),
				"PERIODIC" => Ok(Self::Periodic),
				"ENTERING_INTO_AREA" => Ok(Self::EnteringIntoArea),
				"LEAVING_FROM_AREA" => Ok(Self::LeavingFromArea),
				"BEING_INSIDE_AREA" => Ok(Self::BeingInsideArea),
				"MOTION" => Ok(Self::Motion),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LdrType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LdrType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LdrType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Minimum straight line distance moved by a UE to trigger a motion event
	/// report.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Minimum straight line distance moved by a UE to trigger
	/// a motion event report.",
	///  "type": "integer",
	///  "maximum": 10000.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LinearDistance(pub i64);

	impl ::std::ops::Deref for LinearDistance {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<LinearDistance> for i64 {
		fn from(value: LinearDistance) -> Self {
			value.0
		}
	}

	impl From<&LinearDistance> for LinearDistance {
		fn from(value: &LinearDistance) -> Self {
			value.clone()
		}
	}

	impl From<i64> for LinearDistance {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for LinearDistance {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for LinearDistance {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LinearDistance {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LinearDistance {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for LinearDistance {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// Local area specified by different shape
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Local area specified by different shape",
	///  "oneOf": [
	///    {
	///      "$ref": "#/components/schemas/Local2dPointUncertaintyEllipse"
	///    },
	///    {
	///      "$ref": "#/components/schemas/Local3dPointUncertaintyEllipsoid"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum LocalArea {
		#[default]
		Local2dPointUncertaintyEllipse(Local2dPointUncertaintyEllipse),
		Local3dPointUncertaintyEllipsoid(Local3dPointUncertaintyEllipsoid),
	}

	impl From<&LocalArea> for LocalArea {
		fn from(value: &LocalArea) -> Self {
			value.clone()
		}
	}

	impl From<Local2dPointUncertaintyEllipse> for LocalArea {
		fn from(value: Local2dPointUncertaintyEllipse) -> Self {
			Self::Local2dPointUncertaintyEllipse(value)
		}
	}

	impl From<Local3dPointUncertaintyEllipsoid> for LocalArea {
		fn from(value: Local3dPointUncertaintyEllipsoid) -> Self {
			Self::Local3dPointUncertaintyEllipsoid(value)
		}
	}

	/// LocationArea
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "properties": {
	///    "civicAddresses": {
	///      "description": "Identifies a list of civic addresses of the user
	/// where the UE is located.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CivicAddress"
	///      },
	///      "minItems": 0
	///    },
	///    "geographicAreas": {
	///      "description": "Identifies a list of geographic area of the user
	/// where the UE is located.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GeographicArea"
	///      },
	///      "minItems": 0
	///    },
	///    "nwAreaInfo": {
	///      "$ref": "#/components/schemas/schemas-NetworkAreaInfo"
	///    },
	///    "umtTime": {
	///      "$ref": "#/components/schemas/UmtTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LocationArea {
		/// Identifies a list of civic addresses of the user where the UE is
		/// located.
		#[serde(
			rename = "civicAddresses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub civic_addresses: Vec<CivicAddress>,
		/// Identifies a list of geographic area of the user where the UE is
		/// located.
		#[serde(
			rename = "geographicAreas",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub geographic_areas: Vec<GeographicArea>,
		#[serde(
			rename = "nwAreaInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nw_area_info: Option<SchemasNetworkAreaInfo>,
		#[serde(rename = "umtTime", default, skip_serializing_if = "Option::is_none")]
		pub umt_time: Option<UmtTime>,
	}

	impl From<&LocationArea> for LocationArea {
		fn from(value: &LocationArea) -> Self {
			value.clone()
		}
	}

	/// Type of events initiating location procedures
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Type of events initiating location procedures",
	///  "type": "string",
	///  "enum": [
	///    "EMERGENCY_CALL_ORIGINATION",
	///    "EMERGENCY_CALL_RELEASE",
	///    "EMERGENCY_CALL_HANDOVER",
	///    "ACTIVATION_OF_DEFERRED_LOCATION",
	///    "UE_MOBILITY_FOR_DEFERRED_LOCATION",
	///    "CANCELLATION_OF_DEFERRED_LOCATION"
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
	pub enum LocationEvent {
		#[default]
		#[serde(rename = "EMERGENCY_CALL_ORIGINATION")]
		EmergencyCallOrigination,
		#[serde(rename = "EMERGENCY_CALL_RELEASE")]
		EmergencyCallRelease,
		#[serde(rename = "EMERGENCY_CALL_HANDOVER")]
		EmergencyCallHandover,
		#[serde(rename = "ACTIVATION_OF_DEFERRED_LOCATION")]
		ActivationOfDeferredLocation,
		#[serde(rename = "UE_MOBILITY_FOR_DEFERRED_LOCATION")]
		UeMobilityForDeferredLocation,
		#[serde(rename = "CANCELLATION_OF_DEFERRED_LOCATION")]
		CancellationOfDeferredLocation,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LocationEvent> for LocationEvent {
		fn from(value: &LocationEvent) -> Self {
			value.clone()
		}
	}

	impl ToString for LocationEvent {
		fn to_string(&self) -> String {
			match *self {
				Self::EmergencyCallOrigination => "EMERGENCY_CALL_ORIGINATION".to_string(),
				Self::EmergencyCallRelease => "EMERGENCY_CALL_RELEASE".to_string(),
				Self::EmergencyCallHandover => "EMERGENCY_CALL_HANDOVER".to_string(),
				Self::ActivationOfDeferredLocation => "ACTIVATION_OF_DEFERRED_LOCATION".to_string(),
				Self::UeMobilityForDeferredLocation => {
					"UE_MOBILITY_FOR_DEFERRED_LOCATION".to_string()
				}
				Self::CancellationOfDeferredLocation => {
					"CANCELLATION_OF_DEFERRED_LOCATION".to_string()
				}
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LocationEvent {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EMERGENCY_CALL_ORIGINATION" => Ok(Self::EmergencyCallOrigination),
				"EMERGENCY_CALL_RELEASE" => Ok(Self::EmergencyCallRelease),
				"EMERGENCY_CALL_HANDOVER" => Ok(Self::EmergencyCallHandover),
				"ACTIVATION_OF_DEFERRED_LOCATION" => Ok(Self::ActivationOfDeferredLocation),
				"UE_MOBILITY_FOR_DEFERRED_LOCATION" => Ok(Self::UeMobilityForDeferredLocation),
				"CANCELLATION_OF_DEFERRED_LOCATION" => Ok(Self::CancellationOfDeferredLocation),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LocationEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LocationEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LocationEvent {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Describes the supported filters of LOCATION_REPORT event type
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the supported filters of LOCATION_REPORT
	/// event type",
	///  "type": "string",
	///  "enum": [
	///    "TAI",
	///    "CELL_ID",
	///    "RAN_NODE",
	///    "N3IWF",
	///    "UE_IP",
	///    "UDP_PORT",
	///    "TNAP_ID",
	///    "GLI",
	///    "TWAP_ID"
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
	pub enum LocationFilter {
		#[default]
		#[serde(rename = "TAI")]
		Tai,
		#[serde(rename = "CELL_ID")]
		CellId,
		#[serde(rename = "RAN_NODE")]
		RanNode,
		#[serde(rename = "N3IWF")]
		N3iwf,
		#[serde(rename = "UE_IP")]
		UeIp,
		#[serde(rename = "UDP_PORT")]
		UdpPort,
		#[serde(rename = "TNAP_ID")]
		TnapId,
		#[serde(rename = "GLI")]
		Gli,
		#[serde(rename = "TWAP_ID")]
		TwapId,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LocationFilter> for LocationFilter {
		fn from(value: &LocationFilter) -> Self {
			value.clone()
		}
	}

	impl ToString for LocationFilter {
		fn to_string(&self) -> String {
			match *self {
				Self::Tai => "TAI".to_string(),
				Self::CellId => "CELL_ID".to_string(),
				Self::RanNode => "RAN_NODE".to_string(),
				Self::N3iwf => "N3IWF".to_string(),
				Self::UeIp => "UE_IP".to_string(),
				Self::UdpPort => "UDP_PORT".to_string(),
				Self::TnapId => "TNAP_ID".to_string(),
				Self::Gli => "GLI".to_string(),
				Self::TwapId => "TWAP_ID".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LocationFilter {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TAI" => Ok(Self::Tai),
				"CELL_ID" => Ok(Self::CellId),
				"RAN_NODE" => Ok(Self::RanNode),
				"N3IWF" => Ok(Self::N3iwf),
				"UE_IP" => Ok(Self::UeIp),
				"UDP_PORT" => Ok(Self::UdpPort),
				"TNAP_ID" => Ok(Self::TnapId),
				"GLI" => Ok(Self::Gli),
				"TWAP_ID" => Ok(Self::TwapId),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LocationFilter {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LocationFilter {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LocationFilter {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// The result of location privacy verification by UE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The result of location privacy verification by UE",
	///  "type": "string",
	///  "enum": [
	///    "LOCATION_ALLOWED",
	///    "LOCATION_NOT_ALLOWED",
	///    "RESPONSE_TIME_OUT"
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
	pub enum LocationPrivacyVerResult {
		#[default]
		#[serde(rename = "LOCATION_ALLOWED")]
		LocationAllowed,
		#[serde(rename = "LOCATION_NOT_ALLOWED")]
		LocationNotAllowed,
		#[serde(rename = "RESPONSE_TIME_OUT")]
		ResponseTimeOut,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LocationPrivacyVerResult> for LocationPrivacyVerResult {
		fn from(value: &LocationPrivacyVerResult) -> Self {
			value.clone()
		}
	}

	impl ToString for LocationPrivacyVerResult {
		fn to_string(&self) -> String {
			match *self {
				Self::LocationAllowed => "LOCATION_ALLOWED".to_string(),
				Self::LocationNotAllowed => "LOCATION_NOT_ALLOWED".to_string(),
				Self::ResponseTimeOut => "RESPONSE_TIME_OUT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LocationPrivacyVerResult {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOCATION_ALLOWED" => Ok(Self::LocationAllowed),
				"LOCATION_NOT_ALLOWED" => Ok(Self::LocationNotAllowed),
				"RESPONSE_TIME_OUT" => Ok(Self::ResponseTimeOut),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LocationPrivacyVerResult {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LocationPrivacyVerResult {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LocationPrivacyVerResult {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// QoS of Location request.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "QoS of Location request.",
	///  "type": "object",
	///  "properties": {
	///    "hAccuracy": {
	///      "$ref": "#/components/schemas/schemas-Accuracy"
	///    },
	///    "lcsQosClass": {
	///      "$ref": "#/components/schemas/LcsQosClass"
	///    },
	///    "minorLocQoses": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MinorLocationQoS"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "responseTime": {
	///      "$ref": "#/components/schemas/ResponseTime"
	///    },
	///    "vAccuracy": {
	///      "$ref": "#/components/schemas/schemas-Accuracy"
	///    },
	///    "verticalRequested": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LocationQoS {
		#[serde(rename = "hAccuracy", default, skip_serializing_if = "Option::is_none")]
		pub h_accuracy: Option<SchemasAccuracy>,
		#[serde(
			rename = "lcsQosClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lcs_qos_class: Option<LcsQosClass>,
		#[serde(
			rename = "minorLocQoses",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub minor_loc_qoses: Vec<MinorLocationQoS>,
		#[serde(
			rename = "responseTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub response_time: Option<ResponseTime>,
		#[serde(rename = "vAccuracy", default, skip_serializing_if = "Option::is_none")]
		pub v_accuracy: Option<SchemasAccuracy>,
		#[serde(
			rename = "verticalRequested",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vertical_requested: Option<bool>,
	}

	impl From<&LocationQoS> for LocationQoS {
		fn from(value: &LocationQoS) -> Self {
			value.clone()
		}
	}

	/// Type of location measurement requested
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Type of location measurement requested",
	///  "type": "string",
	///  "enum": [
	///    "CURRENT_LOCATION",
	///    "CURRENT_OR_LAST_KNOWN_LOCATION",
	///    "NOTIFICATION_VERIFICATION_ONLY",
	///    "DEFERRED_LOCATION"
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
	pub enum LocationType {
		#[default]
		#[serde(rename = "CURRENT_LOCATION")]
		CurrentLocation,
		#[serde(rename = "CURRENT_OR_LAST_KNOWN_LOCATION")]
		CurrentOrLastKnownLocation,
		#[serde(rename = "NOTIFICATION_VERIFICATION_ONLY")]
		NotificationVerificationOnly,
		#[serde(rename = "DEFERRED_LOCATION")]
		DeferredLocation,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LocationType> for LocationType {
		fn from(value: &LocationType) -> Self {
			value.clone()
		}
	}

	impl ToString for LocationType {
		fn to_string(&self) -> String {
			match *self {
				Self::CurrentLocation => "CURRENT_LOCATION".to_string(),
				Self::CurrentOrLastKnownLocation => "CURRENT_OR_LAST_KNOWN_LOCATION".to_string(),
				Self::NotificationVerificationOnly => "NOTIFICATION_VERIFICATION_ONLY".to_string(),
				Self::DeferredLocation => "DEFERRED_LOCATION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LocationType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CURRENT_LOCATION" => Ok(Self::CurrentLocation),
				"CURRENT_OR_LAST_KNOWN_LOCATION" => Ok(Self::CurrentOrLastKnownLocation),
				"NOTIFICATION_VERIFICATION_ONLY" => Ok(Self::NotificationVerificationOnly),
				"DEFERRED_LOCATION" => Ok(Self::DeferredLocation),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LocationType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LocationType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LocationType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Describes the reason for loss of connectivity
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the reason for loss of connectivity",
	///  "type": "string",
	///  "enum": [
	///    "DEREGISTERED",
	///    "MAX_DETECTION_TIME_EXPIRED",
	///    "PURGED"
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
	pub enum LossOfConnectivityReason {
		#[default]
		#[serde(rename = "DEREGISTERED")]
		Deregistered,
		#[serde(rename = "MAX_DETECTION_TIME_EXPIRED")]
		MaxDetectionTimeExpired,
		#[serde(rename = "PURGED")]
		Purged,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&LossOfConnectivityReason> for LossOfConnectivityReason {
		fn from(value: &LossOfConnectivityReason) -> Self {
			value.clone()
		}
	}

	impl ToString for LossOfConnectivityReason {
		fn to_string(&self) -> String {
			match *self {
				Self::Deregistered => "DEREGISTERED".to_string(),
				Self::MaxDetectionTimeExpired => "MAX_DETECTION_TIME_EXPIRED".to_string(),
				Self::Purged => "PURGED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for LossOfConnectivityReason {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DEREGISTERED" => Ok(Self::Deregistered),
				"MAX_DETECTION_TIME_EXPIRED" => Ok(Self::MaxDetectionTimeExpired),
				"PURGED" => Ok(Self::Purged),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for LossOfConnectivityReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for LossOfConnectivityReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for LossOfConnectivityReason {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// LTE-M Indication.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "LTE-M Indication.",
	///  "type": "object",
	///  "required": [
	///    "lteCatMInd"
	///  ],
	///  "properties": {
	///    "lteCatMInd": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct LteMInd {
		#[serde(rename = "lteCatMInd")]
		pub lte_cat_m_ind: bool,
	}

	impl From<&LteMInd> for LteMInd {
		fn from(value: &LteMInd) -> Self {
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

	/// Maximum interval between event reports.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Maximum interval between event reports.",
	///  "type": "integer",
	///  "maximum": 86400.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MaximumInterval(pub i64);

	impl ::std::ops::Deref for MaximumInterval {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<MaximumInterval> for i64 {
		fn from(value: MaximumInterval) -> Self {
			value.0
		}
	}

	impl From<&MaximumInterval> for MaximumInterval {
		fn from(value: &MaximumInterval) -> Self {
			value.clone()
		}
	}

	impl From<i64> for MaximumInterval {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for MaximumInterval {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for MaximumInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MaximumInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MaximumInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for MaximumInterval {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// Minimum interval between event reports.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Minimum interval between event reports.",
	///  "type": "integer",
	///  "maximum": 32767.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MinimumInterval(pub i64);

	impl ::std::ops::Deref for MinimumInterval {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<MinimumInterval> for i64 {
		fn from(value: MinimumInterval) -> Self {
			value.0
		}
	}

	impl From<&MinimumInterval> for MinimumInterval {
		fn from(value: &MinimumInterval) -> Self {
			value.clone()
		}
	}

	impl From<i64> for MinimumInterval {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for MinimumInterval {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for MinimumInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MinimumInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MinimumInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for MinimumInterval {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Contain Minor Location QoS.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contain Minor Location QoS.",
	///  "type": "object",
	///  "properties": {
	///    "hAccuracy": {
	///      "$ref": "#/components/schemas/schemas-Accuracy"
	///    },
	///    "vAccuracy": {
	///      "$ref": "#/components/schemas/schemas-Accuracy"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MinorLocationQoS {
		#[serde(rename = "hAccuracy", default, skip_serializing_if = "Option::is_none")]
		pub h_accuracy: Option<SchemasAccuracy>,
		#[serde(rename = "vAccuracy", default, skip_serializing_if = "Option::is_none")]
		pub v_accuracy: Option<SchemasAccuracy>,
	}

	impl From<&MinorLocationQoS> for MinorLocationQoS {
		fn from(value: &MinorLocationQoS) -> Self {
			value.clone()
		}
	}

	/// Represents a Mobility Management Context in UE Context
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a Mobility Management Context in UE
	/// Context",
	///  "type": "object",
	///  "required": [
	///    "accessType"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "allowedHomeNssai": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "allowedNssai": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "anN2ApId": {
	///      "type": "integer"
	///    },
	///    "epsNasSecurityMode": {
	///      "$ref": "#/components/schemas/EpsNasSecurityMode"
	///    },
	///    "expectedUEbehavior": {
	///      "$ref": "#/components/schemas/ExpectedUeBehavior"
	///    },
	///    "manAssiUeRadioCapId": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "n3IwfId": {
	///      "$ref": "#/components/schemas/GlobalRanNodeId"
	///    },
	///    "nasDownlinkCount": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "nasSecurityMode": {
	///      "$ref": "#/components/schemas/NasSecurityMode"
	///    },
	///    "nasUplinkCount": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "nsInstanceList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsiId"
	///      },
	///      "minItems": 1
	///    },
	///    "nssaaStatusList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NssaaStatus"
	///      },
	///      "minItems": 1
	///    },
	///    "nssaiMappingList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NssaiMapping"
	///      },
	///      "minItems": 1
	///    },
	///    "pendingNssaiMappingList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NssaiMapping"
	///      },
	///      "minItems": 1
	///    },
	///    "plmnAssiUeRadioCapId": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "s1UeNetworkCapability": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "tngfId": {
	///      "$ref": "#/components/schemas/GlobalRanNodeId"
	///    },
	///    "ucmfDicEntryId": {
	///      "type": "string"
	///    },
	///    "ueDifferentiationInfo": {
	///      "$ref": "#/components/schemas/UeDifferentiationInfo"
	///    },
	///    "ueSecurityCapability": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "uuaaMmStatus": {
	///      "$ref": "#/components/schemas/UuaaMmStatus"
	///    },
	///    "wagfId": {
	///      "$ref": "#/components/schemas/GlobalRanNodeId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MmContext {
		#[serde(rename = "accessType")]
		pub access_type: AccessType,
		#[serde(
			rename = "allowedHomeNssai",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_home_nssai: Vec<Snssai>,
		#[serde(
			rename = "allowedNssai",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub allowed_nssai: Vec<Snssai>,
		#[serde(rename = "anN2ApId", default, skip_serializing_if = "Option::is_none")]
		pub an_n2_ap_id: Option<i64>,
		#[serde(
			rename = "epsNasSecurityMode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_nas_security_mode: Option<EpsNasSecurityMode>,
		#[serde(
			rename = "expectedUEbehavior",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub expected_u_ebehavior: Option<ExpectedUeBehavior>,
		#[serde(
			rename = "manAssiUeRadioCapId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub man_assi_ue_radio_cap_id: Option<Bytes>,
		#[serde(rename = "n3IwfId", default, skip_serializing_if = "Option::is_none")]
		pub n3_iwf_id: Option<GlobalRanNodeId>,
		#[serde(
			rename = "nasDownlinkCount",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nas_downlink_count: Option<Uinteger>,
		#[serde(
			rename = "nasSecurityMode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nas_security_mode: Option<NasSecurityMode>,
		#[serde(
			rename = "nasUplinkCount",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nas_uplink_count: Option<Uinteger>,
		#[serde(
			rename = "nsInstanceList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ns_instance_list: Vec<NsiId>,
		#[serde(
			rename = "nssaaStatusList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nssaa_status_list: Vec<NssaaStatus>,
		#[serde(
			rename = "nssaiMappingList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub nssai_mapping_list: Vec<NssaiMapping>,
		#[serde(
			rename = "pendingNssaiMappingList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pending_nssai_mapping_list: Vec<NssaiMapping>,
		#[serde(
			rename = "plmnAssiUeRadioCapId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub plmn_assi_ue_radio_cap_id: Option<Bytes>,
		#[serde(
			rename = "s1UeNetworkCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub s1_ue_network_capability: Option<Bytes>,
		#[serde(rename = "tngfId", default, skip_serializing_if = "Option::is_none")]
		pub tngf_id: Option<GlobalRanNodeId>,
		#[serde(
			rename = "ucmfDicEntryId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ucmf_dic_entry_id: Option<String>,
		#[serde(
			rename = "ueDifferentiationInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_differentiation_info: Option<UeDifferentiationInfo>,
		#[serde(
			rename = "ueSecurityCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_security_capability: Option<Bytes>,
		#[serde(
			rename = "uuaaMmStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub uuaa_mm_status: Option<UuaaMmStatus>,
		#[serde(rename = "wagfId", default, skip_serializing_if = "Option::is_none")]
		pub wagf_id: Option<GlobalRanNodeId>,
	}

	impl From<&MmContext> for MmContext {
		fn from(value: &MmContext) -> Self {
			value.clone()
		}
	}

	/// UE MM Transaction Report Item per Location
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "UE MM Transaction Report Item per Location",
	///  "type": "object",
	///  "required": [
	///    "timestamp",
	///    "transactions"
	///  ],
	///  "properties": {
	///    "ecgi": {
	///      "$ref": "#/components/schemas/Ecgi"
	///    },
	///    "n3gaLocation": {
	///      "$ref": "#/components/schemas/N3gaLocation"
	///    },
	///    "ncgi": {
	///      "$ref": "#/components/schemas/Ncgi"
	///    },
	///    "tai": {
	///      "$ref": "#/components/schemas/Tai"
	///    },
	///    "timestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "transactions": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MmTransactionLocationReportItem {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ecgi: Option<Ecgi>,
		#[serde(
			rename = "n3gaLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n3ga_location: Option<N3gaLocation>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ncgi: Option<Ncgi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub tai: Option<Tai>,
		pub timestamp: DateTime,
		pub transactions: i64,
	}

	impl From<&MmTransactionLocationReportItem> for MmTransactionLocationReportItem {
		fn from(value: &MmTransactionLocationReportItem) -> Self {
			value.clone()
		}
	}

	/// UE MM Transaction Report Item per Slice
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "UE MM Transaction Report Item per Slice",
	///  "type": "object",
	///  "required": [
	///    "timestamp",
	///    "transactions"
	///  ],
	///  "properties": {
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "timestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "transactions": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MmTransactionSliceReportItem {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		pub timestamp: DateTime,
		pub transactions: i64,
	}

	impl From<&MmTransactionSliceReportItem> for MmTransactionSliceReportItem {
		fn from(value: &MmTransactionSliceReportItem) -> Self {
			value.clone()
		}
	}

	/// Indicates the information of motion based event reporting.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the information of motion based event
	/// reporting.",
	///  "type": "object",
	///  "required": [
	///    "linearDistance"
	///  ],
	///  "properties": {
	///    "linearDistance": {
	///      "$ref": "#/components/schemas/LinearDistance"
	///    },
	///    "maximumInterval": {
	///      "$ref": "#/components/schemas/MaximumInterval"
	///    },
	///    "minimumInterval": {
	///      "$ref": "#/components/schemas/MinimumInterval"
	///    },
	///    "occurrenceInfo": {
	///      "$ref": "#/components/schemas/OccurrenceInfo"
	///    },
	///    "reportingDuration": {
	///      "$ref": "#/components/schemas/ReportingDuration"
	///    },
	///    "reportingLocationReq": {
	///      "default": true,
	///      "type": "boolean"
	///    },
	///    "samplingInterval": {
	///      "$ref": "#/components/schemas/SamplingInterval"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct MotionEventInfo {
		#[serde(rename = "linearDistance")]
		pub linear_distance: LinearDistance,
		#[serde(
			rename = "maximumInterval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub maximum_interval: Option<MaximumInterval>,
		#[serde(
			rename = "minimumInterval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub minimum_interval: Option<MinimumInterval>,
		#[serde(
			rename = "occurrenceInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub occurrence_info: Option<OccurrenceInfo>,
		#[serde(
			rename = "reportingDuration",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reporting_duration: Option<ReportingDuration>,
		#[serde(
			rename = "reportingLocationReq",
			default = "defaults::default_bool::<true>"
		)]
		pub reporting_location_req: bool,
		#[serde(
			rename = "samplingInterval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub sampling_interval: Option<SamplingInterval>,
	}

	impl From<&MotionEventInfo> for MotionEventInfo {
		fn from(value: &MotionEventInfo) -> Self {
			value.clone()
		}
	}

	/// MsClassmark2
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
	pub struct MsClassmark2(pub Bytes);

	impl ::std::ops::Deref for MsClassmark2 {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<MsClassmark2> for Bytes {
		fn from(value: MsClassmark2) -> Self {
			value.0
		}
	}

	impl From<&MsClassmark2> for MsClassmark2 {
		fn from(value: &MsClassmark2) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for MsClassmark2 {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for MsClassmark2 {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for MsClassmark2 {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for MsClassmark2 {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for MsClassmark2 {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for MsClassmark2 {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// N1 Message container
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "N1 Message container",
	///  "type": "object",
	///  "required": [
	///    "n1MessageClass",
	///    "n1MessageContent"
	///  ],
	///  "properties": {
	///    "n1MessageClass": {
	///      "$ref": "#/components/schemas/N1MessageClass"
	///    },
	///    "n1MessageContent": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "nfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
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
	pub struct N1MessageContainer {
		#[serde(rename = "n1MessageClass")]
		pub n1_message_class: N1MessageClass,
		#[serde(rename = "n1MessageContent")]
		pub n1_message_content: RefToBinaryData,
		#[serde(rename = "nfId", default, skip_serializing_if = "Option::is_none")]
		pub nf_id: Option<NfInstanceId>,
		#[serde(
			rename = "serviceInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_instance_id: Option<String>,
	}

	impl From<&N1MessageContainer> for N1MessageContainer {
		fn from(value: &N1MessageContainer) -> Self {
			value.clone()
		}
	}

	/// Data within a N1 message notification request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a N1 message notification request",
	///  "type": "object",
	///  "required": [
	///    "n1MessageContainer"
	///  ],
	///  "properties": {
	///    "cIoT5GSOptimisation": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ecgi": {
	///      "$ref": "#/components/schemas/Ecgi"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "lcsCorrelationId": {
	///      "$ref": "#/components/schemas/CorrelationID"
	///    },
	///    "n1MessageContainer": {
	///      "$ref": "#/components/schemas/N1MessageContainer"
	///    },
	///    "n1NotifySubscriptionId": {
	///      "type": "string"
	///    },
	///    "ncgi": {
	///      "$ref": "#/components/schemas/Ncgi"
	///    },
	///    "newLmfIdentification": {
	///      "$ref": "#/components/schemas/LMFIdentification"
	///    },
	///    "registrationCtxtContainer": {
	///      "$ref": "#/components/schemas/RegistrationContextContainer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N1MessageNotification {
		#[serde(rename = "cIoT5GSOptimisation", default)]
		pub c_io_t5gs_optimisation: bool,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ecgi: Option<Ecgi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<crate::common::common_models::Guami>,
		#[serde(
			rename = "lcsCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lcs_correlation_id: Option<CorrelationId>,
		#[serde(rename = "n1MessageContainer")]
		pub n1_message_container: N1MessageContainer,
		#[serde(
			rename = "n1NotifySubscriptionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_notify_subscription_id: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ncgi: Option<Ncgi>,
		#[serde(
			rename = "newLmfIdentification",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub new_lmf_identification: Option<LmfIdentification>,
		#[serde(
			rename = "registrationCtxtContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub registration_ctxt_container: Option<RegistrationContextContainer>,
	}

	impl From<&N1MessageNotification> for N1MessageNotification {
		fn from(value: &N1MessageNotification) -> Self {
			value.clone()
		}
	}

	/// Enumeration for N1N2Message Transfer Cause
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Enumeration for N1N2Message Transfer Cause",
	///  "type": "string",
	///  "enum": [
	///    "ATTEMPTING_TO_REACH_UE",
	///    "N1_N2_TRANSFER_INITIATED",
	///    "WAITING_FOR_ASYNCHRONOUS_TRANSFER",
	///    "UE_NOT_RESPONDING",
	///    "N1_MSG_NOT_TRANSFERRED",
	///    "N2_MSG_NOT_TRANSFERRED",
	///    "UE_NOT_REACHABLE_FOR_SESSION",
	///    "TEMPORARY_REJECT_REGISTRATION_ONGOING",
	///    "TEMPORARY_REJECT_HANDOVER_ONGOING",
	///    "REJECTION_DUE_TO_PAGING_RESTRICTION",
	///    "AN_NOT_RESPONDING",
	///    "FAILURE_CAUSE_UNSPECIFIED"
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
	pub enum N1n2MessageTransferCause {
		#[default]
		#[serde(rename = "ATTEMPTING_TO_REACH_UE")]
		AttemptingToReachUe,
		#[serde(rename = "N1_N2_TRANSFER_INITIATED")]
		N1N2TransferInitiated,
		#[serde(rename = "WAITING_FOR_ASYNCHRONOUS_TRANSFER")]
		WaitingForAsynchronousTransfer,
		#[serde(rename = "UE_NOT_RESPONDING")]
		UeNotResponding,
		#[serde(rename = "N1_MSG_NOT_TRANSFERRED")]
		N1MsgNotTransferred,
		#[serde(rename = "N2_MSG_NOT_TRANSFERRED")]
		N2MsgNotTransferred,
		#[serde(rename = "UE_NOT_REACHABLE_FOR_SESSION")]
		UeNotReachableForSession,
		#[serde(rename = "TEMPORARY_REJECT_REGISTRATION_ONGOING")]
		TemporaryRejectRegistrationOngoing,
		#[serde(rename = "TEMPORARY_REJECT_HANDOVER_ONGOING")]
		TemporaryRejectHandoverOngoing,
		#[serde(rename = "REJECTION_DUE_TO_PAGING_RESTRICTION")]
		RejectionDueToPagingRestriction,
		#[serde(rename = "AN_NOT_RESPONDING")]
		AnNotResponding,
		#[serde(rename = "FAILURE_CAUSE_UNSPECIFIED")]
		FailureCauseUnspecified,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&N1n2MessageTransferCause> for N1n2MessageTransferCause {
		fn from(value: &N1n2MessageTransferCause) -> Self {
			value.clone()
		}
	}

	impl ToString for N1n2MessageTransferCause {
		fn to_string(&self) -> String {
			match *self {
				Self::AttemptingToReachUe => "ATTEMPTING_TO_REACH_UE".to_string(),
				Self::N1N2TransferInitiated => "N1_N2_TRANSFER_INITIATED".to_string(),
				Self::WaitingForAsynchronousTransfer => {
					"WAITING_FOR_ASYNCHRONOUS_TRANSFER".to_string()
				}
				Self::UeNotResponding => "UE_NOT_RESPONDING".to_string(),
				Self::N1MsgNotTransferred => "N1_MSG_NOT_TRANSFERRED".to_string(),
				Self::N2MsgNotTransferred => "N2_MSG_NOT_TRANSFERRED".to_string(),
				Self::UeNotReachableForSession => "UE_NOT_REACHABLE_FOR_SESSION".to_string(),
				Self::TemporaryRejectRegistrationOngoing => {
					"TEMPORARY_REJECT_REGISTRATION_ONGOING".to_string()
				}
				Self::TemporaryRejectHandoverOngoing => {
					"TEMPORARY_REJECT_HANDOVER_ONGOING".to_string()
				}
				Self::RejectionDueToPagingRestriction => {
					"REJECTION_DUE_TO_PAGING_RESTRICTION".to_string()
				}
				Self::AnNotResponding => "AN_NOT_RESPONDING".to_string(),
				Self::FailureCauseUnspecified => "FAILURE_CAUSE_UNSPECIFIED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for N1n2MessageTransferCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ATTEMPTING_TO_REACH_UE" => Ok(Self::AttemptingToReachUe),
				"N1_N2_TRANSFER_INITIATED" => Ok(Self::N1N2TransferInitiated),
				"WAITING_FOR_ASYNCHRONOUS_TRANSFER" => Ok(Self::WaitingForAsynchronousTransfer),
				"UE_NOT_RESPONDING" => Ok(Self::UeNotResponding),
				"N1_MSG_NOT_TRANSFERRED" => Ok(Self::N1MsgNotTransferred),
				"N2_MSG_NOT_TRANSFERRED" => Ok(Self::N2MsgNotTransferred),
				"UE_NOT_REACHABLE_FOR_SESSION" => Ok(Self::UeNotReachableForSession),
				"TEMPORARY_REJECT_REGISTRATION_ONGOING" => {
					Ok(Self::TemporaryRejectRegistrationOngoing)
				}
				"TEMPORARY_REJECT_HANDOVER_ONGOING" => Ok(Self::TemporaryRejectHandoverOngoing),
				"REJECTION_DUE_TO_PAGING_RESTRICTION" => Ok(Self::RejectionDueToPagingRestriction),
				"AN_NOT_RESPONDING" => Ok(Self::AnNotResponding),
				"FAILURE_CAUSE_UNSPECIFIED" => Ok(Self::FailureCauseUnspecified),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for N1n2MessageTransferCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for N1n2MessageTransferCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for N1n2MessageTransferCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within a N1/N2 Message Transfer Error response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a N1/N2 Message Transfer Error response",
	///  "type": "object",
	///  "required": [
	///    "error"
	///  ],
	///  "properties": {
	///    "errInfo": {
	///      "$ref": "#/components/schemas/N1N2MsgTxfrErrDetail"
	///    },
	///    "error": {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N1N2MessageTransferError {
		#[serde(rename = "errInfo", default, skip_serializing_if = "Option::is_none")]
		pub err_info: Option<N1n2MsgTxfrErrDetail>,
		pub error: ProblemDetails,
	}

	impl From<&N1N2MessageTransferError> for N1N2MessageTransferError {
		fn from(value: &N1N2MessageTransferError) -> Self {
			value.clone()
		}
	}

	/// Data within a N1/N2 message transfer request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a N1/N2 message transfer request",
	///  "type": "object",
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "areaOfValidity": {
	///      "$ref": "#/components/schemas/AreaOfValidity"
	///    },
	///    "arp": {
	///      "$ref": "#/components/schemas/Arp"
	///    },
	///    "extBufSupport": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "lastMsgIndication": {
	///      "type": "boolean"
	///    },
	///    "lcsCorrelationId": {
	///      "$ref": "#/components/schemas/CorrelationID"
	///    },
	///    "maAcceptedInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "mtData": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "n1MessageContainer": {
	///      "$ref": "#/components/schemas/N1MessageContainer"
	///    },
	///    "n1n2FailureTxfNotifURI": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "n2InfoContainer": {
	///      "$ref": "#/components/schemas/N2InfoContainer"
	///    },
	///    "nfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "oldGuami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "ppi": {
	///      "$ref": "#/components/schemas/Ppi"
	///    },
	///    "skipInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "smfReallocationInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "targetAccess": {
	///      "$ref": "#/components/schemas/AccessType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N1N2MessageTransferReqData {
		#[serde(
			rename = "areaOfValidity",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub area_of_validity: Option<AreaOfValidity>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub arp: Option<Arp>,
		#[serde(rename = "extBufSupport", default)]
		pub ext_buf_support: bool,
		#[serde(rename = "5qi", default, skip_serializing_if = "Option::is_none")]
		pub five_qi: Option<_5qi>,
		#[serde(
			rename = "lastMsgIndication",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub last_msg_indication: Option<bool>,
		#[serde(
			rename = "lcsCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lcs_correlation_id: Option<CorrelationId>,
		#[serde(rename = "maAcceptedInd", default)]
		pub ma_accepted_ind: bool,
		#[serde(rename = "mtData", default, skip_serializing_if = "Option::is_none")]
		pub mt_data: Option<RefToBinaryData>,
		#[serde(
			rename = "n1MessageContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_message_container: Option<N1MessageContainer>,
		#[serde(
			rename = "n1n2FailureTxfNotifURI",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1n2_failure_txf_notif_uri: Option<Uri>,
		#[serde(
			rename = "n2InfoContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_info_container: Option<N2InfoContainer>,
		#[serde(rename = "nfId", default, skip_serializing_if = "Option::is_none")]
		pub nf_id: Option<NfInstanceId>,
		#[serde(rename = "oldGuami", default, skip_serializing_if = "Option::is_none")]
		pub old_guami: Option<crate::common::common_models::Guami>,
		#[serde(
			rename = "pduSessionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_id: Option<PduSessionId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ppi: Option<Ppi>,
		#[serde(rename = "skipInd", default)]
		pub skip_ind: bool,
		#[serde(rename = "smfReallocationInd", default)]
		pub smf_reallocation_ind: bool,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "targetAccess",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_access: Option<AccessType>,
	}

	impl From<&N1N2MessageTransferReqData> for N1N2MessageTransferReqData {
		fn from(value: &N1N2MessageTransferReqData) -> Self {
			value.clone()
		}
	}

	/// Data within a N1/N2 message transfer response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a N1/N2 message transfer response",
	///  "type": "object",
	///  "required": [
	///    "cause"
	///  ],
	///  "properties": {
	///    "cause": {
	///      "$ref": "#/components/schemas/N1N2MessageTransferCause"
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
	pub struct N1N2MessageTransferRspData {
		pub cause: N1n2MessageTransferCause,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&N1N2MessageTransferRspData> for N1N2MessageTransferRspData {
		fn from(value: &N1N2MessageTransferRspData) -> Self {
			value.clone()
		}
	}

	/// N1/N2 Message Transfer Error Details
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "N1/N2 Message Transfer Error Details",
	///  "type": "object",
	///  "properties": {
	///    "highestPrioArp": {
	///      "$ref": "#/components/schemas/Arp"
	///    },
	///    "maxWaitingTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "retryAfter": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N1n2MsgTxfrErrDetail {
		#[serde(
			rename = "highestPrioArp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub highest_prio_arp: Option<Arp>,
		#[serde(
			rename = "maxWaitingTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_waiting_time: Option<DurationSec>,
		#[serde(
			rename = "retryAfter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub retry_after: Option<Uinteger>,
	}

	impl From<&N1n2MsgTxfrErrDetail> for N1n2MsgTxfrErrDetail {
		fn from(value: &N1n2MsgTxfrErrDetail) -> Self {
			value.clone()
		}
	}

	/// Data within a N1/N2 Message Transfer Failure Notification request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a N1/N2 Message Transfer Failure
	/// Notification request",
	///  "type": "object",
	///  "required": [
	///    "cause",
	///    "n1n2MsgDataUri"
	///  ],
	///  "properties": {
	///    "cause": {
	///      "$ref": "#/components/schemas/N1N2MessageTransferCause"
	///    },
	///    "n1n2MsgDataUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N1n2MsgTxfrFailureNotification {
		pub cause: N1n2MessageTransferCause,
		#[serde(rename = "n1n2MsgDataUri")]
		pub n1n2_msg_data_uri: Uri,
	}

	impl From<&N1n2MsgTxfrFailureNotification> for N1n2MsgTxfrFailureNotification {
		fn from(value: &N1n2MsgTxfrFailureNotification) -> Self {
			value.clone()
		}
	}

	/// N2 information container
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "N2 information container",
	///  "type": "object",
	///  "required": [
	///    "n2InformationClass"
	///  ],
	///  "properties": {
	///    "n2InformationClass": {
	///      "$ref": "#/components/schemas/N2InformationClass"
	///    },
	///    "nrppaInfo": {
	///      "$ref": "#/components/schemas/NrppaInformation"
	///    },
	///    "proseInfo": {
	///      "$ref": "#/components/schemas/ProSeInformation"
	///    },
	///    "pwsInfo": {
	///      "$ref": "#/components/schemas/PwsInformation"
	///    },
	///    "ranInfo": {
	///      "$ref": "#/components/schemas/N2RanInformation"
	///    },
	///    "smInfo": {
	///      "$ref": "#/components/schemas/N2SmInformation"
	///    },
	///    "v2xInfo": {
	///      "$ref": "#/components/schemas/V2xInformation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N2InfoContainer {
		#[serde(rename = "n2InformationClass")]
		pub n2_information_class: N2InformationClass,
		#[serde(rename = "nrppaInfo", default, skip_serializing_if = "Option::is_none")]
		pub nrppa_info: Option<NrppaInformation>,
		#[serde(rename = "proseInfo", default, skip_serializing_if = "Option::is_none")]
		pub prose_info: Option<ProSeInformation>,
		#[serde(rename = "pwsInfo", default, skip_serializing_if = "Option::is_none")]
		pub pws_info: Option<PwsInformation>,
		#[serde(rename = "ranInfo", default, skip_serializing_if = "Option::is_none")]
		pub ran_info: Option<N2RanInformation>,
		#[serde(rename = "smInfo", default, skip_serializing_if = "Option::is_none")]
		pub sm_info: Option<N2SmInformation>,
		#[serde(rename = "v2xInfo", default, skip_serializing_if = "Option::is_none")]
		pub v2x_info: Option<V2xInformation>,
	}

	impl From<&N2InfoContainer> for N2InfoContainer {
		fn from(value: &N2InfoContainer) -> Self {
			value.clone()
		}
	}

	/// Represents a transparent N2 information content to be relayed by AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a transparent N2 information content to be
	/// relayed by AMF",
	///  "type": "object",
	///  "required": [
	///    "ngapData"
	///  ],
	///  "properties": {
	///    "ngapData": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "ngapIeType": {
	///      "$ref": "#/components/schemas/NgapIeType"
	///    },
	///    "ngapMessageType": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N2InfoContent {
		#[serde(rename = "ngapData")]
		pub ngap_data: RefToBinaryData,
		#[serde(
			rename = "ngapIeType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ngap_ie_type: Option<NgapIeType>,
		#[serde(
			rename = "ngapMessageType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ngap_message_type: Option<Uinteger>,
	}

	impl From<&N2InfoContent> for N2InfoContent {
		fn from(value: &N2InfoContent) -> Self {
			value.clone()
		}
	}

	/// Data within a N2 information notification response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a N2 information notification response",
	///  "type": "object",
	///  "properties": {
	///    "secRatDataUsageList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/N2SmInformation"
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
	pub struct N2InfoNotificationRspData {
		#[serde(
			rename = "secRatDataUsageList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub sec_rat_data_usage_list: Vec<N2SmInformation>,
	}

	impl From<&N2InfoNotificationRspData> for N2InfoNotificationRspData {
		fn from(value: &N2InfoNotificationRspData) -> Self {
			value.clone()
		}
	}

	/// N2 Information Notify Reason
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "N2 Information Notify Reason",
	///  "type": "string",
	///  "enum": [
	///    "HANDOVER_COMPLETED"
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
	pub enum N2InfoNotifyReason {
		#[default]
		#[serde(rename = "HANDOVER_COMPLETED")]
		HandoverCompleted,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&N2InfoNotifyReason> for N2InfoNotifyReason {
		fn from(value: &N2InfoNotifyReason) -> Self {
			value.clone()
		}
	}

	impl ToString for N2InfoNotifyReason {
		fn to_string(&self) -> String {
			match *self {
				Self::HandoverCompleted => "HANDOVER_COMPLETED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for N2InfoNotifyReason {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"HANDOVER_COMPLETED" => Ok(Self::HandoverCompleted),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for N2InfoNotifyReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for N2InfoNotifyReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for N2InfoNotifyReason {
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

	/// Data within a N2 information notification request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a N2 information notification request",
	///  "type": "object",
	///  "required": [
	///    "n2NotifySubscriptionId"
	///  ],
	///  "properties": {
	///    "anN2IPv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "anN2IPv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "guami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "initialAmfName": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "lcsCorrelationId": {
	///      "$ref": "#/components/schemas/CorrelationID"
	///    },
	///    "n2InfoContainer": {
	///      "$ref": "#/components/schemas/N2InfoContainer"
	///    },
	///    "n2NotifySubscriptionId": {
	///      "type": "string"
	///    },
	///    "notifyReason": {
	///      "$ref": "#/components/schemas/N2InfoNotifyReason"
	///    },
	///    "notifySourceNgRan": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ranNodeId": {
	///      "$ref": "#/components/schemas/GlobalRanNodeId"
	///    },
	///    "smfChangeInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SmfChangeInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "toReleaseSessionList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionId"
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
	pub struct N2InformationNotification {
		#[serde(
			rename = "anN2IPv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub an_n2i_pv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "anN2IPv6Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub an_n2i_pv6_addr: Option<Ipv6Addr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub guami: Option<crate::common::common_models::Guami>,
		#[serde(
			rename = "initialAmfName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub initial_amf_name: Option<Fqdn>,
		#[serde(
			rename = "lcsCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lcs_correlation_id: Option<CorrelationId>,
		#[serde(
			rename = "n2InfoContainer",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_info_container: Option<N2InfoContainer>,
		#[serde(rename = "n2NotifySubscriptionId")]
		pub n2_notify_subscription_id: String,
		#[serde(
			rename = "notifyReason",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notify_reason: Option<N2InfoNotifyReason>,
		#[serde(rename = "notifySourceNgRan", default)]
		pub notify_source_ng_ran: bool,
		#[serde(rename = "ranNodeId", default, skip_serializing_if = "Option::is_none")]
		pub ran_node_id: Option<GlobalRanNodeId>,
		#[serde(
			rename = "smfChangeInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub smf_change_info_list: Vec<SmfChangeInfo>,
		#[serde(
			rename = "toReleaseSessionList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub to_release_session_list: Vec<PduSessionId>,
	}

	impl From<&N2InformationNotification> for N2InformationNotification {
		fn from(value: &N2InformationNotification) -> Self {
			value.clone()
		}
	}

	/// Data within a failure response for a non-UE related N2 Information
	/// Transfer
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a failure response for a non-UE related N2
	/// Information Transfer",
	///  "type": "object",
	///  "required": [
	///    "error"
	///  ],
	///  "properties": {
	///    "error": {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    "pwsErrorInfo": {
	///      "$ref": "#/components/schemas/PWSErrorData"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N2InformationTransferError {
		pub error: ProblemDetails,
		#[serde(
			rename = "pwsErrorInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pws_error_info: Option<PwsErrorData>,
	}

	impl From<&N2InformationTransferError> for N2InformationTransferError {
		fn from(value: &N2InformationTransferError) -> Self {
			value.clone()
		}
	}

	/// Data within a N2 Information Transfer request containing the N2
	/// information requested to be transferred to 5G AN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a N2 Information Transfer request
	/// containing the N2 information requested to be transferred to 5G AN",
	///  "type": "object",
	///  "required": [
	///    "n2Information"
	///  ],
	///  "properties": {
	///    "globalRanNodeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GlobalRanNodeId"
	///      },
	///      "minItems": 1
	///    },
	///    "n2Information": {
	///      "$ref": "#/components/schemas/N2InfoContainer"
	///    },
	///    "ratSelector": {
	///      "$ref": "#/components/schemas/RatSelector"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
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
	pub struct N2InformationTransferReqData {
		#[serde(
			rename = "globalRanNodeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub global_ran_node_list: Vec<GlobalRanNodeId>,
		#[serde(rename = "n2Information")]
		pub n2_information: N2InfoContainer,
		#[serde(
			rename = "ratSelector",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rat_selector: Option<RatSelector>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
	}

	impl From<&N2InformationTransferReqData> for N2InformationTransferReqData {
		fn from(value: &N2InformationTransferReqData) -> Self {
			value.clone()
		}
	}

	/// Describes the result of N2 information transfer by AMF to the AN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the result of N2 information transfer by AMF
	/// to the AN",
	///  "type": "string",
	///  "enum": [
	///    "N2_INFO_TRANSFER_INITIATED"
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
	pub enum N2InformationTransferResult {
		#[default]
		#[serde(rename = "N2_INFO_TRANSFER_INITIATED")]
		N2InfoTransferInitiated,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&N2InformationTransferResult> for N2InformationTransferResult {
		fn from(value: &N2InformationTransferResult) -> Self {
			value.clone()
		}
	}

	impl ToString for N2InformationTransferResult {
		fn to_string(&self) -> String {
			match *self {
				Self::N2InfoTransferInitiated => "N2_INFO_TRANSFER_INITIATED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for N2InformationTransferResult {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"N2_INFO_TRANSFER_INITIATED" => Ok(Self::N2InfoTransferInitiated),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for N2InformationTransferResult {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for N2InformationTransferResult {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for N2InformationTransferResult {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within a successful response to the N2 Information Transfer request
	/// to transfer N2 Information to the AN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a successful response to the N2 Information
	/// Transfer request to transfer N2 Information to the AN",
	///  "type": "object",
	///  "required": [
	///    "result"
	///  ],
	///  "properties": {
	///    "pwsRspData": {
	///      "$ref": "#/components/schemas/PWSResponseData"
	///    },
	///    "result": {
	///      "$ref": "#/components/schemas/N2InformationTransferResult"
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
	pub struct N2InformationTransferRspData {
		#[serde(
			rename = "pwsRspData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pws_rsp_data: Option<PwsResponseData>,
		pub result: N2InformationTransferResult,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&N2InformationTransferRspData> for N2InformationTransferRspData {
		fn from(value: &N2InformationTransferRspData) -> Self {
			value.clone()
		}
	}

	/// Represents the RAN related N2 information data part
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the RAN related N2 information data part",
	///  "type": "object",
	///  "required": [
	///    "n2InfoContent"
	///  ],
	///  "properties": {
	///    "n2InfoContent": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N2RanInformation {
		#[serde(rename = "n2InfoContent")]
		pub n2_info_content: N2InfoContent,
	}

	impl From<&N2RanInformation> for N2RanInformation {
		fn from(value: &N2RanInformation) -> Self {
			value.clone()
		}
	}

	/// Represents the session management SMF related N2 information data part
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the session management SMF related N2
	/// information data part",
	///  "type": "object",
	///  "required": [
	///    "pduSessionId"
	///  ],
	///  "properties": {
	///    "homePlmnSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "iwkSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "n2InfoContent": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "subjectToHo": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct N2SmInformation {
		#[serde(
			rename = "homePlmnSnssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub home_plmn_snssai: Option<Snssai>,
		#[serde(rename = "iwkSnssai", default, skip_serializing_if = "Option::is_none")]
		pub iwk_snssai: Option<Snssai>,
		#[serde(
			rename = "n2InfoContent",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_info_content: Option<N2InfoContent>,
		#[serde(rename = "pduSessionId")]
		pub pdu_session_id: PduSessionId,
		#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
		pub s_nssai: Option<Snssai>,
		#[serde(
			rename = "subjectToHo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subject_to_ho: Option<bool>,
	}

	impl From<&N2SmInformation> for N2SmInformation {
		fn from(value: &N2SmInformation) -> Self {
			value.clone()
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

	/// NasCount
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
	pub struct NasCount(pub Uinteger);

	impl ::std::ops::Deref for NasCount {
		type Target = Uinteger;
		fn deref(&self) -> &Uinteger {
			&self.0
		}
	}

	impl From<NasCount> for Uinteger {
		fn from(value: NasCount) -> Self {
			value.0
		}
	}

	impl From<&NasCount> for NasCount {
		fn from(value: &NasCount) -> Self {
			value.clone()
		}
	}

	impl From<Uinteger> for NasCount {
		fn from(value: Uinteger) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for NasCount {
		type Err = <Uinteger as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for NasCount {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NasCount {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NasCount {
		type Error = <Uinteger as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for NasCount {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Indicates the NAS Security Mode
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the NAS Security Mode",
	///  "type": "object",
	///  "required": [
	///    "cipheringAlgorithm",
	///    "integrityAlgorithm"
	///  ],
	///  "properties": {
	///    "cipheringAlgorithm": {
	///      "$ref": "#/components/schemas/CipheringAlgorithm"
	///    },
	///    "integrityAlgorithm": {
	///      "$ref": "#/components/schemas/IntegrityAlgorithm"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NasSecurityMode {
		#[serde(rename = "cipheringAlgorithm")]
		pub ciphering_algorithm: CipheringAlgorithm,
		#[serde(rename = "integrityAlgorithm")]
		pub integrity_algorithm: IntegrityAlgorithm,
	}

	impl From<&NasSecurityMode> for NasSecurityMode {
		fn from(value: &NasSecurityMode) -> Self {
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

	/// Represents the network performance information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the network performance information.",
	///  "type": "object",
	///  "allOf": [
	///    {
	///      "required": [
	///        "networkArea"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "nwPerfType"
	///      ]
	///    },
	///    {
	///      "oneOf": [
	///        {
	///          "required": [
	///            "relativeRatio"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "absoluteNum"
	///          ]
	///        }
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "absoluteNum": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "networkArea": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "nwPerfType": {
	///      "$ref": "#/components/schemas/NetworkPerfType"
	///    },
	///    "relativeRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum NetworkPerfInfo {
		#[default]
		Variant0 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "networkArea")]
			network_area: NetworkAreaInfo,
			#[serde(rename = "nwPerfType")]
			nw_perf_type: NetworkPerfType,
			#[serde(rename = "relativeRatio")]
			relative_ratio: SamplingRatio,
		},
		Variant1 {
			#[serde(rename = "absoluteNum")]
			absolute_num: Uinteger,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "networkArea")]
			network_area: NetworkAreaInfo,
			#[serde(rename = "nwPerfType")]
			nw_perf_type: NetworkPerfType,
		},
	}

	impl From<&NetworkPerfInfo> for NetworkPerfInfo {
		fn from(value: &NetworkPerfInfo) -> Self {
			value.clone()
		}
	}

	/// Represents a network performance requirement.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a network performance requirement.",
	///  "type": "object",
	///  "required": [
	///    "nwPerfType"
	///  ],
	///  "properties": {
	///    "absoluteNum": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "nwPerfType": {
	///      "$ref": "#/components/schemas/NetworkPerfType"
	///    },
	///    "relativeRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NetworkPerfRequirement {
		#[serde(
			rename = "absoluteNum",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub absolute_num: Option<Uinteger>,
		#[serde(rename = "nwPerfType")]
		pub nw_perf_type: NetworkPerfType,
		#[serde(
			rename = "relativeRatio",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub relative_ratio: Option<SamplingRatio>,
	}

	impl From<&NetworkPerfRequirement> for NetworkPerfRequirement {
		fn from(value: &NetworkPerfRequirement) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - GNB_ACTIVE_RATIO: Indicates that the network performance requirement
	///   is gNodeB active (i.e. up and running) rate. Indicates the ratio of
	///   gNB active (i.e. up and running) number to the total number of gNB
	/// - GNB_COMPUTING_USAGE: Indicates gNodeB computing resource usage.
	/// - GNB_MEMORY_USAGE: Indicates gNodeB memory usage.
	/// - GNB_DISK_USAGE: Indicates gNodeB disk usage.
	/// - NUM_OF_UE: Indicates number of UEs.
	/// - SESS_SUCC_RATIO: Indicates ratio of successful setup of PDU sessions
	///   to total PDU session setup attempts.
	/// - HO_SUCC_RATIO: Indicates Ratio of successful handovers to the total
	///   handover attempts.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- GNB_ACTIVE_RATIO: Indicates
	/// that the network performance requirement is gNodeB active (i.e. up and
	/// running) rate. Indicates the ratio of gNB active (i.e. up and running)
	/// number to the total number of gNB\n- GNB_COMPUTING_USAGE: Indicates
	/// gNodeB computing resource usage.\n- GNB_MEMORY_USAGE: Indicates gNodeB
	/// memory usage.\n- GNB_DISK_USAGE: Indicates gNodeB disk usage.\n-
	/// NUM_OF_UE: Indicates number of UEs.\n- SESS_SUCC_RATIO: Indicates ratio
	/// of successful setup of PDU sessions to total PDU session setup
	/// attempts.\n- HO_SUCC_RATIO: Indicates Ratio of successful handovers to
	/// the total handover attempts.\n",
	///  "type": "string",
	///  "enum": [
	///    "GNB_ACTIVE_RATIO",
	///    "GNB_COMPUTING_USAGE",
	///    "GNB_MEMORY_USAGE",
	///    "GNB_DISK_USAGE",
	///    "NUM_OF_UE",
	///    "SESS_SUCC_RATIO",
	///    "HO_SUCC_RATIO"
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
	pub enum NetworkPerfType {
		#[default]
		#[serde(rename = "GNB_ACTIVE_RATIO")]
		GnbActiveRatio,
		#[serde(rename = "GNB_COMPUTING_USAGE")]
		GnbComputingUsage,
		#[serde(rename = "GNB_MEMORY_USAGE")]
		GnbMemoryUsage,
		#[serde(rename = "GNB_DISK_USAGE")]
		GnbDiskUsage,
		#[serde(rename = "NUM_OF_UE")]
		NumOfUe,
		#[serde(rename = "SESS_SUCC_RATIO")]
		SessSuccRatio,
		#[serde(rename = "HO_SUCC_RATIO")]
		HoSuccRatio,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NetworkPerfType> for NetworkPerfType {
		fn from(value: &NetworkPerfType) -> Self {
			value.clone()
		}
	}

	impl ToString for NetworkPerfType {
		fn to_string(&self) -> String {
			match *self {
				Self::GnbActiveRatio => "GNB_ACTIVE_RATIO".to_string(),
				Self::GnbComputingUsage => "GNB_COMPUTING_USAGE".to_string(),
				Self::GnbMemoryUsage => "GNB_MEMORY_USAGE".to_string(),
				Self::GnbDiskUsage => "GNB_DISK_USAGE".to_string(),
				Self::NumOfUe => "NUM_OF_UE".to_string(),
				Self::SessSuccRatio => "SESS_SUCC_RATIO".to_string(),
				Self::HoSuccRatio => "HO_SUCC_RATIO".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NetworkPerfType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"GNB_ACTIVE_RATIO" => Ok(Self::GnbActiveRatio),
				"GNB_COMPUTING_USAGE" => Ok(Self::GnbComputingUsage),
				"GNB_MEMORY_USAGE" => Ok(Self::GnbMemoryUsage),
				"GNB_DISK_USAGE" => Ok(Self::GnbDiskUsage),
				"NUM_OF_UE" => Ok(Self::NumOfUe),
				"SESS_SUCC_RATIO" => Ok(Self::SessSuccRatio),
				"HO_SUCC_RATIO" => Ok(Self::HoSuccRatio),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NetworkPerfType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NetworkPerfType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NetworkPerfType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents load level information of a given NF instance.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents load level information of a given NF
	/// instance.",
	///  "type": "object",
	///  "allOf": [
	///    {
	///      "required": [
	///        "nfType"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "nfInstanceId"
	///      ]
	///    },
	///    {
	///      "anyOf": [
	///        {
	///          "required": [
	///            "nfStatus"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "nfCpuUsage"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "nfMemoryUsage"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "nfStorageUsage"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "nfLoadLevelAverage"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "nfLoadLevelPeak"
	///          ]
	///        }
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "nfCpuUsage": {
	///      "type": "integer"
	///    },
	///    "nfInstanceId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nfLoadAvgInAoi": {
	///      "type": "integer"
	///    },
	///    "nfLoadLevelAverage": {
	///      "type": "integer"
	///    },
	///    "nfLoadLevelpeak": {
	///      "type": "integer"
	///    },
	///    "nfMemoryUsage": {
	///      "type": "integer"
	///    },
	///    "nfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "nfStatus": {
	///      "$ref": "#/components/schemas/NfStatus"
	///    },
	///    "nfStorageUsage": {
	///      "type": "integer"
	///    },
	///    "nfType": {
	///      "$ref": "#/components/schemas/NFType"
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
	#[serde(untagged)]
	pub enum NfLoadLevelInformation {
		#[default]
		Variant0 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "nfInstanceId")]
			nf_instance_id: NfInstanceId,
			#[serde(
				rename = "nfLoadAvgInAoi",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_avg_in_aoi: Option<i64>,
			#[serde(
				rename = "nfLoadLevelpeak",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_levelpeak: Option<i64>,
			#[serde(rename = "nfSetId", default, skip_serializing_if = "Option::is_none")]
			nf_set_id: Option<NfSetId>,
			#[serde(rename = "nfStatus")]
			nf_status: NfStatus,
			#[serde(rename = "nfType")]
			nf_type: NfType,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
		},
		Variant1 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "nfCpuUsage")]
			nf_cpu_usage: i64,
			#[serde(rename = "nfInstanceId")]
			nf_instance_id: NfInstanceId,
			#[serde(
				rename = "nfLoadAvgInAoi",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_avg_in_aoi: Option<i64>,
			#[serde(
				rename = "nfLoadLevelpeak",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_levelpeak: Option<i64>,
			#[serde(rename = "nfSetId", default, skip_serializing_if = "Option::is_none")]
			nf_set_id: Option<NfSetId>,
			#[serde(rename = "nfType")]
			nf_type: NfType,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
		},
		Variant2 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "nfInstanceId")]
			nf_instance_id: NfInstanceId,
			#[serde(
				rename = "nfLoadAvgInAoi",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_avg_in_aoi: Option<i64>,
			#[serde(
				rename = "nfLoadLevelpeak",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_levelpeak: Option<i64>,
			#[serde(rename = "nfMemoryUsage")]
			nf_memory_usage: i64,
			#[serde(rename = "nfSetId", default, skip_serializing_if = "Option::is_none")]
			nf_set_id: Option<NfSetId>,
			#[serde(rename = "nfType")]
			nf_type: NfType,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
		},
		Variant3 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "nfInstanceId")]
			nf_instance_id: NfInstanceId,
			#[serde(
				rename = "nfLoadAvgInAoi",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_avg_in_aoi: Option<i64>,
			#[serde(
				rename = "nfLoadLevelpeak",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_levelpeak: Option<i64>,
			#[serde(rename = "nfSetId", default, skip_serializing_if = "Option::is_none")]
			nf_set_id: Option<NfSetId>,
			#[serde(rename = "nfStorageUsage")]
			nf_storage_usage: i64,
			#[serde(rename = "nfType")]
			nf_type: NfType,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
		},
		Variant4 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "nfInstanceId")]
			nf_instance_id: NfInstanceId,
			#[serde(
				rename = "nfLoadAvgInAoi",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_avg_in_aoi: Option<i64>,
			#[serde(rename = "nfLoadLevelAverage")]
			nf_load_level_average: i64,
			#[serde(
				rename = "nfLoadLevelpeak",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_levelpeak: Option<i64>,
			#[serde(rename = "nfSetId", default, skip_serializing_if = "Option::is_none")]
			nf_set_id: Option<NfSetId>,
			#[serde(rename = "nfType")]
			nf_type: NfType,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
		},
		Variant5 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "nfInstanceId")]
			nf_instance_id: NfInstanceId,
			#[serde(
				rename = "nfLoadAvgInAoi",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_avg_in_aoi: Option<i64>,
			#[serde(rename = "nfLoadLevelPeak")]
			nf_load_level_peak: ::serde_json::Value,
			#[serde(
				rename = "nfLoadLevelpeak",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			nf_load_levelpeak: Option<i64>,
			#[serde(rename = "nfSetId", default, skip_serializing_if = "Option::is_none")]
			nf_set_id: Option<NfSetId>,
			#[serde(rename = "nfType")]
			nf_type: NfType,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
		},
	}

	impl From<&NfLoadLevelInformation> for NfLoadLevelInformation {
		fn from(value: &NfLoadLevelInformation) -> Self {
			value.clone()
		}
	}

	/// Contains the percentage of time spent on various NF states.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the percentage of time spent on various NF
	/// states.",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "statusRegistered"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "statusUnregistered"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "statusUndiscoverable"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "statusRegistered": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "statusUndiscoverable": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "statusUnregistered": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum NfStatus {
		#[default]
		Variant0 {
			#[serde(rename = "statusRegistered")]
			status_registered: SamplingRatio,
		},
		Variant1 {
			#[serde(rename = "statusUnregistered")]
			status_unregistered: SamplingRatio,
		},
		Variant2 {
			#[serde(rename = "statusUndiscoverable")]
			status_undiscoverable: SamplingRatio,
		},
	}

	impl From<&NfStatus> for NfStatus {
		fn from(value: &NfStatus) -> Self {
			value.clone()
		}
	}

	/// Represents the ngKSI
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the ngKSI",
	///  "type": "object",
	///  "required": [
	///    "ksi",
	///    "tsc"
	///  ],
	///  "properties": {
	///    "ksi": {
	///      "type": "integer",
	///      "maximum": 6.0,
	///      "minimum": 0.0
	///    },
	///    "tsc": {
	///      "$ref": "#/components/schemas/ScType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NgKsi {
		pub ksi: i64,
		pub tsc: ScType,
	}

	impl From<&NgKsi> for NgKsi {
		fn from(value: &NgKsi) -> Self {
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

	/// Indicates the supported NGAP IE types
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the supported NGAP IE types",
	///  "type": "string",
	///  "enum": [
	///    "PDU_RES_SETUP_REQ",
	///    "PDU_RES_REL_CMD",
	///    "PDU_RES_MOD_REQ",
	///    "HANDOVER_CMD",
	///    "HANDOVER_REQUIRED",
	///    "HANDOVER_PREP_FAIL",
	///    "SRC_TO_TAR_CONTAINER",
	///    "TAR_TO_SRC_CONTAINER",
	///    "TAR_TO_SRC_FAIL_CONTAINER",
	///    "RAN_STATUS_TRANS_CONTAINER",
	///    "SON_CONFIG_TRANSFER",
	///    "NRPPA_PDU",
	///    "UE_RADIO_CAPABILITY",
	///    "RIM_INFO_TRANSFER",
	///    "SECONDARY_RAT_USAGE",
	///    "PC5_QOS_PARA",
	///    "EARLY_STATUS_TRANS_CONTAINER",
	///    "UE_RADIO_CAPABILITY_FOR_PAGING"
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
	pub enum NgapIeType {
		#[default]
		#[serde(rename = "PDU_RES_SETUP_REQ")]
		PduResSetupReq,
		#[serde(rename = "PDU_RES_REL_CMD")]
		PduResRelCmd,
		#[serde(rename = "PDU_RES_MOD_REQ")]
		PduResModReq,
		#[serde(rename = "HANDOVER_CMD")]
		HandoverCmd,
		#[serde(rename = "HANDOVER_REQUIRED")]
		HandoverRequired,
		#[serde(rename = "HANDOVER_PREP_FAIL")]
		HandoverPrepFail,
		#[serde(rename = "SRC_TO_TAR_CONTAINER")]
		SrcToTarContainer,
		#[serde(rename = "TAR_TO_SRC_CONTAINER")]
		TarToSrcContainer,
		#[serde(rename = "TAR_TO_SRC_FAIL_CONTAINER")]
		TarToSrcFailContainer,
		#[serde(rename = "RAN_STATUS_TRANS_CONTAINER")]
		RanStatusTransContainer,
		#[serde(rename = "SON_CONFIG_TRANSFER")]
		SonConfigTransfer,
		#[serde(rename = "NRPPA_PDU")]
		NrppaPdu,
		#[serde(rename = "UE_RADIO_CAPABILITY")]
		UeRadioCapability,
		#[serde(rename = "RIM_INFO_TRANSFER")]
		RimInfoTransfer,
		#[serde(rename = "SECONDARY_RAT_USAGE")]
		SecondaryRatUsage,
		#[serde(rename = "PC5_QOS_PARA")]
		Pc5QosPara,
		#[serde(rename = "EARLY_STATUS_TRANS_CONTAINER")]
		EarlyStatusTransContainer,
		#[serde(rename = "UE_RADIO_CAPABILITY_FOR_PAGING")]
		UeRadioCapabilityForPaging,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NgapIeType> for NgapIeType {
		fn from(value: &NgapIeType) -> Self {
			value.clone()
		}
	}

	impl ToString for NgapIeType {
		fn to_string(&self) -> String {
			match *self {
				Self::PduResSetupReq => "PDU_RES_SETUP_REQ".to_string(),
				Self::PduResRelCmd => "PDU_RES_REL_CMD".to_string(),
				Self::PduResModReq => "PDU_RES_MOD_REQ".to_string(),
				Self::HandoverCmd => "HANDOVER_CMD".to_string(),
				Self::HandoverRequired => "HANDOVER_REQUIRED".to_string(),
				Self::HandoverPrepFail => "HANDOVER_PREP_FAIL".to_string(),
				Self::SrcToTarContainer => "SRC_TO_TAR_CONTAINER".to_string(),
				Self::TarToSrcContainer => "TAR_TO_SRC_CONTAINER".to_string(),
				Self::TarToSrcFailContainer => "TAR_TO_SRC_FAIL_CONTAINER".to_string(),
				Self::RanStatusTransContainer => "RAN_STATUS_TRANS_CONTAINER".to_string(),
				Self::SonConfigTransfer => "SON_CONFIG_TRANSFER".to_string(),
				Self::NrppaPdu => "NRPPA_PDU".to_string(),
				Self::UeRadioCapability => "UE_RADIO_CAPABILITY".to_string(),
				Self::RimInfoTransfer => "RIM_INFO_TRANSFER".to_string(),
				Self::SecondaryRatUsage => "SECONDARY_RAT_USAGE".to_string(),
				Self::Pc5QosPara => "PC5_QOS_PARA".to_string(),
				Self::EarlyStatusTransContainer => "EARLY_STATUS_TRANS_CONTAINER".to_string(),
				Self::UeRadioCapabilityForPaging => "UE_RADIO_CAPABILITY_FOR_PAGING".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NgapIeType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PDU_RES_SETUP_REQ" => Ok(Self::PduResSetupReq),
				"PDU_RES_REL_CMD" => Ok(Self::PduResRelCmd),
				"PDU_RES_MOD_REQ" => Ok(Self::PduResModReq),
				"HANDOVER_CMD" => Ok(Self::HandoverCmd),
				"HANDOVER_REQUIRED" => Ok(Self::HandoverRequired),
				"HANDOVER_PREP_FAIL" => Ok(Self::HandoverPrepFail),
				"SRC_TO_TAR_CONTAINER" => Ok(Self::SrcToTarContainer),
				"TAR_TO_SRC_CONTAINER" => Ok(Self::TarToSrcContainer),
				"TAR_TO_SRC_FAIL_CONTAINER" => Ok(Self::TarToSrcFailContainer),
				"RAN_STATUS_TRANS_CONTAINER" => Ok(Self::RanStatusTransContainer),
				"SON_CONFIG_TRANSFER" => Ok(Self::SonConfigTransfer),
				"NRPPA_PDU" => Ok(Self::NrppaPdu),
				"UE_RADIO_CAPABILITY" => Ok(Self::UeRadioCapability),
				"RIM_INFO_TRANSFER" => Ok(Self::RimInfoTransfer),
				"SECONDARY_RAT_USAGE" => Ok(Self::SecondaryRatUsage),
				"PC5_QOS_PARA" => Ok(Self::Pc5QosPara),
				"EARLY_STATUS_TRANS_CONTAINER" => Ok(Self::EarlyStatusTransContainer),
				"UE_RADIO_CAPABILITY_FOR_PAGING" => Ok(Self::UeRadioCapabilityForPaging),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NgapIeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NgapIeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NgapIeType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents an Individual NWDAF Event Subscription resource.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an Individual NWDAF Event Subscription
	/// resource.",
	///  "type": "object",
	///  "required": [
	///    "eventSubscriptions"
	///  ],
	///  "properties": {
	///    "consNfInfo": {
	///      "$ref": "#/components/schemas/ConsumerNfInformation"
	///    },
	///    "eventNotifications": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EventNotification"
	///      },
	///      "minItems": 1
	///    },
	///    "eventSubscriptions": {
	///      "description": "Subscribed events",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/EventSubscription"
	///      },
	///      "minItems": 1
	///    },
	///    "evtReq": {
	///      "$ref": "#/components/schemas/ReportingInformation"
	///    },
	///    "failEventReports": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/FailureEventInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "notifCorrId": {
	///      "description": "Notification correlation identifier.",
	///      "type": "string"
	///    },
	///    "notificationURI": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "prevSub": {
	///      "$ref": "#/components/schemas/PrevSubInfo"
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
	pub struct NnwdafEventsSubscription {
		#[serde(
			rename = "consNfInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cons_nf_info: Option<ConsumerNfInformation>,
		#[serde(
			rename = "eventNotifications",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub event_notifications: Vec<EventNotification>,
		/// Subscribed events
		#[serde(rename = "eventSubscriptions")]
		pub event_subscriptions: Vec<EventSubscription>,
		#[serde(rename = "evtReq", default, skip_serializing_if = "Option::is_none")]
		pub evt_req: Option<ReportingInformation>,
		#[serde(
			rename = "failEventReports",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub fail_event_reports: Vec<FailureEventInfo>,
		/// Notification correlation identifier.
		#[serde(
			rename = "notifCorrId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notif_corr_id: Option<String>,
		#[serde(
			rename = "notificationURI",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notification_uri: Option<Uri>,
		#[serde(rename = "prevSub", default, skip_serializing_if = "Option::is_none")]
		pub prev_sub: Option<PrevSubInfo>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&NnwdafEventsSubscription> for NnwdafEventsSubscription {
		fn from(value: &NnwdafEventsSubscription) -> Self {
			value.clone()
		}
	}

	/// Data within a create subscription request for non-UE specific N2
	/// information notification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a create subscription request for non-UE
	/// specific N2 information notification",
	///  "type": "object",
	///  "required": [
	///    "n2InformationClass",
	///    "n2NotifyCallbackUri"
	///  ],
	///  "properties": {
	///    "anTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "globalRanNodeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GlobalRanNodeId"
	///      },
	///      "minItems": 1
	///    },
	///    "n2InformationClass": {
	///      "$ref": "#/components/schemas/N2InformationClass"
	///    },
	///    "n2NotifyCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
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
	pub struct NonUeN2InfoSubscriptionCreateData {
		#[serde(rename = "anTypeList", default, skip_serializing_if = "Vec::is_empty")]
		pub an_type_list: Vec<AccessType>,
		#[serde(
			rename = "globalRanNodeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub global_ran_node_list: Vec<GlobalRanNodeId>,
		#[serde(rename = "n2InformationClass")]
		pub n2_information_class: N2InformationClass,
		#[serde(rename = "n2NotifyCallbackUri")]
		pub n2_notify_callback_uri: Uri,
		#[serde(rename = "nfId", default, skip_serializing_if = "Option::is_none")]
		pub nf_id: Option<NfInstanceId>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&NonUeN2InfoSubscriptionCreateData> for NonUeN2InfoSubscriptionCreateData {
		fn from(value: &NonUeN2InfoSubscriptionCreateData) -> Self {
			value.clone()
		}
	}

	/// Data for the created subscription for non-UE specific N2 information
	/// notification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data for the created subscription for non-UE specific
	/// N2 information notification",
	///  "type": "object",
	///  "required": [
	///    "n2NotifySubscriptionId"
	///  ],
	///  "properties": {
	///    "n2InformationClass": {
	///      "$ref": "#/components/schemas/N2InformationClass"
	///    },
	///    "n2NotifySubscriptionId": {
	///      "type": "string"
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
	pub struct NonUeN2InfoSubscriptionCreatedData {
		#[serde(
			rename = "n2InformationClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_information_class: Option<N2InformationClass>,
		#[serde(rename = "n2NotifySubscriptionId")]
		pub n2_notify_subscription_id: String,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&NonUeN2InfoSubscriptionCreatedData> for NonUeN2InfoSubscriptionCreatedData {
		fn from(value: &NonUeN2InfoSubscriptionCreatedData) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - PERIODIC: The subscribe of NWDAF Event is periodically. The periodic
	///   of the notification is identified by repetitionPeriod defined in
	///   clause 5.1.6.2.3.
	/// - THRESHOLD: The subscribe of NWDAF Event is upon threshold exceeded.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- PERIODIC: The subscribe of NWDAF Event is periodically. The periodic of the notification is identified by repetitionPeriod defined in clause 5.1.6.2.3.\n- THRESHOLD: The subscribe of NWDAF Event is upon threshold exceeded.\n",
	///  "type": "string",
	///  "enum": [
	///    "PERIODIC",
	///    "THRESHOLD"
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
		#[serde(rename = "THRESHOLD")]
		Threshold,
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
				Self::Threshold => "THRESHOLD".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NotificationMethod {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PERIODIC" => Ok(Self::Periodic),
				"THRESHOLD" => Ok(Self::Threshold),
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

	/// Data within EventNotify notification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within EventNotify notification",
	///  "type": "object",
	///  "required": [
	///    "locationEvent"
	///  ],
	///  "properties": {
	///    "achievedQos": {
	///      "$ref": "#/components/schemas/MinorLocationQoS"
	///    },
	///    "ageOfLocationEstimate": {
	///      "$ref": "#/components/schemas/AgeOfLocationEstimate"
	///    },
	///    "altitude": {
	///      "$ref": "#/components/schemas/Altitude"
	///    },
	///    "barometricPressure": {
	///      "$ref": "#/components/schemas/BarometricPressure"
	///    },
	///    "civicAddress": {
	///      "$ref": "#/components/schemas/CivicAddress"
	///    },
	///    "ecgi": {
	///      "$ref": "#/components/schemas/Ecgi"
	///    },
	///    "gnssPositioningDataList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GnssPositioningMethodAndUsage"
	///      },
	///      "maxItems": 9,
	///      "minItems": 0
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "haGnssMetrics": {
	///      "$ref": "#/components/schemas/HighAccuracyGnssMetrics"
	///    },
	///    "hgmlcCallBackURI": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "ldrReference": {
	///      "$ref": "#/components/schemas/LdrReference"
	///    },
	///    "localLocationEstimate": {
	///      "$ref": "#/components/schemas/LocalArea"
	///    },
	///    "locationEstimate": {
	///      "$ref": "#/components/schemas/GeographicArea"
	///    },
	///    "locationEvent": {
	///      "$ref": "#/components/schemas/LocationEvent"
	///    },
	///    "mscServerId": {
	///      "$ref": "#/components/schemas/E164Number"
	///    },
	///    "ncgi": {
	///      "$ref": "#/components/schemas/Ncgi"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "positioningDataList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PositioningMethodAndUsage"
	///      },
	///      "maxItems": 9,
	///      "minItems": 0
	///    },
	///    "servingLMFIdentification": {
	///      "$ref": "#/components/schemas/LMFIdentification"
	///    },
	///    "servingNode": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "targetMmeName": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "targetMmeRealm": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "terminationCause": {
	///      "$ref": "#/components/schemas/TerminationCause"
	///    },
	///    "timestampOfLocationEstimate": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "utranSrvccInd": {
	///      "type": "boolean"
	///    },
	///    "velocityEstimate": {
	///      "$ref": "#/components/schemas/VelocityEstimate"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NotifiedPosInfo {
		#[serde(
			rename = "achievedQos",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub achieved_qos: Option<MinorLocationQoS>,
		#[serde(
			rename = "ageOfLocationEstimate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub age_of_location_estimate: Option<AgeOfLocationEstimate>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub altitude: Option<Altitude>,
		#[serde(
			rename = "barometricPressure",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub barometric_pressure: Option<BarometricPressure>,
		#[serde(
			rename = "civicAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub civic_address: Option<CivicAddress>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ecgi: Option<Ecgi>,
		#[serde(
			rename = "gnssPositioningDataList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub gnss_positioning_data_list: Vec<GnssPositioningMethodAndUsage>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(
			rename = "haGnssMetrics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ha_gnss_metrics: Option<HighAccuracyGnssMetrics>,
		#[serde(
			rename = "hgmlcCallBackURI",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hgmlc_call_back_uri: Option<Uri>,
		#[serde(
			rename = "ldrReference",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ldr_reference: Option<LdrReference>,
		#[serde(
			rename = "localLocationEstimate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub local_location_estimate: Option<LocalArea>,
		#[serde(
			rename = "locationEstimate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub location_estimate: Option<GeographicArea>,
		#[serde(rename = "locationEvent")]
		pub location_event: LocationEvent,
		#[serde(
			rename = "mscServerId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub msc_server_id: Option<E164Number>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ncgi: Option<Ncgi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(
			rename = "positioningDataList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub positioning_data_list: Vec<PositioningMethodAndUsage>,
		#[serde(
			rename = "servingLMFIdentification",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_lmf_identification: Option<LmfIdentification>,
		#[serde(
			rename = "servingNode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_node: Option<NfInstanceId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "targetMmeName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_mme_name: Option<Fqdn>,
		#[serde(
			rename = "targetMmeRealm",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_mme_realm: Option<Fqdn>,
		#[serde(
			rename = "terminationCause",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub termination_cause: Option<TerminationCause>,
		#[serde(
			rename = "timestampOfLocationEstimate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub timestamp_of_location_estimate: Option<DateTime>,
		#[serde(
			rename = "utranSrvccInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub utran_srvcc_ind: Option<bool>,
		#[serde(
			rename = "velocityEstimate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub velocity_estimate: Option<VelocityEstimate>,
	}

	impl From<&NotifiedPosInfo> for NotifiedPosInfo {
		fn from(value: &NotifiedPosInfo) -> Self {
			value.clone()
		}
	}

	/// NPN Access Information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "NPN Access Information.",
	///  "type": "object",
	///  "properties": {
	///    "cellCagInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CagId"
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
	pub struct NpnAccessInfo {
		#[serde(rename = "cellCagInfo", default, skip_serializing_if = "Vec::is_empty")]
		pub cell_cag_info: Vec<CagId>,
	}

	impl From<&NpnAccessInfo> for NpnAccessInfo {
		fn from(value: &NpnAccessInfo) -> Self {
			value.clone()
		}
	}

	/// Represents a NRPPa related N2 information data part
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a NRPPa related N2 information data part",
	///  "type": "object",
	///  "required": [
	///    "nfId",
	///    "nrppaPdu"
	///  ],
	///  "properties": {
	///    "nfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "nrppaPdu": {
	///      "$ref": "#/components/schemas/N2InfoContent"
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
	pub struct NrppaInformation {
		#[serde(rename = "nfId")]
		pub nf_id: NfInstanceId,
		#[serde(rename = "nrppaPdu")]
		pub nrppa_pdu: N2InfoContent,
		#[serde(
			rename = "serviceInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_instance_id: Option<String>,
	}

	impl From<&NrppaInformation> for NrppaInformation {
		fn from(value: &NrppaInformation) -> Self {
			value.clone()
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

	/// Represents the S-NSSAI and the optionally associated Network Slice
	/// Instance(s).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the S-NSSAI and the optionally associated
	/// Network Slice Instance(s).",
	///  "type": "object",
	///  "required": [
	///    "snssai"
	///  ],
	///  "properties": {
	///    "nsiIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NsiId"
	///      },
	///      "minItems": 1
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
	pub struct NsiIdInfo {
		#[serde(rename = "nsiIds", default, skip_serializing_if = "Vec::is_empty")]
		pub nsi_ids: Vec<NsiId>,
		pub snssai: Snssai,
	}

	impl From<&NsiIdInfo> for NsiIdInfo {
		fn from(value: &NsiIdInfo) -> Self {
			value.clone()
		}
	}

	/// Contains the API URIs of NRF services to be used to discover
	/// NFs/services, subscribe to NF status changes and/or request access
	/// tokens within the selected Network Slice instance and optional the
	/// Identifier of the selected Network Slice instance
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the API URIs of NRF services to be used to
	/// discover NFs/services, subscribe to NF status changes and/or request
	/// access tokens within the selected Network Slice instance and optional
	/// the Identifier of the selected Network Slice instance\n",
	///  "type": "object",
	///  "required": [
	///    "nrfId"
	///  ],
	///  "properties": {
	///    "nrfAccessTokenUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfId": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfNfMgtUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nrfOauth2Required": {
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
	///    "nsiId": {
	///      "$ref": "#/components/schemas/NsiId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NsiInformation {
		#[serde(
			rename = "nrfAccessTokenUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_access_token_uri: Option<Uri>,
		#[serde(rename = "nrfId")]
		pub nrf_id: Uri,
		#[serde(
			rename = "nrfNfMgtUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nrf_nf_mgt_uri: Option<Uri>,
		/// Map indicating whether the NRF requires Oauth2-based authorization
		/// for accessing its services. The key of the map shall be the name of
		/// an NRF service, e.g. "nnrf-nfm" or "nnrf-disc"
		#[serde(
			rename = "nrfOauth2Required",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub nrf_oauth2_required: ::std::collections::HashMap<String, bool>,
		#[serde(rename = "nsiId", default, skip_serializing_if = "Option::is_none")]
		pub nsi_id: Option<NsiId>,
	}

	impl From<&NsiInformation> for NsiInformation {
		fn from(value: &NsiInformation) -> Self {
			value.clone()
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

	/// Represents the mapping between a S-NSSAI in serving PLMN to a S-NSSAI in
	/// home PLMN
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the mapping between a S-NSSAI in serving
	/// PLMN to a S-NSSAI in home PLMN",
	///  "type": "object",
	///  "required": [
	///    "hSnssai",
	///    "mappedSnssai"
	///  ],
	///  "properties": {
	///    "hSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "mappedSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NssaiMapping {
		#[serde(rename = "hSnssai")]
		pub h_snssai: Snssai,
		#[serde(rename = "mappedSnssai")]
		pub mapped_snssai: Snssai,
	}

	impl From<&NssaiMapping> for NssaiMapping {
		fn from(value: &NssaiMapping) -> Self {
			value.clone()
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

	/// Possible values are:
	/// - UNAVAILABLE_DATA: Indicates the requested statistics information for
	///   the event is rejected since necessary data to perform the service is
	///   unavailable.
	/// - BOTH_STAT_PRED_NOT_ALLOWED: Indicates the requested analysis
	///   information for the event is rejected since the start time is in the
	///   past and the end time is in the future, which means the NF service
	///   consumer requested both statistics and prediction for the analytics.
	/// - UNSATISFIED_REQUESTED_ANALYTICS_TIME: Indicates that the requested
	///   event is rejected since the analytics information is not ready when
	///   the time indicated by the "timeAnaNeeded" attribute (as provided
	///   during the creation or modification of subscription) is reached.
	/// - OTHER: Indicates the requested analysis information for the event is
	///   rejected due to other reasons.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- UNAVAILABLE_DATA: Indicates the
	/// requested statistics information for the event is rejected since
	/// necessary data to perform the service is unavailable.\n-
	/// BOTH_STAT_PRED_NOT_ALLOWED: Indicates the requested analysis information
	/// for the event is rejected since the start time is in the past and the
	/// end time is in the future, which means the NF service consumer requested
	/// both statistics and prediction for the analytics.\n-
	/// UNSATISFIED_REQUESTED_ANALYTICS_TIME: Indicates that the requested event
	/// is rejected since the analytics information is not ready when the time
	/// indicated by the \"timeAnaNeeded\" attribute (as provided during the
	/// creation or modification of subscription) is reached.\n- OTHER:
	/// Indicates the requested analysis information for the event is rejected
	/// due to other reasons.\n",
	///  "type": "string",
	///  "enum": [
	///    "UNAVAILABLE_DATA",
	///    "BOTH_STAT_PRED_NOT_ALLOWED",
	///    "UNSATISFIED_REQUESTED_ANALYTICS_TIME",
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
	pub enum NwdafFailureCode {
		#[default]
		#[serde(rename = "UNAVAILABLE_DATA")]
		UnavailableData,
		#[serde(rename = "BOTH_STAT_PRED_NOT_ALLOWED")]
		BothStatPredNotAllowed,
		#[serde(rename = "UNSATISFIED_REQUESTED_ANALYTICS_TIME")]
		UnsatisfiedRequestedAnalyticsTime,
		#[serde(rename = "OTHER")]
		Other,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&NwdafFailureCode> for NwdafFailureCode {
		fn from(value: &NwdafFailureCode) -> Self {
			value.clone()
		}
	}

	impl ToString for NwdafFailureCode {
		fn to_string(&self) -> String {
			match *self {
				Self::UnavailableData => "UNAVAILABLE_DATA".to_string(),
				Self::BothStatPredNotAllowed => "BOTH_STAT_PRED_NOT_ALLOWED".to_string(),
				Self::UnsatisfiedRequestedAnalyticsTime => {
					"UNSATISFIED_REQUESTED_ANALYTICS_TIME".to_string()
				}
				Self::Other => "OTHER".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for NwdafFailureCode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UNAVAILABLE_DATA" => Ok(Self::UnavailableData),
				"BOTH_STAT_PRED_NOT_ALLOWED" => Ok(Self::BothStatPredNotAllowed),
				"UNSATISFIED_REQUESTED_ANALYTICS_TIME" => {
					Ok(Self::UnsatisfiedRequestedAnalyticsTime)
				}
				"OTHER" => Ok(Self::Other),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for NwdafFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for NwdafFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for NwdafFailureCode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Individual NWDAF subscription identified by the subscription Id.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Individual NWDAF subscription identified by the
	/// subscription Id.",
	///  "type": "object",
	///  "required": [
	///    "nwdafEventsSubscription",
	///    "nwdafEvtSubsServiceUri"
	///  ],
	///  "properties": {
	///    "nwdafEventsSubscription": {
	///      "$ref": "#/components/schemas/NnwdafEventsSubscription"
	///    },
	///    "nwdafEvtSubsServiceUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct NwdafSubscription {
		#[serde(rename = "nwdafEventsSubscription")]
		pub nwdaf_events_subscription: NnwdafEventsSubscription,
		#[serde(rename = "nwdafEvtSubsServiceUri")]
		pub nwdaf_evt_subs_service_uri: Uri,
	}

	impl From<&NwdafSubscription> for NwdafSubscription {
		fn from(value: &NwdafSubscription) -> Self {
			value.clone()
		}
	}

	/// Represents the observed redundant transmission experience related
	/// information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the observed redundant transmission
	/// experience related information.",
	///  "type": "object",
	///  "properties": {
	///    "avgPktDelayDl": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "avgPktDelayUl": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "avgPktDropRateDl": {
	///      "$ref": "#/components/schemas/PacketLossRate"
	///    },
	///    "avgPktDropRateUl": {
	///      "$ref": "#/components/schemas/PacketLossRate"
	///    },
	///    "varPktDelayDl": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "varPktDelayUl": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "varPktDropRateDl": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "varPktDropRateUl": {
	///      "$ref": "#/components/schemas/Float"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ObservedRedundantTransExp {
		#[serde(
			rename = "avgPktDelayDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub avg_pkt_delay_dl: Option<PacketDelBudget>,
		#[serde(
			rename = "avgPktDelayUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub avg_pkt_delay_ul: Option<PacketDelBudget>,
		#[serde(
			rename = "avgPktDropRateDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub avg_pkt_drop_rate_dl: Option<PacketLossRate>,
		#[serde(
			rename = "avgPktDropRateUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub avg_pkt_drop_rate_ul: Option<PacketLossRate>,
		#[serde(
			rename = "varPktDelayDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub var_pkt_delay_dl: Option<Float>,
		#[serde(
			rename = "varPktDelayUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub var_pkt_delay_ul: Option<Float>,
		#[serde(
			rename = "varPktDropRateDl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub var_pkt_drop_rate_dl: Option<Float>,
		#[serde(
			rename = "varPktDropRateUl",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub var_pkt_drop_rate_ul: Option<Float>,
	}

	impl From<&ObservedRedundantTransExp> for ObservedRedundantTransExp {
		fn from(value: &ObservedRedundantTransExp) -> Self {
			value.clone()
		}
	}

	/// Specifies occurrence of event reporting.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Specifies occurrence of event reporting.",
	///  "type": "string",
	///  "enum": [
	///    "ONE_TIME_EVENT",
	///    "MULTIPLE_TIME_EVENT"
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
	pub enum OccurrenceInfo {
		#[default]
		#[serde(rename = "ONE_TIME_EVENT")]
		OneTimeEvent,
		#[serde(rename = "MULTIPLE_TIME_EVENT")]
		MultipleTimeEvent,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&OccurrenceInfo> for OccurrenceInfo {
		fn from(value: &OccurrenceInfo) -> Self {
			value.clone()
		}
	}

	impl ToString for OccurrenceInfo {
		fn to_string(&self) -> String {
			match *self {
				Self::OneTimeEvent => "ONE_TIME_EVENT".to_string(),
				Self::MultipleTimeEvent => "MULTIPLE_TIME_EVENT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for OccurrenceInfo {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"ONE_TIME_EVENT" => Ok(Self::OneTimeEvent),
				"MULTIPLE_TIME_EVENT" => Ok(Self::MultipleTimeEvent),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for OccurrenceInfo {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for OccurrenceInfo {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for OccurrenceInfo {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the OMC Identifier
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the OMC Identifier",
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
	pub struct OmcIdentifier(pub String);

	impl ::std::ops::Deref for OmcIdentifier {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<OmcIdentifier> for String {
		fn from(value: OmcIdentifier) -> Self {
			value.0
		}
	}

	impl From<&OmcIdentifier> for OmcIdentifier {
		fn from(value: &OmcIdentifier) -> Self {
			value.clone()
		}
	}

	impl From<String> for OmcIdentifier {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for OmcIdentifier {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for OmcIdentifier {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Possible values are:
	/// - BINARY: Indicates that the analytics shall only be reported when the
	///   requested level of accuracy is reached within a cycle of periodic
	///   notification.
	/// - GRADIENT: Indicates that the analytics shall be reported according
	///   with the periodicity irrespective of whether the requested level of
	///   accuracy has been reached or not.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- BINARY: Indicates that the
	/// analytics shall only be reported when the requested level of accuracy is
	/// reached within a cycle of periodic notification.\n- GRADIENT: Indicates
	/// that the analytics shall be reported according with the periodicity
	/// irrespective of whether the requested level of accuracy has been reached
	/// or not.\n",
	///  "type": "string",
	///  "enum": [
	///    "BINARY",
	///    "GRADIENT"
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
	pub enum OutputStrategy {
		#[default]
		#[serde(rename = "BINARY")]
		Binary,
		#[serde(rename = "GRADIENT")]
		Gradient,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&OutputStrategy> for OutputStrategy {
		fn from(value: &OutputStrategy) -> Self {
			value.clone()
		}
	}

	impl ToString for OutputStrategy {
		fn to_string(&self) -> String {
			match *self {
				Self::Binary => "BINARY".to_string(),
				Self::Gradient => "GRADIENT".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for OutputStrategy {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"BINARY" => Ok(Self::Binary),
				"GRADIENT" => Ok(Self::Gradient),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for OutputStrategy {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for OutputStrategy {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for OutputStrategy {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents a PDU Session Context in UE Context
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a PDU Session Context in UE Context",
	///  "type": "object",
	///  "required": [
	///    "accessType",
	///    "dnn",
	///    "pduSessionId",
	///    "sNssai",
	///    "smContextRef"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "additionalAccessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
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
	///    "anchorSmfOauth2Required": {
	///      "type": "boolean"
	///    },
	///    "anchorSmfSupportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "cnAssistedRanPara": {
	///      "$ref": "#/components/schemas/CnAssistedRanPara"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "hsmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "hsmfServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "hsmfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "interPlmnApiRoot": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "ismfBinding": {
	///      "$ref": "#/components/schemas/SbiBindingLevel"
	///    },
	///    "ismfBindingInfo": {
	///      "type": "string"
	///    },
	///    "ismfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "ismfServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "ismfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "maPduSession": {
	///      "default": false,
	///      "type": "boolean"
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
	///    "nsInstance": {
	///      "$ref": "#/components/schemas/NsiId"
	///    },
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "pgwFqdn": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "pgwIpAddr": {
	///      "$ref": "#/components/schemas/IpAddress"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "selectedDnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "smContextRef": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "smfBinding": {
	///      "$ref": "#/components/schemas/SbiBindingLevel"
	///    },
	///    "smfBindingInfo": {
	///      "type": "string"
	///    },
	///    "smfServiceInstanceId": {
	///      "type": "string"
	///    },
	///    "vsmfBinding": {
	///      "$ref": "#/components/schemas/SbiBindingLevel"
	///    },
	///    "vsmfBindingInfo": {
	///      "type": "string"
	///    },
	///    "vsmfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "vsmfServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "vsmfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PduSessionContext {
		#[serde(rename = "accessType")]
		pub access_type: AccessType,
		#[serde(
			rename = "additionalAccessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_access_type: Option<AccessType>,
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
		#[serde(
			rename = "anchorSmfOauth2Required",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub anchor_smf_oauth2_required: Option<bool>,
		#[serde(
			rename = "anchorSmfSupportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub anchor_smf_supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "cnAssistedRanPara",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub cn_assisted_ran_para: Option<CnAssistedRanPara>,
		pub dnn: Dnn,
		#[serde(rename = "hsmfId", default, skip_serializing_if = "Option::is_none")]
		pub hsmf_id: Option<NfInstanceId>,
		#[serde(
			rename = "hsmfServiceSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hsmf_service_set_id: Option<NfServiceSetId>,
		#[serde(rename = "hsmfSetId", default, skip_serializing_if = "Option::is_none")]
		pub hsmf_set_id: Option<NfSetId>,
		#[serde(
			rename = "interPlmnApiRoot",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub inter_plmn_api_root: Option<Uri>,
		#[serde(
			rename = "ismfBinding",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ismf_binding: Option<SbiBindingLevel>,
		#[serde(
			rename = "ismfBindingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ismf_binding_info: Option<String>,
		#[serde(rename = "ismfId", default, skip_serializing_if = "Option::is_none")]
		pub ismf_id: Option<NfInstanceId>,
		#[serde(
			rename = "ismfServiceSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ismf_service_set_id: Option<NfServiceSetId>,
		#[serde(rename = "ismfSetId", default, skip_serializing_if = "Option::is_none")]
		pub ismf_set_id: Option<NfSetId>,
		#[serde(rename = "maPduSession", default)]
		pub ma_pdu_session: bool,
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
		#[serde(
			rename = "nsInstance",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ns_instance: Option<NsiId>,
		#[serde(rename = "pduSessionId")]
		pub pdu_session_id: PduSessionId,
		#[serde(rename = "pgwFqdn", default, skip_serializing_if = "Option::is_none")]
		pub pgw_fqdn: Option<Fqdn>,
		#[serde(rename = "pgwIpAddr", default, skip_serializing_if = "Option::is_none")]
		pub pgw_ip_addr: Option<IpAddress>,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnId>,
		#[serde(rename = "sNssai")]
		pub s_nssai: Snssai,
		#[serde(
			rename = "selectedDnn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub selected_dnn: Option<Dnn>,
		#[serde(rename = "smContextRef")]
		pub sm_context_ref: Uri,
		#[serde(
			rename = "smfBinding",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_binding: Option<SbiBindingLevel>,
		#[serde(
			rename = "smfBindingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_binding_info: Option<String>,
		#[serde(
			rename = "smfServiceInstanceId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_service_instance_id: Option<String>,
		#[serde(
			rename = "vsmfBinding",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vsmf_binding: Option<SbiBindingLevel>,
		#[serde(
			rename = "vsmfBindingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vsmf_binding_info: Option<String>,
		#[serde(rename = "vsmfId", default, skip_serializing_if = "Option::is_none")]
		pub vsmf_id: Option<NfInstanceId>,
		#[serde(
			rename = "vsmfServiceSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub vsmf_service_set_id: Option<NfServiceSetId>,
		#[serde(rename = "vsmfSetId", default, skip_serializing_if = "Option::is_none")]
		pub vsmf_set_id: Option<NfSetId>,
	}

	impl From<&PduSessionContext> for PduSessionContext {
		fn from(value: &PduSessionContext) -> Self {
			value.clone()
		}
	}

	/// Represents DN performance data.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents DN performance data.",
	///  "type": "object",
	///  "properties": {
	///    "avePacketDelay": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "avgPacketLossRate": {
	///      "$ref": "#/components/schemas/PacketLossRate"
	///    },
	///    "avgTrafficRate": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "maxPacketDelay": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "maxTrafficRate": {
	///      "$ref": "#/components/schemas/BitRate"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PerfData {
		#[serde(
			rename = "avePacketDelay",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ave_packet_delay: Option<PacketDelBudget>,
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
	}

	impl From<&PerfData> for PerfData {
		fn from(value: &PerfData) -> Self {
			value.clone()
		}
	}

	/// Indicates the Periodic Communication Indicator
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the Periodic Communication Indicator",
	///  "type": "string",
	///  "enum": [
	///    "PIORIODICALLY",
	///    "ON_DEMAND"
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
	pub enum PeriodicCommunicationIndicator {
		#[default]
		#[serde(rename = "PIORIODICALLY")]
		Pioriodically,
		#[serde(rename = "ON_DEMAND")]
		OnDemand,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PeriodicCommunicationIndicator> for PeriodicCommunicationIndicator {
		fn from(value: &PeriodicCommunicationIndicator) -> Self {
			value.clone()
		}
	}

	impl ToString for PeriodicCommunicationIndicator {
		fn to_string(&self) -> String {
			match *self {
				Self::Pioriodically => "PIORIODICALLY".to_string(),
				Self::OnDemand => "ON_DEMAND".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PeriodicCommunicationIndicator {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"PIORIODICALLY" => Ok(Self::Pioriodically),
				"ON_DEMAND" => Ok(Self::OnDemand),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PeriodicCommunicationIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PeriodicCommunicationIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PeriodicCommunicationIndicator {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the information of periodic event reporting.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the information of periodic event
	/// reporting.",
	///  "type": "object",
	///  "required": [
	///    "reportingAmount",
	///    "reportingInterval"
	///  ],
	///  "properties": {
	///    "reportingAmount": {
	///      "$ref": "#/components/schemas/ReportingAmount"
	///    },
	///    "reportingInfiniteInd": {
	///      "type": "boolean",
	///      "enum": [
	///        true
	///      ]
	///    },
	///    "reportingInterval": {
	///      "$ref": "#/components/schemas/ReportingInterval"
	///    },
	///    "reportingIntervalMs": {
	///      "$ref": "#/components/schemas/ReportingIntervalMs"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PeriodicEventInfo {
		#[serde(rename = "reportingAmount")]
		pub reporting_amount: ReportingAmount,
		#[serde(
			rename = "reportingInfiniteInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reporting_infinite_ind: Option<bool>,
		#[serde(rename = "reportingInterval")]
		pub reporting_interval: ReportingInterval,
		#[serde(
			rename = "reportingIntervalMs",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reporting_interval_ms: Option<ReportingIntervalMs>,
	}

	impl From<&PeriodicEventInfo> for PeriodicEventInfo {
		fn from(value: &PeriodicEventInfo) -> Self {
			value.clone()
		}
	}

	/// Policy Request Triggers
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Policy Request Triggers",
	///  "type": "string",
	///  "enum": [
	///    "LOCATION_CHANGE",
	///    "PRA_CHANGE",
	///    "ALLOWED_NSSAI_CHANGE",
	///    "NWDAF_DATA_CHANGE",
	///    "PLMN_CHANGE",
	///    "CON_STATE_CHANGE",
	///    "SMF_SELECT_CHANGE",
	///    "ACCESS_TYPE_CHANGE"
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
	pub enum PolicyReqTrigger {
		#[default]
		#[serde(rename = "LOCATION_CHANGE")]
		LocationChange,
		#[serde(rename = "PRA_CHANGE")]
		PraChange,
		#[serde(rename = "ALLOWED_NSSAI_CHANGE")]
		AllowedNssaiChange,
		#[serde(rename = "NWDAF_DATA_CHANGE")]
		NwdafDataChange,
		#[serde(rename = "PLMN_CHANGE")]
		PlmnChange,
		#[serde(rename = "CON_STATE_CHANGE")]
		ConStateChange,
		#[serde(rename = "SMF_SELECT_CHANGE")]
		SmfSelectChange,
		#[serde(rename = "ACCESS_TYPE_CHANGE")]
		AccessTypeChange,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PolicyReqTrigger> for PolicyReqTrigger {
		fn from(value: &PolicyReqTrigger) -> Self {
			value.clone()
		}
	}

	impl ToString for PolicyReqTrigger {
		fn to_string(&self) -> String {
			match *self {
				Self::LocationChange => "LOCATION_CHANGE".to_string(),
				Self::PraChange => "PRA_CHANGE".to_string(),
				Self::AllowedNssaiChange => "ALLOWED_NSSAI_CHANGE".to_string(),
				Self::NwdafDataChange => "NWDAF_DATA_CHANGE".to_string(),
				Self::PlmnChange => "PLMN_CHANGE".to_string(),
				Self::ConStateChange => "CON_STATE_CHANGE".to_string(),
				Self::SmfSelectChange => "SMF_SELECT_CHANGE".to_string(),
				Self::AccessTypeChange => "ACCESS_TYPE_CHANGE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PolicyReqTrigger {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOCATION_CHANGE" => Ok(Self::LocationChange),
				"PRA_CHANGE" => Ok(Self::PraChange),
				"ALLOWED_NSSAI_CHANGE" => Ok(Self::AllowedNssaiChange),
				"NWDAF_DATA_CHANGE" => Ok(Self::NwdafDataChange),
				"PLMN_CHANGE" => Ok(Self::PlmnChange),
				"CON_STATE_CHANGE" => Ok(Self::ConStateChange),
				"SMF_SELECT_CHANGE" => Ok(Self::SmfSelectChange),
				"ACCESS_TYPE_CHANGE" => Ok(Self::AccessTypeChange),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PolicyReqTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PolicyReqTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PolicyReqTrigger {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates supported positioning methods.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates supported positioning methods.",
	///  "type": "string",
	///  "enum": [
	///    "CELLID",
	///    "ECID",
	///    "OTDOA",
	///    "BAROMETRIC_PRESSURE",
	///    "WLAN",
	///    "BLUETOOTH",
	///    "MBS",
	///    "MOTION_SENSOR",
	///    "DL_TDOA",
	///    "DL_AOD",
	///    "MULTI-RTT",
	///    "NR_ECID",
	///    "UL_TDOA",
	///    "UL_AOA",
	///    "NETWORK_SPECIFIC"
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
	pub enum PositioningMethod {
		#[default]
		#[serde(rename = "CELLID")]
		Cellid,
		#[serde(rename = "ECID")]
		Ecid,
		#[serde(rename = "OTDOA")]
		Otdoa,
		#[serde(rename = "BAROMETRIC_PRESSURE")]
		BarometricPressure,
		#[serde(rename = "WLAN")]
		Wlan,
		#[serde(rename = "BLUETOOTH")]
		Bluetooth,
		#[serde(rename = "MBS")]
		Mbs,
		#[serde(rename = "MOTION_SENSOR")]
		MotionSensor,
		#[serde(rename = "DL_TDOA")]
		DlTdoa,
		#[serde(rename = "DL_AOD")]
		DlAod,
		#[serde(rename = "MULTI-RTT")]
		MultiRtt,
		#[serde(rename = "NR_ECID")]
		NrEcid,
		#[serde(rename = "UL_TDOA")]
		UlTdoa,
		#[serde(rename = "UL_AOA")]
		UlAoa,
		#[serde(rename = "NETWORK_SPECIFIC")]
		NetworkSpecific,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PositioningMethod> for PositioningMethod {
		fn from(value: &PositioningMethod) -> Self {
			value.clone()
		}
	}

	impl ToString for PositioningMethod {
		fn to_string(&self) -> String {
			match *self {
				Self::Cellid => "CELLID".to_string(),
				Self::Ecid => "ECID".to_string(),
				Self::Otdoa => "OTDOA".to_string(),
				Self::BarometricPressure => "BAROMETRIC_PRESSURE".to_string(),
				Self::Wlan => "WLAN".to_string(),
				Self::Bluetooth => "BLUETOOTH".to_string(),
				Self::Mbs => "MBS".to_string(),
				Self::MotionSensor => "MOTION_SENSOR".to_string(),
				Self::DlTdoa => "DL_TDOA".to_string(),
				Self::DlAod => "DL_AOD".to_string(),
				Self::MultiRtt => "MULTI-RTT".to_string(),
				Self::NrEcid => "NR_ECID".to_string(),
				Self::UlTdoa => "UL_TDOA".to_string(),
				Self::UlAoa => "UL_AOA".to_string(),
				Self::NetworkSpecific => "NETWORK_SPECIFIC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PositioningMethod {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CELLID" => Ok(Self::Cellid),
				"ECID" => Ok(Self::Ecid),
				"OTDOA" => Ok(Self::Otdoa),
				"BAROMETRIC_PRESSURE" => Ok(Self::BarometricPressure),
				"WLAN" => Ok(Self::Wlan),
				"BLUETOOTH" => Ok(Self::Bluetooth),
				"MBS" => Ok(Self::Mbs),
				"MOTION_SENSOR" => Ok(Self::MotionSensor),
				"DL_TDOA" => Ok(Self::DlTdoa),
				"DL_AOD" => Ok(Self::DlAod),
				"MULTI-RTT" => Ok(Self::MultiRtt),
				"NR_ECID" => Ok(Self::NrEcid),
				"UL_TDOA" => Ok(Self::UlTdoa),
				"UL_AOA" => Ok(Self::UlAoa),
				"NETWORK_SPECIFIC" => Ok(Self::NetworkSpecific),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PositioningMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PositioningMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PositioningMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates the usage of a positioning method.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the usage of a positioning method.",
	///  "type": "object",
	///  "required": [
	///    "method",
	///    "mode",
	///    "usage"
	///  ],
	///  "properties": {
	///    "method": {
	///      "$ref": "#/components/schemas/PositioningMethod"
	///    },
	///    "methodCode": {
	///      "type": "integer",
	///      "maximum": 31.0,
	///      "minimum": 16.0
	///    },
	///    "mode": {
	///      "$ref": "#/components/schemas/PositioningMode"
	///    },
	///    "usage": {
	///      "$ref": "#/components/schemas/Usage"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PositioningMethodAndUsage {
		pub method: PositioningMethod,
		#[serde(
			rename = "methodCode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub method_code: Option<i64>,
		pub mode: PositioningMode,
		pub usage: Usage,
	}

	impl From<&PositioningMethodAndUsage> for PositioningMethodAndUsage {
		fn from(value: &PositioningMethodAndUsage) -> Self {
			value.clone()
		}
	}

	/// Indicates supported modes used for positioning method.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates supported modes used for positioning
	/// method.",
	///  "type": "string",
	///  "enum": [
	///    "UE_BASED",
	///    "UE_ASSISTED",
	///    "CONVENTIONAL"
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
	pub enum PositioningMode {
		#[default]
		#[serde(rename = "UE_BASED")]
		UeBased,
		#[serde(rename = "UE_ASSISTED")]
		UeAssisted,
		#[serde(rename = "CONVENTIONAL")]
		Conventional,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&PositioningMode> for PositioningMode {
		fn from(value: &PositioningMode) -> Self {
			value.clone()
		}
	}

	impl ToString for PositioningMode {
		fn to_string(&self) -> String {
			match *self {
				Self::UeBased => "UE_BASED".to_string(),
				Self::UeAssisted => "UE_ASSISTED".to_string(),
				Self::Conventional => "CONVENTIONAL".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for PositioningMode {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UE_BASED" => Ok(Self::UeBased),
				"UE_ASSISTED" => Ok(Self::UeAssisted),
				"CONVENTIONAL" => Ok(Self::Conventional),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for PositioningMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for PositioningMode {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for PositioningMode {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Paging Policy Indicator
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Paging Policy Indicator",
	///  "type": "integer",
	///  "maximum": 7.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Ppi(pub i64);

	impl ::std::ops::Deref for Ppi {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<Ppi> for i64 {
		fn from(value: Ppi) -> Self {
			value.0
		}
	}

	impl From<&Ppi> for Ppi {
		fn from(value: &Ppi) -> Self {
			value.clone()
		}
	}

	impl From<i64> for Ppi {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Ppi {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Ppi {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Ppi {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Ppi {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Ppi {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Information of the previous subscription.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Information of the previous subscription.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "producerId"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "producerSetId"
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "subscriptionId"
	///  ],
	///  "properties": {
	///    "nfAnaEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "producerId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "producerSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "subscriptionId": {
	///      "description": "The identifier of a subscription.",
	///      "type": "string"
	///    },
	///    "ueAnaEvents": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/UeAnalyticsContextDescriptor"
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
	pub enum PrevSubInfo {
		#[default]
		Variant0 {
			#[serde(rename = "nfAnaEvents", default, skip_serializing_if = "Vec::is_empty")]
			nf_ana_events: Vec<NwdafEvent>,
			#[serde(rename = "producerId")]
			producer_id: NfInstanceId,
			/// The identifier of a subscription.
			#[serde(rename = "subscriptionId")]
			subscription_id: String,
			#[serde(rename = "ueAnaEvents", default, skip_serializing_if = "Vec::is_empty")]
			ue_ana_events: Vec<UeAnalyticsContextDescriptor>,
		},
		Variant1 {
			#[serde(rename = "nfAnaEvents", default, skip_serializing_if = "Vec::is_empty")]
			nf_ana_events: Vec<NwdafEvent>,
			#[serde(rename = "producerSetId")]
			producer_set_id: NfSetId,
			/// The identifier of a subscription.
			#[serde(rename = "subscriptionId")]
			subscription_id: String,
			#[serde(rename = "ueAnaEvents", default, skip_serializing_if = "Vec::is_empty")]
			ue_ana_events: Vec<UeAnalyticsContextDescriptor>,
		},
	}

	impl From<&PrevSubInfo> for PrevSubInfo {
		fn from(value: &PrevSubInfo) -> Self {
			value.clone()
		}
	}

	/// Represents 5G ProSe related N2 information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents 5G ProSe related N2 information.",
	///  "type": "object",
	///  "properties": {
	///    "n2Pc5ProSePol": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProSeInformation {
		#[serde(
			rename = "n2Pc5ProSePol",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_pc5_pro_se_pol: Option<N2InfoContent>,
	}

	impl From<&ProSeInformation> for ProSeInformation {
		fn from(value: &ProSeInformation) -> Self {
			value.clone()
		}
	}

	/// Enable UE Reachability Error Detail
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Enable UE Reachability Error Detail",
	///  "allOf": [
	///    {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    {
	///      "$ref": "#/components/schemas/AdditionInfoEnableUeReachability"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProblemDetailsEnableUeReachability {
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

	impl From<&ProblemDetailsEnableUeReachability> for ProblemDetailsEnableUeReachability {
		fn from(value: &ProblemDetailsEnableUeReachability) -> Self {
			value.clone()
		}
	}

	/// Represents the ProSe services related parameters.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the ProSe services related parameters.",
	///  "type": "object",
	///  "properties": {
	///    "directComm": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "directDiscovery": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "l2Relay": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "l2Remote": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "l3Relay": {
	///      "$ref": "#/components/schemas/UeAuth"
	///    },
	///    "nrUePc5Ambr": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "pc5QoSPara": {
	///      "$ref": "#/components/schemas/Pc5QoSPara"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProseContext {
		#[serde(
			rename = "directComm",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub direct_comm: Option<UeAuth>,
		#[serde(
			rename = "directDiscovery",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub direct_discovery: Option<UeAuth>,
		#[serde(rename = "l2Relay", default, skip_serializing_if = "Option::is_none")]
		pub l2_relay: Option<UeAuth>,
		#[serde(rename = "l2Remote", default, skip_serializing_if = "Option::is_none")]
		pub l2_remote: Option<UeAuth>,
		#[serde(rename = "l3Relay", default, skip_serializing_if = "Option::is_none")]
		pub l3_relay: Option<UeAuth>,
		#[serde(
			rename = "nrUePc5Ambr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nr_ue_pc5_ambr: Option<BitRate>,
		#[serde(
			rename = "pc5QoSPara",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pc5_qo_s_para: Option<Pc5QoSPara>,
	}

	impl From<&ProseContext> for ProseContext {
		fn from(value: &ProseContext) -> Self {
			value.clone()
		}
	}

	/// Data within Provide Location Information Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Provide Location Information Response",
	///  "type": "object",
	///  "properties": {
	///    "additionalLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "currentLoc": {
	///      "type": "boolean"
	///    },
	///    "geoInfo": {
	///      "$ref": "#/components/schemas/GeographicArea"
	///    },
	///    "location": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    },
	///    "locationAge": {
	///      "$ref": "#/components/schemas/AgeOfLocationEstimate"
	///    },
	///    "oldGuami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "timezone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProvideLocInfo {
		#[serde(
			rename = "additionalLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub additional_location: Option<UserLocation>,
		#[serde(
			rename = "currentLoc",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub current_loc: Option<bool>,
		#[serde(rename = "geoInfo", default, skip_serializing_if = "Option::is_none")]
		pub geo_info: Option<GeographicArea>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub location: Option<UserLocation>,
		#[serde(
			rename = "locationAge",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub location_age: Option<AgeOfLocationEstimate>,
		#[serde(rename = "oldGuami", default, skip_serializing_if = "Option::is_none")]
		pub old_guami: Option<crate::common::common_models::Guami>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub timezone: Option<TimeZone>,
	}

	impl From<&ProvideLocInfo> for ProvideLocInfo {
		fn from(value: &ProvideLocInfo) -> Self {
			value.clone()
		}
	}

	/// Data within Provide Positioning Information Response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Provide Positioning Information Response",
	///  "type": "object",
	///  "properties": {
	///    "acceptedPeriodicEventInfo": {
	///      "$ref": "#/components/schemas/PeriodicEventInfo"
	///    },
	///    "accuracyFulfilmentIndicator": {
	///      "$ref": "#/components/schemas/AccuracyFulfilmentIndicator"
	///    },
	///    "achievedQos": {
	///      "$ref": "#/components/schemas/MinorLocationQoS"
	///    },
	///    "ageOfLocationEstimate": {
	///      "$ref": "#/components/schemas/AgeOfLocationEstimate"
	///    },
	///    "altitude": {
	///      "$ref": "#/components/schemas/Altitude"
	///    },
	///    "barometricPressure": {
	///      "$ref": "#/components/schemas/BarometricPressure"
	///    },
	///    "civicAddress": {
	///      "$ref": "#/components/schemas/CivicAddress"
	///    },
	///    "ecgi": {
	///      "$ref": "#/components/schemas/Ecgi"
	///    },
	///    "gnssPositioningDataList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GnssPositioningMethodAndUsage"
	///      },
	///      "maxItems": 9,
	///      "minItems": 0
	///    },
	///    "haGnssMetrics": {
	///      "$ref": "#/components/schemas/HighAccuracyGnssMetrics"
	///    },
	///    "localLocationEstimate": {
	///      "$ref": "#/components/schemas/LocalArea"
	///    },
	///    "locationEstimate": {
	///      "$ref": "#/components/schemas/GeographicArea"
	///    },
	///    "locationPrivacyVerResult": {
	///      "$ref": "#/components/schemas/LocationPrivacyVerResult"
	///    },
	///    "ncgi": {
	///      "$ref": "#/components/schemas/Ncgi"
	///    },
	///    "positioningDataList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PositioningMethodAndUsage"
	///      },
	///      "maxItems": 9,
	///      "minItems": 0
	///    },
	///    "servingLMFIdentification": {
	///      "$ref": "#/components/schemas/LMFIdentification"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "targetMmeName": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "targetMmeRealm": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "targetServingNode": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "timestampOfLocationEstimate": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "utranSrvccInd": {
	///      "type": "boolean"
	///    },
	///    "velocityEstimate": {
	///      "$ref": "#/components/schemas/VelocityEstimate"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ProvidePosInfo {
		#[serde(
			rename = "acceptedPeriodicEventInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub accepted_periodic_event_info: Option<PeriodicEventInfo>,
		#[serde(
			rename = "accuracyFulfilmentIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub accuracy_fulfilment_indicator: Option<AccuracyFulfilmentIndicator>,
		#[serde(
			rename = "achievedQos",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub achieved_qos: Option<MinorLocationQoS>,
		#[serde(
			rename = "ageOfLocationEstimate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub age_of_location_estimate: Option<AgeOfLocationEstimate>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub altitude: Option<Altitude>,
		#[serde(
			rename = "barometricPressure",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub barometric_pressure: Option<BarometricPressure>,
		#[serde(
			rename = "civicAddress",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub civic_address: Option<CivicAddress>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ecgi: Option<Ecgi>,
		#[serde(
			rename = "gnssPositioningDataList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub gnss_positioning_data_list: Vec<GnssPositioningMethodAndUsage>,
		#[serde(
			rename = "haGnssMetrics",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ha_gnss_metrics: Option<HighAccuracyGnssMetrics>,
		#[serde(
			rename = "localLocationEstimate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub local_location_estimate: Option<LocalArea>,
		#[serde(
			rename = "locationEstimate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub location_estimate: Option<GeographicArea>,
		#[serde(
			rename = "locationPrivacyVerResult",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub location_privacy_ver_result: Option<LocationPrivacyVerResult>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ncgi: Option<Ncgi>,
		#[serde(
			rename = "positioningDataList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub positioning_data_list: Vec<PositioningMethodAndUsage>,
		#[serde(
			rename = "servingLMFIdentification",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_lmf_identification: Option<LmfIdentification>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "targetMmeName",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_mme_name: Option<Fqdn>,
		#[serde(
			rename = "targetMmeRealm",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_mme_realm: Option<Fqdn>,
		#[serde(
			rename = "targetServingNode",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_serving_node: Option<NfInstanceId>,
		#[serde(
			rename = "timestampOfLocationEstimate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub timestamp_of_location_estimate: Option<DateTime>,
		#[serde(
			rename = "utranSrvccInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub utran_srvcc_ind: Option<bool>,
		#[serde(
			rename = "velocityEstimate",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub velocity_estimate: Option<VelocityEstimate>,
	}

	impl From<&ProvidePosInfo> for ProvidePosInfo {
		fn from(value: &ProvidePosInfo) -> Self {
			value.clone()
		}
	}

	/// Data related to PWS error included in a N2 Information Transfer failure
	/// response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data related to PWS error included in a N2 Information
	/// Transfer failure response",
	///  "type": "object",
	///  "required": [
	///    "namfCause"
	///  ],
	///  "properties": {
	///    "namfCause": {
	///      "type": "integer"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PwsErrorData {
		#[serde(rename = "namfCause")]
		pub namf_cause: i64,
	}

	impl From<&PwsErrorData> for PwsErrorData {
		fn from(value: &PwsErrorData) -> Self {
			value.clone()
		}
	}

	/// Represents a PWS related information data part
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a PWS related information data part",
	///  "type": "object",
	///  "required": [
	///    "messageIdentifier",
	///    "pwsContainer",
	///    "serialNumber"
	///  ],
	///  "properties": {
	///    "bcEmptyAreaList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GlobalRanNodeId"
	///      },
	///      "minItems": 1
	///    },
	///    "messageIdentifier": {
	///      "$ref": "#/components/schemas/Uint16"
	///    },
	///    "nfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "omcId": {
	///      "$ref": "#/components/schemas/OmcIdentifier"
	///    },
	///    "pwsContainer": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "sendRanResponse": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "serialNumber": {
	///      "$ref": "#/components/schemas/Uint16"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct PwsInformation {
		#[serde(
			rename = "bcEmptyAreaList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub bc_empty_area_list: Vec<GlobalRanNodeId>,
		#[serde(rename = "messageIdentifier")]
		pub message_identifier: Uint16,
		#[serde(rename = "nfId", default, skip_serializing_if = "Option::is_none")]
		pub nf_id: Option<NfInstanceId>,
		#[serde(rename = "omcId", default, skip_serializing_if = "Option::is_none")]
		pub omc_id: Option<OmcIdentifier>,
		#[serde(rename = "pwsContainer")]
		pub pws_container: N2InfoContent,
		#[serde(rename = "sendRanResponse", default)]
		pub send_ran_response: bool,
		#[serde(rename = "serialNumber")]
		pub serial_number: Uint16,
	}

	impl From<&PwsInformation> for PwsInformation {
		fn from(value: &PwsInformation) -> Self {
			value.clone()
		}
	}

	/// Data related PWS included in a N2 Information Transfer response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data related PWS included in a N2 Information Transfer
	/// response",
	///  "type": "object",
	///  "required": [
	///    "messageIdentifier",
	///    "ngapMessageType",
	///    "serialNumber"
	///  ],
	///  "properties": {
	///    "messageIdentifier": {
	///      "type": "integer"
	///    },
	///    "ngapMessageType": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "serialNumber": {
	///      "$ref": "#/components/schemas/Uint16"
	///    },
	///    "unknownTaiList": {
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
	pub struct PwsResponseData {
		#[serde(rename = "messageIdentifier")]
		pub message_identifier: i64,
		#[serde(rename = "ngapMessageType")]
		pub ngap_message_type: Uinteger,
		#[serde(rename = "serialNumber")]
		pub serial_number: Uint16,
		#[serde(
			rename = "unknownTaiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub unknown_tai_list: Vec<Tai>,
	}

	impl From<&PwsResponseData> for PwsResponseData {
		fn from(value: &PwsResponseData) -> Self {
			value.clone()
		}
	}

	/// Represents the QoS requirements.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the QoS requirements.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "5qi"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "resType"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "5qi": {
	///      "$ref": "#/components/schemas/5Qi"
	///    },
	///    "gfbrDl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "gfbrUl": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "pdb": {
	///      "$ref": "#/components/schemas/PacketDelBudget"
	///    },
	///    "per": {
	///      "$ref": "#/components/schemas/PacketErrRate"
	///    },
	///    "resType": {
	///      "$ref": "#/components/schemas/QosResourceType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum QosRequirement {
		#[default]
		Variant0 {
			#[serde(rename = "5qi")]
			five_qi: _5qi,
			#[serde(rename = "gfbrDl", default, skip_serializing_if = "Option::is_none")]
			gfbr_dl: Option<BitRate>,
			#[serde(rename = "gfbrUl", default, skip_serializing_if = "Option::is_none")]
			gfbr_ul: Option<BitRate>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			pdb: Option<PacketDelBudget>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			per: Option<PacketErrRate>,
		},
		Variant1 {
			#[serde(rename = "gfbrDl", default, skip_serializing_if = "Option::is_none")]
			gfbr_dl: Option<BitRate>,
			#[serde(rename = "gfbrUl", default, skip_serializing_if = "Option::is_none")]
			gfbr_ul: Option<BitRate>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			pdb: Option<PacketDelBudget>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			per: Option<PacketErrRate>,
			#[serde(rename = "resType")]
			res_type: QosResourceType,
		},
	}

	impl From<&QosRequirement> for QosRequirement {
		fn from(value: &QosRequirement) -> Self {
			value.clone()
		}
	}

	/// Represents the QoS Sustainability information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the QoS Sustainability information.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "qosFlowRetThd"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ranUeThrouThd"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "areaInfo": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "endTs": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "qosFlowRetThd": {
	///      "$ref": "#/components/schemas/RetainabilityThreshold"
	///    },
	///    "ranUeThrouThd": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "startTs": {
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
	pub enum QosSustainabilityInfo {
		#[default]
		Variant0 {
			#[serde(rename = "areaInfo", default, skip_serializing_if = "Option::is_none")]
			area_info: Option<NetworkAreaInfo>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "endTs", default, skip_serializing_if = "Option::is_none")]
			end_ts: Option<DateTime>,
			#[serde(rename = "qosFlowRetThd")]
			qos_flow_ret_thd: RetainabilityThreshold,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
			#[serde(rename = "startTs", default, skip_serializing_if = "Option::is_none")]
			start_ts: Option<DateTime>,
		},
		Variant1 {
			#[serde(rename = "areaInfo", default, skip_serializing_if = "Option::is_none")]
			area_info: Option<NetworkAreaInfo>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "endTs", default, skip_serializing_if = "Option::is_none")]
			end_ts: Option<DateTime>,
			#[serde(rename = "ranUeThrouThd")]
			ran_ue_throu_thd: BitRate,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
			#[serde(rename = "startTs", default, skip_serializing_if = "Option::is_none")]
			start_ts: Option<DateTime>,
		},
	}

	impl From<&QosSustainabilityInfo> for QosSustainabilityInfo {
		fn from(value: &QosSustainabilityInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates the usage ranking criterion between the high, medium and low
	/// usage UE.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the usage ranking criterion between the high,
	/// medium and low usage UE.",
	///  "type": "object",
	///  "required": [
	///    "highBase",
	///    "lowBase"
	///  ],
	///  "properties": {
	///    "highBase": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "lowBase": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RankingCriterion {
		#[serde(rename = "highBase")]
		pub high_base: SamplingRatio,
		#[serde(rename = "lowBase")]
		pub low_base: SamplingRatio,
	}

	impl From<&RankingCriterion> for RankingCriterion {
		fn from(value: &RankingCriterion) -> Self {
			value.clone()
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

	/// Indicates the RAT type for the transfer of N2 information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the RAT type for the transfer of N2
	/// information",
	///  "type": "string",
	///  "enum": [
	///    "E-UTRA",
	///    "NR"
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
	pub enum RatSelector {
		#[default]
		#[serde(rename = "E-UTRA")]
		EUtra,
		#[serde(rename = "NR")]
		Nr,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RatSelector> for RatSelector {
		fn from(value: &RatSelector) -> Self {
			value.clone()
		}
	}

	impl ToString for RatSelector {
		fn to_string(&self) -> String {
			match *self {
				Self::EUtra => "E-UTRA".to_string(),
				Self::Nr => "NR".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RatSelector {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"E-UTRA" => Ok(Self::EUtra),
				"NR" => Ok(Self::Nr),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RatSelector {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RatSelector {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RatSelector {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Event filter for REACHABILITY_REPORT event type
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Event filter for REACHABILITY_REPORT event type",
	///  "type": "string",
	///  "enum": [
	///    "UE_REACHABILITY_STATUS_CHANGE",
	///    "UE_REACHABLE_DL_TRAFFIC"
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
	pub enum ReachabilityFilter {
		#[default]
		#[serde(rename = "UE_REACHABILITY_STATUS_CHANGE")]
		UeReachabilityStatusChange,
		#[serde(rename = "UE_REACHABLE_DL_TRAFFIC")]
		UeReachableDlTraffic,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReachabilityFilter> for ReachabilityFilter {
		fn from(value: &ReachabilityFilter) -> Self {
			value.clone()
		}
	}

	impl ToString for ReachabilityFilter {
		fn to_string(&self) -> String {
			match *self {
				Self::UeReachabilityStatusChange => "UE_REACHABILITY_STATUS_CHANGE".to_string(),
				Self::UeReachableDlTraffic => "UE_REACHABLE_DL_TRAFFIC".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReachabilityFilter {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UE_REACHABILITY_STATUS_CHANGE" => Ok(Self::UeReachabilityStatusChange),
				"UE_REACHABLE_DL_TRAFFIC" => Ok(Self::UeReachableDlTraffic),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReachabilityFilter {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReachabilityFilter {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReachabilityFilter {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within the UE Reachability Info Notify
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within the UE Reachability Info Notify",
	///  "type": "object",
	///  "properties": {
	///    "reachableUeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ReachableUeInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "unreachableUeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
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
	pub struct ReachabilityNotificationData {
		#[serde(
			rename = "reachableUeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub reachable_ue_list: Vec<ReachableUeInfo>,
		#[serde(
			rename = "unreachableUeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub unreachable_ue_list: Vec<Supi>,
	}

	impl From<&ReachabilityNotificationData> for ReachabilityNotificationData {
		fn from(value: &ReachabilityNotificationData) -> Self {
			value.clone()
		}
	}

	/// Contains the reachable UE Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the reachable UE Information",
	///  "type": "object",
	///  "required": [
	///    "ueList"
	///  ],
	///  "properties": {
	///    "ueList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "userLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReachableUeInfo {
		#[serde(rename = "ueList")]
		pub ue_list: Vec<Supi>,
		#[serde(
			rename = "userLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub user_location: Option<UserLocation>,
	}

	impl From<&ReachableUeInfo> for ReachableUeInfo {
		fn from(value: &ReachableUeInfo) -> Self {
			value.clone()
		}
	}

	/// Possible values are:
	/// - TIME_SLOT_START: Indicates the order of time slot start.
	/// - RED_TRANS_EXP: Indicates the order of Redundant Transmission
	///   Experience.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- TIME_SLOT_START: Indicates the
	/// order of time slot start.\n- RED_TRANS_EXP: Indicates the order of
	/// Redundant Transmission Experience.\n",
	///  "type": "string",
	///  "enum": [
	///    "TIME_SLOT_START",
	///    "RED_TRANS_EXP"
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
	pub enum RedTransExpOrderingCriterion {
		#[default]
		#[serde(rename = "TIME_SLOT_START")]
		TimeSlotStart,
		#[serde(rename = "RED_TRANS_EXP")]
		RedTransExp,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RedTransExpOrderingCriterion> for RedTransExpOrderingCriterion {
		fn from(value: &RedTransExpOrderingCriterion) -> Self {
			value.clone()
		}
	}

	impl ToString for RedTransExpOrderingCriterion {
		fn to_string(&self) -> String {
			match *self {
				Self::TimeSlotStart => "TIME_SLOT_START".to_string(),
				Self::RedTransExp => "RED_TRANS_EXP".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RedTransExpOrderingCriterion {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TIME_SLOT_START" => Ok(Self::TimeSlotStart),
				"RED_TRANS_EXP" => Ok(Self::RedTransExp),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RedTransExpOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RedTransExpOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RedTransExpOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The redundant transmission experience related information. When
	/// subscribed event is "RED_TRANS_EXP", the "redTransInfos" attribute shall
	/// be included.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The redundant transmission experience related
	/// information. When subscribed event is \"RED_TRANS_EXP\", the
	/// \"redTransInfos\" attribute shall be included.\n",
	///  "type": "object",
	///  "required": [
	///    "redTransExps"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "redTransExps": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RedundantTransmissionExpPerTS"
	///      },
	///      "minItems": 1
	///    },
	///    "spatialValidCon": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RedundantTransmissionExpInfo {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(rename = "redTransExps")]
		pub red_trans_exps: Vec<RedundantTransmissionExpPerTs>,
		#[serde(
			rename = "spatialValidCon",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub spatial_valid_con: Option<NetworkAreaInfo>,
	}

	impl From<&RedundantTransmissionExpInfo> for RedundantTransmissionExpInfo {
		fn from(value: &RedundantTransmissionExpInfo) -> Self {
			value.clone()
		}
	}

	/// The redundant transmission experience per Time Slot.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The redundant transmission experience per Time Slot.",
	///  "type": "object",
	///  "required": [
	///    "obsvRedTransExp",
	///    "tsDuration",
	///    "tsStart"
	///  ],
	///  "properties": {
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "obsvRedTransExp": {
	///      "$ref": "#/components/schemas/ObservedRedundantTransExp"
	///    },
	///    "redTransStatus": {
	///      "description": "Redundant Transmission Status. Set to \"true\" if
	/// redundant transmission was activated, otherwise set to \"false\".
	/// Default value is \"false\" if omitted.\n",
	///      "type": "boolean"
	///    },
	///    "tsDuration": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "tsStart": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "ueRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RedundantTransmissionExpPerTs {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub confidence: Option<Uinteger>,
		#[serde(rename = "obsvRedTransExp")]
		pub obsv_red_trans_exp: ObservedRedundantTransExp,
		/// Redundant Transmission Status. Set to "true" if redundant
		/// transmission was activated, otherwise set to "false". Default value
		/// is "false" if omitted.
		#[serde(
			rename = "redTransStatus",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub red_trans_status: Option<bool>,
		#[serde(rename = "tsDuration")]
		pub ts_duration: DurationSec,
		#[serde(rename = "tsStart")]
		pub ts_start: DateTime,
		#[serde(rename = "ueRatio", default, skip_serializing_if = "Option::is_none")]
		pub ue_ratio: Option<SamplingRatio>,
	}

	impl From<&RedundantTransmissionExpPerTs> for RedundantTransmissionExpPerTs {
		fn from(value: &RedundantTransmissionExpPerTs) -> Self {
			value.clone()
		}
	}

	/// Represents other redundant transmission experience analytics
	/// requirements.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents other redundant transmission experience
	/// analytics requirements.",
	///  "type": "object",
	///  "properties": {
	///    "order": {
	///      "$ref": "#/components/schemas/MatchingDirection"
	///    },
	///    "redTOrderCriter": {
	///      "$ref": "#/components/schemas/RedTransExpOrderingCriterion"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RedundantTransmissionExpReq {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub order: Option<MatchingDirection>,
		#[serde(
			rename = "redTOrderCriter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub red_t_order_criter: Option<RedTransExpOrderingCriterion>,
	}

	impl From<&RedundantTransmissionExpReq> for RedundantTransmissionExpReq {
		fn from(value: &RedundantTransmissionExpReq) -> Self {
			value.clone()
		}
	}

	/// ReferenceId
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
	pub struct ReferenceId(pub i64);

	impl ::std::ops::Deref for ReferenceId {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ReferenceId> for i64 {
		fn from(value: ReferenceId) -> Self {
			value.0
		}
	}

	impl From<&ReferenceId> for ReferenceId {
		fn from(value: &ReferenceId) -> Self {
			value.clone()
		}
	}

	impl From<i64> for ReferenceId {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ReferenceId {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ReferenceId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReferenceId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReferenceId {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ReferenceId {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Registration Context Container used to send the UE context information,
	/// N1 message from UE, AN address etc during Registration with AMF
	/// re-allocation procedure
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Registration Context Container used to send the UE
	/// context information, N1 message from UE, AN address etc during
	/// Registration with AMF re-allocation procedure",
	///  "type": "object",
	///  "required": [
	///    "anN2ApId",
	///    "anType",
	///    "initialAmfName",
	///    "ranNodeId",
	///    "ueContext",
	///    "userLocation"
	///  ],
	///  "properties": {
	///    "allowedNssai": {
	///      "$ref": "#/components/schemas/AllowedNssai"
	///    },
	///    "anN2ApId": {
	///      "type": "integer"
	///    },
	///    "anN2IPv4Addr": {
	///      "$ref": "#/components/schemas/Ipv4Addr"
	///    },
	///    "anN2IPv6Addr": {
	///      "$ref": "#/components/schemas/Ipv6Addr"
	///    },
	///    "anType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "authenticatedInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ceModeBInd": {
	///      "$ref": "#/components/schemas/CeModeBInd"
	///    },
	///    "configuredNssai": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ConfiguredSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "iabNodeInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "initialAmfN2ApId": {
	///      "type": "integer"
	///    },
	///    "initialAmfName": {
	///      "$ref": "#/components/schemas/Fqdn"
	///    },
	///    "localTimeZone": {
	///      "$ref": "#/components/schemas/TimeZone"
	///    },
	///    "lteMInd": {
	///      "$ref": "#/components/schemas/LteMInd"
	///    },
	///    "npnAccessInfo": {
	///      "$ref": "#/components/schemas/NpnAccessInfo"
	///    },
	///    "ranNodeId": {
	///      "$ref": "#/components/schemas/GlobalRanNodeId"
	///    },
	///    "rejectedNssaiInPlmn": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "rejectedNssaiInTa": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Snssai"
	///      },
	///      "minItems": 1
	///    },
	///    "rrcEstCause": {
	///      "type": "string",
	///      "pattern": "^[0-9a-fA-F]+$"
	///    },
	///    "selectedPlmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "ueContext": {
	///      "$ref": "#/components/schemas/UeContext"
	///    },
	///    "ueContextRequest": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "userLocation": {
	///      "$ref": "#/components/schemas/UserLocation"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RegistrationContextContainer {
		#[serde(
			rename = "allowedNssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub allowed_nssai: Option<AllowedNssai>,
		#[serde(rename = "anN2ApId")]
		pub an_n2_ap_id: i64,
		#[serde(
			rename = "anN2IPv4Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub an_n2i_pv4_addr: Option<Ipv4Addr>,
		#[serde(
			rename = "anN2IPv6Addr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub an_n2i_pv6_addr: Option<Ipv6Addr>,
		#[serde(rename = "anType")]
		pub an_type: AccessType,
		#[serde(rename = "authenticatedInd", default)]
		pub authenticated_ind: bool,
		#[serde(
			rename = "ceModeBInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ce_mode_b_ind: Option<CeModeBInd>,
		#[serde(
			rename = "configuredNssai",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub configured_nssai: Vec<ConfiguredSnssai>,
		#[serde(rename = "iabNodeInd", default)]
		pub iab_node_ind: bool,
		#[serde(
			rename = "initialAmfN2ApId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub initial_amf_n2_ap_id: Option<i64>,
		#[serde(rename = "initialAmfName")]
		pub initial_amf_name: Fqdn,
		#[serde(
			rename = "localTimeZone",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub local_time_zone: Option<TimeZone>,
		#[serde(rename = "lteMInd", default, skip_serializing_if = "Option::is_none")]
		pub lte_m_ind: Option<LteMInd>,
		#[serde(
			rename = "npnAccessInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub npn_access_info: Option<NpnAccessInfo>,
		#[serde(rename = "ranNodeId")]
		pub ran_node_id: GlobalRanNodeId,
		#[serde(
			rename = "rejectedNssaiInPlmn",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub rejected_nssai_in_plmn: Vec<Snssai>,
		#[serde(
			rename = "rejectedNssaiInTa",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub rejected_nssai_in_ta: Vec<Snssai>,
		#[serde(
			rename = "rrcEstCause",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub rrc_est_cause: Option<RegistrationContextContainerRrcEstCause>,
		#[serde(
			rename = "selectedPlmnId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub selected_plmn_id: Option<PlmnId>,
		#[serde(rename = "ueContext")]
		pub ue_context: UeContext,
		#[serde(rename = "ueContextRequest", default)]
		pub ue_context_request: bool,
		#[serde(rename = "userLocation")]
		pub user_location: UserLocation,
	}

	impl From<&RegistrationContextContainer> for RegistrationContextContainer {
		fn from(value: &RegistrationContextContainer) -> Self {
			value.clone()
		}
	}

	/// RegistrationContextContainerRrcEstCause
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
	pub struct RegistrationContextContainerRrcEstCause(String);

	impl ::std::ops::Deref for RegistrationContextContainerRrcEstCause {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<RegistrationContextContainerRrcEstCause> for String {
		fn from(value: RegistrationContextContainerRrcEstCause) -> Self {
			value.0
		}
	}

	impl From<&RegistrationContextContainerRrcEstCause> for RegistrationContextContainerRrcEstCause {
		fn from(value: &RegistrationContextContainerRrcEstCause) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for RegistrationContextContainerRrcEstCause {
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

	impl ::std::convert::TryFrom<&str> for RegistrationContextContainerRrcEstCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for RegistrationContextContainerRrcEstCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for RegistrationContextContainerRrcEstCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for RegistrationContextContainerRrcEstCause {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
		}
	}

	/// The cause for triggering the release.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The cause for triggering the release.",
	///  "type": "string",
	///  "enum": [
	///    "SNPN_SNPN_MOBILITY",
	///    "NO_HR_AGREEMENT",
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
	pub enum ReleaseCause {
		#[default]
		#[serde(rename = "SNPN_SNPN_MOBILITY")]
		SnpnSnpnMobility,
		#[serde(rename = "NO_HR_AGREEMENT")]
		NoHrAgreement,
		#[serde(rename = "UNSPECIFIED")]
		Unspecified,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReleaseCause> for ReleaseCause {
		fn from(value: &ReleaseCause) -> Self {
			value.clone()
		}
	}

	impl ToString for ReleaseCause {
		fn to_string(&self) -> String {
			match *self {
				Self::SnpnSnpnMobility => "SNPN_SNPN_MOBILITY".to_string(),
				Self::NoHrAgreement => "NO_HR_AGREEMENT".to_string(),
				Self::Unspecified => "UNSPECIFIED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReleaseCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SNPN_SNPN_MOBILITY" => Ok(Self::SnpnSnpnMobility),
				"NO_HR_AGREEMENT" => Ok(Self::NoHrAgreement),
				"UNSPECIFIED" => Ok(Self::Unspecified),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReleaseCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReleaseCause {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReleaseCause {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// PDU session Id(s) and the cause for triggering the release.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "PDU session Id(s) and the cause for triggering the
	/// release.",
	///  "type": "object",
	///  "required": [
	///    "releaseCause",
	///    "releaseSessionList"
	///  ],
	///  "properties": {
	///    "releaseCause": {
	///      "$ref": "#/components/schemas/ReleaseCause"
	///    },
	///    "releaseSessionList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionId"
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
	pub struct ReleaseSessionInfo {
		#[serde(rename = "releaseCause")]
		pub release_cause: ReleaseCause,
		#[serde(rename = "releaseSessionList")]
		pub release_session_list: Vec<PduSessionId>,
	}

	impl From<&ReleaseSessionInfo> for ReleaseSessionInfo {
		fn from(value: &ReleaseSessionInfo) -> Self {
			value.clone()
		}
	}

	/// Number of required periodic event reports.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Number of required periodic event reports.",
	///  "type": "integer",
	///  "maximum": 8639999.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReportingAmount(pub i64);

	impl ::std::ops::Deref for ReportingAmount {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ReportingAmount> for i64 {
		fn from(value: ReportingAmount) -> Self {
			value.0
		}
	}

	impl From<&ReportingAmount> for ReportingAmount {
		fn from(value: &ReportingAmount) -> Self {
			value.clone()
		}
	}

	impl From<i64> for ReportingAmount {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ReportingAmount {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ReportingAmount {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportingAmount {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportingAmount {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ReportingAmount {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Indicates an area for event reporting.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates an area for event reporting.",
	///  "type": "object",
	///  "required": [
	///    "areaType"
	///  ],
	///  "properties": {
	///    "areaType": {
	///      "$ref": "#/components/schemas/ReportingAreaType"
	///    },
	///    "ecgi": {
	///      "$ref": "#/components/schemas/Ecgi"
	///    },
	///    "ncgi": {
	///      "$ref": "#/components/schemas/Ncgi"
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
	pub struct ReportingArea {
		#[serde(rename = "areaType")]
		pub area_type: ReportingAreaType,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ecgi: Option<Ecgi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ncgi: Option<Ncgi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub tai: Option<Tai>,
	}

	impl From<&ReportingArea> for ReportingArea {
		fn from(value: &ReportingArea) -> Self {
			value.clone()
		}
	}

	/// Indicates type of event reporting area.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates type of event reporting area.",
	///  "type": "string",
	///  "enum": [
	///    "EPS_TRACKING_AREA_IDENTITY",
	///    "E-UTRAN_CELL_GLOBAL_IDENTIFICATION",
	///    "5GS_TRACKING_AREA_IDENTITY",
	///    "NR_CELL_GLOBAL_IDENTITY"
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
	pub enum ReportingAreaType {
		#[default]
		#[serde(rename = "EPS_TRACKING_AREA_IDENTITY")]
		EpsTrackingAreaIdentity,
		#[serde(rename = "E-UTRAN_CELL_GLOBAL_IDENTIFICATION")]
		EUtranCellGlobalIdentification,
		#[serde(rename = "5GS_TRACKING_AREA_IDENTITY")]
		FiveGsTrackingAreaIdentity,
		#[serde(rename = "NR_CELL_GLOBAL_IDENTITY")]
		NrCellGlobalIdentity,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ReportingAreaType> for ReportingAreaType {
		fn from(value: &ReportingAreaType) -> Self {
			value.clone()
		}
	}

	impl ToString for ReportingAreaType {
		fn to_string(&self) -> String {
			match *self {
				Self::EpsTrackingAreaIdentity => "EPS_TRACKING_AREA_IDENTITY".to_string(),
				Self::EUtranCellGlobalIdentification => {
					"E-UTRAN_CELL_GLOBAL_IDENTIFICATION".to_string()
				}
				Self::FiveGsTrackingAreaIdentity => "5GS_TRACKING_AREA_IDENTITY".to_string(),
				Self::NrCellGlobalIdentity => "NR_CELL_GLOBAL_IDENTITY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ReportingAreaType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"EPS_TRACKING_AREA_IDENTITY" => Ok(Self::EpsTrackingAreaIdentity),
				"E-UTRAN_CELL_GLOBAL_IDENTIFICATION" => Ok(Self::EUtranCellGlobalIdentification),
				"5GS_TRACKING_AREA_IDENTITY" => Ok(Self::FiveGsTrackingAreaIdentity),
				"NR_CELL_GLOBAL_IDENTITY" => Ok(Self::NrCellGlobalIdentity),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ReportingAreaType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportingAreaType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportingAreaType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Maximum duration of event reporting.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Maximum duration of event reporting.",
	///  "type": "integer",
	///  "maximum": 8640000.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReportingDuration(pub i64);

	impl ::std::ops::Deref for ReportingDuration {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ReportingDuration> for i64 {
		fn from(value: ReportingDuration) -> Self {
			value.0
		}
	}

	impl From<&ReportingDuration> for ReportingDuration {
		fn from(value: &ReportingDuration) -> Self {
			value.clone()
		}
	}

	impl From<i64> for ReportingDuration {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ReportingDuration {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ReportingDuration {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportingDuration {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportingDuration {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ReportingDuration {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Represents the type of reporting that the subscription requires.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the type of reporting that the subscription
	/// requires.",
	///  "type": "object",
	///  "properties": {
	///    "grpRepTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "immRep": {
	///      "type": "boolean"
	///    },
	///    "maxReportNbr": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "monDur": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "notifFlag": {
	///      "$ref": "#/components/schemas/NotificationFlag"
	///    },
	///    "notifMethod": {
	///      "$ref": "#/components/schemas/schemas-NotificationMethod"
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
	///    "repPeriod": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "sampRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReportingInformation {
		#[serde(
			rename = "grpRepTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub grp_rep_time: Option<DurationSec>,
		#[serde(rename = "immRep", default, skip_serializing_if = "Option::is_none")]
		pub imm_rep: Option<bool>,
		#[serde(
			rename = "maxReportNbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_report_nbr: Option<Uinteger>,
		#[serde(rename = "monDur", default, skip_serializing_if = "Option::is_none")]
		pub mon_dur: Option<DateTime>,
		#[serde(rename = "notifFlag", default, skip_serializing_if = "Option::is_none")]
		pub notif_flag: Option<NotificationFlag>,
		#[serde(
			rename = "notifMethod",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub notif_method: Option<SchemasNotificationMethod>,
		/// Criteria for partitioning the UEs before applying the sampling
		/// ratio.
		#[serde(
			rename = "partitionCriteria",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub partition_criteria: Vec<PartitioningCriteria>,
		#[serde(rename = "repPeriod", default, skip_serializing_if = "Option::is_none")]
		pub rep_period: Option<DurationSec>,
		#[serde(rename = "sampRatio", default, skip_serializing_if = "Option::is_none")]
		pub samp_ratio: Option<SamplingRatio>,
	}

	impl From<&ReportingInformation> for ReportingInformation {
		fn from(value: &ReportingInformation) -> Self {
			value.clone()
		}
	}

	/// Event reporting periodic interval in seconds.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Event reporting periodic interval in seconds.",
	///  "type": "integer",
	///  "maximum": 8639999.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReportingInterval(pub i64);

	impl ::std::ops::Deref for ReportingInterval {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ReportingInterval> for i64 {
		fn from(value: ReportingInterval) -> Self {
			value.0
		}
	}

	impl From<&ReportingInterval> for ReportingInterval {
		fn from(value: &ReportingInterval) -> Self {
			value.clone()
		}
	}

	impl From<i64> for ReportingInterval {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ReportingInterval {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ReportingInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportingInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportingInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ReportingInterval {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Event reporting periodic interval in milliseconds.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Event reporting periodic interval in milliseconds.",
	///  "type": "integer",
	///  "maximum": 999.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct ReportingIntervalMs(pub i64);

	impl ::std::ops::Deref for ReportingIntervalMs {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<ReportingIntervalMs> for i64 {
		fn from(value: ReportingIntervalMs) -> Self {
			value.0
		}
	}

	impl From<&ReportingIntervalMs> for ReportingIntervalMs {
		fn from(value: &ReportingIntervalMs) -> Self {
			value.clone()
		}
	}

	impl From<i64> for ReportingIntervalMs {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for ReportingIntervalMs {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for ReportingIntervalMs {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ReportingIntervalMs {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ReportingIntervalMs {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for ReportingIntervalMs {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Data within Provide Location Information Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Provide Location Information Request",
	///  "type": "object",
	///  "properties": {
	///    "req5gsLoc": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "reqCurrentLoc": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "reqRatType": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "reqTimeZone": {
	///      "default": false,
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
	pub struct RequestLocInfo {
		#[serde(rename = "req5gsLoc", default)]
		pub req5gs_loc: bool,
		#[serde(rename = "reqCurrentLoc", default)]
		pub req_current_loc: bool,
		#[serde(rename = "reqRatType", default)]
		pub req_rat_type: bool,
		#[serde(rename = "reqTimeZone", default)]
		pub req_time_zone: bool,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&RequestLocInfo> for RequestLocInfo {
		fn from(value: &RequestLocInfo) -> Self {
			value.clone()
		}
	}

	/// Data within Provide Positioning Information Request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within Provide Positioning Information Request",
	///  "type": "object",
	///  "required": [
	///    "lcsClientType",
	///    "lcsLocation"
	///  ],
	///  "properties": {
	///    "additionalLcsSuppGADShapes": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupportedGADShapes"
	///      },
	///      "minItems": 1
	///    },
	///    "afID": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "areaEventInfo": {
	///      "$ref": "#/components/schemas/AreaEventInfo"
	///    },
	///    "codeWord": {
	///      "$ref": "#/components/schemas/CodeWord"
	///    },
	///    "externalClientIdentification": {
	///      "$ref": "#/components/schemas/ExternalClientIdentification"
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "hgmlcCallBackURI": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "integrityRequirements": {
	///      "$ref": "#/components/schemas/IntegrityRequirements"
	///    },
	///    "lcsClientType": {
	///      "$ref": "#/components/schemas/ExternalClientType"
	///    },
	///    "lcsLocation": {
	///      "$ref": "#/components/schemas/LocationType"
	///    },
	///    "lcsQoS": {
	///      "$ref": "#/components/schemas/LocationQoS"
	///    },
	///    "lcsServiceType": {
	///      "$ref": "#/components/schemas/LcsServiceType"
	///    },
	///    "lcsSupportedGADShapes": {
	///      "$ref": "#/components/schemas/SupportedGADShapes"
	///    },
	///    "ldrReference": {
	///      "$ref": "#/components/schemas/LdrReference"
	///    },
	///    "ldrType": {
	///      "$ref": "#/components/schemas/LdrType"
	///    },
	///    "locationNotificationUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "motionEventInfo": {
	///      "$ref": "#/components/schemas/MotionEventInfo"
	///    },
	///    "oldGuami": {
	///      "$ref": "#/components/schemas/Guami"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "periodicEventInfo": {
	///      "$ref": "#/components/schemas/PeriodicEventInfo"
	///    },
	///    "priority": {
	///      "$ref": "#/components/schemas/LcsPriority"
	///    },
	///    "reliableLocReq": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "scheduledLocTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "uePrivacyRequirements": {
	///      "$ref": "#/components/schemas/UePrivacyRequirements"
	///    },
	///    "velocityRequested": {
	///      "$ref": "#/components/schemas/VelocityRequested"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RequestPosInfo {
		#[serde(
			rename = "additionalLcsSuppGADShapes",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub additional_lcs_supp_gad_shapes: Vec<SupportedGadShapes>,
		#[serde(rename = "afID", default, skip_serializing_if = "Option::is_none")]
		pub af_id: Option<NfInstanceId>,
		#[serde(
			rename = "areaEventInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub area_event_info: Option<AreaEventInfo>,
		#[serde(rename = "codeWord", default, skip_serializing_if = "Option::is_none")]
		pub code_word: Option<CodeWord>,
		#[serde(
			rename = "externalClientIdentification",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub external_client_identification: Option<ExternalClientIdentification>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(
			rename = "hgmlcCallBackURI",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub hgmlc_call_back_uri: Option<Uri>,
		#[serde(
			rename = "integrityRequirements",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub integrity_requirements: Option<IntegrityRequirements>,
		#[serde(rename = "lcsClientType")]
		pub lcs_client_type: ExternalClientType,
		#[serde(rename = "lcsLocation")]
		pub lcs_location: LocationType,
		#[serde(rename = "lcsQoS", default, skip_serializing_if = "Option::is_none")]
		pub lcs_qo_s: Option<LocationQoS>,
		#[serde(
			rename = "lcsServiceType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lcs_service_type: Option<LcsServiceType>,
		#[serde(
			rename = "lcsSupportedGADShapes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lcs_supported_gad_shapes: Option<SupportedGadShapes>,
		#[serde(
			rename = "ldrReference",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ldr_reference: Option<LdrReference>,
		#[serde(rename = "ldrType", default, skip_serializing_if = "Option::is_none")]
		pub ldr_type: Option<LdrType>,
		#[serde(
			rename = "locationNotificationUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub location_notification_uri: Option<Uri>,
		#[serde(
			rename = "motionEventInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub motion_event_info: Option<MotionEventInfo>,
		#[serde(rename = "oldGuami", default, skip_serializing_if = "Option::is_none")]
		pub old_guami: Option<crate::common::common_models::Guami>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(
			rename = "periodicEventInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub periodic_event_info: Option<PeriodicEventInfo>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub priority: Option<LcsPriority>,
		#[serde(rename = "reliableLocReq", default)]
		pub reliable_loc_req: bool,
		#[serde(
			rename = "scheduledLocTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub scheduled_loc_time: Option<DateTime>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "uePrivacyRequirements",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_privacy_requirements: Option<UePrivacyRequirements>,
		#[serde(
			rename = "velocityRequested",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub velocity_requested: Option<VelocityRequested>,
	}

	impl From<&RequestPosInfo> for RequestPosInfo {
		fn from(value: &RequestPosInfo) -> Self {
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

	/// Indicates acceptable delay of location request.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates acceptable delay of location request.",
	///  "type": "string",
	///  "enum": [
	///    "LOW_DELAY",
	///    "DELAY_TOLERANT",
	///    "NO_DELAY"
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
	pub enum ResponseTime {
		#[default]
		#[serde(rename = "LOW_DELAY")]
		LowDelay,
		#[serde(rename = "DELAY_TOLERANT")]
		DelayTolerant,
		#[serde(rename = "NO_DELAY")]
		NoDelay,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ResponseTime> for ResponseTime {
		fn from(value: &ResponseTime) -> Self {
			value.clone()
		}
	}

	impl ToString for ResponseTime {
		fn to_string(&self) -> String {
			match *self {
				Self::LowDelay => "LOW_DELAY".to_string(),
				Self::DelayTolerant => "DELAY_TOLERANT".to_string(),
				Self::NoDelay => "NO_DELAY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ResponseTime {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"LOW_DELAY" => Ok(Self::LowDelay),
				"DELAY_TOLERANT" => Ok(Self::DelayTolerant),
				"NO_DELAY" => Ok(Self::NoDelay),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ResponseTime {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ResponseTime {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ResponseTime {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents a QoS flow retainability threshold.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents a QoS flow retainability threshold.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "allOf": [
	///        {
	///          "required": [
	///            "relFlowNum"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "relTimeUnit"
	///          ]
	///        }
	///      ]
	///    },
	///    {
	///      "required": [
	///        "relFlowRatio"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "relFlowNum": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "relFlowRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "relTimeUnit": {
	///      "$ref": "#/components/schemas/TimeUnit"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum RetainabilityThreshold {
		#[default]
		Variant0 {
			#[serde(rename = "relFlowNum")]
			rel_flow_num: Uinteger,
			#[serde(rename = "relTimeUnit")]
			rel_time_unit: TimeUnit,
		},
		Variant1 {
			#[serde(rename = "relFlowRatio")]
			rel_flow_ratio: SamplingRatio,
		},
	}

	impl From<&RetainabilityThreshold> for RetainabilityThreshold {
		fn from(value: &RetainabilityThreshold) -> Self {
			value.clone()
		}
	}

	/// Represents the registration state of a UE for an access type
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the registration state of a UE for an access
	/// type",
	///  "type": "object",
	///  "required": [
	///    "accessType",
	///    "rmState"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "rmState": {
	///      "$ref": "#/components/schemas/RmState"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct RmInfo {
		#[serde(rename = "accessType")]
		pub access_type: AccessType,
		#[serde(rename = "rmState")]
		pub rm_state: RmState,
	}

	impl From<&RmInfo> for RmInfo {
		fn from(value: &RmInfo) -> Self {
			value.clone()
		}
	}

	/// Describes the registration management state of a UE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the registration management state of a UE",
	///  "type": "string",
	///  "enum": [
	///    "REGISTERED",
	///    "DEREGISTERED"
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
	pub enum RmState {
		#[default]
		#[serde(rename = "REGISTERED")]
		Registered,
		#[serde(rename = "DEREGISTERED")]
		Deregistered,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&RmState> for RmState {
		fn from(value: &RmState) -> Self {
			value.clone()
		}
	}

	impl ToString for RmState {
		fn to_string(&self) -> String {
			match *self {
				Self::Registered => "REGISTERED".to_string(),
				Self::Deregistered => "DEREGISTERED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for RmState {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"REGISTERED" => Ok(Self::Registered),
				"DEREGISTERED" => Ok(Self::Deregistered),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for RmState {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for RmState {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for RmState {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// S1UeNetworkCapability
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
	pub struct S1UeNetworkCapability(pub Bytes);

	impl ::std::ops::Deref for S1UeNetworkCapability {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<S1UeNetworkCapability> for Bytes {
		fn from(value: S1UeNetworkCapability) -> Self {
			value.0
		}
	}

	impl From<&S1UeNetworkCapability> for S1UeNetworkCapability {
		fn from(value: &S1UeNetworkCapability) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for S1UeNetworkCapability {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for S1UeNetworkCapability {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for S1UeNetworkCapability {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for S1UeNetworkCapability {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for S1UeNetworkCapability {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for S1UeNetworkCapability {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Maximum time interval between consecutive evaluations by a UE of a
	/// trigger event.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Maximum time interval between consecutive evaluations
	/// by a UE of a trigger event.",
	///  "type": "integer",
	///  "maximum": 3600.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SamplingInterval(pub i64);

	impl ::std::ops::Deref for SamplingInterval {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<SamplingInterval> for i64 {
		fn from(value: SamplingInterval) -> Self {
			value.0
		}
	}

	impl From<&SamplingInterval> for SamplingInterval {
		fn from(value: &SamplingInterval) -> Self {
			value.clone()
		}
	}

	impl From<i64> for SamplingInterval {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SamplingInterval {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SamplingInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SamplingInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SamplingInterval {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SamplingInterval {
		fn to_string(&self) -> String {
			self.0.to_string()
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

	/// Indicates the security context type
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the security context type",
	///  "type": "string",
	///  "enum": [
	///    "NATIVE",
	///    "MAPPED"
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
	pub enum ScType {
		#[default]
		#[serde(rename = "NATIVE")]
		Native,
		#[serde(rename = "MAPPED")]
		Mapped,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&ScType> for ScType {
		fn from(value: &ScType) -> Self {
			value.clone()
		}
	}

	impl ToString for ScType {
		fn to_string(&self) -> String {
			match *self {
				Self::Native => "NATIVE".to_string(),
				Self::Mapped => "MAPPED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for ScType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"NATIVE" => Ok(Self::Native),
				"MAPPED" => Ok(Self::Mapped),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for ScType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for ScType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for ScType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SchemasAccuracy
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates value of accuracy.",
	///  "type": "number",
	///  "format": "float",
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasAccuracy(pub f32);

	impl ::std::ops::Deref for SchemasAccuracy {
		type Target = f32;
		fn deref(&self) -> &f32 {
			&self.0
		}
	}

	impl From<SchemasAccuracy> for f32 {
		fn from(value: SchemasAccuracy) -> Self {
			value.0
		}
	}

	impl From<&SchemasAccuracy> for SchemasAccuracy {
		fn from(value: &SchemasAccuracy) -> Self {
			value.clone()
		}
	}

	impl From<f32> for SchemasAccuracy {
		fn from(value: f32) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SchemasAccuracy {
		type Err = <f32 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SchemasAccuracy {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SchemasAccuracy {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SchemasAccuracy {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SchemasAccuracy {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Describes an event to be subscribed
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes an event to be subscribed",
	///  "type": "object",
	///  "required": [
	///    "type"
	///  ],
	///  "properties": {
	///    "areaList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-AmfEventArea"
	///      },
	///      "minItems": 1
	///    },
	///    "dispersionArea": {
	///      "$ref": "#/components/schemas/schemas-DispersionArea"
	///    },
	///    "idleStatusInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "immediateFlag": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "locationFilterList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/LocationFilter"
	///      },
	///      "minItems": 1
	///    },
	///    "maxReports": {
	///      "type": "integer"
	///    },
	///    "maxResponseTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "minInterval": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "nextPeriodicReportTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "nextReport": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "presenceInfoList": {
	///      "description": "A map(list of key-value pairs) where praId serves
	/// as key.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "reachabilityFilter": {
	///      "$ref": "#/components/schemas/ReachabilityFilter"
	///    },
	///    "refId": {
	///      "$ref": "#/components/schemas/ReferenceId"
	///    },
	///    "reportUeReachable": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "snssaiFilter": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtSnssai"
	///      },
	///      "minItems": 1
	///    },
	///    "targetArea": {
	///      "$ref": "#/components/schemas/schemas-TargetArea"
	///    },
	///    "trafficDescriptorList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-TrafficDescriptor"
	///      },
	///      "minItems": 1
	///    },
	///    "type": {
	///      "$ref": "#/components/schemas/AmfEventType"
	///    },
	///    "udmDetectInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ueInAreaFilter": {
	///      "$ref": "#/components/schemas/UeInAreaFilter"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasAmfEvent {
		#[serde(rename = "areaList", default, skip_serializing_if = "Vec::is_empty")]
		pub area_list: Vec<SchemasAmfEventArea>,
		#[serde(
			rename = "dispersionArea",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub dispersion_area: Option<SchemasDispersionArea>,
		#[serde(rename = "idleStatusInd", default)]
		pub idle_status_ind: bool,
		#[serde(rename = "immediateFlag", default)]
		pub immediate_flag: bool,
		#[serde(
			rename = "locationFilterList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub location_filter_list: Vec<LocationFilter>,
		#[serde(
			rename = "maxReports",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_reports: Option<i64>,
		#[serde(
			rename = "maxResponseTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_response_time: Option<DurationSec>,
		#[serde(
			rename = "minInterval",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub min_interval: Option<DurationSec>,
		#[serde(
			rename = "nextPeriodicReportTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_periodic_report_time: Option<DateTime>,
		#[serde(
			rename = "nextReport",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub next_report: Option<DateTime>,
		/// A map(list of key-value pairs) where praId serves as key.
		#[serde(
			rename = "presenceInfoList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub presence_info_list: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(
			rename = "reachabilityFilter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reachability_filter: Option<ReachabilityFilter>,
		#[serde(rename = "refId", default, skip_serializing_if = "Option::is_none")]
		pub ref_id: Option<ReferenceId>,
		#[serde(rename = "reportUeReachable", default)]
		pub report_ue_reachable: bool,
		#[serde(
			rename = "snssaiFilter",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub snssai_filter: Vec<ExtSnssai>,
		#[serde(
			rename = "targetArea",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_area: Option<SchemasTargetArea>,
		#[serde(
			rename = "trafficDescriptorList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub traffic_descriptor_list: Vec<SchemasTrafficDescriptor>,
		#[serde(rename = "type")]
		pub type_: AmfEventType,
		#[serde(rename = "udmDetectInd", default)]
		pub udm_detect_ind: bool,
		#[serde(
			rename = "ueInAreaFilter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_in_area_filter: Option<UeInAreaFilter>,
	}

	impl From<&SchemasAmfEvent> for SchemasAmfEvent {
		fn from(value: &SchemasAmfEvent) -> Self {
			value.clone()
		}
	}

	/// Represents an area to be monitored by an AMF event
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an area to be monitored by an AMF event",
	///  "type": "object",
	///  "properties": {
	///    "ladnInfo": {
	///      "$ref": "#/components/schemas/schemas-LadnInfo"
	///    },
	///    "nsiId": {
	///      "$ref": "#/components/schemas/NsiId"
	///    },
	///    "presenceInfo": {
	///      "$ref": "#/components/schemas/PresenceInfo"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasAmfEventArea {
		#[serde(rename = "ladnInfo", default, skip_serializing_if = "Option::is_none")]
		pub ladn_info: Option<SchemasLadnInfo>,
		#[serde(rename = "nsiId", default, skip_serializing_if = "Option::is_none")]
		pub nsi_id: Option<NsiId>,
		#[serde(
			rename = "presenceInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub presence_info: Option<PresenceInfo>,
		#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
		pub s_nssai: Option<Snssai>,
	}

	impl From<&SchemasAmfEventArea> for SchemasAmfEventArea {
		fn from(value: &SchemasAmfEventArea) -> Self {
			value.clone()
		}
	}

	/// Describes how the reports shall be generated by a subscribed event
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes how the reports shall be generated by a
	/// subscribed event",
	///  "type": "object",
	///  "required": [
	///    "trigger"
	///  ],
	///  "properties": {
	///    "expiry": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "maxReports": {
	///      "type": "integer"
	///    },
	///    "notifFlag": {
	///      "$ref": "#/components/schemas/NotificationFlag"
	///    },
	///    "partitioningCriteria": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PartitioningCriteria"
	///      },
	///      "minItems": 1
	///    },
	///    "repPeriod": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "sampRatio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "trigger": {
	///      "$ref": "#/components/schemas/AmfEventTrigger"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasAmfEventMode {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub expiry: Option<DateTime>,
		#[serde(
			rename = "maxReports",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub max_reports: Option<i64>,
		#[serde(rename = "notifFlag", default, skip_serializing_if = "Option::is_none")]
		pub notif_flag: Option<NotificationFlag>,
		#[serde(
			rename = "partitioningCriteria",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub partitioning_criteria: Vec<PartitioningCriteria>,
		#[serde(rename = "repPeriod", default, skip_serializing_if = "Option::is_none")]
		pub rep_period: Option<DurationSec>,
		#[serde(rename = "sampRatio", default, skip_serializing_if = "Option::is_none")]
		pub samp_ratio: Option<SamplingRatio>,
		pub trigger: AmfEventTrigger,
	}

	impl From<&SchemasAmfEventMode> for SchemasAmfEventMode {
		fn from(value: &SchemasAmfEventMode) -> Self {
			value.clone()
		}
	}

	/// Represents an individual event subscription resource on AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an individual event subscription resource on
	/// AMF",
	///  "type": "object",
	///  "required": [
	///    "eventList",
	///    "eventNotifyUri",
	///    "nfId",
	///    "notifyCorrelationId"
	///  ],
	///  "properties": {
	///    "anyUE": {
	///      "type": "boolean"
	///    },
	///    "eventList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-AmfEvent"
	///      },
	///      "minItems": 1
	///    },
	///    "eventNotifyUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "excludeGpsiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "excludeSupiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
	///    },
	///    "groupId": {
	///      "$ref": "#/components/schemas/GroupId"
	///    },
	///    "includeGpsiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "includeSupiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "nfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "notifyCorrelationId": {
	///      "type": "string"
	///    },
	///    "options": {
	///      "$ref": "#/components/schemas/schemas-AmfEventMode"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "sourceNfType": {
	///      "$ref": "#/components/schemas/NFType"
	///    },
	///    "subsChangeNotifyCorrelationId": {
	///      "type": "string"
	///    },
	///    "subsChangeNotifyUri": {
	///      "$ref": "#/components/schemas/Uri"
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
	pub struct SchemasAmfEventSubscription {
		#[serde(rename = "anyUE", default, skip_serializing_if = "Option::is_none")]
		pub any_ue: Option<bool>,
		#[serde(rename = "eventList")]
		pub event_list: Vec<SchemasAmfEvent>,
		#[serde(rename = "eventNotifyUri")]
		pub event_notify_uri: Uri,
		#[serde(
			rename = "excludeGpsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub exclude_gpsi_list: Vec<Gpsi>,
		#[serde(
			rename = "excludeSupiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub exclude_supi_list: Vec<Supi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
		pub group_id: Option<GroupId>,
		#[serde(
			rename = "includeGpsiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub include_gpsi_list: Vec<Gpsi>,
		#[serde(
			rename = "includeSupiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub include_supi_list: Vec<Supi>,
		#[serde(rename = "nfId")]
		pub nf_id: NfInstanceId,
		#[serde(rename = "notifyCorrelationId")]
		pub notify_correlation_id: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub options: Option<SchemasAmfEventMode>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		#[serde(
			rename = "sourceNfType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub source_nf_type: Option<NfType>,
		#[serde(
			rename = "subsChangeNotifyCorrelationId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_change_notify_correlation_id: Option<String>,
		#[serde(
			rename = "subsChangeNotifyUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub subs_change_notify_uri: Option<Uri>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
	}

	impl From<&SchemasAmfEventSubscription> for SchemasAmfEventSubscription {
		fn from(value: &SchemasAmfEventSubscription) -> Self {
			value.clone()
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

	/// integer between and including 1 and 7 denoting a weekday. 1 shall
	/// indicate Monday, and the subsequent weekdays shall be indicated with the
	/// next higher numbers. 7 shall indicate Sunday.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "integer between and including 1 and 7 denoting a
	/// weekday. 1 shall indicate Monday, and the subsequent weekdays shall be
	/// indicated with the next higher numbers. 7 shall indicate Sunday.",
	///  "type": "integer",
	///  "maximum": 7.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasDayOfWeek(pub i64);

	impl ::std::ops::Deref for SchemasDayOfWeek {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<SchemasDayOfWeek> for i64 {
		fn from(value: SchemasDayOfWeek) -> Self {
			value.0
		}
	}

	impl From<&SchemasDayOfWeek> for SchemasDayOfWeek {
		fn from(value: &SchemasDayOfWeek) -> Self {
			value.clone()
		}
	}

	impl From<i64> for SchemasDayOfWeek {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SchemasDayOfWeek {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SchemasDayOfWeek {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SchemasDayOfWeek {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SchemasDayOfWeek {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SchemasDayOfWeek {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Dispersion Area
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Dispersion Area",
	///  "type": "object",
	///  "properties": {
	///    "ecgiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ecgi"
	///      },
	///      "minItems": 1
	///    },
	///    "n3gaInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ncgiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Ncgi"
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
	pub struct SchemasDispersionArea {
		#[serde(rename = "ecgiList", default, skip_serializing_if = "Vec::is_empty")]
		pub ecgi_list: Vec<Ecgi>,
		#[serde(rename = "n3gaInd", default)]
		pub n3ga_ind: bool,
		#[serde(rename = "ncgiList", default, skip_serializing_if = "Vec::is_empty")]
		pub ncgi_list: Vec<Ncgi>,
		#[serde(rename = "taiList", default, skip_serializing_if = "Vec::is_empty")]
		pub tai_list: Vec<Tai>,
	}

	impl From<&SchemasDispersionArea> for SchemasDispersionArea {
		fn from(value: &SchemasDispersionArea) -> Self {
			value.clone()
		}
	}

	/// LADN Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "LADN Information",
	///  "type": "object",
	///  "required": [
	///    "ladn"
	///  ],
	///  "properties": {
	///    "ladn": {
	///      "type": "string"
	///    },
	///    "presence": {
	///      "$ref": "#/components/schemas/PresenceState"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasLadnInfo {
		pub ladn: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub presence: Option<PresenceState>,
	}

	impl From<&SchemasLadnInfo> for SchemasLadnInfo {
		fn from(value: &SchemasLadnInfo) -> Self {
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
	/// service consumer requests the number of UEs.",
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
	pub struct SchemasNetworkAreaInfo {
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

	impl From<&SchemasNetworkAreaInfo> for SchemasNetworkAreaInfo {
		fn from(value: &SchemasNetworkAreaInfo) -> Self {
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
	pub enum SchemasNotificationMethod {
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

	impl From<&SchemasNotificationMethod> for SchemasNotificationMethod {
		fn from(value: &SchemasNotificationMethod) -> Self {
			value.clone()
		}
	}

	impl ToString for SchemasNotificationMethod {
		fn to_string(&self) -> String {
			match *self {
				Self::Periodic => "PERIODIC".to_string(),
				Self::OneTime => "ONE_TIME".to_string(),
				Self::OnEventDetection => "ON_EVENT_DETECTION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SchemasNotificationMethod {
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

	impl std::convert::TryFrom<&str> for SchemasNotificationMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SchemasNotificationMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SchemasNotificationMethod {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents an offered scheduled communication time.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an offered scheduled communication time.",
	///  "type": "object",
	///  "properties": {
	///    "daysOfWeek": {
	///      "description": "Identifies the day(s) of the week. If absent, it
	/// indicates every day of the week.",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/schemas-DayOfWeek"
	///      },
	///      "maxItems": 6,
	///      "minItems": 1
	///    },
	///    "timeOfDayEnd": {
	///      "$ref": "#/components/schemas/schemas-TimeOfDay"
	///    },
	///    "timeOfDayStart": {
	///      "$ref": "#/components/schemas/schemas-TimeOfDay"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasScheduledCommunicationTime {
		/// Identifies the day(s) of the week. If absent, it indicates every day
		/// of the week.
		#[serde(rename = "daysOfWeek", default, skip_serializing_if = "Vec::is_empty")]
		pub days_of_week: Vec<SchemasDayOfWeek>,
		#[serde(
			rename = "timeOfDayEnd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_of_day_end: Option<SchemasTimeOfDay>,
		#[serde(
			rename = "timeOfDayStart",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub time_of_day_start: Option<SchemasTimeOfDay>,
	}

	impl From<&SchemasScheduledCommunicationTime> for SchemasScheduledCommunicationTime {
		fn from(value: &SchemasScheduledCommunicationTime) -> Self {
			value.clone()
		}
	}

	/// TA list or TAI range list or any TA
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "TA list or TAI range list or any TA",
	///  "type": "object",
	///  "properties": {
	///    "anyTa": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "taList": {
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
	pub struct SchemasTargetArea {
		#[serde(rename = "anyTa", default)]
		pub any_ta: bool,
		#[serde(rename = "taList", default, skip_serializing_if = "Vec::is_empty")]
		pub ta_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&SchemasTargetArea> for SchemasTargetArea {
		fn from(value: &SchemasTargetArea) -> Self {
			value.clone()
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
	/// 8 hours behind UTC).",
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
	pub struct SchemasTimeOfDay(pub String);

	impl ::std::ops::Deref for SchemasTimeOfDay {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SchemasTimeOfDay> for String {
		fn from(value: SchemasTimeOfDay) -> Self {
			value.0
		}
	}

	impl From<&SchemasTimeOfDay> for SchemasTimeOfDay {
		fn from(value: &SchemasTimeOfDay) -> Self {
			value.clone()
		}
	}

	impl From<String> for SchemasTimeOfDay {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SchemasTimeOfDay {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for SchemasTimeOfDay {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Represents the Traffic Descriptor
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the Traffic Descriptor",
	///  "type": "object",
	///  "properties": {
	///    "dddTrafficDescriptorList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DddTrafficDescriptor"
	///      },
	///      "minItems": 1
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SchemasTrafficDescriptor {
		#[serde(
			rename = "dddTrafficDescriptorList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ddd_traffic_descriptor_list: Vec<DddTrafficDescriptor>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
		pub s_nssai: Option<Snssai>,
	}

	impl From<&SchemasTrafficDescriptor> for SchemasTrafficDescriptor {
		fn from(value: &SchemasTrafficDescriptor) -> Self {
			value.clone()
		}
	}

	/// Represents SEAF data derived from data received from AUSF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents SEAF data derived from data received from
	/// AUSF",
	///  "type": "object",
	///  "required": [
	///    "keyAmf",
	///    "ngKsi"
	///  ],
	///  "properties": {
	///    "keyAmf": {
	///      "$ref": "#/components/schemas/KeyAmf"
	///    },
	///    "keyAmfChangeInd": {
	///      "type": "boolean"
	///    },
	///    "keyAmfHDerivationInd": {
	///      "type": "boolean"
	///    },
	///    "ncc": {
	///      "type": "integer",
	///      "maximum": 7.0,
	///      "minimum": 0.0
	///    },
	///    "ngKsi": {
	///      "$ref": "#/components/schemas/NgKsi"
	///    },
	///    "nh": {
	///      "type": "string",
	///      "pattern": "^[A-Fa-f0-9]+$"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SeafData {
		#[serde(rename = "keyAmf")]
		pub key_amf: KeyAmf,
		#[serde(
			rename = "keyAmfChangeInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub key_amf_change_ind: Option<bool>,
		#[serde(
			rename = "keyAmfHDerivationInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub key_amf_h_derivation_ind: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ncc: Option<i64>,
		#[serde(rename = "ngKsi")]
		pub ng_ksi: NgKsi,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub nh: Option<SeafDataNh>,
	}

	impl From<&SeafData> for SeafData {
		fn from(value: &SeafData) -> Self {
			value.clone()
		}
	}

	/// SeafDataNh
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
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
	pub struct SeafDataNh(String);

	impl ::std::ops::Deref for SeafDataNh {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<SeafDataNh> for String {
		fn from(value: SeafDataNh) -> Self {
			value.0
		}
	}

	impl From<&SeafDataNh> for SeafDataNh {
		fn from(value: &SeafDataNh) -> Self {
			value.clone()
		}
	}

	impl ::std::str::FromStr for SeafDataNh {
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

	impl ::std::convert::TryFrom<&str> for SeafDataNh {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<&String> for SeafDataNh {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl ::std::convert::TryFrom<String> for SeafDataNh {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl<'de> ::serde::Deserialize<'de> for SeafDataNh {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: ::serde::Deserializer<'de>,
		{
			String::deserialize(deserializer)?
				.parse()
				.map_err(|e: self::error::ConversionError| {
					<D::Error as ::serde::de::Error>::custom(e.to_string())
				})
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

	/// Represents the N4 Session inactivity timer.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the N4 Session inactivity timer.",
	///  "type": "object",
	///  "required": [
	///    "n4SessId",
	///    "sessInactiveTimer"
	///  ],
	///  "properties": {
	///    "n4SessId": {
	///      "$ref": "#/components/schemas/PduSessionId"
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
	pub struct SessInactTimerForUeComm {
		#[serde(rename = "n4SessId")]
		pub n4_sess_id: PduSessionId,
		#[serde(rename = "sessInactiveTimer")]
		pub sess_inactive_timer: DurationSec,
	}

	impl From<&SessInactTimerForUeComm> for SessInactTimerForUeComm {
		fn from(value: &SessInactTimerForUeComm) -> Self {
			value.clone()
		}
	}

	/// Contains load level information applicable for one or several slices.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains load level information applicable for one or
	/// several slices.",
	///  "type": "object",
	///  "required": [
	///    "loadLevelInformation",
	///    "snssais"
	///  ],
	///  "properties": {
	///    "loadLevelInformation": {
	///      "$ref": "#/components/schemas/LoadLevelInformation"
	///    },
	///    "snssais": {
	///      "description": "Identification(s) of network slice to which the
	/// subscription applies.",
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
	pub struct SliceLoadLevelInformation {
		#[serde(rename = "loadLevelInformation")]
		pub load_level_information: LoadLevelInformation,
		/// Identification(s) of network slice to which the subscription
		/// applies.
		pub snssais: Vec<Snssai>,
	}

	impl From<&SliceLoadLevelInformation> for SliceLoadLevelInformation {
		fn from(value: &SliceLoadLevelInformation) -> Self {
			value.clone()
		}
	}

	/// Represents the small data rate status
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the small data rate status",
	///  "type": "object",
	///  "required": [
	///    "Dnn",
	///    "SmallDataRateStatus",
	///    "Snssai"
	///  ],
	///  "properties": {
	///    "Dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "SmallDataRateStatus": {
	///      "$ref": "#/components/schemas/SmallDataRateStatus"
	///    },
	///    "Snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmallDataRateStatusInfo {
		#[serde(rename = "Dnn")]
		pub dnn: Dnn,
		#[serde(rename = "SmallDataRateStatus")]
		pub small_data_rate_status: SmallDataRateStatus,
		#[serde(rename = "Snssai")]
		pub snssai: Snssai,
	}

	impl From<&SmallDataRateStatusInfo> for SmallDataRateStatusInfo {
		fn from(value: &SmallDataRateStatusInfo) -> Self {
			value.clone()
		}
	}

	/// Represents the Session Management congestion control experience
	/// information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the Session Management congestion control
	/// experience information.",
	///  "type": "object",
	///  "required": [
	///    "smcceUeList"
	///  ],
	///  "properties": {
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "smcceUeList": {
	///      "$ref": "#/components/schemas/SmcceUeList"
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
	pub struct SmcceInfo {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(rename = "smcceUeList")]
		pub smcce_ue_list: SmcceUeList,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
	}

	impl From<&SmcceInfo> for SmcceInfo {
		fn from(value: &SmcceInfo) -> Self {
			value.clone()
		}
	}

	/// Represents the List of UEs classified based on experience level of
	/// Session Management  congestion control.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the List of UEs classified based on
	/// experience level of Session Management  congestion control.\n",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "highLevel"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "mediumLevel"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "lowLevel"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "highLevel": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "lowLevel": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
	///      },
	///      "minItems": 1
	///    },
	///    "mediumLevel": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
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
	pub enum SmcceUeList {
		#[default]
		Variant0 {
			#[serde(rename = "highLevel")]
			high_level: Vec<Supi>,
		},
		Variant1 {
			#[serde(rename = "mediumLevel")]
			medium_level: Vec<Supi>,
		},
		Variant2 {
			#[serde(rename = "lowLevel")]
			low_level: Vec<Supi>,
		},
	}

	impl From<&SmcceUeList> for SmcceUeList {
		fn from(value: &SmcceUeList) -> Self {
			value.clone()
		}
	}

	/// Indicates the I-SMF or V-SMF change or removal
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the I-SMF or V-SMF change or removal",
	///  "type": "string",
	///  "enum": [
	///    "CHANGED",
	///    "REMOVED"
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
	pub enum SmfChangeIndication {
		#[default]
		#[serde(rename = "CHANGED")]
		Changed,
		#[serde(rename = "REMOVED")]
		Removed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmfChangeIndication> for SmfChangeIndication {
		fn from(value: &SmfChangeIndication) -> Self {
			value.clone()
		}
	}

	impl ToString for SmfChangeIndication {
		fn to_string(&self) -> String {
			match *self {
				Self::Changed => "CHANGED".to_string(),
				Self::Removed => "REMOVED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmfChangeIndication {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"CHANGED" => Ok(Self::Changed),
				"REMOVED" => Ok(Self::Removed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmfChangeIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmfChangeIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmfChangeIndication {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// SMF change information for PDU session(s)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "SMF change information for PDU session(s)",
	///  "type": "object",
	///  "required": [
	///    "pduSessionIdList",
	///    "smfChangeInd"
	///  ],
	///  "properties": {
	///    "pduSessionIdList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionId"
	///      },
	///      "minItems": 1
	///    },
	///    "smfChangeInd": {
	///      "$ref": "#/components/schemas/SmfChangeIndication"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmfChangeInfo {
		#[serde(rename = "pduSessionIdList")]
		pub pdu_session_id_list: Vec<PduSessionId>,
		#[serde(rename = "smfChangeInd")]
		pub smf_change_ind: SmfChangeIndication,
	}

	impl From<&SmfChangeInfo> for SmfChangeInfo {
		fn from(value: &SmfChangeInfo) -> Self {
			value.clone()
		}
	}

	/// Represents the SMF Selection information that may be replaced by the
	/// PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the SMF Selection information that may be
	/// replaced by the PCF.",
	///  "type": [
	///    "object",
	///    "null"
	///  ],
	///  "properties": {
	///    "candidates": {
	///      "description": "Contains the list of DNNs per S-NSSAI that are
	/// candidates for replacement. The snssai attribute within the
	/// CandidateForReplacement data type is the key of the map.\n",
	///      "type": [
	///        "object",
	///        "null"
	///      ],
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/CandidateForReplacement"
	///      }
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "mappingSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "unsuppDnn": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmfSelectionData(pub Option<SmfSelectionDataInner>);

	impl ::std::ops::Deref for SmfSelectionData {
		type Target = Option<SmfSelectionDataInner>;
		fn deref(&self) -> &Option<SmfSelectionDataInner> {
			&self.0
		}
	}

	impl From<SmfSelectionData> for Option<SmfSelectionDataInner> {
		fn from(value: SmfSelectionData) -> Self {
			value.0
		}
	}

	impl From<&SmfSelectionData> for SmfSelectionData {
		fn from(value: &SmfSelectionData) -> Self {
			value.clone()
		}
	}

	impl From<Option<SmfSelectionDataInner>> for SmfSelectionData {
		fn from(value: Option<SmfSelectionDataInner>) -> Self {
			Self(value)
		}
	}

	/// Represents the SMF Selection information that may be replaced by the
	/// PCF.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the SMF Selection information that may be
	/// replaced by the PCF.",
	///  "type": "object",
	///  "properties": {
	///    "candidates": {
	///      "description": "Contains the list of DNNs per S-NSSAI that are
	/// candidates for replacement. The snssai attribute within the
	/// CandidateForReplacement data type is the key of the map.\n",
	///      "type": [
	///        "object",
	///        "null"
	///      ],
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/CandidateForReplacement"
	///      }
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "mappingSnssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "unsuppDnn": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SmfSelectionDataInner {
		/// Contains the list of DNNs per S-NSSAI that are candidates for
		/// replacement. The snssai attribute within the CandidateForReplacement
		/// data type is the key of the map.
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub candidates: Option<::std::collections::HashMap<String, CandidateForReplacement>>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(
			rename = "mappingSnssai",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mapping_snssai: Option<Snssai>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
		#[serde(rename = "unsuppDnn", default, skip_serializing_if = "Option::is_none")]
		pub unsupp_dnn: Option<bool>,
	}

	impl From<&SmfSelectionDataInner> for SmfSelectionDataInner {
		fn from(value: &SmfSelectionDataInner) -> Self {
			value.clone()
		}
	}

	/// Indicates the supported SMS delivery of a UE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the supported SMS delivery of a UE",
	///  "type": "string",
	///  "enum": [
	///    "3GPP",
	///    "NON_3GPP",
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
	pub enum SmsSupport {
		#[default]
		#[serde(rename = "3GPP")]
		ThreeGpp,
		#[serde(rename = "NON_3GPP")]
		Non3gpp,
		#[serde(rename = "BOTH")]
		Both,
		#[serde(rename = "NONE")]
		None,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&SmsSupport> for SmsSupport {
		fn from(value: &SmsSupport) -> Self {
			value.clone()
		}
	}

	impl ToString for SmsSupport {
		fn to_string(&self) -> String {
			match *self {
				Self::ThreeGpp => "3GPP".to_string(),
				Self::Non3gpp => "NON_3GPP".to_string(),
				Self::Both => "BOTH".to_string(),
				Self::None => "NONE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for SmsSupport {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"3GPP" => Ok(Self::ThreeGpp),
				"NON_3GPP" => Ok(Self::Non3gpp),
				"BOTH" => Ok(Self::Both),
				"NONE" => Ok(Self::None),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for SmsSupport {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SmsSupport {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SmsSupport {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// List of restricted or unrestricted S-NSSAIs per TAI(s)
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "List of restricted or unrestricted S-NSSAIs per
	/// TAI(s)",
	///  "type": "object",
	///  "required": [
	///    "reportingArea"
	///  ],
	///  "properties": {
	///    "accessTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AccessType"
	///      },
	///      "minItems": 1
	///    },
	///    "reportingArea": {
	///      "$ref": "#/components/schemas/TargetArea"
	///    },
	///    "supportedSnssaiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SupportedSnssai"
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
	pub struct SnssaiTaiMapping {
		#[serde(
			rename = "accessTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub access_type_list: Vec<AccessType>,
		#[serde(rename = "reportingArea")]
		pub reporting_area: TargetArea,
		#[serde(
			rename = "supportedSnssaiList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub supported_snssai_list: Vec<SupportedSnssai>,
	}

	impl From<&SnssaiTaiMapping> for SnssaiTaiMapping {
		fn from(value: &SnssaiTaiMapping) -> Self {
			value.clone()
		}
	}

	/// SpeedUncertainty
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates value of speed uncertainty.",
	///  "type": "number",
	///  "format": "float",
	///  "maximum": 255.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct SpeedUncertainty(pub f32);

	impl ::std::ops::Deref for SpeedUncertainty {
		type Target = f32;
		fn deref(&self) -> &f32 {
			&self.0
		}
	}

	impl From<SpeedUncertainty> for f32 {
		fn from(value: SpeedUncertainty) -> Self {
			value.0
		}
	}

	impl From<&SpeedUncertainty> for SpeedUncertainty {
		fn from(value: &SpeedUncertainty) -> Self {
			value.clone()
		}
	}

	impl From<f32> for SpeedUncertainty {
		fn from(value: f32) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SpeedUncertainty {
		type Err = <f32 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SpeedUncertainty {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SpeedUncertainty {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SpeedUncertainty {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SpeedUncertainty {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Enumeration for AMF status
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Enumeration for AMF status",
	///  "type": "string",
	///  "enum": [
	///    "AMF_UNAVAILABLE",
	///    "AMF_AVAILABLE"
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
	pub enum StatusChange {
		#[default]
		#[serde(rename = "AMF_UNAVAILABLE")]
		AmfUnavailable,
		#[serde(rename = "AMF_AVAILABLE")]
		AmfAvailable,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&StatusChange> for StatusChange {
		fn from(value: &StatusChange) -> Self {
			value.clone()
		}
	}

	impl ToString for StatusChange {
		fn to_string(&self) -> String {
			match *self {
				Self::AmfUnavailable => "AMF_UNAVAILABLE".to_string(),
				Self::AmfAvailable => "AMF_AVAILABLE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for StatusChange {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AMF_UNAVAILABLE" => Ok(Self::AmfUnavailable),
				"AMF_AVAILABLE" => Ok(Self::AmfAvailable),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for StatusChange {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for StatusChange {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for StatusChange {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within an AMF Status Change Subscription request and response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within an AMF Status Change Subscription request
	/// and response",
	///  "type": "object",
	///  "required": [
	///    "amfStatusUri"
	///  ],
	///  "properties": {
	///    "amfStatusUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "guamiList": {
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
	pub struct SubscriptionData {
		#[serde(rename = "amfStatusUri")]
		pub amf_status_uri: Uri,
		#[serde(rename = "guamiList", default, skip_serializing_if = "Vec::is_empty")]
		pub guami_list: Vec<crate::common::common_models::Guami>,
	}

	impl From<&SubscriptionData> for SubscriptionData {
		fn from(value: &SubscriptionData) -> Self {
			value.clone()
		}
	}

	/// SupportedCodec
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
	pub struct SupportedCodec(pub Bytes);

	impl ::std::ops::Deref for SupportedCodec {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<SupportedCodec> for Bytes {
		fn from(value: SupportedCodec) -> Self {
			value.0
		}
	}

	impl From<&SupportedCodec> for SupportedCodec {
		fn from(value: &SupportedCodec) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for SupportedCodec {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for SupportedCodec {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for SupportedCodec {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for SupportedCodec {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for SupportedCodec {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for SupportedCodec {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Supported S-NSSAIs
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Supported S-NSSAIs",
	///  "type": "object",
	///  "required": [
	///    "sNssai"
	///  ],
	///  "properties": {
	///    "restrictionInd": {
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
	pub struct SupportedSnssai {
		#[serde(rename = "restrictionInd", default)]
		pub restriction_ind: bool,
		#[serde(rename = "sNssai")]
		pub s_nssai: ExtSnssai,
	}

	impl From<&SupportedSnssai> for SupportedSnssai {
		fn from(value: &SupportedSnssai) -> Self {
			value.clone()
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

	/// TA list or TAI range list or any TA
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "TA list or TAI range list or any TA",
	///  "type": "object",
	///  "properties": {
	///    "anyTa": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "taList": {
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
	pub struct TargetArea {
		#[serde(rename = "anyTa", default)]
		pub any_ta: bool,
		#[serde(rename = "taList", default, skip_serializing_if = "Vec::is_empty")]
		pub ta_list: Vec<Tai>,
		#[serde(
			rename = "taiRangeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub tai_range_list: Vec<TaiRange>,
	}

	impl From<&TargetArea> for TargetArea {
		fn from(value: &TargetArea) -> Self {
			value.clone()
		}
	}

	/// Contains the target integrity risk
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the target integrity risk",
	///  "type": "integer",
	///  "maximum": 90.0,
	///  "minimum": 10.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TargetIntegrityRisk(pub i64);

	impl ::std::ops::Deref for TargetIntegrityRisk {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<TargetIntegrityRisk> for i64 {
		fn from(value: TargetIntegrityRisk) -> Self {
			value.0
		}
	}

	impl From<&TargetIntegrityRisk> for TargetIntegrityRisk {
		fn from(value: &TargetIntegrityRisk) -> Self {
			value.clone()
		}
	}

	impl From<i64> for TargetIntegrityRisk {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for TargetIntegrityRisk {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for TargetIntegrityRisk {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TargetIntegrityRisk {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TargetIntegrityRisk {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for TargetIntegrityRisk {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Identifies the target UE information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the target UE information.",
	///  "type": "object",
	///  "properties": {
	///    "anyUe": {
	///      "type": "boolean"
	///    },
	///    "gpsis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "intGroupIds": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "supis": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
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
	pub struct TargetUeInformation {
		#[serde(rename = "anyUe", default, skip_serializing_if = "Option::is_none")]
		pub any_ue: Option<bool>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub gpsis: Vec<Gpsi>,
		#[serde(rename = "intGroupIds", default, skip_serializing_if = "Vec::is_empty")]
		pub int_group_ids: Vec<GroupId>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		pub supis: Vec<Supi>,
	}

	impl From<&TargetUeInformation> for TargetUeInformation {
		fn from(value: &TargetUeInformation) -> Self {
			value.clone()
		}
	}

	/// Specifies causes of event reporting termination.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Specifies causes of event reporting termination.",
	///  "type": "string",
	///  "enum": [
	///    "TERMINATION_BY_UE",
	///    "TERMINATION_BY_NETWORK",
	///    "NORMAL_TERMINATION"
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
		#[serde(rename = "TERMINATION_BY_UE")]
		TerminationByUe,
		#[serde(rename = "TERMINATION_BY_NETWORK")]
		TerminationByNetwork,
		#[serde(rename = "NORMAL_TERMINATION")]
		NormalTermination,
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
				Self::TerminationByUe => "TERMINATION_BY_UE".to_string(),
				Self::TerminationByNetwork => "TERMINATION_BY_NETWORK".to_string(),
				Self::NormalTermination => "NORMAL_TERMINATION".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TerminationCause {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TERMINATION_BY_UE" => Ok(Self::TerminationByUe),
				"TERMINATION_BY_NETWORK" => Ok(Self::TerminationByNetwork),
				"NORMAL_TERMINATION" => Ok(Self::NormalTermination),
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

	/// Contains the time-to-alert
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the time-to-alert",
	///  "type": "integer",
	///  "maximum": 300.0,
	///  "minimum": 1.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TimeToAlert(pub i64);

	impl ::std::ops::Deref for TimeToAlert {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<TimeToAlert> for i64 {
		fn from(value: TimeToAlert) -> Self {
			value.0
		}
	}

	impl From<&TimeToAlert> for TimeToAlert {
		fn from(value: &TimeToAlert) -> Self {
			value.clone()
		}
	}

	impl From<i64> for TimeToAlert {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for TimeToAlert {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for TimeToAlert {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TimeToAlert {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TimeToAlert {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for TimeToAlert {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Possible values are:
	/// - MINUTE: Time unit is per minute.
	/// - HOUR: Time unit is per hour.
	/// - DAY: Time unit is per day.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- MINUTE: Time unit is per
	/// minute.\n- HOUR: Time unit is per hour.\n- DAY: Time unit is per
	/// day.\n",
	///  "type": "string",
	///  "enum": [
	///    "MINUTE",
	///    "HOUR",
	///    "DAY"
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
	pub enum TimeUnit {
		#[default]
		#[serde(rename = "MINUTE")]
		Minute,
		#[serde(rename = "HOUR")]
		Hour,
		#[serde(rename = "DAY")]
		Day,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&TimeUnit> for TimeUnit {
		fn from(value: &TimeUnit) -> Self {
			value.clone()
		}
	}

	impl ToString for TimeUnit {
		fn to_string(&self) -> String {
			match *self {
				Self::Minute => "MINUTE".to_string(),
				Self::Hour => "HOUR".to_string(),
				Self::Day => "DAY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TimeUnit {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"MINUTE" => Ok(Self::Minute),
				"HOUR" => Ok(Self::Hour),
				"DAY" => Ok(Self::Day),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for TimeUnit {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TimeUnit {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TimeUnit {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
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

	/// Top application that contributes the most to the traffic.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Top application that contributes the most to the
	/// traffic.",
	///  "type": "object",
	///  "oneOf": [
	///    {
	///      "required": [
	///        "appId"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "ipTrafficFilter"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "appId": {
	///      "$ref": "#/components/schemas/ApplicationId"
	///    },
	///    "ipTrafficFilter": {
	///      "$ref": "#/components/schemas/FlowInfo"
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
	#[serde(untagged)]
	pub enum TopApplication {
		#[default]
		Variant0 {
			#[serde(rename = "appId")]
			app_id: ApplicationId,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			ratio: Option<SamplingRatio>,
		},
		Variant1 {
			#[serde(rename = "ipTrafficFilter")]
			ip_traffic_filter: FlowInfo,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			ratio: Option<SamplingRatio>,
		},
	}

	impl From<&TopApplication> for TopApplication {
		fn from(value: &TopApplication) -> Self {
			value.clone()
		}
	}

	/// Identifies the detailed traffic characterization.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Identifies the detailed traffic characterization.",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "ulVol"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "dlVol"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "appId": {
	///      "$ref": "#/components/schemas/ApplicationId"
	///    },
	///    "dlVol": {
	///      "$ref": "#/components/schemas/Volume"
	///    },
	///    "dlVolVariance": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "fDescs": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/IpEthFlowDescription"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "snssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    },
	///    "ulVol": {
	///      "$ref": "#/components/schemas/Volume"
	///    },
	///    "ulVolVariance": {
	///      "$ref": "#/components/schemas/Float"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum TrafficCharacterization {
		#[default]
		Variant0 {
			#[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
			app_id: Option<ApplicationId>,
			#[serde(
				rename = "dlVolVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			dl_vol_variance: Option<Float>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(rename = "fDescs", default, skip_serializing_if = "Vec::is_empty")]
			f_descs: Vec<IpEthFlowDescription>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
			#[serde(rename = "ulVol")]
			ul_vol: Volume,
			#[serde(
				rename = "ulVolVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ul_vol_variance: Option<Float>,
		},
		Variant1 {
			#[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
			app_id: Option<ApplicationId>,
			#[serde(rename = "dlVol")]
			dl_vol: Volume,
			#[serde(
				rename = "dlVolVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			dl_vol_variance: Option<Float>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			dnn: Option<Dnn>,
			#[serde(rename = "fDescs", default, skip_serializing_if = "Vec::is_empty")]
			f_descs: Vec<IpEthFlowDescription>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			snssai: Option<Snssai>,
			#[serde(
				rename = "ulVolVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ul_vol_variance: Option<Float>,
		},
	}

	impl From<&TrafficCharacterization> for TrafficCharacterization {
		fn from(value: &TrafficCharacterization) -> Self {
			value.clone()
		}
	}

	/// Represents the Traffic Descriptor
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the Traffic Descriptor",
	///  "type": "object",
	///  "properties": {
	///    "dddTrafficDescriptorList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/DddTrafficDescriptor"
	///      },
	///      "minItems": 1
	///    },
	///    "dnn": {
	///      "$ref": "#/components/schemas/Dnn"
	///    },
	///    "sNssai": {
	///      "$ref": "#/components/schemas/Snssai"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct TrafficDescriptor {
		#[serde(
			rename = "dddTrafficDescriptorList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ddd_traffic_descriptor_list: Vec<DddTrafficDescriptor>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub dnn: Option<Dnn>,
		#[serde(rename = "sNssai", default, skip_serializing_if = "Option::is_none")]
		pub s_nssai: Option<Snssai>,
	}

	impl From<&TrafficDescriptor> for TrafficDescriptor {
		fn from(value: &TrafficDescriptor) -> Self {
			value.clone()
		}
	}

	/// Traffic information including UL/DL data rate and/or Traffic volume.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Traffic information including UL/DL data rate and/or
	/// Traffic volume.",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "uplinkRate"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "downlinkRate"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "uplinkVolume"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "downlinkVolume"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "totalVolume"
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "downlinkRate": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "downlinkVolume": {
	///      "$ref": "#/components/schemas/Volume"
	///    },
	///    "totalVolume": {
	///      "$ref": "#/components/schemas/Volume"
	///    },
	///    "uplinkRate": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "uplinkVolume": {
	///      "$ref": "#/components/schemas/Volume"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum TrafficInformation {
		#[default]
		Variant0 {
			#[serde(rename = "uplinkRate")]
			uplink_rate: BitRate,
		},
		Variant1 {
			#[serde(rename = "downlinkRate")]
			downlink_rate: BitRate,
		},
		Variant2 {
			#[serde(rename = "uplinkVolume")]
			uplink_volume: Volume,
		},
		Variant3 {
			#[serde(rename = "downlinkVolume")]
			downlink_volume: Volume,
		},
		Variant4 {
			#[serde(rename = "totalVolume")]
			total_volume: Volume,
		},
	}

	impl From<&TrafficInformation> for TrafficInformation {
		fn from(value: &TrafficInformation) -> Self {
			value.clone()
		}
	}

	/// Indicates UE Context Transfer Reason
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates UE Context Transfer Reason",
	///  "type": "string",
	///  "enum": [
	///    "INIT_REG",
	///    "MOBI_REG",
	///    "MOBI_REG_UE_VALIDATED"
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
	pub enum TransferReason {
		#[default]
		#[serde(rename = "INIT_REG")]
		InitReg,
		#[serde(rename = "MOBI_REG")]
		MobiReg,
		#[serde(rename = "MOBI_REG_UE_VALIDATED")]
		MobiRegUeValidated,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&TransferReason> for TransferReason {
		fn from(value: &TransferReason) -> Self {
			value.clone()
		}
	}

	impl ToString for TransferReason {
		fn to_string(&self) -> String {
			match *self {
				Self::InitReg => "INIT_REG".to_string(),
				Self::MobiReg => "MOBI_REG".to_string(),
				Self::MobiRegUeValidated => "MOBI_REG_UE_VALIDATED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for TransferReason {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"INIT_REG" => Ok(Self::InitReg),
				"MOBI_REG" => Ok(Self::MobiReg),
				"MOBI_REG_UE_VALIDATED" => Ok(Self::MobiRegUeValidated),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for TransferReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for TransferReason {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for TransferReason {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Report Item for UE Access Behavior Trends event.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Report Item for UE Access Behavior Trends event.",
	///  "type": "object",
	///  "required": [
	///    "duration",
	///    "spacing",
	///    "stateTransitionType"
	///  ],
	///  "properties": {
	///    "duration": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "spacing": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "stateTransitionType": {
	///      "$ref": "#/components/schemas/AccessStateTransitionType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeAccessBehaviorReportItem {
		pub duration: DurationSec,
		pub spacing: DurationSec,
		#[serde(rename = "stateTransitionType")]
		pub state_transition_type: AccessStateTransitionType,
	}

	impl From<&UeAccessBehaviorReportItem> for UeAccessBehaviorReportItem {
		fn from(value: &UeAccessBehaviorReportItem) -> Self {
			value.clone()
		}
	}

	/// Contains information about available UE related analytics contexts.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains information about available UE related
	/// analytics contexts.",
	///  "type": "object",
	///  "required": [
	///    "anaTypes",
	///    "supi"
	///  ],
	///  "properties": {
	///    "anaTypes": {
	///      "description": "List of analytics types for which UE related
	/// analytics contexts can be retrieved.\n",
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/NwdafEvent"
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
	pub struct UeAnalyticsContextDescriptor {
		/// List of analytics types for which UE related analytics contexts can
		/// be retrieved.
		#[serde(rename = "anaTypes")]
		pub ana_types: Vec<NwdafEvent>,
		pub supi: Supi,
	}

	impl From<&UeAnalyticsContextDescriptor> for UeAnalyticsContextDescriptor {
		fn from(value: &UeAnalyticsContextDescriptor) -> Self {
			value.clone()
		}
	}

	/// Represents UE communication information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents UE communication information.",
	///  "type": "object",
	///  "allOf": [
	///    {
	///      "required": [
	///        "commDur"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "trafChar"
	///      ]
	///    },
	///    {
	///      "oneOf": [
	///        {
	///          "required": [
	///            "ts"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "recurringTime"
	///          ]
	///        }
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "anaOfAppList": {
	///      "$ref": "#/components/schemas/AppListForUeComm"
	///    },
	///    "commDur": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "commDurVariance": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "perioCommInd": {
	///      "type": "boolean"
	///    },
	///    "perioTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "perioTimeVariance": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "ratio": {
	///      "$ref": "#/components/schemas/SamplingRatio"
	///    },
	///    "recurringTime": {
	///      "$ref": "#/components/schemas/schemas-ScheduledCommunicationTime"
	///    },
	///    "sessInactTimer": {
	///      "$ref": "#/components/schemas/SessInactTimerForUeComm"
	///    },
	///    "trafChar": {
	///      "$ref": "#/components/schemas/TrafficCharacterization"
	///    },
	///    "ts": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "tsVariance": {
	///      "$ref": "#/components/schemas/Float"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum UeCommunication {
		#[default]
		Variant0 {
			#[serde(
				rename = "anaOfAppList",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ana_of_app_list: Option<AppListForUeComm>,
			#[serde(rename = "commDur")]
			comm_dur: DurationSec,
			#[serde(
				rename = "commDurVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			comm_dur_variance: Option<Float>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(
				rename = "perioCommInd",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			perio_comm_ind: Option<bool>,
			#[serde(rename = "perioTime", default, skip_serializing_if = "Option::is_none")]
			perio_time: Option<DurationSec>,
			#[serde(
				rename = "perioTimeVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			perio_time_variance: Option<Float>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			ratio: Option<SamplingRatio>,
			#[serde(
				rename = "sessInactTimer",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			sess_inact_timer: Option<SessInactTimerForUeComm>,
			#[serde(rename = "trafChar")]
			traf_char: TrafficCharacterization,
			ts: DateTime,
			#[serde(
				rename = "tsVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ts_variance: Option<Float>,
		},
		Variant1 {
			#[serde(
				rename = "anaOfAppList",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ana_of_app_list: Option<AppListForUeComm>,
			#[serde(rename = "commDur")]
			comm_dur: DurationSec,
			#[serde(
				rename = "commDurVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			comm_dur_variance: Option<Float>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(
				rename = "perioCommInd",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			perio_comm_ind: Option<bool>,
			#[serde(rename = "perioTime", default, skip_serializing_if = "Option::is_none")]
			perio_time: Option<DurationSec>,
			#[serde(
				rename = "perioTimeVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			perio_time_variance: Option<Float>,
			#[serde(default, skip_serializing_if = "Option::is_none")]
			ratio: Option<SamplingRatio>,
			#[serde(rename = "recurringTime")]
			recurring_time: SchemasScheduledCommunicationTime,
			#[serde(
				rename = "sessInactTimer",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			sess_inact_timer: Option<SessInactTimerForUeComm>,
			#[serde(rename = "trafChar")]
			traf_char: TrafficCharacterization,
			#[serde(
				rename = "tsVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			ts_variance: Option<Float>,
		},
	}

	impl From<&UeCommunication> for UeCommunication {
		fn from(value: &UeCommunication) -> Self {
			value.clone()
		}
	}

	/// Represents an individual ueContext resource
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents an individual ueContext resource",
	///  "type": "object",
	///  "properties": {
	///    "5gMmCapability": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "amPolicyReqTriggerList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PolicyReqTrigger"
	///      },
	///      "minItems": 1
	///    },
	///    "analyticsSubscriptionList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/AnalyticsSubscription"
	///      },
	///      "minItems": 1
	///    },
	///    "asTimeDisParam": {
	///      "$ref": "#/components/schemas/AsTimeDistributionParam"
	///    },
	///    "astiDistributionIndication": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ausfGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "cMsisdn": {
	///      "$ref": "#/components/schemas/CMsisdn"
	///    },
	///    "cagData": {
	///      "$ref": "#/components/schemas/CagData"
	///    },
	///    "disasterPlmn": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "disasterRoamingInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "drxParameter": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "ecRestrictionDataNb": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ecRestrictionDataWb": {
	///      "$ref": "#/components/schemas/EcRestrictionDataWb"
	///    },
	///    "epsInterworkingInfo": {
	///      "$ref": "#/components/schemas/EpsInterworkingInfo"
	///    },
	///    "eventSubscriptionList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ExtAmfEventSubscription"
	///      },
	///      "minItems": 1
	///    },
	///    "forbiddenAreaList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Area"
	///      },
	///      "minItems": 1
	///    },
	///    "gpsiList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Gpsi"
	///      },
	///      "minItems": 1
	///    },
	///    "groupList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/GroupId"
	///      },
	///      "minItems": 1
	///    },
	///    "hNwPubKeyId": {
	///      "type": "integer"
	///    },
	///    "hpcfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "hpcfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "iabOperationAllowed": {
	///      "type": "boolean"
	///    },
	///    "immediateMdtConf": {
	///      "$ref": "#/components/schemas/ImmediateMdtConf"
	///    },
	///    "lteCatMInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "managementMdtInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "mmContextList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/MmContext"
	///      },
	///      "maxItems": 2,
	///      "minItems": 1
	///    },
	///    "moExpDataCounter": {
	///      "$ref": "#/components/schemas/MoExpDataCounter"
	///    },
	///    "msClassmark2": {
	///      "$ref": "#/components/schemas/Bytes"
	///    },
	///    "pcfAmPolicyUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "pcfAmpBindingInfo": {
	///      "type": "string"
	///    },
	///    "pcfAmpServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "pcfBinding": {
	///      "$ref": "#/components/schemas/SbiBindingLevel"
	///    },
	///    "pcfGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "pcfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "pcfRfsp": {
	///      "$ref": "#/components/schemas/RfspIndex"
	///    },
	///    "pcfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "pcfUeAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "pcfUeCallbackInfo": {
	///      "$ref": "#/components/schemas/PcfUeCallbackInfo"
	///    },
	///    "pcfUePolicyUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "pcfUeSliceMbrList": {
	///      "description": "A map(list of key-value pairs) where Snssai serves
	/// as key.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SliceMbr"
	///      }
	///    },
	///    "pcfUepBindingInfo": {
	///      "type": "string"
	///    },
	///    "pcfUepServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "pei": {
	///      "$ref": "#/components/schemas/Pei"
	///    },
	///    "praInAmPolicy": {
	///      "description": "A map(list of key-value pairs) where praId serves
	/// as key.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "praInUePolicy": {
	///      "description": "A map(list of key-value pairs) where praId serves
	/// as key.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/PresenceInfo"
	///      }
	///    },
	///    "proseContext": {
	///      "$ref": "#/components/schemas/ProseContext"
	///    },
	///    "redCapInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "restrictedCoreNwTypeList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/CoreNetworkType"
	///      },
	///      "minItems": 1
	///    },
	///    "restrictedPrimaryRatList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "minItems": 1
	///    },
	///    "restrictedRatList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "minItems": 1
	///    },
	///    "restrictedSecondaryRatList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/RatType"
	///      },
	///      "minItems": 1
	///    },
	///    "routingIndicator": {
	///      "type": "string"
	///    },
	///    "seafData": {
	///      "$ref": "#/components/schemas/SeafData"
	///    },
	///    "serviceAreaRestriction": {
	///      "$ref": "#/components/schemas/ServiceAreaRestriction"
	///    },
	///    "serviceGapExpiryTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "sessionContextList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionContext"
	///      },
	///      "minItems": 1
	///    },
	///    "smPolicyNotifyPduList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "smallDataRateStatusInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SmallDataRateStatusInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "smfSelInfo": {
	///      "$ref": "#/components/schemas/SmfSelectionData"
	///    },
	///    "smsfBindingInfo": {
	///      "type": "string"
	///    },
	///    "smsfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "smsfServiceSetId": {
	///      "$ref": "#/components/schemas/NfServiceSetId"
	///    },
	///    "smsfSetId": {
	///      "$ref": "#/components/schemas/NfSetId"
	///    },
	///    "snpnOnboardInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "stnSr": {
	///      "$ref": "#/components/schemas/StnSr"
	///    },
	///    "subRfsp": {
	///      "$ref": "#/components/schemas/RfspIndex"
	///    },
	///    "subUeAmbr": {
	///      "$ref": "#/components/schemas/Ambr"
	///    },
	///    "subUeSliceMbrList": {
	///      "description": "A map(list of key-value pairs) where Snssai serves
	/// as key.",
	///      "type": "object",
	///      "minProperties": 1,
	///      "additionalProperties": {
	///        "$ref": "#/components/schemas/SliceMbr"
	///      }
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "supiUnauthInd": {
	///      "type": "boolean"
	///    },
	///    "supportedCodecList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Bytes"
	///      },
	///      "minItems": 1
	///    },
	///    "traceData": {
	///      "$ref": "#/components/schemas/TraceData"
	///    },
	///    "tsErrorBudget": {
	///      "type": "integer"
	///    },
	///    "udmGroupId": {
	///      "$ref": "#/components/schemas/NfGroupId"
	///    },
	///    "uePolicyReqTriggerList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PolicyReqTrigger"
	///      },
	///      "minItems": 1
	///    },
	///    "uePositioningCap": {
	///      "$ref": "#/components/schemas/UePositioningCapabilities"
	///    },
	///    "updpSubscriptionData": {
	///      "$ref": "#/components/schemas/UpdpSubscriptionData"
	///    },
	///    "usedRfsp": {
	///      "$ref": "#/components/schemas/RfspIndex"
	///    },
	///    "usedServiceAreaRestriction": {
	///      "$ref": "#/components/schemas/ServiceAreaRestriction"
	///    },
	///    "v2xContext": {
	///      "$ref": "#/components/schemas/V2xContext"
	///    },
	///    "wlServAreaRes": {
	///      "$ref": "#/components/schemas/WirelineServiceAreaRestriction"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeContext {
		#[serde(
			rename = "amPolicyReqTriggerList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub am_policy_req_trigger_list: Vec<PolicyReqTrigger>,
		#[serde(
			rename = "analyticsSubscriptionList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub analytics_subscription_list: Vec<AnalyticsSubscription>,
		#[serde(
			rename = "asTimeDisParam",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub as_time_dis_param: Option<AsTimeDistributionParam>,
		#[serde(rename = "astiDistributionIndication", default)]
		pub asti_distribution_indication: bool,
		#[serde(
			rename = "ausfGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ausf_group_id: Option<NfGroupId>,
		#[serde(rename = "cMsisdn", default, skip_serializing_if = "Option::is_none")]
		pub c_msisdn: Option<CMsisdn>,
		#[serde(rename = "cagData", default, skip_serializing_if = "Option::is_none")]
		pub cag_data: Option<CagData>,
		#[serde(
			rename = "disasterPlmn",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub disaster_plmn: Option<PlmnId>,
		#[serde(rename = "disasterRoamingInd", default)]
		pub disaster_roaming_ind: bool,
		#[serde(
			rename = "drxParameter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub drx_parameter: Option<Bytes>,
		#[serde(rename = "ecRestrictionDataNb", default)]
		pub ec_restriction_data_nb: bool,
		#[serde(
			rename = "ecRestrictionDataWb",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ec_restriction_data_wb: Option<EcRestrictionDataWb>,
		#[serde(
			rename = "epsInterworkingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub eps_interworking_info: Option<EpsInterworkingInfo>,
		#[serde(
			rename = "eventSubscriptionList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub event_subscription_list: Vec<ExtAmfEventSubscription>,
		#[serde(
			rename = "5gMmCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub five_g_mm_capability: Option<Bytes>,
		#[serde(
			rename = "forbiddenAreaList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub forbidden_area_list: Vec<Area>,
		#[serde(rename = "gpsiList", default, skip_serializing_if = "Vec::is_empty")]
		pub gpsi_list: Vec<Gpsi>,
		#[serde(rename = "groupList", default, skip_serializing_if = "Vec::is_empty")]
		pub group_list: Vec<GroupId>,
		#[serde(
			rename = "hNwPubKeyId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub h_nw_pub_key_id: Option<i64>,
		#[serde(rename = "hpcfId", default, skip_serializing_if = "Option::is_none")]
		pub hpcf_id: Option<NfInstanceId>,
		#[serde(rename = "hpcfSetId", default, skip_serializing_if = "Option::is_none")]
		pub hpcf_set_id: Option<NfSetId>,
		#[serde(
			rename = "iabOperationAllowed",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub iab_operation_allowed: Option<bool>,
		#[serde(
			rename = "immediateMdtConf",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub immediate_mdt_conf: Option<ImmediateMdtConf>,
		#[serde(rename = "lteCatMInd", default)]
		pub lte_cat_m_ind: bool,
		#[serde(rename = "managementMdtInd", default)]
		pub management_mdt_ind: bool,
		#[serde(
			rename = "mmContextList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub mm_context_list: Vec<MmContext>,
		#[serde(
			rename = "moExpDataCounter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub mo_exp_data_counter: Option<MoExpDataCounter>,
		#[serde(
			rename = "msClassmark2",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ms_classmark2: Option<Bytes>,
		#[serde(
			rename = "pcfAmPolicyUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_am_policy_uri: Option<Uri>,
		#[serde(
			rename = "pcfAmpBindingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_amp_binding_info: Option<String>,
		#[serde(
			rename = "pcfAmpServiceSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_amp_service_set_id: Option<NfServiceSetId>,
		#[serde(
			rename = "pcfBinding",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_binding: Option<SbiBindingLevel>,
		#[serde(
			rename = "pcfGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_group_id: Option<NfGroupId>,
		#[serde(rename = "pcfId", default, skip_serializing_if = "Option::is_none")]
		pub pcf_id: Option<NfInstanceId>,
		#[serde(rename = "pcfRfsp", default, skip_serializing_if = "Option::is_none")]
		pub pcf_rfsp: Option<RfspIndex>,
		#[serde(rename = "pcfSetId", default, skip_serializing_if = "Option::is_none")]
		pub pcf_set_id: Option<NfSetId>,
		#[serde(rename = "pcfUeAmbr", default, skip_serializing_if = "Option::is_none")]
		pub pcf_ue_ambr: Option<Ambr>,
		#[serde(
			rename = "pcfUeCallbackInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_ue_callback_info: Option<PcfUeCallbackInfo>,
		#[serde(
			rename = "pcfUePolicyUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_ue_policy_uri: Option<Uri>,
		/// A map(list of key-value pairs) where Snssai serves as key.
		#[serde(
			rename = "pcfUeSliceMbrList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub pcf_ue_slice_mbr_list: ::std::collections::HashMap<String, SliceMbr>,
		#[serde(
			rename = "pcfUepBindingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_uep_binding_info: Option<String>,
		#[serde(
			rename = "pcfUepServiceSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_uep_service_set_id: Option<NfServiceSetId>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub pei: Option<Pei>,
		/// A map(list of key-value pairs) where praId serves as key.
		#[serde(
			rename = "praInAmPolicy",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub pra_in_am_policy: ::std::collections::HashMap<String, PresenceInfo>,
		/// A map(list of key-value pairs) where praId serves as key.
		#[serde(
			rename = "praInUePolicy",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub pra_in_ue_policy: ::std::collections::HashMap<String, PresenceInfo>,
		#[serde(
			rename = "proseContext",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub prose_context: Option<ProseContext>,
		#[serde(rename = "redCapInd", default)]
		pub red_cap_ind: bool,
		#[serde(
			rename = "restrictedCoreNwTypeList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub restricted_core_nw_type_list: Vec<CoreNetworkType>,
		#[serde(
			rename = "restrictedPrimaryRatList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub restricted_primary_rat_list: Vec<RatType>,
		#[serde(
			rename = "restrictedRatList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub restricted_rat_list: Vec<RatType>,
		#[serde(
			rename = "restrictedSecondaryRatList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub restricted_secondary_rat_list: Vec<RatType>,
		#[serde(
			rename = "routingIndicator",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub routing_indicator: Option<String>,
		#[serde(rename = "seafData", default, skip_serializing_if = "Option::is_none")]
		pub seaf_data: Option<SeafData>,
		#[serde(
			rename = "serviceAreaRestriction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_area_restriction: Option<ServiceAreaRestriction>,
		#[serde(
			rename = "serviceGapExpiryTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub service_gap_expiry_time: Option<DateTime>,
		#[serde(
			rename = "sessionContextList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub session_context_list: Vec<PduSessionContext>,
		#[serde(
			rename = "smPolicyNotifyPduList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub sm_policy_notify_pdu_list: Vec<PduSessionInfo>,
		#[serde(
			rename = "smallDataRateStatusInfos",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub small_data_rate_status_infos: Vec<SmallDataRateStatusInfo>,
		#[serde(
			rename = "smfSelInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smf_sel_info: Option<SmfSelectionData>,
		#[serde(
			rename = "smsfBindingInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smsf_binding_info: Option<String>,
		#[serde(rename = "smsfId", default, skip_serializing_if = "Option::is_none")]
		pub smsf_id: Option<NfInstanceId>,
		#[serde(
			rename = "smsfServiceSetId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub smsf_service_set_id: Option<NfServiceSetId>,
		#[serde(rename = "smsfSetId", default, skip_serializing_if = "Option::is_none")]
		pub smsf_set_id: Option<NfSetId>,
		#[serde(rename = "snpnOnboardInd", default)]
		pub snpn_onboard_ind: bool,
		#[serde(rename = "stnSr", default, skip_serializing_if = "Option::is_none")]
		pub stn_sr: Option<StnSr>,
		#[serde(rename = "subRfsp", default, skip_serializing_if = "Option::is_none")]
		pub sub_rfsp: Option<RfspIndex>,
		#[serde(rename = "subUeAmbr", default, skip_serializing_if = "Option::is_none")]
		pub sub_ue_ambr: Option<Ambr>,
		/// A map(list of key-value pairs) where Snssai serves as key.
		#[serde(
			rename = "subUeSliceMbrList",
			default,
			skip_serializing_if = "::std::collections::HashMap::is_empty"
		)]
		pub sub_ue_slice_mbr_list: ::std::collections::HashMap<String, SliceMbr>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(
			rename = "supiUnauthInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supi_unauth_ind: Option<bool>,
		#[serde(
			rename = "supportedCodecList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub supported_codec_list: Vec<Bytes>,
		#[serde(rename = "traceData", default, skip_serializing_if = "Option::is_none")]
		pub trace_data: Option<TraceData>,
		#[serde(
			rename = "tsErrorBudget",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ts_error_budget: Option<i64>,
		#[serde(
			rename = "udmGroupId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub udm_group_id: Option<NfGroupId>,
		#[serde(
			rename = "uePolicyReqTriggerList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub ue_policy_req_trigger_list: Vec<PolicyReqTrigger>,
		#[serde(
			rename = "uePositioningCap",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_positioning_cap: Option<UePositioningCapabilities>,
		#[serde(
			rename = "updpSubscriptionData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub updp_subscription_data: Option<UpdpSubscriptionData>,
		#[serde(rename = "usedRfsp", default, skip_serializing_if = "Option::is_none")]
		pub used_rfsp: Option<RfspIndex>,
		#[serde(
			rename = "usedServiceAreaRestriction",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub used_service_area_restriction: Option<ServiceAreaRestriction>,
		#[serde(
			rename = "v2xContext",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub v2x_context: Option<V2xContext>,
		#[serde(
			rename = "wlServAreaRes",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub wl_serv_area_res: Option<WirelineServiceAreaRestriction>,
	}

	impl From<&UeContext> for UeContext {
		fn from(value: &UeContext) -> Self {
			value.clone()
		}
	}

	/// Data structure used for cancellation of UE Context Relocation
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data structure used for cancellation of UE Context
	/// Relocation",
	///  "type": "object",
	///  "required": [
	///    "relocationCancelRequest"
	///  ],
	///  "properties": {
	///    "relocationCancelRequest": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
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
	pub struct UeContextCancelRelocateData {
		#[serde(rename = "relocationCancelRequest")]
		pub relocation_cancel_request: RefToBinaryData,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
	}

	impl From<&UeContextCancelRelocateData> for UeContextCancelRelocateData {
		fn from(value: &UeContextCancelRelocateData) -> Self {
			value.clone()
		}
	}

	/// Data within a request to create an individual ueContext resource
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a request to create an individual ueContext
	/// resource",
	///  "type": "object",
	///  "required": [
	///    "pduSessionList",
	///    "sourceToTargetData",
	///    "targetId",
	///    "ueContext"
	///  ],
	///  "properties": {
	///    "n2NotifyUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "ngapCause": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    },
	///    "pduSessionList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/N2SmInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "servingNetwork": {
	///      "$ref": "#/components/schemas/PlmnIdNid"
	///    },
	///    "sourceToTargetData": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "targetId": {
	///      "$ref": "#/components/schemas/NgRanTargetId"
	///    },
	///    "ueContext": {
	///      "$ref": "#/components/schemas/UeContext"
	///    },
	///    "ueRadioCapability": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "ueRadioCapabilityForPaging": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeContextCreateData {
		#[serde(
			rename = "n2NotifyUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_notify_uri: Option<Uri>,
		#[serde(rename = "ngapCause", default, skip_serializing_if = "Option::is_none")]
		pub ngap_cause: Option<NgApCause>,
		#[serde(rename = "pduSessionList")]
		pub pdu_session_list: Vec<N2SmInformation>,
		#[serde(
			rename = "servingNetwork",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub serving_network: Option<PlmnIdNid>,
		#[serde(rename = "sourceToTargetData")]
		pub source_to_target_data: N2InfoContent,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "targetId")]
		pub target_id: NgRanTargetId,
		#[serde(rename = "ueContext")]
		pub ue_context: UeContext,
		#[serde(
			rename = "ueRadioCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_radio_capability: Option<N2InfoContent>,
		#[serde(
			rename = "ueRadioCapabilityForPaging",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_radio_capability_for_paging: Option<N2InfoContent>,
	}

	impl From<&UeContextCreateData> for UeContextCreateData {
		fn from(value: &UeContextCreateData) -> Self {
			value.clone()
		}
	}

	/// Data within a failure response for creating a UE context
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a failure response for creating a UE
	/// context",
	///  "type": "object",
	///  "required": [
	///    "error"
	///  ],
	///  "properties": {
	///    "error": {
	///      "$ref": "#/components/schemas/ProblemDetails"
	///    },
	///    "ngapCause": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    },
	///    "targetToSourceFailureData": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeContextCreateError {
		pub error: ProblemDetails,
		#[serde(rename = "ngapCause", default, skip_serializing_if = "Option::is_none")]
		pub ngap_cause: Option<NgApCause>,
		#[serde(
			rename = "targetToSourceFailureData",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub target_to_source_failure_data: Option<N2InfoContent>,
	}

	impl From<&UeContextCreateError> for UeContextCreateError {
		fn from(value: &UeContextCreateError) -> Self {
			value.clone()
		}
	}

	/// Data within a successful response for creating an individual ueContext
	/// resource
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a successful response for creating an
	/// individual ueContext resource",
	///  "type": "object",
	///  "required": [
	///    "pduSessionList",
	///    "targetToSourceData",
	///    "ueContext"
	///  ],
	///  "properties": {
	///    "analyticsNotUsedList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uri"
	///      },
	///      "minItems": 1
	///    },
	///    "failedSessionList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/N2SmInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "pcfReselectedInd": {
	///      "type": "boolean"
	///    },
	///    "pduSessionList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/N2SmInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "targetToSourceData": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "ueContext": {
	///      "$ref": "#/components/schemas/UeContext"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeContextCreatedData {
		#[serde(
			rename = "analyticsNotUsedList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub analytics_not_used_list: Vec<Uri>,
		#[serde(
			rename = "failedSessionList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub failed_session_list: Vec<N2SmInformation>,
		#[serde(
			rename = "pcfReselectedInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_reselected_ind: Option<bool>,
		#[serde(rename = "pduSessionList")]
		pub pdu_session_list: Vec<N2SmInformation>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "targetToSourceData")]
		pub target_to_source_data: N2InfoContent,
		#[serde(rename = "ueContext")]
		pub ue_context: UeContext,
	}

	impl From<&UeContextCreatedData> for UeContextCreatedData {
		fn from(value: &UeContextCreatedData) -> Self {
			value.clone()
		}
	}

	/// UE Context Information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "UE Context Information",
	///  "type": "object",
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "lastActTime": {
	///      "$ref": "#/components/schemas/DateTime"
	///    },
	///    "ratType": {
	///      "$ref": "#/components/schemas/RatType"
	///    },
	///    "supportVoPS": {
	///      "type": "boolean"
	///    },
	///    "supportVoPSn3gpp": {
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
	pub struct UeContextInfo {
		#[serde(
			rename = "accessType",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub access_type: Option<AccessType>,
		#[serde(
			rename = "lastActTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub last_act_time: Option<DateTime>,
		#[serde(rename = "ratType", default, skip_serializing_if = "Option::is_none")]
		pub rat_type: Option<RatType>,
		#[serde(
			rename = "supportVoPSn3gpp",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub support_vo_p_sn3gpp: Option<bool>,
		#[serde(
			rename = "supportVoPS",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub support_vo_ps: Option<bool>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&UeContextInfo> for UeContextInfo {
		fn from(value: &UeContextInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates the UE Context information class
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the UE Context information class",
	///  "type": "string",
	///  "enum": [
	///    "TADS"
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
	pub enum UeContextInfoClass {
		#[default]
		#[serde(rename = "TADS")]
		Tads,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UeContextInfoClass> for UeContextInfoClass {
		fn from(value: &UeContextInfoClass) -> Self {
			value.clone()
		}
	}

	impl ToString for UeContextInfoClass {
		fn to_string(&self) -> String {
			match *self {
				Self::Tads => "TADS".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UeContextInfoClass {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TADS" => Ok(Self::Tads),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UeContextInfoClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UeContextInfoClass {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UeContextInfoClass {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within a Release UE Context request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a Release UE Context request",
	///  "type": "object",
	///  "required": [
	///    "ngapCause"
	///  ],
	///  "properties": {
	///    "ngapCause": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    },
	///    "supi": {
	///      "$ref": "#/components/schemas/Supi"
	///    },
	///    "unauthenticatedSupi": {
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
	pub struct UeContextRelease {
		#[serde(rename = "ngapCause")]
		pub ngap_cause: NgApCause,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
		#[serde(rename = "unauthenticatedSupi", default)]
		pub unauthenticated_supi: bool,
	}

	impl From<&UeContextRelease> for UeContextRelease {
		fn from(value: &UeContextRelease) -> Self {
			value.clone()
		}
	}

	/// Data within a Relocate UE Context request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a Relocate UE Context request",
	///  "type": "object",
	///  "required": [
	///    "forwardRelocationRequest",
	///    "sourceToTargetData",
	///    "targetId",
	///    "ueContext"
	///  ],
	///  "properties": {
	///    "forwardRelocationRequest": {
	///      "$ref": "#/components/schemas/RefToBinaryData"
	///    },
	///    "ngapCause": {
	///      "$ref": "#/components/schemas/NgApCause"
	///    },
	///    "pduSessionList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/N2SmInformation"
	///      },
	///      "minItems": 1
	///    },
	///    "sourceToTargetData": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "targetId": {
	///      "$ref": "#/components/schemas/NgRanTargetId"
	///    },
	///    "ueContext": {
	///      "$ref": "#/components/schemas/UeContext"
	///    },
	///    "ueRadioCapability": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeContextRelocateData {
		#[serde(rename = "forwardRelocationRequest")]
		pub forward_relocation_request: RefToBinaryData,
		#[serde(rename = "ngapCause", default, skip_serializing_if = "Option::is_none")]
		pub ngap_cause: Option<NgApCause>,
		#[serde(
			rename = "pduSessionList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub pdu_session_list: Vec<N2SmInformation>,
		#[serde(rename = "sourceToTargetData")]
		pub source_to_target_data: N2InfoContent,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "targetId")]
		pub target_id: NgRanTargetId,
		#[serde(rename = "ueContext")]
		pub ue_context: UeContext,
		#[serde(
			rename = "ueRadioCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_radio_capability: Option<N2InfoContent>,
	}

	impl From<&UeContextRelocateData> for UeContextRelocateData {
		fn from(value: &UeContextRelocateData) -> Self {
			value.clone()
		}
	}

	/// Data within a Relocate UE Context response
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a Relocate UE Context response",
	///  "type": "object",
	///  "required": [
	///    "ueContext"
	///  ],
	///  "properties": {
	///    "ueContext": {
	///      "$ref": "#/components/schemas/UeContext"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeContextRelocatedData {
		#[serde(rename = "ueContext")]
		pub ue_context: UeContext,
	}

	impl From<&UeContextRelocatedData> for UeContextRelocatedData {
		fn from(value: &UeContextRelocatedData) -> Self {
			value.clone()
		}
	}

	/// Data within a UE Context Transfer Request to start transferring of an
	/// individual ueContext resource from old AMF to new AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a UE Context Transfer Request to start
	/// transferring of an individual ueContext resource from old AMF to new
	/// AMF",
	///  "type": "object",
	///  "required": [
	///    "accessType",
	///    "reason"
	///  ],
	///  "properties": {
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    },
	///    "plmnId": {
	///      "$ref": "#/components/schemas/PlmnId"
	///    },
	///    "reason": {
	///      "$ref": "#/components/schemas/TransferReason"
	///    },
	///    "regRequest": {
	///      "$ref": "#/components/schemas/N1MessageContainer"
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
	pub struct UeContextTransferReqData {
		#[serde(rename = "accessType")]
		pub access_type: AccessType,
		#[serde(rename = "plmnId", default, skip_serializing_if = "Option::is_none")]
		pub plmn_id: Option<PlmnId>,
		pub reason: TransferReason,
		#[serde(
			rename = "regRequest",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub reg_request: Option<N1MessageContainer>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&UeContextTransferReqData> for UeContextTransferReqData {
		fn from(value: &UeContextTransferReqData) -> Self {
			value.clone()
		}
	}

	/// Data within a successful response to the UE Context Transfer request
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a successful response to the UE Context
	/// Transfer request",
	///  "type": "object",
	///  "required": [
	///    "ueContext"
	///  ],
	///  "properties": {
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "ueContext": {
	///      "$ref": "#/components/schemas/UeContext"
	///    },
	///    "ueNbiotRadioCapability": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "ueRadioCapability": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    },
	///    "ueRadioCapabilityForPaging": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeContextTransferRspData {
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(rename = "ueContext")]
		pub ue_context: UeContext,
		#[serde(
			rename = "ueNbiotRadioCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_nbiot_radio_capability: Option<N2InfoContent>,
		#[serde(
			rename = "ueRadioCapability",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_radio_capability: Option<N2InfoContent>,
		#[serde(
			rename = "ueRadioCapabilityForPaging",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub ue_radio_capability_for_paging: Option<N2InfoContent>,
	}

	impl From<&UeContextTransferRspData> for UeContextTransferRspData {
		fn from(value: &UeContextTransferRspData) -> Self {
			value.clone()
		}
	}

	/// Describes the status of an individual ueContext resource in UE Context
	/// Transfer procedures
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the status of an individual ueContext
	/// resource in UE Context Transfer procedures",
	///  "type": "string",
	///  "enum": [
	///    "TRANSFERRED",
	///    "NOT_TRANSFERRED"
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
	pub enum UeContextTransferStatus {
		#[default]
		#[serde(rename = "TRANSFERRED")]
		Transferred,
		#[serde(rename = "NOT_TRANSFERRED")]
		NotTransferred,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UeContextTransferStatus> for UeContextTransferStatus {
		fn from(value: &UeContextTransferStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for UeContextTransferStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Transferred => "TRANSFERRED".to_string(),
				Self::NotTransferred => "NOT_TRANSFERRED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UeContextTransferStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TRANSFERRED" => Ok(Self::Transferred),
				"NOT_TRANSFERRED" => Ok(Self::NotTransferred),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UeContextTransferStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UeContextTransferStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UeContextTransferStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the UE Differentiation Information and its validity time
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the UE Differentiation Information and its
	/// validity time",
	///  "type": "object",
	///  "properties": {
	///    "batteryInd": {
	///      "$ref": "#/components/schemas/BatteryIndication"
	///    },
	///    "periodicComInd": {
	///      "$ref": "#/components/schemas/PeriodicCommunicationIndicator"
	///    },
	///    "periodicTime": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "scheduledComTime": {
	///      "$ref": "#/components/schemas/ScheduledCommunicationTime"
	///    },
	///    "stationaryInd": {
	///      "$ref": "#/components/schemas/StationaryIndication"
	///    },
	///    "trafficProfile": {
	///      "$ref": "#/components/schemas/TrafficProfile"
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
	pub struct UeDifferentiationInfo {
		#[serde(
			rename = "batteryInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub battery_ind: Option<BatteryIndication>,
		#[serde(
			rename = "periodicComInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub periodic_com_ind: Option<PeriodicCommunicationIndicator>,
		#[serde(
			rename = "periodicTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub periodic_time: Option<DurationSec>,
		#[serde(
			rename = "scheduledComTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub scheduled_com_time: Option<ScheduledCommunicationTime>,
		#[serde(
			rename = "stationaryInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub stationary_ind: Option<StationaryIndication>,
		#[serde(
			rename = "trafficProfile",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub traffic_profile: Option<TrafficProfile>,
		#[serde(
			rename = "validityTime",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub validity_time: Option<DateTime>,
	}

	impl From<&UeDifferentiationInfo> for UeDifferentiationInfo {
		fn from(value: &UeDifferentiationInfo) -> Self {
			value.clone()
		}
	}

	/// UE Identity
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "UE Identity",
	///  "type": "object",
	///  "properties": {
	///    "gpsi": {
	///      "$ref": "#/components/schemas/Gpsi"
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
	pub struct UeIdExt {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub gpsi: Option<Gpsi>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub supi: Option<Supi>,
	}

	impl From<&UeIdExt> for UeIdExt {
		fn from(value: &UeIdExt) -> Self {
			value.clone()
		}
	}

	/// Additional filters for UE in Area Report event
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Additional filters for UE in Area Report event",
	///  "type": "object",
	///  "properties": {
	///    "aerialSrvDnnInd": {
	///      "default": false,
	///      "type": "boolean"
	///    },
	///    "ueType": {
	///      "$ref": "#/components/schemas/UeType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeInAreaFilter {
		#[serde(rename = "aerialSrvDnnInd", default)]
		pub aerial_srv_dnn_ind: bool,
		#[serde(rename = "ueType", default, skip_serializing_if = "Option::is_none")]
		pub ue_type: Option<UeType>,
	}

	impl From<&UeInAreaFilter> for UeInAreaFilter {
		fn from(value: &UeInAreaFilter) -> Self {
			value.clone()
		}
	}

	/// list of UEs requested to be made reachable for the MBS Session
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "list of UEs requested to be made reachable for the MBS
	/// Session",
	///  "type": "object",
	///  "required": [
	///    "ueList"
	///  ],
	///  "properties": {
	///    "pduSessionId": {
	///      "$ref": "#/components/schemas/PduSessionId"
	///    },
	///    "ueList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Supi"
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
	pub struct UeInfo {
		#[serde(
			rename = "pduSessionId",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pdu_session_id: Option<PduSessionId>,
		#[serde(rename = "ueList")]
		pub ue_list: Vec<Supi>,
	}

	impl From<&UeInfo> for UeInfo {
		fn from(value: &UeInfo) -> Self {
			value.clone()
		}
	}

	/// Report Item for UE Location Trends event.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Report Item for UE Location Trends event.",
	///  "type": "object",
	///  "required": [
	///    "duration",
	///    "spacing",
	///    "timestamp"
	///  ],
	///  "properties": {
	///    "duration": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "ecgi": {
	///      "$ref": "#/components/schemas/Ecgi"
	///    },
	///    "n3gaLocation": {
	///      "$ref": "#/components/schemas/N3gaLocation"
	///    },
	///    "ncgi": {
	///      "$ref": "#/components/schemas/Ncgi"
	///    },
	///    "spacing": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "tai": {
	///      "$ref": "#/components/schemas/Tai"
	///    },
	///    "timestamp": {
	///      "$ref": "#/components/schemas/DateTime"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeLocationTrendsReportItem {
		pub duration: DurationSec,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ecgi: Option<Ecgi>,
		#[serde(
			rename = "n3gaLocation",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n3ga_location: Option<N3gaLocation>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub ncgi: Option<Ncgi>,
		pub spacing: DurationSec,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub tai: Option<Tai>,
		pub timestamp: DateTime,
	}

	impl From<&UeLocationTrendsReportItem> for UeLocationTrendsReportItem {
		fn from(value: &UeLocationTrendsReportItem) -> Self {
			value.clone()
		}
	}

	/// Represents UE mobility information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents UE mobility information.",
	///  "type": "object",
	///  "allOf": [
	///    {
	///      "required": [
	///        "duration"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "locInfos"
	///      ]
	///    },
	///    {
	///      "oneOf": [
	///        {
	///          "required": [
	///            "ts"
	///          ]
	///        },
	///        {
	///          "required": [
	///            "recurringTime"
	///          ]
	///        }
	///      ]
	///    }
	///  ],
	///  "properties": {
	///    "duration": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "durationVariance": {
	///      "$ref": "#/components/schemas/Float"
	///    },
	///    "locInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/LocationInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "recurringTime": {
	///      "$ref": "#/components/schemas/schemas-ScheduledCommunicationTime"
	///    },
	///    "ts": {
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
	pub enum UeMobility {
		#[default]
		Variant0 {
			duration: DurationSec,
			#[serde(
				rename = "durationVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			duration_variance: Option<Float>,
			#[serde(rename = "locInfos")]
			loc_infos: Vec<LocationInfo>,
			ts: DateTime,
		},
		Variant1 {
			duration: DurationSec,
			#[serde(
				rename = "durationVariance",
				default,
				skip_serializing_if = "Option::is_none"
			)]
			duration_variance: Option<Float>,
			#[serde(rename = "locInfos")]
			loc_infos: Vec<LocationInfo>,
			#[serde(rename = "recurringTime")]
			recurring_time: SchemasScheduledCommunicationTime,
		},
	}

	impl From<&UeMobility> for UeMobility {
		fn from(value: &UeMobility) -> Self {
			value.clone()
		}
	}

	/// Data within a create subscription request for UE specific N1 and/or N2
	/// information notification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a create subscription request for UE
	/// specific N1 and/or N2 information notification",
	///  "type": "object",
	///  "properties": {
	///    "n1MessageClass": {
	///      "$ref": "#/components/schemas/N1MessageClass"
	///    },
	///    "n1NotifyCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "n2InformationClass": {
	///      "$ref": "#/components/schemas/N2InformationClass"
	///    },
	///    "n2NotifyCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "nfId": {
	///      "$ref": "#/components/schemas/NfInstanceId"
	///    },
	///    "oldGuami": {
	///      "$ref": "#/components/schemas/Guami"
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
	pub struct UeN1N2InfoSubscriptionCreateData {
		#[serde(
			rename = "n1MessageClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_message_class: Option<N1MessageClass>,
		#[serde(
			rename = "n1NotifyCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n1_notify_callback_uri: Option<Uri>,
		#[serde(
			rename = "n2InformationClass",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_information_class: Option<N2InformationClass>,
		#[serde(
			rename = "n2NotifyCallbackUri",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub n2_notify_callback_uri: Option<Uri>,
		#[serde(rename = "nfId", default, skip_serializing_if = "Option::is_none")]
		pub nf_id: Option<NfInstanceId>,
		#[serde(rename = "oldGuami", default, skip_serializing_if = "Option::is_none")]
		pub old_guami: Option<crate::common::common_models::Guami>,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&UeN1N2InfoSubscriptionCreateData> for UeN1N2InfoSubscriptionCreateData {
		fn from(value: &UeN1N2InfoSubscriptionCreateData) -> Self {
			value.clone()
		}
	}

	/// Data for the created subscription for UE specific N1 and/or N2
	/// information notification
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data for the created subscription for UE specific N1
	/// and/or N2 information notification",
	///  "type": "object",
	///  "required": [
	///    "n1n2NotifySubscriptionId"
	///  ],
	///  "properties": {
	///    "n1n2NotifySubscriptionId": {
	///      "type": "string"
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
	pub struct UeN1N2InfoSubscriptionCreatedData {
		#[serde(rename = "n1n2NotifySubscriptionId")]
		pub n1n2_notify_subscription_id: String,
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
	}

	impl From<&UeN1N2InfoSubscriptionCreatedData> for UeN1N2InfoSubscriptionCreatedData {
		fn from(value: &UeN1N2InfoSubscriptionCreatedData) -> Self {
			value.clone()
		}
	}

	/// Positioning capabilities supported by the UE. A string encoding the
	/// "ProvideCapabilities-r9-IEs" IE as specified in clause 6.3 of 3GPP TS
	/// 37.355 (start from octet 1).
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Positioning capabilities supported by the UE. A string
	/// encoding the \"ProvideCapabilities-r9-IEs\" IE as specified in clause
	/// 6.3 of 3GPP TS 37.355 (start from octet 1).",
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
	pub struct UePositioningCapabilities(pub String);

	impl ::std::ops::Deref for UePositioningCapabilities {
		type Target = String;
		fn deref(&self) -> &String {
			&self.0
		}
	}

	impl From<UePositioningCapabilities> for String {
		fn from(value: UePositioningCapabilities) -> Self {
			value.0
		}
	}

	impl From<&UePositioningCapabilities> for UePositioningCapabilities {
		fn from(value: &UePositioningCapabilities) -> Self {
			value.clone()
		}
	}

	impl From<String> for UePositioningCapabilities {
		fn from(value: String) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for UePositioningCapabilities {
		type Err = std::convert::Infallible;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.to_string()))
		}
	}

	impl ToString for UePositioningCapabilities {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// UE privacy requirements from (H)GMLC to the serving AMF or VGMLC(in the
	/// roaming case) for the target UE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "UE privacy requirements from (H)GMLC to the serving AMF
	/// or VGMLC(in the roaming case) for the target UE",
	///  "type": "object",
	///  "properties": {
	///    "codeWordCheck": {
	///      "type": "boolean"
	///    },
	///    "lcsServiceAuthInfo": {
	///      "$ref": "#/components/schemas/LcsServiceAuth"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UePrivacyRequirements {
		#[serde(
			rename = "codeWordCheck",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub code_word_check: Option<bool>,
		#[serde(
			rename = "lcsServiceAuthInfo",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lcs_service_auth_info: Option<LcsServiceAuth>,
	}

	impl From<&UePrivacyRequirements> for UePrivacyRequirements {
		fn from(value: &UePrivacyRequirements) -> Self {
			value.clone()
		}
	}

	/// Describes the reachability of the UE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the reachability of the UE",
	///  "type": "string",
	///  "enum": [
	///    "UNREACHABLE",
	///    "REACHABLE",
	///    "REGULATORY_ONLY"
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
	pub enum UeReachability {
		#[default]
		#[serde(rename = "UNREACHABLE")]
		Unreachable,
		#[serde(rename = "REACHABLE")]
		Reachable,
		#[serde(rename = "REGULATORY_ONLY")]
		RegulatoryOnly,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UeReachability> for UeReachability {
		fn from(value: &UeReachability) -> Self {
			value.clone()
		}
	}

	impl ToString for UeReachability {
		fn to_string(&self) -> String {
			match *self {
				Self::Unreachable => "UNREACHABLE".to_string(),
				Self::Reachable => "REACHABLE".to_string(),
				Self::RegulatoryOnly => "REGULATORY_ONLY".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UeReachability {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UNREACHABLE" => Ok(Self::Unreachable),
				"REACHABLE" => Ok(Self::Reachable),
				"REGULATORY_ONLY" => Ok(Self::RegulatoryOnly),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UeReachability {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UeReachability {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UeReachability {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Data within a UE registration status update request to indicate a
	/// completion of transferring at a target AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a UE registration status update request to
	/// indicate a completion of transferring at a target AMF",
	///  "type": "object",
	///  "required": [
	///    "transferStatus"
	///  ],
	///  "properties": {
	///    "analyticsNotUsedList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/Uri"
	///      },
	///      "minItems": 1
	///    },
	///    "pcfReselectedInd": {
	///      "type": "boolean"
	///    },
	///    "smfChangeInfoList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/SmfChangeInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "toReleaseSessionInfo": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/ReleaseSessionInfo"
	///      },
	///      "minItems": 1
	///    },
	///    "toReleaseSessionList": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/PduSessionId"
	///      },
	///      "minItems": 1
	///    },
	///    "transferStatus": {
	///      "$ref": "#/components/schemas/UeContextTransferStatus"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeRegStatusUpdateReqData {
		#[serde(
			rename = "analyticsNotUsedList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub analytics_not_used_list: Vec<Uri>,
		#[serde(
			rename = "pcfReselectedInd",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pcf_reselected_ind: Option<bool>,
		#[serde(
			rename = "smfChangeInfoList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub smf_change_info_list: Vec<SmfChangeInfo>,
		#[serde(
			rename = "toReleaseSessionInfo",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub to_release_session_info: Vec<ReleaseSessionInfo>,
		#[serde(
			rename = "toReleaseSessionList",
			default,
			skip_serializing_if = "Vec::is_empty"
		)]
		pub to_release_session_list: Vec<PduSessionId>,
		#[serde(rename = "transferStatus")]
		pub transfer_status: UeContextTransferStatus,
	}

	impl From<&UeRegStatusUpdateReqData> for UeRegStatusUpdateReqData {
		fn from(value: &UeRegStatusUpdateReqData) -> Self {
			value.clone()
		}
	}

	/// Data within a UE registration status update response to provide the
	/// status of UE context transfer status update at a source AMF
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Data within a UE registration status update response to
	/// provide the status of UE context transfer status update at a source
	/// AMF",
	///  "type": "object",
	///  "required": [
	///    "regStatusTransferComplete"
	///  ],
	///  "properties": {
	///    "regStatusTransferComplete": {
	///      "type": "boolean"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UeRegStatusUpdateRspData {
		#[serde(rename = "regStatusTransferComplete")]
		pub reg_status_transfer_complete: bool,
	}

	impl From<&UeRegStatusUpdateRspData> for UeRegStatusUpdateRspData {
		fn from(value: &UeRegStatusUpdateRspData) -> Self {
			value.clone()
		}
	}

	/// UeSecurityCapability
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
	pub struct UeSecurityCapability(pub Bytes);

	impl ::std::ops::Deref for UeSecurityCapability {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<UeSecurityCapability> for Bytes {
		fn from(value: UeSecurityCapability) -> Self {
			value.0
		}
	}

	impl From<&UeSecurityCapability> for UeSecurityCapability {
		fn from(value: &UeSecurityCapability) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for UeSecurityCapability {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for UeSecurityCapability {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for UeSecurityCapability {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UeSecurityCapability {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UeSecurityCapability {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for UeSecurityCapability {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Describes the type of UEs
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the type of UEs",
	///  "type": "string",
	///  "enum": [
	///    "AERIAL_UE"
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
	pub enum UeType {
		#[default]
		#[serde(rename = "AERIAL_UE")]
		AerialUe,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UeType> for UeType {
		fn from(value: &UeType) -> Self {
			value.clone()
		}
	}

	impl ToString for UeType {
		fn to_string(&self) -> String {
			match *self {
				Self::AerialUe => "AERIAL_UE".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UeType {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"AERIAL_UE" => Ok(Self::AerialUe),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UeType {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UeType {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// UmtTime
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "type": "object",
	///  "required": [
	///    "dayOfWeek",
	///    "timeOfDay"
	///  ],
	///  "properties": {
	///    "dayOfWeek": {
	///      "$ref": "#/components/schemas/DayOfWeek"
	///    },
	///    "timeOfDay": {
	///      "$ref": "#/components/schemas/TimeOfDay"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UmtTime {
		#[serde(rename = "dayOfWeek")]
		pub day_of_week: DayOfWeek,
		#[serde(rename = "timeOfDay")]
		pub time_of_day: TimeOfDay,
	}

	impl From<&UmtTime> for UmtTime {
		fn from(value: &UmtTime) -> Self {
			value.clone()
		}
	}

	/// UE policy delivery related N1 message notification subscription data.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "UE policy delivery related N1 message notification
	/// subscription data.",
	///  "type": "object",
	///  "required": [
	///    "updpNotifyCallbackUri",
	///    "updpNotifySubscriptionId"
	///  ],
	///  "properties": {
	///    "supportedFeatures": {
	///      "$ref": "#/components/schemas/SupportedFeatures"
	///    },
	///    "updpCallbackBinding": {
	///      "type": "string"
	///    },
	///    "updpNotifyCallbackUri": {
	///      "$ref": "#/components/schemas/Uri"
	///    },
	///    "updpNotifySubscriptionId": {
	///      "type": "string"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct UpdpSubscriptionData {
		#[serde(
			rename = "supportedFeatures",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub supported_features: Option<SupportedFeatures>,
		#[serde(
			rename = "updpCallbackBinding",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub updp_callback_binding: Option<String>,
		#[serde(rename = "updpNotifyCallbackUri")]
		pub updp_notify_callback_uri: Uri,
		#[serde(rename = "updpNotifySubscriptionId")]
		pub updp_notify_subscription_id: String,
	}

	impl From<&UpdpSubscriptionData> for UpdpSubscriptionData {
		fn from(value: &UpdpSubscriptionData) -> Self {
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

	/// Indicates usage made of the location measurement.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates usage made of the location measurement.",
	///  "type": "string",
	///  "enum": [
	///    "UNSUCCESS",
	///    "SUCCESS_RESULTS_NOT_USED",
	///    "SUCCESS_RESULTS_USED_TO_VERIFY_LOCATION",
	///    "SUCCESS_RESULTS_USED_TO_GENERATE_LOCATION",
	///    "SUCCESS_METHOD_NOT_DETERMINED"
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
	pub enum Usage {
		#[default]
		#[serde(rename = "UNSUCCESS")]
		Unsuccess,
		#[serde(rename = "SUCCESS_RESULTS_NOT_USED")]
		SuccessResultsNotUsed,
		#[serde(rename = "SUCCESS_RESULTS_USED_TO_VERIFY_LOCATION")]
		SuccessResultsUsedToVerifyLocation,
		#[serde(rename = "SUCCESS_RESULTS_USED_TO_GENERATE_LOCATION")]
		SuccessResultsUsedToGenerateLocation,
		#[serde(rename = "SUCCESS_METHOD_NOT_DETERMINED")]
		SuccessMethodNotDetermined,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&Usage> for Usage {
		fn from(value: &Usage) -> Self {
			value.clone()
		}
	}

	impl ToString for Usage {
		fn to_string(&self) -> String {
			match *self {
				Self::Unsuccess => "UNSUCCESS".to_string(),
				Self::SuccessResultsNotUsed => "SUCCESS_RESULTS_NOT_USED".to_string(),
				Self::SuccessResultsUsedToVerifyLocation => {
					"SUCCESS_RESULTS_USED_TO_VERIFY_LOCATION".to_string()
				}
				Self::SuccessResultsUsedToGenerateLocation => {
					"SUCCESS_RESULTS_USED_TO_GENERATE_LOCATION".to_string()
				}
				Self::SuccessMethodNotDetermined => "SUCCESS_METHOD_NOT_DETERMINED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for Usage {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UNSUCCESS" => Ok(Self::Unsuccess),
				"SUCCESS_RESULTS_NOT_USED" => Ok(Self::SuccessResultsNotUsed),
				"SUCCESS_RESULTS_USED_TO_VERIFY_LOCATION" => {
					Ok(Self::SuccessResultsUsedToVerifyLocation)
				}
				"SUCCESS_RESULTS_USED_TO_GENERATE_LOCATION" => {
					Ok(Self::SuccessResultsUsedToGenerateLocation)
				}
				"SUCCESS_METHOD_NOT_DETERMINED" => Ok(Self::SuccessMethodNotDetermined),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for Usage {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Usage {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Usage {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the user data congestion information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the user data congestion information.",
	///  "type": "object",
	///  "required": [
	///    "congestionInfo",
	///    "networkArea"
	///  ],
	///  "properties": {
	///    "congestionInfo": {
	///      "$ref": "#/components/schemas/CongestionInfo"
	///    },
	///    "networkArea": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
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
	pub struct UserDataCongestionInfo {
		#[serde(rename = "congestionInfo")]
		pub congestion_info: CongestionInfo,
		#[serde(rename = "networkArea")]
		pub network_area: NetworkAreaInfo,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub snssai: Option<Snssai>,
	}

	impl From<&UserDataCongestionInfo> for UserDataCongestionInfo {
		fn from(value: &UserDataCongestionInfo) -> Self {
			value.clone()
		}
	}

	/// Indicates the UUAA-MM status
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates the UUAA-MM status",
	///  "type": "string",
	///  "enum": [
	///    "SUCCESS",
	///    "PENDING",
	///    "FAILED"
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
	pub enum UuaaMmStatus {
		#[default]
		#[serde(rename = "SUCCESS")]
		Success,
		#[serde(rename = "PENDING")]
		Pending,
		#[serde(rename = "FAILED")]
		Failed,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&UuaaMmStatus> for UuaaMmStatus {
		fn from(value: &UuaaMmStatus) -> Self {
			value.clone()
		}
	}

	impl ToString for UuaaMmStatus {
		fn to_string(&self) -> String {
			match *self {
				Self::Success => "SUCCESS".to_string(),
				Self::Pending => "PENDING".to_string(),
				Self::Failed => "FAILED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for UuaaMmStatus {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"SUCCESS" => Ok(Self::Success),
				"PENDING" => Ok(Self::Pending),
				"FAILED" => Ok(Self::Failed),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for UuaaMmStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for UuaaMmStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for UuaaMmStatus {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the V2X services related parameters
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the V2X services related parameters",
	///  "type": "object",
	///  "properties": {
	///    "lteUeSidelinkAmbr": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "lteV2xServicesAuth": {
	///      "$ref": "#/components/schemas/LteV2xAuth"
	///    },
	///    "nrUeSidelinkAmbr": {
	///      "$ref": "#/components/schemas/BitRate"
	///    },
	///    "nrV2xServicesAuth": {
	///      "$ref": "#/components/schemas/NrV2xAuth"
	///    },
	///    "pc5QoSPara": {
	///      "$ref": "#/components/schemas/Pc5QoSPara"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct V2xContext {
		#[serde(
			rename = "lteUeSidelinkAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lte_ue_sidelink_ambr: Option<BitRate>,
		#[serde(
			rename = "lteV2xServicesAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub lte_v2x_services_auth: Option<LteV2xAuth>,
		#[serde(
			rename = "nrUeSidelinkAmbr",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nr_ue_sidelink_ambr: Option<BitRate>,
		#[serde(
			rename = "nrV2xServicesAuth",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub nr_v2x_services_auth: Option<NrV2xAuth>,
		#[serde(
			rename = "pc5QoSPara",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub pc5_qo_s_para: Option<Pc5QoSPara>,
	}

	impl From<&V2xContext> for V2xContext {
		fn from(value: &V2xContext) -> Self {
			value.clone()
		}
	}

	/// V2X related N2 information
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "V2X related N2 information",
	///  "type": "object",
	///  "properties": {
	///    "n2Pc5Pol": {
	///      "$ref": "#/components/schemas/N2InfoContent"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct V2xInformation {
		#[serde(rename = "n2Pc5Pol", default, skip_serializing_if = "Option::is_none")]
		pub n2_pc5_pol: Option<N2InfoContent>,
	}

	impl From<&V2xInformation> for V2xInformation {
		fn from(value: &V2xInformation) -> Self {
			value.clone()
		}
	}

	/// Velocity estimate.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Velocity estimate.",
	///  "oneOf": [
	///    {
	///      "$ref": "#/components/schemas/HorizontalVelocity"
	///    },
	///    {
	///      "$ref": "#/components/schemas/HorizontalWithVerticalVelocity"
	///    },
	///    {
	///      "$ref": "#/components/schemas/HorizontalVelocityWithUncertainty"
	///    },
	///    {
	///      "$ref":
	/// "#/components/schemas/HorizontalWithVerticalVelocityAndUncertainty"
	///    }
	///  ]
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	#[serde(untagged)]
	pub enum VelocityEstimate {
		#[default]
		Velocity(HorizontalVelocity),
		WithVerticalVelocity(HorizontalWithVerticalVelocity),
		VelocityWithUncertainty(HorizontalVelocityWithUncertainty),
		WithVerticalVelocityAndUncertainty(HorizontalWithVerticalVelocityAndUncertainty),
	}

	impl From<&VelocityEstimate> for VelocityEstimate {
		fn from(value: &VelocityEstimate) -> Self {
			value.clone()
		}
	}

	impl From<HorizontalVelocity> for VelocityEstimate {
		fn from(value: HorizontalVelocity) -> Self {
			Self::Velocity(value)
		}
	}

	impl From<HorizontalWithVerticalVelocity> for VelocityEstimate {
		fn from(value: HorizontalWithVerticalVelocity) -> Self {
			Self::WithVerticalVelocity(value)
		}
	}

	impl From<HorizontalVelocityWithUncertainty> for VelocityEstimate {
		fn from(value: HorizontalVelocityWithUncertainty) -> Self {
			Self::VelocityWithUncertainty(value)
		}
	}

	impl From<HorizontalWithVerticalVelocityAndUncertainty> for VelocityEstimate {
		fn from(value: HorizontalWithVerticalVelocityAndUncertainty) -> Self {
			Self::WithVerticalVelocityAndUncertainty(value)
		}
	}

	/// Indicates velocity requirement.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates velocity requirement.",
	///  "type": "string",
	///  "enum": [
	///    "VELOCITY_IS_NOT_REQUESTED",
	///    "VELOCITY_IS_REQUESTED"
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
	pub enum VelocityRequested {
		#[default]
		#[serde(rename = "VELOCITY_IS_NOT_REQUESTED")]
		VelocityIsNotRequested,
		#[serde(rename = "VELOCITY_IS_REQUESTED")]
		VelocityIsRequested,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&VelocityRequested> for VelocityRequested {
		fn from(value: &VelocityRequested) -> Self {
			value.clone()
		}
	}

	impl ToString for VelocityRequested {
		fn to_string(&self) -> String {
			match *self {
				Self::VelocityIsNotRequested => "VELOCITY_IS_NOT_REQUESTED".to_string(),
				Self::VelocityIsRequested => "VELOCITY_IS_REQUESTED".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for VelocityRequested {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"VELOCITY_IS_NOT_REQUESTED" => Ok(Self::VelocityIsNotRequested),
				"VELOCITY_IS_REQUESTED" => Ok(Self::VelocityIsRequested),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for VelocityRequested {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for VelocityRequested {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for VelocityRequested {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Indicates direction of vertical speed.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates direction of vertical speed.",
	///  "type": "string",
	///  "enum": [
	///    "UPWARD",
	///    "DOWNWARD"
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
	pub enum VerticalDirection {
		#[default]
		#[serde(rename = "UPWARD")]
		Upward,
		#[serde(rename = "DOWNWARD")]
		Downward,
	}

	impl From<&VerticalDirection> for VerticalDirection {
		fn from(value: &VerticalDirection) -> Self {
			value.clone()
		}
	}

	impl ToString for VerticalDirection {
		fn to_string(&self) -> String {
			match *self {
				Self::Upward => "UPWARD".to_string(),
				Self::Downward => "DOWNWARD".to_string(),
			}
		}
	}

	impl std::str::FromStr for VerticalDirection {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"UPWARD" => Ok(Self::Upward),
				"DOWNWARD" => Ok(Self::Downward),
				_ => Err("invalid value".into()),
			}
		}
	}

	impl std::convert::TryFrom<&str> for VerticalDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for VerticalDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for VerticalDirection {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Contains the Vertical Protection Level
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Contains the Vertical Protection Level",
	///  "type": "integer",
	///  "maximum": 50000.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VerticalProtectionLevel(pub i64);

	impl ::std::ops::Deref for VerticalProtectionLevel {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<VerticalProtectionLevel> for i64 {
		fn from(value: VerticalProtectionLevel) -> Self {
			value.0
		}
	}

	impl From<&VerticalProtectionLevel> for VerticalProtectionLevel {
		fn from(value: &VerticalProtectionLevel) -> Self {
			value.clone()
		}
	}

	impl From<i64> for VerticalProtectionLevel {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for VerticalProtectionLevel {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for VerticalProtectionLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for VerticalProtectionLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for VerticalProtectionLevel {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for VerticalProtectionLevel {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// VerticalSpeed
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Indicates value of vertical speed.",
	///  "type": "number",
	///  "format": "float",
	///  "maximum": 255.0,
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct VerticalSpeed(pub f32);

	impl ::std::ops::Deref for VerticalSpeed {
		type Target = f32;
		fn deref(&self) -> &f32 {
			&self.0
		}
	}

	impl From<VerticalSpeed> for f32 {
		fn from(value: VerticalSpeed) -> Self {
			value.0
		}
	}

	impl From<&VerticalSpeed> for VerticalSpeed {
		fn from(value: &VerticalSpeed) -> Self {
			value.clone()
		}
	}

	impl From<f32> for VerticalSpeed {
		fn from(value: f32) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for VerticalSpeed {
		type Err = <f32 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for VerticalSpeed {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for VerticalSpeed {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for VerticalSpeed {
		type Error = <f32 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for VerticalSpeed {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Unsigned integer identifying a volume in units of bytes.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Unsigned integer identifying a volume in units of
	/// bytes.",
	///  "type": "integer",
	///  "format": "int64",
	///  "minimum": 0.0
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct Volume(pub i64);

	impl ::std::ops::Deref for Volume {
		type Target = i64;
		fn deref(&self) -> &i64 {
			&self.0
		}
	}

	impl From<Volume> for i64 {
		fn from(value: Volume) -> Self {
			value.0
		}
	}

	impl From<&Volume> for Volume {
		fn from(value: &Volume) -> Self {
			value.clone()
		}
	}

	impl From<i64> for Volume {
		fn from(value: i64) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for Volume {
		type Err = <i64 as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for Volume {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for Volume {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for Volume {
		type Error = <i64 as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for Volume {
		fn to_string(&self) -> String {
			self.0.to_string()
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
	///        "$ref": "#/components/schemas/Bytes"
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
		pub global_line_ids: Vec<Bytes>,
		#[serde(rename = "hfcNIds", default, skip_serializing_if = "Vec::is_empty")]
		pub hfc_n_ids: Vec<HfcNId>,
	}

	/// Possible values are:
	/// - TIME_SLOT_START: Indicates the order of time slot start.
	/// - NUMBER_OF_UES: Indicates the order of number of UEs.
	/// - RSSI: Indicates the order of RSSI.
	/// - RTT: Indicates the order of RTT.
	/// - TRAFFIC_INFO: Indicates the order of Traffic information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Possible values are:\n- TIME_SLOT_START: Indicates the
	/// order of time slot start.\n- NUMBER_OF_UES: Indicates the order of
	/// number of UEs.\n- RSSI: Indicates the order of RSSI.\n- RTT: Indicates
	/// the order of RTT.\n- TRAFFIC_INFO: Indicates the order of Traffic
	/// information.\n",
	///  "type": "string",
	///  "enum": [
	///    "TIME_SLOT_START",
	///    "NUMBER_OF_UES",
	///    "RSSI",
	///    "RTT",
	///    "TRAFFIC_INFO"
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
	pub enum WlanOrderingCriterion {
		#[default]
		#[serde(rename = "TIME_SLOT_START")]
		TimeSlotStart,
		#[serde(rename = "NUMBER_OF_UES")]
		NumberOfUes,
		#[serde(rename = "RSSI")]
		Rssi,
		#[serde(rename = "RTT")]
		Rtt,
		#[serde(rename = "TRAFFIC_INFO")]
		TrafficInfo,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&WlanOrderingCriterion> for WlanOrderingCriterion {
		fn from(value: &WlanOrderingCriterion) -> Self {
			value.clone()
		}
	}

	impl ToString for WlanOrderingCriterion {
		fn to_string(&self) -> String {
			match *self {
				Self::TimeSlotStart => "TIME_SLOT_START".to_string(),
				Self::NumberOfUes => "NUMBER_OF_UES".to_string(),
				Self::Rssi => "RSSI".to_string(),
				Self::Rtt => "RTT".to_string(),
				Self::TrafficInfo => "TRAFFIC_INFO".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for WlanOrderingCriterion {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"TIME_SLOT_START" => Ok(Self::TimeSlotStart),
				"NUMBER_OF_UES" => Ok(Self::NumberOfUes),
				"RSSI" => Ok(Self::Rssi),
				"RTT" => Ok(Self::Rtt),
				"TRAFFIC_INFO" => Ok(Self::TrafficInfo),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for WlanOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for WlanOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for WlanOrderingCriterion {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// The WLAN performance per SSID.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The WLAN performance per SSID.",
	///  "type": "object",
	///  "required": [
	///    "ssId",
	///    "wlanPerTsInfos"
	///  ],
	///  "properties": {
	///    "ssId": {
	///      "type": "string"
	///    },
	///    "wlanPerTsInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/WlanPerTsPerformanceInfo"
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
	pub struct WlanPerSsIdPerformanceInfo {
		#[serde(rename = "ssId")]
		pub ss_id: String,
		#[serde(rename = "wlanPerTsInfos")]
		pub wlan_per_ts_infos: Vec<WlanPerTsPerformanceInfo>,
	}

	impl From<&WlanPerSsIdPerformanceInfo> for WlanPerSsIdPerformanceInfo {
		fn from(value: &WlanPerSsIdPerformanceInfo) -> Self {
			value.clone()
		}
	}

	/// WLAN performance information per Time Slot during the analytics target
	/// period.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "WLAN performance information per Time Slot during the
	/// analytics target period.",
	///  "type": "object",
	///  "anyOf": [
	///    {
	///      "required": [
	///        "rssi"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "rtt"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "trafficInfo"
	///      ]
	///    },
	///    {
	///      "required": [
	///        "numberOfUes"
	///      ]
	///    }
	///  ],
	///  "required": [
	///    "tsDuration",
	///    "tsStart"
	///  ],
	///  "properties": {
	///    "confidence": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "numberOfUes": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "rssi": {
	///      "type": "integer"
	///    },
	///    "rtt": {
	///      "$ref": "#/components/schemas/Uinteger"
	///    },
	///    "trafficInfo": {
	///      "$ref": "#/components/schemas/TrafficInformation"
	///    },
	///    "tsDuration": {
	///      "$ref": "#/components/schemas/DurationSec"
	///    },
	///    "tsStart": {
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
	pub enum WlanPerTsPerformanceInfo {
		#[default]
		Variant0 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			rssi: i64,
			#[serde(rename = "tsDuration")]
			ts_duration: DurationSec,
			#[serde(rename = "tsStart")]
			ts_start: DateTime,
		},
		Variant1 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			rtt: Uinteger,
			#[serde(rename = "tsDuration")]
			ts_duration: DurationSec,
			#[serde(rename = "tsStart")]
			ts_start: DateTime,
		},
		Variant2 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "trafficInfo")]
			traffic_info: TrafficInformation,
			#[serde(rename = "tsDuration")]
			ts_duration: DurationSec,
			#[serde(rename = "tsStart")]
			ts_start: DateTime,
		},
		Variant3 {
			#[serde(default, skip_serializing_if = "Option::is_none")]
			confidence: Option<Uinteger>,
			#[serde(rename = "numberOfUes")]
			number_of_ues: Uinteger,
			#[serde(rename = "tsDuration")]
			ts_duration: DurationSec,
			#[serde(rename = "tsStart")]
			ts_start: DateTime,
		},
	}

	impl From<&WlanPerTsPerformanceInfo> for WlanPerTsPerformanceInfo {
		fn from(value: &WlanPerTsPerformanceInfo) -> Self {
			value.clone()
		}
	}

	/// The WLAN performance related information.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "The WLAN performance related information.",
	///  "type": "object",
	///  "required": [
	///    "wlanPerSsidInfos"
	///  ],
	///  "properties": {
	///    "networkArea": {
	///      "$ref": "#/components/schemas/NetworkAreaInfo"
	///    },
	///    "wlanPerSsidInfos": {
	///      "type": "array",
	///      "items": {
	///        "$ref": "#/components/schemas/WlanPerSsIdPerformanceInfo"
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
	pub struct WlanPerformanceInfo {
		#[serde(
			rename = "networkArea",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub network_area: Option<NetworkAreaInfo>,
		#[serde(rename = "wlanPerSsidInfos")]
		pub wlan_per_ssid_infos: Vec<WlanPerSsIdPerformanceInfo>,
	}

	impl From<&WlanPerformanceInfo> for WlanPerformanceInfo {
		fn from(value: &WlanPerformanceInfo) -> Self {
			value.clone()
		}
	}

	/// Represents other WLAN performance analytics requirements.
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents other WLAN performance analytics
	/// requirements.",
	///  "type": "object",
	///  "properties": {
	///    "bssIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "order": {
	///      "$ref": "#/components/schemas/MatchingDirection"
	///    },
	///    "ssIds": {
	///      "type": "array",
	///      "items": {
	///        "type": "string"
	///      },
	///      "minItems": 1
	///    },
	///    "wlanOrderCriter": {
	///      "$ref": "#/components/schemas/WlanOrderingCriterion"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct WlanPerformanceReq {
		#[serde(rename = "bssIds", default, skip_serializing_if = "Vec::is_empty")]
		pub bss_ids: Vec<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		pub order: Option<MatchingDirection>,
		#[serde(rename = "ssIds", default, skip_serializing_if = "Vec::is_empty")]
		pub ss_ids: Vec<String>,
		#[serde(
			rename = "wlanOrderCriter",
			default,
			skip_serializing_if = "Option::is_none"
		)]
		pub wlan_order_criter: Option<WlanOrderingCriterion>,
	}

	impl From<&WlanPerformanceReq> for WlanPerformanceReq {
		fn from(value: &WlanPerformanceReq) -> Self {
			value.clone()
		}
	}

	/// _5gMmCapability
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
	pub struct _5gMmCapability(pub Bytes);

	impl ::std::ops::Deref for _5gMmCapability {
		type Target = Bytes;
		fn deref(&self) -> &Bytes {
			&self.0
		}
	}

	impl From<_5gMmCapability> for Bytes {
		fn from(value: _5gMmCapability) -> Self {
			value.0
		}
	}

	impl From<&_5gMmCapability> for _5gMmCapability {
		fn from(value: &_5gMmCapability) -> Self {
			value.clone()
		}
	}

	impl From<Bytes> for _5gMmCapability {
		fn from(value: Bytes) -> Self {
			Self(value)
		}
	}

	impl std::str::FromStr for _5gMmCapability {
		type Err = <Bytes as std::str::FromStr>::Err;
		fn from_str(value: &str) -> Result<Self, Self::Err> {
			Ok(Self(value.parse()?))
		}
	}

	impl std::convert::TryFrom<&str> for _5gMmCapability {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &str) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for _5gMmCapability {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: &String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for _5gMmCapability {
		type Error = <Bytes as std::str::FromStr>::Err;
		fn try_from(value: String) -> Result<Self, Self::Error> {
			value.parse()
		}
	}

	impl ToString for _5gMmCapability {
		fn to_string(&self) -> String {
			self.0.to_string()
		}
	}

	/// Describes the 5GS User State of a UE
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Describes the 5GS User State of a UE",
	///  "type": "string",
	///  "enum": [
	///    "DEREGISTERED",
	///    "CONNECTED_NOT_REACHABLE_FOR_PAGING",
	///    "CONNECTED_REACHABLE_FOR_PAGING",
	///    "NOT_PROVIDED_FROM_AMF"
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
	pub enum _5gsUserState {
		#[default]
		#[serde(rename = "DEREGISTERED")]
		Deregistered,
		#[serde(rename = "CONNECTED_NOT_REACHABLE_FOR_PAGING")]
		ConnectedNotReachableForPaging,
		#[serde(rename = "CONNECTED_REACHABLE_FOR_PAGING")]
		ConnectedReachableForPaging,
		#[serde(rename = "NOT_PROVIDED_FROM_AMF")]
		NotProvidedFromAmf,
		#[serde(untagged)]
		UnknownOther(String),
	}

	impl From<&_5gsUserState> for _5gsUserState {
		fn from(value: &_5gsUserState) -> Self {
			value.clone()
		}
	}

	impl ToString for _5gsUserState {
		fn to_string(&self) -> String {
			match *self {
				Self::Deregistered => "DEREGISTERED".to_string(),
				Self::ConnectedNotReachableForPaging => {
					"CONNECTED_NOT_REACHABLE_FOR_PAGING".to_string()
				}
				Self::ConnectedReachableForPaging => "CONNECTED_REACHABLE_FOR_PAGING".to_string(),
				Self::NotProvidedFromAmf => "NOT_PROVIDED_FROM_AMF".to_string(),
				Self::UnknownOther(ref value) => value.clone(),
			}
		}
	}

	impl std::str::FromStr for _5gsUserState {
		type Err = self::error::ConversionError;
		fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
			match value {
				"DEREGISTERED" => Ok(Self::Deregistered),
				"CONNECTED_NOT_REACHABLE_FOR_PAGING" => Ok(Self::ConnectedNotReachableForPaging),
				"CONNECTED_REACHABLE_FOR_PAGING" => Ok(Self::ConnectedReachableForPaging),
				"NOT_PROVIDED_FROM_AMF" => Ok(Self::NotProvidedFromAmf),
				_ => Ok(Self::UnknownOther(value.to_string())),
			}
		}
	}

	impl std::convert::TryFrom<&str> for _5gsUserState {
		type Error = self::error::ConversionError;
		fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<&String> for _5gsUserState {
		type Error = self::error::ConversionError;
		fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	impl std::convert::TryFrom<String> for _5gsUserState {
		type Error = self::error::ConversionError;
		fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
			value.parse()
		}
	}

	/// Represents the 5GS User state of the UE for an access type
	///
	/// <details><summary>JSON schema</summary>
	///
	/// ```json
	/// {
	///  "description": "Represents the 5GS User state of the UE for an access
	/// type",
	///  "type": "object",
	///  "required": [
	///    "5gsUserState",
	///    "accessType"
	///  ],
	///  "properties": {
	///    "5gsUserState": {
	///      "$ref": "#/components/schemas/5GsUserState"
	///    },
	///    "accessType": {
	///      "$ref": "#/components/schemas/AccessType"
	///    }
	///  }
	/// }
	/// ```
	/// </details>
	#[derive(
		::serde::Deserialize, ::serde::Serialize, Clone, Debug, smart_default::SmartDefault,
	)]
	pub struct _5gsUserStateInfo {
		#[serde(rename = "accessType")]
		pub access_type: AccessType,
		#[serde(rename = "5gsUserState")]
		pub five_gs_user_state: _5gsUserState,
	}

	impl From<&_5gsUserStateInfo> for _5gsUserStateInfo {
		fn from(value: &_5gsUserStateInfo) -> Self {
			value.clone()
		}
	}

	/// Generation of default values for serde.
	pub mod defaults {
		pub(super) fn default_bool<const V: bool>() -> bool {
			V
		}
	}
}
